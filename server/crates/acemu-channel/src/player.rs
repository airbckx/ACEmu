//! Character-level views over the inventory: the default character, a
//! player's level and the character sheet.

use crate::inventory;

const ACCOUNT_FILE: &str = "acemu-account.txt";

/// The character of the original single account (`save/acemu-account.txt`);
/// used as the fallback name and as the owner of the legacy inventory file.
pub fn default_character() -> String {
    std::fs::read_to_string(acemu_paths::save_file(ACCOUNT_FILE))
        .map(|s| s.trim().to_string())
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "ACEmu".into())
}

pub fn level_of(name: &str) -> i32 {
    inventory::read_named(name, |inv| inv.stats.level)
}

/// The character sheet the client shows in My Info; points/cash come from the
/// inventory so the shop balance and the lobby agree.
pub fn char_info(name: &str) -> acemu_proto::CharInfo {
    inventory::read_named(name, |inv| {
        let s = &inv.stats;
        acemu_proto::CharInfo {
            level: s.level as u16,
            exp: s.exp,
            kill: s.kills,
            death: s.deaths,
            win: s.wins,
            lose: s.losses,
            team_kill: s.team_kills,
            assist: s.assists,
            head_shot2: s.headshots,
            tot_play_time: s.play_time,
            point: inv.points,
            cash: inv.cash,
            ..Default::default()
        }
    })
}
