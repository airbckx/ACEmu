'use client';

import Link from 'next/link';
import { useCallback, useEffect, useMemo, useState } from 'react';
import { api, type LeaderRow } from '@/lib/api';
import { fmtDuration, fmtInt, fmtUtc, kdRatio } from '@/lib/format';
import { Badge } from '@/components/Badge';
import { Button } from '@/components/Button';
import { Card } from '@/components/Card';
import { ErrorBox, Loading, PageHeader } from '@/components/Feedback';
import { DataTable, type Column } from '@/components/Table';
import { errorMessage } from '@/components/Toast';

type Board = 'kills' | 'kd' | 'wins' | 'level' | 'headshots' | 'matches';

const BOARDS: { key: Board; label: string; hint: string }[] = [
  { key: 'kills', label: 'Kills', hint: 'career kills' },
  { key: 'kd', label: 'K/D', hint: 'kill/death ratio, at least one match played' },
  { key: 'wins', label: 'Wins', hint: 'matches won' },
  { key: 'level', label: 'Level', hint: 'level, then EXP' },
  { key: 'headshots', label: 'Headshots', hint: 'career headshots' },
  { key: 'matches', label: 'Matches', hint: 'recorded matches played' },
];

function kd(r: LeaderRow): number {
  return r.deaths === 0 ? r.kills : r.kills / r.deaths;
}

function rank(rows: LeaderRow[], board: Board): LeaderRow[] {
  const sorted = [...rows];
  const by = (f: (r: LeaderRow) => number, tie?: (r: LeaderRow) => number) => (a: LeaderRow, b: LeaderRow) =>
    f(b) - f(a) || (tie ? tie(b) - tie(a) : 0) || a.name.localeCompare(b.name);
  switch (board) {
    case 'kills':
      return sorted.sort(by((r) => r.kills, (r) => -r.deaths));
    case 'kd':
      return sorted.filter((r) => r.matches > 0 || r.kills + r.deaths > 0).sort(by(kd, (r) => r.kills));
    case 'wins':
      return sorted.sort(by((r) => r.wins, (r) => -r.losses));
    case 'level':
      return sorted.sort(by((r) => r.level, (r) => r.exp));
    case 'headshots':
      return sorted.sort(by((r) => r.headshots, (r) => r.kills));
    case 'matches':
      return sorted.sort(by((r) => r.matches, (r) => r.kills));
  }
}

function playerHref(name: string): string {
  return `/players/view/?name=${encodeURIComponent(name)}`;
}

function RankBadge({ rank }: { rank: number }) {
  const tone = rank === 1 ? 'warning' : rank === 2 ? 'neutral' : rank === 3 ? 'accent' : 'neutral';
  return (
    <Badge tone={tone} className="w-9 justify-center font-mono">
      #{rank}
    </Badge>
  );
}

export default function LeaderboardPage() {
  const [rows, setRows] = useState<LeaderRow[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [board, setBoard] = useState<Board>('kills');

  const load = useCallback(async () => {
    setError(null);
    try {
      setRows(await api.leaderboard());
    } catch (err) {
      setError(errorMessage(err));
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  const ranked = useMemo(() => (rows ? rank(rows, board) : []), [rows, board]);

  const columns: Column<LeaderRow>[] = [
    { key: 'rank', header: '#', className: 'w-14', render: (_r, i) => <RankBadge rank={i + 1} /> },
    {
      key: 'name',
      header: 'Player',
      render: (r) => (
        <Link href={playerHref(r.name)} className="font-medium text-accent-300 hover:underline">
          {r.name}
        </Link>
      ),
    },
    { key: 'level', header: 'Lvl', className: `text-right ${board === 'level' ? 'text-zinc-50' : ''}`, render: (r) => fmtInt(r.level) },
    { key: 'kills', header: 'Kills', className: `text-right ${board === 'kills' ? 'text-zinc-50' : ''}`, render: (r) => fmtInt(r.kills) },
    { key: 'deaths', header: 'Deaths', className: 'text-right', hideBelow: 'sm', render: (r) => fmtInt(r.deaths) },
    { key: 'kd', header: 'K/D', className: `text-right ${board === 'kd' ? 'text-zinc-50' : ''}`, render: (r) => kdRatio(r.kills, r.deaths) },
    { key: 'hs', header: 'HS', className: `text-right ${board === 'headshots' ? 'text-zinc-50' : ''}`, hideBelow: 'md', render: (r) => fmtInt(r.headshots) },
    {
      key: 'wl',
      header: 'W / L',
      className: `text-right ${board === 'wins' ? 'text-zinc-50' : ''}`,
      hideBelow: 'sm',
      render: (r) => `${fmtInt(r.wins)} / ${fmtInt(r.losses)}`,
    },
    {
      key: 'matches',
      header: 'Matches',
      className: `text-right ${board === 'matches' ? 'text-zinc-50' : ''}`,
      hideBelow: 'md',
      render: (r) => (
        <span title={r.best_kills > 0 ? `best: ${fmtInt(r.best_kills)} kills in one match` : undefined}>{fmtInt(r.matches)}</span>
      ),
    },
    { key: 'time', header: 'Play time', className: 'text-right', hideBelow: 'lg', render: (r) => fmtDuration(r.play_time) },
    {
      key: 'last',
      header: 'Last match',
      hideBelow: 'lg',
      render: (r) => (r.last_played ? <span className="text-zinc-400">{fmtUtc(r.last_played)}</span> : <span className="text-zinc-600">-</span>),
    },
  ];

  const current = BOARDS.find((b) => b.key === board)!;

  return (
    <div>
      <PageHeader title="Leaderboard" subtitle="Career stats of every saved player, ranked by the selected category." />

      {error && <ErrorBox message={error} onRetry={load} />}
      {!error && !rows && <Loading />}

      {rows && (
        <Card
          flush
          title={`By ${current.label.toLowerCase()}`}
          description={current.hint}
          actions={
            <div className="flex flex-wrap gap-1.5">
              {BOARDS.map((b) => (
                <Button key={b.key} size="sm" variant={b.key === board ? 'primary' : 'secondary'} onClick={() => setBoard(b.key)}>
                  {b.label}
                </Button>
              ))}
            </div>
          }
        >
          <DataTable
            columns={columns}
            rows={ranked}
            rowKey={(r) => r.name}
            emptyText={board === 'kd' ? 'Nobody has played a match yet.' : 'No players saved yet.'}
          />
        </Card>
      )}
    </div>
  );
}
