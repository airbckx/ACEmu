'use client';

import Link from 'next/link';
import { useRouter, useSearchParams } from 'next/navigation';
import { Suspense, useCallback, useEffect, useState } from 'react';
import { api, type MatchDetail, type MatchEvent, type MatchEventActor, type MatchKill, type MatchPlayer } from '@/lib/api';
import { fmtClock, fmtInt, fmtMode, fmtTeam, fmtUtc, kdRatio } from '@/lib/format';
import { Badge } from '@/components/Badge';
import { Button } from '@/components/Button';
import { Card, StatCard } from '@/components/Card';
import { ConfirmDialog } from '@/components/ConfirmDialog';
import { ErrorBox, Loading, PageHeader } from '@/components/Feedback';
import { IconArrowLeft, IconTrash } from '@/components/Icons';
import { DataTable, type Column } from '@/components/Table';
import { errorMessage, useToast } from '@/components/Toast';
import { WinnerBadge } from '@/components/MatchBits';
import { ItemIcon } from '@/components/ItemIcon';

export default function MatchViewPage() {
  return (
    <Suspense fallback={<Loading />}>
      <MatchView />
    </Suspense>
  );
}

function playerHref(name: string): string {
  return `/players/view/?name=${encodeURIComponent(name)}`;
}

const BODY_PART: Record<number, string> = { 1: 'Head', 2: 'Chest', 3: 'Stomach' };

function PlayerName({ name, isBot }: { name: string; isBot: boolean }) {
  if (isBot) return <span className="text-zinc-400">{name}</span>;
  return (
    <Link href={playerHref(name)} className="font-medium text-accent-300 hover:underline">
      {name}
    </Link>
  );
}

const SCORE_COLUMNS: Column<MatchPlayer>[] = [
  {
    key: 'name',
    header: 'Player',
    render: (p) => (
      <span className="inline-flex items-center gap-2">
        <PlayerName name={p.name} isBot={p.is_bot} />
        {p.is_bot && <Badge tone="neutral">bot</Badge>}
      </span>
    ),
  },
  { key: 'score', header: 'Score', className: 'text-right', render: (p) => fmtInt(p.score) },
  { key: 'kills', header: 'Kills', className: 'text-right', render: (p) => fmtInt(p.kills) },
  { key: 'assists', header: 'Assists', className: 'text-right', hideBelow: 'sm', render: (p) => fmtInt(p.assists) },
  { key: 'deaths', header: 'Deaths', className: 'text-right', render: (p) => fmtInt(p.deaths) },
  { key: 'kd', header: 'K/D', className: 'text-right', hideBelow: 'sm', render: (p) => kdRatio(p.kills, p.deaths) },
  { key: 'hs', header: 'HS', className: 'text-right', hideBelow: 'md', render: (p) => fmtInt(p.headshots) },
  { key: 'exp', header: 'EXP', className: 'text-right', hideBelow: 'md', render: (p) => fmtInt(p.exp) },
  { key: 'points', header: 'Points', className: 'text-right', hideBelow: 'md', render: (p) => fmtInt(p.points) },
];

const KILL_COLUMNS: Column<MatchKill>[] = [
  { key: 'at', header: 'Time', className: 'w-16 font-mono text-zinc-400', render: (k) => fmtClock(k.at_secs) },
  { key: 'killer', header: 'Killer', render: (k) => <span className="text-zinc-100">{k.killer_name}</span> },
  {
    key: 'weapon',
    header: 'Weapon',
    render: (k) => (
      <span className="inline-flex items-center gap-2">
        <ItemIcon id={k.weapon === 99999 ? 0 : k.weapon} size="xs" hideMissing />
        <span className="text-zinc-300">{k.weapon_name ?? (k.weapon === 99999 ? 'Melee' : `#${k.weapon}`)}</span>
        {k.headshot && <Badge tone="danger">headshot</Badge>}
      </span>
    ),
  },
  { key: 'victim', header: 'Victim', render: (k) => <span className="text-zinc-100">{k.victim_name}</span> },
  { key: 'part', header: 'Hit', hideBelow: 'md', render: (k) => BODY_PART[k.body_part] ?? `part ${k.body_part}` },
];

const SITE_LABEL: Record<number, string> = { 0: 'A', 1: 'B', 2: 'C' };
const POSTURE: Record<number, string> = { 0: 'standing', 1: 'crouched', 2: 'prone' };
/** RoundEndType from the client's GetRoundEndType (Search & Destroy family). */
const ROUND_END: Record<number, string> = {
  2: 'side wiped out',
  3: 'bomb exploded',
  4: 'defused / time ran out',
  11: 'a side was empty',
  1: 'humans won',
};

type EventBadge = { tone: 'neutral' | 'accent' | 'success' | 'warning' | 'danger'; label: string };

const EVENT_BADGE: Record<string, EventBadge> = {
  round_end: { tone: 'accent', label: 'round' },
  bomb_planted: { tone: 'danger', label: 'bomb planted' },
  bomb_defused: { tone: 'success', label: 'bomb defused' },
  point_taken: { tone: 'warning', label: 'point taken' },
  dom_score: { tone: 'neutral', label: 'score' },
  chopper_kill: { tone: 'danger', label: 'chopper down' },
  udp_kick: { tone: 'neutral', label: 'udp dropped' },
  host_lost: { tone: 'neutral', label: 'host lost' },
};

function num(v: unknown): number | null {
  return typeof v === 'number' && Number.isFinite(v) ? v : null;
}

function actorNote(x: Record<string, unknown>): string {
  const bits: string[] = [];
  const posture = num(x.posture);
  if (posture !== null && POSTURE[posture]) bits.push(POSTURE[posture]);
  const range = num(x.range);
  if (range !== null) bits.push(`${fmtInt(range)} uu from the site`);
  return bits.join(', ');
}

/** One line describing the event; team labels come from fmtTeam. */
function describeEvent(e: MatchEvent): string {
  const x = e.extra ?? {};
  switch (e.kind) {
    case 'round_end': {
      const round = num(x.round);
      const winner = num(x.winner);
      const endType = num(x.end_type);
      const ad = num(x.winner_ad);
      const secs = num(x.round_secs);
      const parts = [`Round ${round ?? '?'} to ${winner === null ? '?' : fmtTeam(winner)}`];
      if (ad === 1) parts.push('(attacking)');
      else if (ad === 2) parts.push('(defending)');
      if (endType !== null) parts.push(`- ${ROUND_END[endType] ?? `end type ${endType}`}`);
      if (secs !== null) parts.push(`after ${fmtClock(secs)}`);
      parts.push(`· ${fmtTeam(0)} ${num(x.af) ?? 0} : ${num(x.rsa) ?? 0} ${fmtTeam(1)}`);
      return parts.join(' ');
    }
    case 'bomb_planted':
    case 'bomb_defused': {
      const site = num(x.site);
      const verb = e.kind === 'bomb_planted' ? 'planted the bomb' : 'defused the bomb';
      const note = actorNote(x);
      return `${e.actor_name || `#${e.actor_uid}`} ${verb} at site ${site === null ? '?' : (SITE_LABEL[site] ?? site)}${note ? ` (${note})` : ''}`;
    }
    case 'point_taken': {
      const flag = num(x.flag);
      const takers = Array.isArray(x.takers) ? (x.takers as MatchEventActor[]) : [];
      const names = takers.map((t) => t.name || `#${t.uid}`);
      const who = names.length > 0 ? names.join(', ') : e.actor_name || '?';
      return `${who} took flag ${flag === null ? '?' : (SITE_LABEL[flag] ?? flag)}`;
    }
    case 'dom_score':
      return `${fmtTeam(0)} ${fmtInt(num(x.af) ?? 0)} : ${fmtInt(num(x.rsa) ?? 0)} ${fmtTeam(1)}`;
    case 'chopper_kill': {
      const caller = (x.caller ?? {}) as { name?: string };
      const assists = Array.isArray(x.assists) ? (x.assists as { name?: string; uid?: number }[]) : [];
      const damage = num(x.damage);
      let s = `${e.actor_name || `#${e.actor_uid}`} shot down ${caller.name ? `${caller.name}'s` : 'a'} chopper`;
      if (damage !== null) s += ` (${fmtInt(damage)} damage)`;
      if (assists.length > 0) s += `, assisted by ${assists.map((a) => a.name ?? `#${a.uid}`).join(', ')}`;
      return s;
    }
    case 'udp_kick':
      return `${e.actor_name || `#${e.actor_uid}`} was dropped from the host's UDP link by ${String(x.by_name ?? x.by ?? 'the host')}`;
    case 'host_lost':
      return `host ${e.actor_name || `#${e.actor_uid}`} lost while loading${x.err_msg ? ` (${String(x.err_msg)})` : ''}`;
    default:
      return JSON.stringify(x);
  }
}

const EVENT_COLUMNS: Column<MatchEvent>[] = [
  { key: 'at', header: 'Time', className: 'w-16 font-mono text-zinc-400', render: (e) => fmtClock(e.at_secs) },
  {
    key: 'kind',
    header: 'Event',
    className: 'w-32',
    render: (e) => {
      const b = EVENT_BADGE[e.kind] ?? { tone: 'neutral' as const, label: e.kind };
      return <Badge tone={b.tone}>{b.label}</Badge>;
    },
  },
  {
    key: 'team',
    header: 'Team',
    className: 'w-16',
    hideBelow: 'sm',
    render: (e) => (e.team === 0 || e.team === 1 ? <Badge tone={e.team === 0 ? 'accent' : 'warning'}>{fmtTeam(e.team)}</Badge> : <span className="text-zinc-600">-</span>),
  },
  { key: 'what', header: 'What happened', render: (e) => <span className="text-zinc-200">{describeEvent(e)}</span> },
];

function MatchView() {
  const params = useSearchParams();
  const id = Number(params.get('id') ?? '');
  const router = useRouter();
  const toast = useToast();
  const [match, setMatch] = useState<MatchDetail | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [confirmDelete, setConfirmDelete] = useState(false);

  const load = useCallback(async () => {
    if (!Number.isFinite(id) || id <= 0) {
      setError('No match id given. Open this page from the match list.');
      return;
    }
    setError(null);
    try {
      setMatch(await api.matches.get(id));
    } catch (err) {
      setError(errorMessage(err));
    }
  }, [id]);

  useEffect(() => {
    void load();
  }, [load]);

  const remove = async () => {
    try {
      await api.matches.remove(id);
      toast.success('Match deleted');
      router.push('/matches/');
    } catch (err) {
      toast.error(errorMessage(err));
    }
  };

  if (error) {
    return (
      <div>
        <BackLink />
        <ErrorBox message={error} onRetry={id > 0 ? load : undefined} />
      </div>
    );
  }
  if (!match) return <Loading />;

  const teams = [0, 1].map((t) => ({
    team: t,
    score: t === 0 ? match.team0_score : match.team1_score,
    players: match.scoreboard.filter((p) => p.team === t),
  }));
  const humans = match.scoreboard.filter((p) => !p.is_bot);
  const mvp = humans.length > 0 ? humans.reduce((best, p) => (p.score > best.score ? p : best), humans[0]) : null;
  const events = match.events ?? [];

  return (
    <div>
      <BackLink />
      <PageHeader
        title={`${match.map || 'Match'} · ${fmtMode(match.mode)}`}
        subtitle={
          <span>
            {fmtUtc(match.played_at)} · room &quot;{match.room_name}&quot; hosted by {match.host || '-'} · {fmtClock(match.duration)}
          </span>
        }
        actions={
          <Button variant="danger" size="sm" onClick={() => setConfirmDelete(true)}>
            <IconTrash className="h-4 w-4" /> Delete
          </Button>
        }
      />

      <div className="mb-6 grid grid-cols-2 gap-4 lg:grid-cols-4">
        <StatCard label={fmtTeam(0)} value={<span className="text-accent-300">{fmtInt(match.team0_score)}</span>} hint={`${teams[0].players.length} on the team`} />
        <StatCard label={fmtTeam(1)} value={<span className="text-amber-300">{fmtInt(match.team1_score)}</span>} hint={`${teams[1].players.length} on the team`} />
        <StatCard label="Outcome" value={<WinnerBadge winner={match.winner} />} hint={`end type ${match.end_type}`} />
        <StatCard label="Top scorer" value={mvp ? mvp.name : '-'} hint={mvp ? `${fmtInt(mvp.kills)} kills, ${fmtInt(mvp.score)} score` : 'no human players'} />
      </div>

      <div className="space-y-6">
        {teams.map((t) => (
          <Card
            key={t.team}
            flush
            title={
              <span className="inline-flex items-center gap-2">
                <Badge tone={t.team === 0 ? 'accent' : 'warning'}>{fmtTeam(t.team)}</Badge>
                <span>{fmtInt(t.score)} points</span>
                {match.winner === t.team && <Badge tone="success">winner</Badge>}
              </span>
            }
          >
            <DataTable columns={SCORE_COLUMNS} rows={t.players} rowKey={(p) => p.uid} emptyText="Nobody on this team." dense />
          </Card>
        ))}

        <Card flush title="Kill feed" description={`${fmtInt(match.kill_feed.length)} kills reported by the host`}>
          <DataTable columns={KILL_COLUMNS} rows={match.kill_feed} rowKey={(k) => k.seq} emptyText="No kills were reported." dense />
        </Card>

        <Card
          flush
          title="Match events"
          description={`${fmtInt(events.length)} objective events: round ends, bomb plants and defuses, points taken, score updates, chopper kills`}
        >
          <DataTable
            columns={EVENT_COLUMNS}
            rows={events}
            rowKey={(e) => e.id}
            emptyText="No objective events were reported (team deathmatch has none)."
            dense
          />
        </Card>
      </div>

      <ConfirmDialog
        open={confirmDelete}
        title="Delete this match?"
        message="The match, its scoreboard, kill feed and events are removed from the history. Career stats are not changed."
        confirmLabel="Delete"
        danger
        onCancel={() => setConfirmDelete(false)}
        onConfirm={() => {
          setConfirmDelete(false);
          void remove();
        }}
      />
    </div>
  );
}

function BackLink() {
  return (
    <Link href="/matches/" className="mb-4 inline-flex items-center gap-1.5 text-sm text-zinc-400 hover:text-zinc-100">
      <IconArrowLeft className="h-4 w-4" /> All matches
    </Link>
  );
}
