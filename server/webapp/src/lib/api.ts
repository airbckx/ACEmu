// Typed client for the ACEmu admin REST API (Rust axum service, port 55990).

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export interface Summary {
  players: number;
  accounts: number;
  items: number;
  catalog: number;
  shop: number;
  matches: number;
}

// Match history ---------------------------------------------------------------

/** winner 0/1 = team (AF/RSA), -1 = draw or undecided; played_at is UTC "YYYY-MM-DD HH:MM:SS". */
export interface MatchSummary {
  id: number;
  played_at: string;
  room_name: string;
  map: string;
  mode: string;
  host: string;
  /** Seconds. */
  duration: number;
  end_type: number;
  team0_score: number;
  team1_score: number;
  winner: number;
  /** Human participants. */
  players: number;
  bots: number;
  names: string[];
}

/** won 1 = won, 0 = lost, -1 = draw / undecided. */
export interface MatchPlayer {
  uid: number;
  name: string;
  team: number;
  is_bot: boolean;
  score: number;
  kills: number;
  assists: number;
  deaths: number;
  exp: number;
  points: number;
  headshots: number;
  won: number;
}

export interface MatchKill {
  seq: number;
  at_secs: number;
  killer_uid: number;
  killer_name: string;
  victim_uid: number;
  victim_name: string;
  weapon: number;
  weapon_name: string | null;
  body_part: number;
  headshot: boolean;
}

/** Where a player stood and what they held when an objective event happened. */
export interface MatchEventActor {
  uid: number;
  name: string;
  team: number;
  x: number;
  y: number;
  z: number;
  /** 0 standing, 1 crouched, 2 prone. */
  posture: number;
  weapon: number;
  weapon_part: number;
  range: number;
}

export type MatchEventKind =
  | 'round_end'
  | 'bomb_planted'
  | 'bomb_defused'
  | 'point_taken'
  | 'dom_score'
  | 'chopper_kill'
  | 'udp_kick'
  | 'host_lost';

/**
 * One objective event the host reported during the match. `extra` depends on
 * `kind`: round_end {round, end_type, round_secs, winner, winner_ad, start_players,
 * end_players, af, rsa}; bomb_planted / bomb_defused: MatchEventActor + {site};
 * point_taken {flag, takers: MatchEventActor[]}; dom_score {af, rsa};
 * chopper_kill: MatchEventActor + {damage, assists: [{uid, name}], caller: {uid, name, x, y, z}}.
 */
export interface MatchEvent {
  id: number;
  at_secs: number;
  kind: MatchEventKind | string;
  actor_uid: number;
  actor_name: string;
  /** 0 AF, 1 RSA, -1 none. */
  team: number;
  extra: Record<string, unknown> | null;
}

export interface MatchDetail extends MatchSummary {
  scoreboard: MatchPlayer[];
  kill_feed: MatchKill[];
  events: MatchEvent[];
}

export interface MatchPage {
  total: number;
  matches: MatchSummary[];
}

export interface LeaderRow {
  name: string;
  level: number;
  exp: number;
  kills: number;
  deaths: number;
  assists: number;
  headshots: number;
  wins: number;
  losses: number;
  play_time: number;
  matches: number;
  best_kills: number;
  last_played: string | null;
}

export interface Account {
  id: number;
  username: string;
  has_password: boolean;
  character: string | null;
  created_at: string;
}

export interface PlayerSummary {
  name: string;
  level: number;
  exp: number;
  points: number;
  cash: number;
  kills: number;
  deaths: number;
  wins: number;
  losses: number;
  play_time: number;
  item_count: number;
  account: string | null;
}

export interface Stats {
  level: number;
  exp: number;
  kills: number;
  deaths: number;
  assists: number;
  headshots: number;
  team_kills: number;
  wins: number;
  losses: number;
  play_time: number;
}

/** slot -1 = in bag, >= 0 equipped; until 0 = permanent, else an OLE date. */
export interface Item {
  unique_id: number;
  item_id: number;
  name: string;
  item_type: number;
  slot: number;
  stack: number;
  durability: number;
  add_part_id: number;
  paint_id: number;
  until: number;
}

export interface Skill {
  unique_id: number;
  skill_id: number;
  name: string;
  slot: number;
}

/** main/pistol/throw1/throw2 are item unique_ids, 0 = empty. */
export interface QSlot {
  id: number;
  name: string;
  main: number;
  pistol: number;
  throw1: number;
  throw2: number;
  skills: number[];
}

export interface Player {
  name: string;
  points: number;
  cash: number;
  next_unique: number;
  stats: Stats;
  items: Item[];
  skills: Skill[];
  qslots: QSlot[];
  account: string | null;
}

export type Availability = 'shop' | 'lootbox' | 'hidden';

/** hours 0 = permanent; each row is one purchase option the client shows. */
export interface Price {
  idx: number;
  hours: number;
  points: number;
  cash: number;
}

export interface CatalogItem {
  id: number;
  name: string;
  item_type: number;
  kind: number;
  /** The client's shop category, e.g. "Store", "Cash Item", "Superior", "Rare", "Legendary". */
  category: string;
  sell_price: number;
  availability: Availability;
  shop_order: number;
  in_shop_table: boolean;
  prices: Price[];
}

export interface CatalogPage {
  total: number;
  items: CatalogItem[];
}

export interface Kind {
  kind: number;
  label: string;
}

export interface OkResponse {
  ok: true;
}

// Loot boxes ------------------------------------------------------------------

export type Reel = 'main' | 'bonus';

export interface BoxSummary {
  id: number;
  name: string;
  main_entries: number;
  bonus_entries: number;
}

/**
 * reel 'main' = the item reel (what the player wins), 'bonus' = the second reel
 * (vouchers / tickets awarded alongside). weight is relative (>= 0, 0 = never drops);
 * percent is computed server-side as weight / sum(weights in reel) * 100.
 * featured = printed on the box (display only). hours = rental period, 0 = permanent.
 */
export interface BoxEntry {
  item_id: number;
  name: string;
  reel: Reel;
  weight: number;
  featured: boolean;
  hours: number;
  percent: number;
}

export interface Box {
  id: number;
  name: string;
  entries: BoxEntry[];
}

export interface BoxEntryInput {
  item_id: number;
  reel: Reel;
  weight: number;
  featured: boolean;
  hours: number;
}

export interface PatchBoxBody {
  /** Replaces the whole entry list of the box. */
  entries: BoxEntryInput[];
}

// Coupons, mail, quests ---------------------------------------------------------

export interface Coupon {
  serial: string;
  item_id: number;
  item_name: string | null;
  item_count: number;
  points: number;
  cash: number;
  uses_left: number;
  /** UTC "YYYY-MM-DD HH:MM:SS" or null. */
  expires_at: string | null;
  note: string;
  used: number;
}

export interface NewCouponBody {
  serial: string;
  item_id?: number;
  item_count?: number;
  points?: number;
  cash?: number;
  uses_left?: number;
  expires_at?: string | null;
  note?: string;
}

/** box 0 = inbox, 1 = sent; msg_type 1 text, 2 item, 4 points; status 0 unread. */
export interface Mail {
  id: number;
  owner: string;
  box: number;
  sender: string;
  recipient: string;
  text: string;
  msg_type: number;
  item_uid: number;
  item_id: number;
  item_part: number;
  item_count: number;
  points: number;
  status: number;
  sent_date: string;
  hours_left: number;
}

export interface SendMailBody {
  /** Character names, or ["*"] for every player. */
  to: string[];
  sender?: string;
  text: string;
  item_id?: number;
  item_count?: number;
  points?: number;
}

export interface SendMailOut {
  delivered: string[];
  failed: string[];
}

/**
 * quest_type: 0 basic training, 2 daily, 3 normal chain, 4 weapon proficiency, 1 event.
 * event_start/event_end are OLE dates (days since 1899-12-30), 0 = no window.
 */
export interface Quest {
  id: number;
  quest_type: number;
  link_id: number;
  link_step: number;
  create_level: number;
  display_order: number;
  category: string;
  description: string;
  mission_type: number;
  mission_count: number;
  reward_exp: number;
  reward_point: number;
  reward_items: number[];
  reward_parts: number[];
  next_quests: number[];
  event_start: number;
  event_end: number;
  enabled: boolean;
  reward_item_names: string[];
  active: boolean;
  players: number;
  completed: number;
}

export interface QuestProgressRow {
  player: string;
  progress: number;
  complete: boolean;
  claimed: boolean;
}

export interface QuestDetail extends Quest {
  progress: QuestProgressRow[];
}

export interface PatchQuestBody {
  enabled?: boolean;
  quest_type?: number;
  category?: string;
  description?: string;
  mission_count?: number;
  reward_exp?: number;
  reward_point?: number;
  reward_items?: number[];
  next_quests?: number[];
  create_level?: number;
  event_start?: number;
  event_end?: number;
}

export interface NewQuestBody {
  id: number;
  quest_type: number;
  category: string;
  description: string;
  mission_count: number;
  reward_exp?: number;
  reward_point?: number;
  reward_items?: number[];
  next_quests?: number[];
  create_level?: number;
  event_start?: number;
  event_end?: number;
  enabled?: boolean;
  link_id?: number;
  link_step?: number;
}

export interface PlayerQuest {
  quest_id: number;
  category: string;
  description: string;
  quest_type: number;
  mission_count: number;
  progress: number;
  complete: boolean;
  claimed: boolean;
  assigned_at: string;
  completed_at: string | null;
}

export interface PatchPlayerQuestBody {
  progress?: number;
  complete?: boolean;
  claimed?: boolean;
}

// Server overview ---------------------------------------------------------------

export interface ServerCheck {
  label: string;
  target: string;
  ok: boolean;
  detail: string;
}

export interface ServerStatus {
  id: 'login' | 'channel' | 'admin' | string;
  name: string;
  description: string;
  operational: boolean;
  checks: ServerCheck[];
  /** Unix seconds. */
  started_at: number | null;
}

// Live game-server state ------------------------------------------------------

/** name is "" until the client logged in. */
export interface LiveSession {
  uid: number;
  name: string;
  in_channel: boolean;
  nat_type: number;
  in_room: boolean;
}

/** team 0/1 (AF/RSA); state != 0 = ready; udp is "ip:port" once the match started. */
export interface LiveMember {
  uid: number;
  name: string;
  team: number;
  state: number;
  is_owner: boolean;
  udp: string | null;
}

export interface LiveRoom {
  number: number;
  name: string;
  map: string;
  mode: string;
  mode_num: number;
  max_users: number;
  owner_uid: number;
  owner_name: string;
  playing: boolean;
  members: LiveMember[];
}

export interface Live {
  online: boolean;
  /** Unix seconds. */
  started_at: number | null;
  heartbeat_age_secs: number | null;
  sessions: LiveSession[];
  /** The first open room; `rooms` has them all. */
  room: LiveRoom | null;
  rooms: LiveRoom[];
}

// Request bodies -------------------------------------------------------------

export interface CreateAccountBody {
  username: string;
  password: string;
}

export interface PatchAccountBody {
  username?: string;
  password?: string;
  character?: string | null;
}

export interface PatchPlayerBody {
  points?: number;
  cash?: number;
  stats?: Partial<Stats>;
}

export interface GrantItemBody {
  item_id: number;
  stack?: number;
}

export interface PatchItemBody {
  stack?: number;
  durability?: number;
  slot?: number;
}

/**
 * A catalogue row added by hand. The id has to be one the game client knows -
 * it supplies the name, model and icon - and `item_type` decides which
 * inventory slot the item takes. On a server running without the game's own
 * shop table, this is how a shop gets built.
 */
export interface NewCatalogItemBody {
  id: number;
  name?: string;
  item_type: number;
  kind?: number;
  category?: string;
  sell_price?: number;
  availability?: Availability;
  prices?: { hours: number; points: number; cash: number }[];
}

export interface PatchCatalogItemBody {
  name?: string;
  sell_price?: number;
  availability?: Availability;
  prices?: { hours: number; points: number; cash: number }[];
}

export interface CatalogQuery {
  q?: string;
  kind?: string;
  availability?: string;
  limit?: number;
  offset?: number;
}

// ---------------------------------------------------------------------------
// Base URL
// ---------------------------------------------------------------------------

import { currentToken, handleUnauthorized } from './auth';

export const API_PORT = '55990';

/**
 * Base URL of the admin API, without a trailing slash.
 *  - NEXT_PUBLIC_API_URL when set (dev server),
 *  - '' when the page is served by the API itself (same origin, port 55990),
 *  - http://localhost:55990 otherwise.
 */
export function apiBase(): string {
  const env = process.env.NEXT_PUBLIC_API_URL;
  if (env) return env.replace(/\/+$/, '');
  if (typeof window !== 'undefined' && window.location.port === API_PORT) return '';
  return `http://localhost:${API_PORT}`;
}

// ---------------------------------------------------------------------------
// Fetch helpers
// ---------------------------------------------------------------------------

type Method = 'GET' | 'POST' | 'PATCH' | 'DELETE';

function extractError(data: unknown, fallback: string): string {
  if (data && typeof data === 'object' && 'error' in data) {
    const msg = (data as { error: unknown }).error;
    if (typeof msg === 'string' && msg.length > 0) return msg;
  }
  return fallback;
}

async function request<T>(method: Method, path: string, body?: unknown): Promise<T> {
  const url = apiBase() + path;
  const headers: Record<string, string> = { Accept: 'application/json' };
  const token = typeof window !== 'undefined' ? currentToken() : null;
  if (token) headers.Authorization = `Bearer ${token}`;
  const init: RequestInit = { method, headers };
  if (body !== undefined) {
    headers['Content-Type'] = 'application/json';
    init.body = JSON.stringify(body);
  }

  let res: Response;
  try {
    res = await fetch(url, init);
  } catch {
    throw new Error(`Cannot reach the admin API at ${apiBase() || window.location.origin}`);
  }

  // an expired or missing token: back to the sign-in page (the login call
  // itself reports its 401 as a normal error)
  if (res.status === 401 && !path.startsWith('/api/auth/login')) {
    handleUnauthorized();
    throw new Error('sign in required');
  }

  const text = await res.text();
  let data: unknown = null;
  if (text.length > 0) {
    try {
      data = JSON.parse(text);
    } catch {
      data = null;
    }
  }

  if (!res.ok) {
    throw new Error(extractError(data, `${res.status} ${res.statusText || 'request failed'}`));
  }
  return data as T;
}

export function get<T>(path: string): Promise<T> {
  return request<T>('GET', path);
}

export function post<T>(path: string, body?: unknown): Promise<T> {
  return request<T>('POST', path, body);
}

export function patch<T>(path: string, body?: unknown): Promise<T> {
  return request<T>('PATCH', path, body);
}

export function del<T>(path: string): Promise<T> {
  return request<T>('DELETE', path);
}

export interface PlayerFriend {
  name: string;
  level: number;
  /** The friend has blocked this player (one-directional). */
  blocked_you: boolean;
}

export interface PlayerSocial {
  friends: PlayerFriend[];
  blocks: string[];
  /** Players who have blocked this one. */
  blocked_by: string[];
}

/** Raw durability of a fresh weapon (the client shows it as 100%). */
export const FULL_DURABILITY = 100_000;

/** Builds a query string from defined, non-empty values. */
export function qs(params: Record<string, string | number | boolean | undefined | null>): string {
  const parts: string[] = [];
  for (const [key, value] of Object.entries(params)) {
    if (value === undefined || value === null || value === '') continue;
    parts.push(`${encodeURIComponent(key)}=${encodeURIComponent(String(value))}`);
  }
  return parts.length > 0 ? `?${parts.join('&')}` : '';
}

// ---------------------------------------------------------------------------
// Endpoint wrappers
// ---------------------------------------------------------------------------

const enc = encodeURIComponent;

export interface AuthSession {
  token: string;
  username: string;
  /** Unix seconds. */
  expires_at: number;
}

export const api = {
  auth: {
    login: (username: string, password: string) => post<AuthSession>('/api/auth/login', { username, password }),
    me: () => get<{ username: string; expires_at: number }>('/api/auth/me'),
    changePassword: (current: string, next: string) => post<OkResponse>('/api/auth/password', { current, new: next }),
  },

  summary: () => get<Summary>('/api/summary'),
  servers: () => get<ServerStatus[]>('/api/servers'),
  live: () => get<Live>('/api/live'),

  accounts: {
    list: () => get<Account[]>('/api/accounts'),
    create: (body: CreateAccountBody) => post<Account>('/api/accounts', body),
    update: (id: number, body: PatchAccountBody) => patch<Account>(`/api/accounts/${id}`, body),
    remove: (id: number, withPlayer: boolean) =>
      del<OkResponse>(`/api/accounts/${id}?with_player=${withPlayer ? 'true' : 'false'}`),
  },

  players: {
    list: () => get<PlayerSummary[]>('/api/players'),
    get: (name: string) => get<Player>(`/api/players/${enc(name)}`),
    update: (name: string, body: PatchPlayerBody) => patch<Player>(`/api/players/${enc(name)}`, body),
    remove: (name: string) => del<OkResponse>(`/api/players/${enc(name)}`),
    grantItem: (name: string, body: GrantItemBody) => post<Item>(`/api/players/${enc(name)}/items`, body),
    updateItem: (name: string, uniqueId: number, body: PatchItemBody) =>
      patch<Item>(`/api/players/${enc(name)}/items/${uniqueId}`, body),
    removeItem: (name: string, uniqueId: number) =>
      del<OkResponse>(`/api/players/${enc(name)}/items/${uniqueId}`),
    grantSkill: (name: string, skillId: number) =>
      post<Skill>(`/api/players/${enc(name)}/skills`, { skill_id: skillId }),
    removeSkill: (name: string, uniqueId: number) =>
      del<OkResponse>(`/api/players/${enc(name)}/skills/${uniqueId}`),
    social: (name: string) => get<PlayerSocial>(`/api/players/${enc(name)}/social`),
    addFriend: (name: string, other: string) => post<PlayerSocial>(`/api/players/${enc(name)}/friends`, { name: other }),
    removeFriend: (name: string, other: string) => del<PlayerSocial>(`/api/players/${enc(name)}/friends/${enc(other)}`),
    addBlock: (name: string, other: string) => post<PlayerSocial>(`/api/players/${enc(name)}/blocks`, { name: other }),
    removeBlock: (name: string, other: string) => del<PlayerSocial>(`/api/players/${enc(name)}/blocks/${enc(other)}`),
    repairAll: (name: string) => post<{ ok: boolean; repaired: number }>(`/api/players/${enc(name)}/repair`, {}),
  },

  items: {
    list: (query: CatalogQuery) =>
      get<CatalogPage>(
        `/api/items${qs({
          q: query.q,
          kind: query.kind,
          availability: query.availability,
          limit: query.limit ?? 50,
          offset: query.offset ?? 0,
        })}`,
      ),
    get: (id: number) => get<CatalogItem>(`/api/items/${id}`),
    create: (body: NewCatalogItemBody) => post<CatalogItem>('/api/items', body),
    update: (id: number, body: PatchCatalogItemBody) => patch<CatalogItem>(`/api/items/${id}`, body),
  },

  kinds: () => get<Kind[]>('/api/kinds'),

  matches: {
    list: (limit = 50, offset = 0, player?: string) => get<MatchPage>(`/api/matches${qs({ limit, offset, player })}`),
    get: (id: number) => get<MatchDetail>(`/api/matches/${id}`),
    remove: (id: number) => del<OkResponse>(`/api/matches/${id}`),
    ofPlayer: (name: string, limit = 20, offset = 0) =>
      get<MatchPage>(`/api/players/${enc(name)}/matches${qs({ limit, offset })}`),
  },

  leaderboard: () => get<LeaderRow[]>('/api/leaderboard'),

  boxes: {
    list: () => get<BoxSummary[]>('/api/boxes'),
    get: (id: number) => get<Box>(`/api/boxes/${id}`),
    update: (id: number, body: PatchBoxBody) => patch<Box>(`/api/boxes/${id}`, body),
  },

  coupons: {
    list: () => get<Coupon[]>('/api/coupons'),
    create: (body: NewCouponBody) => post<Coupon>('/api/coupons', body),
    remove: (serial: string) => del<OkResponse>(`/api/coupons/${enc(serial)}`),
  },

  mail: {
    list: (player?: string, limit = 200) => get<Mail[]>(`/api/mail${qs({ player, limit })}`),
    send: (body: SendMailBody) => post<SendMailOut>('/api/mail', body),
    remove: (id: number) => del<OkResponse>(`/api/mail/${id}`),
  },

  quests: {
    list: (query: { q?: string; quest_type?: number | ''; filter?: string }) =>
      get<Quest[]>(`/api/quests${qs({ q: query.q, quest_type: query.quest_type === '' ? undefined : query.quest_type, filter: query.filter })}`),
    get: (id: number) => get<QuestDetail>(`/api/quests/${id}`),
    update: (id: number, body: PatchQuestBody) => patch<QuestDetail>(`/api/quests/${id}`, body),
    create: (body: NewQuestBody) => post<QuestDetail>('/api/quests', body),
    remove: (id: number) => del<OkResponse>(`/api/quests/${id}`),
    ofPlayer: (name: string) => get<PlayerQuest[]>(`/api/players/${enc(name)}/quests`),
    assign: (name: string, questId: number) => post<PlayerQuest[]>(`/api/players/${enc(name)}/quests`, { quest_id: questId }),
    patchPlayer: (name: string, questId: number, body: PatchPlayerQuestBody) =>
      patch<PlayerQuest[]>(`/api/players/${enc(name)}/quests/${questId}`, body),
    unassign: (name: string, questId: number) => del<PlayerQuest[]>(`/api/players/${enc(name)}/quests/${questId}`),
  },
};

/** OLE date (days since 1899-12-30) <-> `datetime-local` input value. */
export function oleToLocalInput(ole: number): string {
  if (!ole || ole <= 0) return '';
  const ms = (ole - 25569) * 86400000;
  const d = new Date(ms);
  const pad = (n: number) => String(n).padStart(2, '0');
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}T${pad(d.getHours())}:${pad(d.getMinutes())}`;
}

export function localInputToOle(value: string): number {
  if (!value) return 0;
  const ms = new Date(value).getTime();
  if (Number.isNaN(ms)) return 0;
  return ms / 86400000 + 25569;
}

export const QUEST_TYPE_LABEL: Record<number, string> = {
  0: 'Basic training',
  1: 'Event',
  2: 'Daily',
  3: 'Chain',
  4: 'Proficiency',
  8: 'Mission',
};

export function questTypeLabel(t: number): string {
  return QUEST_TYPE_LABEL[t] ?? `Type ${t}`;
}
