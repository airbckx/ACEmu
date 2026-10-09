import type { Availability, Price } from './api';

/** Thousands-separated integer. */
export function fmtInt(n: number): string {
  return Number.isFinite(n) ? Math.trunc(n).toLocaleString('en-US') : String(n);
}

/** OLE automation date (days since 1899-12-30) to a JS Date. */
export function oleToDate(ole: number): Date {
  const epoch = Date.UTC(1899, 11, 30);
  return new Date(epoch + ole * 86_400_000);
}

/** Item expiry: 0 = permanent, otherwise an OLE date. */
export function fmtUntil(until: number): string {
  if (!until) return 'Permanent';
  const d = oleToDate(until);
  if (Number.isNaN(d.getTime())) return String(until);
  return d.toLocaleString();
}

/** Play time (seconds) as "1h 23m". */
export function fmtDuration(seconds: number): string {
  const s = Math.max(0, Math.trunc(seconds));
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  if (h > 0) return `${h}h ${m}m`;
  if (m > 0) return `${m}m ${s % 60}s`;
  return `${s}s`;
}

export function fmtDate(iso: string): string {
  const d = new Date(iso);
  if (Number.isNaN(d.getTime())) return iso;
  return d.toLocaleString();
}

/** Unix seconds to a local date-time string. */
export function fmtUnix(seconds: number): string {
  const d = new Date(seconds * 1000);
  if (Number.isNaN(d.getTime())) return String(seconds);
  return d.toLocaleString();
}

export const TEAM_LABEL: Record<number, string> = { 0: 'AF', 1: 'RSA' };

export function fmtTeam(team: number): string {
  return TEAM_LABEL[team] ?? `Team ${team}`;
}

export function fmtSlot(slot: number): string {
  return slot < 0 ? 'Bag' : `Equipped #${slot}`;
}

export function fmtHours(hours: number): string {
  if (hours === 0) return 'Permanent';
  if (hours % 24 === 0) return `${hours / 24}d`;
  return `${hours}h`;
}

/** Short one-line summary of the price rows, e.g. "7d 300P / 30C, Perm 1,200P". */
export function fmtPrices(prices: Price[]): string {
  if (prices.length === 0) return '-';
  return prices
    .map((p) => {
      const parts: string[] = [];
      if (p.points > 0) parts.push(`${fmtInt(p.points)}P`);
      if (p.cash > 0) parts.push(`${fmtInt(p.cash)}C`);
      const cost = parts.length > 0 ? parts.join(' / ') : 'free';
      return `${fmtHours(p.hours)} ${cost}`;
    })
    .join(', ');
}

export const AVAILABILITY_LABEL: Record<Availability, string> = {
  shop: 'In shop',
  lootbox: 'Lootbox only',
  hidden: 'Hidden',
};

/** SQLite UTC timestamp ("YYYY-MM-DD HH:MM:SS") to a local date-time string. */
export function fmtUtc(ts: string): string {
  const d = new Date(ts.replace(' ', 'T') + 'Z');
  if (Number.isNaN(d.getTime())) return ts;
  return d.toLocaleString();
}

/** "3:07" style match clock. */
export function fmtClock(seconds: number): string {
  const s = Math.max(0, Math.trunc(seconds));
  const m = Math.floor(s / 60);
  return `${m}:${String(s % 60).padStart(2, '0')}`;
}

/** The last path segment of a game class, e.g. "WGame.wTeamGame" -> "Team Game". */
export function fmtMode(mode: string): string {
  const MODES: Record<string, string> = {
    wTeamGame: 'Team Deathmatch',
    wNDFGameInfo: 'Defense',
    wSBTGameInfo: 'Search & Destroy',
    wDomGameInfo: 'Domination',
    wDeathMatch: 'Deathmatch',
  };
  const tail = mode.split('.').pop() ?? mode;
  return MODES[tail] ?? tail.replace(/^w/, '').replace(/GameInfo$/, '');
}

export function kdRatio(kills: number, deaths: number): string {
  if (deaths === 0) return kills > 0 ? kills.toFixed(2) : '0.00';
  return (kills / deaths).toFixed(2);
}
