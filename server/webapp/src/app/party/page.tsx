'use client';

/**
 * Party screen: a fullscreen, read-only board for a TV during a LAN party.
 * No interaction, no scrollbars, large type, polls the API and never clears a
 * panel while refetching (stale data stays until fresh data arrives). When the
 * API cannot be reached the status strip says "server offline" and the last
 * known state stays on screen.
 *
 * ?since=YYYY-MM-DD[THH:MM] overrides the "tonight" cutoff (default: today
 * at local midnight).
 */

import { useSearchParams } from 'next/navigation';
import { Suspense, useEffect, useMemo, useRef, useState } from 'react';
import { api, type LeaderRow, type Live, type MatchDetail, type MatchPlayer, type MatchSummary, type Summary } from '@/lib/api';
import { fmtClock, fmtDuration, fmtInt, fmtMode, fmtTeam, kdRatio } from '@/lib/format';
import { ItemIcon } from '@/components/ItemIcon';

const LIVE_MS = 2000;
const MATCH_MS = 5000;
const TONIGHT_MS = 30000;
const ROTATE_MS = 15000;
const KILL_FEED_ROWS = 10;
const STANDINGS_ROWS = 10;

export default function PartyPage() {
  return (
    <Suspense fallback={null}>
      <Party />
    </Suspense>
  );
}

// ---------------------------------------------------------------------------
// data
// ---------------------------------------------------------------------------

/** SQLite UTC "YYYY-MM-DD HH:MM:SS" -> Date. */
function utcDate(ts: string): Date {
  return new Date(ts.replace(' ', 'T') + 'Z');
}

function defaultSince(): Date {
  const d = new Date();
  d.setHours(0, 0, 0, 0);
  return d;
}

function parseSince(raw: string | null): Date {
  if (!raw) return defaultSince();
  const d = new Date(raw);
  return Number.isNaN(d.getTime()) ? defaultSince() : d;
}

interface TonightRow {
  name: string;
  matches: number;
  wins: number;
  kills: number;
  deaths: number;
  assists: number;
  headshots: number;
  score: number;
  bestKills: number;
}

function aggregate(details: MatchDetail[]): TonightRow[] {
  const rows = new Map<string, TonightRow>();
  for (const m of details) {
    for (const p of m.scoreboard) {
      if (p.is_bot) continue;
      const r = rows.get(p.name) ?? { name: p.name, matches: 0, wins: 0, kills: 0, deaths: 0, assists: 0, headshots: 0, score: 0, bestKills: 0 };
      r.matches += 1;
      if (p.won === 1) r.wins += 1;
      r.kills += p.kills;
      r.deaths += p.deaths;
      r.assists += p.assists;
      r.headshots += p.headshots;
      r.score += p.score;
      r.bestKills = Math.max(r.bestKills, p.kills);
      rows.set(p.name, r);
    }
  }
  return [...rows.values()].sort((a, b) => b.kills - a.kills || b.score - a.score || a.deaths - b.deaths);
}

/** Polls `fn` every `ms`; keeps the last good value and reports reachability. */
function usePoll<T>(fn: () => Promise<T>, ms: number, deps: unknown[] = []): { data: T | null; ok: boolean } {
  const [data, setData] = useState<T | null>(null);
  const [ok, setOk] = useState(true);
  const inFlight = useRef(false);
  useEffect(() => {
    let cancelled = false;
    const tick = async () => {
      if (inFlight.current) return;
      inFlight.current = true;
      try {
        const v = await fn();
        if (!cancelled) {
          setData(v);
          setOk(true);
        }
      } catch {
        if (!cancelled) setOk(false);
      } finally {
        inFlight.current = false;
      }
    };
    void tick();
    const t = window.setInterval(() => void tick(), ms);
    return () => {
      cancelled = true;
      window.clearInterval(t);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, deps);
  return { data, ok };
}

// ---------------------------------------------------------------------------
// page
// ---------------------------------------------------------------------------

function Party() {
  const params = useSearchParams();
  const since = useMemo(() => parseSince(params.get('since')), [params]);

  const live = usePoll<Live>(() => api.live(), LIVE_MS);
  const summary = usePoll<Summary>(() => api.summary(), TONIGHT_MS);
  const career = usePoll<LeaderRow[]>(() => api.leaderboard(), TONIGHT_MS);
  const newest = usePoll<MatchSummary | null>(() => api.matches.list(1).then((p) => p.matches[0] ?? null), MATCH_MS);

  // the last match's detail, refetched only when the newest id changes
  const [last, setLast] = useState<MatchDetail | null>(null);
  useEffect(() => {
    const id = newest.data?.id;
    if (!id || last?.id === id) return;
    let cancelled = false;
    api.matches
      .get(id)
      .then((d) => {
        if (!cancelled) setLast(d);
      })
      .catch(() => {});
    return () => {
      cancelled = true;
    };
  }, [newest.data?.id, last?.id]);

  // tonight: every match since the cutoff, details cached by id
  const detailCache = useRef(new Map<number, MatchDetail>());
  const [tonight, setTonight] = useState<{ rows: TonightRow[]; matches: number }>({ rows: [], matches: 0 });
  useEffect(() => {
    let cancelled = false;
    const refresh = async () => {
      try {
        const page = await api.matches.list(50);
        const recent = page.matches.filter((m) => utcDate(m.played_at) >= since);
        const details: MatchDetail[] = [];
        for (const m of recent) {
          let d = detailCache.current.get(m.id);
          if (!d) {
            d = await api.matches.get(m.id);
            detailCache.current.set(m.id, d);
          }
          details.push(d);
        }
        if (!cancelled) setTonight({ rows: aggregate(details), matches: details.length });
      } catch {
        // keep what we have
      }
    };
    void refresh();
    const t = window.setInterval(() => void refresh(), TONIGHT_MS);
    return () => {
      cancelled = true;
      window.clearInterval(t);
    };
  }, [since, newest.data?.id]);

  // rotate the right panel between tonight and career standings
  const [rotate, setRotate] = useState(0);
  useEffect(() => {
    const t = window.setInterval(() => setRotate((r) => r + 1), ROTATE_MS);
    return () => window.clearInterval(t);
  }, []);
  const showCareer = rotate % 2 === 1 && (career.data?.length ?? 0) > 0;

  // wall clock
  const [now, setNow] = useState(() => new Date());
  useEffect(() => {
    const t = window.setInterval(() => setNow(new Date()), 1000);
    return () => window.clearInterval(t);
  }, []);

  const reachable = live.ok;
  const gameOnline = reachable && (live.data?.online ?? false);
  const sessions = (live.data?.sessions ?? []).filter((s) => s.name);
  const rooms = live.data?.rooms ?? [];

  return (
    <div className="party fixed inset-0 flex cursor-none select-none flex-col overflow-hidden bg-zinc-950 text-zinc-100">
      {/* status strip */}
      <header className="flex items-center justify-between border-b border-zinc-800 px-[2vw] py-[0.8vh]">
        <div className="flex items-center gap-[1.2vw]">
          <span className="flex h-[3.2vh] w-[3.2vh] items-center justify-center rounded-md bg-accent-500 text-[1.4vh] font-black text-zinc-950">AC</span>
          <span className="text-[2.4vh] font-bold tracking-wide">Arctic Combat</span>
          <StatusPill ok={reachable} online={gameOnline} />
        </div>
        <div className="flex items-center gap-[2vw] text-[1.8vh] text-zinc-400">
          <span>
            <b className="text-zinc-100">{sessions.length}</b> online
          </span>
          <span>
            <b className="text-zinc-100">{rooms.length}</b> room{rooms.length === 1 ? '' : 's'}
          </span>
          <span>
            <b className="text-zinc-100">{tonight.matches}</b> match{tonight.matches === 1 ? '' : 'es'} tonight
          </span>
          {summary.data && (
            <span>
              <b className="text-zinc-100">{fmtInt(summary.data.matches)}</b> all time
            </span>
          )}
          <span className="font-mono text-[2.2vh] text-zinc-100">{now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}</span>
        </div>
      </header>

      <main className="grid min-h-0 flex-1 grid-cols-[1fr_1.6fr_1.2fr] gap-[1.2vw] p-[1.2vw]">
        {/* now */}
        <Panel title="Right now">
          {rooms.length === 0 && (
            <div className="text-[2.2vh] text-zinc-500">{gameOnline ? 'No room open. Make one!' : 'Game server offline.'}</div>
          )}
          <div className="flex flex-col gap-[1.2vh]">
            {rooms.slice(0, 3).map((r) => (
              <div key={r.number} className={`rounded-xl border p-[1vw] ${r.playing ? 'border-emerald-500/50 bg-emerald-500/10' : 'border-zinc-800 bg-zinc-900/60'}`}>
                <div className="flex items-baseline justify-between gap-[1vw]">
                  <div className="truncate text-[2.6vh] font-bold">{r.name || `Room ${r.number}`}</div>
                  <div className={`shrink-0 text-[1.8vh] font-semibold uppercase tracking-wider ${r.playing ? 'text-emerald-300' : 'text-zinc-400'}`}>
                    {r.playing ? 'playing' : 'in lobby'}
                  </div>
                </div>
                <div className="truncate text-[1.9vh] text-zinc-400">
                  {r.map} - {fmtMode(r.mode)} - {r.members.length}/{r.max_users}
                </div>
                <div className="mt-[0.8vh] grid grid-cols-2 gap-x-[1vw] gap-y-[0.3vh] text-[2vh]">
                  {[0, 1].map((team) => (
                    <div key={team} className="min-w-0">
                      <div className={`text-[1.5vh] font-semibold uppercase tracking-wider ${team === 0 ? 'text-sky-300' : 'text-amber-300'}`}>{fmtTeam(team)}</div>
                      {r.members
                        .filter((m) => m.team === team)
                        .slice(0, 8)
                        .map((m) => (
                          <div key={m.uid} className="truncate">
                            {m.name || `#${m.uid}`}
                            {m.is_owner && <span className="ml-1 text-[1.4vh] text-zinc-500">host</span>}
                            {!r.playing && m.state !== 0 && !m.is_owner && <span className="ml-1 text-[1.4vh] text-emerald-400">ready</span>}
                          </div>
                        ))}
                    </div>
                  ))}
                </div>
              </div>
            ))}
          </div>
          <div className="mt-[1.5vh] text-[1.5vh] font-semibold uppercase tracking-wider text-zinc-500">In the lobby</div>
          <div className="flex flex-wrap gap-[0.5vw] text-[1.9vh]">
            {sessions.filter((s) => !s.in_room).length === 0 && <span className="text-zinc-600">nobody</span>}
            {sessions
              .filter((s) => !s.in_room)
              .slice(0, 16)
              .map((s) => (
                <span key={s.uid} className="rounded-md bg-zinc-800 px-[0.6vw] py-[0.2vh]">
                  {s.name}
                </span>
              ))}
          </div>
        </Panel>

        {/* last match */}
        <Panel title={last ? `Last match - ${last.map} - ${fmtMode(last.mode)}` : 'Last match'}>
          {!last && <div className="text-[2.2vh] text-zinc-500">No match recorded yet.</div>}
          {last && <LastMatch m={last} />}
        </Panel>

        {/* standings */}
        <Panel title={showCareer ? 'All-time standings' : `Tonight's standings`} subtitle={showCareer ? 'career totals' : `since ${since.toLocaleDateString()} ${since.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}`}>
          {showCareer ? <CareerTable rows={career.data ?? []} /> : <TonightTable rows={tonight.rows} />}
        </Panel>
      </main>
    </div>
  );
}

// ---------------------------------------------------------------------------
// pieces
// ---------------------------------------------------------------------------

function StatusPill({ ok, online }: { ok: boolean; online: boolean }) {
  const label = !ok ? 'server offline' : online ? 'online' : 'game server down';
  const tone = !ok ? 'bg-rose-500/20 text-rose-300' : online ? 'bg-emerald-500/20 text-emerald-300' : 'bg-amber-500/20 text-amber-300';
  const dot = !ok ? 'bg-rose-400' : online ? 'bg-emerald-400 animate-pulse' : 'bg-amber-400';
  return (
    <span className={`inline-flex items-center gap-[0.5vw] rounded-full px-[0.8vw] py-[0.3vh] text-[1.6vh] font-semibold uppercase tracking-wider ${tone}`}>
      <span className={`h-[1.2vh] w-[1.2vh] rounded-full ${dot}`} />
      {label}
    </span>
  );
}

function Panel({ title, subtitle, children }: { title: string; subtitle?: string; children: React.ReactNode }) {
  return (
    <section className="flex min-h-0 flex-col overflow-hidden rounded-2xl border border-zinc-800 bg-zinc-900/40">
      <header className="flex items-baseline justify-between gap-[1vw] border-b border-zinc-800 px-[1.2vw] py-[1vh]">
        <h2 className="truncate text-[2.4vh] font-bold uppercase tracking-wider text-zinc-200">{title}</h2>
        {subtitle && <span className="shrink-0 text-[1.6vh] text-zinc-500">{subtitle}</span>}
      </header>
      <div className="min-h-0 flex-1 overflow-hidden p-[1.2vw]">{children}</div>
    </section>
  );
}

function LastMatch({ m }: { m: MatchDetail }) {
  const teams = [0, 1].map((t) => m.scoreboard.filter((p) => p.team === t).sort((a, b) => b.score - a.score || b.kills - a.kills));
  const winnerLabel = m.winner < 0 ? 'Draw' : `${fmtTeam(m.winner)} wins`;
  const feed = [...m.kill_feed].sort((a, b) => b.seq - a.seq).slice(0, KILL_FEED_ROWS);
  return (
    <div className="flex h-full min-h-0 flex-col gap-[1.2vh]">
      <div className="flex items-center justify-between">
        <div className="flex items-baseline gap-[1vw]">
          <span className="text-[5vh] font-black tabular-nums text-sky-300">{m.team0_score}</span>
          <span className="text-[2.5vh] text-zinc-600">:</span>
          <span className="text-[5vh] font-black tabular-nums text-amber-300">{m.team1_score}</span>
        </div>
        <div className="text-right">
          <div className={`text-[3vh] font-bold ${m.winner === 0 ? 'text-sky-300' : m.winner === 1 ? 'text-amber-300' : 'text-zinc-300'}`}>{winnerLabel}</div>
          <div className="text-[1.7vh] text-zinc-500">
            {fmtDuration(m.duration)} - host {m.host} - {utcDate(m.played_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
          </div>
        </div>
      </div>
      <div className="grid grid-cols-2 gap-[1vw]">
        {teams.map((rows, t) => (
          <ScoreTable key={t} team={t} rows={rows} />
        ))}
      </div>
      <div className="min-h-0 flex-1">
        <div className="mb-[0.5vh] text-[1.5vh] font-semibold uppercase tracking-wider text-zinc-500">Kill feed (last {KILL_FEED_ROWS})</div>
        <div className="flex flex-col gap-[0.25vh] text-[1.9vh]">
          {feed.length === 0 && <span className="text-zinc-600">no kills recorded</span>}
          {feed.map((k) => (
            <div key={k.seq} className="flex items-center gap-[0.8vw] truncate">
              <span className="w-[3.2vw] shrink-0 font-mono text-[1.6vh] text-zinc-500">{fmtClock(k.at_secs)}</span>
              <span className="shrink-0 font-semibold text-zinc-100">{k.killer_name}</span>
              <ItemIcon id={k.weapon === 99999 ? 0 : k.weapon} size="sm" hideMissing />
              <span className="min-w-0 truncate text-zinc-400">{k.weapon_name ?? (k.weapon === 99999 ? 'melee' : '')}</span>
              {k.headshot && <span className="shrink-0 rounded bg-rose-500/20 px-[0.4vw] text-[1.4vh] font-bold uppercase text-rose-300">headshot</span>}
              <span className="shrink-0 text-zinc-600">&rarr;</span>
              <span className="shrink-0 text-zinc-300">{k.victim_name}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

function ScoreTable({ team, rows }: { team: number; rows: MatchPlayer[] }) {
  const color = team === 0 ? 'text-sky-300' : 'text-amber-300';
  return (
    <div>
      <div className={`mb-[0.3vh] text-[1.6vh] font-semibold uppercase tracking-wider ${color}`}>{fmtTeam(team)}</div>
      <table className="w-full text-[1.9vh] tabular-nums">
        <thead className="text-[1.4vh] uppercase tracking-wider text-zinc-500">
          <tr>
            <th className="text-left font-semibold">Player</th>
            <th className="text-right font-semibold">K</th>
            <th className="text-right font-semibold">D</th>
            <th className="text-right font-semibold">A</th>
            <th className="text-right font-semibold">Score</th>
          </tr>
        </thead>
        <tbody>
          {rows.slice(0, 8).map((p) => (
            <tr key={p.uid} className={p.is_bot ? 'text-zinc-500' : 'text-zinc-100'}>
              <td className="truncate pr-[0.5vw]">
                {p.name}
                {p.is_bot && <span className="ml-1 text-[1.3vh] text-zinc-600">bot</span>}
              </td>
              <td className="text-right font-semibold">{p.kills}</td>
              <td className="text-right">{p.deaths}</td>
              <td className="text-right">{p.assists}</td>
              <td className="text-right">{fmtInt(p.score)}</td>
            </tr>
          ))}
          {rows.length === 0 && (
            <tr>
              <td colSpan={5} className="text-zinc-600">
                -
              </td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
}

function Rank({ i }: { i: number }) {
  const cls = i === 0 ? 'text-amber-300' : i === 1 ? 'text-zinc-300' : i === 2 ? 'text-orange-400' : 'text-zinc-600';
  return <span className={`inline-block w-[2vw] font-black ${cls}`}>{i + 1}</span>;
}

function TonightTable({ rows }: { rows: TonightRow[] }) {
  if (rows.length === 0) return <div className="text-[2.2vh] text-zinc-500">No match played yet tonight.</div>;
  return (
    <table className="w-full text-[2.1vh] tabular-nums">
      <thead className="text-[1.5vh] uppercase tracking-wider text-zinc-500">
        <tr>
          <th className="text-left font-semibold" colSpan={2}>
            Player
          </th>
          <th className="text-right font-semibold">Kills</th>
          <th className="text-right font-semibold">Deaths</th>
          <th className="text-right font-semibold">K/D</th>
          <th className="text-right font-semibold">HS</th>
          <th className="text-right font-semibold">W / M</th>
        </tr>
      </thead>
      <tbody>
        {rows.slice(0, STANDINGS_ROWS).map((r, i) => (
          <tr key={r.name} className={i === 0 ? 'text-[2.5vh] font-bold text-zinc-50' : 'text-zinc-200'}>
            <td className="w-[2.5vw]">
              <Rank i={i} />
            </td>
            <td className="truncate pr-[0.5vw]">{r.name}</td>
            <td className="text-right font-semibold">{r.kills}</td>
            <td className="text-right">{r.deaths}</td>
            <td className="text-right">{kdRatio(r.kills, r.deaths)}</td>
            <td className="text-right">{r.headshots}</td>
            <td className="text-right">
              {r.wins} / {r.matches}
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

function CareerTable({ rows }: { rows: LeaderRow[] }) {
  const sorted = [...rows].sort((a, b) => b.kills - a.kills || b.exp - a.exp).slice(0, STANDINGS_ROWS);
  if (sorted.length === 0) return <div className="text-[2.2vh] text-zinc-500">No players yet.</div>;
  return (
    <table className="w-full text-[2.1vh] tabular-nums">
      <thead className="text-[1.5vh] uppercase tracking-wider text-zinc-500">
        <tr>
          <th className="text-left font-semibold" colSpan={2}>
            Player
          </th>
          <th className="text-right font-semibold">Lvl</th>
          <th className="text-right font-semibold">Kills</th>
          <th className="text-right font-semibold">K/D</th>
          <th className="text-right font-semibold">Wins</th>
          <th className="text-right font-semibold">Matches</th>
        </tr>
      </thead>
      <tbody>
        {sorted.map((r, i) => (
          <tr key={r.name} className={i === 0 ? 'text-[2.5vh] font-bold text-zinc-50' : 'text-zinc-200'}>
            <td className="w-[2.5vw]">
              <Rank i={i} />
            </td>
            <td className="truncate pr-[0.5vw]">{r.name}</td>
            <td className="text-right">{r.level}</td>
            <td className="text-right font-semibold">{fmtInt(r.kills)}</td>
            <td className="text-right">{kdRatio(r.kills, r.deaths)}</td>
            <td className="text-right">{fmtInt(r.wins)}</td>
            <td className="text-right">{fmtInt(r.matches)}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
