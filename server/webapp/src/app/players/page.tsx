'use client';

import Link from 'next/link';
import { useCallback, useEffect, useMemo, useState } from 'react';
import { api, type PlayerSummary } from '@/lib/api';
import { fmtDuration, fmtInt, kdRatio } from '@/lib/format';
import { Card } from '@/components/Card';
import { ErrorBox, Loading, PageHeader } from '@/components/Feedback';
import { Input } from '@/components/Input';
import { DataTable, type Column } from '@/components/Table';
import { errorMessage } from '@/components/Toast';

function playerHref(name: string): string {
  return `/players/view/?name=${encodeURIComponent(name)}`;
}

const COLUMNS: Column<PlayerSummary>[] = [
  {
    key: 'name',
    header: 'Name',
    render: (p) => (
      <Link href={playerHref(p.name)} className="font-medium text-accent-300 hover:underline">
        {p.name}
      </Link>
    ),
  },
  { key: 'level', header: 'Lvl', className: 'text-right', render: (p) => fmtInt(p.level) },
  { key: 'exp', header: 'EXP', className: 'text-right', hideBelow: 'md', render: (p) => fmtInt(p.exp) },
  { key: 'points', header: 'Points', className: 'text-right', render: (p) => fmtInt(p.points) },
  { key: 'cash', header: 'Cash', className: 'text-right', render: (p) => fmtInt(p.cash) },
  {
    key: 'kd',
    header: 'K / D',
    className: 'text-right',
    hideBelow: 'sm',
    render: (p) => (
      <span title={`${kdRatio(p.kills, p.deaths)} K/D`}>
        {fmtInt(p.kills)} / {fmtInt(p.deaths)}
      </span>
    ),
  },
  {
    key: 'wl',
    header: 'W / L',
    className: 'text-right',
    hideBelow: 'md',
    render: (p) => `${fmtInt(p.wins)} / ${fmtInt(p.losses)}`,
  },
  {
    key: 'play_time',
    header: 'Play time',
    className: 'text-right',
    hideBelow: 'lg',
    render: (p) => <span title={`${p.play_time}`}>{fmtDuration(p.play_time)}</span>,
  },
  { key: 'items', header: 'Items', className: 'text-right', hideBelow: 'sm', render: (p) => fmtInt(p.item_count) },
  {
    key: 'account',
    header: 'Account',
    hideBelow: 'md',
    render: (p) => (p.account ? <span className="text-zinc-300">{p.account}</span> : <span className="text-zinc-600">-</span>),
  },
];

export default function PlayersPage() {
  const [players, setPlayers] = useState<PlayerSummary[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [query, setQuery] = useState('');

  const load = useCallback(async () => {
    setError(null);
    try {
      setPlayers(await api.players.list());
    } catch (err) {
      setError(errorMessage(err));
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  const filtered = useMemo(() => {
    if (!players) return [];
    const q = query.trim().toLowerCase();
    if (!q) return players;
    return players.filter((p) => p.name.toLowerCase().includes(q) || (p.account ?? '').toLowerCase().includes(q));
  }, [players, query]);

  return (
    <div>
      <PageHeader
        title="Players"
        subtitle={players ? `${fmtInt(players.length)} saved character${players.length === 1 ? '' : 's'}` : undefined}
      />

      {error && <ErrorBox message={error} onRetry={load} />}
      {!error && !players && <Loading />}

      {players && (
        <Card
          flush
          title="All players"
          actions={
            <div className="w-full sm:w-64">
              <Input
                type="search"
                placeholder="Filter by name or account"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                aria-label="Filter players"
              />
            </div>
          }
        >
          <DataTable
            columns={COLUMNS}
            rows={filtered}
            rowKey={(p) => p.name}
            emptyText={query ? 'No players match the filter.' : 'No players saved yet. A character is created the first time a client logs in.'}
          />
        </Card>
      )}
    </div>
  );
}
