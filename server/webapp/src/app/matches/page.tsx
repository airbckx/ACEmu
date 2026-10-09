'use client';

import Link from 'next/link';
import { useCallback, useEffect, useState } from 'react';
import { api, type MatchPage, type MatchSummary } from '@/lib/api';
import { fmtClock, fmtInt, fmtMode, fmtUtc } from '@/lib/format';
import { Button } from '@/components/Button';
import { Card } from '@/components/Card';
import { ErrorBox, Loading, PageHeader } from '@/components/Feedback';
import { Input } from '@/components/Input';
import { matchHref, WinnerBadge } from '@/components/MatchBits';
import { DataTable, type Column } from '@/components/Table';
import { errorMessage } from '@/components/Toast';

const PAGE = 25;

const COLUMNS: Column<MatchSummary>[] = [
  {
    key: 'played_at',
    header: 'Played',
    render: (m) => (
      <Link href={matchHref(m.id)} className="font-medium text-accent-300 hover:underline">
        {fmtUtc(m.played_at)}
      </Link>
    ),
  },
  { key: 'map', header: 'Map', render: (m) => <span className="text-zinc-200">{m.map || '-'}</span> },
  { key: 'mode', header: 'Mode', hideBelow: 'sm', render: (m) => fmtMode(m.mode) },
  {
    key: 'score',
    header: 'Score',
    className: 'text-right',
    render: (m) => (
      <span className="font-mono">
        <span className="text-accent-300">{fmtInt(m.team0_score)}</span>
        <span className="text-zinc-500"> : </span>
        <span className="text-amber-300">{fmtInt(m.team1_score)}</span>
      </span>
    ),
  },
  { key: 'winner', header: 'Outcome', render: (m) => <WinnerBadge winner={m.winner} /> },
  {
    key: 'players',
    header: 'Players',
    hideBelow: 'md',
    render: (m) => (
      <span title={m.names.join(', ')}>
        {m.names.slice(0, 4).join(', ')}
        {m.names.length > 4 ? ` +${m.names.length - 4}` : ''}
        {m.bots > 0 && <span className="text-zinc-500"> · {fmtInt(m.bots)} bots</span>}
      </span>
    ),
  },
  { key: 'host', header: 'Host', hideBelow: 'lg', render: (m) => m.host || '-' },
  { key: 'duration', header: 'Length', className: 'text-right', hideBelow: 'sm', render: (m) => fmtClock(m.duration) },
];

export default function MatchesPage() {
  const [page, setPage] = useState<MatchPage | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [offset, setOffset] = useState(0);
  const [player, setPlayer] = useState('');
  const [applied, setApplied] = useState('');

  const load = useCallback(async () => {
    setError(null);
    try {
      setPage(await api.matches.list(PAGE, offset, applied || undefined));
    } catch (err) {
      setError(errorMessage(err));
    }
  }, [offset, applied]);

  useEffect(() => {
    void load();
  }, [load]);

  const total = page?.total ?? 0;
  const from = total === 0 ? 0 : offset + 1;
  const to = Math.min(offset + PAGE, total);

  return (
    <div>
      <PageHeader
        title="Match history"
        subtitle={page ? `${fmtInt(total)} recorded match${total === 1 ? '' : 'es'}` : undefined}
        actions={
          <form
            className="flex items-center gap-2"
            onSubmit={(e) => {
              e.preventDefault();
              setOffset(0);
              setApplied(player.trim());
            }}
          >
            <div className="w-48 sm:w-64">
              <Input type="search" placeholder="Filter by player name" value={player} onChange={(e) => setPlayer(e.target.value)} aria-label="Filter by player" />
            </div>
            <Button type="submit" size="sm">
              Filter
            </Button>
          </form>
        }
      />

      {error && <ErrorBox message={error} onRetry={load} />}
      {!error && !page && <Loading />}

      {page && (
        <Card
          flush
          title={applied ? `Matches of ${applied}` : 'All matches'}
          description="Newest first. A match is recorded when the host reports the final score table."
          actions={
            <div className="flex items-center gap-2 text-xs text-zinc-400">
              <span>
                {fmtInt(from)}-{fmtInt(to)} of {fmtInt(total)}
              </span>
              <Button size="sm" disabled={offset === 0} onClick={() => setOffset(Math.max(0, offset - PAGE))}>
                Newer
              </Button>
              <Button size="sm" disabled={offset + PAGE >= total} onClick={() => setOffset(offset + PAGE)}>
                Older
              </Button>
            </div>
          }
        >
          <DataTable
            columns={COLUMNS}
            rows={page.matches}
            rowKey={(m) => m.id}
            emptyText={applied ? `No recorded matches with ${applied}.` : 'No matches recorded yet. Finish a match and it shows up here.'}
          />
        </Card>
      )}
    </div>
  );
}
