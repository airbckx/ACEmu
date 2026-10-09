'use client';

import Link from 'next/link';
import { useRouter, useSearchParams } from 'next/navigation';
import { Suspense, useEffect, useMemo, useState, type FormEvent } from 'react';
import { api, type Availability, type CatalogItem, type Kind } from '@/lib/api';
import { AVAILABILITY_LABEL, fmtInt, fmtPrices } from '@/lib/format';
import { useDebounced } from '@/lib/hooks';
import { AvailabilityBadge, Badge } from '@/components/Badge';
import { Button } from '@/components/Button';
import { Card } from '@/components/Card';
import { ErrorBox, Loading, PageHeader } from '@/components/Feedback';
import { Input, Select } from '@/components/Input';
import { Modal } from '@/components/Modal';
import { DataTable, type Column } from '@/components/Table';
import { errorMessage, useToast } from '@/components/Toast';
import { ItemIcon } from '@/components/ItemIcon';

const PAGE_SIZE = 50;
const AVAILABILITIES: Availability[] = ['shop', 'lootbox', 'hidden'];

/**
 * Item types, which decide the inventory slot an item takes. The labelled ones
 * are confirmed; the bare numbers also occur in the game's own table and are
 * offered as-is.
 */
const ITEM_TYPES: { value: number; label: string }[] = [
  { value: 1, label: '1 - Primary weapon' },
  { value: 2, label: '2 - Sidearm' },
  { value: 3, label: '3 - Knife' },
  { value: 5, label: '5 - Throwable' },
  { value: 11, label: '11 - Melee strike' },
  { value: 15, label: '15 - Skill' },
  { value: 16, label: '16 - Consumable / coupon' },
  { value: 0, label: '0' },
  { value: 6, label: '6' },
  { value: 7, label: '7' },
];

export default function ItemsPage() {
  return (
    <Suspense fallback={<Loading />}>
      <ItemsBrowser />
    </Suspense>
  );
}

function ItemsBrowser() {
  const router = useRouter();
  const params = useSearchParams();

  // Initial filter values can be passed via the query string (e.g. from the dashboard).
  const [query, setQuery] = useState(params.get('q') ?? '');
  const [kind, setKind] = useState(params.get('kind') ?? '');
  const [availability, setAvailability] = useState(params.get('availability') ?? '');
  const [offset, setOffset] = useState(0);

  const debouncedQuery = useDebounced(query, 300);

  const [kinds, setKinds] = useState<Kind[]>([]);
  const [page, setPage] = useState<{ total: number; items: CatalogItem[] } | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [reloadTick, setReloadTick] = useState(0);
  const [adding, setAdding] = useState(false);

  useEffect(() => {
    api.kinds().then(setKinds).catch(() => setKinds([]));
  }, []);

  // Reset paging whenever a filter changes.
  useEffect(() => {
    setOffset(0);
  }, [debouncedQuery, kind, availability]);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    setError(null);
    api.items
      .list({ q: debouncedQuery.trim(), kind, availability, limit: PAGE_SIZE, offset })
      .then((p) => {
        if (!cancelled) setPage(p);
      })
      .catch((err) => {
        if (!cancelled) setError(errorMessage(err));
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [debouncedQuery, kind, availability, offset, reloadTick]);

  const kindLabel = useMemo(() => new Map(kinds.map((k) => [k.kind, k.label])), [kinds]);

  const columns: Column<CatalogItem>[] = [
    { key: 'id', header: 'ID', className: 'w-16 text-zinc-500', render: (it) => it.id },
    {
      key: 'name',
      header: 'Name',
      render: (it) => (
        <span className="inline-flex items-center gap-2">
          <ItemIcon id={it.id} size="sm" />
          <Link
            href={`/items/view/?id=${it.id}`}
            className="font-medium text-zinc-100 hover:text-accent-300 hover:underline"
            onClick={(e) => e.stopPropagation()}
          >
            {it.name}
          </Link>
        </span>
      ),
    },
    {
      key: 'kind',
      header: 'Kind',
      hideBelow: 'sm',
      render: (it) => <Badge tone="neutral">{kindLabel.get(it.kind) ?? `kind ${it.kind}`}</Badge>,
    },
    {
      key: 'category',
      header: 'Category',
      hideBelow: 'md',
      className: 'text-zinc-500',
      render: (it) => it.category || <span className="text-zinc-700">-</span>,
    },
    { key: 'type', header: 'Type', hideBelow: 'lg', className: 'text-zinc-400', render: (it) => it.item_type },
    { key: 'availability', header: 'Availability', render: (it) => <AvailabilityBadge value={it.availability} /> },
    {
      key: 'prices',
      header: 'Prices',
      hideBelow: 'md',
      render: (it) => <span className="text-zinc-300">{fmtPrices(it.prices)}</span>,
    },
    { key: 'sell', header: 'Sell', hideBelow: 'lg', className: 'text-right text-zinc-400', render: (it) => fmtInt(it.sell_price) },
  ];

  const total = page?.total ?? 0;
  const from = total === 0 ? 0 : offset + 1;
  const to = Math.min(offset + PAGE_SIZE, total);
  const hasFilter = query !== '' || kind !== '' || availability !== '';

  return (
    <div>
      <PageHeader
        title="Item catalog"
        subtitle={page ? `${fmtInt(page.total)} item${page.total === 1 ? '' : 's'} match` : undefined}
        actions={
          <Button variant="primary" onClick={() => setAdding(true)}>
            Add item
          </Button>
        }
      />

      <AddItemDialog
        open={adding}
        onClose={() => setAdding(false)}
        onCreated={(it) => {
          setAdding(false);
          router.push(`/items/view/?id=${it.id}`);
        }}
      />

      <Card
        flush
        title="Catalog"
        actions={
          <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row sm:items-center">
            <div className="sm:w-56">
              <Input
                type="search"
                placeholder="Search name or id"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                aria-label="Search catalog"
              />
            </div>
            <Select value={kind} onChange={(e) => setKind(e.target.value)} aria-label="Kind" className="sm:w-40">
              <option value="">All kinds</option>
              {kinds.map((k) => (
                <option key={k.kind} value={String(k.kind)}>
                  {k.label}
                </option>
              ))}
            </Select>
            <Select value={availability} onChange={(e) => setAvailability(e.target.value)} aria-label="Availability" className="sm:w-40">
              <option value="">Any availability</option>
              {AVAILABILITIES.map((a) => (
                <option key={a} value={a}>
                  {AVAILABILITY_LABEL[a]}
                </option>
              ))}
            </Select>
            {hasFilter && (
              <Button
                variant="ghost"
                onClick={() => {
                  setQuery('');
                  setKind('');
                  setAvailability('');
                }}
              >
                Clear
              </Button>
            )}
          </div>
        }
      >
        {error && (
          <div className="p-4">
            <ErrorBox message={error} onRetry={() => setReloadTick((t) => t + 1)} />
          </div>
        )}
        {!error && !page && <Loading />}
        {page && (
          <div className={loading ? 'opacity-60 transition-opacity' : 'transition-opacity'}>
            <DataTable
              columns={columns}
              rows={page.items}
              rowKey={(it) => it.id}
              onRowClick={(it) => router.push(`/items/view/?id=${it.id}`)}
              emptyText="No catalog items match."
            />
            <div className="flex flex-wrap items-center justify-between gap-3 border-t border-zinc-800 px-4 py-3 text-sm text-zinc-400">
              <span>
                Showing {fmtInt(from)}-{fmtInt(to)} of {fmtInt(total)}
              </span>
              <div className="flex items-center gap-2">
                <Button size="sm" onClick={() => setOffset(Math.max(0, offset - PAGE_SIZE))} disabled={offset === 0 || loading}>
                  Previous
                </Button>
                <Button size="sm" onClick={() => setOffset(offset + PAGE_SIZE)} disabled={to >= total || loading}>
                  Next
                </Button>
              </div>
            </div>
          </div>
        )}
      </Card>
    </div>
  );
}

/**
 * Adds a catalogue row by hand. Mainly for servers running without the game's
 * own shop table: the client already knows the item's name, model and icon, so
 * an id, a type and a price are enough to put it in the shop.
 */
function AddItemDialog({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: (it: CatalogItem) => void }) {
  const toast = useToast();
  const [id, setId] = useState('');
  const [name, setName] = useState('');
  const [itemType, setItemType] = useState(1);
  const [availability, setAvailability] = useState<Availability>('shop');
  const [sellPrice, setSellPrice] = useState(0);
  const [hours, setHours] = useState(0);
  const [points, setPoints] = useState(0);
  const [cash, setCash] = useState(0);
  const [busy, setBusy] = useState(false);

  const numericId = Number(id);
  const valid = Number.isInteger(numericId) && numericId > 0;

  const submit = async (e: FormEvent) => {
    e.preventDefault();
    if (!valid) return;
    setBusy(true);
    try {
      const created = await api.items.create({
        id: numericId,
        name: name.trim(),
        item_type: itemType,
        availability,
        sell_price: sellPrice,
        prices: points > 0 || cash > 0 ? [{ hours, points, cash }] : [],
      });
      toast.success(`${created.name} added to the catalog`);
      setId('');
      setName('');
      setPoints(0);
      setCash(0);
      onCreated(created);
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal open={open} title="Add a catalog item" onClose={onClose}>
      <form onSubmit={submit} className="space-y-4">
        <p className="text-xs text-zinc-500">
          The id has to be one the game client knows - it supplies the name, the model and the icon. The name here is only used by this
          app and the logs.
        </p>
        <div className="grid gap-3 sm:grid-cols-2">
          <Input
            id="new-item-id"
            label="Item id"
            type="number"
            min={1}
            value={id}
            onChange={(e) => setId(e.target.value)}
            autoComplete="off"
            required
          />
          <Select id="new-item-type" label="Type" value={String(itemType)} onChange={(e) => setItemType(Number(e.target.value))}>
            {ITEM_TYPES.map((t) => (
              <option key={t.value} value={String(t.value)}>
                {t.label}
              </option>
            ))}
          </Select>
        </div>
        <Input
          id="new-item-name"
          label="Name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder={valid ? `Item ${numericId}` : 'Item 10100'}
          hint="Optional"
        />
        <div className="grid gap-3 sm:grid-cols-2">
          <Select
            id="new-item-availability"
            label="Availability"
            value={availability}
            onChange={(e) => setAvailability(e.target.value as Availability)}
          >
            {AVAILABILITIES.map((a) => (
              <option key={a} value={a}>
                {AVAILABILITY_LABEL[a]}
              </option>
            ))}
          </Select>
          <Input
            id="new-item-sell"
            label="Resale value"
            type="number"
            min={0}
            value={sellPrice}
            onChange={(e) => setSellPrice(Math.max(0, Number(e.target.value)))}
            hint="0 makes it unsellable"
          />
        </div>
        <div className="grid gap-3 sm:grid-cols-3">
          <Input
            id="new-item-hours"
            label="Rental hours"
            type="number"
            min={0}
            value={hours}
            onChange={(e) => setHours(Math.max(0, Number(e.target.value)))}
            hint="0 = permanent"
          />
          <Input
            id="new-item-points"
            label="Points"
            type="number"
            min={0}
            value={points}
            onChange={(e) => setPoints(Math.max(0, Number(e.target.value)))}
          />
          <Input
            id="new-item-cash"
            label="W Coin"
            type="number"
            min={0}
            value={cash}
            onChange={(e) => setCash(Math.max(0, Number(e.target.value)))}
          />
        </div>
        <div className="flex justify-end gap-2">
          <Button type="button" onClick={onClose}>
            Cancel
          </Button>
          <Button type="submit" variant="primary" loading={busy} disabled={!valid}>
            Add item
          </Button>
        </div>
      </form>
    </Modal>
  );
}
