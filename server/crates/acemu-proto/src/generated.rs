// @generated - do not edit by hand. 8 wire structs, 71 Login RPCs, 544 Channel RPCs.
//
// One struct per packet, fields in wire order. The generator and its inputs
// are tables read out of a client install and are not part of this repository;
// this file is checked in because nothing builds without it.
//
// Opcodes are the index of each rf* function within its BTTcpLink_* class, and
// the parameter lists come from the matching property chains. Three opcodes
// and their payload sizes are confirmed against a live capture; the rest follow
// the same derivation. See docs/PROTOCOL.md.
#![allow(dead_code, unused_mut, clippy::all)]

use crate::wire::{Reader, Writer};
use anyhow::Result;

// ---------- wire structs ----------

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtrDouble {
    pub dummy_1_do_not_use: i32,
    pub dummy_2_do_not_use: i32,
}

impl BtrDouble {
    pub fn decode(r: &mut Reader) -> Result<Self> {
        Ok(Self {
            dummy_1_do_not_use: r.i32()?,
            dummy_2_do_not_use: r.i32()?,
        })
    }

    pub fn encode(&self, w: &mut Writer) {
        w.i32(self.dummy_1_do_not_use);
        w.i32(self.dummy_2_do_not_use);
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameRoomInfo {
    pub room_number: i32,
    pub room_name: String,
    pub map_name: String,
    pub game_class: String,
    pub game_mode_num: u8,
    pub max_user_cnt: u8,
    pub team_cnt: u8,
    pub weapon_limit: u8,
    pub game_round: i32,
    pub game_minute: i32,
    pub is_hard_core: u8,
    pub room_owner: String,
    pub map_num: u8,
    pub is_intervention: u8,
    pub is_free_cam: u8,
    pub is_voice_chat: u8,
    pub is_team_valance: u8,
    pub is_our_forces_damage: u8,
    pub is_kill_cam: u8,
    pub is_vote: u8,
    pub is_change_offense_and_defense: u8,
    pub change_offense_and_defense_round: u8,
    pub bomb_hold: u8,
    pub is_password: u8,
    pub password: String,
    pub play_with_bots: u8,
    pub max_af_user_and_bot_num: u8,
    pub max_rsa_user_and_bot_num: u8,
    pub user_team_num: u8,
    pub bot_mode_difficulty: u8,
    pub room_type: u8,
    pub respon_type: u8,
}

impl GameRoomInfo {
    pub fn decode(r: &mut Reader) -> Result<Self> {
        Ok(Self {
            room_number: r.i32()?,
            room_name: r.string()?,
            map_name: r.string()?,
            game_class: r.string()?,
            game_mode_num: r.u8()?,
            max_user_cnt: r.u8()?,
            team_cnt: r.u8()?,
            weapon_limit: r.u8()?,
            game_round: r.i32()?,
            game_minute: r.i32()?,
            is_hard_core: r.u8()?,
            room_owner: r.string()?,
            map_num: r.u8()?,
            is_intervention: r.u8()?,
            is_free_cam: r.u8()?,
            is_voice_chat: r.u8()?,
            is_team_valance: r.u8()?,
            is_our_forces_damage: r.u8()?,
            is_kill_cam: r.u8()?,
            is_vote: r.u8()?,
            is_change_offense_and_defense: r.u8()?,
            change_offense_and_defense_round: r.u8()?,
            bomb_hold: r.u8()?,
            is_password: r.u8()?,
            password: r.string()?,
            play_with_bots: r.u8()?,
            max_af_user_and_bot_num: r.u8()?,
            max_rsa_user_and_bot_num: r.u8()?,
            user_team_num: r.u8()?,
            bot_mode_difficulty: r.u8()?,
            room_type: r.u8()?,
            respon_type: r.u8()?,
        })
    }

    pub fn encode(&self, w: &mut Writer) {
        w.i32(self.room_number);
        w.string(&self.room_name);
        w.string(&self.map_name);
        w.string(&self.game_class);
        w.u8(self.game_mode_num);
        w.u8(self.max_user_cnt);
        w.u8(self.team_cnt);
        w.u8(self.weapon_limit);
        w.i32(self.game_round);
        w.i32(self.game_minute);
        w.u8(self.is_hard_core);
        w.string(&self.room_owner);
        w.u8(self.map_num);
        w.u8(self.is_intervention);
        w.u8(self.is_free_cam);
        w.u8(self.is_voice_chat);
        w.u8(self.is_team_valance);
        w.u8(self.is_our_forces_damage);
        w.u8(self.is_kill_cam);
        w.u8(self.is_vote);
        w.u8(self.is_change_offense_and_defense);
        w.u8(self.change_offense_and_defense_round);
        w.u8(self.bomb_hold);
        w.u8(self.is_password);
        w.string(&self.password);
        w.u8(self.play_with_bots);
        w.u8(self.max_af_user_and_bot_num);
        w.u8(self.max_rsa_user_and_bot_num);
        w.u8(self.user_team_num);
        w.u8(self.bot_mode_difficulty);
        w.u8(self.room_type);
        w.u8(self.respon_type);
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharInfo {
    pub level: u16,
    pub exp: i32,
    pub stamina: i32,
    pub hp: i32,
    pub prev_ranking: i32,
    pub ranking: i32,
    pub prev_head_shot_ranking: i32,
    pub head_shot_ranking: i32,
    pub prev_woman_ranking: i32,
    pub woman_ranking: i32,
    pub monthly_prev_ranking: i32,
    pub monthly_ranking: i32,
    pub monthly_prev_head_shot_ranking: i32,
    pub monthly_head_shot_ranking: i32,
    pub monthly_prev_woman_ranking: i32,
    pub monthly_woman_ranking: i32,
    pub kill: i32,
    pub death: i32,
    pub win: i32,
    pub lose: i32,
    pub team_kill: i32,
    pub assist: i32,
    pub point: i32,
    pub cash: i32,
    pub head_shot2: i32,
    pub dis_con: i32,
    pub tot_play_time: i32,
    pub disp_head_item: i32,
    pub disp_face_item: i32,
    pub disp_backpack: i32,
    pub look_for_clan: u8,
    pub clan_contribue: i32,
    pub clan_win: i32,
    pub clan_lose: i32,
    pub clan_kill: i32,
    pub clan_death: i32,
    pub clan_assist: i32,
    pub clan_headshot: i32,
    pub clan_discon: i32,
    pub clan_ranking_prev: i32,
    pub clan_ranking: i32,
    pub clan_ranking_monthly_prev: i32,
    pub clan_ranking_monthly: i32,
    pub level_mark_id: i32,
    pub title_mark_id: i32,
    pub disp_pouch: i32,
    pub disp_camouflage: i32,
    pub reserved1: i32,
}

impl CharInfo {
    pub fn decode(r: &mut Reader) -> Result<Self> {
        Ok(Self {
            level: r.u16()?,
            exp: r.i32()?,
            stamina: r.i32()?,
            hp: r.i32()?,
            prev_ranking: r.i32()?,
            ranking: r.i32()?,
            prev_head_shot_ranking: r.i32()?,
            head_shot_ranking: r.i32()?,
            prev_woman_ranking: r.i32()?,
            woman_ranking: r.i32()?,
            monthly_prev_ranking: r.i32()?,
            monthly_ranking: r.i32()?,
            monthly_prev_head_shot_ranking: r.i32()?,
            monthly_head_shot_ranking: r.i32()?,
            monthly_prev_woman_ranking: r.i32()?,
            monthly_woman_ranking: r.i32()?,
            kill: r.i32()?,
            death: r.i32()?,
            win: r.i32()?,
            lose: r.i32()?,
            team_kill: r.i32()?,
            assist: r.i32()?,
            point: r.i32()?,
            cash: r.i32()?,
            head_shot2: r.i32()?,
            dis_con: r.i32()?,
            tot_play_time: r.i32()?,
            disp_head_item: r.i32()?,
            disp_face_item: r.i32()?,
            disp_backpack: r.i32()?,
            look_for_clan: r.u8()?,
            clan_contribue: r.i32()?,
            clan_win: r.i32()?,
            clan_lose: r.i32()?,
            clan_kill: r.i32()?,
            clan_death: r.i32()?,
            clan_assist: r.i32()?,
            clan_headshot: r.i32()?,
            clan_discon: r.i32()?,
            clan_ranking_prev: r.i32()?,
            clan_ranking: r.i32()?,
            clan_ranking_monthly_prev: r.i32()?,
            clan_ranking_monthly: r.i32()?,
            level_mark_id: r.i32()?,
            title_mark_id: r.i32()?,
            disp_pouch: r.i32()?,
            disp_camouflage: r.i32()?,
            reserved1: r.i32()?,
        })
    }

    pub fn encode(&self, w: &mut Writer) {
        w.u16(self.level);
        w.i32(self.exp);
        w.i32(self.stamina);
        w.i32(self.hp);
        w.i32(self.prev_ranking);
        w.i32(self.ranking);
        w.i32(self.prev_head_shot_ranking);
        w.i32(self.head_shot_ranking);
        w.i32(self.prev_woman_ranking);
        w.i32(self.woman_ranking);
        w.i32(self.monthly_prev_ranking);
        w.i32(self.monthly_ranking);
        w.i32(self.monthly_prev_head_shot_ranking);
        w.i32(self.monthly_head_shot_ranking);
        w.i32(self.monthly_prev_woman_ranking);
        w.i32(self.monthly_woman_ranking);
        w.i32(self.kill);
        w.i32(self.death);
        w.i32(self.win);
        w.i32(self.lose);
        w.i32(self.team_kill);
        w.i32(self.assist);
        w.i32(self.point);
        w.i32(self.cash);
        w.i32(self.head_shot2);
        w.i32(self.dis_con);
        w.i32(self.tot_play_time);
        w.i32(self.disp_head_item);
        w.i32(self.disp_face_item);
        w.i32(self.disp_backpack);
        w.u8(self.look_for_clan);
        w.i32(self.clan_contribue);
        w.i32(self.clan_win);
        w.i32(self.clan_lose);
        w.i32(self.clan_kill);
        w.i32(self.clan_death);
        w.i32(self.clan_assist);
        w.i32(self.clan_headshot);
        w.i32(self.clan_discon);
        w.i32(self.clan_ranking_prev);
        w.i32(self.clan_ranking);
        w.i32(self.clan_ranking_monthly_prev);
        w.i32(self.clan_ranking_monthly);
        w.i32(self.level_mark_id);
        w.i32(self.title_mark_id);
        w.i32(self.disp_pouch);
        w.i32(self.disp_camouflage);
        w.i32(self.reserved1);
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SystemMsgParam {
    pub msg_code: i32,
    pub char_name: String,
    pub server_id: u16,
    pub channel_num: u16,
    pub char_level: u16,
    pub item_id: i32,
    pub point: i32,
}

impl SystemMsgParam {
    pub fn decode(r: &mut Reader) -> Result<Self> {
        Ok(Self {
            msg_code: r.i32()?,
            char_name: r.string()?,
            server_id: r.u16()?,
            channel_num: r.u16()?,
            char_level: r.u16()?,
            item_id: r.i32()?,
            point: r.i32()?,
        })
    }

    pub fn encode(&self, w: &mut Writer) {
        w.i32(self.msg_code);
        w.string(&self.char_name);
        w.u16(self.server_id);
        w.u16(self.channel_num);
        w.u16(self.char_level);
        w.i32(self.item_id);
        w.i32(self.point);
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClanInfo {
    pub clan_master_charname: String,
    pub clan_win: i32,
    pub clan_lose: i32,
    pub clan_draw: i32,
    pub t_rank_cur: i32,
    pub t_rank_prev: i32,
    pub m_rank_cur: i32,
    pub m_rank_prev: i32,
    pub region: u8,
    pub member_cnt: i32,
    pub member_total: i32,
    pub is_recruit: u8,
    pub how_old: String,
    pub keyword: String,
    pub intro: String,
    pub cm_clan_name: String,
    pub cm_pattern: i32,
    pub cm_bg: i32,
    pub cm_bl: i32,
    pub clan_noti: String,
    pub clan_exp: i32,
    pub clan_kill: i32,
    pub clan_death: i32,
    pub clan_assist: i32,
    pub clan_activity: i32,
    pub clan_activity_rank: i32,
    pub clan_activity_rank_prev: i32,
    pub clan_activity_m_rank: i32,
    pub clan_activity_m_rank_prev: i32,
    pub clan_level: i32,
}

impl ClanInfo {
    pub fn decode(r: &mut Reader) -> Result<Self> {
        Ok(Self {
            clan_master_charname: r.string()?,
            clan_win: r.i32()?,
            clan_lose: r.i32()?,
            clan_draw: r.i32()?,
            t_rank_cur: r.i32()?,
            t_rank_prev: r.i32()?,
            m_rank_cur: r.i32()?,
            m_rank_prev: r.i32()?,
            region: r.u8()?,
            member_cnt: r.i32()?,
            member_total: r.i32()?,
            is_recruit: r.u8()?,
            how_old: r.string()?,
            keyword: r.string()?,
            intro: r.string()?,
            cm_clan_name: r.string()?,
            cm_pattern: r.i32()?,
            cm_bg: r.i32()?,
            cm_bl: r.i32()?,
            clan_noti: r.string()?,
            clan_exp: r.i32()?,
            clan_kill: r.i32()?,
            clan_death: r.i32()?,
            clan_assist: r.i32()?,
            clan_activity: r.i32()?,
            clan_activity_rank: r.i32()?,
            clan_activity_rank_prev: r.i32()?,
            clan_activity_m_rank: r.i32()?,
            clan_activity_m_rank_prev: r.i32()?,
            clan_level: r.i32()?,
        })
    }

    pub fn encode(&self, w: &mut Writer) {
        w.string(&self.clan_master_charname);
        w.i32(self.clan_win);
        w.i32(self.clan_lose);
        w.i32(self.clan_draw);
        w.i32(self.t_rank_cur);
        w.i32(self.t_rank_prev);
        w.i32(self.m_rank_cur);
        w.i32(self.m_rank_prev);
        w.u8(self.region);
        w.i32(self.member_cnt);
        w.i32(self.member_total);
        w.u8(self.is_recruit);
        w.string(&self.how_old);
        w.string(&self.keyword);
        w.string(&self.intro);
        w.string(&self.cm_clan_name);
        w.i32(self.cm_pattern);
        w.i32(self.cm_bg);
        w.i32(self.cm_bl);
        w.string(&self.clan_noti);
        w.i32(self.clan_exp);
        w.i32(self.clan_kill);
        w.i32(self.clan_death);
        w.i32(self.clan_assist);
        w.i32(self.clan_activity);
        w.i32(self.clan_activity_rank);
        w.i32(self.clan_activity_rank_prev);
        w.i32(self.clan_activity_m_rank);
        w.i32(self.clan_activity_m_rank_prev);
        w.i32(self.clan_level);
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KillDeathLog {
    pub char_kill_uid: i32,
    pub kill_x_coord: f32,
    pub kill_y_coord: f32,
    pub kill_z_coord: f32,
    pub kill_pos: i32,
    pub kill_weapon: i32,
    pub kill_weapon_part: i32,
    pub kill_range: i32,
    pub kill_damage: i32,
    pub kill_body_part: u8,
    pub is_uav: u8,
    pub char_death_uid: i32,
    pub death_x_coord: f32,
    pub death_y_coord: f32,
    pub death_z_coord: f32,
    pub death_pos: i32,
    pub death_weapon: i32,
    pub death_weapon_part: i32,
    pub is_drop_item: u8,
    pub drop_item_id: i32,
    pub get_item_id_after_kill: i32,
}

impl KillDeathLog {
    pub fn decode(r: &mut Reader) -> Result<Self> {
        Ok(Self {
            char_kill_uid: r.i32()?,
            kill_x_coord: r.f32()?,
            kill_y_coord: r.f32()?,
            kill_z_coord: r.f32()?,
            kill_pos: r.i32()?,
            kill_weapon: r.i32()?,
            kill_weapon_part: r.i32()?,
            kill_range: r.i32()?,
            kill_damage: r.i32()?,
            kill_body_part: r.u8()?,
            is_uav: r.u8()?,
            char_death_uid: r.i32()?,
            death_x_coord: r.f32()?,
            death_y_coord: r.f32()?,
            death_z_coord: r.f32()?,
            death_pos: r.i32()?,
            death_weapon: r.i32()?,
            death_weapon_part: r.i32()?,
            is_drop_item: r.u8()?,
            drop_item_id: r.i32()?,
            get_item_id_after_kill: r.i32()?,
        })
    }

    pub fn encode(&self, w: &mut Writer) {
        w.i32(self.char_kill_uid);
        w.f32(self.kill_x_coord);
        w.f32(self.kill_y_coord);
        w.f32(self.kill_z_coord);
        w.i32(self.kill_pos);
        w.i32(self.kill_weapon);
        w.i32(self.kill_weapon_part);
        w.i32(self.kill_range);
        w.i32(self.kill_damage);
        w.u8(self.kill_body_part);
        w.u8(self.is_uav);
        w.i32(self.char_death_uid);
        w.f32(self.death_x_coord);
        w.f32(self.death_y_coord);
        w.f32(self.death_z_coord);
        w.i32(self.death_pos);
        w.i32(self.death_weapon);
        w.i32(self.death_weapon_part);
        w.u8(self.is_drop_item);
        w.i32(self.drop_item_id);
        w.i32(self.get_item_id_after_kill);
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchState {
    pub i_num_team: i32,
    pub i_goal_round: i32,
    pub b_friendly_fire: u8,
    pub b_allow_spectate: u8,
    pub b_autobalance: u8,
    pub b_allow_join: u8,
    pub b_allow3rd_person_view: u8,
    pub b_chat_victim_only: u8,
    pub i_minutes_kick_afk: i32,
    pub i_min_players: i32,
}

impl MatchState {
    pub fn decode(r: &mut Reader) -> Result<Self> {
        Ok(Self {
            i_num_team: r.i32()?,
            i_goal_round: r.i32()?,
            b_friendly_fire: r.u8()?,
            b_allow_spectate: r.u8()?,
            b_autobalance: r.u8()?,
            b_allow_join: r.u8()?,
            b_allow3rd_person_view: r.u8()?,
            b_chat_victim_only: r.u8()?,
            i_minutes_kick_afk: r.i32()?,
            i_min_players: r.i32()?,
        })
    }

    pub fn encode(&self, w: &mut Writer) {
        w.i32(self.i_num_team);
        w.i32(self.i_goal_round);
        w.u8(self.b_friendly_fire);
        w.u8(self.b_allow_spectate);
        w.u8(self.b_autobalance);
        w.u8(self.b_allow_join);
        w.u8(self.b_allow3rd_person_view);
        w.u8(self.b_chat_victim_only);
        w.i32(self.i_minutes_kick_afk);
        w.i32(self.i_min_players);
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NetAuctionItem {
    pub unique_id: i32,
    pub item_id: i32,
    pub add_parts_id: i32,
    pub until_time: i32,
    pub bidding_price: i32,
    pub instantly_buying_price: i32,
    pub item_damaged: i32,
    pub highest_bidder_name: String,
    pub seller_name: String,
    pub guaranty_point: i32,
}

impl NetAuctionItem {
    pub fn decode(r: &mut Reader) -> Result<Self> {
        Ok(Self {
            unique_id: r.i32()?,
            item_id: r.i32()?,
            add_parts_id: r.i32()?,
            until_time: r.i32()?,
            bidding_price: r.i32()?,
            instantly_buying_price: r.i32()?,
            item_damaged: r.i32()?,
            highest_bidder_name: r.string()?,
            seller_name: r.string()?,
            guaranty_point: r.i32()?,
        })
    }

    pub fn encode(&self, w: &mut Writer) {
        w.i32(self.unique_id);
        w.i32(self.item_id);
        w.i32(self.add_parts_id);
        w.i32(self.until_time);
        w.i32(self.bidding_price);
        w.i32(self.instantly_buying_price);
        w.i32(self.item_damaged);
        w.string(&self.highest_bidder_name);
        w.string(&self.seller_name);
        w.i32(self.guaranty_point);
    }
}

// ---------- Login link ----------

pub mod login {
    use super::*;

    /// Every RPC on this link, indexed by opcode.
    pub const OPCODES: &[(u32, &str)] = &[
        (0, "rfReqHash"),
        (1, "rfAckHash"),
        (2, "rfAckExceedMaxUser"),
        (3, "rfReqLogin"),
        (4, "rfAckLogin"),
        (5, "rfReqHeadItemTypeList"),
        (6, "rfAckHeadItemTypeList"),
        (7, "rfReqHelmetItemTypeList"),
        (8, "rfAckHelmetItemTypeList"),
        (9, "rfReqAccesItemTypeList"),
        (10, "rfAckAccesItemTypeList"),
        (11, "rfReqCreateChar"),
        (12, "rfAckCreateChar"),
        (13, "rfReqServerList"),
        (14, "rfAckServerList"),
        (15, "rfReqConnectChannelServer"),
        (16, "rfAckConnectChannelServer"),
        (17, "rfReqFriendsList"),
        (18, "rfAckFriendsListStart"),
        (19, "rfAckFriendsList"),
        (20, "rfAckFriendsListEnd"),
        (21, "rfReqUserDetailInfo"),
        (22, "rfAckUserDetailInfo"),
        (23, "rfReqAdvertisement"),
        (24, "rfAckAdvertisement"),
        (25, "rfReqLoginInHouseTest"),
        (26, "rfAckLoginInHouseTest"),
        (27, "rfReqProcessPreLoginUser"),
        (28, "rfAckProcessPreLoginUser"),
        (29, "rfReqLoginWithWZAuth"),
        (30, "rfAckLoginWithWZAuth"),
        (31, "rfReqClanMemberListFromClient"),
        (32, "rfAckClanMemberListFromClientStart"),
        (33, "rfAckClanMemberListFromClient"),
        (34, "rfAckClanMemberListFromClientEnd"),
        (35, "rfAckMaxUserExceed"),
        (36, "rfReqKeepAlive"),
        (37, "rfReqForceLogoutByOwner"),
        (38, "rfReqCharInfo"),
        (39, "rfAckCharInfo"),
        (40, "rfReqRegisteredPCBangInfo"),
        (41, "rfAckRegisteredPCBangInfo"),
        (42, "rfReqPCBangUserList"),
        (43, "rfAckPCBangUserListStart"),
        (44, "rfAckPCBangUserList"),
        (45, "rfAckPCBangUserListEnd"),
        (46, "rfReqRegisterMainPCBang"),
        (47, "rfAckRegisterMainPCBang"),
        (48, "rfReqGetCharBlockInfo"),
        (49, "rfAckGetCharBlockInfo"),
        (50, "rfReqServerTime"),
        (51, "rfAckServerTime"),
        (52, "rfReqClientChallenge"),
        (53, "rfAckClientChallenge"),
        (54, "rfReqClientChecksum"),
        (55, "rfAckClientChecksum"),
        (56, "rfReqErrorReportServerAddr"),
        (57, "rfAckErrorReportServerAddr"),
        (58, "rfReqEventNotify"),
        (59, "rfAckEventNotify"),
        (60, "rfAckTencent_AASInfo"),
        (61, "rfReqLoginWithTencentAuth"),
        (62, "rfAckLoginWithTencentAuth"),
        (63, "rfReqLogOut"),
        (64, "rfAckLogOut"),
        (65, "rfReqLoginWithThailand"),
        (66, "rfAckLoginWithThailand"),
        (67, "rfReqLoginWithGlobal"),
        (68, "rfAckLoginWithGlobal"),
        (69, "rfReqLoginWithIndonesia"),
        (70, "rfAckLoginWithIndonesia"),
    ];

    pub fn opcode_name(op: u32) -> Option<&'static str> {
        OPCODES.iter().find(|(o, _)| *o == op).map(|(_, n)| *n)
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqHash {
        pub hash_field0: i32,
        pub hash_field1: i32,
        pub hash_field2: i32,
        pub hash_field3: i32,
    }

    impl ReqHash {
        pub const OPCODE: u32 = 0;
        pub const NAME: &'static str = "rfReqHash";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                hash_field0: r.i32()?,
                hash_field1: r.i32()?,
                hash_field2: r.i32()?,
                hash_field3: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.hash_field0);
            w.i32(self.hash_field1);
            w.i32(self.hash_field2);
            w.i32(self.hash_field3);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckHash {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckHash {
        pub const OPCODE: u32 = 1;
        pub const NAME: &'static str = "rfAckHash";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckExceedMaxUser;

    impl AckExceedMaxUser {
        pub const OPCODE: u32 = 2;
        pub const NAME: &'static str = "rfAckExceedMaxUser";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLogin {
        pub portal_id: String,
    }

    impl ReqLogin {
        pub const OPCODE: u32 = 3;
        pub const NAME: &'static str = "rfReqLogin";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                portal_id: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.portal_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLogin {
        pub sucess: u8,
        pub char_name: String,
        pub time_stamp: i32,
        pub login_hash1: i32,
        pub login_hash2: i32,
        pub login_hash3: i32,
        pub login_hash4: i32,
        pub login_hash5: i32,
        pub need_char: u8,
        pub err_msg: String,
    }

    impl AckLogin {
        pub const OPCODE: u32 = 4;
        pub const NAME: &'static str = "rfAckLogin";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                sucess: r.u8()?,
                char_name: r.string()?,
                time_stamp: r.i32()?,
                login_hash1: r.i32()?,
                login_hash2: r.i32()?,
                login_hash3: r.i32()?,
                login_hash4: r.i32()?,
                login_hash5: r.i32()?,
                need_char: r.u8()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.sucess);
            w.string(&self.char_name);
            w.i32(self.time_stamp);
            w.i32(self.login_hash1);
            w.i32(self.login_hash2);
            w.i32(self.login_hash3);
            w.i32(self.login_hash4);
            w.i32(self.login_hash5);
            w.u8(self.need_char);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqHeadItemTypeList;

    impl ReqHeadItemTypeList {
        pub const OPCODE: u32 = 5;
        pub const NAME: &'static str = "rfReqHeadItemTypeList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckHeadItemTypeList {
        pub head_item_id: Vec<i32>,
    }

    impl AckHeadItemTypeList {
        pub const OPCODE: u32 = 6;
        pub const NAME: &'static str = "rfAckHeadItemTypeList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                head_item_id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.head_item_id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqHelmetItemTypeList;

    impl ReqHelmetItemTypeList {
        pub const OPCODE: u32 = 7;
        pub const NAME: &'static str = "rfReqHelmetItemTypeList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckHelmetItemTypeList {
        pub helmet_item_name: Vec<String>,
        pub helmet_item_id: Vec<i32>,
    }

    impl AckHelmetItemTypeList {
        pub const OPCODE: u32 = 8;
        pub const NAME: &'static str = "rfAckHelmetItemTypeList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                helmet_item_name: r.array(|r| r.string())?,
                helmet_item_id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.helmet_item_name, |w, v| { w.string(v); });
            w.array(&self.helmet_item_id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqAccesItemTypeList;

    impl ReqAccesItemTypeList {
        pub const OPCODE: u32 = 9;
        pub const NAME: &'static str = "rfReqAccesItemTypeList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAccesItemTypeList {
        pub acceso_item_name: Vec<String>,
        pub acceso_item_id: Vec<i32>,
    }

    impl AckAccesItemTypeList {
        pub const OPCODE: u32 = 10;
        pub const NAME: &'static str = "rfAckAccesItemTypeList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                acceso_item_name: r.array(|r| r.string())?,
                acceso_item_id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.acceso_item_name, |w, v| { w.string(v); });
            w.array(&self.acceso_item_id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqCreateChar {
        pub char_name: String,
        pub head_item_id: i32,
        pub helmet_item_id: i32,
        pub acceso_item_id: i32,
        pub backpack_item_id: i32,
        pub pouch_item_id: i32,
        pub camouflage_item_id: i32,
    }

    impl ReqCreateChar {
        pub const OPCODE: u32 = 11;
        pub const NAME: &'static str = "rfReqCreateChar";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
                head_item_id: r.i32()?,
                helmet_item_id: r.i32()?,
                acceso_item_id: r.i32()?,
                backpack_item_id: r.i32()?,
                pouch_item_id: r.i32()?,
                camouflage_item_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            w.i32(self.head_item_id);
            w.i32(self.helmet_item_id);
            w.i32(self.acceso_item_id);
            w.i32(self.backpack_item_id);
            w.i32(self.pouch_item_id);
            w.i32(self.camouflage_item_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckCreateChar {
        pub success: i32,
        pub err_msg: String,
    }

    impl AckCreateChar {
        pub const OPCODE: u32 = 12;
        pub const NAME: &'static str = "rfAckCreateChar";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                success: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.success);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqServerList;

    impl ReqServerList {
        pub const OPCODE: u32 = 13;
        pub const NAME: &'static str = "rfReqServerList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckServerList {
        pub c_servername: Vec<String>,
        pub c_server_short_name: Vec<String>,
        pub server_ip: Vec<String>,
        pub server_port: Vec<i32>,
        pub channel_name: Vec<String>,
        pub channel_nick_name: Vec<String>,
        pub channel_num: Vec<i32>,
        pub ch_max_user: Vec<i32>,
        pub ch_cur_user: Vec<i32>,
        pub server_group_name: Vec<String>,
        pub server_group_id: Vec<i32>,
        pub server_disp_order: Vec<i32>,
        pub data_port: Vec<i32>,
        pub option_count_byte: Vec<u8>,
        pub option_key_byte: Vec<u8>,
        pub option_value_byte: Vec<u8>,
        pub option_count_int: Vec<u8>,
        pub option_key_int: Vec<u8>,
        pub option_value_int: Vec<i32>,
        pub option_count_float: Vec<u8>,
        pub option_key_float: Vec<u8>,
        pub option_value_float: Vec<f32>,
        pub is_clan_channel: Vec<u8>,
        pub is_use_dedicate_host: Vec<u8>,
    }

    impl AckServerList {
        pub const OPCODE: u32 = 14;
        pub const NAME: &'static str = "rfAckServerList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                c_servername: r.array(|r| r.string())?,
                c_server_short_name: r.array(|r| r.string())?,
                server_ip: r.array(|r| r.string())?,
                server_port: r.array(|r| r.i32())?,
                channel_name: r.array(|r| r.string())?,
                channel_nick_name: r.array(|r| r.string())?,
                channel_num: r.array(|r| r.i32())?,
                ch_max_user: r.array(|r| r.i32())?,
                ch_cur_user: r.array(|r| r.i32())?,
                server_group_name: r.array(|r| r.string())?,
                server_group_id: r.array(|r| r.i32())?,
                server_disp_order: r.array(|r| r.i32())?,
                data_port: r.array(|r| r.i32())?,
                option_count_byte: r.array(|r| r.u8())?,
                option_key_byte: r.array(|r| r.u8())?,
                option_value_byte: r.array(|r| r.u8())?,
                option_count_int: r.array(|r| r.u8())?,
                option_key_int: r.array(|r| r.u8())?,
                option_value_int: r.array(|r| r.i32())?,
                option_count_float: r.array(|r| r.u8())?,
                option_key_float: r.array(|r| r.u8())?,
                option_value_float: r.array(|r| r.f32())?,
                is_clan_channel: r.array(|r| r.u8())?,
                is_use_dedicate_host: r.array(|r| r.u8())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.c_servername, |w, v| { w.string(v); });
            w.array(&self.c_server_short_name, |w, v| { w.string(v); });
            w.array(&self.server_ip, |w, v| { w.string(v); });
            w.array(&self.server_port, |w, v| { w.i32(*v); });
            w.array(&self.channel_name, |w, v| { w.string(v); });
            w.array(&self.channel_nick_name, |w, v| { w.string(v); });
            w.array(&self.channel_num, |w, v| { w.i32(*v); });
            w.array(&self.ch_max_user, |w, v| { w.i32(*v); });
            w.array(&self.ch_cur_user, |w, v| { w.i32(*v); });
            w.array(&self.server_group_name, |w, v| { w.string(v); });
            w.array(&self.server_group_id, |w, v| { w.i32(*v); });
            w.array(&self.server_disp_order, |w, v| { w.i32(*v); });
            w.array(&self.data_port, |w, v| { w.i32(*v); });
            w.array(&self.option_count_byte, |w, v| { w.u8(*v); });
            w.array(&self.option_key_byte, |w, v| { w.u8(*v); });
            w.array(&self.option_value_byte, |w, v| { w.u8(*v); });
            w.array(&self.option_count_int, |w, v| { w.u8(*v); });
            w.array(&self.option_key_int, |w, v| { w.u8(*v); });
            w.array(&self.option_value_int, |w, v| { w.i32(*v); });
            w.array(&self.option_count_float, |w, v| { w.u8(*v); });
            w.array(&self.option_key_float, |w, v| { w.u8(*v); });
            w.array(&self.option_value_float, |w, v| { w.f32(*v); });
            w.array(&self.is_clan_channel, |w, v| { w.u8(*v); });
            w.array(&self.is_use_dedicate_host, |w, v| { w.u8(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqConnectChannelServer {
        pub server_ip: String,
        pub server_port: i32,
    }

    impl ReqConnectChannelServer {
        pub const OPCODE: u32 = 15;
        pub const NAME: &'static str = "rfReqConnectChannelServer";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                server_ip: r.string()?,
                server_port: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.server_ip);
            w.i32(self.server_port);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckConnectChannelServer {
        pub session_key: String,
        pub time_stamp: i32,
    }

    impl AckConnectChannelServer {
        pub const OPCODE: u32 = 16;
        pub const NAME: &'static str = "rfAckConnectChannelServer";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                session_key: r.string()?,
                time_stamp: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.session_key);
            w.i32(self.time_stamp);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqFriendsList;

    impl ReqFriendsList {
        pub const OPCODE: u32 = 17;
        pub const NAME: &'static str = "rfReqFriendsList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckFriendsListStart {
        pub result: i32,
        pub total_count: i32,
    }

    impl AckFriendsListStart {
        pub const OPCODE: u32 = 18;
        pub const NAME: &'static str = "rfAckFriendsListStart";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                total_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.total_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckFriendsList {
        pub list_type: u8,
        pub user_name: Vec<String>,
        pub is_log_on: Vec<u8>,
        pub c_mark_pattern: Vec<i32>,
        pub c_mark_bg: Vec<i32>,
        pub c_mark_bl: Vec<i32>,
        pub clan_name: Vec<String>,
        pub level: Vec<i32>,
    }

    impl AckFriendsList {
        pub const OPCODE: u32 = 19;
        pub const NAME: &'static str = "rfAckFriendsList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                list_type: r.u8()?,
                user_name: r.array(|r| r.string())?,
                is_log_on: r.array(|r| r.u8())?,
                c_mark_pattern: r.array(|r| r.i32())?,
                c_mark_bg: r.array(|r| r.i32())?,
                c_mark_bl: r.array(|r| r.i32())?,
                clan_name: r.array(|r| r.string())?,
                level: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.list_type);
            w.array(&self.user_name, |w, v| { w.string(v); });
            w.array(&self.is_log_on, |w, v| { w.u8(*v); });
            w.array(&self.c_mark_pattern, |w, v| { w.i32(*v); });
            w.array(&self.c_mark_bg, |w, v| { w.i32(*v); });
            w.array(&self.c_mark_bl, |w, v| { w.i32(*v); });
            w.array(&self.clan_name, |w, v| { w.string(v); });
            w.array(&self.level, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckFriendsListEnd;

    impl AckFriendsListEnd {
        pub const OPCODE: u32 = 20;
        pub const NAME: &'static str = "rfAckFriendsListEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqUserDetailInfo {
        pub user_id: i32,
        pub user_name: String,
    }

    impl ReqUserDetailInfo {
        pub const OPCODE: u32 = 21;
        pub const NAME: &'static str = "rfReqUserDetailInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
                user_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            w.string(&self.user_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckUserDetailInfo {
        pub result: i32,
        pub user_name: String,
        pub clan_name: String,
        pub ranking: i32,
        pub match_num: i32,
        pub win_num: i32,
        pub lose_num: i32,
        pub current_exp: i32,
        pub next_exp: i32,
        pub forced_disconnected_num: i32,
        pub primary_weapon_id: i32,
        pub sub_weapon_id: i32,
        pub point: i32,
    }

    impl AckUserDetailInfo {
        pub const OPCODE: u32 = 22;
        pub const NAME: &'static str = "rfAckUserDetailInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                user_name: r.string()?,
                clan_name: r.string()?,
                ranking: r.i32()?,
                match_num: r.i32()?,
                win_num: r.i32()?,
                lose_num: r.i32()?,
                current_exp: r.i32()?,
                next_exp: r.i32()?,
                forced_disconnected_num: r.i32()?,
                primary_weapon_id: r.i32()?,
                sub_weapon_id: r.i32()?,
                point: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.user_name);
            w.string(&self.clan_name);
            w.i32(self.ranking);
            w.i32(self.match_num);
            w.i32(self.win_num);
            w.i32(self.lose_num);
            w.i32(self.current_exp);
            w.i32(self.next_exp);
            w.i32(self.forced_disconnected_num);
            w.i32(self.primary_weapon_id);
            w.i32(self.sub_weapon_id);
            w.i32(self.point);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqAdvertisement;

    impl ReqAdvertisement {
        pub const OPCODE: u32 = 23;
        pub const NAME: &'static str = "rfReqAdvertisement";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAdvertisement {
        pub result: i32,
        pub type_: u8,
        pub address: String,
    }

    impl AckAdvertisement {
        pub const OPCODE: u32 = 24;
        pub const NAME: &'static str = "rfAckAdvertisement";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                type_: r.u8()?,
                address: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u8(self.type_);
            w.string(&self.address);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLoginInHouseTest {
        pub user_name: String,
        pub password: String,
        pub guid: String,
    }

    impl ReqLoginInHouseTest {
        pub const OPCODE: u32 = 25;
        pub const NAME: &'static str = "rfReqLoginInHouseTest";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_name: r.string()?,
                password: r.string()?,
                guid: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.user_name);
            w.string(&self.password);
            w.string(&self.guid);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLoginInHouseTest {
        pub sucess: u8,
        pub char_name: String,
        pub time_stamp: i32,
        pub login_hash1: i32,
        pub login_hash2: i32,
        pub login_hash3: i32,
        pub login_hash4: i32,
        pub login_hash5: i32,
        pub need_char: u8,
        pub err_msg: String,
        pub b_day: String,
        pub sex: u8,
        pub server_time: f64,
    }

    impl AckLoginInHouseTest {
        pub const OPCODE: u32 = 26;
        pub const NAME: &'static str = "rfAckLoginInHouseTest";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                sucess: r.u8()?,
                char_name: r.string()?,
                time_stamp: r.i32()?,
                login_hash1: r.i32()?,
                login_hash2: r.i32()?,
                login_hash3: r.i32()?,
                login_hash4: r.i32()?,
                login_hash5: r.i32()?,
                need_char: r.u8()?,
                err_msg: r.string()?,
                b_day: r.string()?,
                sex: r.u8()?,
                server_time: r.f64()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.sucess);
            w.string(&self.char_name);
            w.i32(self.time_stamp);
            w.i32(self.login_hash1);
            w.i32(self.login_hash2);
            w.i32(self.login_hash3);
            w.i32(self.login_hash4);
            w.i32(self.login_hash5);
            w.u8(self.need_char);
            w.string(&self.err_msg);
            w.string(&self.b_day);
            w.u8(self.sex);
            w.f64(self.server_time);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqProcessPreLoginUser {
        pub char_name: String,
        pub type_: u8,
    }

    impl ReqProcessPreLoginUser {
        pub const OPCODE: u32 = 27;
        pub const NAME: &'static str = "rfReqProcessPreLoginUser";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
                type_: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            w.u8(self.type_);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckProcessPreLoginUser {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckProcessPreLoginUser {
        pub const OPCODE: u32 = 28;
        pub const NAME: &'static str = "rfAckProcessPreLoginUser";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLoginWithWZAuth {
        pub account_id: String,
        pub guid: String,
        pub auth_key: String,
    }

    impl ReqLoginWithWZAuth {
        pub const OPCODE: u32 = 29;
        pub const NAME: &'static str = "rfReqLoginWithWZAuth";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                account_id: r.string()?,
                guid: r.string()?,
                auth_key: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.account_id);
            w.string(&self.guid);
            w.string(&self.auth_key);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLoginWithWZAuth {
        pub result: i32,
        pub wzp_account_type: u8,
        pub wzp_is_block: u8,
        pub wzp_auth_key_success: u8,
        pub channel_type: u8,
    }

    impl AckLoginWithWZAuth {
        pub const OPCODE: u32 = 30;
        pub const NAME: &'static str = "rfAckLoginWithWZAuth";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                wzp_account_type: r.u8()?,
                wzp_is_block: r.u8()?,
                wzp_auth_key_success: r.u8()?,
                channel_type: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u8(self.wzp_account_type);
            w.u8(self.wzp_is_block);
            w.u8(self.wzp_auth_key_success);
            w.u8(self.channel_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMemberListFromClient;

    impl ReqClanMemberListFromClient {
        pub const OPCODE: u32 = 31;
        pub const NAME: &'static str = "rfReqClanMemberListFromClient";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMemberListFromClientStart {
        pub result: i32,
        pub total_count: i32,
    }

    impl AckClanMemberListFromClientStart {
        pub const OPCODE: u32 = 32;
        pub const NAME: &'static str = "rfAckClanMemberListFromClientStart";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                total_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.total_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMemberListFromClient {
        pub result: i32,
        pub err_msg: String,
        pub level: Vec<i32>,
        pub char_name: Vec<String>,
        pub clan_grade: Vec<u8>,
        pub server_name: Vec<String>,
        pub server_short_name: Vec<String>,
        pub server_ip: Vec<String>,
        pub server_port: Vec<i32>,
        pub cur_pos: Vec<u8>,
        pub channel_num: Vec<i32>,
        pub channel_nick_name: Vec<String>,
        pub game_room_num: Vec<i32>,
    }

    impl AckClanMemberListFromClient {
        pub const OPCODE: u32 = 33;
        pub const NAME: &'static str = "rfAckClanMemberListFromClient";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                level: r.array(|r| r.i32())?,
                char_name: r.array(|r| r.string())?,
                clan_grade: r.array(|r| r.u8())?,
                server_name: r.array(|r| r.string())?,
                server_short_name: r.array(|r| r.string())?,
                server_ip: r.array(|r| r.string())?,
                server_port: r.array(|r| r.i32())?,
                cur_pos: r.array(|r| r.u8())?,
                channel_num: r.array(|r| r.i32())?,
                channel_nick_name: r.array(|r| r.string())?,
                game_room_num: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.level, |w, v| { w.i32(*v); });
            w.array(&self.char_name, |w, v| { w.string(v); });
            w.array(&self.clan_grade, |w, v| { w.u8(*v); });
            w.array(&self.server_name, |w, v| { w.string(v); });
            w.array(&self.server_short_name, |w, v| { w.string(v); });
            w.array(&self.server_ip, |w, v| { w.string(v); });
            w.array(&self.server_port, |w, v| { w.i32(*v); });
            w.array(&self.cur_pos, |w, v| { w.u8(*v); });
            w.array(&self.channel_num, |w, v| { w.i32(*v); });
            w.array(&self.channel_nick_name, |w, v| { w.string(v); });
            w.array(&self.game_room_num, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMemberListFromClientEnd;

    impl AckClanMemberListFromClientEnd {
        pub const OPCODE: u32 = 34;
        pub const NAME: &'static str = "rfAckClanMemberListFromClientEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckMaxUserExceed;

    impl AckMaxUserExceed {
        pub const OPCODE: u32 = 35;
        pub const NAME: &'static str = "rfAckMaxUserExceed";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqKeepAlive;

    impl ReqKeepAlive {
        pub const OPCODE: u32 = 36;
        pub const NAME: &'static str = "rfReqKeepAlive";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqForceLogoutByOwner;

    impl ReqForceLogoutByOwner {
        pub const OPCODE: u32 = 37;
        pub const NAME: &'static str = "rfReqForceLogoutByOwner";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqCharInfo {
        pub char_name: String,
    }

    impl ReqCharInfo {
        pub const OPCODE: u32 = 38;
        pub const NAME: &'static str = "rfReqCharInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckCharInfo {
        pub result: i32,
        pub char_name: String,
        pub level: i32,
        pub exp: i32,
        pub clan_name: String,
    }

    impl AckCharInfo {
        pub const OPCODE: u32 = 39;
        pub const NAME: &'static str = "rfAckCharInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                char_name: r.string()?,
                level: r.i32()?,
                exp: r.i32()?,
                clan_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.char_name);
            w.i32(self.level);
            w.i32(self.exp);
            w.string(&self.clan_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRegisteredPCBangInfo;

    impl ReqRegisteredPCBangInfo {
        pub const OPCODE: u32 = 40;
        pub const NAME: &'static str = "rfReqRegisteredPCBangInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRegisteredPCBangInfo {
        pub result: i32,
        pub err_msg: String,
        pub is_registered_pc_bang: u8,
        pub is_main_pc_bang: u8,
        pub pc_bang_name: String,
    }

    impl AckRegisteredPCBangInfo {
        pub const OPCODE: u32 = 41;
        pub const NAME: &'static str = "rfAckRegisteredPCBangInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                is_registered_pc_bang: r.u8()?,
                is_main_pc_bang: r.u8()?,
                pc_bang_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.u8(self.is_registered_pc_bang);
            w.u8(self.is_main_pc_bang);
            w.string(&self.pc_bang_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqPCBangUserList;

    impl ReqPCBangUserList {
        pub const OPCODE: u32 = 42;
        pub const NAME: &'static str = "rfReqPCBangUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPCBangUserListStart {
        pub result: i32,
        pub total_count: i32,
    }

    impl AckPCBangUserListStart {
        pub const OPCODE: u32 = 43;
        pub const NAME: &'static str = "rfAckPCBangUserListStart";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                total_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.total_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPCBangUserList {
        pub friend_name: Vec<String>,
        pub is_log_on: Vec<u8>,
        pub clan_name: Vec<String>,
        pub cm_pattern: Vec<i32>,
        pub cm_bg: Vec<i32>,
        pub cm_bl: Vec<i32>,
        pub level: Vec<i32>,
        pub clan_grade: Vec<i32>,
        pub clan_win: Vec<i32>,
        pub clan_lose: Vec<i32>,
    }

    impl AckPCBangUserList {
        pub const OPCODE: u32 = 44;
        pub const NAME: &'static str = "rfAckPCBangUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                friend_name: r.array(|r| r.string())?,
                is_log_on: r.array(|r| r.u8())?,
                clan_name: r.array(|r| r.string())?,
                cm_pattern: r.array(|r| r.i32())?,
                cm_bg: r.array(|r| r.i32())?,
                cm_bl: r.array(|r| r.i32())?,
                level: r.array(|r| r.i32())?,
                clan_grade: r.array(|r| r.i32())?,
                clan_win: r.array(|r| r.i32())?,
                clan_lose: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.friend_name, |w, v| { w.string(v); });
            w.array(&self.is_log_on, |w, v| { w.u8(*v); });
            w.array(&self.clan_name, |w, v| { w.string(v); });
            w.array(&self.cm_pattern, |w, v| { w.i32(*v); });
            w.array(&self.cm_bg, |w, v| { w.i32(*v); });
            w.array(&self.cm_bl, |w, v| { w.i32(*v); });
            w.array(&self.level, |w, v| { w.i32(*v); });
            w.array(&self.clan_grade, |w, v| { w.i32(*v); });
            w.array(&self.clan_win, |w, v| { w.i32(*v); });
            w.array(&self.clan_lose, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPCBangUserListEnd;

    impl AckPCBangUserListEnd {
        pub const OPCODE: u32 = 45;
        pub const NAME: &'static str = "rfAckPCBangUserListEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRegisterMainPCBang;

    impl ReqRegisterMainPCBang {
        pub const OPCODE: u32 = 46;
        pub const NAME: &'static str = "rfReqRegisterMainPCBang";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRegisterMainPCBang {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckRegisterMainPCBang {
        pub const OPCODE: u32 = 47;
        pub const NAME: &'static str = "rfAckRegisterMainPCBang";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGetCharBlockInfo;

    impl ReqGetCharBlockInfo {
        pub const OPCODE: u32 = 48;
        pub const NAME: &'static str = "rfReqGetCharBlockInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGetCharBlockInfo {
        pub result: i32,
        pub is_account_block: u8,
        pub is_chatting_block: u8,
        pub chatting_block_end_time: f64,
        pub blocked_item_name: Vec<String>,
    }

    impl AckGetCharBlockInfo {
        pub const OPCODE: u32 = 49;
        pub const NAME: &'static str = "rfAckGetCharBlockInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                is_account_block: r.u8()?,
                is_chatting_block: r.u8()?,
                chatting_block_end_time: r.f64()?,
                blocked_item_name: r.array(|r| r.string())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u8(self.is_account_block);
            w.u8(self.is_chatting_block);
            w.f64(self.chatting_block_end_time);
            w.array(&self.blocked_item_name, |w, v| { w.string(v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqServerTime;

    impl ReqServerTime {
        pub const OPCODE: u32 = 50;
        pub const NAME: &'static str = "rfReqServerTime";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckServerTime {
        pub time: i32,
    }

    impl AckServerTime {
        pub const OPCODE: u32 = 51;
        pub const NAME: &'static str = "rfAckServerTime";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                time: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.time);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClientChallenge;

    impl ReqClientChallenge {
        pub const OPCODE: u32 = 52;
        pub const NAME: &'static str = "rfReqClientChallenge";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClientChallenge {
        pub key1: String,
        pub key2: f64,
    }

    impl AckClientChallenge {
        pub const OPCODE: u32 = 53;
        pub const NAME: &'static str = "rfAckClientChallenge";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                key1: r.string()?,
                key2: r.f64()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.key1);
            w.f64(self.key2);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClientChecksum {
        pub key1: String,
        pub key2: f64,
    }

    impl ReqClientChecksum {
        pub const OPCODE: u32 = 54;
        pub const NAME: &'static str = "rfReqClientChecksum";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                key1: r.string()?,
                key2: r.f64()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.key1);
            w.f64(self.key2);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClientChecksum {
        pub result: i32,
    }

    impl AckClientChecksum {
        pub const OPCODE: u32 = 55;
        pub const NAME: &'static str = "rfAckClientChecksum";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqErrorReportServerAddr;

    impl ReqErrorReportServerAddr {
        pub const OPCODE: u32 = 56;
        pub const NAME: &'static str = "rfReqErrorReportServerAddr";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckErrorReportServerAddr {
        pub addr: String,
        pub port: i32,
    }

    impl AckErrorReportServerAddr {
        pub const OPCODE: u32 = 57;
        pub const NAME: &'static str = "rfAckErrorReportServerAddr";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                addr: r.string()?,
                port: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.addr);
            w.i32(self.port);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqEventNotify;

    impl ReqEventNotify {
        pub const OPCODE: u32 = 58;
        pub const NAME: &'static str = "rfReqEventNotify";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckEventNotify {
        pub type_: i32,
    }

    impl AckEventNotify {
        pub const OPCODE: u32 = 59;
        pub const NAME: &'static str = "rfAckEventNotify";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                type_: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.type_);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckTencentAASInfo {
        pub user_id: i32,
        pub is_adult: u8,
        pub is_qq_auth: u8,
        pub accum_time_sec: i32,
    }

    impl AckTencentAASInfo {
        pub const OPCODE: u32 = 60;
        pub const NAME: &'static str = "rfAckTencent_AASInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
                is_adult: r.u8()?,
                is_qq_auth: r.u8()?,
                accum_time_sec: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            w.u8(self.is_adult);
            w.u8(self.is_qq_auth);
            w.i32(self.accum_time_sec);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLoginWithTencentAuth {
        pub account_id: String,
        pub signature: Vec<u8>,
    }

    impl ReqLoginWithTencentAuth {
        pub const OPCODE: u32 = 61;
        pub const NAME: &'static str = "rfReqLoginWithTencentAuth";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                account_id: r.string()?,
                signature: r.array(|r| r.u8())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.account_id);
            w.array(&self.signature, |w, v| { w.u8(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLoginWithTencentAuth {
        pub result: i32,
        pub tc_account_type: u8,
        pub tc_is_block: u8,
    }

    impl AckLoginWithTencentAuth {
        pub const OPCODE: u32 = 62;
        pub const NAME: &'static str = "rfAckLoginWithTencentAuth";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                tc_account_type: r.u8()?,
                tc_is_block: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u8(self.tc_account_type);
            w.u8(self.tc_is_block);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLogOut {
        pub reason: u8,
    }

    impl ReqLogOut {
        pub const OPCODE: u32 = 63;
        pub const NAME: &'static str = "rfReqLogOut";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                reason: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.reason);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLogOut {
        pub result: u8,
        pub reason: u8,
    }

    impl AckLogOut {
        pub const OPCODE: u32 = 64;
        pub const NAME: &'static str = "rfAckLogOut";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.u8()?,
                reason: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.result);
            w.u8(self.reason);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLoginWithThailand {
        pub usn: String,
        pub blood_display_type: i32,
        pub pc_bang_type: i32,
        pub unix_time_stamp: String,
        pub hash_value: String,
        pub sex: u8,
    }

    impl ReqLoginWithThailand {
        pub const OPCODE: u32 = 65;
        pub const NAME: &'static str = "rfReqLoginWithThailand";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                usn: r.string()?,
                blood_display_type: r.i32()?,
                pc_bang_type: r.i32()?,
                unix_time_stamp: r.string()?,
                hash_value: r.string()?,
                sex: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.usn);
            w.i32(self.blood_display_type);
            w.i32(self.pc_bang_type);
            w.string(&self.unix_time_stamp);
            w.string(&self.hash_value);
            w.u8(self.sex);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLoginWithThailand {
        pub result: i32,
    }

    impl AckLoginWithThailand {
        pub const OPCODE: u32 = 66;
        pub const NAME: &'static str = "rfAckLoginWithThailand";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLoginWithGlobal {
        pub g_account_id: String,
        pub g_guid: String,
        pub g_auth_key: String,
        pub user_type: i32,
    }

    impl ReqLoginWithGlobal {
        pub const OPCODE: u32 = 67;
        pub const NAME: &'static str = "rfReqLoginWithGlobal";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                g_account_id: r.string()?,
                g_guid: r.string()?,
                g_auth_key: r.string()?,
                user_type: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.g_account_id);
            w.string(&self.g_guid);
            w.string(&self.g_auth_key);
            w.i32(self.user_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLoginWithGlobal {
        pub result: i32,
    }

    impl AckLoginWithGlobal {
        pub const OPCODE: u32 = 68;
        pub const NAME: &'static str = "rfAckLoginWithGlobal";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLoginWithIndonesia {
        pub hash_value: String,
    }

    impl ReqLoginWithIndonesia {
        pub const OPCODE: u32 = 69;
        pub const NAME: &'static str = "rfReqLoginWithIndonesia";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                hash_value: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.hash_value);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLoginWithIndonesia {
        pub result: i32,
    }

    impl AckLoginWithIndonesia {
        pub const OPCODE: u32 = 70;
        pub const NAME: &'static str = "rfAckLoginWithIndonesia";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

}

// ---------- Channel link ----------

pub mod channel {
    use super::*;

    /// Every RPC on this link, indexed by opcode.
    pub const OPCODES: &[(u32, &str)] = &[
        (269, "rfReqGamePlay_InstallBomb"),
        (270, "rfReqGamePlay_DefuseBomb"),
        (271, "rfReqNotifyChangeRoomState"),
        (272, "rfReqGamePlay_TakeAPoint"),
        (273, "rfReqForceDisconnectUDPFromHost"),
        (274, "rfAckForceDisconnectUDPFromHost"),
        (275, "rfReqRecvForceDisconnectUDPFromHost"),
        (276, "rfReqGamePlay_SetGameStartTime"),
        (277, "rfReqGamePlay_SetRoundStartTime"),
        (278, "rfReqWriteServerLog"),
        (279, "rfReqUpdateMyHostPriorityPoint"),
        (280, "rfReqGetUserHostPriorityPoint"),
        (281, "rfAckGetUserHostPriorityPoint"),
        (282, "rfReqNotifyRelayServerAddr"),
        (283, "rfReqGamePlay_UpdateDomScore"),
        (284, "rfReqGamePlay_ChangeWeaponSet"),
        (285, "rfReqNotifyRecvDummyForHolePunching"),
        (286, "rfReqGetHolePunchingResult"),
        (287, "rfAckGetHolePunchingResult"),
        (288, "rfReqNotifyHostBlocking"),
        (289, "rfAckNotifyHostBlocking"),
        (290, "rfReqGetIsHostBlocking"),
        (291, "rfAckGetIsHostBlocking"),
        (292, "rfReqIsAvailClanURL"),
        (293, "rfAckIsAvailClanURL"),
        (294, "rfAckAcquirePromotionItems"),
        (295, "rfReqGamePlay_GetChopper"),
        (296, "rfReqCheckEquipedWeaponDurability"),
        (297, "rfAckCheckEquipedWeaponDurability"),
        (298, "rfReqGameGuardAuthData"),
        (299, "rfAckGameGuardAuthData"),
        (300, "rfReqDeleteFriend"),
        (301, "rfAckDeleteFriend"),
        (302, "rfReqRecvDeleteFriend"),
        (303, "rfReqChannelUserListInGameRoom"),
        (304, "rfAckChannelUserListInGameRoom"),
        (305, "rfReqSpecificGameRoomInfo"),
        (306, "rfAckSpecificGameRoomInfo"),
        (307, "rfReqSpecificChannelUserInfo"),
        (308, "rfAckSpecificChannelUserInfo"),
        (309, "rfReqNotice_GM"),
        (310, "rfAckNotice_GM"),
        (311, "rfReqRecvNotice_FromGM"),
        (312, "rfReqForceDisconnectUser_GM"),
        (313, "rfAckForceDisconnectUser_GM"),
        (314, "rfReqRecvForceDisconnected_FromGM"),
        (315, "rfReqNotifyDisconnectHostInLoading"),
        (316, "rfReqChatWispherMyCommunity"),
        (317, "rfAckChatWispherMyCommunity"),
        (318, "rfReqNotifyDurabilityZeroItem"),
        (319, "rfReqNotifyMyCharInfoToRoomUser"),
        (320, "rfAckNotifyMyCharInfoToRoomUser"),
        (321, "rfReqStartHolePunching"),
        (322, "rfAckStartHolePunching"),
        (323, "rfReqForceFailedClientLoading"),
        (324, "rfAckForceFailedClientLoading"),
        (325, "rfReqRecvForceFailedClientLoading"),
        (326, "rfReqItemListInItemBox"),
        (327, "rfAckItemListInItemBox"),
        (328, "rfReqGetItemFromItemBox"),
        (329, "rfAckGetItemFromItemBox"),
        (330, "rfAckAutoChangedTeamUserList"),
        (331, "rfReqClanMatch_GetReady"),
        (332, "rfAckClanMatch_GetReady"),
        (333, "rfReqClanMatch_StartCombat"),
        (334, "rfAckClanMatch_StartCombat"),
        (335, "rfAckClanMatch_NotifiyStartCombat"),
        (336, "rfReqClanMatch_InviteCrew"),
        (337, "rfAckClanMatch_InviteCrew"),
        (338, "rfReqClanMatch_RecvInviteCrew"),
        (339, "rfAckClanMatch_RecvInviteCrew"),
        (340, "rfReqClanMatch_InviteCrewResult"),
        (341, "rfReqClanMatch_Join"),
        (342, "rfAckClanMatch_Join"),
        (343, "rfAckClanMatch_JoinNotify"),
        (344, "rfReqClanMatch_MemberList"),
        (345, "rfAckClanMatch_MemberList"),
        (346, "rfReqClanMatch_Leave"),
        (347, "rfAckClanMatch_Leave"),
        (348, "rfAckClanMatch_LeaveNotify"),
        (349, "rfReqClanMatch_Chat"),
        (350, "rfAckClanMatch_Chat"),
        (351, "rfReqClanMatch_WaittingTeamList"),
        (352, "rfAckClanMatch_WaittingTeamList"),
        (353, "rfReqClanMatch_ChangeLeader"),
        (354, "rfAckClanMatch_ChangeLeader"),
        (355, "rfReqClanMatch_StopSearching"),
        (356, "rfAckClanMatch_StopSearching"),
        (357, "rfReqInviteGameRoom"),
        (358, "rfAckInviteGameRoom"),
        (359, "rfReqRecvInviteGameRoom"),
        (360, "rfAckRecvInviteGameRoom"),
        (361, "rfAckRecvInviteGameRoomResult"),
        (362, "rfReqLimitedListInChannel"),
        (363, "rfAckLimitedListInChannel"),
        (364, "rfReqClanMatch_StartSearching"),
        (365, "rfAckClanMatch_StartSearching"),
        (366, "rfAckClanMatch_NotifyStartSearching"),
        (367, "rfReqClanMatch_FoundMatch"),
        (368, "rfAckClanMatch_FoundMatch"),
        (369, "rfReqPermanentRoomInfo"),
        (370, "rfAckPermanentRoomInfo"),
        (371, "rfReqQuestList"),
        (372, "rfAckQuestListStart"),
        (373, "rfAckQuestList"),
        (374, "rfAckQuestListEnd"),
        (375, "rfReqQuestUpdate"),
        (376, "rfAckQuestUpdate"),
        (377, "rfReqQuestNewAvailable"),
        (378, "rfReqDailyQuestNewAvailable"),
        (379, "rfReqPCBangUserList"),
        (380, "rfAckPCBangUserListStart"),
        (381, "rfAckPCBangUserList"),
        (382, "rfAckPCBangUserListEnd"),
        (383, "rfReqRegisterMainPCBang"),
        (384, "rfAckRegisterMainPCBang"),
        (385, "rfReqRegisteredPCBangInfo"),
        (386, "rfAckRegisteredPCBangInfo"),
        (387, "rfReqWebzenCash"),
        (388, "rfAckWebzenCash"),
        (389, "rfReqWebzenShopScriptVersion"),
        (390, "rfAckWebzenShopScriptVersion"),
        (391, "rfReqBuyItemByWebzenCashGP20"),
        (392, "rfReqBuyItemByWebzenCash"),
        (393, "rfAckBuyItemByWebzenCash"),
        (394, "rfReqGiftItemByWebzenCashGP20"),
        (395, "rfReqGiftItemByWebzenCash"),
        (396, "rfAckGiftItemByWebzenCash"),
        (397, "rfReqWebzenBillingStorageList"),
        (398, "rfAckWebzenBillingStorageListStart"),
        (399, "rfAckWebzenBillingStorageList"),
        (400, "rfAckWebzenBillingStorageListEnd"),
        (401, "rfReqWebzenBillingUseStorageItem"),
        (402, "rfAckWebzenBillingUseStorageItem"),
        (403, "rfReqParamCheckSum"),
        (404, "rfAckParamCheckSum"),
        (405, "rfReqClanMatch_ChangeRoomSetting"),
        (406, "rfAckClanMatch_ChangeRoomSetting"),
        (407, "rfReqGetBotID"),
        (408, "rfAckGetBotID"),
        (409, "rfReqReturnBotID"),
        (410, "rfAckReturnBotID"),
        (411, "rfReqClanMatch_GetClanWaittingRoomInfo"),
        (412, "rfAckClanMatch_GetClanWaittingRoomInfo"),
        (413, "rfReqIsPaidWebzenPCBang"),
        (414, "rfReqCharBlockInfo"),
        (415, "rfAckCharBlockInfo"),
        (416, "rfReqQuestGetAvailableEventList"),
        (417, "rfAckQuestGetAvailableEventList"),
        (418, "rfAckGMEnterRoom"),
        (419, "rfAckGMLeaveRoom"),
        (420, "rfReqChangeRoomTitle_GM"),
        (421, "rfAckChangeRoomTitle_GM"),
        (422, "rfAckChangedRoomTitle_FromGM"),
        (423, "rfReqRandomBox_Use"),
        (424, "rfAckRandomBox_Use"),
        (425, "rfReqSendSystemMsg"),
        (426, "rfAckSendSystemMsg"),
        (427, "rfAckRecvSystemMsg"),
        (428, "rfReqPaidItem_ChangeCharname"),
        (429, "rfAckPaidItem_ChangeCharname"),
        (430, "rfReqPaidItem_EraseRecord"),
        (431, "rfAckPaidItem_EraseRecord"),
        (432, "rfReqPaidItem_BroadCast"),
        (433, "rfAckPaidItem_BroadCast"),
        (434, "rfAckPaidItem_RelayBroadcast"),
        (435, "rfReqPaidItem_ClanMarkDupCheck"),
        (436, "rfAckPaidItem_ClanMarkDupCheck"),
        (437, "rfReqPaidItem_ChangeClanMark"),
        (438, "rfAckPaidItem_ChangeClanMark"),
        (439, "rfAckPaidItem_NotifyChangeClanMark"),
        (440, "rfReqRemoveCheckTeamBalanceList"),
        (441, "rfReqClanMatch_SetWaitRoomPublic"),
        (442, "rfAckClanMatch_SetWaitRoomPublic"),
        (443, "rfReqExteriorRoomOwnerInfo"),
        (444, "rfAckExteriorRoomOwnerInfo"),
        (445, "rfReqSetLookForClan"),
        (446, "rfAckSetLookForClan"),
        (447, "rfReqGetMyMailBoxState"),
        (448, "rfAckGetMyMailBoxState"),
        (449, "rfReqMyEquipedItemList"),
        (450, "rfAckMyEquipedItemList"),
        (451, "rfReqMyEquipedSkillList"),
        (452, "rfAckMyEquipedSkillList"),
        (453, "rfReqMyEquipedQSlotList"),
        (454, "rfAckMyEquipedQSlotList"),
        (455, "rfReqRoomUserPingInfo"),
        (456, "rfAckRoomUserPingInfo"),
        (457, "rfReqNotifyStartTutorial"),
        (458, "rfAckNotifyStartTutorial"),
        (459, "rfReqNotifyEndTutorial"),
        (460, "rfAckNotifyEndTutorial"),
        (461, "rfReqServerShutdown"),
        (462, "rfAckServerShutdown"),
        (463, "rfReqServerType"),
        (464, "rfAckServerType"),
        (465, "rfReqClanMatch_GetPublicWaitRoomList"),
        (466, "rfAckClanMatch_StartPublicWaitRoomList"),
        (467, "rfAckClanMatch_PublicWaitRoomList"),
        (468, "rfAckClanMatch_EndPublicWaitRoomList"),
        (469, "rfReqPaidItem_ChangeClanname"),
        (470, "rfAckPaidItem_ChangeClanname"),
        (471, "rfAckPaidItem_NotifyChangeClanname"),
        (472, "rfReqEventNotify"),
        (473, "rfAckEventNotify"),
        (474, "rfAckClanMatch_PublicWaitRoomChanged"),
        (475, "rfReqDeleteMailAll"),
        (476, "rfAckDeleteMailAll"),
        (477, "rfReqClanMatchResultHistory"),
        (478, "rfAckClanMatchResultHistory"),
        (479, "rfReqInGameRoomUserInfoChanged"),
        (480, "rfReqInGameRoomUserClanLevelChanged"),
        (481, "rfReqCharDetailInfo"),
        (482, "rfAckCharDetailInfo"),
        (483, "rfReqGetRankingInfoList"),
        (484, "rfAckGetRankingInfoListStart"),
        (485, "rfAckGetRankingInfoList"),
        (486, "rfAckGetRankingInfoListEnd"),
        (487, "rfAckNotifyUpdateCharInfo_LevelMark"),
        (488, "rfAckNotifyUpdateCharInfo_TitleMark"),
        (489, "rfReqCommunityOptionList"),
        (490, "rfAckCommunityOptionList"),
        (491, "rfReqSetCommunityOption"),
        (492, "rfAckSetCommunityOption"),
        (493, "rfAckTencent_AASInfo"),
        (494, "rfReqLogOut"),
        (495, "rfAckLogOut"),
        (496, "rfReqDedicateHost_Login"),
        (497, "rfAckDedicateHost_Login"),
        (498, "rfReqFriendPosition"),
        (499, "rfAckFriendPosition"),
        (500, "rfReqSetClientVersion"),
        (501, "rfUpdateWebzenShopScriptVersion"),
        (502, "rfAckPopUpMessage"),
        (503, "rfAckMoveToLobby"),
        (504, "rfAckTencent_StartTenProtect"),
        (505, "rfReqGameMapData"),
        (506, "rfAckGameMapData"),
        (507, "rfNotifyNewNickName"),
        (508, "rfNotifyNewClanName"),
        (509, "rfNotifyClanLevelup"),
        (510, "rfReqAhnHsObject"),
        (511, "rfAckAhnHsObject"),
        (512, "rfReqAhnNotifyDisconnct"),
        (513, "rfReqUpdateUserPerformenceInfo"),
        (514, "rfReqNotifyLoadingStatus"),
        (515, "rfReqShopItemData"),
        (516, "rfAckShopItemData"),
        (517, "rfReqNotifyShopItemData"),
        (518, "rfReqNotifyShopItemDataEnd"),
        (519, "rfReqCouponUse"),
        (520, "rfAckCouponUse"),
        (521, "rfNotifyCouponUseError"),
        (522, "rfAckAIRewardResult"),
        (523, "rfReqAttendanceList"),
        (524, "rfAckAttendanceList"),
        (525, "rfReqPuzzleList"),
        (526, "rfAckPuzzleList"),
        (527, "rfReqDailyRewardListThisMonth"),
        (528, "rfAckDailyRewardListThisMonth"),
        (529, "rfReqReceiveSpecialReward"),
        (530, "rfAckReceiveSpecialReward"),
        (531, "rfReqSpecialQuestRank"),
        (532, "rfAckSpecialQuestRank"),
        (533, "rfReqLuckShop_LotteryWinnerList"),
        (534, "rfAckLuckShop_LotteryWinnerList"),
        (535, "rfReqLuckShop_StorageInfo"),
        (536, "rfAckLuckShop_StorageInfo"),
        (537, "rfReqLuckShop_RewardBuyingCount"),
        (538, "rfAckLuckShop_RewardBuyingCount"),
        (539, "rfReqLuckShop_RewardCoupon"),
        (540, "rfAckLuckShop_RewardCoupon"),
        (541, "rfChangeItemInstanceInfo"),
        (542, "rfReqUseItemDuringGame"),
        (543, "rfAckUseItemDuringGame"),
        (268, "rfAckDBCharInfo"),
        (267, "rfReqDBCharInfo"),
        (266, "rfReqForceNatTypeSetting"),
        (265, "rfAckGetServerTime"),
        (264, "rfReqGetServerTime"),
        (263, "rfReqLogEndEachRound"),
        (262, "rfReqLogGetScore"),
        (261, "rfAckAuctionTradeHistory"),
        (260, "rfReqAuctionTradeHistory"),
        (259, "rfAckSendGift"),
        (258, "rfReqSendGift"),
        (257, "rfAckGetAttachedItemFromMemo"),
        (256, "rfReqGetAttachedItemFromMemo"),
        (255, "rfAckNotifyNewClanMember"),
        (254, "rfAckChatClan"),
        (253, "rfReqChatClan"),
        (252, "rfAckTestDouble"),
        (251, "rfReqTestDouble"),
        (250, "rfAckNotifyLoadingState"),
        (249, "rfReqNotifyLoadingState"),
        (248, "rfAckFailedStartGame"),
        (247, "rfReqFailedStartGame"),
        (246, "rfAckSuccessStartGame"),
        (245, "rfReqSuccessStartGame"),
        (244, "rfAckClanCharPersonelIntro"),
        (243, "rfReqClanCharPersonelIntro"),
        (242, "rfAckClanPersonelIntro"),
        (241, "rfReqClanPersonelIntro"),
        (240, "rfAckClanChangeInfo"),
        (239, "rfReqClanChangeInfo"),
        (238, "rfAckClanClose"),
        (237, "rfReqClanClose"),
        (236, "rfAckNotifyClanSecession"),
        (235, "rfAckClanSecession"),
        (234, "rfReqClanSecession"),
        (233, "rfAckClanNews"),
        (232, "rfReqClanNews"),
        (231, "rfAckNotifyAdjustClanMemberGrade"),
        (230, "rfAckAdjustClanMemberGrade"),
        (229, "rfReqAdjustClanMemberGrade"),
        (228, "rfAckClanUserListEnd"),
        (227, "rfAckClanUserList"),
        (226, "rfAckClanUserListStart"),
        (225, "rfReqClanUserList"),
        (224, "rfAckSearchClan"),
        (223, "rfReqSerachClan"),
        (222, "rfAckJoinClanResult"),
        (221, "rfAckRecvJoinClanResult"),
        (220, "rfAckRecvJoinClan"),
        (219, "rfReqRecvJoinClan"),
        (218, "rfAckJoinClan"),
        (217, "rfReqJoinClan"),
        (216, "rfAckInviteClanResult"),
        (215, "rfAckRecvClanInviteResult"),
        (214, "rfAckRecvClanInvite"),
        (213, "rfReqRecvClanInvite"),
        (212, "rfAckInviteClan"),
        (211, "rfReqInviteClan"),
        (210, "rfAckClanInfo"),
        (209, "rfReqClanInfo"),
        (208, "rfAckCreateClan"),
        (207, "rfReqCreateClan"),
        (206, "rfAckDupCheckClanName"),
        (205, "rfReqDupCheckClanName"),
        (204, "rfAckSellShopItemInfo"),
        (203, "rfReqSellShopItemInfo"),
        (202, "rfAckGameOverEnd"),
        (201, "rfReqGameOverEnd"),
        (200, "rfAckAuctionItemListCount"),
        (199, "rfReqAuctionItemListCount"),
        (198, "rfAckServerTime"),
        (197, "rfReqServerTime"),
        (196, "rfAckEntryQSlotIDList"),
        (195, "rfReqEntryQSlotIDList"),
        (194, "rfAckEntryQSlotUniqueIDList"),
        (193, "rfReqEntryQSlotUniqueIDList"),
        (192, "rfAckAuctionBidding"),
        (191, "rfReqAuctionBidding"),
        (190, "rfAckAuctionInstantlyBuying"),
        (189, "rfReqAuctionInstantlyBuying"),
        (188, "rfAckAuctionItemListEnd"),
        (187, "rfAckAuctionItemList"),
        (186, "rfAckAuctionItemListStart"),
        (185, "rfReqAuctionItemList"),
        (184, "rfAckAuctionMyItemCancel"),
        (183, "rfReqAuctionMyItemCancel"),
        (182, "rfAckAuctionMyItemRegister"),
        (181, "rfReqAuctionMyItemRegister"),
        (180, "rfAckMyMoney"),
        (179, "rfReqMyMoney"),
        (178, "rfAckUpdateQSlot"),
        (177, "rfReqUpdateQSlot"),
        (176, "rfAckQSlotList"),
        (175, "rfReqQSlotList"),
        (174, "rfAckRepairItem"),
        (173, "rfReqRepairItem"),
        (172, "rfAckRepairCost"),
        (171, "rfReqRepairCost"),
        (170, "rfAckUpdateWeaponDurability"),
        (169, "rfReqUpdateWeaponDurability"),
        (168, "rfAckModifyWeapon"),
        (167, "rfReqModifyWeapon"),
        (166, "rfAckSellShopItem"),
        (165, "rfReqSellShopItem"),
        (164, "rfAckBuyShopItem"),
        (163, "rfReqBuyShopItem"),
        (162, "rfAckShopItemListEnd"),
        (161, "rfAckShopItemList"),
        (160, "rfAckShopItemListStart"),
        (159, "rfReqShopItemList"),
        (158, "rfAckForceGameOver"),
        (157, "rfReqForceGameOver"),
        (156, "rfAckGiveSkill"),
        (155, "rfReqGiveSkill"),
        (154, "rfAckNotifyExpiredItemIDs"),
        (153, "rfReqNotifyExpiredItemIDs"),
        (152, "rfAckNotifyExpiredSkillIDs"),
        (151, "rfReqNotifyExpiredSkillIDs"),
        (150, "rfAckNotifyChangedSkillState"),
        (149, "rfReqNotifyChangedSkillState"),
        (148, "rfAckNotifyChangedItemState"),
        (147, "rfReqNotifyChangedItemState"),
        (146, "rfAckConfirmSkillChanged"),
        (145, "rfReqConfirmSkillChanged"),
        (144, "rfAckEquipSkill"),
        (143, "rfReqEquipSkill"),
        (142, "rfAckUserSkillListEnd"),
        (141, "rfAckUserSkillList"),
        (140, "rfAckUserSkillListStart"),
        (139, "rfReqUserSkillList"),
        (138, "rfAckDelMemo"),
        (137, "rfReqDelMemo"),
        (136, "rfAckReadMemo"),
        (135, "rfReqReadMemo"),
        (134, "rfAckEndMemoList"),
        (133, "rfAckMemoList"),
        (132, "rfAckStartMemoList"),
        (131, "rfReqMemoList"),
        (130, "rfReqRecvMemo"),
        (129, "rfAckSendMemo"),
        (128, "rfReqSendMemo"),
        (127, "rfReqRecvChatWispher"),
        (126, "rfAckChatWispher"),
        (125, "rfReqChatWispher"),
        (124, "rfAckRemoveBlockUserList"),
        (123, "rfReqRemoveBlockUserList"),
        (122, "rfAckAddBlockUserList"),
        (121, "rfReqAddBlockUserList"),
        (120, "rfAckBlockUserListEnd"),
        (119, "rfAckBlockUserList"),
        (118, "rfAckBlockUserListStart"),
        (117, "rfReqBlockUserList"),
        (116, "rfReqCompleteFriendInvite"),
        (115, "rfAckConfirmFriendInvite"),
        (114, "rfReqConfirmFriendInvite"),
        (113, "rfReqRecvFriendInvite"),
        (112, "rfAckSendFriendInvite"),
        (111, "rfReqSendFriendInvite"),
        (110, "rfAckChatNormal"),
        (109, "rfReqChatNormal"),
        (108, "rfAckDummy"),
        (107, "rfReqDummy"),
        (106, "rfAckConfirmItemChanged"),
        (105, "rfReqConfirmItemChanged"),
        (104, "rfAckEquipItem"),
        (103, "rfReqEquipItem"),
        (102, "rfAckItemListEnd"),
        (101, "rfAckItemList"),
        (100, "rfAckItemListStart"),
        (99, "rfReqItemList"),
        (98, "rfAckEntrySkillList"),
        (97, "rfReqEntrySkillList"),
        (96, "rfAckEntryItemList"),
        (95, "rfReqEntryItemList"),
        (94, "rfAckChangeUserState"),
        (93, "rfReqChangeUserState"),
        (92, "rfAckExteriorRoomState"),
        (91, "rfReqExteriorRoomState"),
        (90, "rfAckExteriorRoomUserCount"),
        (89, "rfReqExteriorRoomUserCount"),
        (88, "rfAckExteriorRoomInfo"),
        (87, "rfReqExteriorRoomInfo"),
        (86, "rfAckGetBootyChoose"),
        (85, "rfAckGetBooty"),
        (84, "rfReqGetBooty"),
        (83, "rfAckSetUserBootySeedValue"),
        (82, "rfReqSetUserBootySeedValue"),
        (81, "rfAckChangeRoomSetting"),
        (80, "rfReqChangeRoomSetting"),
        (79, "rfReqNotifyLevelUp"),
        (78, "rfAckClientUdpInfoList"),
        (77, "rfReqClientUdpInfoList"),
        (76, "rfAckRegisterOtherUdpInfo"),
        (75, "rfAckRegisterMyUdpInfo"),
        (74, "rfReqRegisterMyUdpInfo"),
        (73, "rfAckNotifyClientJoinTime"),
        (72, "rfReqNotifyClientJoinTime"),
        (71, "rfAckChangeNewGuest"),
        (70, "rfAckChangeHost"),
        (69, "rfAckDisconnectGame"),
        (68, "rfReqDisconnectGame"),
        (67, "rfAckConnectGame"),
        (66, "rfReqConnectGame"),
        (65, "rfReqRelayServerOff"),
        (64, "rfReqRelayServerOn"),
        (63, "rfAckGameOver"),
        (62, "rfReqGameOver"),
        (61, "rfAckIsGameRoomPlaying"),
        (60, "rfReqIsGameRoomPlaying"),
        (59, "rfAckGameRoomDestroy"),
        (58, "rfAckChangeRoomMaxNum"),
        (57, "rfReqChangeRoomMaxNum"),
        (56, "rfAckUpdateScoreResult"),
        (55, "rfAckUpdateScore"),
        (54, "rfReqUpdateScore"),
        (53, "rfAckStartGame"),
        (52, "rfReqStartGame"),
        (51, "rfAckGameHostInfo"),
        (50, "rfReqGameHostInfo"),
        (49, "rfAckIsHostAvailable"),
        (48, "rfReqIsHostAvailable"),
        (47, "rfAckChangeTeam"),
        (46, "rfReqChangeTeam"),
        (45, "rfAckChangeRoomOwner"),
        (44, "rfReqChangeRoomOwner"),
        (43, "rfAckChangeGameMode"),
        (42, "rfReqChangeGameMode"),
        (41, "rfAckBanUser"),
        (40, "rfReqBanUser"),
        (39, "rfAckSelectMap"),
        (38, "rfReqSelectMap"),
        (37, "rfAckGameRoomInfo"),
        (36, "rfReqGameRoomInfo"),
        (35, "rfAckCreateGameRoom"),
        (34, "rfReqCreateGameRoom"),
        (33, "rfAckExceedMaxUser"),
        (32, "rfAckLeaveRoom"),
        (31, "rfReqLeaveRoom"),
        (30, "rfAckRoomUserList"),
        (29, "rfReqRoomUserList"),
        (28, "rfAckEnterRoom"),
        (27, "rfReqEnterRoom"),
        (26, "rfAckRoomListEnd"),
        (25, "rfAckRoomList"),
        (24, "rfAckRoomListStart"),
        (23, "rfReqRoomList"),
        (22, "rfAckChannelUserList"),
        (21, "rfReqChannelUserList"),
        (20, "rfAckLeaveChannel"),
        (19, "rfReqLeaveChannel"),
        (18, "rfAckEnterChannel"),
        (17, "rfReqEnterChannel"),
        (16, "rfAckChannelList"),
        (15, "rfReqChannelList"),
        (14, "rfAckFriendListEnd"),
        (13, "rfAckFriendList"),
        (12, "rfAckFriendListStart"),
        (11, "rfReqFriendList"),
        (10, "rfAckCharInfo"),
        (9, "rfReqCharInfo"),
        (8, "rfAckLogin2"),
        (7, "rfReqLogin2"),
        (6, "rfAckStartNatDiag"),
        (5, "rfReqStartNatDiag"),
        (4, "rfReqForceLogoutByOwner"),
        (3, "rfAckLogin"),
        (2, "rfReqLogin"),
        (1, "rfAckHash"),
        (0, "rfReqHash"),
    ];

    pub fn opcode_name(op: u32) -> Option<&'static str> {
        OPCODES.iter().find(|(o, _)| *o == op).map(|(_, n)| *n)
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGamePlayInstallBomb {
        pub installer_uid: i32,
        pub install_pos: u8,
        pub installer_x_coord: f32,
        pub installer_y_coord: f32,
        pub installer_z_coord: f32,
        pub installer_pos: i32,
        pub installer_weapon_id: i32,
        pub installer_weapon_part_id: i32,
        pub installer_range: i32,
    }

    impl ReqGamePlayInstallBomb {
        pub const OPCODE: u32 = 269;
        pub const NAME: &'static str = "rfReqGamePlay_InstallBomb";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                installer_uid: r.i32()?,
                install_pos: r.u8()?,
                installer_x_coord: r.f32()?,
                installer_y_coord: r.f32()?,
                installer_z_coord: r.f32()?,
                installer_pos: r.i32()?,
                installer_weapon_id: r.i32()?,
                installer_weapon_part_id: r.i32()?,
                installer_range: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.installer_uid);
            w.u8(self.install_pos);
            w.f32(self.installer_x_coord);
            w.f32(self.installer_y_coord);
            w.f32(self.installer_z_coord);
            w.i32(self.installer_pos);
            w.i32(self.installer_weapon_id);
            w.i32(self.installer_weapon_part_id);
            w.i32(self.installer_range);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGamePlayDefuseBomb {
        pub defuser_uid: i32,
        pub defuse_pos: u8,
        pub defuser_x_coord: f32,
        pub defuser_y_coord: f32,
        pub defuser_z_coord: f32,
        pub defuser_pos: i32,
        pub defuser_weapon_id: i32,
        pub defuser_weapon_part_id: i32,
        pub defuser_range: i32,
    }

    impl ReqGamePlayDefuseBomb {
        pub const OPCODE: u32 = 270;
        pub const NAME: &'static str = "rfReqGamePlay_DefuseBomb";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                defuser_uid: r.i32()?,
                defuse_pos: r.u8()?,
                defuser_x_coord: r.f32()?,
                defuser_y_coord: r.f32()?,
                defuser_z_coord: r.f32()?,
                defuser_pos: r.i32()?,
                defuser_weapon_id: r.i32()?,
                defuser_weapon_part_id: r.i32()?,
                defuser_range: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.defuser_uid);
            w.u8(self.defuse_pos);
            w.f32(self.defuser_x_coord);
            w.f32(self.defuser_y_coord);
            w.f32(self.defuser_z_coord);
            w.i32(self.defuser_pos);
            w.i32(self.defuser_weapon_id);
            w.i32(self.defuser_weapon_part_id);
            w.i32(self.defuser_range);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyChangeRoomState {
        pub room_state: u8,
    }

    impl ReqNotifyChangeRoomState {
        pub const OPCODE: u32 = 271;
        pub const NAME: &'static str = "rfReqNotifyChangeRoomState";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                room_state: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.room_state);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGamePlayTakeAPoint {
        pub taker_uid: Vec<i32>,
        pub take_pos: u8,
        pub taker_x_coord: Vec<f32>,
        pub taker_y_coord: Vec<f32>,
        pub taker_z_coord: Vec<f32>,
        pub taker_pos: Vec<i32>,
        pub taker_weapon_id: Vec<i32>,
        pub taker_weapon_part_id: Vec<i32>,
        pub taker_range: Vec<i32>,
    }

    impl ReqGamePlayTakeAPoint {
        pub const OPCODE: u32 = 272;
        pub const NAME: &'static str = "rfReqGamePlay_TakeAPoint";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                taker_uid: r.array(|r| r.i32())?,
                take_pos: r.u8()?,
                taker_x_coord: r.array(|r| r.f32())?,
                taker_y_coord: r.array(|r| r.f32())?,
                taker_z_coord: r.array(|r| r.f32())?,
                taker_pos: r.array(|r| r.i32())?,
                taker_weapon_id: r.array(|r| r.i32())?,
                taker_weapon_part_id: r.array(|r| r.i32())?,
                taker_range: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.taker_uid, |w, v| { w.i32(*v); });
            w.u8(self.take_pos);
            w.array(&self.taker_x_coord, |w, v| { w.f32(*v); });
            w.array(&self.taker_y_coord, |w, v| { w.f32(*v); });
            w.array(&self.taker_z_coord, |w, v| { w.f32(*v); });
            w.array(&self.taker_pos, |w, v| { w.i32(*v); });
            w.array(&self.taker_weapon_id, |w, v| { w.i32(*v); });
            w.array(&self.taker_weapon_part_id, |w, v| { w.i32(*v); });
            w.array(&self.taker_range, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqForceDisconnectUDPFromHost {
        pub user_id: i32,
    }

    impl ReqForceDisconnectUDPFromHost {
        pub const OPCODE: u32 = 273;
        pub const NAME: &'static str = "rfReqForceDisconnectUDPFromHost";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckForceDisconnectUDPFromHost {
        pub result: i32,
    }

    impl AckForceDisconnectUDPFromHost {
        pub const OPCODE: u32 = 274;
        pub const NAME: &'static str = "rfAckForceDisconnectUDPFromHost";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRecvForceDisconnectUDPFromHost {
        pub result: i32,
    }

    impl ReqRecvForceDisconnectUDPFromHost {
        pub const OPCODE: u32 = 275;
        pub const NAME: &'static str = "rfReqRecvForceDisconnectUDPFromHost";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGamePlaySetGameStartTime {
        pub uid: Vec<i32>,
    }

    impl ReqGamePlaySetGameStartTime {
        pub const OPCODE: u32 = 276;
        pub const NAME: &'static str = "rfReqGamePlay_SetGameStartTime";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                uid: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.uid, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGamePlaySetRoundStartTime {
        pub uid: Vec<i32>,
    }

    impl ReqGamePlaySetRoundStartTime {
        pub const OPCODE: u32 = 277;
        pub const NAME: &'static str = "rfReqGamePlay_SetRoundStartTime";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                uid: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.uid, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqWriteServerLog {
        pub destination: u8,
        pub log_msg: String,
    }

    impl ReqWriteServerLog {
        pub const OPCODE: u32 = 278;
        pub const NAME: &'static str = "rfReqWriteServerLog";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                destination: r.u8()?,
                log_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.destination);
            w.string(&self.log_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqUpdateMyHostPriorityPoint {
        pub host_priority_point: i32,
    }

    impl ReqUpdateMyHostPriorityPoint {
        pub const OPCODE: u32 = 279;
        pub const NAME: &'static str = "rfReqUpdateMyHostPriorityPoint";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                host_priority_point: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.host_priority_point);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGetUserHostPriorityPoint {
        pub user_id: i32,
    }

    impl ReqGetUserHostPriorityPoint {
        pub const OPCODE: u32 = 280;
        pub const NAME: &'static str = "rfReqGetUserHostPriorityPoint";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGetUserHostPriorityPoint {
        pub result: i32,
        pub user_id: i32,
        pub host_priority_point: i32,
    }

    impl AckGetUserHostPriorityPoint {
        pub const OPCODE: u32 = 281;
        pub const NAME: &'static str = "rfAckGetUserHostPriorityPoint";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                user_id: r.i32()?,
                host_priority_point: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.user_id);
            w.i32(self.host_priority_point);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyRelayServerAddr {
        pub relay_ip: String,
        pub relay_port: i32,
    }

    impl ReqNotifyRelayServerAddr {
        pub const OPCODE: u32 = 282;
        pub const NAME: &'static str = "rfReqNotifyRelayServerAddr";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                relay_ip: r.string()?,
                relay_port: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.relay_ip);
            w.i32(self.relay_port);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGamePlayUpdateDomScore {
        pub af_score: i32,
        pub rsa_score: i32,
    }

    impl ReqGamePlayUpdateDomScore {
        pub const OPCODE: u32 = 283;
        pub const NAME: &'static str = "rfReqGamePlay_UpdateDomScore";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                af_score: r.i32()?,
                rsa_score: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.af_score);
            w.i32(self.rsa_score);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGamePlayChangeWeaponSet {
        pub uid: i32,
        pub set_number: i32,
    }

    impl ReqGamePlayChangeWeaponSet {
        pub const OPCODE: u32 = 284;
        pub const NAME: &'static str = "rfReqGamePlay_ChangeWeaponSet";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                uid: r.i32()?,
                set_number: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.uid);
            w.i32(self.set_number);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyRecvDummyForHolePunching {
        pub client_user_id: i32,
        pub phrase: u8,
    }

    impl ReqNotifyRecvDummyForHolePunching {
        pub const OPCODE: u32 = 285;
        pub const NAME: &'static str = "rfReqNotifyRecvDummyForHolePunching";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                client_user_id: r.i32()?,
                phrase: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.client_user_id);
            w.u8(self.phrase);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGetHolePunchingResult;

    impl ReqGetHolePunchingResult {
        pub const OPCODE: u32 = 286;
        pub const NAME: &'static str = "rfReqGetHolePunchingResult";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGetHolePunchingResult {
        pub result: i32,
        pub phrase: u8,
        pub intervention_user_id: i32,
        pub intervention: u8,
        pub failed_user_i_ds: Vec<i32>,
    }

    impl AckGetHolePunchingResult {
        pub const OPCODE: u32 = 287;
        pub const NAME: &'static str = "rfAckGetHolePunchingResult";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                phrase: r.u8()?,
                intervention_user_id: r.i32()?,
                intervention: r.u8()?,
                failed_user_i_ds: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u8(self.phrase);
            w.i32(self.intervention_user_id);
            w.u8(self.intervention);
            w.array(&self.failed_user_i_ds, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyHostBlocking {
        pub blocking: u8,
    }

    impl ReqNotifyHostBlocking {
        pub const OPCODE: u32 = 288;
        pub const NAME: &'static str = "rfReqNotifyHostBlocking";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                blocking: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.blocking);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNotifyHostBlocking {
        pub result: i32,
        pub blocking: u8,
    }

    impl AckNotifyHostBlocking {
        pub const OPCODE: u32 = 289;
        pub const NAME: &'static str = "rfAckNotifyHostBlocking";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                blocking: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u8(self.blocking);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGetIsHostBlocking;

    impl ReqGetIsHostBlocking {
        pub const OPCODE: u32 = 290;
        pub const NAME: &'static str = "rfReqGetIsHostBlocking";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGetIsHostBlocking {
        pub result: i32,
        pub blocking: u8,
    }

    impl AckGetIsHostBlocking {
        pub const OPCODE: u32 = 291;
        pub const NAME: &'static str = "rfAckGetIsHostBlocking";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                blocking: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u8(self.blocking);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqIsAvailClanURL {
        pub clan_url: String,
    }

    impl ReqIsAvailClanURL {
        pub const OPCODE: u32 = 292;
        pub const NAME: &'static str = "rfReqIsAvailClanURL";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_url: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_url);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckIsAvailClanURL {
        pub clan_url: String,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckIsAvailClanURL {
        pub const OPCODE: u32 = 293;
        pub const NAME: &'static str = "rfAckIsAvailClanURL";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_url: r.string()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_url);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAcquirePromotionItems {
        pub uid: i32,
        pub result: i32,
        pub err_msg: String,
        pub item_unique_id: Vec<f64>,
        pub item_i_ds: Vec<i32>,
        pub part_i_ds: Vec<i32>,
    }

    impl AckAcquirePromotionItems {
        pub const OPCODE: u32 = 294;
        pub const NAME: &'static str = "rfAckAcquirePromotionItems";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                uid: r.i32()?,
                result: r.i32()?,
                err_msg: r.string()?,
                item_unique_id: r.array(|r| r.f64())?,
                item_i_ds: r.array(|r| r.i32())?,
                part_i_ds: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.uid);
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.item_unique_id, |w, v| { w.f64(*v); });
            w.array(&self.item_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.part_i_ds, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGamePlayGetChopper {
        pub get_chopper_uid: i32,
        pub get_chopper_assist_uid: Vec<i32>,
        pub kill_x_coord: f32,
        pub kill_y_coord: f32,
        pub kill_z_coord: f32,
        pub kill_pos: i32,
        pub kill_weapon_id: i32,
        pub kill_weapon_part_id: i32,
        pub kill_range: i32,
        pub kill_damage: i32,
        pub chopper_caller_id: i32,
        pub chopper_caller_name: String,
        pub chopper_x_coord: f32,
        pub chopper_y_coord: f32,
        pub chopper_z_coord: f32,
    }

    impl ReqGamePlayGetChopper {
        pub const OPCODE: u32 = 295;
        pub const NAME: &'static str = "rfReqGamePlay_GetChopper";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                get_chopper_uid: r.i32()?,
                get_chopper_assist_uid: r.array(|r| r.i32())?,
                kill_x_coord: r.f32()?,
                kill_y_coord: r.f32()?,
                kill_z_coord: r.f32()?,
                kill_pos: r.i32()?,
                kill_weapon_id: r.i32()?,
                kill_weapon_part_id: r.i32()?,
                kill_range: r.i32()?,
                kill_damage: r.i32()?,
                chopper_caller_id: r.i32()?,
                chopper_caller_name: r.string()?,
                chopper_x_coord: r.f32()?,
                chopper_y_coord: r.f32()?,
                chopper_z_coord: r.f32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.get_chopper_uid);
            w.array(&self.get_chopper_assist_uid, |w, v| { w.i32(*v); });
            w.f32(self.kill_x_coord);
            w.f32(self.kill_y_coord);
            w.f32(self.kill_z_coord);
            w.i32(self.kill_pos);
            w.i32(self.kill_weapon_id);
            w.i32(self.kill_weapon_part_id);
            w.i32(self.kill_range);
            w.i32(self.kill_damage);
            w.i32(self.chopper_caller_id);
            w.string(&self.chopper_caller_name);
            w.f32(self.chopper_x_coord);
            w.f32(self.chopper_y_coord);
            w.f32(self.chopper_z_coord);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqCheckEquipedWeaponDurability;

    impl ReqCheckEquipedWeaponDurability {
        pub const OPCODE: u32 = 296;
        pub const NAME: &'static str = "rfReqCheckEquipedWeaponDurability";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckCheckEquipedWeaponDurability {
        pub result: i32,
        pub item_unique_id: Vec<f64>,
        pub item_slot_num: Vec<i32>,
    }

    impl AckCheckEquipedWeaponDurability {
        pub const OPCODE: u32 = 297;
        pub const NAME: &'static str = "rfAckCheckEquipedWeaponDurability";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                item_unique_id: r.array(|r| r.f64())?,
                item_slot_num: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.array(&self.item_unique_id, |w, v| { w.f64(*v); });
            w.array(&self.item_slot_num, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGameGuardAuthData {
        pub auth_data_index: i32,
        pub auth_data_val1: i32,
        pub auth_data_val2: i32,
        pub auth_data_val3: i32,
    }

    impl ReqGameGuardAuthData {
        pub const OPCODE: u32 = 298;
        pub const NAME: &'static str = "rfReqGameGuardAuthData";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                auth_data_index: r.i32()?,
                auth_data_val1: r.i32()?,
                auth_data_val2: r.i32()?,
                auth_data_val3: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.auth_data_index);
            w.i32(self.auth_data_val1);
            w.i32(self.auth_data_val2);
            w.i32(self.auth_data_val3);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGameGuardAuthData {
        pub auth_data_index: i32,
        pub auth_data_val1: i32,
        pub auth_data_val2: i32,
        pub auth_data_val3: i32,
    }

    impl AckGameGuardAuthData {
        pub const OPCODE: u32 = 299;
        pub const NAME: &'static str = "rfAckGameGuardAuthData";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                auth_data_index: r.i32()?,
                auth_data_val1: r.i32()?,
                auth_data_val2: r.i32()?,
                auth_data_val3: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.auth_data_index);
            w.i32(self.auth_data_val1);
            w.i32(self.auth_data_val2);
            w.i32(self.auth_data_val3);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqDeleteFriend {
        pub delete_char_name: Vec<String>,
    }

    impl ReqDeleteFriend {
        pub const OPCODE: u32 = 300;
        pub const NAME: &'static str = "rfReqDeleteFriend";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                delete_char_name: r.array(|r| r.string())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.delete_char_name, |w, v| { w.string(v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckDeleteFriend {
        pub result: i32,
        pub failed_charname: Vec<String>,
    }

    impl AckDeleteFriend {
        pub const OPCODE: u32 = 301;
        pub const NAME: &'static str = "rfAckDeleteFriend";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                failed_charname: r.array(|r| r.string())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.array(&self.failed_charname, |w, v| { w.string(v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRecvDeleteFriend {
        pub result: i32,
        pub from_char_name: String,
    }

    impl ReqRecvDeleteFriend {
        pub const OPCODE: u32 = 302;
        pub const NAME: &'static str = "rfReqRecvDeleteFriend";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                from_char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.from_char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqChannelUserListInGameRoom {
        pub channel_num: u16,
        pub start_index: u16,
        pub count: u16,
    }

    impl ReqChannelUserListInGameRoom {
        pub const OPCODE: u32 = 303;
        pub const NAME: &'static str = "rfReqChannelUserListInGameRoom";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                channel_num: r.u16()?,
                start_index: r.u16()?,
                count: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.channel_num);
            w.u16(self.start_index);
            w.u16(self.count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChannelUserListInGameRoom {
        pub result: i32,
        pub total_count: i32,
        pub count: i32,
        pub start_index: i32,
        pub uid: Vec<i32>,
        pub char_name: Vec<String>,
        pub user_level: Vec<i32>,
        pub clan_name: Vec<String>,
        pub clan_pattern: Vec<i32>,
        pub clan_bg: Vec<i32>,
        pub clan_bl: Vec<i32>,
        pub is_look_for_clan: Vec<u8>,
        pub clan_level: Vec<i32>,
        pub level_mark_id: Vec<i32>,
        pub title_mark_id: Vec<i32>,
        pub reserved1: Vec<i32>,
        pub reserved2: Vec<i32>,
        pub reserved3: Vec<i32>,
    }

    impl AckChannelUserListInGameRoom {
        pub const OPCODE: u32 = 304;
        pub const NAME: &'static str = "rfAckChannelUserListInGameRoom";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                total_count: r.i32()?,
                count: r.i32()?,
                start_index: r.i32()?,
                uid: r.array(|r| r.i32())?,
                char_name: r.array(|r| r.string())?,
                user_level: r.array(|r| r.i32())?,
                clan_name: r.array(|r| r.string())?,
                clan_pattern: r.array(|r| r.i32())?,
                clan_bg: r.array(|r| r.i32())?,
                clan_bl: r.array(|r| r.i32())?,
                is_look_for_clan: r.array(|r| r.u8())?,
                clan_level: r.array(|r| r.i32())?,
                level_mark_id: r.array(|r| r.i32())?,
                title_mark_id: r.array(|r| r.i32())?,
                reserved1: r.array(|r| r.i32())?,
                reserved2: r.array(|r| r.i32())?,
                reserved3: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.total_count);
            w.i32(self.count);
            w.i32(self.start_index);
            w.array(&self.uid, |w, v| { w.i32(*v); });
            w.array(&self.char_name, |w, v| { w.string(v); });
            w.array(&self.user_level, |w, v| { w.i32(*v); });
            w.array(&self.clan_name, |w, v| { w.string(v); });
            w.array(&self.clan_pattern, |w, v| { w.i32(*v); });
            w.array(&self.clan_bg, |w, v| { w.i32(*v); });
            w.array(&self.clan_bl, |w, v| { w.i32(*v); });
            w.array(&self.is_look_for_clan, |w, v| { w.u8(*v); });
            w.array(&self.clan_level, |w, v| { w.i32(*v); });
            w.array(&self.level_mark_id, |w, v| { w.i32(*v); });
            w.array(&self.title_mark_id, |w, v| { w.i32(*v); });
            w.array(&self.reserved1, |w, v| { w.i32(*v); });
            w.array(&self.reserved2, |w, v| { w.i32(*v); });
            w.array(&self.reserved3, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSpecificGameRoomInfo {
        pub room_num: i32,
    }

    impl ReqSpecificGameRoomInfo {
        pub const OPCODE: u32 = 305;
        pub const NAME: &'static str = "rfReqSpecificGameRoomInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                room_num: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.room_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckSpecificGameRoomInfo {
        pub result: i32,
        pub game_num: i32,
        pub title: String,
        pub map_name: String,
        pub map_num: i32,
        pub user_count: u8,
        pub max_count: u8,
        pub game_mode_name: String,
        pub mode_num: i32,
        pub weapon_limit: u8,
        pub is_hard_core: u8,
        pub status: u8,
        pub room_owner_ip: String,
        pub game_minute: i32,
        pub room_owner_name: String,
        pub is_special: u8,
        pub is_password: u8,
    }

    impl AckSpecificGameRoomInfo {
        pub const OPCODE: u32 = 306;
        pub const NAME: &'static str = "rfAckSpecificGameRoomInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                game_num: r.i32()?,
                title: r.string()?,
                map_name: r.string()?,
                map_num: r.i32()?,
                user_count: r.u8()?,
                max_count: r.u8()?,
                game_mode_name: r.string()?,
                mode_num: r.i32()?,
                weapon_limit: r.u8()?,
                is_hard_core: r.u8()?,
                status: r.u8()?,
                room_owner_ip: r.string()?,
                game_minute: r.i32()?,
                room_owner_name: r.string()?,
                is_special: r.u8()?,
                is_password: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.game_num);
            w.string(&self.title);
            w.string(&self.map_name);
            w.i32(self.map_num);
            w.u8(self.user_count);
            w.u8(self.max_count);
            w.string(&self.game_mode_name);
            w.i32(self.mode_num);
            w.u8(self.weapon_limit);
            w.u8(self.is_hard_core);
            w.u8(self.status);
            w.string(&self.room_owner_ip);
            w.i32(self.game_minute);
            w.string(&self.room_owner_name);
            w.u8(self.is_special);
            w.u8(self.is_password);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSpecificChannelUserInfo {
        pub user_id: i32,
    }

    impl ReqSpecificChannelUserInfo {
        pub const OPCODE: u32 = 307;
        pub const NAME: &'static str = "rfReqSpecificChannelUserInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckSpecificChannelUserInfo {
        pub uid: i32,
        pub user_id: String,
        pub user_levels: i32,
        pub clan_name: String,
        pub clan_pattern: i32,
        pub clan_bg: i32,
        pub clan_bl: i32,
        pub result: i32,
        pub err_msg: String,
        pub level_mark_id: i32,
        pub title_mark_id: i32,
        pub reserved1: i32,
        pub reserved2: i32,
        pub reserved3: i32,
    }

    impl AckSpecificChannelUserInfo {
        pub const OPCODE: u32 = 308;
        pub const NAME: &'static str = "rfAckSpecificChannelUserInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                uid: r.i32()?,
                user_id: r.string()?,
                user_levels: r.i32()?,
                clan_name: r.string()?,
                clan_pattern: r.i32()?,
                clan_bg: r.i32()?,
                clan_bl: r.i32()?,
                result: r.i32()?,
                err_msg: r.string()?,
                level_mark_id: r.i32()?,
                title_mark_id: r.i32()?,
                reserved1: r.i32()?,
                reserved2: r.i32()?,
                reserved3: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.uid);
            w.string(&self.user_id);
            w.i32(self.user_levels);
            w.string(&self.clan_name);
            w.i32(self.clan_pattern);
            w.i32(self.clan_bg);
            w.i32(self.clan_bl);
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.level_mark_id);
            w.i32(self.title_mark_id);
            w.i32(self.reserved1);
            w.i32(self.reserved2);
            w.i32(self.reserved3);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNoticeGM {
        pub notice_type: u8,
        pub server_id: i32,
        pub channel_num: u16,
        pub game_room_num: u16,
        pub msg: String,
    }

    impl ReqNoticeGM {
        pub const OPCODE: u32 = 309;
        pub const NAME: &'static str = "rfReqNotice_GM";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                notice_type: r.u8()?,
                server_id: r.i32()?,
                channel_num: r.u16()?,
                game_room_num: r.u16()?,
                msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.notice_type);
            w.i32(self.server_id);
            w.u16(self.channel_num);
            w.u16(self.game_room_num);
            w.string(&self.msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNoticeGM {
        pub result: i32,
        pub err_msg: String,
        pub notice_type: u8,
        pub server_id: i32,
        pub channel_num: u16,
        pub game_room_num: u16,
        pub msg: String,
    }

    impl AckNoticeGM {
        pub const OPCODE: u32 = 310;
        pub const NAME: &'static str = "rfAckNotice_GM";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                notice_type: r.u8()?,
                server_id: r.i32()?,
                channel_num: r.u16()?,
                game_room_num: r.u16()?,
                msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.u8(self.notice_type);
            w.i32(self.server_id);
            w.u16(self.channel_num);
            w.u16(self.game_room_num);
            w.string(&self.msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRecvNoticeFromGM {
        pub result: i32,
        pub err_msg: String,
        pub notice_msg: Vec<String>,
        pub is_from_db: u8,
    }

    impl ReqRecvNoticeFromGM {
        pub const OPCODE: u32 = 311;
        pub const NAME: &'static str = "rfReqRecvNotice_FromGM";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                notice_msg: r.array(|r| r.string())?,
                is_from_db: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.notice_msg, |w, v| { w.string(v); });
            w.u8(self.is_from_db);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqForceDisconnectUserGM {
        pub char_name: String,
    }

    impl ReqForceDisconnectUserGM {
        pub const OPCODE: u32 = 312;
        pub const NAME: &'static str = "rfReqForceDisconnectUser_GM";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckForceDisconnectUserGM {
        pub result: i32,
        pub err_msg: String,
        pub char_name: String,
    }

    impl AckForceDisconnectUserGM {
        pub const OPCODE: u32 = 313;
        pub const NAME: &'static str = "rfAckForceDisconnectUser_GM";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.string(&self.char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRecvForceDisconnectedFromGM {
        pub result: i32,
        pub err_msg: String,
    }

    impl ReqRecvForceDisconnectedFromGM {
        pub const OPCODE: u32 = 314;
        pub const NAME: &'static str = "rfReqRecvForceDisconnected_FromGM";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyDisconnectHostInLoading {
        pub result: i32,
        pub err_msg: String,
        pub host_id: i32,
    }

    impl ReqNotifyDisconnectHostInLoading {
        pub const OPCODE: u32 = 315;
        pub const NAME: &'static str = "rfReqNotifyDisconnectHostInLoading";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                host_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.host_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqChatWispherMyCommunity {
        pub source_char_name: String,
        pub sender_name: String,
        pub msg: String,
    }

    impl ReqChatWispherMyCommunity {
        pub const OPCODE: u32 = 316;
        pub const NAME: &'static str = "rfReqChatWispherMyCommunity";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                source_char_name: r.string()?,
                sender_name: r.string()?,
                msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.source_char_name);
            w.string(&self.sender_name);
            w.string(&self.msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChatWispherMyCommunity {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckChatWispherMyCommunity {
        pub const OPCODE: u32 = 317;
        pub const NAME: &'static str = "rfAckChatWispherMyCommunity";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyDurabilityZeroItem {
        pub result: i32,
        pub err_msg: String,
        pub user_id: i32,
        pub item_unique_i_ds: Vec<f64>,
        pub item_i_ds: Vec<i32>,
        pub item_part_i_ds: Vec<i32>,
        pub item_slot_pos: Vec<u8>,
    }

    impl ReqNotifyDurabilityZeroItem {
        pub const OPCODE: u32 = 318;
        pub const NAME: &'static str = "rfReqNotifyDurabilityZeroItem";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                user_id: r.i32()?,
                item_unique_i_ds: r.array(|r| r.f64())?,
                item_i_ds: r.array(|r| r.i32())?,
                item_part_i_ds: r.array(|r| r.i32())?,
                item_slot_pos: r.array(|r| r.u8())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.user_id);
            w.array(&self.item_unique_i_ds, |w, v| { w.f64(*v); });
            w.array(&self.item_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.item_part_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.item_slot_pos, |w, v| { w.u8(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyMyCharInfoToRoomUser;

    impl ReqNotifyMyCharInfoToRoomUser {
        pub const OPCODE: u32 = 319;
        pub const NAME: &'static str = "rfReqNotifyMyCharInfoToRoomUser";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNotifyMyCharInfoToRoomUser {
        pub result: i32,
        pub err_msg: String,
        pub uid: i32,
        pub user_name: String,
        pub grade: i32,
        pub team_num: i32,
        pub statue: u8,
        pub is_host: u8,
        pub user_ip: String,
        pub clan_name: String,
        pub cm_pattern: i32,
        pub cm_bg: i32,
        pub cm_bl: i32,
        pub is_look_for: u8,
    }

    impl AckNotifyMyCharInfoToRoomUser {
        pub const OPCODE: u32 = 320;
        pub const NAME: &'static str = "rfAckNotifyMyCharInfoToRoomUser";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                uid: r.i32()?,
                user_name: r.string()?,
                grade: r.i32()?,
                team_num: r.i32()?,
                statue: r.u8()?,
                is_host: r.u8()?,
                user_ip: r.string()?,
                clan_name: r.string()?,
                cm_pattern: r.i32()?,
                cm_bg: r.i32()?,
                cm_bl: r.i32()?,
                is_look_for: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.uid);
            w.string(&self.user_name);
            w.i32(self.grade);
            w.i32(self.team_num);
            w.u8(self.statue);
            w.u8(self.is_host);
            w.string(&self.user_ip);
            w.string(&self.clan_name);
            w.i32(self.cm_pattern);
            w.i32(self.cm_bg);
            w.i32(self.cm_bl);
            w.u8(self.is_look_for);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqStartHolePunching {
        pub game_type: i32,
        pub req_start_user_id: i32,
    }

    impl ReqStartHolePunching {
        pub const OPCODE: u32 = 321;
        pub const NAME: &'static str = "rfReqStartHolePunching";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_type: r.i32()?,
                req_start_user_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.game_type);
            w.i32(self.req_start_user_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckStartHolePunching {
        pub result: i32,
        pub err_msg: String,
        pub game_type: i32,
        pub req_start_user_id: i32,
    }

    impl AckStartHolePunching {
        pub const OPCODE: u32 = 322;
        pub const NAME: &'static str = "rfAckStartHolePunching";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                game_type: r.i32()?,
                req_start_user_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.game_type);
            w.i32(self.req_start_user_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqForceFailedClientLoading {
        pub failed_user_id: i32,
        pub err_code: i32,
    }

    impl ReqForceFailedClientLoading {
        pub const OPCODE: u32 = 323;
        pub const NAME: &'static str = "rfReqForceFailedClientLoading";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                failed_user_id: r.i32()?,
                err_code: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.failed_user_id);
            w.i32(self.err_code);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckForceFailedClientLoading {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckForceFailedClientLoading {
        pub const OPCODE: u32 = 324;
        pub const NAME: &'static str = "rfAckForceFailedClientLoading";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRecvForceFailedClientLoading {
        pub err_code: i32,
    }

    impl ReqRecvForceFailedClientLoading {
        pub const OPCODE: u32 = 325;
        pub const NAME: &'static str = "rfReqRecvForceFailedClientLoading";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                err_code: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.err_code);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqItemListInItemBox {
        pub item_unique_id: f64,
        pub box_item_id: i32,
        pub from_content: u8,
    }

    impl ReqItemListInItemBox {
        pub const OPCODE: u32 = 326;
        pub const NAME: &'static str = "rfReqItemListInItemBox";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_unique_id: r.f64()?,
                box_item_id: r.i32()?,
                from_content: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.f64(self.item_unique_id);
            w.i32(self.box_item_id);
            w.u8(self.from_content);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckItemListInItemBox {
        pub result: i32,
        pub err_msg: String,
        pub item_unique_id: f64,
        pub box_item_id: i32,
        pub item_id: Vec<i32>,
        pub part_id: Vec<i32>,
        pub from_content: u8,
    }

    impl AckItemListInItemBox {
        pub const OPCODE: u32 = 327;
        pub const NAME: &'static str = "rfAckItemListInItemBox";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                item_unique_id: r.f64()?,
                box_item_id: r.i32()?,
                item_id: r.array(|r| r.i32())?,
                part_id: r.array(|r| r.i32())?,
                from_content: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.f64(self.item_unique_id);
            w.i32(self.box_item_id);
            w.array(&self.item_id, |w, v| { w.i32(*v); });
            w.array(&self.part_id, |w, v| { w.i32(*v); });
            w.u8(self.from_content);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGetItemFromItemBox {
        pub item_unique_id: f64,
        pub item_id: i32,
        pub item_id_choose: i32,
        pub part_id_choose: i32,
    }

    impl ReqGetItemFromItemBox {
        pub const OPCODE: u32 = 328;
        pub const NAME: &'static str = "rfReqGetItemFromItemBox";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_unique_id: r.f64()?,
                item_id: r.i32()?,
                item_id_choose: r.i32()?,
                part_id_choose: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.f64(self.item_unique_id);
            w.i32(self.item_id);
            w.i32(self.item_id_choose);
            w.i32(self.part_id_choose);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGetItemFromItemBox {
        pub result: i32,
        pub err_msg: String,
        pub item_id_choose: i32,
        pub part_id_choose: i32,
        pub box_item_idx: f64,
        pub bxo_item_count: i32,
        pub b_have: u8,
        pub item_idx: f64,
        pub until_time: f64,
        pub stack_count: i32,
    }

    impl AckGetItemFromItemBox {
        pub const OPCODE: u32 = 329;
        pub const NAME: &'static str = "rfAckGetItemFromItemBox";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                item_id_choose: r.i32()?,
                part_id_choose: r.i32()?,
                box_item_idx: r.f64()?,
                bxo_item_count: r.i32()?,
                b_have: r.u8()?,
                item_idx: r.f64()?,
                until_time: r.f64()?,
                stack_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.item_id_choose);
            w.i32(self.part_id_choose);
            w.f64(self.box_item_idx);
            w.i32(self.bxo_item_count);
            w.u8(self.b_have);
            w.f64(self.item_idx);
            w.f64(self.until_time);
            w.i32(self.stack_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAutoChangedTeamUserList {
        pub user_id: Vec<i32>,
        pub team_num: Vec<i32>,
    }

    impl AckAutoChangedTeamUserList {
        pub const OPCODE: u32 = 330;
        pub const NAME: &'static str = "rfAckAutoChangedTeamUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.array(|r| r.i32())?,
                team_num: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.user_id, |w, v| { w.i32(*v); });
            w.array(&self.team_num, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchGetReady;

    impl ReqClanMatchGetReady {
        pub const OPCODE: u32 = 331;
        pub const NAME: &'static str = "rfReqClanMatch_GetReady";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchGetReady {
        pub result: i32,
        pub err_msg: String,
        pub room_index: i32,
    }

    impl AckClanMatchGetReady {
        pub const OPCODE: u32 = 332;
        pub const NAME: &'static str = "rfAckClanMatch_GetReady";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                room_index: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.room_index);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchStartCombat {
        pub map_num_array: Vec<i32>,
        pub versus_mode: i32,
    }

    impl ReqClanMatchStartCombat {
        pub const OPCODE: u32 = 333;
        pub const NAME: &'static str = "rfReqClanMatch_StartCombat";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                map_num_array: r.array(|r| r.i32())?,
                versus_mode: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.map_num_array, |w, v| { w.i32(*v); });
            w.i32(self.versus_mode);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchStartCombat {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckClanMatchStartCombat {
        pub const OPCODE: u32 = 334;
        pub const NAME: &'static str = "rfAckClanMatch_StartCombat";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchNotifiyStartCombat {
        pub map_num_array: Vec<i32>,
        pub versus_mode: i32,
    }

    impl AckClanMatchNotifiyStartCombat {
        pub const OPCODE: u32 = 335;
        pub const NAME: &'static str = "rfAckClanMatch_NotifiyStartCombat";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                map_num_array: r.array(|r| r.i32())?,
                versus_mode: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.map_num_array, |w, v| { w.i32(*v); });
            w.i32(self.versus_mode);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchInviteCrew {
        pub invitee: String,
    }

    impl ReqClanMatchInviteCrew {
        pub const OPCODE: u32 = 336;
        pub const NAME: &'static str = "rfReqClanMatch_InviteCrew";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                invitee: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.invitee);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchInviteCrew {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckClanMatchInviteCrew {
        pub const OPCODE: u32 = 337;
        pub const NAME: &'static str = "rfAckClanMatch_InviteCrew";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchRecvInviteCrew {
        pub inviter: String,
        pub server_ip: String,
        pub port: i32,
        pub waitting_id: i32,
        pub msg: String,
        pub map_num: i32,
        pub invite_user_channel_num: i32,
        pub leader_name: String,
        pub room_name: String,
    }

    impl ReqClanMatchRecvInviteCrew {
        pub const OPCODE: u32 = 338;
        pub const NAME: &'static str = "rfReqClanMatch_RecvInviteCrew";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                inviter: r.string()?,
                server_ip: r.string()?,
                port: r.i32()?,
                waitting_id: r.i32()?,
                msg: r.string()?,
                map_num: r.i32()?,
                invite_user_channel_num: r.i32()?,
                leader_name: r.string()?,
                room_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.inviter);
            w.string(&self.server_ip);
            w.i32(self.port);
            w.i32(self.waitting_id);
            w.string(&self.msg);
            w.i32(self.map_num);
            w.i32(self.invite_user_channel_num);
            w.string(&self.leader_name);
            w.string(&self.room_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchRecvInviteCrew {
        pub result: i32,
        pub inviter: String,
    }

    impl AckClanMatchRecvInviteCrew {
        pub const OPCODE: u32 = 339;
        pub const NAME: &'static str = "rfAckClanMatch_RecvInviteCrew";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                inviter: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.inviter);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchInviteCrewResult {
        pub invitee: String,
        pub result: i32,
    }

    impl ReqClanMatchInviteCrewResult {
        pub const OPCODE: u32 = 340;
        pub const NAME: &'static str = "rfReqClanMatch_InviteCrewResult";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                invitee: r.string()?,
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.invitee);
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchJoin {
        pub waitting_id: i32,
    }

    impl ReqClanMatchJoin {
        pub const OPCODE: u32 = 341;
        pub const NAME: &'static str = "rfReqClanMatch_Join";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                waitting_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.waitting_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchJoin {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckClanMatchJoin {
        pub const OPCODE: u32 = 342;
        pub const NAME: &'static str = "rfAckClanMatch_Join";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchJoinNotify {
        pub is_leader: u8,
        pub clan_mark_pa: i32,
        pub clan_mark_bg: i32,
        pub clan_mark_bl: i32,
        pub level: i32,
        pub char_name: String,
        pub clan_grade: i32,
        pub clan_win: i32,
        pub clan_lose: i32,
        pub clan_name: String,
        pub clan_level: i32,
        pub level_mark_id: i32,
        pub title_mark_id: i32,
        pub reserved1: i32,
        pub reserved2: i32,
        pub reserved3: i32,
    }

    impl AckClanMatchJoinNotify {
        pub const OPCODE: u32 = 343;
        pub const NAME: &'static str = "rfAckClanMatch_JoinNotify";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                is_leader: r.u8()?,
                clan_mark_pa: r.i32()?,
                clan_mark_bg: r.i32()?,
                clan_mark_bl: r.i32()?,
                level: r.i32()?,
                char_name: r.string()?,
                clan_grade: r.i32()?,
                clan_win: r.i32()?,
                clan_lose: r.i32()?,
                clan_name: r.string()?,
                clan_level: r.i32()?,
                level_mark_id: r.i32()?,
                title_mark_id: r.i32()?,
                reserved1: r.i32()?,
                reserved2: r.i32()?,
                reserved3: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.is_leader);
            w.i32(self.clan_mark_pa);
            w.i32(self.clan_mark_bg);
            w.i32(self.clan_mark_bl);
            w.i32(self.level);
            w.string(&self.char_name);
            w.i32(self.clan_grade);
            w.i32(self.clan_win);
            w.i32(self.clan_lose);
            w.string(&self.clan_name);
            w.i32(self.clan_level);
            w.i32(self.level_mark_id);
            w.i32(self.title_mark_id);
            w.i32(self.reserved1);
            w.i32(self.reserved2);
            w.i32(self.reserved3);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchMemberList;

    impl ReqClanMatchMemberList {
        pub const OPCODE: u32 = 344;
        pub const NAME: &'static str = "rfReqClanMatch_MemberList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchMemberList {
        pub is_leader: Vec<u8>,
        pub clan_mark_pa: Vec<i32>,
        pub clan_mark_bg: Vec<i32>,
        pub clan_mark_bl: Vec<i32>,
        pub level: Vec<i32>,
        pub char_name: Vec<String>,
        pub clan_grade: Vec<i32>,
        pub clan_win: Vec<i32>,
        pub clan_lose: Vec<i32>,
        pub clan_name: Vec<String>,
        pub clan_level: Vec<i32>,
        pub level_mark_id: Vec<i32>,
        pub title_mark_id: Vec<i32>,
        pub reserved1: Vec<i32>,
        pub reserved2: Vec<i32>,
        pub reserved3: Vec<i32>,
    }

    impl AckClanMatchMemberList {
        pub const OPCODE: u32 = 345;
        pub const NAME: &'static str = "rfAckClanMatch_MemberList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                is_leader: r.array(|r| r.u8())?,
                clan_mark_pa: r.array(|r| r.i32())?,
                clan_mark_bg: r.array(|r| r.i32())?,
                clan_mark_bl: r.array(|r| r.i32())?,
                level: r.array(|r| r.i32())?,
                char_name: r.array(|r| r.string())?,
                clan_grade: r.array(|r| r.i32())?,
                clan_win: r.array(|r| r.i32())?,
                clan_lose: r.array(|r| r.i32())?,
                clan_name: r.array(|r| r.string())?,
                clan_level: r.array(|r| r.i32())?,
                level_mark_id: r.array(|r| r.i32())?,
                title_mark_id: r.array(|r| r.i32())?,
                reserved1: r.array(|r| r.i32())?,
                reserved2: r.array(|r| r.i32())?,
                reserved3: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.is_leader, |w, v| { w.u8(*v); });
            w.array(&self.clan_mark_pa, |w, v| { w.i32(*v); });
            w.array(&self.clan_mark_bg, |w, v| { w.i32(*v); });
            w.array(&self.clan_mark_bl, |w, v| { w.i32(*v); });
            w.array(&self.level, |w, v| { w.i32(*v); });
            w.array(&self.char_name, |w, v| { w.string(v); });
            w.array(&self.clan_grade, |w, v| { w.i32(*v); });
            w.array(&self.clan_win, |w, v| { w.i32(*v); });
            w.array(&self.clan_lose, |w, v| { w.i32(*v); });
            w.array(&self.clan_name, |w, v| { w.string(v); });
            w.array(&self.clan_level, |w, v| { w.i32(*v); });
            w.array(&self.level_mark_id, |w, v| { w.i32(*v); });
            w.array(&self.title_mark_id, |w, v| { w.i32(*v); });
            w.array(&self.reserved1, |w, v| { w.i32(*v); });
            w.array(&self.reserved2, |w, v| { w.i32(*v); });
            w.array(&self.reserved3, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchLeave {
        pub waitting_id: i32,
    }

    impl ReqClanMatchLeave {
        pub const OPCODE: u32 = 346;
        pub const NAME: &'static str = "rfReqClanMatch_Leave";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                waitting_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.waitting_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchLeave {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckClanMatchLeave {
        pub const OPCODE: u32 = 347;
        pub const NAME: &'static str = "rfAckClanMatch_Leave";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchLeaveNotify {
        pub char_name: String,
    }

    impl AckClanMatchLeaveNotify {
        pub const OPCODE: u32 = 348;
        pub const NAME: &'static str = "rfAckClanMatch_LeaveNotify";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchChat {
        pub msg: String,
    }

    impl ReqClanMatchChat {
        pub const OPCODE: u32 = 349;
        pub const NAME: &'static str = "rfReqClanMatch_Chat";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchChat {
        pub result: i32,
        pub err_msg: String,
        pub char_name: String,
        pub msg: String,
    }

    impl AckClanMatchChat {
        pub const OPCODE: u32 = 350;
        pub const NAME: &'static str = "rfAckClanMatch_Chat";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                char_name: r.string()?,
                msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.string(&self.char_name);
            w.string(&self.msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchWaittingTeamList {
        pub start_index: i32,
        pub return_count: i32,
    }

    impl ReqClanMatchWaittingTeamList {
        pub const OPCODE: u32 = 351;
        pub const NAME: &'static str = "rfReqClanMatch_WaittingTeamList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                start_index: r.i32()?,
                return_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.start_index);
            w.i32(self.return_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchWaittingTeamList {
        pub ttotl_count: i32,
        pub grade: Vec<i32>,
        pub map: Vec<i32>,
        pub versus: Vec<i32>,
    }

    impl AckClanMatchWaittingTeamList {
        pub const OPCODE: u32 = 352;
        pub const NAME: &'static str = "rfAckClanMatch_WaittingTeamList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                ttotl_count: r.i32()?,
                grade: r.array(|r| r.i32())?,
                map: r.array(|r| r.i32())?,
                versus: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.ttotl_count);
            w.array(&self.grade, |w, v| { w.i32(*v); });
            w.array(&self.map, |w, v| { w.i32(*v); });
            w.array(&self.versus, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchChangeLeader {
        pub wait_room_idx: i32,
        pub new_leader_id: i32,
        pub new_leader_name: String,
    }

    impl ReqClanMatchChangeLeader {
        pub const OPCODE: u32 = 353;
        pub const NAME: &'static str = "rfReqClanMatch_ChangeLeader";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                wait_room_idx: r.i32()?,
                new_leader_id: r.i32()?,
                new_leader_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.wait_room_idx);
            w.i32(self.new_leader_id);
            w.string(&self.new_leader_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchChangeLeader {
        pub result: i32,
        pub err_msg: String,
        pub new_leader: i32,
        pub new_leader_name: String,
    }

    impl AckClanMatchChangeLeader {
        pub const OPCODE: u32 = 354;
        pub const NAME: &'static str = "rfAckClanMatch_ChangeLeader";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                new_leader: r.i32()?,
                new_leader_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.new_leader);
            w.string(&self.new_leader_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchStopSearching;

    impl ReqClanMatchStopSearching {
        pub const OPCODE: u32 = 355;
        pub const NAME: &'static str = "rfReqClanMatch_StopSearching";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchStopSearching;

    impl AckClanMatchStopSearching {
        pub const OPCODE: u32 = 356;
        pub const NAME: &'static str = "rfAckClanMatch_StopSearching";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqInviteGameRoom {
        pub guest_char_name: String,
    }

    impl ReqInviteGameRoom {
        pub const OPCODE: u32 = 357;
        pub const NAME: &'static str = "rfReqInviteGameRoom";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                guest_char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.guest_char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckInviteGameRoom {
        pub result: i32,
        pub answer: u8,
        pub guest_char_name: String,
    }

    impl AckInviteGameRoom {
        pub const OPCODE: u32 = 358;
        pub const NAME: &'static str = "rfAckInviteGameRoom";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                answer: r.u8()?,
                guest_char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u8(self.answer);
            w.string(&self.guest_char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRecvInviteGameRoom {
        pub invite_user_server_name: String,
        pub invite_user_server_id: i32,
        pub invite_user_channel_number: u16,
        pub room_unique_id: String,
        pub room_number: u16,
        pub room_name: String,
        pub room_state: u8,
        pub map_name: String,
        pub owner_name: String,
        pub game_mode: u8,
        pub weapon_limit: u8,
        pub is_team_balance: u8,
        pub password: String,
        pub invite_char_name: String,
    }

    impl ReqRecvInviteGameRoom {
        pub const OPCODE: u32 = 359;
        pub const NAME: &'static str = "rfReqRecvInviteGameRoom";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                invite_user_server_name: r.string()?,
                invite_user_server_id: r.i32()?,
                invite_user_channel_number: r.u16()?,
                room_unique_id: r.string()?,
                room_number: r.u16()?,
                room_name: r.string()?,
                room_state: r.u8()?,
                map_name: r.string()?,
                owner_name: r.string()?,
                game_mode: r.u8()?,
                weapon_limit: r.u8()?,
                is_team_balance: r.u8()?,
                password: r.string()?,
                invite_char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.invite_user_server_name);
            w.i32(self.invite_user_server_id);
            w.u16(self.invite_user_channel_number);
            w.string(&self.room_unique_id);
            w.u16(self.room_number);
            w.string(&self.room_name);
            w.u8(self.room_state);
            w.string(&self.map_name);
            w.string(&self.owner_name);
            w.u8(self.game_mode);
            w.u8(self.weapon_limit);
            w.u8(self.is_team_balance);
            w.string(&self.password);
            w.string(&self.invite_char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRecvInviteGameRoom {
        pub room_unique_id: String,
        pub room_number: u16,
        pub answer: u8,
        pub invite_char_name: String,
    }

    impl AckRecvInviteGameRoom {
        pub const OPCODE: u32 = 360;
        pub const NAME: &'static str = "rfAckRecvInviteGameRoom";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                room_unique_id: r.string()?,
                room_number: r.u16()?,
                answer: r.u8()?,
                invite_char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.room_unique_id);
            w.u16(self.room_number);
            w.u8(self.answer);
            w.string(&self.invite_char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRecvInviteGameRoomResult {
        pub result: i32,
    }

    impl AckRecvInviteGameRoomResult {
        pub const OPCODE: u32 = 361;
        pub const NAME: &'static str = "rfAckRecvInviteGameRoomResult";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLimitedListInChannel;

    impl ReqLimitedListInChannel {
        pub const OPCODE: u32 = 362;
        pub const NAME: &'static str = "rfReqLimitedListInChannel";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLimitedListInChannel {
        pub result: i32,
        pub limited_map_id: Vec<i32>,
        pub limited_mode_id: Vec<i32>,
    }

    impl AckLimitedListInChannel {
        pub const OPCODE: u32 = 363;
        pub const NAME: &'static str = "rfAckLimitedListInChannel";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                limited_map_id: r.array(|r| r.i32())?,
                limited_mode_id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.array(&self.limited_map_id, |w, v| { w.i32(*v); });
            w.array(&self.limited_mode_id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchStartSearching {
        pub request_count: i32,
    }

    impl ReqClanMatchStartSearching {
        pub const OPCODE: u32 = 364;
        pub const NAME: &'static str = "rfReqClanMatch_StartSearching";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                request_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.request_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchStartSearching {
        pub result: i32,
        pub err_msg: String,
        pub should_wait_more: i32,
    }

    impl AckClanMatchStartSearching {
        pub const OPCODE: u32 = 365;
        pub const NAME: &'static str = "rfAckClanMatch_StartSearching";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                should_wait_more: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.should_wait_more);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchNotifyStartSearching {
        pub should_wait_more: i32,
    }

    impl AckClanMatchNotifyStartSearching {
        pub const OPCODE: u32 = 366;
        pub const NAME: &'static str = "rfAckClanMatch_NotifyStartSearching";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                should_wait_more: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.should_wait_more);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchFoundMatch {
        pub channel_id: i32,
        pub room_number: i32,
    }

    impl ReqClanMatchFoundMatch {
        pub const OPCODE: u32 = 367;
        pub const NAME: &'static str = "rfReqClanMatch_FoundMatch";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                channel_id: r.i32()?,
                room_number: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.channel_id);
            w.i32(self.room_number);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchFoundMatch;

    impl AckClanMatchFoundMatch {
        pub const OPCODE: u32 = 368;
        pub const NAME: &'static str = "rfAckClanMatch_FoundMatch";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqPermanentRoomInfo {
        pub room_num: i32,
    }

    impl ReqPermanentRoomInfo {
        pub const OPCODE: u32 = 369;
        pub const NAME: &'static str = "rfReqPermanentRoomInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                room_num: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.room_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPermanentRoomInfo {
        pub result: i32,
        pub clan_idx: i32,
        pub fixed_game_mode: u8,
        pub fixed_max_user_cnt: u8,
    }

    impl AckPermanentRoomInfo {
        pub const OPCODE: u32 = 370;
        pub const NAME: &'static str = "rfAckPermanentRoomInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                clan_idx: r.i32()?,
                fixed_game_mode: r.u8()?,
                fixed_max_user_cnt: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.clan_idx);
            w.u8(self.fixed_game_mode);
            w.u8(self.fixed_max_user_cnt);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqQuestList;

    impl ReqQuestList {
        pub const OPCODE: u32 = 371;
        pub const NAME: &'static str = "rfReqQuestList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckQuestListStart {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckQuestListStart {
        pub const OPCODE: u32 = 372;
        pub const NAME: &'static str = "rfAckQuestListStart";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckQuestList {
        pub quest_id: Vec<i32>,
        pub count_prograss: Vec<i32>,
        pub quest_type: Vec<u8>,
        pub is_complete: Vec<u8>,
    }

    impl AckQuestList {
        pub const OPCODE: u32 = 373;
        pub const NAME: &'static str = "rfAckQuestList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                quest_id: r.array(|r| r.i32())?,
                count_prograss: r.array(|r| r.i32())?,
                quest_type: r.array(|r| r.u8())?,
                is_complete: r.array(|r| r.u8())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.quest_id, |w, v| { w.i32(*v); });
            w.array(&self.count_prograss, |w, v| { w.i32(*v); });
            w.array(&self.quest_type, |w, v| { w.u8(*v); });
            w.array(&self.is_complete, |w, v| { w.u8(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckQuestListEnd;

    impl AckQuestListEnd {
        pub const OPCODE: u32 = 374;
        pub const NAME: &'static str = "rfAckQuestListEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqQuestUpdate {
        pub unique_id: i32,
        pub game_room_num: i32,
        pub quest_id: Vec<i32>,
        pub count_prograss: Vec<i32>,
    }

    impl ReqQuestUpdate {
        pub const OPCODE: u32 = 375;
        pub const NAME: &'static str = "rfReqQuestUpdate";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                unique_id: r.i32()?,
                game_room_num: r.i32()?,
                quest_id: r.array(|r| r.i32())?,
                count_prograss: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.unique_id);
            w.i32(self.game_room_num);
            w.array(&self.quest_id, |w, v| { w.i32(*v); });
            w.array(&self.count_prograss, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckQuestUpdate {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckQuestUpdate {
        pub const OPCODE: u32 = 376;
        pub const NAME: &'static str = "rfAckQuestUpdate";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqQuestNewAvailable {
        pub achived_quest_id: Vec<i32>,
        pub new_quest_id: Vec<i32>,
        pub is_daily_quest: Vec<u8>,
        pub expired_quest: Vec<i32>,
    }

    impl ReqQuestNewAvailable {
        pub const OPCODE: u32 = 377;
        pub const NAME: &'static str = "rfReqQuestNewAvailable";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                achived_quest_id: r.array(|r| r.i32())?,
                new_quest_id: r.array(|r| r.i32())?,
                is_daily_quest: r.array(|r| r.u8())?,
                expired_quest: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.achived_quest_id, |w, v| { w.i32(*v); });
            w.array(&self.new_quest_id, |w, v| { w.i32(*v); });
            w.array(&self.is_daily_quest, |w, v| { w.u8(*v); });
            w.array(&self.expired_quest, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqDailyQuestNewAvailable {
        pub expired_quest_id: Vec<i32>,
        pub new_quest_id: Vec<i32>,
    }

    impl ReqDailyQuestNewAvailable {
        pub const OPCODE: u32 = 378;
        pub const NAME: &'static str = "rfReqDailyQuestNewAvailable";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                expired_quest_id: r.array(|r| r.i32())?,
                new_quest_id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.expired_quest_id, |w, v| { w.i32(*v); });
            w.array(&self.new_quest_id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqPCBangUserList;

    impl ReqPCBangUserList {
        pub const OPCODE: u32 = 379;
        pub const NAME: &'static str = "rfReqPCBangUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPCBangUserListStart {
        pub result: i32,
        pub total_count: i32,
    }

    impl AckPCBangUserListStart {
        pub const OPCODE: u32 = 380;
        pub const NAME: &'static str = "rfAckPCBangUserListStart";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                total_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.total_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPCBangUserList {
        pub friend_name: Vec<String>,
        pub clan_name: Vec<String>,
        pub cm_pattern: Vec<i32>,
        pub cm_bg: Vec<i32>,
        pub cm_bl: Vec<i32>,
        pub level: Vec<i32>,
        pub clan_grade: Vec<i32>,
        pub clan_win: Vec<i32>,
        pub clan_lose: Vec<i32>,
        pub is_look_for_clan: Vec<u8>,
        pub is_combat: Vec<u8>,
        pub clan_level: Vec<i32>,
        pub level_mark_id: Vec<i32>,
        pub title_mark_id: Vec<i32>,
        pub reserved1: Vec<i32>,
        pub reserved2: Vec<i32>,
        pub reserved3: Vec<i32>,
    }

    impl AckPCBangUserList {
        pub const OPCODE: u32 = 381;
        pub const NAME: &'static str = "rfAckPCBangUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                friend_name: r.array(|r| r.string())?,
                clan_name: r.array(|r| r.string())?,
                cm_pattern: r.array(|r| r.i32())?,
                cm_bg: r.array(|r| r.i32())?,
                cm_bl: r.array(|r| r.i32())?,
                level: r.array(|r| r.i32())?,
                clan_grade: r.array(|r| r.i32())?,
                clan_win: r.array(|r| r.i32())?,
                clan_lose: r.array(|r| r.i32())?,
                is_look_for_clan: r.array(|r| r.u8())?,
                is_combat: r.array(|r| r.u8())?,
                clan_level: r.array(|r| r.i32())?,
                level_mark_id: r.array(|r| r.i32())?,
                title_mark_id: r.array(|r| r.i32())?,
                reserved1: r.array(|r| r.i32())?,
                reserved2: r.array(|r| r.i32())?,
                reserved3: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.friend_name, |w, v| { w.string(v); });
            w.array(&self.clan_name, |w, v| { w.string(v); });
            w.array(&self.cm_pattern, |w, v| { w.i32(*v); });
            w.array(&self.cm_bg, |w, v| { w.i32(*v); });
            w.array(&self.cm_bl, |w, v| { w.i32(*v); });
            w.array(&self.level, |w, v| { w.i32(*v); });
            w.array(&self.clan_grade, |w, v| { w.i32(*v); });
            w.array(&self.clan_win, |w, v| { w.i32(*v); });
            w.array(&self.clan_lose, |w, v| { w.i32(*v); });
            w.array(&self.is_look_for_clan, |w, v| { w.u8(*v); });
            w.array(&self.is_combat, |w, v| { w.u8(*v); });
            w.array(&self.clan_level, |w, v| { w.i32(*v); });
            w.array(&self.level_mark_id, |w, v| { w.i32(*v); });
            w.array(&self.title_mark_id, |w, v| { w.i32(*v); });
            w.array(&self.reserved1, |w, v| { w.i32(*v); });
            w.array(&self.reserved2, |w, v| { w.i32(*v); });
            w.array(&self.reserved3, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPCBangUserListEnd;

    impl AckPCBangUserListEnd {
        pub const OPCODE: u32 = 382;
        pub const NAME: &'static str = "rfAckPCBangUserListEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRegisterMainPCBang;

    impl ReqRegisterMainPCBang {
        pub const OPCODE: u32 = 383;
        pub const NAME: &'static str = "rfReqRegisterMainPCBang";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRegisterMainPCBang {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckRegisterMainPCBang {
        pub const OPCODE: u32 = 384;
        pub const NAME: &'static str = "rfAckRegisterMainPCBang";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRegisteredPCBangInfo;

    impl ReqRegisteredPCBangInfo {
        pub const OPCODE: u32 = 385;
        pub const NAME: &'static str = "rfReqRegisteredPCBangInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRegisteredPCBangInfo {
        pub result: i32,
        pub err_msg: String,
        pub is_registered_pc_bang: u8,
        pub is_main_pc_bang: u8,
        pub pc_bang_name: String,
    }

    impl AckRegisteredPCBangInfo {
        pub const OPCODE: u32 = 386;
        pub const NAME: &'static str = "rfAckRegisteredPCBangInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                is_registered_pc_bang: r.u8()?,
                is_main_pc_bang: r.u8()?,
                pc_bang_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.u8(self.is_registered_pc_bang);
            w.u8(self.is_main_pc_bang);
            w.string(&self.pc_bang_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqWebzenCash;

    impl ReqWebzenCash {
        pub const OPCODE: u32 = 387;
        pub const NAME: &'static str = "rfReqWebzenCash";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckWebzenCash {
        pub result: i32,
        pub err_msg: String,
        pub p_coin_cash: f64,
        pub c_coin_cash: f64,
    }

    impl AckWebzenCash {
        pub const OPCODE: u32 = 388;
        pub const NAME: &'static str = "rfAckWebzenCash";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                p_coin_cash: r.f64()?,
                c_coin_cash: r.f64()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.f64(self.p_coin_cash);
            w.f64(self.c_coin_cash);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqWebzenShopScriptVersion;

    impl ReqWebzenShopScriptVersion {
        pub const OPCODE: u32 = 389;
        pub const NAME: &'static str = "rfReqWebzenShopScriptVersion";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckWebzenShopScriptVersion {
        pub year: i32,
        pub year_id: i32,
        pub sales_zone_code: i32,
    }

    impl AckWebzenShopScriptVersion {
        pub const OPCODE: u32 = 390;
        pub const NAME: &'static str = "rfAckWebzenShopScriptVersion";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                year: r.i32()?,
                year_id: r.i32()?,
                sales_zone_code: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.year);
            w.i32(self.year_id);
            w.i32(self.sales_zone_code);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqBuyItemByWebzenCashGP20 {
        pub product_no: i32,
        pub cointype: u8,
        pub item_id: f64,
    }

    impl ReqBuyItemByWebzenCashGP20 {
        pub const OPCODE: u32 = 391;
        pub const NAME: &'static str = "rfReqBuyItemByWebzenCashGP20";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                product_no: r.i32()?,
                cointype: r.u8()?,
                item_id: r.f64()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.product_no);
            w.u8(self.cointype);
            w.f64(self.item_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqBuyItemByWebzenCash {
        pub package_product_seq: i32,
        pub product_display_seq: i32,
        pub price_seq: i32,
        pub item_i_ds: Vec<i32>,
        pub item_unique_id: f64,
        pub buy_option1: u8,
        pub buy_option2: u8,
    }

    impl ReqBuyItemByWebzenCash {
        pub const OPCODE: u32 = 392;
        pub const NAME: &'static str = "rfReqBuyItemByWebzenCash";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                package_product_seq: r.i32()?,
                product_display_seq: r.i32()?,
                price_seq: r.i32()?,
                item_i_ds: r.array(|r| r.i32())?,
                item_unique_id: r.f64()?,
                buy_option1: r.u8()?,
                buy_option2: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.package_product_seq);
            w.i32(self.product_display_seq);
            w.i32(self.price_seq);
            w.array(&self.item_i_ds, |w, v| { w.i32(*v); });
            w.f64(self.item_unique_id);
            w.u8(self.buy_option1);
            w.u8(self.buy_option2);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckBuyItemByWebzenCash {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckBuyItemByWebzenCash {
        pub const OPCODE: u32 = 393;
        pub const NAME: &'static str = "rfAckBuyItemByWebzenCash";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGiftItemByWebzenCashGP20 {
        pub receiver_char_name: String,
        pub message: String,
        pub product_no: i32,
        pub cointype: u8,
    }

    impl ReqGiftItemByWebzenCashGP20 {
        pub const OPCODE: u32 = 394;
        pub const NAME: &'static str = "rfReqGiftItemByWebzenCashGP20";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                receiver_char_name: r.string()?,
                message: r.string()?,
                product_no: r.i32()?,
                cointype: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.receiver_char_name);
            w.string(&self.message);
            w.i32(self.product_no);
            w.u8(self.cointype);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGiftItemByWebzenCash {
        pub receiver_char_name: String,
        pub message: String,
        pub package_product_seq: i32,
        pub price_seq: i32,
        pub product_display_seq: i32,
    }

    impl ReqGiftItemByWebzenCash {
        pub const OPCODE: u32 = 395;
        pub const NAME: &'static str = "rfReqGiftItemByWebzenCash";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                receiver_char_name: r.string()?,
                message: r.string()?,
                package_product_seq: r.i32()?,
                price_seq: r.i32()?,
                product_display_seq: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.receiver_char_name);
            w.string(&self.message);
            w.i32(self.package_product_seq);
            w.i32(self.price_seq);
            w.i32(self.product_display_seq);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGiftItemByWebzenCash {
        pub result: i32,
        pub err_msg: String,
        pub left_product_count: i32,
    }

    impl AckGiftItemByWebzenCash {
        pub const OPCODE: u32 = 396;
        pub const NAME: &'static str = "rfAckGiftItemByWebzenCash";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                left_product_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.left_product_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqWebzenBillingStorageList {
        pub page: i32,
        pub count_for_page: i32,
        pub type_: u8,
    }

    impl ReqWebzenBillingStorageList {
        pub const OPCODE: u32 = 397;
        pub const NAME: &'static str = "rfReqWebzenBillingStorageList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                page: r.i32()?,
                count_for_page: r.i32()?,
                type_: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.page);
            w.i32(self.count_for_page);
            w.u8(self.type_);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckWebzenBillingStorageListStart {
        pub result: i32,
        pub err_msg: String,
        pub total_count: i32,
        pub total_page: i32,
        pub cur_page: i32,
    }

    impl AckWebzenBillingStorageListStart {
        pub const OPCODE: u32 = 398;
        pub const NAME: &'static str = "rfAckWebzenBillingStorageListStart";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                total_count: r.i32()?,
                total_page: r.i32()?,
                cur_page: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.total_count);
            w.i32(self.total_page);
            w.i32(self.cur_page);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckWebzenBillingStorageList {
        pub seq: Vec<i32>,
        pub item_seq: Vec<i32>,
        pub group_code: Vec<i32>,
        pub share_flag: Vec<i32>,
        pub product_seq: Vec<i32>,
        pub cash_name: Vec<String>,
        pub cash_point: Vec<f64>,
        pub send_account_id: Vec<String>,
        pub send_message: Vec<String>,
        pub item_type: Vec<u8>,
        pub relation_type: Vec<u8>,
        pub product_type: Vec<i32>,
        pub price_seq: Vec<i32>,
    }

    impl AckWebzenBillingStorageList {
        pub const OPCODE: u32 = 399;
        pub const NAME: &'static str = "rfAckWebzenBillingStorageList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                seq: r.array(|r| r.i32())?,
                item_seq: r.array(|r| r.i32())?,
                group_code: r.array(|r| r.i32())?,
                share_flag: r.array(|r| r.i32())?,
                product_seq: r.array(|r| r.i32())?,
                cash_name: r.array(|r| r.string())?,
                cash_point: r.array(|r| r.f64())?,
                send_account_id: r.array(|r| r.string())?,
                send_message: r.array(|r| r.string())?,
                item_type: r.array(|r| r.u8())?,
                relation_type: r.array(|r| r.u8())?,
                product_type: r.array(|r| r.i32())?,
                price_seq: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.seq, |w, v| { w.i32(*v); });
            w.array(&self.item_seq, |w, v| { w.i32(*v); });
            w.array(&self.group_code, |w, v| { w.i32(*v); });
            w.array(&self.share_flag, |w, v| { w.i32(*v); });
            w.array(&self.product_seq, |w, v| { w.i32(*v); });
            w.array(&self.cash_name, |w, v| { w.string(v); });
            w.array(&self.cash_point, |w, v| { w.f64(*v); });
            w.array(&self.send_account_id, |w, v| { w.string(v); });
            w.array(&self.send_message, |w, v| { w.string(v); });
            w.array(&self.item_type, |w, v| { w.u8(*v); });
            w.array(&self.relation_type, |w, v| { w.u8(*v); });
            w.array(&self.product_type, |w, v| { w.i32(*v); });
            w.array(&self.price_seq, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckWebzenBillingStorageListEnd;

    impl AckWebzenBillingStorageListEnd {
        pub const OPCODE: u32 = 400;
        pub const NAME: &'static str = "rfAckWebzenBillingStorageListEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqWebzenBillingUseStorageItem {
        pub storage_seq: i32,
        pub storage_item_seq: i32,
        pub storage_item_type: u8,
    }

    impl ReqWebzenBillingUseStorageItem {
        pub const OPCODE: u32 = 401;
        pub const NAME: &'static str = "rfReqWebzenBillingUseStorageItem";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                storage_seq: r.i32()?,
                storage_item_seq: r.i32()?,
                storage_item_type: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.storage_seq);
            w.i32(self.storage_item_seq);
            w.u8(self.storage_item_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckWebzenBillingUseStorageItem {
        pub result: i32,
        pub err_msg: String,
        pub item_idx: f64,
        pub item_id: i32,
        pub item_extra: f64,
    }

    impl AckWebzenBillingUseStorageItem {
        pub const OPCODE: u32 = 402;
        pub const NAME: &'static str = "rfAckWebzenBillingUseStorageItem";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                item_idx: r.f64()?,
                item_id: r.i32()?,
                item_extra: r.f64()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.f64(self.item_idx);
            w.i32(self.item_id);
            w.f64(self.item_extra);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqParamCheckSum {
        pub key: String,
        pub value: String,
    }

    impl ReqParamCheckSum {
        pub const OPCODE: u32 = 403;
        pub const NAME: &'static str = "rfReqParamCheckSum";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                key: r.string()?,
                value: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.key);
            w.string(&self.value);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckParamCheckSum {
        pub result: i32,
    }

    impl AckParamCheckSum {
        pub const OPCODE: u32 = 404;
        pub const NAME: &'static str = "rfAckParamCheckSum";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchChangeRoomSetting {
        pub wait_room_index: i32,
        pub map_num_array: Vec<i32>,
        pub versus_mode: i32,
        pub is_public: u8,
    }

    impl ReqClanMatchChangeRoomSetting {
        pub const OPCODE: u32 = 405;
        pub const NAME: &'static str = "rfReqClanMatch_ChangeRoomSetting";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                wait_room_index: r.i32()?,
                map_num_array: r.array(|r| r.i32())?,
                versus_mode: r.i32()?,
                is_public: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.wait_room_index);
            w.array(&self.map_num_array, |w, v| { w.i32(*v); });
            w.i32(self.versus_mode);
            w.u8(self.is_public);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchChangeRoomSetting {
        pub wait_room_index: i32,
        pub result: i32,
        pub err_msg: String,
        pub map_num_array: Vec<i32>,
        pub versus_mode: i32,
        pub is_public: u8,
    }

    impl AckClanMatchChangeRoomSetting {
        pub const OPCODE: u32 = 406;
        pub const NAME: &'static str = "rfAckClanMatch_ChangeRoomSetting";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                wait_room_index: r.i32()?,
                result: r.i32()?,
                err_msg: r.string()?,
                map_num_array: r.array(|r| r.i32())?,
                versus_mode: r.i32()?,
                is_public: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.wait_room_index);
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.map_num_array, |w, v| { w.i32(*v); });
            w.i32(self.versus_mode);
            w.u8(self.is_public);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGetBotID {
        pub af_count: u8,
        pub rsa_count: u8,
    }

    impl ReqGetBotID {
        pub const OPCODE: u32 = 407;
        pub const NAME: &'static str = "rfReqGetBotID";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                af_count: r.u8()?,
                rsa_count: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.af_count);
            w.u8(self.rsa_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGetBotID {
        pub result: i32,
        pub requested_af_count: u8,
        pub requested_rsa_count: u8,
        pub af_bot_id: Vec<i32>,
        pub rsa_bot_id: Vec<i32>,
    }

    impl AckGetBotID {
        pub const OPCODE: u32 = 408;
        pub const NAME: &'static str = "rfAckGetBotID";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                requested_af_count: r.u8()?,
                requested_rsa_count: r.u8()?,
                af_bot_id: r.array(|r| r.i32())?,
                rsa_bot_id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u8(self.requested_af_count);
            w.u8(self.requested_rsa_count);
            w.array(&self.af_bot_id, |w, v| { w.i32(*v); });
            w.array(&self.rsa_bot_id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqReturnBotID {
        pub bot_id: Vec<i32>,
    }

    impl ReqReturnBotID {
        pub const OPCODE: u32 = 409;
        pub const NAME: &'static str = "rfReqReturnBotID";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                bot_id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.bot_id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckReturnBotID {
        pub result: i32,
    }

    impl AckReturnBotID {
        pub const OPCODE: u32 = 410;
        pub const NAME: &'static str = "rfAckReturnBotID";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchGetClanWaittingRoomInfo;

    impl ReqClanMatchGetClanWaittingRoomInfo {
        pub const OPCODE: u32 = 411;
        pub const NAME: &'static str = "rfReqClanMatch_GetClanWaittingRoomInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchGetClanWaittingRoomInfo {
        pub wait_room_index: i32,
        pub result: i32,
        pub err_msg: String,
        pub map_num_array: Vec<i32>,
        pub versus_mode: i32,
        pub wait_room_name: String,
    }

    impl AckClanMatchGetClanWaittingRoomInfo {
        pub const OPCODE: u32 = 412;
        pub const NAME: &'static str = "rfAckClanMatch_GetClanWaittingRoomInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                wait_room_index: r.i32()?,
                result: r.i32()?,
                err_msg: r.string()?,
                map_num_array: r.array(|r| r.i32())?,
                versus_mode: r.i32()?,
                wait_room_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.wait_room_index);
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.map_num_array, |w, v| { w.i32(*v); });
            w.i32(self.versus_mode);
            w.string(&self.wait_room_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqIsPaidWebzenPCBang {
        pub is_paid: u8,
    }

    impl ReqIsPaidWebzenPCBang {
        pub const OPCODE: u32 = 413;
        pub const NAME: &'static str = "rfReqIsPaidWebzenPCBang";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                is_paid: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.is_paid);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqCharBlockInfo;

    impl ReqCharBlockInfo {
        pub const OPCODE: u32 = 414;
        pub const NAME: &'static str = "rfReqCharBlockInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckCharBlockInfo {
        pub result: i32,
        pub chat_block_start_time: f64,
        pub chat_block_end_time: f64,
        pub blocked_item_idx: Vec<f64>,
    }

    impl AckCharBlockInfo {
        pub const OPCODE: u32 = 415;
        pub const NAME: &'static str = "rfAckCharBlockInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                chat_block_start_time: r.f64()?,
                chat_block_end_time: r.f64()?,
                blocked_item_idx: r.array(|r| r.f64())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.f64(self.chat_block_start_time);
            w.f64(self.chat_block_end_time);
            w.array(&self.blocked_item_idx, |w, v| { w.f64(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqQuestGetAvailableEventList;

    impl ReqQuestGetAvailableEventList {
        pub const OPCODE: u32 = 416;
        pub const NAME: &'static str = "rfReqQuestGetAvailableEventList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckQuestGetAvailableEventList {
        pub result: i32,
        pub err_msg: String,
        pub quest_id: Vec<i32>,
    }

    impl AckQuestGetAvailableEventList {
        pub const OPCODE: u32 = 417;
        pub const NAME: &'static str = "rfAckQuestGetAvailableEventList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                quest_id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.quest_id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGMEnterRoom {
        pub result: i32,
        pub room_number: u16,
        pub user_id: i32,
    }

    impl AckGMEnterRoom {
        pub const OPCODE: u32 = 418;
        pub const NAME: &'static str = "rfAckGMEnterRoom";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                room_number: r.u16()?,
                user_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u16(self.room_number);
            w.i32(self.user_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGMLeaveRoom {
        pub result: i32,
        pub room_number: u16,
        pub user_id: i32,
    }

    impl AckGMLeaveRoom {
        pub const OPCODE: u32 = 419;
        pub const NAME: &'static str = "rfAckGMLeaveRoom";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                room_number: r.u16()?,
                user_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u16(self.room_number);
            w.i32(self.user_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqChangeRoomTitleGM {
        pub room_number: u16,
        pub new_room_title: String,
    }

    impl ReqChangeRoomTitleGM {
        pub const OPCODE: u32 = 420;
        pub const NAME: &'static str = "rfReqChangeRoomTitle_GM";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                room_number: r.u16()?,
                new_room_title: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.room_number);
            w.string(&self.new_room_title);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChangeRoomTitleGM {
        pub result: i32,
        pub room_number: u16,
        pub new_room_title: String,
    }

    impl AckChangeRoomTitleGM {
        pub const OPCODE: u32 = 421;
        pub const NAME: &'static str = "rfAckChangeRoomTitle_GM";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                room_number: r.u16()?,
                new_room_title: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u16(self.room_number);
            w.string(&self.new_room_title);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChangedRoomTitleFromGM {
        pub result: i32,
        pub room_number: u16,
        pub new_room_title: String,
    }

    impl AckChangedRoomTitleFromGM {
        pub const OPCODE: u32 = 422;
        pub const NAME: &'static str = "rfAckChangedRoomTitle_FromGM";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                room_number: r.u16()?,
                new_room_title: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u16(self.room_number);
            w.string(&self.new_room_title);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRandomBoxUse {
        pub item_idx_random_box: f64,
        pub item_idx_key: f64,
    }

    impl ReqRandomBoxUse {
        pub const OPCODE: u32 = 423;
        pub const NAME: &'static str = "rfReqRandomBox_Use";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_idx_random_box: r.f64()?,
                item_idx_key: r.f64()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.f64(self.item_idx_random_box);
            w.f64(self.item_idx_key);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRandomBoxUse {
        pub result: i32,
        pub err_msg: String,
        pub random_box_item_idx: f64,
        pub random_bxo_item_count: i32,
        pub key_item_idx: f64,
        pub key_item_remain_count: i32,
        pub item_id_choose: Vec<i32>,
        pub part_id_choose: Vec<i32>,
        pub period_on_choosen_item: Vec<i32>,
        pub stack_count_on_choosen_item: Vec<i32>,
        pub b_have: Vec<u8>,
        pub item_idx: Vec<f64>,
        pub until_time: Vec<f64>,
        pub stack_count: Vec<i32>,
    }

    impl AckRandomBoxUse {
        pub const OPCODE: u32 = 424;
        pub const NAME: &'static str = "rfAckRandomBox_Use";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                random_box_item_idx: r.f64()?,
                random_bxo_item_count: r.i32()?,
                key_item_idx: r.f64()?,
                key_item_remain_count: r.i32()?,
                item_id_choose: r.array(|r| r.i32())?,
                part_id_choose: r.array(|r| r.i32())?,
                period_on_choosen_item: r.array(|r| r.i32())?,
                stack_count_on_choosen_item: r.array(|r| r.i32())?,
                b_have: r.array(|r| r.u8())?,
                item_idx: r.array(|r| r.f64())?,
                until_time: r.array(|r| r.f64())?,
                stack_count: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.f64(self.random_box_item_idx);
            w.i32(self.random_bxo_item_count);
            w.f64(self.key_item_idx);
            w.i32(self.key_item_remain_count);
            w.array(&self.item_id_choose, |w, v| { w.i32(*v); });
            w.array(&self.part_id_choose, |w, v| { w.i32(*v); });
            w.array(&self.period_on_choosen_item, |w, v| { w.i32(*v); });
            w.array(&self.stack_count_on_choosen_item, |w, v| { w.i32(*v); });
            w.array(&self.b_have, |w, v| { w.u8(*v); });
            w.array(&self.item_idx, |w, v| { w.f64(*v); });
            w.array(&self.until_time, |w, v| { w.f64(*v); });
            w.array(&self.stack_count, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSendSystemMsg {
        pub recv_type: u8,
        pub send_char_name: String,
        pub recv_charname: String,
        pub msg_info: SystemMsgParam,
    }

    impl ReqSendSystemMsg {
        pub const OPCODE: u32 = 425;
        pub const NAME: &'static str = "rfReqSendSystemMsg";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                recv_type: r.u8()?,
                send_char_name: r.string()?,
                recv_charname: r.string()?,
                msg_info: SystemMsgParam::decode(r)?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.recv_type);
            w.string(&self.send_char_name);
            w.string(&self.recv_charname);
            self.msg_info.encode(w);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckSendSystemMsg {
        pub result: i32,
    }

    impl AckSendSystemMsg {
        pub const OPCODE: u32 = 426;
        pub const NAME: &'static str = "rfAckSendSystemMsg";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRecvSystemMsg {
        pub community_type: u8,
        pub msg_info: SystemMsgParam,
    }

    impl AckRecvSystemMsg {
        pub const OPCODE: u32 = 427;
        pub const NAME: &'static str = "rfAckRecvSystemMsg";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                community_type: r.u8()?,
                msg_info: SystemMsgParam::decode(r)?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.community_type);
            self.msg_info.encode(w);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqPaidItemChangeCharname {
        pub item_idx: f64,
        pub new_charname: String,
    }

    impl ReqPaidItemChangeCharname {
        pub const OPCODE: u32 = 428;
        pub const NAME: &'static str = "rfReqPaidItem_ChangeCharname";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_idx: r.f64()?,
                new_charname: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.f64(self.item_idx);
            w.string(&self.new_charname);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPaidItemChangeCharname {
        pub result: i32,
        pub err_msg: String,
        pub item_idx: f64,
        pub remain_count: i32,
        pub new_charname: String,
    }

    impl AckPaidItemChangeCharname {
        pub const OPCODE: u32 = 429;
        pub const NAME: &'static str = "rfAckPaidItem_ChangeCharname";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                item_idx: r.f64()?,
                remain_count: r.i32()?,
                new_charname: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.f64(self.item_idx);
            w.i32(self.remain_count);
            w.string(&self.new_charname);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqPaidItemEraseRecord {
        pub item_idx: f64,
        pub command: i32,
    }

    impl ReqPaidItemEraseRecord {
        pub const OPCODE: u32 = 430;
        pub const NAME: &'static str = "rfReqPaidItem_EraseRecord";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_idx: r.f64()?,
                command: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.f64(self.item_idx);
            w.i32(self.command);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPaidItemEraseRecord {
        pub result: i32,
        pub err_msg: String,
        pub item_idx: f64,
        pub command: i32,
        pub remain_count: i32,
    }

    impl AckPaidItemEraseRecord {
        pub const OPCODE: u32 = 431;
        pub const NAME: &'static str = "rfAckPaidItem_EraseRecord";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                item_idx: r.f64()?,
                command: r.i32()?,
                remain_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.f64(self.item_idx);
            w.i32(self.command);
            w.i32(self.remain_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqPaidItemBroadCast {
        pub item_idx: f64,
        pub command: i32,
        pub message: String,
    }

    impl ReqPaidItemBroadCast {
        pub const OPCODE: u32 = 432;
        pub const NAME: &'static str = "rfReqPaidItem_BroadCast";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_idx: r.f64()?,
                command: r.i32()?,
                message: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.f64(self.item_idx);
            w.i32(self.command);
            w.string(&self.message);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPaidItemBroadCast {
        pub result: i32,
        pub err_msg: String,
        pub item_idx: f64,
        pub stack_count: i32,
    }

    impl AckPaidItemBroadCast {
        pub const OPCODE: u32 = 433;
        pub const NAME: &'static str = "rfAckPaidItem_BroadCast";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                item_idx: r.f64()?,
                stack_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.f64(self.item_idx);
            w.i32(self.stack_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPaidItemRelayBroadcast {
        pub char_name: String,
        pub msg: String,
    }

    impl AckPaidItemRelayBroadcast {
        pub const OPCODE: u32 = 434;
        pub const NAME: &'static str = "rfAckPaidItem_RelayBroadcast";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
                msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            w.string(&self.msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqPaidItemClanMarkDupCheck {
        pub is_paid: u8,
        pub pattern: i32,
        pub bg: i32,
        pub bl: i32,
    }

    impl ReqPaidItemClanMarkDupCheck {
        pub const OPCODE: u32 = 435;
        pub const NAME: &'static str = "rfReqPaidItem_ClanMarkDupCheck";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                is_paid: r.u8()?,
                pattern: r.i32()?,
                bg: r.i32()?,
                bl: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.is_paid);
            w.i32(self.pattern);
            w.i32(self.bg);
            w.i32(self.bl);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPaidItemClanMarkDupCheck {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckPaidItemClanMarkDupCheck {
        pub const OPCODE: u32 = 436;
        pub const NAME: &'static str = "rfAckPaidItem_ClanMarkDupCheck";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqPaidItemChangeClanMark {
        pub item_idx: f64,
        pub is_paid: u8,
        pub pattern: i32,
        pub bg: i32,
        pub bl: i32,
    }

    impl ReqPaidItemChangeClanMark {
        pub const OPCODE: u32 = 437;
        pub const NAME: &'static str = "rfReqPaidItem_ChangeClanMark";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_idx: r.f64()?,
                is_paid: r.u8()?,
                pattern: r.i32()?,
                bg: r.i32()?,
                bl: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.f64(self.item_idx);
            w.u8(self.is_paid);
            w.i32(self.pattern);
            w.i32(self.bg);
            w.i32(self.bl);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPaidItemChangeClanMark {
        pub result: i32,
        pub err_msg: String,
        pub item_idx: f64,
        pub stack_count: i32,
    }

    impl AckPaidItemChangeClanMark {
        pub const OPCODE: u32 = 438;
        pub const NAME: &'static str = "rfAckPaidItem_ChangeClanMark";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                item_idx: r.f64()?,
                stack_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.f64(self.item_idx);
            w.i32(self.stack_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPaidItemNotifyChangeClanMark {
        pub clan_name: String,
        pub pattern: i32,
        pub bg: i32,
        pub bl: i32,
    }

    impl AckPaidItemNotifyChangeClanMark {
        pub const OPCODE: u32 = 439;
        pub const NAME: &'static str = "rfAckPaidItem_NotifyChangeClanMark";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
                pattern: r.i32()?,
                bg: r.i32()?,
                bl: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            w.i32(self.pattern);
            w.i32(self.bg);
            w.i32(self.bl);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRemoveCheckTeamBalanceList;

    impl ReqRemoveCheckTeamBalanceList {
        pub const OPCODE: u32 = 440;
        pub const NAME: &'static str = "rfReqRemoveCheckTeamBalanceList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchSetWaitRoomPublic {
        pub set: u8,
    }

    impl ReqClanMatchSetWaitRoomPublic {
        pub const OPCODE: u32 = 441;
        pub const NAME: &'static str = "rfReqClanMatch_SetWaitRoomPublic";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                set: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.set);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchSetWaitRoomPublic {
        pub result: i32,
        pub err_msg: String,
        pub set: u8,
    }

    impl AckClanMatchSetWaitRoomPublic {
        pub const OPCODE: u32 = 442;
        pub const NAME: &'static str = "rfAckClanMatch_SetWaitRoomPublic";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                set: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.u8(self.set);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqExteriorRoomOwnerInfo {
        pub room_num: u16,
        pub owner_name: String,
        pub owner_ip: String,
    }

    impl ReqExteriorRoomOwnerInfo {
        pub const OPCODE: u32 = 443;
        pub const NAME: &'static str = "rfReqExteriorRoomOwnerInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                room_num: r.u16()?,
                owner_name: r.string()?,
                owner_ip: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.room_num);
            w.string(&self.owner_name);
            w.string(&self.owner_ip);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckExteriorRoomOwnerInfo;

    impl AckExteriorRoomOwnerInfo {
        pub const OPCODE: u32 = 444;
        pub const NAME: &'static str = "rfAckExteriorRoomOwnerInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSetLookForClan {
        pub is_looking_for: u8,
    }

    impl ReqSetLookForClan {
        pub const OPCODE: u32 = 445;
        pub const NAME: &'static str = "rfReqSetLookForClan";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                is_looking_for: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.is_looking_for);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckSetLookForClan {
        pub result: i32,
        pub err_msg: String,
        pub is_looking_for: u8,
    }

    impl AckSetLookForClan {
        pub const OPCODE: u32 = 446;
        pub const NAME: &'static str = "rfAckSetLookForClan";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                is_looking_for: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.u8(self.is_looking_for);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGetMyMailBoxState;

    impl ReqGetMyMailBoxState {
        pub const OPCODE: u32 = 447;
        pub const NAME: &'static str = "rfReqGetMyMailBoxState";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGetMyMailBoxState {
        pub result: i32,
        pub is_full: u8,
        pub new_mail_count: u16,
    }

    impl AckGetMyMailBoxState {
        pub const OPCODE: u32 = 448;
        pub const NAME: &'static str = "rfAckGetMyMailBoxState";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                is_full: r.u8()?,
                new_mail_count: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u8(self.is_full);
            w.u16(self.new_mail_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqMyEquipedItemList;

    impl ReqMyEquipedItemList {
        pub const OPCODE: u32 = 449;
        pub const NAME: &'static str = "rfReqMyEquipedItemList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckMyEquipedItemList {
        pub item_i_ds: Vec<i32>,
        pub slot_positions: Vec<i32>,
        pub add_parts_id: Vec<i32>,
        pub paint_id: Vec<i32>,
        pub unique_id: Vec<f64>,
        pub result: i32,
    }

    impl AckMyEquipedItemList {
        pub const OPCODE: u32 = 450;
        pub const NAME: &'static str = "rfAckMyEquipedItemList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_i_ds: r.array(|r| r.i32())?,
                slot_positions: r.array(|r| r.i32())?,
                add_parts_id: r.array(|r| r.i32())?,
                paint_id: r.array(|r| r.i32())?,
                unique_id: r.array(|r| r.f64())?,
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.item_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.slot_positions, |w, v| { w.i32(*v); });
            w.array(&self.add_parts_id, |w, v| { w.i32(*v); });
            w.array(&self.paint_id, |w, v| { w.i32(*v); });
            w.array(&self.unique_id, |w, v| { w.f64(*v); });
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqMyEquipedSkillList;

    impl ReqMyEquipedSkillList {
        pub const OPCODE: u32 = 451;
        pub const NAME: &'static str = "rfReqMyEquipedSkillList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckMyEquipedSkillList {
        pub skill_i_ds: Vec<i32>,
        pub slot_positions: Vec<i32>,
        pub result: i32,
    }

    impl AckMyEquipedSkillList {
        pub const OPCODE: u32 = 452;
        pub const NAME: &'static str = "rfAckMyEquipedSkillList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                skill_i_ds: r.array(|r| r.i32())?,
                slot_positions: r.array(|r| r.i32())?,
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.skill_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.slot_positions, |w, v| { w.i32(*v); });
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqMyEquipedQSlotList;

    impl ReqMyEquipedQSlotList {
        pub const OPCODE: u32 = 453;
        pub const NAME: &'static str = "rfReqMyEquipedQSlotList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckMyEquipedQSlotList {
        pub q_slot_disp_order: Vec<u8>,
        pub mw_unique_id: Vec<f64>,
        pub mw_item_id: Vec<i32>,
        pub mw_part_id: Vec<i32>,
        pub mw_paint_id: Vec<i32>,
        pub pistol_unique_id: Vec<f64>,
        pub pistol_item_id: Vec<i32>,
        pub pistol_part_id: Vec<i32>,
        pub pistol_paint_id: Vec<i32>,
        pub tw1unique_id: Vec<f64>,
        pub tw_item_id1: Vec<i32>,
        pub tw2unique_id: Vec<f64>,
        pub tw_item_id2: Vec<i32>,
        pub skill1id: Vec<i32>,
        pub skill2id: Vec<i32>,
        pub skill3id: Vec<i32>,
        pub skill4id: Vec<i32>,
        pub result: i32,
    }

    impl AckMyEquipedQSlotList {
        pub const OPCODE: u32 = 454;
        pub const NAME: &'static str = "rfAckMyEquipedQSlotList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                q_slot_disp_order: r.array(|r| r.u8())?,
                mw_unique_id: r.array(|r| r.f64())?,
                mw_item_id: r.array(|r| r.i32())?,
                mw_part_id: r.array(|r| r.i32())?,
                mw_paint_id: r.array(|r| r.i32())?,
                pistol_unique_id: r.array(|r| r.f64())?,
                pistol_item_id: r.array(|r| r.i32())?,
                pistol_part_id: r.array(|r| r.i32())?,
                pistol_paint_id: r.array(|r| r.i32())?,
                tw1unique_id: r.array(|r| r.f64())?,
                tw_item_id1: r.array(|r| r.i32())?,
                tw2unique_id: r.array(|r| r.f64())?,
                tw_item_id2: r.array(|r| r.i32())?,
                skill1id: r.array(|r| r.i32())?,
                skill2id: r.array(|r| r.i32())?,
                skill3id: r.array(|r| r.i32())?,
                skill4id: r.array(|r| r.i32())?,
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.q_slot_disp_order, |w, v| { w.u8(*v); });
            w.array(&self.mw_unique_id, |w, v| { w.f64(*v); });
            w.array(&self.mw_item_id, |w, v| { w.i32(*v); });
            w.array(&self.mw_part_id, |w, v| { w.i32(*v); });
            w.array(&self.mw_paint_id, |w, v| { w.i32(*v); });
            w.array(&self.pistol_unique_id, |w, v| { w.f64(*v); });
            w.array(&self.pistol_item_id, |w, v| { w.i32(*v); });
            w.array(&self.pistol_part_id, |w, v| { w.i32(*v); });
            w.array(&self.pistol_paint_id, |w, v| { w.i32(*v); });
            w.array(&self.tw1unique_id, |w, v| { w.f64(*v); });
            w.array(&self.tw_item_id1, |w, v| { w.i32(*v); });
            w.array(&self.tw2unique_id, |w, v| { w.f64(*v); });
            w.array(&self.tw_item_id2, |w, v| { w.i32(*v); });
            w.array(&self.skill1id, |w, v| { w.i32(*v); });
            w.array(&self.skill2id, |w, v| { w.i32(*v); });
            w.array(&self.skill3id, |w, v| { w.i32(*v); });
            w.array(&self.skill4id, |w, v| { w.i32(*v); });
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRoomUserPingInfo {
        pub unque_id: Vec<i32>,
        pub ping: Vec<i32>,
        pub host_priority_point: i32,
    }

    impl ReqRoomUserPingInfo {
        pub const OPCODE: u32 = 455;
        pub const NAME: &'static str = "rfReqRoomUserPingInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                unque_id: r.array(|r| r.i32())?,
                ping: r.array(|r| r.i32())?,
                host_priority_point: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.unque_id, |w, v| { w.i32(*v); });
            w.array(&self.ping, |w, v| { w.i32(*v); });
            w.i32(self.host_priority_point);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRoomUserPingInfo {
        pub unque_id: Vec<i32>,
        pub ping: Vec<i32>,
        pub host_priority_point: i32,
    }

    impl AckRoomUserPingInfo {
        pub const OPCODE: u32 = 456;
        pub const NAME: &'static str = "rfAckRoomUserPingInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                unque_id: r.array(|r| r.i32())?,
                ping: r.array(|r| r.i32())?,
                host_priority_point: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.unque_id, |w, v| { w.i32(*v); });
            w.array(&self.ping, |w, v| { w.i32(*v); });
            w.i32(self.host_priority_point);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyStartTutorial;

    impl ReqNotifyStartTutorial {
        pub const OPCODE: u32 = 457;
        pub const NAME: &'static str = "rfReqNotifyStartTutorial";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNotifyStartTutorial {
        pub result: i32,
        pub user_id: i32,
    }

    impl AckNotifyStartTutorial {
        pub const OPCODE: u32 = 458;
        pub const NAME: &'static str = "rfAckNotifyStartTutorial";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                user_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.user_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyEndTutorial {
        pub tutorial_result_type: i32,
    }

    impl ReqNotifyEndTutorial {
        pub const OPCODE: u32 = 459;
        pub const NAME: &'static str = "rfReqNotifyEndTutorial";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                tutorial_result_type: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.tutorial_result_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNotifyEndTutorial {
        pub user_id: i32,
        pub char_name: String,
        pub level: i32,
        pub clan_name: String,
        pub clan_mark_pattern: i32,
        pub clan_mark_bg: i32,
        pub clan_mark_bl: i32,
        pub clan_grade: i32,
        pub clan_win: i32,
        pub clan_lose: i32,
        pub is_look_for: u8,
        pub result: i32,
        pub clan_level: i32,
        pub level_mark_id: i32,
        pub title_mark_id: i32,
        pub reserved1: i32,
        pub reserved2: i32,
        pub reserved3: i32,
    }

    impl AckNotifyEndTutorial {
        pub const OPCODE: u32 = 460;
        pub const NAME: &'static str = "rfAckNotifyEndTutorial";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
                char_name: r.string()?,
                level: r.i32()?,
                clan_name: r.string()?,
                clan_mark_pattern: r.i32()?,
                clan_mark_bg: r.i32()?,
                clan_mark_bl: r.i32()?,
                clan_grade: r.i32()?,
                clan_win: r.i32()?,
                clan_lose: r.i32()?,
                is_look_for: r.u8()?,
                result: r.i32()?,
                clan_level: r.i32()?,
                level_mark_id: r.i32()?,
                title_mark_id: r.i32()?,
                reserved1: r.i32()?,
                reserved2: r.i32()?,
                reserved3: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            w.string(&self.char_name);
            w.i32(self.level);
            w.string(&self.clan_name);
            w.i32(self.clan_mark_pattern);
            w.i32(self.clan_mark_bg);
            w.i32(self.clan_mark_bl);
            w.i32(self.clan_grade);
            w.i32(self.clan_win);
            w.i32(self.clan_lose);
            w.u8(self.is_look_for);
            w.i32(self.result);
            w.i32(self.clan_level);
            w.i32(self.level_mark_id);
            w.i32(self.title_mark_id);
            w.i32(self.reserved1);
            w.i32(self.reserved2);
            w.i32(self.reserved3);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqServerShutdown {
        pub auth_key: String,
        pub second_after: i32,
        pub shutdown_time: f64,
    }

    impl ReqServerShutdown {
        pub const OPCODE: u32 = 461;
        pub const NAME: &'static str = "rfReqServerShutdown";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                auth_key: r.string()?,
                second_after: r.i32()?,
                shutdown_time: r.f64()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.auth_key);
            w.i32(self.second_after);
            w.f64(self.shutdown_time);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckServerShutdown {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckServerShutdown {
        pub const OPCODE: u32 = 462;
        pub const NAME: &'static str = "rfAckServerShutdown";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqServerType;

    impl ReqServerType {
        pub const OPCODE: u32 = 463;
        pub const NAME: &'static str = "rfReqServerType";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckServerType {
        pub type_: u8,
    }

    impl AckServerType {
        pub const OPCODE: u32 = 464;
        pub const NAME: &'static str = "rfAckServerType";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                type_: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.type_);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchGetPublicWaitRoomList;

    impl ReqClanMatchGetPublicWaitRoomList {
        pub const OPCODE: u32 = 465;
        pub const NAME: &'static str = "rfReqClanMatch_GetPublicWaitRoomList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchStartPublicWaitRoomList {
        pub result: i32,
        pub err_msg: String,
        pub count: i32,
    }

    impl AckClanMatchStartPublicWaitRoomList {
        pub const OPCODE: u32 = 466;
        pub const NAME: &'static str = "rfAckClanMatch_StartPublicWaitRoomList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchPublicWaitRoomList {
        pub wait_room_idx: Vec<i32>,
        pub leader_name: Vec<String>,
        pub room_name: Vec<String>,
        pub map_num: Vec<i32>,
        pub cur_user_count: Vec<i32>,
        pub max_user_count: Vec<i32>,
        pub status: Vec<u8>,
    }

    impl AckClanMatchPublicWaitRoomList {
        pub const OPCODE: u32 = 467;
        pub const NAME: &'static str = "rfAckClanMatch_PublicWaitRoomList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                wait_room_idx: r.array(|r| r.i32())?,
                leader_name: r.array(|r| r.string())?,
                room_name: r.array(|r| r.string())?,
                map_num: r.array(|r| r.i32())?,
                cur_user_count: r.array(|r| r.i32())?,
                max_user_count: r.array(|r| r.i32())?,
                status: r.array(|r| r.u8())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.wait_room_idx, |w, v| { w.i32(*v); });
            w.array(&self.leader_name, |w, v| { w.string(v); });
            w.array(&self.room_name, |w, v| { w.string(v); });
            w.array(&self.map_num, |w, v| { w.i32(*v); });
            w.array(&self.cur_user_count, |w, v| { w.i32(*v); });
            w.array(&self.max_user_count, |w, v| { w.i32(*v); });
            w.array(&self.status, |w, v| { w.u8(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchEndPublicWaitRoomList;

    impl AckClanMatchEndPublicWaitRoomList {
        pub const OPCODE: u32 = 468;
        pub const NAME: &'static str = "rfAckClanMatch_EndPublicWaitRoomList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqPaidItemChangeClanname {
        pub item_idx: f64,
        pub new_clanname: String,
    }

    impl ReqPaidItemChangeClanname {
        pub const OPCODE: u32 = 469;
        pub const NAME: &'static str = "rfReqPaidItem_ChangeClanname";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_idx: r.f64()?,
                new_clanname: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.f64(self.item_idx);
            w.string(&self.new_clanname);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPaidItemChangeClanname {
        pub result: i32,
        pub err_msg: String,
        pub item_idx: f64,
        pub remain_count: i32,
        pub new_clanname: String,
    }

    impl AckPaidItemChangeClanname {
        pub const OPCODE: u32 = 470;
        pub const NAME: &'static str = "rfAckPaidItem_ChangeClanname";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                item_idx: r.f64()?,
                remain_count: r.i32()?,
                new_clanname: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.f64(self.item_idx);
            w.i32(self.remain_count);
            w.string(&self.new_clanname);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPaidItemNotifyChangeClanname {
        pub new_clanname: String,
    }

    impl AckPaidItemNotifyChangeClanname {
        pub const OPCODE: u32 = 471;
        pub const NAME: &'static str = "rfAckPaidItem_NotifyChangeClanname";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                new_clanname: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.new_clanname);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqEventNotify;

    impl ReqEventNotify {
        pub const OPCODE: u32 = 472;
        pub const NAME: &'static str = "rfReqEventNotify";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckEventNotify {
        pub type_: i32,
    }

    impl AckEventNotify {
        pub const OPCODE: u32 = 473;
        pub const NAME: &'static str = "rfAckEventNotify";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                type_: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.type_);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchPublicWaitRoomChanged {
        pub wait_room_idx: i32,
        pub room_name: String,
        pub map_num: i32,
        pub cur_user_count: i32,
        pub max_user_count: i32,
        pub status: u8,
        pub is_public: u8,
    }

    impl AckClanMatchPublicWaitRoomChanged {
        pub const OPCODE: u32 = 474;
        pub const NAME: &'static str = "rfAckClanMatch_PublicWaitRoomChanged";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                wait_room_idx: r.i32()?,
                room_name: r.string()?,
                map_num: r.i32()?,
                cur_user_count: r.i32()?,
                max_user_count: r.i32()?,
                status: r.u8()?,
                is_public: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.wait_room_idx);
            w.string(&self.room_name);
            w.i32(self.map_num);
            w.i32(self.cur_user_count);
            w.i32(self.max_user_count);
            w.u8(self.status);
            w.u8(self.is_public);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqDeleteMailAll {
        pub mail_type: u8,
    }

    impl ReqDeleteMailAll {
        pub const OPCODE: u32 = 475;
        pub const NAME: &'static str = "rfReqDeleteMailAll";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                mail_type: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.mail_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckDeleteMailAll {
        pub result: i32,
        pub mail_type: u8,
        pub count: i32,
    }

    impl AckDeleteMailAll {
        pub const OPCODE: u32 = 476;
        pub const NAME: &'static str = "rfAckDeleteMailAll";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                mail_type: r.u8()?,
                count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u8(self.mail_type);
            w.i32(self.count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanMatchResultHistory {
        pub clan_nm: String,
    }

    impl ReqClanMatchResultHistory {
        pub const OPCODE: u32 = 477;
        pub const NAME: &'static str = "rfReqClanMatchResultHistory";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_nm: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_nm);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanMatchResultHistory {
        pub match_date: Vec<f64>,
        pub is_win: Vec<u8>,
        pub enemy_clanname: Vec<String>,
    }

    impl AckClanMatchResultHistory {
        pub const OPCODE: u32 = 478;
        pub const NAME: &'static str = "rfAckClanMatchResultHistory";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                match_date: r.array(|r| r.f64())?,
                is_win: r.array(|r| r.u8())?,
                enemy_clanname: r.array(|r| r.string())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.match_date, |w, v| { w.f64(*v); });
            w.array(&self.is_win, |w, v| { w.u8(*v); });
            w.array(&self.enemy_clanname, |w, v| { w.string(v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqInGameRoomUserInfoChanged;

    impl ReqInGameRoomUserInfoChanged {
        pub const OPCODE: u32 = 479;
        pub const NAME: &'static str = "rfReqInGameRoomUserInfoChanged";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqInGameRoomUserClanLevelChanged {
        pub user_id: i32,
        pub clan_level: i32,
    }

    impl ReqInGameRoomUserClanLevelChanged {
        pub const OPCODE: u32 = 480;
        pub const NAME: &'static str = "rfReqInGameRoomUserClanLevelChanged";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
                clan_level: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            w.i32(self.clan_level);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqCharDetailInfo;

    impl ReqCharDetailInfo {
        pub const OPCODE: u32 = 481;
        pub const NAME: &'static str = "rfReqCharDetailInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckCharDetailInfo {
        pub max_serial: i32,
        pub max_score: i32,
        pub max_kill: i32,
        pub max_assist: i32,
        pub serial3: i32,
        pub serial5: i32,
        pub serial7: i32,
        pub serial10: i32,
        pub serial15: i32,
        pub ar: i32,
        pub smg: i32,
        pub sr: i32,
        pub sw: i32,
        pub melee: i32,
        pub grenade: i32,
        pub flash_bang: i32,
        pub foot: i32,
        pub at4: i32,
        pub mg: i32,
        pub fire: i32,
        pub rpg7: i32,
        pub time_bomb: i32,
        pub icendiary_bomb: i32,
        pub cluster_bomb: i32,
        pub by_chopper: i32,
        pub artillery: i32,
        pub kill_chopper: i32,
        pub first_kill: i32,
        pub last_kill: i32,
        pub all_kill: i32,
        pub dominate: i32,
        pub install_bomb: i32,
        pub defuse_bomb: i32,
    }

    impl AckCharDetailInfo {
        pub const OPCODE: u32 = 482;
        pub const NAME: &'static str = "rfAckCharDetailInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                max_serial: r.i32()?,
                max_score: r.i32()?,
                max_kill: r.i32()?,
                max_assist: r.i32()?,
                serial3: r.i32()?,
                serial5: r.i32()?,
                serial7: r.i32()?,
                serial10: r.i32()?,
                serial15: r.i32()?,
                ar: r.i32()?,
                smg: r.i32()?,
                sr: r.i32()?,
                sw: r.i32()?,
                melee: r.i32()?,
                grenade: r.i32()?,
                flash_bang: r.i32()?,
                foot: r.i32()?,
                at4: r.i32()?,
                mg: r.i32()?,
                fire: r.i32()?,
                rpg7: r.i32()?,
                time_bomb: r.i32()?,
                icendiary_bomb: r.i32()?,
                cluster_bomb: r.i32()?,
                by_chopper: r.i32()?,
                artillery: r.i32()?,
                kill_chopper: r.i32()?,
                first_kill: r.i32()?,
                last_kill: r.i32()?,
                all_kill: r.i32()?,
                dominate: r.i32()?,
                install_bomb: r.i32()?,
                defuse_bomb: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.max_serial);
            w.i32(self.max_score);
            w.i32(self.max_kill);
            w.i32(self.max_assist);
            w.i32(self.serial3);
            w.i32(self.serial5);
            w.i32(self.serial7);
            w.i32(self.serial10);
            w.i32(self.serial15);
            w.i32(self.ar);
            w.i32(self.smg);
            w.i32(self.sr);
            w.i32(self.sw);
            w.i32(self.melee);
            w.i32(self.grenade);
            w.i32(self.flash_bang);
            w.i32(self.foot);
            w.i32(self.at4);
            w.i32(self.mg);
            w.i32(self.fire);
            w.i32(self.rpg7);
            w.i32(self.time_bomb);
            w.i32(self.icendiary_bomb);
            w.i32(self.cluster_bomb);
            w.i32(self.by_chopper);
            w.i32(self.artillery);
            w.i32(self.kill_chopper);
            w.i32(self.first_kill);
            w.i32(self.last_kill);
            w.i32(self.all_kill);
            w.i32(self.dominate);
            w.i32(self.install_bomb);
            w.i32(self.defuse_bomb);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGetRankingInfoList {
        pub ranking_type: u8,
        pub community_type: u8,
    }

    impl ReqGetRankingInfoList {
        pub const OPCODE: u32 = 483;
        pub const NAME: &'static str = "rfReqGetRankingInfoList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                ranking_type: r.u8()?,
                community_type: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.ranking_type);
            w.u8(self.community_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGetRankingInfoListStart {
        pub result: i32,
        pub ranking_type: u8,
        pub community_type: u8,
        pub total_count: i32,
    }

    impl AckGetRankingInfoListStart {
        pub const OPCODE: u32 = 484;
        pub const NAME: &'static str = "rfAckGetRankingInfoListStart";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                ranking_type: r.u8()?,
                community_type: r.u8()?,
                total_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u8(self.ranking_type);
            w.u8(self.community_type);
            w.i32(self.total_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGetRankingInfoList {
        pub result: i32,
        pub ranking_type: u8,
        pub community_type: u8,
        pub char_name: Vec<String>,
        pub char_level: Vec<i32>,
        pub clan_name: Vec<String>,
        pub clan_mark_pattern: Vec<i32>,
        pub clan_mark_bg: Vec<i32>,
        pub clan_mark_bl: Vec<i32>,
        pub ranking_accum: Vec<i32>,
        pub ranking_accum_prev: Vec<i32>,
        pub ranking_accum_exp: Vec<i32>,
        pub level_mark_id: Vec<i32>,
        pub title_mark_id: Vec<i32>,
        pub reserved1: Vec<i32>,
        pub reserved2: Vec<i32>,
        pub reserved3: Vec<i32>,
    }

    impl AckGetRankingInfoList {
        pub const OPCODE: u32 = 485;
        pub const NAME: &'static str = "rfAckGetRankingInfoList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                ranking_type: r.u8()?,
                community_type: r.u8()?,
                char_name: r.array(|r| r.string())?,
                char_level: r.array(|r| r.i32())?,
                clan_name: r.array(|r| r.string())?,
                clan_mark_pattern: r.array(|r| r.i32())?,
                clan_mark_bg: r.array(|r| r.i32())?,
                clan_mark_bl: r.array(|r| r.i32())?,
                ranking_accum: r.array(|r| r.i32())?,
                ranking_accum_prev: r.array(|r| r.i32())?,
                ranking_accum_exp: r.array(|r| r.i32())?,
                level_mark_id: r.array(|r| r.i32())?,
                title_mark_id: r.array(|r| r.i32())?,
                reserved1: r.array(|r| r.i32())?,
                reserved2: r.array(|r| r.i32())?,
                reserved3: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u8(self.ranking_type);
            w.u8(self.community_type);
            w.array(&self.char_name, |w, v| { w.string(v); });
            w.array(&self.char_level, |w, v| { w.i32(*v); });
            w.array(&self.clan_name, |w, v| { w.string(v); });
            w.array(&self.clan_mark_pattern, |w, v| { w.i32(*v); });
            w.array(&self.clan_mark_bg, |w, v| { w.i32(*v); });
            w.array(&self.clan_mark_bl, |w, v| { w.i32(*v); });
            w.array(&self.ranking_accum, |w, v| { w.i32(*v); });
            w.array(&self.ranking_accum_prev, |w, v| { w.i32(*v); });
            w.array(&self.ranking_accum_exp, |w, v| { w.i32(*v); });
            w.array(&self.level_mark_id, |w, v| { w.i32(*v); });
            w.array(&self.title_mark_id, |w, v| { w.i32(*v); });
            w.array(&self.reserved1, |w, v| { w.i32(*v); });
            w.array(&self.reserved2, |w, v| { w.i32(*v); });
            w.array(&self.reserved3, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGetRankingInfoListEnd;

    impl AckGetRankingInfoListEnd {
        pub const OPCODE: u32 = 486;
        pub const NAME: &'static str = "rfAckGetRankingInfoListEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNotifyUpdateCharInfoLevelMark {
        pub char_name: String,
        pub level_mark_id: i32,
    }

    impl AckNotifyUpdateCharInfoLevelMark {
        pub const OPCODE: u32 = 487;
        pub const NAME: &'static str = "rfAckNotifyUpdateCharInfo_LevelMark";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
                level_mark_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            w.i32(self.level_mark_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNotifyUpdateCharInfoTitleMark {
        pub char_name: String,
        pub title_mark_id: i32,
    }

    impl AckNotifyUpdateCharInfoTitleMark {
        pub const OPCODE: u32 = 488;
        pub const NAME: &'static str = "rfAckNotifyUpdateCharInfo_TitleMark";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
                title_mark_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            w.i32(self.title_mark_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqCommunityOptionList;

    impl ReqCommunityOptionList {
        pub const OPCODE: u32 = 489;
        pub const NAME: &'static str = "rfReqCommunityOptionList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckCommunityOptionList {
        pub opt_wispher: u16,
        pub opt_game_invite: u16,
        pub opt_friend_invite: u16,
        pub opt_clan_invite: u16,
    }

    impl AckCommunityOptionList {
        pub const OPCODE: u32 = 490;
        pub const NAME: &'static str = "rfAckCommunityOptionList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                opt_wispher: r.u16()?,
                opt_game_invite: r.u16()?,
                opt_friend_invite: r.u16()?,
                opt_clan_invite: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.opt_wispher);
            w.u16(self.opt_game_invite);
            w.u16(self.opt_friend_invite);
            w.u16(self.opt_clan_invite);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSetCommunityOption {
        pub opt_wispher: u16,
        pub opt_game_invite: u16,
        pub opt_friend_invite: u16,
        pub opt_clan_invite: u16,
    }

    impl ReqSetCommunityOption {
        pub const OPCODE: u32 = 491;
        pub const NAME: &'static str = "rfReqSetCommunityOption";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                opt_wispher: r.u16()?,
                opt_game_invite: r.u16()?,
                opt_friend_invite: r.u16()?,
                opt_clan_invite: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.opt_wispher);
            w.u16(self.opt_game_invite);
            w.u16(self.opt_friend_invite);
            w.u16(self.opt_clan_invite);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckSetCommunityOption {
        pub result: i32,
        pub err_str: String,
    }

    impl AckSetCommunityOption {
        pub const OPCODE: u32 = 492;
        pub const NAME: &'static str = "rfAckSetCommunityOption";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_str: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_str);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckTencentAASInfo {
        pub user_id: i32,
        pub is_adult: u8,
        pub is_qq_auth: u8,
        pub accum_time_sec: i32,
    }

    impl AckTencentAASInfo {
        pub const OPCODE: u32 = 493;
        pub const NAME: &'static str = "rfAckTencent_AASInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
                is_adult: r.u8()?,
                is_qq_auth: r.u8()?,
                accum_time_sec: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            w.u8(self.is_adult);
            w.u8(self.is_qq_auth);
            w.i32(self.accum_time_sec);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLogOut {
        pub reason: u8,
    }

    impl ReqLogOut {
        pub const OPCODE: u32 = 494;
        pub const NAME: &'static str = "rfReqLogOut";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                reason: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.reason);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLogOut {
        pub result: u8,
        pub reason: u8,
    }

    impl AckLogOut {
        pub const OPCODE: u32 = 495;
        pub const NAME: &'static str = "rfAckLogOut";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.u8()?,
                reason: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.result);
            w.u8(self.reason);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqDedicateHostLogin {
        pub process_id: i32,
        pub channel_num: i32,
        pub room_num: i32,
    }

    impl ReqDedicateHostLogin {
        pub const OPCODE: u32 = 496;
        pub const NAME: &'static str = "rfReqDedicateHost_Login";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                process_id: r.i32()?,
                channel_num: r.i32()?,
                room_num: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.process_id);
            w.i32(self.channel_num);
            w.i32(self.room_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckDedicateHostLogin {
        pub result: i32,
        pub user_id: i32,
        pub channel_num: i32,
        pub room_num: i32,
    }

    impl AckDedicateHostLogin {
        pub const OPCODE: u32 = 497;
        pub const NAME: &'static str = "rfAckDedicateHost_Login";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                user_id: r.i32()?,
                channel_num: r.i32()?,
                room_num: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.user_id);
            w.i32(self.channel_num);
            w.i32(self.room_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqFriendPosition {
        pub target_char_name: String,
        pub community_type: u8,
    }

    impl ReqFriendPosition {
        pub const OPCODE: u32 = 498;
        pub const NAME: &'static str = "rfReqFriendPosition";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                target_char_name: r.string()?,
                community_type: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.target_char_name);
            w.u8(self.community_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckFriendPosition {
        pub result: i32,
        pub target_char_name: String,
        pub server_type: u8,
        pub f_servername: String,
        pub f_server_short_name: String,
        pub f_server_ip: String,
        pub f_server_port: i32,
        pub cur_pos: u8,
        pub cur_state: u8,
        pub channel_num: i32,
        pub channel_nick_name: String,
        pub game_room_num: i32,
    }

    impl AckFriendPosition {
        pub const OPCODE: u32 = 499;
        pub const NAME: &'static str = "rfAckFriendPosition";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                target_char_name: r.string()?,
                server_type: r.u8()?,
                f_servername: r.string()?,
                f_server_short_name: r.string()?,
                f_server_ip: r.string()?,
                f_server_port: r.i32()?,
                cur_pos: r.u8()?,
                cur_state: r.u8()?,
                channel_num: r.i32()?,
                channel_nick_name: r.string()?,
                game_room_num: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.target_char_name);
            w.u8(self.server_type);
            w.string(&self.f_servername);
            w.string(&self.f_server_short_name);
            w.string(&self.f_server_ip);
            w.i32(self.f_server_port);
            w.u8(self.cur_pos);
            w.u8(self.cur_state);
            w.i32(self.channel_num);
            w.string(&self.channel_nick_name);
            w.i32(self.game_room_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSetClientVersion {
        pub client_version: i32,
    }

    impl ReqSetClientVersion {
        pub const OPCODE: u32 = 500;
        pub const NAME: &'static str = "rfReqSetClientVersion";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                client_version: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.client_version);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct UpdateWebzenShopScriptVersion {
        pub year: i32,
        pub year_id: i32,
        pub sales_zone_code: i32,
    }

    impl UpdateWebzenShopScriptVersion {
        pub const OPCODE: u32 = 501;
        pub const NAME: &'static str = "rfUpdateWebzenShopScriptVersion";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                year: r.i32()?,
                year_id: r.i32()?,
                sales_zone_code: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.year);
            w.i32(self.year_id);
            w.i32(self.sales_zone_code);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPopUpMessage {
        pub message_type: i32,
    }

    impl AckPopUpMessage {
        pub const OPCODE: u32 = 502;
        pub const NAME: &'static str = "rfAckPopUpMessage";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                message_type: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.message_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckMoveToLobby {
        pub message_type: u8,
    }

    impl AckMoveToLobby {
        pub const OPCODE: u32 = 503;
        pub const NAME: &'static str = "rfAckMoveToLobby";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                message_type: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.message_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckTencentStartTenProtect {
        pub char_dbid: i32,
    }

    impl AckTencentStartTenProtect {
        pub const OPCODE: u32 = 504;
        pub const NAME: &'static str = "rfAckTencent_StartTenProtect";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_dbid: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.char_dbid);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGameMapData;

    impl ReqGameMapData {
        pub const OPCODE: u32 = 505;
        pub const NAME: &'static str = "rfReqGameMapData";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGameMapData {
        pub n_idx: Vec<i32>,
        pub n_map_id: Vec<i32>,
        pub n_map_nm: Vec<String>,
        pub f_sup_health: Vec<f32>,
        pub f_sup_heli: Vec<f32>,
        pub f_sup_bomb: Vec<f32>,
        pub f_sup_uav: Vec<f32>,
        pub n_mod_ffa: Vec<i32>,
        pub n_mod_ffa_max: Vec<i32>,
        pub n_mod_tdm: Vec<i32>,
        pub n_mod_tdm_max: Vec<i32>,
        pub n_mod_sd: Vec<i32>,
        pub n_mod_sd_max: Vec<i32>,
        pub n_mod_dom: Vec<i32>,
        pub n_mod_dom_max: Vec<i32>,
        pub n_mod_sab: Vec<i32>,
        pub n_mod_sab_max: Vec<i32>,
    }

    impl AckGameMapData {
        pub const OPCODE: u32 = 506;
        pub const NAME: &'static str = "rfAckGameMapData";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                n_idx: r.array(|r| r.i32())?,
                n_map_id: r.array(|r| r.i32())?,
                n_map_nm: r.array(|r| r.string())?,
                f_sup_health: r.array(|r| r.f32())?,
                f_sup_heli: r.array(|r| r.f32())?,
                f_sup_bomb: r.array(|r| r.f32())?,
                f_sup_uav: r.array(|r| r.f32())?,
                n_mod_ffa: r.array(|r| r.i32())?,
                n_mod_ffa_max: r.array(|r| r.i32())?,
                n_mod_tdm: r.array(|r| r.i32())?,
                n_mod_tdm_max: r.array(|r| r.i32())?,
                n_mod_sd: r.array(|r| r.i32())?,
                n_mod_sd_max: r.array(|r| r.i32())?,
                n_mod_dom: r.array(|r| r.i32())?,
                n_mod_dom_max: r.array(|r| r.i32())?,
                n_mod_sab: r.array(|r| r.i32())?,
                n_mod_sab_max: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.n_idx, |w, v| { w.i32(*v); });
            w.array(&self.n_map_id, |w, v| { w.i32(*v); });
            w.array(&self.n_map_nm, |w, v| { w.string(v); });
            w.array(&self.f_sup_health, |w, v| { w.f32(*v); });
            w.array(&self.f_sup_heli, |w, v| { w.f32(*v); });
            w.array(&self.f_sup_bomb, |w, v| { w.f32(*v); });
            w.array(&self.f_sup_uav, |w, v| { w.f32(*v); });
            w.array(&self.n_mod_ffa, |w, v| { w.i32(*v); });
            w.array(&self.n_mod_ffa_max, |w, v| { w.i32(*v); });
            w.array(&self.n_mod_tdm, |w, v| { w.i32(*v); });
            w.array(&self.n_mod_tdm_max, |w, v| { w.i32(*v); });
            w.array(&self.n_mod_sd, |w, v| { w.i32(*v); });
            w.array(&self.n_mod_sd_max, |w, v| { w.i32(*v); });
            w.array(&self.n_mod_dom, |w, v| { w.i32(*v); });
            w.array(&self.n_mod_dom_max, |w, v| { w.i32(*v); });
            w.array(&self.n_mod_sab, |w, v| { w.i32(*v); });
            w.array(&self.n_mod_sab_max, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct NotifyNewNickName {
        pub community_type: i32,
        pub old_nick_name: String,
        pub new_nick_name: String,
    }

    impl NotifyNewNickName {
        pub const OPCODE: u32 = 507;
        pub const NAME: &'static str = "rfNotifyNewNickName";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                community_type: r.i32()?,
                old_nick_name: r.string()?,
                new_nick_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.community_type);
            w.string(&self.old_nick_name);
            w.string(&self.new_nick_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct NotifyNewClanName {
        pub new_nick_name: String,
    }

    impl NotifyNewClanName {
        pub const OPCODE: u32 = 508;
        pub const NAME: &'static str = "rfNotifyNewClanName";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                new_nick_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.new_nick_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct NotifyClanLevelup {
        pub clan_name: String,
        pub levelup: i32,
    }

    impl NotifyClanLevelup {
        pub const OPCODE: u32 = 509;
        pub const NAME: &'static str = "rfNotifyClanLevelup";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
                levelup: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            w.i32(self.levelup);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqAhnHsObject {
        pub data: Vec<u8>,
        pub data_size: i32,
    }

    impl ReqAhnHsObject {
        pub const OPCODE: u32 = 510;
        pub const NAME: &'static str = "rfReqAhnHsObject";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                data: r.array(|r| r.u8())?,
                data_size: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.data, |w, v| { w.u8(*v); });
            w.i32(self.data_size);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAhnHsObject {
        pub data: Vec<u8>,
        pub data_size: i32,
    }

    impl AckAhnHsObject {
        pub const OPCODE: u32 = 511;
        pub const NAME: &'static str = "rfAckAhnHsObject";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                data: r.array(|r| r.u8())?,
                data_size: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.data, |w, v| { w.u8(*v); });
            w.i32(self.data_size);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqAhnNotifyDisconnct {
        pub code: i32,
    }

    impl ReqAhnNotifyDisconnct {
        pub const OPCODE: u32 = 512;
        pub const NAME: &'static str = "rfReqAhnNotifyDisconnct";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                code: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.code);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqUpdateUserPerformenceInfo {
        pub user_id: i32,
        pub min_fps: i32,
        pub max_fps: i32,
        pub avg_fps: i32,
        pub min_ping: i32,
        pub max_ping: i32,
        pub avg_ping: i32,
    }

    impl ReqUpdateUserPerformenceInfo {
        pub const OPCODE: u32 = 513;
        pub const NAME: &'static str = "rfReqUpdateUserPerformenceInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
                min_fps: r.i32()?,
                max_fps: r.i32()?,
                avg_fps: r.i32()?,
                min_ping: r.i32()?,
                max_ping: r.i32()?,
                avg_ping: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            w.i32(self.min_fps);
            w.i32(self.max_fps);
            w.i32(self.avg_fps);
            w.i32(self.min_ping);
            w.i32(self.max_ping);
            w.i32(self.avg_ping);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyLoadingStatus {
        pub status: i32,
    }

    impl ReqNotifyLoadingStatus {
        pub const OPCODE: u32 = 514;
        pub const NAME: &'static str = "rfReqNotifyLoadingStatus";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                status: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.status);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqShopItemData {
        pub ver: i32,
    }

    impl ReqShopItemData {
        pub const OPCODE: u32 = 515;
        pub const NAME: &'static str = "rfReqShopItemData";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                ver: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.ver);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckShopItemData;

    impl AckShopItemData {
        pub const OPCODE: u32 = 516;
        pub const NAME: &'static str = "rfAckShopItemData";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyShopItemData {
        pub product_no: Vec<i32>,
        pub product_name: Vec<String>,
        pub product_code: Vec<i32>,
        pub sale_type: Vec<u8>,
        pub product_cost: Vec<i32>,
        pub product_hour: Vec<i32>,
        pub product_quantity: Vec<i32>,
        pub sale_master_flag: Vec<i32>,
        pub sale_status: Vec<i32>,
        pub is_gift: Vec<i32>,
        pub sale_end_time: Vec<f64>,
        pub discountcost: Vec<i32>,
    }

    impl ReqNotifyShopItemData {
        pub const OPCODE: u32 = 517;
        pub const NAME: &'static str = "rfReqNotifyShopItemData";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                product_no: r.array(|r| r.i32())?,
                product_name: r.array(|r| r.string())?,
                product_code: r.array(|r| r.i32())?,
                sale_type: r.array(|r| r.u8())?,
                product_cost: r.array(|r| r.i32())?,
                product_hour: r.array(|r| r.i32())?,
                product_quantity: r.array(|r| r.i32())?,
                sale_master_flag: r.array(|r| r.i32())?,
                sale_status: r.array(|r| r.i32())?,
                is_gift: r.array(|r| r.i32())?,
                sale_end_time: r.array(|r| r.f64())?,
                discountcost: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.product_no, |w, v| { w.i32(*v); });
            w.array(&self.product_name, |w, v| { w.string(v); });
            w.array(&self.product_code, |w, v| { w.i32(*v); });
            w.array(&self.sale_type, |w, v| { w.u8(*v); });
            w.array(&self.product_cost, |w, v| { w.i32(*v); });
            w.array(&self.product_hour, |w, v| { w.i32(*v); });
            w.array(&self.product_quantity, |w, v| { w.i32(*v); });
            w.array(&self.sale_master_flag, |w, v| { w.i32(*v); });
            w.array(&self.sale_status, |w, v| { w.i32(*v); });
            w.array(&self.is_gift, |w, v| { w.i32(*v); });
            w.array(&self.sale_end_time, |w, v| { w.f64(*v); });
            w.array(&self.discountcost, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyShopItemDataEnd {
        pub ver: i32,
    }

    impl ReqNotifyShopItemDataEnd {
        pub const OPCODE: u32 = 518;
        pub const NAME: &'static str = "rfReqNotifyShopItemDataEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                ver: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.ver);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqCouponUse {
        pub serial: String,
    }

    impl ReqCouponUse {
        pub const OPCODE: u32 = 519;
        pub const NAME: &'static str = "rfReqCouponUse";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                serial: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.serial);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckCouponUse {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckCouponUse {
        pub const OPCODE: u32 = 520;
        pub const NAME: &'static str = "rfAckCouponUse";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct NotifyCouponUseError {
        pub time: i32,
    }

    impl NotifyCouponUseError {
        pub const OPCODE: u32 = 521;
        pub const NAME: &'static str = "rfNotifyCouponUseError";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                time: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.time);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAIRewardResult {
        pub clear_reward_type: u8,
        pub clear_reward_id: i32,
        pub clear_reward_count: i32,
        pub score_reward_user_id: Vec<i32>,
        pub score_reward_type: Vec<u8>,
        pub score_reward_id: Vec<i32>,
        pub score_reward_count: Vec<i32>,
    }

    impl AckAIRewardResult {
        pub const OPCODE: u32 = 522;
        pub const NAME: &'static str = "rfAckAIRewardResult";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clear_reward_type: r.u8()?,
                clear_reward_id: r.i32()?,
                clear_reward_count: r.i32()?,
                score_reward_user_id: r.array(|r| r.i32())?,
                score_reward_type: r.array(|r| r.u8())?,
                score_reward_id: r.array(|r| r.i32())?,
                score_reward_count: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.clear_reward_type);
            w.i32(self.clear_reward_id);
            w.i32(self.clear_reward_count);
            w.array(&self.score_reward_user_id, |w, v| { w.i32(*v); });
            w.array(&self.score_reward_type, |w, v| { w.u8(*v); });
            w.array(&self.score_reward_id, |w, v| { w.i32(*v); });
            w.array(&self.score_reward_count, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqAttendanceList {
        pub req_type: u8,
        pub update_num: u8,
        pub update_state: u8,
        pub paid_item_idx: f64,
    }

    impl ReqAttendanceList {
        pub const OPCODE: u32 = 523;
        pub const NAME: &'static str = "rfReqAttendanceList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                req_type: r.u8()?,
                update_num: r.u8()?,
                update_state: r.u8()?,
                paid_item_idx: r.f64()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.req_type);
            w.u8(self.update_num);
            w.u8(self.update_state);
            w.f64(self.paid_item_idx);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAttendanceList {
        pub result: i32,
        pub is_complete: u8,
        pub updated_num: u8,
        pub state_list: Vec<u8>,
        pub item_idx: f64,
        pub reward_item_id: i32,
        pub reward_item_count: i32,
        pub paid_item_idx: f64,
        pub paid_item_stack_count: i32,
    }

    impl AckAttendanceList {
        pub const OPCODE: u32 = 524;
        pub const NAME: &'static str = "rfAckAttendanceList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                is_complete: r.u8()?,
                updated_num: r.u8()?,
                state_list: r.array(|r| r.u8())?,
                item_idx: r.f64()?,
                reward_item_id: r.i32()?,
                reward_item_count: r.i32()?,
                paid_item_idx: r.f64()?,
                paid_item_stack_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u8(self.is_complete);
            w.u8(self.updated_num);
            w.array(&self.state_list, |w, v| { w.u8(*v); });
            w.f64(self.item_idx);
            w.i32(self.reward_item_id);
            w.i32(self.reward_item_count);
            w.f64(self.paid_item_idx);
            w.i32(self.paid_item_stack_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqPuzzleList {
        pub req_type: u8,
        pub update_state: u8,
        pub paid_item_idx: f64,
    }

    impl ReqPuzzleList {
        pub const OPCODE: u32 = 525;
        pub const NAME: &'static str = "rfReqPuzzleList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                req_type: r.u8()?,
                update_state: r.u8()?,
                paid_item_idx: r.f64()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.req_type);
            w.u8(self.update_state);
            w.f64(self.paid_item_idx);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckPuzzleList {
        pub result: i32,
        pub is_complete: u8,
        pub updated_num: Vec<u8>,
        pub state_list: Vec<u8>,
        pub item_idx: f64,
        pub reward_item_id: i32,
        pub reward_item_count: i32,
        pub paid_item_idx: f64,
        pub paid_item_stack_count: i32,
    }

    impl AckPuzzleList {
        pub const OPCODE: u32 = 526;
        pub const NAME: &'static str = "rfAckPuzzleList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                is_complete: r.u8()?,
                updated_num: r.array(|r| r.u8())?,
                state_list: r.array(|r| r.u8())?,
                item_idx: r.f64()?,
                reward_item_id: r.i32()?,
                reward_item_count: r.i32()?,
                paid_item_idx: r.f64()?,
                paid_item_stack_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.u8(self.is_complete);
            w.array(&self.updated_num, |w, v| { w.u8(*v); });
            w.array(&self.state_list, |w, v| { w.u8(*v); });
            w.f64(self.item_idx);
            w.i32(self.reward_item_id);
            w.i32(self.reward_item_count);
            w.f64(self.paid_item_idx);
            w.i32(self.paid_item_stack_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqDailyRewardListThisMonth;

    impl ReqDailyRewardListThisMonth {
        pub const OPCODE: u32 = 527;
        pub const NAME: &'static str = "rfReqDailyRewardListThisMonth";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckDailyRewardListThisMonth {
        pub type_: Vec<u8>,
        pub number: Vec<u8>,
        pub item_id: Vec<i32>,
        pub item_count: Vec<i32>,
    }

    impl AckDailyRewardListThisMonth {
        pub const OPCODE: u32 = 528;
        pub const NAME: &'static str = "rfAckDailyRewardListThisMonth";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                type_: r.array(|r| r.u8())?,
                number: r.array(|r| r.u8())?,
                item_id: r.array(|r| r.i32())?,
                item_count: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.type_, |w, v| { w.u8(*v); });
            w.array(&self.number, |w, v| { w.u8(*v); });
            w.array(&self.item_id, |w, v| { w.i32(*v); });
            w.array(&self.item_count, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqReceiveSpecialReward {
        pub quest_id: i32,
    }

    impl ReqReceiveSpecialReward {
        pub const OPCODE: u32 = 529;
        pub const NAME: &'static str = "rfReqReceiveSpecialReward";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                quest_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.quest_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckReceiveSpecialReward {
        pub result: i32,
        pub rewarded_quest_id: i32,
        pub rewarded_quest_is_complete: u8,
        pub item_idx: f64,
        pub reward_item_id: i32,
        pub reward_item_count: i32,
    }

    impl AckReceiveSpecialReward {
        pub const OPCODE: u32 = 530;
        pub const NAME: &'static str = "rfAckReceiveSpecialReward";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                rewarded_quest_id: r.i32()?,
                rewarded_quest_is_complete: r.u8()?,
                item_idx: r.f64()?,
                reward_item_id: r.i32()?,
                reward_item_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.rewarded_quest_id);
            w.u8(self.rewarded_quest_is_complete);
            w.f64(self.item_idx);
            w.i32(self.reward_item_id);
            w.i32(self.reward_item_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSpecialQuestRank;

    impl ReqSpecialQuestRank {
        pub const OPCODE: u32 = 531;
        pub const NAME: &'static str = "rfReqSpecialQuestRank";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckSpecialQuestRank {
        pub char_list: Vec<String>,
    }

    impl AckSpecialQuestRank {
        pub const OPCODE: u32 = 532;
        pub const NAME: &'static str = "rfAckSpecialQuestRank";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_list: r.array(|r| r.string())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.char_list, |w, v| { w.string(v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLuckShopLotteryWinnerList {
        pub capsule_item_id: i32,
    }

    impl ReqLuckShopLotteryWinnerList {
        pub const OPCODE: u32 = 533;
        pub const NAME: &'static str = "rfReqLuckShop_LotteryWinnerList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                capsule_item_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.capsule_item_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLuckShopLotteryWinnerList {
        pub capsule_item_id: i32,
        pub is_notification: u8,
        pub nick_name: Vec<String>,
        pub time: Vec<f64>,
        pub get_item_id: i32,
    }

    impl AckLuckShopLotteryWinnerList {
        pub const OPCODE: u32 = 534;
        pub const NAME: &'static str = "rfAckLuckShop_LotteryWinnerList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                capsule_item_id: r.i32()?,
                is_notification: r.u8()?,
                nick_name: r.array(|r| r.string())?,
                time: r.array(|r| r.f64())?,
                get_item_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.capsule_item_id);
            w.u8(self.is_notification);
            w.array(&self.nick_name, |w, v| { w.string(v); });
            w.array(&self.time, |w, v| { w.f64(*v); });
            w.i32(self.get_item_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLuckShopStorageInfo;

    impl ReqLuckShopStorageInfo {
        pub const OPCODE: u32 = 535;
        pub const NAME: &'static str = "rfReqLuckShop_StorageInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLuckShopStorageInfo {
        pub error_code: i32,
        pub buying_capsule_count: u16,
        pub capsule_reward_state: i32,
    }

    impl AckLuckShopStorageInfo {
        pub const OPCODE: u32 = 536;
        pub const NAME: &'static str = "rfAckLuckShop_StorageInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                error_code: r.i32()?,
                buying_capsule_count: r.u16()?,
                capsule_reward_state: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.error_code);
            w.u16(self.buying_capsule_count);
            w.i32(self.capsule_reward_state);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLuckShopRewardBuyingCount {
        pub choose_num: u16,
        pub item_id: i32,
        pub item_id_choose: i32,
        pub part_id_choose: i32,
    }

    impl ReqLuckShopRewardBuyingCount {
        pub const OPCODE: u32 = 537;
        pub const NAME: &'static str = "rfReqLuckShop_RewardBuyingCount";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                choose_num: r.u16()?,
                item_id: r.i32()?,
                item_id_choose: r.i32()?,
                part_id_choose: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.choose_num);
            w.i32(self.item_id);
            w.i32(self.item_id_choose);
            w.i32(self.part_id_choose);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLuckShopRewardBuyingCount {
        pub error_code: i32,
        pub final_capsule_reward_state: i32,
    }

    impl AckLuckShopRewardBuyingCount {
        pub const OPCODE: u32 = 538;
        pub const NAME: &'static str = "rfAckLuckShop_RewardBuyingCount";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                error_code: r.i32()?,
                final_capsule_reward_state: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.error_code);
            w.i32(self.final_capsule_reward_state);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLuckShopRewardCoupon {
        pub choose_num: u16,
        pub item_id: i32,
        pub item_id_choose: i32,
        pub part_id_choose: i32,
    }

    impl ReqLuckShopRewardCoupon {
        pub const OPCODE: u32 = 539;
        pub const NAME: &'static str = "rfReqLuckShop_RewardCoupon";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                choose_num: r.u16()?,
                item_id: r.i32()?,
                item_id_choose: r.i32()?,
                part_id_choose: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.choose_num);
            w.i32(self.item_id);
            w.i32(self.item_id_choose);
            w.i32(self.part_id_choose);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLuckShopRewardCoupon {
        pub error_code: i32,
        pub coupon_item_type_id: i32,
        pub coupon_item_unique_id: f64,
        pub coupon_count_after_reduction: i32,
    }

    impl AckLuckShopRewardCoupon {
        pub const OPCODE: u32 = 540;
        pub const NAME: &'static str = "rfAckLuckShop_RewardCoupon";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                error_code: r.i32()?,
                coupon_item_type_id: r.i32()?,
                coupon_item_unique_id: r.f64()?,
                coupon_count_after_reduction: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.error_code);
            w.i32(self.coupon_item_type_id);
            w.f64(self.coupon_item_unique_id);
            w.i32(self.coupon_count_after_reduction);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ChangeItemInstanceInfo {
        pub error_code: i32,
        pub item_id: i32,
        pub item_unique_id: f64,
        pub change_count: u8,
        pub remain_count: i32,
    }

    impl ChangeItemInstanceInfo {
        pub const OPCODE: u32 = 541;
        pub const NAME: &'static str = "rfChangeItemInstanceInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                error_code: r.i32()?,
                item_id: r.i32()?,
                item_unique_id: r.f64()?,
                change_count: r.u8()?,
                remain_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.error_code);
            w.i32(self.item_id);
            w.f64(self.item_unique_id);
            w.u8(self.change_count);
            w.i32(self.remain_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqUseItemDuringGame {
        pub char_name: String,
        pub item_id: i32,
        pub item_unique_id: f64,
        pub using_count: u8,
    }

    impl ReqUseItemDuringGame {
        pub const OPCODE: u32 = 542;
        pub const NAME: &'static str = "rfReqUseItemDuringGame";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
                item_id: r.i32()?,
                item_unique_id: r.f64()?,
                using_count: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            w.i32(self.item_id);
            w.f64(self.item_unique_id);
            w.u8(self.using_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckUseItemDuringGame {
        pub error_code: i32,
        pub char_name: String,
        pub item_id: i32,
        pub item_unique_id: f64,
        pub remain_count: i32,
    }

    impl AckUseItemDuringGame {
        pub const OPCODE: u32 = 543;
        pub const NAME: &'static str = "rfAckUseItemDuringGame";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                error_code: r.i32()?,
                char_name: r.string()?,
                item_id: r.i32()?,
                item_unique_id: r.f64()?,
                remain_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.error_code);
            w.string(&self.char_name);
            w.i32(self.item_id);
            w.f64(self.item_unique_id);
            w.i32(self.remain_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckDBCharInfo {
        pub result: i32,
        pub err_msg: String,
        pub char_name: String,
        pub clan_name: String,
        pub cm_pattern: i32,
        pub cm_bg: i32,
        pub cm_bl: i32,
        pub clan_person_intro: String,
        pub main_weapon_item_id: i32,
        pub main_weapon_part_id: i32,
        pub sub_weapon_item_id: i32,
        pub sub_weapon_part_id: i32,
        pub af_special_costume_item_id: i32,
        pub rsa_special_costume_item_id: i32,
        pub st_char_info: CharInfo,
        pub clan_level: i32,
    }

    impl AckDBCharInfo {
        pub const OPCODE: u32 = 268;
        pub const NAME: &'static str = "rfAckDBCharInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                char_name: r.string()?,
                clan_name: r.string()?,
                cm_pattern: r.i32()?,
                cm_bg: r.i32()?,
                cm_bl: r.i32()?,
                clan_person_intro: r.string()?,
                main_weapon_item_id: r.i32()?,
                main_weapon_part_id: r.i32()?,
                sub_weapon_item_id: r.i32()?,
                sub_weapon_part_id: r.i32()?,
                af_special_costume_item_id: r.i32()?,
                rsa_special_costume_item_id: r.i32()?,
                st_char_info: CharInfo::decode(r)?,
                clan_level: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.string(&self.char_name);
            w.string(&self.clan_name);
            w.i32(self.cm_pattern);
            w.i32(self.cm_bg);
            w.i32(self.cm_bl);
            w.string(&self.clan_person_intro);
            w.i32(self.main_weapon_item_id);
            w.i32(self.main_weapon_part_id);
            w.i32(self.sub_weapon_item_id);
            w.i32(self.sub_weapon_part_id);
            w.i32(self.af_special_costume_item_id);
            w.i32(self.rsa_special_costume_item_id);
            self.st_char_info.encode(w);
            w.i32(self.clan_level);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqDBCharInfo {
        pub char_name: String,
    }

    impl ReqDBCharInfo {
        pub const OPCODE: u32 = 267;
        pub const NAME: &'static str = "rfReqDBCharInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqForceNatTypeSetting {
        pub nat_type: i32,
    }

    impl ReqForceNatTypeSetting {
        pub const OPCODE: u32 = 266;
        pub const NAME: &'static str = "rfReqForceNatTypeSetting";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                nat_type: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.nat_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGetServerTime {
        pub server_time: f64,
        pub client_ip: String,
    }

    impl AckGetServerTime {
        pub const OPCODE: u32 = 265;
        pub const NAME: &'static str = "rfAckGetServerTime";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                server_time: r.f64()?,
                client_ip: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.f64(self.server_time);
            w.string(&self.client_ip);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGetServerTime;

    impl ReqGetServerTime {
        pub const OPCODE: u32 = 264;
        pub const NAME: &'static str = "rfReqGetServerTime";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLogEndEachRound {
        pub start_cnt: i32,
        pub end_cnt: i32,
        pub round_end_type: i32,
        pub round_end_time: i32,
        pub v_team: u8,
        pub v_team_ad: u8,
        pub af_score: i32,
        pub rsa_score: i32,
        pub round: i32,
    }

    impl ReqLogEndEachRound {
        pub const OPCODE: u32 = 263;
        pub const NAME: &'static str = "rfReqLogEndEachRound";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                start_cnt: r.i32()?,
                end_cnt: r.i32()?,
                round_end_type: r.i32()?,
                round_end_time: r.i32()?,
                v_team: r.u8()?,
                v_team_ad: r.u8()?,
                af_score: r.i32()?,
                rsa_score: r.i32()?,
                round: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.start_cnt);
            w.i32(self.end_cnt);
            w.i32(self.round_end_type);
            w.i32(self.round_end_time);
            w.u8(self.v_team);
            w.u8(self.v_team_ad);
            w.i32(self.af_score);
            w.i32(self.rsa_score);
            w.i32(self.round);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLogGetScore {
        pub kill_death_log_param: KillDeathLog,
        pub assist_ui_ds: Vec<i32>,
        pub assist_scores: Vec<u8>,
    }

    impl ReqLogGetScore {
        pub const OPCODE: u32 = 262;
        pub const NAME: &'static str = "rfReqLogGetScore";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                kill_death_log_param: KillDeathLog::decode(r)?,
                assist_ui_ds: r.array(|r| r.i32())?,
                assist_scores: r.array(|r| r.u8())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            self.kill_death_log_param.encode(w);
            w.array(&self.assist_ui_ds, |w, v| { w.i32(*v); });
            w.array(&self.assist_scores, |w, v| { w.u8(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAuctionTradeHistory {
        pub result: i32,
        pub err_msg: String,
        pub item_id: i32,
        pub part_id: i32,
        pub seller_char_nm: Vec<String>,
        pub damage_degree: Vec<i32>,
        pub price: Vec<i32>,
    }

    impl AckAuctionTradeHistory {
        pub const OPCODE: u32 = 261;
        pub const NAME: &'static str = "rfAckAuctionTradeHistory";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                item_id: r.i32()?,
                part_id: r.i32()?,
                seller_char_nm: r.array(|r| r.string())?,
                damage_degree: r.array(|r| r.i32())?,
                price: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.item_id);
            w.i32(self.part_id);
            w.array(&self.seller_char_nm, |w, v| { w.string(v); });
            w.array(&self.damage_degree, |w, v| { w.i32(*v); });
            w.array(&self.price, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqAuctionTradeHistory {
        pub item_id: i32,
        pub part_id: i32,
    }

    impl ReqAuctionTradeHistory {
        pub const OPCODE: u32 = 260;
        pub const NAME: &'static str = "rfReqAuctionTradeHistory";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_id: r.i32()?,
                part_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.item_id);
            w.i32(self.part_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckSendGift {
        pub result: i32,
        pub err_msg: String,
        pub point_remain: i32,
        pub cash_remain: i32,
    }

    impl AckSendGift {
        pub const OPCODE: u32 = 259;
        pub const NAME: &'static str = "rfAckSendGift";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                point_remain: r.i32()?,
                cash_remain: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.point_remain);
            w.i32(self.cash_remain);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSendGift {
        pub item_id: i32,
        pub recv_charname: String,
        pub recv_message: String,
        pub buy_option1: u8,
        pub buy_option2: u8,
    }

    impl ReqSendGift {
        pub const OPCODE: u32 = 258;
        pub const NAME: &'static str = "rfReqSendGift";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_id: r.i32()?,
                recv_charname: r.string()?,
                recv_message: r.string()?,
                buy_option1: r.u8()?,
                buy_option2: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.item_id);
            w.string(&self.recv_charname);
            w.string(&self.recv_message);
            w.u8(self.buy_option1);
            w.u8(self.buy_option2);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGetAttachedItemFromMemo {
        pub result: i32,
        pub err_msg: String,
        pub item_idx: f64,
        pub item_id: i32,
    }

    impl AckGetAttachedItemFromMemo {
        pub const OPCODE: u32 = 257;
        pub const NAME: &'static str = "rfAckGetAttachedItemFromMemo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                item_idx: r.f64()?,
                item_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.f64(self.item_idx);
            w.i32(self.item_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGetAttachedItemFromMemo {
        pub user_id: i32,
        pub memo_idx: i32,
        pub item_id: i32,
    }

    impl ReqGetAttachedItemFromMemo {
        pub const OPCODE: u32 = 256;
        pub const NAME: &'static str = "rfReqGetAttachedItemFromMemo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
                memo_idx: r.i32()?,
                item_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            w.i32(self.memo_idx);
            w.i32(self.item_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNotifyNewClanMember {
        pub clan_name: String,
        pub char_name: String,
    }

    impl AckNotifyNewClanMember {
        pub const OPCODE: u32 = 255;
        pub const NAME: &'static str = "rfAckNotifyNewClanMember";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
                char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            w.string(&self.char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChatClan {
        pub char_name: String,
        pub message: String,
        pub result: i32,
    }

    impl AckChatClan {
        pub const OPCODE: u32 = 254;
        pub const NAME: &'static str = "rfAckChatClan";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
                message: r.string()?,
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            w.string(&self.message);
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqChatClan {
        pub message: String,
    }

    impl ReqChatClan {
        pub const OPCODE: u32 = 253;
        pub const NAME: &'static str = "rfReqChatClan";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                message: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.message);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckTestDouble {
        pub v_double: f64,
        pub v_doubles: Vec<f64>,
    }

    impl AckTestDouble {
        pub const OPCODE: u32 = 252;
        pub const NAME: &'static str = "rfAckTestDouble";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                v_double: r.f64()?,
                v_doubles: r.array(|r| r.f64())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.f64(self.v_double);
            w.array(&self.v_doubles, |w, v| { w.f64(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqTestDouble {
        pub v_double: f64,
        pub v_doubles: Vec<f64>,
    }

    impl ReqTestDouble {
        pub const OPCODE: u32 = 251;
        pub const NAME: &'static str = "rfReqTestDouble";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                v_double: r.f64()?,
                v_doubles: r.array(|r| r.f64())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.f64(self.v_double);
            w.array(&self.v_doubles, |w, v| { w.f64(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNotifyLoadingState {
        pub result: i32,
        pub user_id: i32,
        pub loading_state: u8,
    }

    impl AckNotifyLoadingState {
        pub const OPCODE: u32 = 250;
        pub const NAME: &'static str = "rfAckNotifyLoadingState";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                user_id: r.i32()?,
                loading_state: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.user_id);
            w.u8(self.loading_state);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyLoadingState {
        pub loading_state: u8,
    }

    impl ReqNotifyLoadingState {
        pub const OPCODE: u32 = 249;
        pub const NAME: &'static str = "rfReqNotifyLoadingState";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                loading_state: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.loading_state);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckFailedStartGame {
        pub result: i32,
        pub err_msg: String,
        pub is_host: u8,
        pub failed_user_id: i32,
        pub loading_step: i32,
    }

    impl AckFailedStartGame {
        pub const OPCODE: u32 = 248;
        pub const NAME: &'static str = "rfAckFailedStartGame";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                is_host: r.u8()?,
                failed_user_id: r.i32()?,
                loading_step: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.u8(self.is_host);
            w.i32(self.failed_user_id);
            w.i32(self.loading_step);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqFailedStartGame {
        pub loading_step: i32,
    }

    impl ReqFailedStartGame {
        pub const OPCODE: u32 = 247;
        pub const NAME: &'static str = "rfReqFailedStartGame";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                loading_step: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.loading_step);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckSuccessStartGame {
        pub result: i32,
        pub err_msg: String,
        pub is_host: u8,
        pub host_ip: String,
        pub host_port: i32,
        pub loading_step: i32,
    }

    impl AckSuccessStartGame {
        pub const OPCODE: u32 = 246;
        pub const NAME: &'static str = "rfAckSuccessStartGame";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                is_host: r.u8()?,
                host_ip: r.string()?,
                host_port: r.i32()?,
                loading_step: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.u8(self.is_host);
            w.string(&self.host_ip);
            w.i32(self.host_port);
            w.i32(self.loading_step);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSuccessStartGame {
        pub loading_step: i32,
    }

    impl ReqSuccessStartGame {
        pub const OPCODE: u32 = 245;
        pub const NAME: &'static str = "rfReqSuccessStartGame";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                loading_step: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.loading_step);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanCharPersonelIntro {
        pub result: i32,
        pub err_msg: String,
        pub personel_intro: String,
    }

    impl AckClanCharPersonelIntro {
        pub const OPCODE: u32 = 244;
        pub const NAME: &'static str = "rfAckClanCharPersonelIntro";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                personel_intro: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.string(&self.personel_intro);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanCharPersonelIntro {
        pub clan_name: String,
        pub char_name: String,
    }

    impl ReqClanCharPersonelIntro {
        pub const OPCODE: u32 = 243;
        pub const NAME: &'static str = "rfReqClanCharPersonelIntro";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
                char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            w.string(&self.char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanPersonelIntro {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckClanPersonelIntro {
        pub const OPCODE: u32 = 242;
        pub const NAME: &'static str = "rfAckClanPersonelIntro";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanPersonelIntro {
        pub clan_name: String,
        pub char_name: String,
        pub personel_intro: String,
    }

    impl ReqClanPersonelIntro {
        pub const OPCODE: u32 = 241;
        pub const NAME: &'static str = "rfReqClanPersonelIntro";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
                char_name: r.string()?,
                personel_intro: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            w.string(&self.char_name);
            w.string(&self.personel_intro);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanChangeInfo {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckClanChangeInfo {
        pub const OPCODE: u32 = 240;
        pub const NAME: &'static str = "rfAckClanChangeInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanChangeInfo {
        pub clan_name: String,
        pub is_recruit: u8,
        pub region: u8,
        pub keyword: String,
        pub clan_intro: String,
        pub notice: String,
    }

    impl ReqClanChangeInfo {
        pub const OPCODE: u32 = 239;
        pub const NAME: &'static str = "rfReqClanChangeInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
                is_recruit: r.u8()?,
                region: r.u8()?,
                keyword: r.string()?,
                clan_intro: r.string()?,
                notice: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            w.u8(self.is_recruit);
            w.u8(self.region);
            w.string(&self.keyword);
            w.string(&self.clan_intro);
            w.string(&self.notice);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanClose {
        pub result: i32,
        pub err_msg: String,
        pub clan_name: String,
    }

    impl AckClanClose {
        pub const OPCODE: u32 = 238;
        pub const NAME: &'static str = "rfAckClanClose";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                clan_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.string(&self.clan_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanClose {
        pub clan_name: String,
    }

    impl ReqClanClose {
        pub const OPCODE: u32 = 237;
        pub const NAME: &'static str = "rfReqClanClose";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNotifyClanSecession {
        pub clan_name: String,
        pub char_name: String,
    }

    impl AckNotifyClanSecession {
        pub const OPCODE: u32 = 236;
        pub const NAME: &'static str = "rfAckNotifyClanSecession";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
                char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            w.string(&self.char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanSecession {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckClanSecession {
        pub const OPCODE: u32 = 235;
        pub const NAME: &'static str = "rfAckClanSecession";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanSecession {
        pub clan_name: String,
    }

    impl ReqClanSecession {
        pub const OPCODE: u32 = 234;
        pub const NAME: &'static str = "rfReqClanSecession";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanNews {
        pub issue_date: Vec<String>,
        pub issue_type: Vec<u8>,
        pub issued_clan_member: Vec<String>,
    }

    impl AckClanNews {
        pub const OPCODE: u32 = 233;
        pub const NAME: &'static str = "rfAckClanNews";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                issue_date: r.array(|r| r.string())?,
                issue_type: r.array(|r| r.u8())?,
                issued_clan_member: r.array(|r| r.string())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.issue_date, |w, v| { w.string(v); });
            w.array(&self.issue_type, |w, v| { w.u8(*v); });
            w.array(&self.issued_clan_member, |w, v| { w.string(v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanNews {
        pub clan_name: String,
    }

    impl ReqClanNews {
        pub const OPCODE: u32 = 232;
        pub const NAME: &'static str = "rfReqClanNews";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNotifyAdjustClanMemberGrade {
        pub clan_name: String,
        pub char_name: String,
        pub char_grade: u8,
    }

    impl AckNotifyAdjustClanMemberGrade {
        pub const OPCODE: u32 = 231;
        pub const NAME: &'static str = "rfAckNotifyAdjustClanMemberGrade";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
                char_name: r.string()?,
                char_grade: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            w.string(&self.char_name);
            w.u8(self.char_grade);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAdjustClanMemberGrade {
        pub result: i32,
        pub err_msg: String,
        pub clan_name: String,
        pub char_name: String,
        pub char_grade: u8,
    }

    impl AckAdjustClanMemberGrade {
        pub const OPCODE: u32 = 230;
        pub const NAME: &'static str = "rfAckAdjustClanMemberGrade";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                clan_name: r.string()?,
                char_name: r.string()?,
                char_grade: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.string(&self.clan_name);
            w.string(&self.char_name);
            w.u8(self.char_grade);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqAdjustClanMemberGrade {
        pub clan_name: String,
        pub char_name: String,
        pub char_grade: u8,
    }

    impl ReqAdjustClanMemberGrade {
        pub const OPCODE: u32 = 229;
        pub const NAME: &'static str = "rfReqAdjustClanMemberGrade";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
                char_name: r.string()?,
                char_grade: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            w.string(&self.char_name);
            w.u8(self.char_grade);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanUserListEnd;

    impl AckClanUserListEnd {
        pub const OPCODE: u32 = 228;
        pub const NAME: &'static str = "rfAckClanUserListEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanUserList {
        pub result: i32,
        pub err_msg: String,
        pub level: Vec<i32>,
        pub char_name: Vec<String>,
        pub clan_grade: Vec<u8>,
        pub clan_win_cnt: Vec<i32>,
        pub clan_lose_cnt: Vec<i32>,
        pub level_mark_id: Vec<i32>,
        pub title_mark_id: Vec<i32>,
        pub dude_id: Vec<i32>,
    }

    impl AckClanUserList {
        pub const OPCODE: u32 = 227;
        pub const NAME: &'static str = "rfAckClanUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                level: r.array(|r| r.i32())?,
                char_name: r.array(|r| r.string())?,
                clan_grade: r.array(|r| r.u8())?,
                clan_win_cnt: r.array(|r| r.i32())?,
                clan_lose_cnt: r.array(|r| r.i32())?,
                level_mark_id: r.array(|r| r.i32())?,
                title_mark_id: r.array(|r| r.i32())?,
                dude_id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.level, |w, v| { w.i32(*v); });
            w.array(&self.char_name, |w, v| { w.string(v); });
            w.array(&self.clan_grade, |w, v| { w.u8(*v); });
            w.array(&self.clan_win_cnt, |w, v| { w.i32(*v); });
            w.array(&self.clan_lose_cnt, |w, v| { w.i32(*v); });
            w.array(&self.level_mark_id, |w, v| { w.i32(*v); });
            w.array(&self.title_mark_id, |w, v| { w.i32(*v); });
            w.array(&self.dude_id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanUserListStart {
        pub result: i32,
        pub total_count: i32,
    }

    impl AckClanUserListStart {
        pub const OPCODE: u32 = 226;
        pub const NAME: &'static str = "rfAckClanUserListStart";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                total_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.total_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanUserList {
        pub clan_name: String,
    }

    impl ReqClanUserList {
        pub const OPCODE: u32 = 225;
        pub const NAME: &'static str = "rfReqClanUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckSearchClan {
        pub result: i32,
        pub err_msg: String,
        pub total_page: u16,
        pub clan_mark_pattern: Vec<i32>,
        pub clan_mark_bg: Vec<i32>,
        pub clan_mark_bl: Vec<u8>,
        pub clan_name: Vec<String>,
        pub clan_master: Vec<String>,
        pub clan_member: Vec<i32>,
        pub clan_member_max: Vec<i32>,
        pub clan_create_date: Vec<i32>,
        pub clan_win: Vec<i32>,
        pub clan_lose: Vec<i32>,
        pub clan_draw: Vec<i32>,
        pub clan_rank: Vec<i32>,
        pub clan_recruit: Vec<u8>,
        pub clan_level: Vec<i32>,
    }

    impl AckSearchClan {
        pub const OPCODE: u32 = 224;
        pub const NAME: &'static str = "rfAckSearchClan";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                total_page: r.u16()?,
                clan_mark_pattern: r.array(|r| r.i32())?,
                clan_mark_bg: r.array(|r| r.i32())?,
                clan_mark_bl: r.array(|r| r.u8())?,
                clan_name: r.array(|r| r.string())?,
                clan_master: r.array(|r| r.string())?,
                clan_member: r.array(|r| r.i32())?,
                clan_member_max: r.array(|r| r.i32())?,
                clan_create_date: r.array(|r| r.i32())?,
                clan_win: r.array(|r| r.i32())?,
                clan_lose: r.array(|r| r.i32())?,
                clan_draw: r.array(|r| r.i32())?,
                clan_rank: r.array(|r| r.i32())?,
                clan_recruit: r.array(|r| r.u8())?,
                clan_level: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.u16(self.total_page);
            w.array(&self.clan_mark_pattern, |w, v| { w.i32(*v); });
            w.array(&self.clan_mark_bg, |w, v| { w.i32(*v); });
            w.array(&self.clan_mark_bl, |w, v| { w.u8(*v); });
            w.array(&self.clan_name, |w, v| { w.string(v); });
            w.array(&self.clan_master, |w, v| { w.string(v); });
            w.array(&self.clan_member, |w, v| { w.i32(*v); });
            w.array(&self.clan_member_max, |w, v| { w.i32(*v); });
            w.array(&self.clan_create_date, |w, v| { w.i32(*v); });
            w.array(&self.clan_win, |w, v| { w.i32(*v); });
            w.array(&self.clan_lose, |w, v| { w.i32(*v); });
            w.array(&self.clan_draw, |w, v| { w.i32(*v); });
            w.array(&self.clan_rank, |w, v| { w.i32(*v); });
            w.array(&self.clan_recruit, |w, v| { w.u8(*v); });
            w.array(&self.clan_level, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSerachClan {
        pub search_type: u8,
        pub keyword: String,
        pub clan_status: u8,
        pub clan_region: u8,
        pub req_page: u16,
    }

    impl ReqSerachClan {
        pub const OPCODE: u32 = 223;
        pub const NAME: &'static str = "rfReqSerachClan";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                search_type: r.u8()?,
                keyword: r.string()?,
                clan_status: r.u8()?,
                clan_region: r.u8()?,
                req_page: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.search_type);
            w.string(&self.keyword);
            w.u8(self.clan_status);
            w.u8(self.clan_region);
            w.u16(self.req_page);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckJoinClanResult {
        pub clan_name: String,
        pub permit: u8,
        pub permit_char_name: String,
    }

    impl AckJoinClanResult {
        pub const OPCODE: u32 = 222;
        pub const NAME: &'static str = "rfAckJoinClanResult";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
                permit: r.u8()?,
                permit_char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            w.u8(self.permit);
            w.string(&self.permit_char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRecvJoinClanResult {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckRecvJoinClanResult {
        pub const OPCODE: u32 = 221;
        pub const NAME: &'static str = "rfAckRecvJoinClanResult";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRecvJoinClan {
        pub user_idx: i32,
        pub char_name: String,
        pub req_msg: String,
        pub permit: u8,
    }

    impl AckRecvJoinClan {
        pub const OPCODE: u32 = 220;
        pub const NAME: &'static str = "rfAckRecvJoinClan";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_idx: r.i32()?,
                char_name: r.string()?,
                req_msg: r.string()?,
                permit: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_idx);
            w.string(&self.char_name);
            w.string(&self.req_msg);
            w.u8(self.permit);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRecvJoinClan {
        pub user_idx: i32,
        pub char_name: String,
        pub message: String,
    }

    impl ReqRecvJoinClan {
        pub const OPCODE: u32 = 219;
        pub const NAME: &'static str = "rfReqRecvJoinClan";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_idx: r.i32()?,
                char_name: r.string()?,
                message: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_idx);
            w.string(&self.char_name);
            w.string(&self.message);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckJoinClan {
        pub result: i32,
        pub err_msg: String,
        pub specific_char_name: String,
        pub real_recv_char_name: String,
        pub real_recv_char_clan_grade: u8,
    }

    impl AckJoinClan {
        pub const OPCODE: u32 = 218;
        pub const NAME: &'static str = "rfAckJoinClan";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                specific_char_name: r.string()?,
                real_recv_char_name: r.string()?,
                real_recv_char_clan_grade: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.string(&self.specific_char_name);
            w.string(&self.real_recv_char_name);
            w.u8(self.real_recv_char_clan_grade);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqJoinClan {
        pub clan_name: String,
        pub clan_manager_name: String,
        pub message: String,
    }

    impl ReqJoinClan {
        pub const OPCODE: u32 = 217;
        pub const NAME: &'static str = "rfReqJoinClan";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
                clan_manager_name: r.string()?,
                message: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            w.string(&self.clan_manager_name);
            w.string(&self.message);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckInviteClanResult {
        pub char_name: String,
        pub permit: u8,
        pub clan_nm: String,
    }

    impl AckInviteClanResult {
        pub const OPCODE: u32 = 216;
        pub const NAME: &'static str = "rfAckInviteClanResult";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
                permit: r.u8()?,
                clan_nm: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            w.u8(self.permit);
            w.string(&self.clan_nm);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRecvClanInviteResult {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckRecvClanInviteResult {
        pub const OPCODE: u32 = 215;
        pub const NAME: &'static str = "rfAckRecvClanInviteResult";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRecvClanInvite {
        pub inviter_user_dbid: i32,
        pub inviter_charname: String,
        pub inviter_msg: String,
        pub clan_name: String,
        pub permit: u8,
    }

    impl AckRecvClanInvite {
        pub const OPCODE: u32 = 214;
        pub const NAME: &'static str = "rfAckRecvClanInvite";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                inviter_user_dbid: r.i32()?,
                inviter_charname: r.string()?,
                inviter_msg: r.string()?,
                clan_name: r.string()?,
                permit: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.inviter_user_dbid);
            w.string(&self.inviter_charname);
            w.string(&self.inviter_msg);
            w.string(&self.clan_name);
            w.u8(self.permit);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRecvClanInvite {
        pub inviter_user_dbid: i32,
        pub inviter_charname: String,
        pub clan_name: String,
        pub invite_msg: String,
    }

    impl ReqRecvClanInvite {
        pub const OPCODE: u32 = 213;
        pub const NAME: &'static str = "rfReqRecvClanInvite";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                inviter_user_dbid: r.i32()?,
                inviter_charname: r.string()?,
                clan_name: r.string()?,
                invite_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.inviter_user_dbid);
            w.string(&self.inviter_charname);
            w.string(&self.clan_name);
            w.string(&self.invite_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckInviteClan {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckInviteClan {
        pub const OPCODE: u32 = 212;
        pub const NAME: &'static str = "rfAckInviteClan";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqInviteClan {
        pub char_name: String,
        pub invite_msg: String,
    }

    impl ReqInviteClan {
        pub const OPCODE: u32 = 211;
        pub const NAME: &'static str = "rfReqInviteClan";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
                invite_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            w.string(&self.invite_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClanInfo {
        pub st_clan_info: ClanInfo,
    }

    impl AckClanInfo {
        pub const OPCODE: u32 = 210;
        pub const NAME: &'static str = "rfAckClanInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                st_clan_info: ClanInfo::decode(r)?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            self.st_clan_info.encode(w);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClanInfo {
        pub clan_name: String,
    }

    impl ReqClanInfo {
        pub const OPCODE: u32 = 209;
        pub const NAME: &'static str = "rfReqClanInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckCreateClan {
        pub result: i32,
        pub err_msg: String,
        pub item_idx: f64,
        pub stack_count: i32,
    }

    impl AckCreateClan {
        pub const OPCODE: u32 = 208;
        pub const NAME: &'static str = "rfAckCreateClan";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                item_idx: r.f64()?,
                stack_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.f64(self.item_idx);
            w.i32(self.stack_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqCreateClan {
        pub clan_name: String,
        pub clan_url: String,
        pub clan_intro: String,
        pub clan_keyword: String,
        pub clan_region: u8,
    }

    impl ReqCreateClan {
        pub const OPCODE: u32 = 207;
        pub const NAME: &'static str = "rfReqCreateClan";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_name: r.string()?,
                clan_url: r.string()?,
                clan_intro: r.string()?,
                clan_keyword: r.string()?,
                clan_region: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_name);
            w.string(&self.clan_url);
            w.string(&self.clan_intro);
            w.string(&self.clan_keyword);
            w.u8(self.clan_region);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckDupCheckClanName {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckDupCheckClanName {
        pub const OPCODE: u32 = 206;
        pub const NAME: &'static str = "rfAckDupCheckClanName";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqDupCheckClanName {
        pub clan_nam: String,
    }

    impl ReqDupCheckClanName {
        pub const OPCODE: u32 = 205;
        pub const NAME: &'static str = "rfReqDupCheckClanName";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                clan_nam: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.clan_nam);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckSellShopItemInfo {
        pub result: i32,
        pub err_msg: String,
        pub unique_item_id: f64,
        pub sell_point: i32,
        pub cur_point: i32,
        pub after_point: i32,
    }

    impl AckSellShopItemInfo {
        pub const OPCODE: u32 = 204;
        pub const NAME: &'static str = "rfAckSellShopItemInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                unique_item_id: r.f64()?,
                sell_point: r.i32()?,
                cur_point: r.i32()?,
                after_point: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.f64(self.unique_item_id);
            w.i32(self.sell_point);
            w.i32(self.cur_point);
            w.i32(self.after_point);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSellShopItemInfo {
        pub unique_item_id: f64,
        pub count: i32,
    }

    impl ReqSellShopItemInfo {
        pub const OPCODE: u32 = 203;
        pub const NAME: &'static str = "rfReqSellShopItemInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                unique_item_id: r.f64()?,
                count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.f64(self.unique_item_id);
            w.i32(self.count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGameOverEnd {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckGameOverEnd {
        pub const OPCODE: u32 = 202;
        pub const NAME: &'static str = "rfAckGameOverEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGameOverEnd;

    impl ReqGameOverEnd {
        pub const OPCODE: u32 = 201;
        pub const NAME: &'static str = "rfReqGameOverEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAuctionItemListCount {
        pub result: i32,
        pub err_msg: String,
        pub section: u8,
        pub classfy: u8,
        pub item_type: i32,
        pub item_grade: i32,
        pub count: i32,
    }

    impl AckAuctionItemListCount {
        pub const OPCODE: u32 = 200;
        pub const NAME: &'static str = "rfAckAuctionItemListCount";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                section: r.u8()?,
                classfy: r.u8()?,
                item_type: r.i32()?,
                item_grade: r.i32()?,
                count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.u8(self.section);
            w.u8(self.classfy);
            w.i32(self.item_type);
            w.i32(self.item_grade);
            w.i32(self.count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqAuctionItemListCount {
        pub section: u8,
        pub classfy: u8,
        pub item_type: i32,
        pub item_grade: i32,
    }

    impl ReqAuctionItemListCount {
        pub const OPCODE: u32 = 199;
        pub const NAME: &'static str = "rfReqAuctionItemListCount";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                section: r.u8()?,
                classfy: r.u8()?,
                item_type: r.i32()?,
                item_grade: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.section);
            w.u8(self.classfy);
            w.i32(self.item_type);
            w.i32(self.item_grade);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckServerTime {
        pub time: i32,
    }

    impl AckServerTime {
        pub const OPCODE: u32 = 198;
        pub const NAME: &'static str = "rfAckServerTime";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                time: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.time);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqServerTime;

    impl ReqServerTime {
        pub const OPCODE: u32 = 197;
        pub const NAME: &'static str = "rfReqServerTime";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckEntryQSlotIDList {
        pub user_num: i32,
        pub q_slot_disp_order: Vec<u8>,
        pub mw_item_id: Vec<i32>,
        pub mw_part_id: Vec<i32>,
        pub mw_paint_id: Vec<i32>,
        pub pistol_item_id: Vec<i32>,
        pub pistol_part_id: Vec<i32>,
        pub pistol_paint_id: Vec<i32>,
        pub tw_item_id1: Vec<i32>,
        pub tw_item_id2: Vec<i32>,
        pub skill1id: Vec<i32>,
        pub skill2id: Vec<i32>,
        pub skill3id: Vec<i32>,
        pub skill4id: Vec<i32>,
    }

    impl AckEntryQSlotIDList {
        pub const OPCODE: u32 = 196;
        pub const NAME: &'static str = "rfAckEntryQSlotIDList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_num: r.i32()?,
                q_slot_disp_order: r.array(|r| r.u8())?,
                mw_item_id: r.array(|r| r.i32())?,
                mw_part_id: r.array(|r| r.i32())?,
                mw_paint_id: r.array(|r| r.i32())?,
                pistol_item_id: r.array(|r| r.i32())?,
                pistol_part_id: r.array(|r| r.i32())?,
                pistol_paint_id: r.array(|r| r.i32())?,
                tw_item_id1: r.array(|r| r.i32())?,
                tw_item_id2: r.array(|r| r.i32())?,
                skill1id: r.array(|r| r.i32())?,
                skill2id: r.array(|r| r.i32())?,
                skill3id: r.array(|r| r.i32())?,
                skill4id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_num);
            w.array(&self.q_slot_disp_order, |w, v| { w.u8(*v); });
            w.array(&self.mw_item_id, |w, v| { w.i32(*v); });
            w.array(&self.mw_part_id, |w, v| { w.i32(*v); });
            w.array(&self.mw_paint_id, |w, v| { w.i32(*v); });
            w.array(&self.pistol_item_id, |w, v| { w.i32(*v); });
            w.array(&self.pistol_part_id, |w, v| { w.i32(*v); });
            w.array(&self.pistol_paint_id, |w, v| { w.i32(*v); });
            w.array(&self.tw_item_id1, |w, v| { w.i32(*v); });
            w.array(&self.tw_item_id2, |w, v| { w.i32(*v); });
            w.array(&self.skill1id, |w, v| { w.i32(*v); });
            w.array(&self.skill2id, |w, v| { w.i32(*v); });
            w.array(&self.skill3id, |w, v| { w.i32(*v); });
            w.array(&self.skill4id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqEntryQSlotIDList {
        pub user_num: i32,
    }

    impl ReqEntryQSlotIDList {
        pub const OPCODE: u32 = 195;
        pub const NAME: &'static str = "rfReqEntryQSlotIDList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_num: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckEntryQSlotUniqueIDList {
        pub user_num: i32,
        pub q_slot_disp_order: Vec<u8>,
        pub mw_unique_id: Vec<f64>,
        pub pistol_unique_id: Vec<f64>,
        pub tw1unique_id: Vec<f64>,
        pub tw2unique_id: Vec<f64>,
    }

    impl AckEntryQSlotUniqueIDList {
        pub const OPCODE: u32 = 194;
        pub const NAME: &'static str = "rfAckEntryQSlotUniqueIDList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_num: r.i32()?,
                q_slot_disp_order: r.array(|r| r.u8())?,
                mw_unique_id: r.array(|r| r.f64())?,
                pistol_unique_id: r.array(|r| r.f64())?,
                tw1unique_id: r.array(|r| r.f64())?,
                tw2unique_id: r.array(|r| r.f64())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_num);
            w.array(&self.q_slot_disp_order, |w, v| { w.u8(*v); });
            w.array(&self.mw_unique_id, |w, v| { w.f64(*v); });
            w.array(&self.pistol_unique_id, |w, v| { w.f64(*v); });
            w.array(&self.tw1unique_id, |w, v| { w.f64(*v); });
            w.array(&self.tw2unique_id, |w, v| { w.f64(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqEntryQSlotUniqueIDList {
        pub user_num: i32,
    }

    impl ReqEntryQSlotUniqueIDList {
        pub const OPCODE: u32 = 193;
        pub const NAME: &'static str = "rfReqEntryQSlotUniqueIDList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_num: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAuctionBidding {
        pub result: i32,
        pub err_msg: String,
        pub point_remain: i32,
    }

    impl AckAuctionBidding {
        pub const OPCODE: u32 = 192;
        pub const NAME: &'static str = "rfAckAuctionBidding";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                point_remain: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.point_remain);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqAuctionBidding {
        pub unique_id: i32,
        pub bidding_price: i32,
    }

    impl ReqAuctionBidding {
        pub const OPCODE: u32 = 191;
        pub const NAME: &'static str = "rfReqAuctionBidding";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                unique_id: r.i32()?,
                bidding_price: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.unique_id);
            w.i32(self.bidding_price);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAuctionInstantlyBuying {
        pub result: i32,
        pub err_msg: String,
        pub point_remain: i32,
    }

    impl AckAuctionInstantlyBuying {
        pub const OPCODE: u32 = 190;
        pub const NAME: &'static str = "rfAckAuctionInstantlyBuying";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                point_remain: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.point_remain);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqAuctionInstantlyBuying {
        pub unique_id: i32,
    }

    impl ReqAuctionInstantlyBuying {
        pub const OPCODE: u32 = 189;
        pub const NAME: &'static str = "rfReqAuctionInstantlyBuying";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                unique_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.unique_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAuctionItemListEnd {
        pub section: u8,
    }

    impl AckAuctionItemListEnd {
        pub const OPCODE: u32 = 188;
        pub const NAME: &'static str = "rfAckAuctionItemListEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                section: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.section);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAuctionItemList {
        pub section: u8,
        pub unique_i_ds: Vec<i32>,
        pub item_i_ds: Vec<i32>,
        pub add_parts_i_ds: Vec<i32>,
        pub until_times: Vec<f64>,
        pub bidding_prices: Vec<i32>,
        pub instantly_byuing_prices: Vec<i32>,
        pub item_damageds: Vec<i32>,
        pub highest_bidder_names: Vec<String>,
        pub seller_names: Vec<String>,
        pub item_count: Vec<i32>,
    }

    impl AckAuctionItemList {
        pub const OPCODE: u32 = 187;
        pub const NAME: &'static str = "rfAckAuctionItemList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                section: r.u8()?,
                unique_i_ds: r.array(|r| r.i32())?,
                item_i_ds: r.array(|r| r.i32())?,
                add_parts_i_ds: r.array(|r| r.i32())?,
                until_times: r.array(|r| r.f64())?,
                bidding_prices: r.array(|r| r.i32())?,
                instantly_byuing_prices: r.array(|r| r.i32())?,
                item_damageds: r.array(|r| r.i32())?,
                highest_bidder_names: r.array(|r| r.string())?,
                seller_names: r.array(|r| r.string())?,
                item_count: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.section);
            w.array(&self.unique_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.item_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.add_parts_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.until_times, |w, v| { w.f64(*v); });
            w.array(&self.bidding_prices, |w, v| { w.i32(*v); });
            w.array(&self.instantly_byuing_prices, |w, v| { w.i32(*v); });
            w.array(&self.item_damageds, |w, v| { w.i32(*v); });
            w.array(&self.highest_bidder_names, |w, v| { w.string(v); });
            w.array(&self.seller_names, |w, v| { w.string(v); });
            w.array(&self.item_count, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAuctionItemListStart {
        pub result: i32,
        pub err_msg: String,
        pub section: u8,
        pub page_num: i32,
        pub count: i32,
    }

    impl AckAuctionItemListStart {
        pub const OPCODE: u32 = 186;
        pub const NAME: &'static str = "rfAckAuctionItemListStart";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                section: r.u8()?,
                page_num: r.i32()?,
                count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.u8(self.section);
            w.i32(self.page_num);
            w.i32(self.count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqAuctionItemList {
        pub section: u8,
        pub keyword: String,
        pub item_rank: i32,
        pub item_classfy: u8,
        pub is_available: u8,
        pub page_num: i32,
        pub count: i32,
        pub sort_opt: i32,
    }

    impl ReqAuctionItemList {
        pub const OPCODE: u32 = 185;
        pub const NAME: &'static str = "rfReqAuctionItemList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                section: r.u8()?,
                keyword: r.string()?,
                item_rank: r.i32()?,
                item_classfy: r.u8()?,
                is_available: r.u8()?,
                page_num: r.i32()?,
                count: r.i32()?,
                sort_opt: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.section);
            w.string(&self.keyword);
            w.i32(self.item_rank);
            w.u8(self.item_classfy);
            w.u8(self.is_available);
            w.i32(self.page_num);
            w.i32(self.count);
            w.i32(self.sort_opt);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAuctionMyItemCancel {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckAuctionMyItemCancel {
        pub const OPCODE: u32 = 184;
        pub const NAME: &'static str = "rfAckAuctionMyItemCancel";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqAuctionMyItemCancel {
        pub unique_id: i32,
    }

    impl ReqAuctionMyItemCancel {
        pub const OPCODE: u32 = 183;
        pub const NAME: &'static str = "rfReqAuctionMyItemCancel";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                unique_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.unique_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAuctionMyItemRegister {
        pub result: i32,
        pub err_msg: String,
        pub begin_price: i32,
        pub buy_price: i32,
        pub guaranty_point: i32,
        pub point_remain: i32,
    }

    impl AckAuctionMyItemRegister {
        pub const OPCODE: u32 = 182;
        pub const NAME: &'static str = "rfAckAuctionMyItemRegister";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                begin_price: r.i32()?,
                buy_price: r.i32()?,
                guaranty_point: r.i32()?,
                point_remain: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.begin_price);
            w.i32(self.buy_price);
            w.i32(self.guaranty_point);
            w.i32(self.point_remain);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqAuctionMyItemRegister {
        pub unique_id: f64,
        pub item_count: i32,
        pub begin_price: i32,
        pub period_hour: i32,
        pub instantly_buying_price: i32,
    }

    impl ReqAuctionMyItemRegister {
        pub const OPCODE: u32 = 181;
        pub const NAME: &'static str = "rfReqAuctionMyItemRegister";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                unique_id: r.f64()?,
                item_count: r.i32()?,
                begin_price: r.i32()?,
                period_hour: r.i32()?,
                instantly_buying_price: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.f64(self.unique_id);
            w.i32(self.item_count);
            w.i32(self.begin_price);
            w.i32(self.period_hour);
            w.i32(self.instantly_buying_price);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckMyMoney {
        pub result: i32,
        pub err_msg: String,
        pub point: i32,
        pub cash: i32,
    }

    impl AckMyMoney {
        pub const OPCODE: u32 = 180;
        pub const NAME: &'static str = "rfAckMyMoney";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                point: r.i32()?,
                cash: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.point);
            w.i32(self.cash);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqMyMoney;

    impl ReqMyMoney {
        pub const OPCODE: u32 = 179;
        pub const NAME: &'static str = "rfReqMyMoney";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckUpdateQSlot {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckUpdateQSlot {
        pub const OPCODE: u32 = 178;
        pub const NAME: &'static str = "rfAckUpdateQSlot";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqUpdateQSlot {
        pub q_slot_id: i32,
        pub q_slot_name: String,
        pub mw_item_id: f64,
        pub pistol_item_id: f64,
        pub tw_item_id1: f64,
        pub tw_item_id2: f64,
        pub skill1: i32,
        pub skill2: i32,
        pub skill3: i32,
        pub skill4: i32,
    }

    impl ReqUpdateQSlot {
        pub const OPCODE: u32 = 177;
        pub const NAME: &'static str = "rfReqUpdateQSlot";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                q_slot_id: r.i32()?,
                q_slot_name: r.string()?,
                mw_item_id: r.f64()?,
                pistol_item_id: r.f64()?,
                tw_item_id1: r.f64()?,
                tw_item_id2: r.f64()?,
                skill1: r.i32()?,
                skill2: r.i32()?,
                skill3: r.i32()?,
                skill4: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.q_slot_id);
            w.string(&self.q_slot_name);
            w.f64(self.mw_item_id);
            w.f64(self.pistol_item_id);
            w.f64(self.tw_item_id1);
            w.f64(self.tw_item_id2);
            w.i32(self.skill1);
            w.i32(self.skill2);
            w.i32(self.skill3);
            w.i32(self.skill4);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckQSlotList {
        pub result: i32,
        pub err_msg: String,
        pub q_slot_id: Vec<i32>,
        pub q_slot_name: Vec<String>,
        pub q_slot_disp_order: Vec<u8>,
        pub mw_item_id: Vec<f64>,
        pub pistol_item_id: Vec<f64>,
        pub tw_item_id1: Vec<f64>,
        pub tw_item_id2: Vec<f64>,
        pub skill1: Vec<i32>,
        pub skill2: Vec<i32>,
        pub skill3: Vec<i32>,
        pub skill4: Vec<i32>,
    }

    impl AckQSlotList {
        pub const OPCODE: u32 = 176;
        pub const NAME: &'static str = "rfAckQSlotList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                q_slot_id: r.array(|r| r.i32())?,
                q_slot_name: r.array(|r| r.string())?,
                q_slot_disp_order: r.array(|r| r.u8())?,
                mw_item_id: r.array(|r| r.f64())?,
                pistol_item_id: r.array(|r| r.f64())?,
                tw_item_id1: r.array(|r| r.f64())?,
                tw_item_id2: r.array(|r| r.f64())?,
                skill1: r.array(|r| r.i32())?,
                skill2: r.array(|r| r.i32())?,
                skill3: r.array(|r| r.i32())?,
                skill4: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.q_slot_id, |w, v| { w.i32(*v); });
            w.array(&self.q_slot_name, |w, v| { w.string(v); });
            w.array(&self.q_slot_disp_order, |w, v| { w.u8(*v); });
            w.array(&self.mw_item_id, |w, v| { w.f64(*v); });
            w.array(&self.pistol_item_id, |w, v| { w.f64(*v); });
            w.array(&self.tw_item_id1, |w, v| { w.f64(*v); });
            w.array(&self.tw_item_id2, |w, v| { w.f64(*v); });
            w.array(&self.skill1, |w, v| { w.i32(*v); });
            w.array(&self.skill2, |w, v| { w.i32(*v); });
            w.array(&self.skill3, |w, v| { w.i32(*v); });
            w.array(&self.skill4, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqQSlotList;

    impl ReqQSlotList {
        pub const OPCODE: u32 = 175;
        pub const NAME: &'static str = "rfReqQSlotList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRepairItem {
        pub result: i32,
        pub err_msg: String,
        pub unique_item_id: Vec<f64>,
        pub repair_cost: i32,
        pub damage_degree: Vec<i32>,
    }

    impl AckRepairItem {
        pub const OPCODE: u32 = 174;
        pub const NAME: &'static str = "rfAckRepairItem";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                unique_item_id: r.array(|r| r.f64())?,
                repair_cost: r.i32()?,
                damage_degree: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.unique_item_id, |w, v| { w.f64(*v); });
            w.i32(self.repair_cost);
            w.array(&self.damage_degree, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRepairItem {
        pub unique_item_id: Vec<f64>,
    }

    impl ReqRepairItem {
        pub const OPCODE: u32 = 173;
        pub const NAME: &'static str = "rfReqRepairItem";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                unique_item_id: r.array(|r| r.f64())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.unique_item_id, |w, v| { w.f64(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRepairCost {
        pub result: i32,
        pub err_msg: String,
        pub unique_item_id: Vec<f64>,
        pub repair_cost: Vec<i32>,
    }

    impl AckRepairCost {
        pub const OPCODE: u32 = 172;
        pub const NAME: &'static str = "rfAckRepairCost";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                unique_item_id: r.array(|r| r.f64())?,
                repair_cost: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.unique_item_id, |w, v| { w.f64(*v); });
            w.array(&self.repair_cost, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRepairCost {
        pub unique_item_id: Vec<f64>,
    }

    impl ReqRepairCost {
        pub const OPCODE: u32 = 171;
        pub const NAME: &'static str = "rfReqRepairCost";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                unique_item_id: r.array(|r| r.f64())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.unique_item_id, |w, v| { w.f64(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckUpdateWeaponDurability {
        pub result: i32,
        pub err_msg: String,
        pub unique_item_id: Vec<f64>,
        pub durability: Vec<i32>,
        pub is_destroy: Vec<u8>,
        pub failed_weapon_unique_id: Vec<f64>,
    }

    impl AckUpdateWeaponDurability {
        pub const OPCODE: u32 = 170;
        pub const NAME: &'static str = "rfAckUpdateWeaponDurability";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                unique_item_id: r.array(|r| r.f64())?,
                durability: r.array(|r| r.i32())?,
                is_destroy: r.array(|r| r.u8())?,
                failed_weapon_unique_id: r.array(|r| r.f64())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.unique_item_id, |w, v| { w.f64(*v); });
            w.array(&self.durability, |w, v| { w.i32(*v); });
            w.array(&self.is_destroy, |w, v| { w.u8(*v); });
            w.array(&self.failed_weapon_unique_id, |w, v| { w.f64(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqUpdateWeaponDurability {
        pub total_play_time: i32,
        pub owner_id: Vec<i32>,
        pub number: Vec<i32>,
        pub unique_item_id: Vec<f64>,
        pub using_sec: Vec<i32>,
        pub using_round_in_sd: Vec<i32>,
    }

    impl ReqUpdateWeaponDurability {
        pub const OPCODE: u32 = 169;
        pub const NAME: &'static str = "rfReqUpdateWeaponDurability";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                total_play_time: r.i32()?,
                owner_id: r.array(|r| r.i32())?,
                number: r.array(|r| r.i32())?,
                unique_item_id: r.array(|r| r.f64())?,
                using_sec: r.array(|r| r.i32())?,
                using_round_in_sd: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.total_play_time);
            w.array(&self.owner_id, |w, v| { w.i32(*v); });
            w.array(&self.number, |w, v| { w.i32(*v); });
            w.array(&self.unique_item_id, |w, v| { w.f64(*v); });
            w.array(&self.using_sec, |w, v| { w.i32(*v); });
            w.array(&self.using_round_in_sd, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckModifyWeapon {
        pub result: i32,
        pub err_msg: String,
        pub unique_id: f64,
        pub add_part_id: i32,
        pub paint_id: i32,
        pub point: i32,
        pub cash: i32,
    }

    impl AckModifyWeapon {
        pub const OPCODE: u32 = 168;
        pub const NAME: &'static str = "rfAckModifyWeapon";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                unique_id: r.f64()?,
                add_part_id: r.i32()?,
                paint_id: r.i32()?,
                point: r.i32()?,
                cash: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.f64(self.unique_id);
            w.i32(self.add_part_id);
            w.i32(self.paint_id);
            w.i32(self.point);
            w.i32(self.cash);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqModifyWeapon {
        pub unique_id: f64,
        pub add_part_id: i32,
        pub paint_id: i32,
    }

    impl ReqModifyWeapon {
        pub const OPCODE: u32 = 167;
        pub const NAME: &'static str = "rfReqModifyWeapon";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                unique_id: r.f64()?,
                add_part_id: r.i32()?,
                paint_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.f64(self.unique_id);
            w.i32(self.add_part_id);
            w.i32(self.paint_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckSellShopItem {
        pub result: i32,
        pub err_msg: String,
        pub point_remain: i32,
        pub cash_remain: i32,
        pub count: i32,
    }

    impl AckSellShopItem {
        pub const OPCODE: u32 = 166;
        pub const NAME: &'static str = "rfAckSellShopItem";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                point_remain: r.i32()?,
                cash_remain: r.i32()?,
                count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.point_remain);
            w.i32(self.cash_remain);
            w.i32(self.count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSellShopItem {
        pub item_unique_id: f64,
        pub count: i32,
    }

    impl ReqSellShopItem {
        pub const OPCODE: u32 = 165;
        pub const NAME: &'static str = "rfReqSellShopItem";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_unique_id: r.f64()?,
                count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.f64(self.item_unique_id);
            w.i32(self.count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckBuyShopItem {
        pub item_count: i32,
        pub result: Vec<i32>,
        pub bought_item_type_id: Vec<i32>,
        pub bought_item_unique_id: Vec<f64>,
        pub bought_part_id: Vec<i32>,
        pub bought_until_t_ime: Vec<f64>,
        pub bought_stack_cnt: Vec<i32>,
        pub point_remain: i32,
        pub cash_remain: i32,
    }

    impl AckBuyShopItem {
        pub const OPCODE: u32 = 164;
        pub const NAME: &'static str = "rfAckBuyShopItem";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_count: r.i32()?,
                result: r.array(|r| r.i32())?,
                bought_item_type_id: r.array(|r| r.i32())?,
                bought_item_unique_id: r.array(|r| r.f64())?,
                bought_part_id: r.array(|r| r.i32())?,
                bought_until_t_ime: r.array(|r| r.f64())?,
                bought_stack_cnt: r.array(|r| r.i32())?,
                point_remain: r.i32()?,
                cash_remain: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.item_count);
            w.array(&self.result, |w, v| { w.i32(*v); });
            w.array(&self.bought_item_type_id, |w, v| { w.i32(*v); });
            w.array(&self.bought_item_unique_id, |w, v| { w.f64(*v); });
            w.array(&self.bought_part_id, |w, v| { w.i32(*v); });
            w.array(&self.bought_until_t_ime, |w, v| { w.f64(*v); });
            w.array(&self.bought_stack_cnt, |w, v| { w.i32(*v); });
            w.i32(self.point_remain);
            w.i32(self.cash_remain);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqBuyShopItem {
        pub item_id: i32,
        pub item_unique_id: f64,
        pub buy_option1: u8,
        pub buy_option2: u8,
    }

    impl ReqBuyShopItem {
        pub const OPCODE: u32 = 163;
        pub const NAME: &'static str = "rfReqBuyShopItem";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_id: r.i32()?,
                item_unique_id: r.f64()?,
                buy_option1: r.u8()?,
                buy_option2: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.item_id);
            w.f64(self.item_unique_id);
            w.u8(self.buy_option1);
            w.u8(self.buy_option2);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckShopItemListEnd;

    impl AckShopItemListEnd {
        pub const OPCODE: u32 = 162;
        pub const NAME: &'static str = "rfAckShopItemListEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckShopItemList {
        pub item_id: Vec<i32>,
    }

    impl AckShopItemList {
        pub const OPCODE: u32 = 161;
        pub const NAME: &'static str = "rfAckShopItemList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.item_id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckShopItemListStart {
        pub count: i32,
    }

    impl AckShopItemListStart {
        pub const OPCODE: u32 = 160;
        pub const NAME: &'static str = "rfAckShopItemListStart";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqShopItemList;

    impl ReqShopItemList {
        pub const OPCODE: u32 = 159;
        pub const NAME: &'static str = "rfReqShopItemList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckForceGameOver {
        pub user_id: i32,
        pub game_room_number: u16,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckForceGameOver {
        pub const OPCODE: u32 = 158;
        pub const NAME: &'static str = "rfAckForceGameOver";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
                game_room_number: r.u16()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            w.u16(self.game_room_number);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqForceGameOver;

    impl ReqForceGameOver {
        pub const OPCODE: u32 = 157;
        pub const NAME: &'static str = "rfReqForceGameOver";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGiveSkill {
        pub skill_id: i32,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckGiveSkill {
        pub const OPCODE: u32 = 156;
        pub const NAME: &'static str = "rfAckGiveSkill";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                skill_id: r.i32()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.skill_id);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGiveSkill;

    impl ReqGiveSkill {
        pub const OPCODE: u32 = 155;
        pub const NAME: &'static str = "rfReqGiveSkill";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNotifyExpiredItemIDs {
        pub user_id: i32,
        pub expired_item_unique_i_ds: Vec<f64>,
        pub expired_item_i_ds: Vec<i32>,
        pub expired_item_part_i_ds: Vec<i32>,
        pub expired_item_slot_pos: Vec<u8>,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckNotifyExpiredItemIDs {
        pub const OPCODE: u32 = 154;
        pub const NAME: &'static str = "rfAckNotifyExpiredItemIDs";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
                expired_item_unique_i_ds: r.array(|r| r.f64())?,
                expired_item_i_ds: r.array(|r| r.i32())?,
                expired_item_part_i_ds: r.array(|r| r.i32())?,
                expired_item_slot_pos: r.array(|r| r.u8())?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            w.array(&self.expired_item_unique_i_ds, |w, v| { w.f64(*v); });
            w.array(&self.expired_item_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.expired_item_part_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.expired_item_slot_pos, |w, v| { w.u8(*v); });
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyExpiredItemIDs;

    impl ReqNotifyExpiredItemIDs {
        pub const OPCODE: u32 = 153;
        pub const NAME: &'static str = "rfReqNotifyExpiredItemIDs";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNotifyExpiredSkillIDs {
        pub user_id: i32,
        pub expired_skill_unique_i_ds: Vec<f64>,
        pub expired_skill_i_ds: Vec<i32>,
        pub result: i32,
        pub err_msg: String,
        pub expired_skill_slot_positions: Vec<u8>,
    }

    impl AckNotifyExpiredSkillIDs {
        pub const OPCODE: u32 = 152;
        pub const NAME: &'static str = "rfAckNotifyExpiredSkillIDs";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
                expired_skill_unique_i_ds: r.array(|r| r.f64())?,
                expired_skill_i_ds: r.array(|r| r.i32())?,
                result: r.i32()?,
                err_msg: r.string()?,
                expired_skill_slot_positions: r.array(|r| r.u8())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            w.array(&self.expired_skill_unique_i_ds, |w, v| { w.f64(*v); });
            w.array(&self.expired_skill_i_ds, |w, v| { w.i32(*v); });
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.expired_skill_slot_positions, |w, v| { w.u8(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyExpiredSkillIDs;

    impl ReqNotifyExpiredSkillIDs {
        pub const OPCODE: u32 = 151;
        pub const NAME: &'static str = "rfReqNotifyExpiredSkillIDs";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNotifyChangedSkillState {
        pub user_id: i32,
        pub skill_i_ds: Vec<i32>,
        pub new_states: Vec<u8>,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckNotifyChangedSkillState {
        pub const OPCODE: u32 = 150;
        pub const NAME: &'static str = "rfAckNotifyChangedSkillState";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
                skill_i_ds: r.array(|r| r.i32())?,
                new_states: r.array(|r| r.u8())?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            w.array(&self.skill_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.new_states, |w, v| { w.u8(*v); });
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyChangedSkillState;

    impl ReqNotifyChangedSkillState {
        pub const OPCODE: u32 = 149;
        pub const NAME: &'static str = "rfReqNotifyChangedSkillState";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNotifyChangedItemState {
        pub user_id: i32,
        pub item_i_ds: Vec<i32>,
        pub new_states: Vec<u8>,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckNotifyChangedItemState {
        pub const OPCODE: u32 = 148;
        pub const NAME: &'static str = "rfAckNotifyChangedItemState";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
                item_i_ds: r.array(|r| r.i32())?,
                new_states: r.array(|r| r.u8())?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            w.array(&self.item_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.new_states, |w, v| { w.u8(*v); });
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyChangedItemState;

    impl ReqNotifyChangedItemState {
        pub const OPCODE: u32 = 147;
        pub const NAME: &'static str = "rfReqNotifyChangedItemState";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckConfirmSkillChanged {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckConfirmSkillChanged {
        pub const OPCODE: u32 = 146;
        pub const NAME: &'static str = "rfAckConfirmSkillChanged";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqConfirmSkillChanged;

    impl ReqConfirmSkillChanged {
        pub const OPCODE: u32 = 145;
        pub const NAME: &'static str = "rfReqConfirmSkillChanged";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckEquipSkill {
        pub changed_new_skill_id: i32,
        pub changed_new_skill_unique_id: i32,
        pub changed_new_slot_position: u8,
        pub changed_old_skill_id: i32,
        pub changed_old_skill_unique_id: i32,
        pub changed_old_slot_position: u8,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckEquipSkill {
        pub const OPCODE: u32 = 144;
        pub const NAME: &'static str = "rfAckEquipSkill";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                changed_new_skill_id: r.i32()?,
                changed_new_skill_unique_id: r.i32()?,
                changed_new_slot_position: r.u8()?,
                changed_old_skill_id: r.i32()?,
                changed_old_skill_unique_id: r.i32()?,
                changed_old_slot_position: r.u8()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.changed_new_skill_id);
            w.i32(self.changed_new_skill_unique_id);
            w.u8(self.changed_new_slot_position);
            w.i32(self.changed_old_skill_id);
            w.i32(self.changed_old_skill_unique_id);
            w.u8(self.changed_old_slot_position);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqEquipSkill {
        pub skill_id: i32,
        pub skill_unique_id: i32,
        pub new_slot_position: u8,
    }

    impl ReqEquipSkill {
        pub const OPCODE: u32 = 143;
        pub const NAME: &'static str = "rfReqEquipSkill";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                skill_id: r.i32()?,
                skill_unique_id: r.i32()?,
                new_slot_position: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.skill_id);
            w.i32(self.skill_unique_id);
            w.u8(self.new_slot_position);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckUserSkillListEnd;

    impl AckUserSkillListEnd {
        pub const OPCODE: u32 = 142;
        pub const NAME: &'static str = "rfAckUserSkillListEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckUserSkillList {
        pub unique_skill_id: Vec<i32>,
        pub skill_id: Vec<i32>,
        pub until_time: Vec<f64>,
        pub slot_position: Vec<u8>,
    }

    impl AckUserSkillList {
        pub const OPCODE: u32 = 141;
        pub const NAME: &'static str = "rfAckUserSkillList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                unique_skill_id: r.array(|r| r.i32())?,
                skill_id: r.array(|r| r.i32())?,
                until_time: r.array(|r| r.f64())?,
                slot_position: r.array(|r| r.u8())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.unique_skill_id, |w, v| { w.i32(*v); });
            w.array(&self.skill_id, |w, v| { w.i32(*v); });
            w.array(&self.until_time, |w, v| { w.f64(*v); });
            w.array(&self.slot_position, |w, v| { w.u8(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckUserSkillListStart {
        pub count: i32,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckUserSkillListStart {
        pub const OPCODE: u32 = 140;
        pub const NAME: &'static str = "rfAckUserSkillListStart";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                count: r.i32()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.count);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqUserSkillList {
        pub user_id: i32,
    }

    impl ReqUserSkillList {
        pub const OPCODE: u32 = 139;
        pub const NAME: &'static str = "rfReqUserSkillList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckDelMemo {
        pub result: i32,
        pub err_msg: String,
        pub failed_memo_id: Vec<i32>,
    }

    impl AckDelMemo {
        pub const OPCODE: u32 = 138;
        pub const NAME: &'static str = "rfAckDelMemo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                failed_memo_id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.failed_memo_id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqDelMemo {
        pub memo_idx: Vec<i32>,
        pub type_: u8,
    }

    impl ReqDelMemo {
        pub const OPCODE: u32 = 137;
        pub const NAME: &'static str = "rfReqDelMemo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                memo_idx: r.array(|r| r.i32())?,
                type_: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.memo_idx, |w, v| { w.i32(*v); });
            w.u8(self.type_);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckReadMemo {
        pub cname_from: String,
        pub text: String,
        pub date_recv: String,
        pub item_uid: f64,
    }

    impl AckReadMemo {
        pub const OPCODE: u32 = 136;
        pub const NAME: &'static str = "rfAckReadMemo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                cname_from: r.string()?,
                text: r.string()?,
                date_recv: r.string()?,
                item_uid: r.f64()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.cname_from);
            w.string(&self.text);
            w.string(&self.date_recv);
            w.f64(self.item_uid);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqReadMemo {
        pub memo_idx: i32,
    }

    impl ReqReadMemo {
        pub const OPCODE: u32 = 135;
        pub const NAME: &'static str = "rfReqReadMemo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                memo_idx: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.memo_idx);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckEndMemoList;

    impl AckEndMemoList {
        pub const OPCODE: u32 = 134;
        pub const NAME: &'static str = "rfAckEndMemoList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckMemoList {
        pub memo_idx: Vec<i32>,
        pub cname_from: Vec<String>,
        pub date_recv: Vec<String>,
        pub text: Vec<String>,
        pub status: Vec<u8>,
        pub until_expire: Vec<i32>,
        pub item_uid: Vec<f64>,
        pub item_id: Vec<i32>,
        pub msg_type: Vec<u8>,
        pub item_part_id: Vec<i32>,
    }

    impl AckMemoList {
        pub const OPCODE: u32 = 133;
        pub const NAME: &'static str = "rfAckMemoList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                memo_idx: r.array(|r| r.i32())?,
                cname_from: r.array(|r| r.string())?,
                date_recv: r.array(|r| r.string())?,
                text: r.array(|r| r.string())?,
                status: r.array(|r| r.u8())?,
                until_expire: r.array(|r| r.i32())?,
                item_uid: r.array(|r| r.f64())?,
                item_id: r.array(|r| r.i32())?,
                msg_type: r.array(|r| r.u8())?,
                item_part_id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.memo_idx, |w, v| { w.i32(*v); });
            w.array(&self.cname_from, |w, v| { w.string(v); });
            w.array(&self.date_recv, |w, v| { w.string(v); });
            w.array(&self.text, |w, v| { w.string(v); });
            w.array(&self.status, |w, v| { w.u8(*v); });
            w.array(&self.until_expire, |w, v| { w.i32(*v); });
            w.array(&self.item_uid, |w, v| { w.f64(*v); });
            w.array(&self.item_id, |w, v| { w.i32(*v); });
            w.array(&self.msg_type, |w, v| { w.u8(*v); });
            w.array(&self.item_part_id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckStartMemoList {
        pub result: i32,
        pub err_msg: String,
        pub type_: u8,
        pub total_count: i32,
    }

    impl AckStartMemoList {
        pub const OPCODE: u32 = 132;
        pub const NAME: &'static str = "rfAckStartMemoList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                type_: r.u8()?,
                total_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.u8(self.type_);
            w.i32(self.total_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqMemoList {
        pub type_: u8,
        pub memo_type: u8,
    }

    impl ReqMemoList {
        pub const OPCODE: u32 = 131;
        pub const NAME: &'static str = "rfReqMemoList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                type_: r.u8()?,
                memo_type: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.type_);
            w.u8(self.memo_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRecvMemo {
        pub cname_from: String,
        pub memo_type: u8,
    }

    impl ReqRecvMemo {
        pub const OPCODE: u32 = 130;
        pub const NAME: &'static str = "rfReqRecvMemo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                cname_from: r.string()?,
                memo_type: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.cname_from);
            w.u8(self.memo_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckSendMemo {
        pub cnames_to_fail: Vec<String>,
        pub result_array: Vec<i32>,
    }

    impl AckSendMemo {
        pub const OPCODE: u32 = 129;
        pub const NAME: &'static str = "rfAckSendMemo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                cnames_to_fail: r.array(|r| r.string())?,
                result_array: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.cnames_to_fail, |w, v| { w.string(v); });
            w.array(&self.result_array, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSendMemo {
        pub cnames: Vec<String>,
        pub msg: String,
        pub item_uid: i32,
    }

    impl ReqSendMemo {
        pub const OPCODE: u32 = 128;
        pub const NAME: &'static str = "rfReqSendMemo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                cnames: r.array(|r| r.string())?,
                msg: r.string()?,
                item_uid: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.cnames, |w, v| { w.string(v); });
            w.string(&self.msg);
            w.i32(self.item_uid);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRecvChatWispher {
        pub charname_from: String,
        pub message: String,
    }

    impl ReqRecvChatWispher {
        pub const OPCODE: u32 = 127;
        pub const NAME: &'static str = "rfReqRecvChatWispher";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                charname_from: r.string()?,
                message: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.charname_from);
            w.string(&self.message);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChatWispher {
        pub result: i32,
        pub err_msg: String,
        pub to_char_name: String,
    }

    impl AckChatWispher {
        pub const OPCODE: u32 = 126;
        pub const NAME: &'static str = "rfAckChatWispher";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                to_char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.string(&self.to_char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqChatWispher {
        pub char_name: String,
        pub message: String,
    }

    impl ReqChatWispher {
        pub const OPCODE: u32 = 125;
        pub const NAME: &'static str = "rfReqChatWispher";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
                message: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            w.string(&self.message);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRemoveBlockUserList {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckRemoveBlockUserList {
        pub const OPCODE: u32 = 124;
        pub const NAME: &'static str = "rfAckRemoveBlockUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRemoveBlockUserList {
        pub char_name: String,
    }

    impl ReqRemoveBlockUserList {
        pub const OPCODE: u32 = 123;
        pub const NAME: &'static str = "rfReqRemoveBlockUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckAddBlockUserList {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckAddBlockUserList {
        pub const OPCODE: u32 = 122;
        pub const NAME: &'static str = "rfAckAddBlockUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqAddBlockUserList {
        pub char_name: String,
    }

    impl ReqAddBlockUserList {
        pub const OPCODE: u32 = 121;
        pub const NAME: &'static str = "rfReqAddBlockUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckBlockUserListEnd;

    impl AckBlockUserListEnd {
        pub const OPCODE: u32 = 120;
        pub const NAME: &'static str = "rfAckBlockUserListEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckBlockUserList {
        pub char_name: Vec<String>,
    }

    impl AckBlockUserList {
        pub const OPCODE: u32 = 119;
        pub const NAME: &'static str = "rfAckBlockUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.array(|r| r.string())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.char_name, |w, v| { w.string(v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckBlockUserListStart {
        pub result: i32,
        pub total_count: i32,
    }

    impl AckBlockUserListStart {
        pub const OPCODE: u32 = 118;
        pub const NAME: &'static str = "rfAckBlockUserListStart";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                total_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.total_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqBlockUserList;

    impl ReqBlockUserList {
        pub const OPCODE: u32 = 117;
        pub const NAME: &'static str = "rfReqBlockUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqCompleteFriendInvite {
        pub charname_to_be_invited: String,
        pub is_permit: u8,
    }

    impl ReqCompleteFriendInvite {
        pub const OPCODE: u32 = 116;
        pub const NAME: &'static str = "rfReqCompleteFriendInvite";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                charname_to_be_invited: r.string()?,
                is_permit: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.charname_to_be_invited);
            w.u8(self.is_permit);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckConfirmFriendInvite {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckConfirmFriendInvite {
        pub const OPCODE: u32 = 115;
        pub const NAME: &'static str = "rfAckConfirmFriendInvite";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqConfirmFriendInvite {
        pub req_user_id: i32,
        pub reqed_charname: String,
        pub is_permit: u8,
    }

    impl ReqConfirmFriendInvite {
        pub const OPCODE: u32 = 114;
        pub const NAME: &'static str = "rfReqConfirmFriendInvite";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                req_user_id: r.i32()?,
                reqed_charname: r.string()?,
                is_permit: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.req_user_id);
            w.string(&self.reqed_charname);
            w.u8(self.is_permit);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRecvFriendInvite {
        pub req_user_id: i32,
        pub req_charname: String,
        pub invite_msg: String,
    }

    impl ReqRecvFriendInvite {
        pub const OPCODE: u32 = 113;
        pub const NAME: &'static str = "rfReqRecvFriendInvite";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                req_user_id: r.i32()?,
                req_charname: r.string()?,
                invite_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.req_user_id);
            w.string(&self.req_charname);
            w.string(&self.invite_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckSendFriendInvite {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckSendFriendInvite {
        pub const OPCODE: u32 = 112;
        pub const NAME: &'static str = "rfAckSendFriendInvite";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSendFriendInvite {
        pub invite_user_id: i32,
        pub invite_name: String,
        pub invite_msg: String,
    }

    impl ReqSendFriendInvite {
        pub const OPCODE: u32 = 111;
        pub const NAME: &'static str = "rfReqSendFriendInvite";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                invite_user_id: r.i32()?,
                invite_name: r.string()?,
                invite_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.invite_user_id);
            w.string(&self.invite_name);
            w.string(&self.invite_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChatNormal {
        pub id: u16,
        pub sender: String,
        pub message: String,
        pub result: i32,
    }

    impl AckChatNormal {
        pub const OPCODE: u32 = 110;
        pub const NAME: &'static str = "rfAckChatNormal";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                id: r.u16()?,
                sender: r.string()?,
                message: r.string()?,
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.id);
            w.string(&self.sender);
            w.string(&self.message);
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqChatNormal {
        pub message: String,
    }

    impl ReqChatNormal {
        pub const OPCODE: u32 = 109;
        pub const NAME: &'static str = "rfReqChatNormal";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                message: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.message);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckDummy {
        pub result: i32,
    }

    impl AckDummy {
        pub const OPCODE: u32 = 108;
        pub const NAME: &'static str = "rfAckDummy";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqDummy {
        pub delay_check: i32,
    }

    impl ReqDummy {
        pub const OPCODE: u32 = 107;
        pub const NAME: &'static str = "rfReqDummy";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                delay_check: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.delay_check);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckConfirmItemChanged {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckConfirmItemChanged {
        pub const OPCODE: u32 = 106;
        pub const NAME: &'static str = "rfAckConfirmItemChanged";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqConfirmItemChanged;

    impl ReqConfirmItemChanged {
        pub const OPCODE: u32 = 105;
        pub const NAME: &'static str = "rfReqConfirmItemChanged";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckEquipItem {
        pub changed_new_item_id: i32,
        pub changed_new_unique_id: f64,
        pub changed_new_slot_position: i32,
        pub changed_old_item_id: i32,
        pub changed_old_unique_id: f64,
        pub changed_old_slot_position: i32,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckEquipItem {
        pub const OPCODE: u32 = 104;
        pub const NAME: &'static str = "rfAckEquipItem";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                changed_new_item_id: r.i32()?,
                changed_new_unique_id: r.f64()?,
                changed_new_slot_position: r.i32()?,
                changed_old_item_id: r.i32()?,
                changed_old_unique_id: r.f64()?,
                changed_old_slot_position: r.i32()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.changed_new_item_id);
            w.f64(self.changed_new_unique_id);
            w.i32(self.changed_new_slot_position);
            w.i32(self.changed_old_item_id);
            w.f64(self.changed_old_unique_id);
            w.i32(self.changed_old_slot_position);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqEquipItem {
        pub item_id: i32,
        pub unique_id: f64,
        pub slot_position: i32,
    }

    impl ReqEquipItem {
        pub const OPCODE: u32 = 103;
        pub const NAME: &'static str = "rfReqEquipItem";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                item_id: r.i32()?,
                unique_id: r.f64()?,
                slot_position: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.item_id);
            w.f64(self.unique_id);
            w.i32(self.slot_position);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckItemListEnd;

    impl AckItemListEnd {
        pub const OPCODE: u32 = 102;
        pub const NAME: &'static str = "rfAckItemListEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckItemList {
        pub unique_id: Vec<f64>,
        pub item_id: Vec<i32>,
        pub item_type: Vec<i32>,
        pub durability: Vec<i32>,
        pub until_time: Vec<f64>,
        pub slot_position: Vec<i32>,
        pub add_parts_id: Vec<i32>,
        pub damage_degree: Vec<i32>,
        pub item_state: Vec<u8>,
        pub stack_count: Vec<i32>,
        pub paint_id: Vec<i32>,
        pub trade_count: Vec<i32>,
    }

    impl AckItemList {
        pub const OPCODE: u32 = 101;
        pub const NAME: &'static str = "rfAckItemList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                unique_id: r.array(|r| r.f64())?,
                item_id: r.array(|r| r.i32())?,
                item_type: r.array(|r| r.i32())?,
                durability: r.array(|r| r.i32())?,
                until_time: r.array(|r| r.f64())?,
                slot_position: r.array(|r| r.i32())?,
                add_parts_id: r.array(|r| r.i32())?,
                damage_degree: r.array(|r| r.i32())?,
                item_state: r.array(|r| r.u8())?,
                stack_count: r.array(|r| r.i32())?,
                paint_id: r.array(|r| r.i32())?,
                trade_count: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.unique_id, |w, v| { w.f64(*v); });
            w.array(&self.item_id, |w, v| { w.i32(*v); });
            w.array(&self.item_type, |w, v| { w.i32(*v); });
            w.array(&self.durability, |w, v| { w.i32(*v); });
            w.array(&self.until_time, |w, v| { w.f64(*v); });
            w.array(&self.slot_position, |w, v| { w.i32(*v); });
            w.array(&self.add_parts_id, |w, v| { w.i32(*v); });
            w.array(&self.damage_degree, |w, v| { w.i32(*v); });
            w.array(&self.item_state, |w, v| { w.u8(*v); });
            w.array(&self.stack_count, |w, v| { w.i32(*v); });
            w.array(&self.paint_id, |w, v| { w.i32(*v); });
            w.array(&self.trade_count, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckItemListStart {
        pub count: i32,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckItemListStart {
        pub const OPCODE: u32 = 100;
        pub const NAME: &'static str = "rfAckItemListStart";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                count: r.i32()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.count);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqItemList {
        pub uid: i32,
    }

    impl ReqItemList {
        pub const OPCODE: u32 = 99;
        pub const NAME: &'static str = "rfReqItemList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                uid: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.uid);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckEntrySkillList {
        pub user_i_ds: Vec<i32>,
        pub skill_i_ds: Vec<i32>,
        pub slot_positions: Vec<i32>,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckEntrySkillList {
        pub const OPCODE: u32 = 98;
        pub const NAME: &'static str = "rfAckEntrySkillList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_i_ds: r.array(|r| r.i32())?,
                skill_i_ds: r.array(|r| r.i32())?,
                slot_positions: r.array(|r| r.i32())?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.user_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.skill_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.slot_positions, |w, v| { w.i32(*v); });
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqEntrySkillList {
        pub user_id: i32,
    }

    impl ReqEntrySkillList {
        pub const OPCODE: u32 = 97;
        pub const NAME: &'static str = "rfReqEntrySkillList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckEntryItemList {
        pub user_i_ds: Vec<i32>,
        pub item_i_ds: Vec<i32>,
        pub slot_positions: Vec<i32>,
        pub add_parts_id: Vec<i32>,
        pub unique_id: Vec<f64>,
        pub paint_id: Vec<i32>,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckEntryItemList {
        pub const OPCODE: u32 = 96;
        pub const NAME: &'static str = "rfAckEntryItemList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_i_ds: r.array(|r| r.i32())?,
                item_i_ds: r.array(|r| r.i32())?,
                slot_positions: r.array(|r| r.i32())?,
                add_parts_id: r.array(|r| r.i32())?,
                unique_id: r.array(|r| r.f64())?,
                paint_id: r.array(|r| r.i32())?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.user_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.item_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.slot_positions, |w, v| { w.i32(*v); });
            w.array(&self.add_parts_id, |w, v| { w.i32(*v); });
            w.array(&self.unique_id, |w, v| { w.f64(*v); });
            w.array(&self.paint_id, |w, v| { w.i32(*v); });
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqEntryItemList {
        pub user_id: i32,
    }

    impl ReqEntryItemList {
        pub const OPCODE: u32 = 95;
        pub const NAME: &'static str = "rfReqEntryItemList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChangeUserState {
        pub result: i32,
        pub err_msg: String,
        pub user_i_ds: Vec<i32>,
        pub positions: Vec<u8>,
        pub numbers: Vec<i32>,
        pub old_states: Vec<u8>,
        pub new_states: Vec<u8>,
    }

    impl AckChangeUserState {
        pub const OPCODE: u32 = 94;
        pub const NAME: &'static str = "rfAckChangeUserState";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                user_i_ds: r.array(|r| r.i32())?,
                positions: r.array(|r| r.u8())?,
                numbers: r.array(|r| r.i32())?,
                old_states: r.array(|r| r.u8())?,
                new_states: r.array(|r| r.u8())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.user_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.positions, |w, v| { w.u8(*v); });
            w.array(&self.numbers, |w, v| { w.i32(*v); });
            w.array(&self.old_states, |w, v| { w.u8(*v); });
            w.array(&self.new_states, |w, v| { w.u8(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqChangeUserState {
        pub new_state: u8,
        pub check_playing: u8,
    }

    impl ReqChangeUserState {
        pub const OPCODE: u32 = 93;
        pub const NAME: &'static str = "rfReqChangeUserState";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                new_state: r.u8()?,
                check_playing: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.new_state);
            w.u8(self.check_playing);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckExteriorRoomState;

    impl AckExteriorRoomState {
        pub const OPCODE: u32 = 92;
        pub const NAME: &'static str = "rfAckExteriorRoomState";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqExteriorRoomState {
        pub room_number: i32,
        pub room_state: u8,
    }

    impl ReqExteriorRoomState {
        pub const OPCODE: u32 = 91;
        pub const NAME: &'static str = "rfReqExteriorRoomState";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                room_number: r.i32()?,
                room_state: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.room_number);
            w.u8(self.room_state);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckExteriorRoomUserCount;

    impl AckExteriorRoomUserCount {
        pub const OPCODE: u32 = 90;
        pub const NAME: &'static str = "rfAckExteriorRoomUserCount";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqExteriorRoomUserCount {
        pub room_number: i32,
        pub room_user_count: u16,
    }

    impl ReqExteriorRoomUserCount {
        pub const OPCODE: u32 = 89;
        pub const NAME: &'static str = "rfReqExteriorRoomUserCount";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                room_number: r.i32()?,
                room_user_count: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.room_number);
            w.u16(self.room_user_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckExteriorRoomInfo;

    impl AckExteriorRoomInfo {
        pub const OPCODE: u32 = 88;
        pub const NAME: &'static str = "rfAckExteriorRoomInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqExteriorRoomInfo {
        pub sp: u16,
        pub room_number: i32,
        pub is_pw: u8,
        pub room_name: String,
        pub map_name: String,
        pub mode_name: String,
        pub map_num: i32,
        pub mode_num: i32,
        pub is_team_balance: u8,
        pub max_user_count: u16,
        pub room_user_count: u16,
        pub room_state: u8,
        pub bot_difficult: u8,
        pub weapon_limit: u8,
        pub bot_mode_user_team_num: u8,
    }

    impl ReqExteriorRoomInfo {
        pub const OPCODE: u32 = 87;
        pub const NAME: &'static str = "rfReqExteriorRoomInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                sp: r.u16()?,
                room_number: r.i32()?,
                is_pw: r.u8()?,
                room_name: r.string()?,
                map_name: r.string()?,
                mode_name: r.string()?,
                map_num: r.i32()?,
                mode_num: r.i32()?,
                is_team_balance: r.u8()?,
                max_user_count: r.u16()?,
                room_user_count: r.u16()?,
                room_state: r.u8()?,
                bot_difficult: r.u8()?,
                weapon_limit: r.u8()?,
                bot_mode_user_team_num: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.sp);
            w.i32(self.room_number);
            w.u8(self.is_pw);
            w.string(&self.room_name);
            w.string(&self.map_name);
            w.string(&self.mode_name);
            w.i32(self.map_num);
            w.i32(self.mode_num);
            w.u8(self.is_team_balance);
            w.u16(self.max_user_count);
            w.u16(self.room_user_count);
            w.u8(self.room_state);
            w.u8(self.bot_difficult);
            w.u8(self.weapon_limit);
            w.u8(self.bot_mode_user_team_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGetBootyChoose {
        pub result: i32,
        pub err_msg: String,
        pub give_type: u8,
        pub item_unique_number: f64,
        pub item_id: i32,
        pub part_id: i32,
        pub item_type: u8,
        pub until_time: f64,
        pub stack_count: i32,
        pub paint_id: i32,
    }

    impl AckGetBootyChoose {
        pub const OPCODE: u32 = 86;
        pub const NAME: &'static str = "rfAckGetBootyChoose";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                give_type: r.u8()?,
                item_unique_number: r.f64()?,
                item_id: r.i32()?,
                part_id: r.i32()?,
                item_type: r.u8()?,
                until_time: r.f64()?,
                stack_count: r.i32()?,
                paint_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.u8(self.give_type);
            w.f64(self.item_unique_number);
            w.i32(self.item_id);
            w.i32(self.part_id);
            w.u8(self.item_type);
            w.f64(self.until_time);
            w.i32(self.stack_count);
            w.i32(self.paint_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGetBooty {
        pub result: i32,
        pub err_msg: String,
        pub prize_win_user_id: Vec<i32>,
        pub booty_item_id: Vec<i32>,
        pub booty_parts_id: Vec<i32>,
    }

    impl AckGetBooty {
        pub const OPCODE: u32 = 85;
        pub const NAME: &'static str = "rfAckGetBooty";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                prize_win_user_id: r.array(|r| r.i32())?,
                booty_item_id: r.array(|r| r.i32())?,
                booty_parts_id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.prize_win_user_id, |w, v| { w.i32(*v); });
            w.array(&self.booty_item_id, |w, v| { w.i32(*v); });
            w.array(&self.booty_parts_id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGetBooty {
        pub room_number: u16,
    }

    impl ReqGetBooty {
        pub const OPCODE: u32 = 84;
        pub const NAME: &'static str = "rfReqGetBooty";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                room_number: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.room_number);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckSetUserBootySeedValue {
        pub user_id: i32,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckSetUserBootySeedValue {
        pub const OPCODE: u32 = 83;
        pub const NAME: &'static str = "rfAckSetUserBootySeedValue";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSetUserBootySeedValue {
        pub user_id: i32,
        pub seed_value: i32,
    }

    impl ReqSetUserBootySeedValue {
        pub const OPCODE: u32 = 82;
        pub const NAME: &'static str = "rfReqSetUserBootySeedValue";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
                seed_value: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            w.i32(self.seed_value);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChangeRoomSetting {
        pub result: i32,
        pub err_msg: String,
        pub room_info: GameRoomInfo,
    }

    impl AckChangeRoomSetting {
        pub const OPCODE: u32 = 81;
        pub const NAME: &'static str = "rfAckChangeRoomSetting";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                room_info: GameRoomInfo::decode(r)?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            self.room_info.encode(w);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqChangeRoomSetting {
        pub room_info: GameRoomInfo,
    }

    impl ReqChangeRoomSetting {
        pub const OPCODE: u32 = 80;
        pub const NAME: &'static str = "rfReqChangeRoomSetting";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                room_info: GameRoomInfo::decode(r)?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            self.room_info.encode(w);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyLevelUp {
        pub level_up_user_id: i32,
        pub level: i32,
        pub exp: i32,
        pub item_id: Vec<i32>,
        pub part_id: Vec<i32>,
    }

    impl ReqNotifyLevelUp {
        pub const OPCODE: u32 = 79;
        pub const NAME: &'static str = "rfReqNotifyLevelUp";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                level_up_user_id: r.i32()?,
                level: r.i32()?,
                exp: r.i32()?,
                item_id: r.array(|r| r.i32())?,
                part_id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.level_up_user_id);
            w.i32(self.level);
            w.i32(self.exp);
            w.array(&self.item_id, |w, v| { w.i32(*v); });
            w.array(&self.part_id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckClientUdpInfoList {
        pub result: i32,
        pub err_msg: String,
        pub uid: Vec<i32>,
        pub ip: Vec<String>,
        pub port: Vec<i32>,
    }

    impl AckClientUdpInfoList {
        pub const OPCODE: u32 = 78;
        pub const NAME: &'static str = "rfAckClientUdpInfoList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                uid: r.array(|r| r.i32())?,
                ip: r.array(|r| r.string())?,
                port: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.uid, |w, v| { w.i32(*v); });
            w.array(&self.ip, |w, v| { w.string(v); });
            w.array(&self.port, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqClientUdpInfoList {
        pub game_num: u16,
    }

    impl ReqClientUdpInfoList {
        pub const OPCODE: u32 = 77;
        pub const NAME: &'static str = "rfReqClientUdpInfoList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRegisterOtherUdpInfo {
        pub uid: i32,
        pub ip: String,
        pub port: i32,
        pub nat_type: u8,
        pub intervention: u8,
        pub host_inner_ip: String,
        pub host_inner_port: i32,
    }

    impl AckRegisterOtherUdpInfo {
        pub const OPCODE: u32 = 76;
        pub const NAME: &'static str = "rfAckRegisterOtherUdpInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                uid: r.i32()?,
                ip: r.string()?,
                port: r.i32()?,
                nat_type: r.u8()?,
                intervention: r.u8()?,
                host_inner_ip: r.string()?,
                host_inner_port: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.uid);
            w.string(&self.ip);
            w.i32(self.port);
            w.u8(self.nat_type);
            w.u8(self.intervention);
            w.string(&self.host_inner_ip);
            w.i32(self.host_inner_port);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRegisterMyUdpInfo {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckRegisterMyUdpInfo {
        pub const OPCODE: u32 = 75;
        pub const NAME: &'static str = "rfAckRegisterMyUdpInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRegisterMyUdpInfo {
        pub is_host: u8,
        pub ip: String,
        pub port: i32,
        pub intervention: u8,
        pub host_inner_ip: String,
        pub host_inner_port: i32,
    }

    impl ReqRegisterMyUdpInfo {
        pub const OPCODE: u32 = 74;
        pub const NAME: &'static str = "rfReqRegisterMyUdpInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                is_host: r.u8()?,
                ip: r.string()?,
                port: r.i32()?,
                intervention: r.u8()?,
                host_inner_ip: r.string()?,
                host_inner_port: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.is_host);
            w.string(&self.ip);
            w.i32(self.port);
            w.u8(self.intervention);
            w.string(&self.host_inner_ip);
            w.i32(self.host_inner_port);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckNotifyClientJoinTime {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckNotifyClientJoinTime {
        pub const OPCODE: u32 = 73;
        pub const NAME: &'static str = "rfAckNotifyClientJoinTime";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqNotifyClientJoinTime {
        pub joined_client_id: i32,
    }

    impl ReqNotifyClientJoinTime {
        pub const OPCODE: u32 = 72;
        pub const NAME: &'static str = "rfReqNotifyClientJoinTime";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                joined_client_id: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.joined_client_id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChangeNewGuest {
        pub host_ip: String,
    }

    impl AckChangeNewGuest {
        pub const OPCODE: u32 = 71;
        pub const NAME: &'static str = "rfAckChangeNewGuest";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                host_ip: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.host_ip);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChangeHost {
        pub client_num: u8,
    }

    impl AckChangeHost {
        pub const OPCODE: u32 = 70;
        pub const NAME: &'static str = "rfAckChangeHost";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                client_num: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u8(self.client_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckDisconnectGame {
        pub result: i32,
        pub disconnect_user_id: i32,
        pub disconnect_user_name: String,
    }

    impl AckDisconnectGame {
        pub const OPCODE: u32 = 69;
        pub const NAME: &'static str = "rfAckDisconnectGame";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                disconnect_user_id: r.i32()?,
                disconnect_user_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.disconnect_user_id);
            w.string(&self.disconnect_user_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqDisconnectGame;

    impl ReqDisconnectGame {
        pub const OPCODE: u32 = 68;
        pub const NAME: &'static str = "rfReqDisconnectGame";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckConnectGame {
        pub user_id: i32,
        pub result: i32,
    }

    impl AckConnectGame {
        pub const OPCODE: u32 = 67;
        pub const NAME: &'static str = "rfAckConnectGame";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_id: r.i32()?,
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.user_id);
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqConnectGame;

    impl ReqConnectGame {
        pub const OPCODE: u32 = 66;
        pub const NAME: &'static str = "rfReqConnectGame";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRelayServerOff;

    impl ReqRelayServerOff {
        pub const OPCODE: u32 = 65;
        pub const NAME: &'static str = "rfReqRelayServerOff";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRelayServerOn {
        pub relay_server_ip: String,
        pub relay_server_port_num: i32,
    }

    impl ReqRelayServerOn {
        pub const OPCODE: u32 = 64;
        pub const NAME: &'static str = "rfReqRelayServerOn";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                relay_server_ip: r.string()?,
                relay_server_port_num: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.relay_server_ip);
            w.i32(self.relay_server_port_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGameOver {
        pub uid: i32,
        pub game_room_number: u16,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckGameOver {
        pub const OPCODE: u32 = 63;
        pub const NAME: &'static str = "rfAckGameOver";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                uid: r.i32()?,
                game_room_number: r.u16()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.uid);
            w.u16(self.game_room_number);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGameOver {
        pub uid: i32,
        pub game_room_number: u16,
    }

    impl ReqGameOver {
        pub const OPCODE: u32 = 62;
        pub const NAME: &'static str = "rfReqGameOver";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                uid: r.i32()?,
                game_room_number: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.uid);
            w.u16(self.game_room_number);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckIsGameRoomPlaying {
        pub game_room_num: u16,
        pub is_playing: u8,
    }

    impl AckIsGameRoomPlaying {
        pub const OPCODE: u32 = 61;
        pub const NAME: &'static str = "rfAckIsGameRoomPlaying";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_room_num: r.u16()?,
                is_playing: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_room_num);
            w.u8(self.is_playing);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqIsGameRoomPlaying {
        pub game_room_num: u16,
    }

    impl ReqIsGameRoomPlaying {
        pub const OPCODE: u32 = 60;
        pub const NAME: &'static str = "rfReqIsGameRoomPlaying";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_room_num: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_room_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGameRoomDestroy {
        pub game_room_num: u16,
    }

    impl AckGameRoomDestroy {
        pub const OPCODE: u32 = 59;
        pub const NAME: &'static str = "rfAckGameRoomDestroy";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_room_num: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_room_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChangeRoomMaxNum {
        pub game_num: u16,
        pub number: u16,
        pub team_number: u16,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckChangeRoomMaxNum {
        pub const OPCODE: u32 = 58;
        pub const NAME: &'static str = "rfAckChangeRoomMaxNum";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.u16()?,
                number: r.u16()?,
                team_number: r.u16()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_num);
            w.u16(self.number);
            w.u16(self.team_number);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqChangeRoomMaxNum {
        pub game_num: u16,
        pub uid: i32,
        pub number: u16,
        pub team_num: u16,
    }

    impl ReqChangeRoomMaxNum {
        pub const OPCODE: u32 = 57;
        pub const NAME: &'static str = "rfReqChangeRoomMaxNum";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.u16()?,
                uid: r.i32()?,
                number: r.u16()?,
                team_num: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_num);
            w.i32(self.uid);
            w.u16(self.number);
            w.u16(self.team_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckUpdateScoreResult {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckUpdateScoreResult {
        pub const OPCODE: u32 = 56;
        pub const NAME: &'static str = "rfAckUpdateScoreResult";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckUpdateScore {
        pub user_i_ds: Vec<i32>,
        pub nick_names: Vec<String>,
        pub team_i_ds: Vec<u8>,
        pub scores: Vec<i32>,
        pub kills: Vec<i32>,
        pub assists: Vec<i32>,
        pub deaths: Vec<i32>,
        pub ex_ps: Vec<i32>,
        pub exp_bonuses: Vec<i32>,
        pub points: Vec<i32>,
        pub point_bonuses: Vec<i32>,
        pub team_scores: Vec<i32>,
        pub bonus_owner_id: Vec<i32>,
        pub bonus_id: Vec<i32>,
    }

    impl AckUpdateScore {
        pub const OPCODE: u32 = 55;
        pub const NAME: &'static str = "rfAckUpdateScore";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                user_i_ds: r.array(|r| r.i32())?,
                nick_names: r.array(|r| r.string())?,
                team_i_ds: r.array(|r| r.u8())?,
                scores: r.array(|r| r.i32())?,
                kills: r.array(|r| r.i32())?,
                assists: r.array(|r| r.i32())?,
                deaths: r.array(|r| r.i32())?,
                ex_ps: r.array(|r| r.i32())?,
                exp_bonuses: r.array(|r| r.i32())?,
                points: r.array(|r| r.i32())?,
                point_bonuses: r.array(|r| r.i32())?,
                team_scores: r.array(|r| r.i32())?,
                bonus_owner_id: r.array(|r| r.i32())?,
                bonus_id: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.user_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.nick_names, |w, v| { w.string(v); });
            w.array(&self.team_i_ds, |w, v| { w.u8(*v); });
            w.array(&self.scores, |w, v| { w.i32(*v); });
            w.array(&self.kills, |w, v| { w.i32(*v); });
            w.array(&self.assists, |w, v| { w.i32(*v); });
            w.array(&self.deaths, |w, v| { w.i32(*v); });
            w.array(&self.ex_ps, |w, v| { w.i32(*v); });
            w.array(&self.exp_bonuses, |w, v| { w.i32(*v); });
            w.array(&self.points, |w, v| { w.i32(*v); });
            w.array(&self.point_bonuses, |w, v| { w.i32(*v); });
            w.array(&self.team_scores, |w, v| { w.i32(*v); });
            w.array(&self.bonus_owner_id, |w, v| { w.i32(*v); });
            w.array(&self.bonus_id, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqUpdateScore {
        pub game_number: i32,
        pub user_i_ds: Vec<i32>,
        pub kills: Vec<i32>,
        pub assists: Vec<i32>,
        pub deaths: Vec<i32>,
        pub ex_ps: Vec<i32>,
        pub points: Vec<i32>,
        pub scores: Vec<i32>,
        pub team_scores: Vec<i32>,
        pub round_join: Vec<i32>,
        pub time_join: Vec<i32>,
        pub playtime: i32,
        pub game_end_type: u8,
        pub user_names: Vec<String>,
        pub is_ai_clear: u8,
    }

    impl ReqUpdateScore {
        pub const OPCODE: u32 = 54;
        pub const NAME: &'static str = "rfReqUpdateScore";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_number: r.i32()?,
                user_i_ds: r.array(|r| r.i32())?,
                kills: r.array(|r| r.i32())?,
                assists: r.array(|r| r.i32())?,
                deaths: r.array(|r| r.i32())?,
                ex_ps: r.array(|r| r.i32())?,
                points: r.array(|r| r.i32())?,
                scores: r.array(|r| r.i32())?,
                team_scores: r.array(|r| r.i32())?,
                round_join: r.array(|r| r.i32())?,
                time_join: r.array(|r| r.i32())?,
                playtime: r.i32()?,
                game_end_type: r.u8()?,
                user_names: r.array(|r| r.string())?,
                is_ai_clear: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.game_number);
            w.array(&self.user_i_ds, |w, v| { w.i32(*v); });
            w.array(&self.kills, |w, v| { w.i32(*v); });
            w.array(&self.assists, |w, v| { w.i32(*v); });
            w.array(&self.deaths, |w, v| { w.i32(*v); });
            w.array(&self.ex_ps, |w, v| { w.i32(*v); });
            w.array(&self.points, |w, v| { w.i32(*v); });
            w.array(&self.scores, |w, v| { w.i32(*v); });
            w.array(&self.team_scores, |w, v| { w.i32(*v); });
            w.array(&self.round_join, |w, v| { w.i32(*v); });
            w.array(&self.time_join, |w, v| { w.i32(*v); });
            w.i32(self.playtime);
            w.u8(self.game_end_type);
            w.array(&self.user_names, |w, v| { w.string(v); });
            w.u8(self.is_ai_clear);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckStartGame {
        pub game_number: u16,
        pub clients_count: u16,
        pub start_seed: i32,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckStartGame {
        pub const OPCODE: u32 = 53;
        pub const NAME: &'static str = "rfAckStartGame";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_number: r.u16()?,
                clients_count: r.u16()?,
                start_seed: r.i32()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_number);
            w.u16(self.clients_count);
            w.i32(self.start_seed);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqStartGame {
        pub game_number: u16,
    }

    impl ReqStartGame {
        pub const OPCODE: u32 = 52;
        pub const NAME: &'static str = "rfReqStartGame";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_number: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_number);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGameHostInfo {
        pub game_number: u16,
        pub user_id: String,
        pub ip: String,
        pub port: u16,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckGameHostInfo {
        pub const OPCODE: u32 = 51;
        pub const NAME: &'static str = "rfAckGameHostInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_number: r.u16()?,
                user_id: r.string()?,
                ip: r.string()?,
                port: r.u16()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_number);
            w.string(&self.user_id);
            w.string(&self.ip);
            w.u16(self.port);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGameHostInfo {
        pub game_number: u16,
    }

    impl ReqGameHostInfo {
        pub const OPCODE: u32 = 50;
        pub const NAME: &'static str = "rfReqGameHostInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_number: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_number);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckIsHostAvailable {
        pub result: i32,
    }

    impl AckIsHostAvailable {
        pub const OPCODE: u32 = 49;
        pub const NAME: &'static str = "rfAckIsHostAvailable";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqIsHostAvailable {
        pub my_ip: String,
        pub cpu_grade: u16,
        pub ram_grade: u16,
        pub vga_grade: u16,
        pub vga_ram: u16,
    }

    impl ReqIsHostAvailable {
        pub const OPCODE: u32 = 48;
        pub const NAME: &'static str = "rfReqIsHostAvailable";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                my_ip: r.string()?,
                cpu_grade: r.u16()?,
                ram_grade: r.u16()?,
                vga_grade: r.u16()?,
                vga_ram: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.my_ip);
            w.u16(self.cpu_grade);
            w.u16(self.ram_grade);
            w.u16(self.vga_grade);
            w.u16(self.vga_ram);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChangeTeam {
        pub game_number: u16,
        pub user_id: String,
        pub uid: i32,
        pub team_num: u16,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckChangeTeam {
        pub const OPCODE: u32 = 47;
        pub const NAME: &'static str = "rfAckChangeTeam";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_number: r.u16()?,
                user_id: r.string()?,
                uid: r.i32()?,
                team_num: r.u16()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_number);
            w.string(&self.user_id);
            w.i32(self.uid);
            w.u16(self.team_num);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqChangeTeam {
        pub team_num: u16,
    }

    impl ReqChangeTeam {
        pub const OPCODE: u32 = 46;
        pub const NAME: &'static str = "rfReqChangeTeam";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                team_num: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.team_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChangeRoomOwner {
        pub game_num: u16,
        pub old_owner: i32,
        pub new_owner: i32,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckChangeRoomOwner {
        pub const OPCODE: u32 = 45;
        pub const NAME: &'static str = "rfAckChangeRoomOwner";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.u16()?,
                old_owner: r.i32()?,
                new_owner: r.i32()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_num);
            w.i32(self.old_owner);
            w.i32(self.new_owner);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqChangeRoomOwner {
        pub game_num: u16,
        pub uid: i32,
        pub new_owner: i32,
    }

    impl ReqChangeRoomOwner {
        pub const OPCODE: u32 = 44;
        pub const NAME: &'static str = "rfReqChangeRoomOwner";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.u16()?,
                uid: r.i32()?,
                new_owner: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_num);
            w.i32(self.uid);
            w.i32(self.new_owner);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChangeGameMode {
        pub game_num: u16,
        pub uid: i32,
        pub game_mode: String,
        pub game_mode_num: u8,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckChangeGameMode {
        pub const OPCODE: u32 = 43;
        pub const NAME: &'static str = "rfAckChangeGameMode";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.u16()?,
                uid: r.i32()?,
                game_mode: r.string()?,
                game_mode_num: r.u8()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_num);
            w.i32(self.uid);
            w.string(&self.game_mode);
            w.u8(self.game_mode_num);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqChangeGameMode {
        pub game_num: u16,
        pub uid: i32,
        pub game_mode: String,
        pub game_mode_num: u8,
    }

    impl ReqChangeGameMode {
        pub const OPCODE: u32 = 42;
        pub const NAME: &'static str = "rfReqChangeGameMode";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.u16()?,
                uid: r.i32()?,
                game_mode: r.string()?,
                game_mode_num: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_num);
            w.i32(self.uid);
            w.string(&self.game_mode);
            w.u8(self.game_mode_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckBanUser {
        pub game_num: u16,
        pub uid: i32,
        pub ban_uid: i32,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckBanUser {
        pub const OPCODE: u32 = 41;
        pub const NAME: &'static str = "rfAckBanUser";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.u16()?,
                uid: r.i32()?,
                ban_uid: r.i32()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_num);
            w.i32(self.uid);
            w.i32(self.ban_uid);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqBanUser {
        pub game_num: u16,
        pub uid: i32,
        pub ban_uid: i32,
    }

    impl ReqBanUser {
        pub const OPCODE: u32 = 40;
        pub const NAME: &'static str = "rfReqBanUser";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.u16()?,
                uid: r.i32()?,
                ban_uid: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_num);
            w.i32(self.uid);
            w.i32(self.ban_uid);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckSelectMap {
        pub game_num: u16,
        pub uid: i32,
        pub map_name: String,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckSelectMap {
        pub const OPCODE: u32 = 39;
        pub const NAME: &'static str = "rfAckSelectMap";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.u16()?,
                uid: r.i32()?,
                map_name: r.string()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_num);
            w.i32(self.uid);
            w.string(&self.map_name);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqSelectMap {
        pub game_num: u16,
        pub uid: i32,
        pub map_name: String,
    }

    impl ReqSelectMap {
        pub const OPCODE: u32 = 38;
        pub const NAME: &'static str = "rfReqSelectMap";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.u16()?,
                uid: r.i32()?,
                map_name: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_num);
            w.i32(self.uid);
            w.string(&self.map_name);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckGameRoomInfo {
        pub result: i32,
        pub err_msg: String,
        pub room_info: GameRoomInfo,
    }

    impl AckGameRoomInfo {
        pub const OPCODE: u32 = 37;
        pub const NAME: &'static str = "rfAckGameRoomInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                room_info: GameRoomInfo::decode(r)?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            self.room_info.encode(w);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqGameRoomInfo {
        pub game_num: u16,
        pub uid: i32,
    }

    impl ReqGameRoomInfo {
        pub const OPCODE: u32 = 36;
        pub const NAME: &'static str = "rfReqGameRoomInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.u16()?,
                uid: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_num);
            w.i32(self.uid);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckCreateGameRoom {
        pub result: i32,
        pub err_msg: String,
        pub user_id: i32,
        pub room_owner_ip: String,
        pub net_room_info: GameRoomInfo,
    }

    impl AckCreateGameRoom {
        pub const OPCODE: u32 = 35;
        pub const NAME: &'static str = "rfAckCreateGameRoom";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                user_id: r.i32()?,
                room_owner_ip: r.string()?,
                net_room_info: GameRoomInfo::decode(r)?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.user_id);
            w.string(&self.room_owner_ip);
            self.net_room_info.encode(w);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqCreateGameRoom {
        pub net_room_info: GameRoomInfo,
    }

    impl ReqCreateGameRoom {
        pub const OPCODE: u32 = 34;
        pub const NAME: &'static str = "rfReqCreateGameRoom";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                net_room_info: GameRoomInfo::decode(r)?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            self.net_room_info.encode(w);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckExceedMaxUser;

    impl AckExceedMaxUser {
        pub const OPCODE: u32 = 33;
        pub const NAME: &'static str = "rfAckExceedMaxUser";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLeaveRoom {
        pub num: u16,
        pub uid: i32,
        pub user_id: String,
        pub result: i32,
        pub err_msg: String,
    }

    impl AckLeaveRoom {
        pub const OPCODE: u32 = 32;
        pub const NAME: &'static str = "rfAckLeaveRoom";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                num: r.u16()?,
                uid: r.i32()?,
                user_id: r.string()?,
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.num);
            w.i32(self.uid);
            w.string(&self.user_id);
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLeaveRoom {
        pub num: u16,
    }

    impl ReqLeaveRoom {
        pub const OPCODE: u32 = 31;
        pub const NAME: &'static str = "rfReqLeaveRoom";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                num: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRoomUserList {
        pub uid: Vec<i32>,
        pub user_name: Vec<String>,
        pub grade: Vec<i32>,
        pub team_num: Vec<i32>,
        pub statue: Vec<u8>,
        pub is_host: Vec<u8>,
        pub user_ip: Vec<String>,
        pub clan_name: Vec<String>,
        pub cm_pattern: Vec<i32>,
        pub cm_bg: Vec<i32>,
        pub cm_bl: Vec<i32>,
        pub is_looking_for: Vec<u8>,
        pub gmuid: Vec<i32>,
        pub clan_level: Vec<i32>,
        pub level_mark_id: Vec<i32>,
        pub title_mark_id: Vec<i32>,
        pub reserved1: Vec<i32>,
        pub reserved2: Vec<i32>,
        pub reserved3: Vec<i32>,
    }

    impl AckRoomUserList {
        pub const OPCODE: u32 = 30;
        pub const NAME: &'static str = "rfAckRoomUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                uid: r.array(|r| r.i32())?,
                user_name: r.array(|r| r.string())?,
                grade: r.array(|r| r.i32())?,
                team_num: r.array(|r| r.i32())?,
                statue: r.array(|r| r.u8())?,
                is_host: r.array(|r| r.u8())?,
                user_ip: r.array(|r| r.string())?,
                clan_name: r.array(|r| r.string())?,
                cm_pattern: r.array(|r| r.i32())?,
                cm_bg: r.array(|r| r.i32())?,
                cm_bl: r.array(|r| r.i32())?,
                is_looking_for: r.array(|r| r.u8())?,
                gmuid: r.array(|r| r.i32())?,
                clan_level: r.array(|r| r.i32())?,
                level_mark_id: r.array(|r| r.i32())?,
                title_mark_id: r.array(|r| r.i32())?,
                reserved1: r.array(|r| r.i32())?,
                reserved2: r.array(|r| r.i32())?,
                reserved3: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.uid, |w, v| { w.i32(*v); });
            w.array(&self.user_name, |w, v| { w.string(v); });
            w.array(&self.grade, |w, v| { w.i32(*v); });
            w.array(&self.team_num, |w, v| { w.i32(*v); });
            w.array(&self.statue, |w, v| { w.u8(*v); });
            w.array(&self.is_host, |w, v| { w.u8(*v); });
            w.array(&self.user_ip, |w, v| { w.string(v); });
            w.array(&self.clan_name, |w, v| { w.string(v); });
            w.array(&self.cm_pattern, |w, v| { w.i32(*v); });
            w.array(&self.cm_bg, |w, v| { w.i32(*v); });
            w.array(&self.cm_bl, |w, v| { w.i32(*v); });
            w.array(&self.is_looking_for, |w, v| { w.u8(*v); });
            w.array(&self.gmuid, |w, v| { w.i32(*v); });
            w.array(&self.clan_level, |w, v| { w.i32(*v); });
            w.array(&self.level_mark_id, |w, v| { w.i32(*v); });
            w.array(&self.title_mark_id, |w, v| { w.i32(*v); });
            w.array(&self.reserved1, |w, v| { w.i32(*v); });
            w.array(&self.reserved2, |w, v| { w.i32(*v); });
            w.array(&self.reserved3, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRoomUserList {
        pub game_num: u16,
    }

    impl ReqRoomUserList {
        pub const OPCODE: u32 = 29;
        pub const NAME: &'static str = "rfReqRoomUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_num);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckEnterRoom {
        pub game_num: u16,
        pub owner_id: i32,
        pub uid: i32,
        pub user_id: String,
        pub team_num: u16,
        pub grade: u16,
        pub user_ip: String,
        pub clan_name: String,
        pub clan_mark_pattern: i32,
        pub clan_mark_bg: i32,
        pub clan_mark_bl: i32,
        pub is_look_for: u8,
        pub result: i32,
        pub err_msg: String,
        pub clan_level: i32,
        pub level_mark_id: i32,
        pub title_mark_id: i32,
        pub reserved1: i32,
        pub reserved2: i32,
        pub reserved3: i32,
    }

    impl AckEnterRoom {
        pub const OPCODE: u32 = 28;
        pub const NAME: &'static str = "rfAckEnterRoom";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.u16()?,
                owner_id: r.i32()?,
                uid: r.i32()?,
                user_id: r.string()?,
                team_num: r.u16()?,
                grade: r.u16()?,
                user_ip: r.string()?,
                clan_name: r.string()?,
                clan_mark_pattern: r.i32()?,
                clan_mark_bg: r.i32()?,
                clan_mark_bl: r.i32()?,
                is_look_for: r.u8()?,
                result: r.i32()?,
                err_msg: r.string()?,
                clan_level: r.i32()?,
                level_mark_id: r.i32()?,
                title_mark_id: r.i32()?,
                reserved1: r.i32()?,
                reserved2: r.i32()?,
                reserved3: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_num);
            w.i32(self.owner_id);
            w.i32(self.uid);
            w.string(&self.user_id);
            w.u16(self.team_num);
            w.u16(self.grade);
            w.string(&self.user_ip);
            w.string(&self.clan_name);
            w.i32(self.clan_mark_pattern);
            w.i32(self.clan_mark_bg);
            w.i32(self.clan_mark_bl);
            w.u8(self.is_look_for);
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.clan_level);
            w.i32(self.level_mark_id);
            w.i32(self.title_mark_id);
            w.i32(self.reserved1);
            w.i32(self.reserved2);
            w.i32(self.reserved3);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqEnterRoom {
        pub game_num: u16,
        pub password: String,
        pub room_enter_type: u8,
    }

    impl ReqEnterRoom {
        pub const OPCODE: u32 = 27;
        pub const NAME: &'static str = "rfReqEnterRoom";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.u16()?,
                password: r.string()?,
                room_enter_type: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.game_num);
            w.string(&self.password);
            w.u8(self.room_enter_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRoomListEnd;

    impl AckRoomListEnd {
        pub const OPCODE: u32 = 26;
        pub const NAME: &'static str = "rfAckRoomListEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRoomList {
        pub game_num: Vec<i32>,
        pub title: Vec<String>,
        pub map_name: Vec<String>,
        pub map_num: Vec<i32>,
        pub user_count: Vec<u8>,
        pub max_count: Vec<u8>,
        pub mode_name: Vec<String>,
        pub mode_num: Vec<i32>,
        pub weapon_limit: Vec<u8>,
        pub is_team_balance: Vec<u8>,
        pub status: Vec<u8>,
        pub room_owner_ip: Vec<String>,
        pub game_minute: Vec<i32>,
        pub room_owner_name: Vec<String>,
        pub is_special: Vec<u8>,
        pub is_password: Vec<u8>,
        pub bot_difficult: Vec<u8>,
        pub bot_mode_user_team_num: Vec<u8>,
    }

    impl AckRoomList {
        pub const OPCODE: u32 = 25;
        pub const NAME: &'static str = "rfAckRoomList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                game_num: r.array(|r| r.i32())?,
                title: r.array(|r| r.string())?,
                map_name: r.array(|r| r.string())?,
                map_num: r.array(|r| r.i32())?,
                user_count: r.array(|r| r.u8())?,
                max_count: r.array(|r| r.u8())?,
                mode_name: r.array(|r| r.string())?,
                mode_num: r.array(|r| r.i32())?,
                weapon_limit: r.array(|r| r.u8())?,
                is_team_balance: r.array(|r| r.u8())?,
                status: r.array(|r| r.u8())?,
                room_owner_ip: r.array(|r| r.string())?,
                game_minute: r.array(|r| r.i32())?,
                room_owner_name: r.array(|r| r.string())?,
                is_special: r.array(|r| r.u8())?,
                is_password: r.array(|r| r.u8())?,
                bot_difficult: r.array(|r| r.u8())?,
                bot_mode_user_team_num: r.array(|r| r.u8())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.game_num, |w, v| { w.i32(*v); });
            w.array(&self.title, |w, v| { w.string(v); });
            w.array(&self.map_name, |w, v| { w.string(v); });
            w.array(&self.map_num, |w, v| { w.i32(*v); });
            w.array(&self.user_count, |w, v| { w.u8(*v); });
            w.array(&self.max_count, |w, v| { w.u8(*v); });
            w.array(&self.mode_name, |w, v| { w.string(v); });
            w.array(&self.mode_num, |w, v| { w.i32(*v); });
            w.array(&self.weapon_limit, |w, v| { w.u8(*v); });
            w.array(&self.is_team_balance, |w, v| { w.u8(*v); });
            w.array(&self.status, |w, v| { w.u8(*v); });
            w.array(&self.room_owner_ip, |w, v| { w.string(v); });
            w.array(&self.game_minute, |w, v| { w.i32(*v); });
            w.array(&self.room_owner_name, |w, v| { w.string(v); });
            w.array(&self.is_special, |w, v| { w.u8(*v); });
            w.array(&self.is_password, |w, v| { w.u8(*v); });
            w.array(&self.bot_difficult, |w, v| { w.u8(*v); });
            w.array(&self.bot_mode_user_team_num, |w, v| { w.u8(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckRoomListStart {
        pub result: i32,
        pub total_count: i32,
        pub count: i32,
        pub start_index: i32,
    }

    impl AckRoomListStart {
        pub const OPCODE: u32 = 24;
        pub const NAME: &'static str = "rfAckRoomListStart";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                total_count: r.i32()?,
                count: r.i32()?,
                start_index: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.total_count);
            w.i32(self.count);
            w.i32(self.start_index);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqRoomList {
        pub id: u16,
        pub start_index: i32,
        pub count: u16,
    }

    impl ReqRoomList {
        pub const OPCODE: u32 = 23;
        pub const NAME: &'static str = "rfReqRoomList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                id: r.u16()?,
                start_index: r.i32()?,
                count: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.id);
            w.i32(self.start_index);
            w.u16(self.count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChannelUserList {
        pub total_count: i32,
        pub count: i32,
        pub start_index: i32,
        pub uid: Vec<i32>,
        pub user_id: Vec<String>,
        pub user_levels: Vec<i32>,
        pub clan_name: Vec<String>,
        pub clan_pattern: Vec<i32>,
        pub clan_bg: Vec<i32>,
        pub clan_bl: Vec<i32>,
        pub clan_grade: Vec<i32>,
        pub clan_win: Vec<i32>,
        pub clan_lose: Vec<i32>,
        pub is_look_for_clan: Vec<u8>,
        pub result: i32,
        pub err_msg: String,
        pub clan_level: Vec<i32>,
        pub level_mark_id: Vec<i32>,
        pub title_mark_id: Vec<i32>,
        pub reserved1: Vec<i32>,
        pub reserved2: Vec<i32>,
        pub reserved3: Vec<i32>,
    }

    impl AckChannelUserList {
        pub const OPCODE: u32 = 22;
        pub const NAME: &'static str = "rfAckChannelUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                total_count: r.i32()?,
                count: r.i32()?,
                start_index: r.i32()?,
                uid: r.array(|r| r.i32())?,
                user_id: r.array(|r| r.string())?,
                user_levels: r.array(|r| r.i32())?,
                clan_name: r.array(|r| r.string())?,
                clan_pattern: r.array(|r| r.i32())?,
                clan_bg: r.array(|r| r.i32())?,
                clan_bl: r.array(|r| r.i32())?,
                clan_grade: r.array(|r| r.i32())?,
                clan_win: r.array(|r| r.i32())?,
                clan_lose: r.array(|r| r.i32())?,
                is_look_for_clan: r.array(|r| r.u8())?,
                result: r.i32()?,
                err_msg: r.string()?,
                clan_level: r.array(|r| r.i32())?,
                level_mark_id: r.array(|r| r.i32())?,
                title_mark_id: r.array(|r| r.i32())?,
                reserved1: r.array(|r| r.i32())?,
                reserved2: r.array(|r| r.i32())?,
                reserved3: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.total_count);
            w.i32(self.count);
            w.i32(self.start_index);
            w.array(&self.uid, |w, v| { w.i32(*v); });
            w.array(&self.user_id, |w, v| { w.string(v); });
            w.array(&self.user_levels, |w, v| { w.i32(*v); });
            w.array(&self.clan_name, |w, v| { w.string(v); });
            w.array(&self.clan_pattern, |w, v| { w.i32(*v); });
            w.array(&self.clan_bg, |w, v| { w.i32(*v); });
            w.array(&self.clan_bl, |w, v| { w.i32(*v); });
            w.array(&self.clan_grade, |w, v| { w.i32(*v); });
            w.array(&self.clan_win, |w, v| { w.i32(*v); });
            w.array(&self.clan_lose, |w, v| { w.i32(*v); });
            w.array(&self.is_look_for_clan, |w, v| { w.u8(*v); });
            w.i32(self.result);
            w.string(&self.err_msg);
            w.array(&self.clan_level, |w, v| { w.i32(*v); });
            w.array(&self.level_mark_id, |w, v| { w.i32(*v); });
            w.array(&self.title_mark_id, |w, v| { w.i32(*v); });
            w.array(&self.reserved1, |w, v| { w.i32(*v); });
            w.array(&self.reserved2, |w, v| { w.i32(*v); });
            w.array(&self.reserved3, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqChannelUserList {
        pub id: u16,
        pub start_index: u16,
        pub count: u16,
    }

    impl ReqChannelUserList {
        pub const OPCODE: u32 = 21;
        pub const NAME: &'static str = "rfReqChannelUserList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                id: r.u16()?,
                start_index: r.u16()?,
                count: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.id);
            w.u16(self.start_index);
            w.u16(self.count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLeaveChannel {
        pub result: i32,
        pub user_id: String,
        pub uid: i32,
    }

    impl AckLeaveChannel {
        pub const OPCODE: u32 = 20;
        pub const NAME: &'static str = "rfAckLeaveChannel";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                user_id: r.string()?,
                uid: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.user_id);
            w.i32(self.uid);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLeaveChannel {
        pub id: u16,
    }

    impl ReqLeaveChannel {
        pub const OPCODE: u32 = 19;
        pub const NAME: &'static str = "rfReqLeaveChannel";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                id: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckEnterChannel {
        pub id: u16,
        pub uid: i32,
        pub user_id: String,
        pub level: i32,
        pub clan_name: String,
        pub clan_mark_pattern: i32,
        pub clan_mark_bg: i32,
        pub clan_mark_bl: i32,
        pub clan_grade: i32,
        pub clan_win: i32,
        pub clan_lose: i32,
        pub is_look_for: u8,
        pub result: i32,
        pub err_msg: String,
        pub clan_level: i32,
        pub level_mark_id: i32,
        pub title_mark_id: i32,
        pub reserved1: i32,
        pub reserved2: i32,
        pub reserved3: i32,
    }

    impl AckEnterChannel {
        pub const OPCODE: u32 = 18;
        pub const NAME: &'static str = "rfAckEnterChannel";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                id: r.u16()?,
                uid: r.i32()?,
                user_id: r.string()?,
                level: r.i32()?,
                clan_name: r.string()?,
                clan_mark_pattern: r.i32()?,
                clan_mark_bg: r.i32()?,
                clan_mark_bl: r.i32()?,
                clan_grade: r.i32()?,
                clan_win: r.i32()?,
                clan_lose: r.i32()?,
                is_look_for: r.u8()?,
                result: r.i32()?,
                err_msg: r.string()?,
                clan_level: r.i32()?,
                level_mark_id: r.i32()?,
                title_mark_id: r.i32()?,
                reserved1: r.i32()?,
                reserved2: r.i32()?,
                reserved3: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.id);
            w.i32(self.uid);
            w.string(&self.user_id);
            w.i32(self.level);
            w.string(&self.clan_name);
            w.i32(self.clan_mark_pattern);
            w.i32(self.clan_mark_bg);
            w.i32(self.clan_mark_bl);
            w.i32(self.clan_grade);
            w.i32(self.clan_win);
            w.i32(self.clan_lose);
            w.u8(self.is_look_for);
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.clan_level);
            w.i32(self.level_mark_id);
            w.i32(self.title_mark_id);
            w.i32(self.reserved1);
            w.i32(self.reserved2);
            w.i32(self.reserved3);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqEnterChannel {
        pub id: u16,
    }

    impl ReqEnterChannel {
        pub const OPCODE: u32 = 17;
        pub const NAME: &'static str = "rfReqEnterChannel";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                id: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckChannelList {
        pub id: Vec<i32>,
        pub title: Vec<String>,
        pub cur_user: Vec<i32>,
        pub max_user: Vec<i32>,
    }

    impl AckChannelList {
        pub const OPCODE: u32 = 16;
        pub const NAME: &'static str = "rfAckChannelList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                id: r.array(|r| r.i32())?,
                title: r.array(|r| r.string())?,
                cur_user: r.array(|r| r.i32())?,
                max_user: r.array(|r| r.i32())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.id, |w, v| { w.i32(*v); });
            w.array(&self.title, |w, v| { w.string(v); });
            w.array(&self.cur_user, |w, v| { w.i32(*v); });
            w.array(&self.max_user, |w, v| { w.i32(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqChannelList {
        pub b_channel_type: u16,
    }

    impl ReqChannelList {
        pub const OPCODE: u32 = 15;
        pub const NAME: &'static str = "rfReqChannelList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                b_channel_type: r.u16()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.u16(self.b_channel_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckFriendListEnd;

    impl AckFriendListEnd {
        pub const OPCODE: u32 = 14;
        pub const NAME: &'static str = "rfAckFriendListEnd";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckFriendList {
        pub friend_name: Vec<String>,
        pub is_log_on: Vec<u8>,
        pub clan_name: Vec<String>,
        pub cm_pattern: Vec<i32>,
        pub cm_bg: Vec<i32>,
        pub cm_bl: Vec<i32>,
        pub level_mark_id: Vec<i32>,
        pub clan_level: Vec<i32>,
        pub level: Vec<i32>,
        pub is_look_for_clan: Vec<u8>,
    }

    impl AckFriendList {
        pub const OPCODE: u32 = 13;
        pub const NAME: &'static str = "rfAckFriendList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                friend_name: r.array(|r| r.string())?,
                is_log_on: r.array(|r| r.u8())?,
                clan_name: r.array(|r| r.string())?,
                cm_pattern: r.array(|r| r.i32())?,
                cm_bg: r.array(|r| r.i32())?,
                cm_bl: r.array(|r| r.i32())?,
                level_mark_id: r.array(|r| r.i32())?,
                clan_level: r.array(|r| r.i32())?,
                level: r.array(|r| r.i32())?,
                is_look_for_clan: r.array(|r| r.u8())?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.array(&self.friend_name, |w, v| { w.string(v); });
            w.array(&self.is_log_on, |w, v| { w.u8(*v); });
            w.array(&self.clan_name, |w, v| { w.string(v); });
            w.array(&self.cm_pattern, |w, v| { w.i32(*v); });
            w.array(&self.cm_bg, |w, v| { w.i32(*v); });
            w.array(&self.cm_bl, |w, v| { w.i32(*v); });
            w.array(&self.level_mark_id, |w, v| { w.i32(*v); });
            w.array(&self.clan_level, |w, v| { w.i32(*v); });
            w.array(&self.level, |w, v| { w.i32(*v); });
            w.array(&self.is_look_for_clan, |w, v| { w.u8(*v); });
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckFriendListStart {
        pub result: i32,
        pub total_count: i32,
    }

    impl AckFriendListStart {
        pub const OPCODE: u32 = 12;
        pub const NAME: &'static str = "rfAckFriendListStart";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                total_count: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.i32(self.total_count);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqFriendList;

    impl ReqFriendList {
        pub const OPCODE: u32 = 11;
        pub const NAME: &'static str = "rfReqFriendList";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckCharInfo {
        pub result: i32,
        pub err_msg: String,
        pub user_id: i32,
        pub char_name: String,
        pub clan_name: String,
        pub cm_pattern: i32,
        pub cm_bg: i32,
        pub cm_bl: i32,
        pub st_char_info: CharInfo,
        pub in_clan_rank: u8,
        pub clan_level: i32,
    }

    impl AckCharInfo {
        pub const OPCODE: u32 = 10;
        pub const NAME: &'static str = "rfAckCharInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                user_id: r.i32()?,
                char_name: r.string()?,
                clan_name: r.string()?,
                cm_pattern: r.i32()?,
                cm_bg: r.i32()?,
                cm_bl: r.i32()?,
                st_char_info: CharInfo::decode(r)?,
                in_clan_rank: r.u8()?,
                clan_level: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.user_id);
            w.string(&self.char_name);
            w.string(&self.clan_name);
            w.i32(self.cm_pattern);
            w.i32(self.cm_bg);
            w.i32(self.cm_bl);
            self.st_char_info.encode(w);
            w.u8(self.in_clan_rank);
            w.i32(self.clan_level);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqCharInfo;

    impl ReqCharInfo {
        pub const OPCODE: u32 = 9;
        pub const NAME: &'static str = "rfReqCharInfo";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLogin2 {
        pub result: i32,
        pub err_msg: String,
        pub user_id: i32,
        pub char_name: String,
        pub cur_server_time: f64,
        pub char_dbid: i32,
        pub is_gm: u8,
    }

    impl AckLogin2 {
        pub const OPCODE: u32 = 8;
        pub const NAME: &'static str = "rfAckLogin2";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                user_id: r.i32()?,
                char_name: r.string()?,
                cur_server_time: r.f64()?,
                char_dbid: r.i32()?,
                is_gm: r.u8()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.user_id);
            w.string(&self.char_name);
            w.f64(self.cur_server_time);
            w.i32(self.char_dbid);
            w.u8(self.is_gm);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLogin2 {
        pub char_name: String,
        pub time_stamp: i32,
        pub login_hash1: i32,
        pub login_hash2: i32,
        pub login_hash3: i32,
        pub login_hash4: i32,
        pub login_hash5: i32,
        pub guid: String,
        pub user_type: i32,
    }

    impl ReqLogin2 {
        pub const OPCODE: u32 = 7;
        pub const NAME: &'static str = "rfReqLogin2";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                char_name: r.string()?,
                time_stamp: r.i32()?,
                login_hash1: r.i32()?,
                login_hash2: r.i32()?,
                login_hash3: r.i32()?,
                login_hash4: r.i32()?,
                login_hash5: r.i32()?,
                guid: r.string()?,
                user_type: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.char_name);
            w.i32(self.time_stamp);
            w.i32(self.login_hash1);
            w.i32(self.login_hash2);
            w.i32(self.login_hash3);
            w.i32(self.login_hash4);
            w.i32(self.login_hash5);
            w.string(&self.guid);
            w.i32(self.user_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckStartNatDiag {
        pub nat_type: i32,
    }

    impl AckStartNatDiag {
        pub const OPCODE: u32 = 6;
        pub const NAME: &'static str = "rfAckStartNatDiag";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                nat_type: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.nat_type);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqStartNatDiag {
        pub server1addr: String,
        pub server1port: i32,
        pub server2addr: String,
        pub server2port: i32,
    }

    impl ReqStartNatDiag {
        pub const OPCODE: u32 = 5;
        pub const NAME: &'static str = "rfReqStartNatDiag";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                server1addr: r.string()?,
                server1port: r.i32()?,
                server2addr: r.string()?,
                server2port: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.server1addr);
            w.i32(self.server1port);
            w.string(&self.server2addr);
            w.i32(self.server2port);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqForceLogoutByOwner;

    impl ReqForceLogoutByOwner {
        pub const OPCODE: u32 = 4;
        pub const NAME: &'static str = "rfReqForceLogoutByOwner";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self;
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckLogin {
        pub result: i32,
        pub err_msg: String,
        pub user_id: i32,
        pub char_name: String,
        pub clan_name: String,
        pub bt_char_info: CharInfo,
    }

    impl AckLogin {
        pub const OPCODE: u32 = 3;
        pub const NAME: &'static str = "rfAckLogin";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
                user_id: r.i32()?,
                char_name: r.string()?,
                clan_name: r.string()?,
                bt_char_info: CharInfo::decode(r)?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            w.i32(self.user_id);
            w.string(&self.char_name);
            w.string(&self.clan_name);
            self.bt_char_info.encode(w);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqLogin {
        pub id: String,
    }

    impl ReqLogin {
        pub const OPCODE: u32 = 2;
        pub const NAME: &'static str = "rfReqLogin";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                id: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.string(&self.id);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct AckHash {
        pub result: i32,
        pub err_msg: String,
    }

    impl AckHash {
        pub const OPCODE: u32 = 1;
        pub const NAME: &'static str = "rfAckHash";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                result: r.i32()?,
                err_msg: r.string()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.result);
            w.string(&self.err_msg);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct ReqHash {
        pub hash_field0: i32,
        pub hash_field1: i32,
        pub hash_field2: i32,
        pub hash_field3: i32,
    }

    impl ReqHash {
        pub const OPCODE: u32 = 0;
        pub const NAME: &'static str = "rfReqHash";

        pub fn decode(buf: &[u8]) -> Result<Self> {
            let mut reader = Reader::new(buf);
            let r = &mut reader;
            let v = Self {
                hash_field0: r.i32()?,
                hash_field1: r.i32()?,
                hash_field2: r.i32()?,
                hash_field3: r.i32()?,
            };
            if r.remaining() != 0 {
                crate::tracing_warn_tail(Self::NAME, r.remaining());
            }
            Ok(v)
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut writer = Writer::new();
            let w = &mut writer;
            w.i32(self.hash_field0);
            w.i32(self.hash_field1);
            w.i32(self.hash_field2);
            w.i32(self.hash_field3);
            let _ = &w;
            writer.into_inner()
        }

        pub fn packet(&self) -> crate::frame::Packet {
            crate::frame::Packet::new(Self::OPCODE, self.encode())
        }
    }

}

