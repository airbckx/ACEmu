'use client';

import Link from 'next/link';
import { useSearchParams } from 'next/navigation';
import { Suspense, useCallback, useEffect, useMemo, useState } from 'react';
import { api, type Availability, type CatalogItem, type Kind } from '@/lib/api';
import { AVAILABILITY_LABEL, fmtHours, fmtInt } from '@/lib/format';
import { AvailabilityBadge, Badge } from '@/components/Badge';
import { Button } from '@/components/Button';
import { Card } from '@/components/Card';
import { ErrorBox, Loading, PageHeader } from '@/components/Feedback';
import { IconArrowLeft, IconPlus, IconTrash } from '@/components/Icons';
import { Input } from '@/components/Input';
import { errorMessage, useToast } from '@/components/Toast';
import { ItemIcon } from '@/components/ItemIcon';

export default function ItemViewPage() {
  return (
    <Suspense fallback={<Loading />}>
      <ItemView />
    </Suspense>
  );
}

interface PriceDraft {
  key: number;
  hours: string;
  points: string;
  cash: string;
}

interface Draft {
  name: string;
  sell_price: string;
  availability: Availability;
  prices: PriceDraft[];
}

let priceKey = 1;

function toDraft(item: CatalogItem): Draft {
  return {
    name: item.name,
    sell_price: String(item.sell_price),
    availability: item.availability,
    prices: item.prices.map((p) => ({ key: priceKey++, hours: String(p.hours), points: String(p.points), cash: String(p.cash) })),
  };
}

const AVAILABILITY_OPTIONS: { value: Availability; help: string }[] = [
  { value: 'shop', help: 'Listed in the in-game shop with the price options below.' },
  { value: 'lootbox', help: 'Not sold directly; only obtainable from lootboxes.' },
  { value: 'hidden', help: 'Never shown to players.' },
];

function ItemView() {
  const params = useSearchParams();
  const idParam = params.get('id') ?? '';
  const id = Number(idParam);
  const toast = useToast();

  const [item, setItem] = useState<CatalogItem | null>(null);
  const [kinds, setKinds] = useState<Kind[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [draft, setDraft] = useState<Draft | null>(null);
  const [busy, setBusy] = useState(false);

  const load = useCallback(async () => {
    if (idParam === '' || !Number.isInteger(id)) {
      setError('No item id given. Open this page from the catalog.');
      return;
    }
    setError(null);
    try {
      const it = await api.items.get(id);
      setItem(it);
      setDraft(toDraft(it));
    } catch (err) {
      setError(errorMessage(err));
    }
  }, [id, idParam]);

  useEffect(() => {
    void load();
  }, [load]);

  useEffect(() => {
    api.kinds().then(setKinds).catch(() => setKinds([]));
  }, []);

  const kindLabel = useMemo(() => new Map(kinds.map((k) => [k.kind, k.label])), [kinds]);

  if (error) {
    return (
      <div>
        <BackLink />
        <ErrorBox message={error} onRetry={idParam ? load : undefined} />
      </div>
    );
  }
  if (!item || !draft) return <Loading />;

  // Validation ------------------------------------------------------------
  const nameValid = draft.name.trim() !== '';
  const sellPrice = Number(draft.sell_price);
  const sellValid = draft.sell_price.trim() !== '' && Number.isInteger(sellPrice) && sellPrice >= 0;
  const parsedPrices = draft.prices.map((p) => ({
    hours: Number(p.hours),
    points: Number(p.points),
    cash: Number(p.cash),
  }));
  const pricesValid = draft.prices.every(
    (p, i) =>
      p.hours.trim() !== '' &&
      p.points.trim() !== '' &&
      p.cash.trim() !== '' &&
      Number.isInteger(parsedPrices[i].hours) &&
      Number.isInteger(parsedPrices[i].points) &&
      Number.isInteger(parsedPrices[i].cash) &&
      parsedPrices[i].hours >= 0 &&
      parsedPrices[i].points >= 0 &&
      parsedPrices[i].cash >= 0,
  );
  const valid = nameValid && sellValid && pricesValid;

  const pricesChanged =
    parsedPrices.length !== item.prices.length ||
    parsedPrices.some((p, i) => p.hours !== item.prices[i].hours || p.points !== item.prices[i].points || p.cash !== item.prices[i].cash);
  const dirty =
    draft.name.trim() !== item.name || sellPrice !== item.sell_price || draft.availability !== item.availability || pricesChanged;

  // Actions ---------------------------------------------------------------
  const save = async () => {
    if (!valid || !dirty) return;
    setBusy(true);
    try {
      const body: Parameters<typeof api.items.update>[1] = {};
      if (draft.name.trim() !== item.name) body.name = draft.name.trim();
      if (sellPrice !== item.sell_price) body.sell_price = sellPrice;
      if (draft.availability !== item.availability) body.availability = draft.availability;
      if (pricesChanged) body.prices = parsedPrices;
      const updated = await api.items.update(item.id, body);
      setItem(updated);
      setDraft(toDraft(updated));
      toast.success(`${updated.name} saved`);
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setBusy(false);
    }
  };

  const setPrice = (key: number, field: 'hours' | 'points' | 'cash', value: string) =>
    setDraft((d) => (d ? { ...d, prices: d.prices.map((p) => (p.key === key ? { ...p, [field]: value } : p)) } : d));

  const addPrice = () =>
    setDraft((d) => (d ? { ...d, prices: [...d.prices, { key: priceKey++, hours: '0', points: '0', cash: '0' }] } : d));

  const removePrice = (key: number) => setDraft((d) => (d ? { ...d, prices: d.prices.filter((p) => p.key !== key) } : d));

  return (
    <div>
      <BackLink />
      <PageHeader
        title={
          <span className="flex flex-wrap items-center gap-4">
            <ItemIcon id={item.id} size="md" alt={item.name} />
            <span>{item.name}</span>
          </span>
        }
        subtitle={
          <span className="flex flex-wrap items-center gap-2">
            <span className="text-zinc-500">#{item.id}</span>
            <Badge tone="neutral">{kindLabel.get(item.kind) ?? `kind ${item.kind}`}</Badge>
            {item.category && <span className="text-zinc-500">{item.category}</span>}
            <span className="text-zinc-500">type {item.item_type}</span>
            <AvailabilityBadge value={item.availability} />
          </span>
        }
        actions={
          <>
            <Button variant="ghost" onClick={() => setDraft(toDraft(item))} disabled={!dirty || busy}>
              Reset
            </Button>
            <Button variant="primary" onClick={save} disabled={!dirty || !valid} loading={busy}>
              Save changes
            </Button>
          </>
        }
      />

      <form
        onSubmit={(e) => {
          e.preventDefault();
          void save();
        }}
        className="grid grid-cols-1 gap-6 lg:grid-cols-3"
      >
        <Card title="Picture" description="The client's shop icon at full size." className="lg:col-span-3">
          <div className="flex justify-center">
            <ItemIcon id={item.id} size="hero" alt={item.name} />
          </div>
        </Card>
        <Card title="Details" className="lg:col-span-2">
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
            <div className="sm:col-span-2">
              <Input
                id="item-name"
                label="Name"
                value={draft.name}
                onChange={(e) => setDraft({ ...draft, name: e.target.value })}
                className={!nameValid ? 'border-rose-500' : ''}
              />
            </div>
            <Input
              id="item-sell"
              label="Sell price"
              type="number"
              min={0}
              inputMode="numeric"
              value={draft.sell_price}
              onChange={(e) => setDraft({ ...draft, sell_price: e.target.value })}
              className={!sellValid ? 'border-rose-500' : ''}
            />
          </div>

          <fieldset className="mt-5">
            <legend className="mb-2 text-xs font-medium uppercase tracking-wide text-zinc-400">Availability</legend>
            <div className="grid grid-cols-1 gap-2 sm:grid-cols-3">
              {AVAILABILITY_OPTIONS.map((opt) => {
                const active = draft.availability === opt.value;
                return (
                  <label
                    key={opt.value}
                    className={`flex cursor-pointer items-start gap-2 rounded-md border p-3 text-sm transition-colors ${
                      active ? 'border-accent-500 bg-accent-500/10' : 'border-zinc-700 hover:border-zinc-600'
                    }`}
                  >
                    <input
                      type="radio"
                      name="availability"
                      value={opt.value}
                      checked={active}
                      onChange={() => setDraft({ ...draft, availability: opt.value })}
                      className="mt-0.5 h-4 w-4 border-zinc-600 bg-zinc-900 text-accent-500 focus:ring-accent-500 focus:ring-offset-zinc-950"
                    />
                    <span>
                      <span className="block font-medium text-zinc-100">{AVAILABILITY_LABEL[opt.value]}</span>
                      <span className="block text-xs text-zinc-500">{opt.help}</span>
                    </span>
                  </label>
                );
              })}
            </div>
          </fieldset>
        </Card>

        <Card title="Catalog info" description="Read-only values from the item tables.">
          <dl className="space-y-2 text-sm">
            <Row label="Item id" value={item.id} />
            <Row label="Item type" value={item.item_type} />
            <Row label="Kind" value={kindLabel.get(item.kind) ?? item.kind} />
            <Row label="Category" value={item.category || '-'} />
            <Row label="Shop order" value={item.shop_order} />
            <Row label="In shop table" value={item.in_shop_table ? 'yes' : 'no'} />
            <Row label="Current sell price" value={fmtInt(item.sell_price)} />
          </dl>
        </Card>

        <Card
          title="Price options"
          description="Each row is one purchase option the client shows. Hours 0 = permanent."
          className="lg:col-span-3"
          actions={
            <Button size="sm" onClick={addPrice}>
              <IconPlus className="h-4 w-4" />
              Add row
            </Button>
          }
        >
          {draft.prices.length === 0 ? (
            <p className="text-sm text-zinc-500">No price options. The item cannot be bought even if it is in the shop.</p>
          ) : (
            <div className="space-y-2">
              <div className="hidden grid-cols-[1fr_1fr_1fr_auto_auto] gap-3 px-1 text-xs font-medium uppercase tracking-wide text-zinc-400 sm:grid">
                <span>Hours</span>
                <span>Points</span>
                <span>Cash</span>
                <span className="w-24">Duration</span>
                <span className="w-9" />
              </div>
              {draft.prices.map((p, i) => {
                const hours = Number(p.hours);
                return (
                  <div key={p.key} className="grid grid-cols-2 items-end gap-3 rounded-md border border-zinc-800 p-3 sm:grid-cols-[1fr_1fr_1fr_auto_auto] sm:border-0 sm:p-0 sm:px-1">
                    <Input label={<span className="sm:hidden">Hours</span>} type="number" min={0} inputMode="numeric" value={p.hours} onChange={(e) => setPrice(p.key, 'hours', e.target.value)} aria-label={`Row ${i + 1} hours`} />
                    <Input label={<span className="sm:hidden">Points</span>} type="number" min={0} inputMode="numeric" value={p.points} onChange={(e) => setPrice(p.key, 'points', e.target.value)} aria-label={`Row ${i + 1} points`} />
                    <Input label={<span className="sm:hidden">Cash</span>} type="number" min={0} inputMode="numeric" value={p.cash} onChange={(e) => setPrice(p.key, 'cash', e.target.value)} aria-label={`Row ${i + 1} cash`} />
                    <div className="flex h-9 w-24 items-center text-sm text-zinc-400">
                      {Number.isFinite(hours) && hours >= 0 ? fmtHours(hours) : '?'}
                    </div>
                    <Button size="sm" variant="ghost" className="h-9 w-9 text-rose-300 hover:bg-rose-500/10" onClick={() => removePrice(p.key)} aria-label={`Remove row ${i + 1}`}>
                      <IconTrash className="h-4 w-4" />
                    </Button>
                  </div>
                );
              })}
            </div>
          )}
          {!pricesValid && <p className="mt-3 text-xs text-rose-300">Every price field must be a whole number of 0 or more.</p>}
        </Card>

        <button type="submit" className="hidden" aria-hidden="true" tabIndex={-1} />
      </form>
    </div>
  );
}

function Row({ label, value }: { label: string; value: string | number }) {
  return (
    <div className="flex justify-between gap-4">
      <dt className="text-zinc-500">{label}</dt>
      <dd className="text-zinc-200">{value}</dd>
    </div>
  );
}

function BackLink() {
  return (
    <Link href="/items/" className="mb-4 inline-flex items-center gap-1 text-sm text-zinc-400 hover:text-zinc-100">
      <IconArrowLeft className="h-4 w-4" />
      Catalog
    </Link>
  );
}
