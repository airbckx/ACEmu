'use client';

import Link from 'next/link';
import { useRouter } from 'next/navigation';
import { useCallback, useEffect, useState } from 'react';
import { api, type BoxSummary } from '@/lib/api';
import { fmtInt } from '@/lib/format';
import { Card } from '@/components/Card';
import { ErrorBox, Loading, PageHeader } from '@/components/Feedback';
import { DataTable, type Column } from '@/components/Table';
import { errorMessage } from '@/components/Toast';
import { ItemIcon } from '@/components/ItemIcon';

function boxHref(id: number): string {
  return `/boxes/view/?id=${id}`;
}

const COLUMNS: Column<BoxSummary>[] = [
  { key: 'id', header: 'ID', className: 'w-16 text-zinc-500', render: (b) => b.id },
  {
    key: 'name',
    header: 'Name',
    render: (b) => (
      <span className="inline-flex items-center gap-2">
        <ItemIcon id={b.id} size="sm" />
        <Link href={boxHref(b.id)} className="font-medium text-accent-300 hover:underline" onClick={(e) => e.stopPropagation()}>
          {b.name}
        </Link>
      </span>
    ),
  },
  { key: 'main', header: 'Item reel', className: 'text-right', render: (b) => fmtInt(b.main_entries) },
  {
    key: 'bonus',
    header: 'Bonus reel',
    className: 'text-right',
    render: (b) => (b.bonus_entries > 0 ? fmtInt(b.bonus_entries) : <span className="text-zinc-600">-</span>),
  },
];

export default function BoxesPage() {
  const router = useRouter();
  const [boxes, setBoxes] = useState<BoxSummary[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setError(null);
    try {
      setBoxes(await api.boxes.list());
    } catch (err) {
      setError(errorMessage(err));
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  return (
    <div>
      <PageHeader
        title="Loot boxes"
        subtitle={boxes ? `${fmtInt(boxes.length)} box${boxes.length === 1 ? '' : 'es'} - drop tables the client rolls on` : undefined}
      />

      {error && <ErrorBox message={error} onRetry={load} />}
      {!error && !boxes && <Loading />}

      {boxes && (
        <Card flush title="All boxes" description="Entry counts per reel. Open a box to edit its drops and weights.">
          <DataTable columns={COLUMNS} rows={boxes} rowKey={(b) => b.id} onRowClick={(b) => router.push(boxHref(b.id))} emptyText="No loot boxes defined." />
        </Card>
      )}
    </div>
  );
}
