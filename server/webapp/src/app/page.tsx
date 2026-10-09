'use client';

import Link from 'next/link';
import { useCallback, useEffect, useState } from 'react';
import { api, type Summary } from '@/lib/api';
import { fmtInt } from '@/lib/format';
import { Card, StatCard } from '@/components/Card';
import { ErrorBox, Loading, PageHeader } from '@/components/Feedback';
import { IconAccounts, IconHistory, IconItems, IconPlayers, IconShop } from '@/components/Icons';
import { LivePanel } from '@/components/LivePanel';
import { ServersPanel } from '@/components/ServersPanel';
import { errorMessage } from '@/components/Toast';

const QUICK_LINKS: { href: string; title: string; text: string }[] = [
  { href: '/players/', title: 'Players', text: 'Balances, stats, inventory, skills and quick slots of every saved character.' },
  { href: '/matches/', title: 'Match history', text: 'Every finished match with its scoreboard and kill feed; the leaderboard ranks the players.' },
  { href: '/accounts/', title: 'Accounts', text: 'Login accounts, passwords and which character each one plays.' },
  { href: '/items/', title: 'Item catalog', text: 'Shop availability and price options for every item the client knows.' },
];

export default function DashboardPage() {
  const [summary, setSummary] = useState<Summary | null>(null);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setError(null);
    try {
      setSummary(await api.summary());
    } catch (err) {
      setError(errorMessage(err));
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  return (
    <div>
      <PageHeader title="Dashboard" subtitle="Which services are up, the live game-server state and an overview of the saved data." />

      <div className="mb-8">
        <ServersPanel />
      </div>

      <div className="mb-8">
        <LivePanel />
      </div>

      <h2 className="mb-3 text-base font-semibold text-zinc-100">Saved data</h2>

      {error && <ErrorBox message={error} onRetry={load} />}
      {!error && !summary && <Loading />}

      {summary && (
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-6">
          <Link href="/players/" className="block">
            <StatCard label="Players" value={fmtInt(summary.players)} icon={<IconPlayers className="h-6 w-6" />} />
          </Link>
          <Link href="/matches/" className="block">
            <StatCard label="Matches" value={fmtInt(summary.matches)} hint="recorded" icon={<IconHistory className="h-6 w-6" />} />
          </Link>
          <Link href="/accounts/" className="block">
            <StatCard label="Accounts" value={fmtInt(summary.accounts)} icon={<IconAccounts className="h-6 w-6" />} />
          </Link>
          <StatCard label="Owned items" value={fmtInt(summary.items)} hint="across all players" icon={<IconItems className="h-6 w-6" />} />
          <Link href="/items/" className="block">
            <StatCard label="Catalog items" value={fmtInt(summary.catalog)} icon={<IconItems className="h-6 w-6" />} />
          </Link>
          <Link href="/items/?availability=shop" className="block">
            <StatCard label="In shop" value={fmtInt(summary.shop)} hint="items purchasable in-game" icon={<IconShop className="h-6 w-6" />} />
          </Link>
        </div>
      )}

      <div className="mt-8 grid grid-cols-1 gap-4 md:grid-cols-3">
        {QUICK_LINKS.map((q) => (
          <Link key={q.href} href={q.href} className="group block">
            <Card className="h-full transition-colors group-hover:border-accent-500/50">
              <div className="text-sm font-semibold text-zinc-100 group-hover:text-accent-300">{q.title}</div>
              <p className="mt-1 text-sm text-zinc-400">{q.text}</p>
            </Card>
          </Link>
        ))}
      </div>
    </div>
  );
}
