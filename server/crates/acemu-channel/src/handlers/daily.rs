//! Attendance ("roll call"), the puzzle and the month's daily reward list.
//!
//! From the client behaviour (`BTAttendQuest`, `BTDailyQuest`,
//! `BTPageServerSelection.rfAck{AttendanceList,PuzzleList,DailyRewardListThisMonth}`,
//! `wGameManager.DailyQuestParams`):
//!
//! - The client's `wDailyQuestParam` table (`data/daily-quests.json`) gives
//!   every daily-typed quest a *daily type*: 2 attendance, 3/4/5 puzzle 1..3,
//!   7 special (1 is the ordinary daily set). All candidates of a type are step
//!   1, and the quest of the day is `candidates[serverDay % n]`
//!   (`GetDailyQuestIDFromTypeAndStep`). The quest list draws the attendance /
//!   puzzle mark from that table, so only these ids count.
//! - `rfAckDailyRewardListThisMonth(Type[], Number[], ItemID[], ItemCount[])`
//!   is the reward list the panels index directly: 0..19 the attendance
//!   rewards for 1..20 check-ins, 20 the puzzle reward, 21 the special
//!   mission's reward (`data/daily-rewards.json`, from the client's
//!   `wDailyQuestRewardParam` table: Attend_Check_Num / Puzzle_Num).
//! - Attendance: `rfReqAttendanceList(ReqType, UpdateNum, UpdateState, PaidItemIdx)`
//!   - type 0 asks for the state, type 2 claims reward `UpdateNum`. `StateList`
//!   (20 entries): 0 not reached, 1 claimable, 2 taken; the panel shows
//!   the number of 2s as the attendance count. Completing the day's
//!   attendance quest is a check-in; the client is told with
//!   `rfAckAttendanceList(IsComplete=1, UpdatedNum=count)`.
//! - Puzzle: `rfReqPuzzleList(ReqType, UpdateState, PaidItemIdx)` likewise;
//!   `StateList` has 8 entries (pieces). All eight at 1 enables "receive".
//!   Completing the day's puzzle quest earns the next piece, announced with
//!   `rfAckPuzzleList(IsComplete=1, UpdatedNum=[piece])` (the quest-complete
//!   window draws those pieces, so it must arrive before
//!   `rfReqQuestNewAvailable`). Claiming pays the puzzle's reward, clears the
//!   pieces and moves on to the next puzzle (1 -> 2 -> 3 -> 1).
//! - A reward answer is `IsComplete=0` with `RewardItemID/Count` set: the
//!   client reloads its item list and shows "you received <item>".

use super::Reply;
use crate::inventory;
use crate::session::Ctx;
use acemu_db::social::{self as db_social, DailyRewardRow, DailyState};
use acemu_proto::channel;
use acemu_proto::frame::Packet;

const ATTENDANCE_DAYS: usize = 20;
const PUZZLE_PIECES: i32 = 8;
const PUZZLE_TIERS: i32 = 3;

/// Serial day number from the OLE clock (what the client feeds the rotation).
fn server_day() -> i64 {
    inventory::ole_now().floor() as i64
}

fn month_of(day: &str) -> String {
    day.chars().take(7).collect()
}

/// Daily type of a quest id (None for quests outside the daily table).
pub fn daily_type_of(quest_id: i32) -> Option<i32> {
    inventory::db().daily_quests().unwrap_or_default().iter().find(|r| r.quest_id == quest_id).map(|r| r.daily_type)
}

/// The quest of `daily_type` for the current day.
fn quest_of_the_day(daily_type: i32) -> Option<i32> {
    let cands: Vec<i32> = inventory::db().daily_quests().unwrap_or_default().iter().filter(|r| r.daily_type == daily_type).map(|r| r.quest_id).collect();
    if cands.is_empty() {
        return None;
    }
    let idx = (server_day().rem_euclid(cands.len() as i64)) as usize;
    Some(cands[idx])
}

/// Today's attendance, puzzle (the player's current tier) and special quest.
pub fn today_quests(name: &str) -> Vec<i32> {
    let tier = state_for(name, &super::quests::today()).puzzle_tier.clamp(1, PUZZLE_TIERS);
    [db_social::DAILY_ATTENDANCE, db_social::DAILY_PUZZLE_FIRST + tier - 1, db_social::DAILY_SPECIAL].iter().filter_map(|&t| quest_of_the_day(t)).collect()
}

/// The player's daily state with the month rollover applied.
fn state_for(name: &str, today: &str) -> DailyState {
    let db = inventory::db();
    let mut s = db.daily_state(name).unwrap_or_default();
    let month = month_of(today);
    if s.attend_month != month {
        s.attend_month = month;
        s.attend_count = 0;
        s.attend_claimed = 0;
        s.attend_day.clear();
        let _ = db.daily_state_set(name, &s);
    }
    if s.puzzle_tier < 1 || s.puzzle_tier > PUZZLE_TIERS {
        s.puzzle_tier = 1;
    }
    s
}

fn rewards() -> Vec<DailyRewardRow> {
    inventory::db().daily_rewards().unwrap_or_default()
}

fn reward(rows: &[DailyRewardRow], reward_type: i32, num: i32) -> Option<&DailyRewardRow> {
    rows.iter().find(|r| r.reward_type == reward_type && r.num == num)
}

/// The special mission's reward item (list index 21).
pub fn special_reward() -> Option<(i32, i32)> {
    reward(&rewards(), db_social::REWARD_SPECIAL, 1).map(|r| (r.item_id, r.count.max(1)))
}

fn attendance_states(s: &DailyState) -> Vec<u8> {
    (0..ATTENDANCE_DAYS as i32)
        .map(|i| {
            if s.attend_claimed & (1 << i) != 0 {
                2
            } else if s.attend_count > i {
                1
            } else {
                0
            }
        })
        .collect()
}

fn puzzle_states(s: &DailyState) -> Vec<u8> {
    (0..PUZZLE_PIECES).map(|i| u8::from(s.puzzle_pieces & (1 << i) != 0)).collect()
}

fn attendance_packet(s: &DailyState, result: i32, is_complete: u8, updated: u8, reward: (i32, i32), paid: f64) -> Packet {
    channel::AckAttendanceList {
        result,
        is_complete,
        updated_num: updated,
        state_list: attendance_states(s),
        item_idx: 0.0,
        reward_item_id: reward.0,
        reward_item_count: reward.1,
        paid_item_idx: paid,
        paid_item_stack_count: 0,
    }
    .packet()
}

fn puzzle_packet(s: &DailyState, result: i32, is_complete: u8, updated: Vec<u8>, reward: (i32, i32), paid: f64) -> Packet {
    channel::AckPuzzleList {
        result,
        is_complete,
        updated_num: updated,
        state_list: puzzle_states(s),
        item_idx: 0.0,
        reward_item_id: reward.0,
        reward_item_count: reward.1,
        paid_item_idx: paid,
        paid_item_stack_count: 0,
    }
    .packet()
}

/// The month's reward list for `name` (index 20 depends on the player's puzzle).
pub fn reward_list_packet(name: &str) -> Packet {
    let rows = rewards();
    let s = state_for(name, &super::quests::today());
    let mut out = channel::AckDailyRewardListThisMonth::default();
    for n in 1..=ATTENDANCE_DAYS as i32 {
        let r = reward(&rows, db_social::REWARD_ATTENDANCE, n);
        out.type_.push(db_social::REWARD_ATTENDANCE as u8);
        out.number.push(n as u8);
        out.item_id.push(r.map(|r| r.item_id).unwrap_or(0));
        out.item_count.push(r.map(|r| r.count).unwrap_or(0));
    }
    let puzzle = reward(&rows, db_social::REWARD_PUZZLE, s.puzzle_tier).or_else(|| reward(&rows, db_social::REWARD_PUZZLE, 1));
    out.type_.push(db_social::REWARD_PUZZLE as u8);
    out.number.push(s.puzzle_tier as u8);
    out.item_id.push(puzzle.map(|r| r.item_id).unwrap_or(0));
    out.item_count.push(puzzle.map(|r| r.count).unwrap_or(0));
    let special = reward(&rows, db_social::REWARD_SPECIAL, 1);
    out.type_.push(db_social::REWARD_SPECIAL as u8);
    out.number.push(1);
    out.item_id.push(special.map(|r| r.item_id).unwrap_or(0));
    out.item_count.push(special.map(|r| r.count).unwrap_or(0));
    out.packet()
}

/// A quest of `name` was just completed: a check-in for the day's attendance
/// quest, a puzzle piece for the day's puzzle quest. Returns the pushes that
/// tell the client (send them before `rfReqQuestNewAvailable`).
pub fn on_quest_completed(name: &str, quest_id: i32) -> Vec<Packet> {
    let Some(dtype) = daily_type_of(quest_id) else { return vec![] };
    let today = super::quests::today();
    let db = inventory::db();
    let mut s = state_for(name, &today);
    match dtype {
        db_social::DAILY_ATTENDANCE => {
            if s.attend_day == today {
                return vec![];
            }
            s.attend_day = today;
            s.attend_count = (s.attend_count + 1).min(ATTENDANCE_DAYS as i32);
            let _ = db.daily_state_set(name, &s);
            tracing::info!(player = name, count = s.attend_count, "attendance check-in");
            vec![attendance_packet(&s, 0, 1, s.attend_count as u8, (0, 0), 0.0)]
        }
        t if (db_social::DAILY_PUZZLE_FIRST..=db_social::DAILY_PUZZLE_LAST).contains(&t) => {
            let Some(piece) = (0..PUZZLE_PIECES).find(|i| s.puzzle_pieces & (1 << i) == 0) else { return vec![] };
            s.puzzle_pieces |= 1 << piece;
            s.puzzle_day = today;
            let _ = db.daily_state_set(name, &s);
            tracing::info!(player = name, tier = s.puzzle_tier, piece = piece + 1, "puzzle piece earned");
            vec![puzzle_packet(&s, 0, 1, vec![(piece + 1) as u8], (0, 0), 0.0)]
        }
        _ => vec![],
    }
}

pub fn handle(pkt: &Packet, ctx: &mut Ctx) -> Reply {
    Ok(Some(match pkt.opcode {
        channel::ReqDailyRewardListThisMonth::OPCODE => vec![reward_list_packet(&ctx.name)],

        channel::ReqAttendanceList::OPCODE => {
            let req = channel::ReqAttendanceList::decode(&pkt.payload)?;
            let today = super::quests::today();
            let db = inventory::db();
            let mut s = state_for(&ctx.name, &today);
            if req.req_type != 2 {
                return Ok(Some(vec![attendance_packet(&s, 0, 0, 0, (0, 0), 0.0)]));
            }
            let n = req.update_num as i32;
            let idx = n - 1;
            let claimable = (0..ATTENDANCE_DAYS as i32).contains(&idx) && s.attend_count >= n && s.attend_claimed & (1 << idx) == 0;
            let reward = reward(&rewards(), db_social::REWARD_ATTENDANCE, n).map(|r| (r.item_id, r.count.max(1)));
            tracing::info!(player = %ctx.name, day = n, claimable, reward = ?reward, "ReqAttendanceList (claim)");
            match (claimable, reward) {
                (true, Some((item, count))) => {
                    s.attend_claimed |= 1 << idx;
                    let _ = db.daily_state_set(&ctx.name, &s);
                    inventory::grant(&ctx.name, item, count);
                    vec![attendance_packet(&s, 0, 0, n as u8, (item, count), req.paid_item_idx)]
                }
                _ => vec![attendance_packet(&s, 1, 1, n as u8, (0, 0), req.paid_item_idx)],
            }
        }

        channel::ReqPuzzleList::OPCODE => {
            let req = channel::ReqPuzzleList::decode(&pkt.payload)?;
            let today = super::quests::today();
            let db = inventory::db();
            let mut s = state_for(&ctx.name, &today);
            if req.req_type != 2 {
                return Ok(Some(vec![puzzle_packet(&s, 0, 0, vec![], (0, 0), 0.0)]));
            }
            let complete = s.puzzle_pieces & ((1 << PUZZLE_PIECES) - 1) == (1 << PUZZLE_PIECES) - 1;
            let rows = rewards();
            let reward = reward(&rows, db_social::REWARD_PUZZLE, s.puzzle_tier)
                .or_else(|| reward(&rows, db_social::REWARD_PUZZLE, 1))
                .map(|r| (r.item_id, r.count.max(1)));
            tracing::info!(player = %ctx.name, tier = s.puzzle_tier, complete, reward = ?reward, "ReqPuzzleList (claim)");
            match (complete, reward) {
                (true, Some((item, count))) => {
                    s.puzzle_pieces = 0;
                    s.puzzles_done += 1;
                    s.puzzle_tier = s.puzzle_tier % PUZZLE_TIERS + 1;
                    let _ = db.daily_state_set(&ctx.name, &s);
                    inventory::grant(&ctx.name, item, count);
                    // the reward list changes with the tier: refresh it first
                    vec![reward_list_packet(&ctx.name), puzzle_packet(&s, 0, 0, vec![], (item, count), req.paid_item_idx)]
                }
                _ => vec![puzzle_packet(&s, 1, 1, vec![], (0, 0), req.paid_item_idx)],
            }
        }

        _ => return Ok(None),
    }))
}
