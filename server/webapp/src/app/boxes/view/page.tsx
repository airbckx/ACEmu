'use client';

import Link from 'next/link';
import { useSearchParams } from 'next/navigation';
import { Suspense, useCallback, useEffect, useState } from 'react';
import { api, type Box, type BoxEntry, type BoxEntryInput, type CatalogItem, type Reel } from '@/lib/api';
import { fmtHours, fmtInt } from '@/lib/format';
import { Badge } from '@/components/Badge';
import { Button } from '@/components/Button';
import { Card } from '@/components/Card';
import { CatalogSearch } from '@/components/CatalogSearch';
import { ErrorBox, Loading, PageHeader } from '@/components/Feedback';
import { IconArrowLeft, IconPlus, IconTrash } from '@/components/Icons';
import { Checkbox, Input } from '@/components/Input';
import { Modal } from '@/components/Modal';
import { errorMessage, useToast } from '@/components/Toast';
import { ItemIcon } from '@/components/ItemIcon';

export default function BoxViewPage() {
  return (
    <Suspense fallback={<Loading />}>
      <BoxView />
    </Suspense>
  );
}

// ---------------------------------------------------------------------------
// Draft model
// ---------------------------------------------------------------------------

interface EntryDraft {
  key: number;
  item_id: number;
  name: string;
  reel: Reel;
  weight: string;
  hours: string;
  featured: boolean;
}

let nextKey = 1;

function toDraft(entries: BoxEntry[]): EntryDraft[] {
  return entries.map((e) => ({
    key: nextKey++,
    item_id: e.item_id,
    name: e.name,
    reel: e.reel,
    weight: String(e.weight),
    hours: String(e.hours),
    featured: e.featured,
  }));
}

function parseWeight(s: string): number | null {
  if (s.trim() === '') return null;
  const n = Number(s);
  return Number.isFinite(n) && n >= 0 ? n : null;
}

function parseHours(s: string): number | null {
  if (s.trim() === '') return null;
  const n = Number(s);
  return Number.isInteger(n) && n >= 0 ? n : null;
}

/** Same formula as the server: weight / sum(weights in reel) * 100. */
function percentOf(weight: number, sum: number): number {
  return sum > 0 ? (weight / sum) * 100 : 0;
}

function fmtPercent(p: number): string {
  return `${p.toFixed(2)}%`;
}

const REEL_TITLE: Record<Reel, string> = { main: 'Item reel', bonus: 'Bonus reel' };
const REEL_DESC: Record<Reel, string> = {
  main: 'What the player wins. One entry is rolled per opening.',
  bonus: 'Second reel: vouchers or tickets awarded alongside the item. Leave empty for boxes without a bonus.',
};

// ---------------------------------------------------------------------------
// Page
// ---------------------------------------------------------------------------

function BoxView() {
  const params = useSearchParams();
  const idParam = params.get('id') ?? '';
  const id = Number(idParam);
  const toast = useToast();

  const [box, setBox] = useState<Box | null>(null);
  const [draft, setDraft] = useState<EntryDraft[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [adding, setAdding] = useState<Reel | null>(null);

  const load = useCallback(async () => {
    if (idParam === '' || !Number.isInteger(id)) {
      setError('No box id given. Open this page from the loot box list.');
      return;
    }
    setError(null);
    try {
      const b = await api.boxes.get(id);
      setBox(b);
      setDraft(toDraft(b.entries));
    } catch (err) {
      setError(errorMessage(err));
    }
  }, [id, idParam]);

  useEffect(() => {
    void load();
  }, [load]);

  if (error) {
    return (
      <div>
        <BackLink />
        <ErrorBox message={error} onRetry={idParam ? load : undefined} />
      </div>
    );
  }
  if (!box || !draft) return <Loading />;

  // Validation / dirty tracking ------------------------------------------
  const parsed: (BoxEntryInput | null)[] = draft.map((e) => {
    const weight = parseWeight(e.weight);
    const hours = parseHours(e.hours);
    if (weight === null || hours === null) return null;
    return { item_id: e.item_id, reel: e.reel, weight, featured: e.featured, hours };
  });
  const valid = parsed.every((p) => p !== null);

  const original: BoxEntryInput[] = box.entries.map((e) => ({ item_id: e.item_id, reel: e.reel, weight: e.weight, featured: e.featured, hours: e.hours }));
  const dirty =
    parsed.length !== original.length ||
    parsed.some((p, i) => {
      const o = original[i];
      return !p || p.item_id !== o.item_id || p.reel !== o.reel || p.weight !== o.weight || p.featured !== o.featured || p.hours !== o.hours;
    });

  // Actions ---------------------------------------------------------------
  const save = async () => {
    if (!valid || !dirty) return;
    setBusy(true);
    try {
      const updated = await api.boxes.update(box.id, { entries: parsed as BoxEntryInput[] });
      setBox(updated);
      setDraft(toDraft(updated.entries));
      toast.success(`${updated.name} saved`);
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setBusy(false);
    }
  };

  const update = (key: number, patch: Partial<EntryDraft>) => setDraft((d) => (d ? d.map((e) => (e.key === key ? { ...e, ...patch } : e)) : d));
  const remove = (key: number) => setDraft((d) => (d ? d.filter((e) => e.key !== key) : d));
  const equalise = (reel: Reel) => setDraft((d) => (d ? d.map((e) => (e.reel === reel ? { ...e, weight: '1' } : e)) : d));
  const add = (reel: Reel, item_id: number, name: string) => {
    setDraft((d) => (d ? [...d, { key: nextKey++, item_id, name, reel, weight: '1', hours: '0', featured: false }] : d));
    setAdding(null);
  };

  const reels: Reel[] = ['main', 'bonus'];

  return (
    <div>
      <BackLink />
      <PageHeader
        title={box.name}
        subtitle={
          <span className="flex flex-wrap items-center gap-2">
            <span className="text-zinc-500">box #{box.id}</span>
            <span className="text-zinc-500">{draft.filter((e) => e.reel === 'main').length} item entries</span>
            <span className="text-zinc-500">{draft.filter((e) => e.reel === 'bonus').length} bonus entries</span>
            {dirty && <Badge tone="warning">Unsaved changes</Badge>}
          </span>
        }
        actions={
          <>
            <Button variant="ghost" onClick={() => setDraft(toDraft(box.entries))} disabled={!dirty || busy}>
              Reset
            </Button>
            <Button variant="primary" onClick={save} disabled={!dirty || !valid} loading={busy}>
              Save changes
            </Button>
          </>
        }
      />

      <p className="mb-6 rounded-md border border-zinc-800 bg-zinc-900/60 px-4 py-3 text-sm text-zinc-400">
        Weights are <span className="text-zinc-200">relative</span>: an entry's drop chance is its weight divided by the sum of all
        weights in the same reel. Weight 0 keeps the entry in the list but it never drops. Featured only marks the prizes printed on
        the box. Hours is the rental period of the drop, 0 = permanent. Saving replaces the whole list.
      </p>

      <div className="space-y-6">
        {reels.map((reel) => (
          <ReelTable
            key={reel}
            reel={reel}
            entries={draft.filter((e) => e.reel === reel)}
            onChange={update}
            onRemove={remove}
            onEqualise={() => equalise(reel)}
            onAdd={() => setAdding(reel)}
            busy={busy}
          />
        ))}
      </div>

      {adding && (
        <AddEntryDialog
          reel={adding}
          existingIds={new Set(draft.filter((e) => e.reel === adding).map((e) => e.item_id))}
          onClose={() => setAdding(null)}
          onAdd={(item_id, name) => add(adding, item_id, name)}
        />
      )}
    </div>
  );
}

function BackLink() {
  return (
    <Link href="/boxes/" className="mb-4 inline-flex items-center gap-1 text-sm text-zinc-400 hover:text-zinc-100">
      <IconArrowLeft className="h-4 w-4" />
      Loot boxes
    </Link>
  );
}

// ---------------------------------------------------------------------------
// Reel table
// ---------------------------------------------------------------------------

function ReelTable({
  reel,
  entries,
  onChange,
  onRemove,
  onEqualise,
  onAdd,
  busy,
}: {
  reel: Reel;
  entries: EntryDraft[];
  onChange: (key: number, patch: Partial<EntryDraft>) => void;
  onRemove: (key: number) => void;
  onEqualise: () => void;
  onAdd: () => void;
  busy: boolean;
}) {
  const weights = entries.map((e) => parseWeight(e.weight));
  const sum = weights.reduce<number>((acc, w) => acc + (w ?? 0), 0);
  const anyInvalid = entries.some((e) => parseWeight(e.weight) === null || parseHours(e.hours) === null);
  const pad = 'px-3 py-2 sm:px-4';

  return (
    <Card
      flush
      title={REEL_TITLE[reel]}
      description={`${REEL_DESC[reel]}${entries.length > 0 ? ` Total weight ${Number.isInteger(sum) ? fmtInt(sum) : sum.toFixed(2)}.` : ''}`}
      actions={
        <>
          <Button size="sm" onClick={onEqualise} disabled={entries.length === 0 || busy} title="Set every weight in this reel to 1">
            Equalise weights
          </Button>
          <Button size="sm" variant="primary" onClick={onAdd} disabled={busy}>
            <IconPlus className="h-4 w-4" />
            Add item
          </Button>
        </>
      }
    >
      <div className="w-full overflow-x-auto">
        <table className="w-full min-w-max text-left text-sm">
          <thead className="bg-zinc-900 text-xs uppercase tracking-wide text-zinc-500">
            <tr>
              <th scope="col" className={`${pad} font-medium`}>Item</th>
              <th scope="col" className={`${pad} font-medium`}>Featured</th>
              <th scope="col" className={`${pad} font-medium`}>Rental hours</th>
              <th scope="col" className={`${pad} font-medium`}>Weight</th>
              <th scope="col" className={`${pad} text-right font-medium`}>Chance</th>
              <th scope="col" className={`${pad} font-medium`} />
            </tr>
          </thead>
          <tbody className="divide-y divide-zinc-800/80">
            {entries.length === 0 && (
              <tr>
                <td colSpan={6} className="px-4 py-8 text-center text-sm text-zinc-500">
                  {reel === 'bonus' ? 'No bonus reel on this box.' : 'No entries. Add an item to give the box something to drop.'}
                </td>
              </tr>
            )}
            {entries.map((e, i) => {
              const w = weights[i];
              const h = parseHours(e.hours);
              const pct = w === null ? null : percentOf(w, sum);
              return (
                <tr key={e.key} className="transition-colors hover:bg-zinc-800/50">
                  <td className={`${pad} align-middle`}>
                    <ItemIcon id={e.item_id} size="sm" className="mr-2 inline-block align-middle" />
                    <Link href={`/items/view/?id=${e.item_id}`} className="font-medium text-zinc-100 hover:text-accent-300 hover:underline">
                      {e.name || `Item #${e.item_id}`}
                    </Link>
                    <span className="ml-2 text-xs text-zinc-500">#{e.item_id}</span>
                    {e.featured && (
                      <Badge tone="accent" className="ml-2">
                        Featured
                      </Badge>
                    )}
                  </td>
                  <td className={`${pad} align-middle`}>
                    <Checkbox label={<span className="sr-only">Featured</span>} checked={e.featured} onChange={(ev) => onChange(e.key, { featured: ev.target.checked })} aria-label={`${e.name} featured`} />
                  </td>
                  <td className={`${pad} align-middle`}>
                    <div className="flex items-center gap-2">
                      <div className="w-24">
                        <Input
                          inputSize="sm"
                          type="number"
                          min={0}
                          step={1}
                          inputMode="numeric"
                          value={e.hours}
                          onChange={(ev) => onChange(e.key, { hours: ev.target.value })}
                          aria-label={`${e.name} rental hours`}
                          className={h === null ? 'border-rose-500' : ''}
                        />
                      </div>
                      <span className="text-xs text-zinc-500">{h === null ? '?' : fmtHours(h)}</span>
                    </div>
                  </td>
                  <td className={`${pad} align-middle`}>
                    <div className="w-28">
                      <Input
                        inputSize="sm"
                        type="number"
                        min={0}
                        step="any"
                        inputMode="decimal"
                        value={e.weight}
                        onChange={(ev) => onChange(e.key, { weight: ev.target.value })}
                        aria-label={`${e.name} weight`}
                        className={w === null ? 'border-rose-500' : ''}
                      />
                    </div>
                  </td>
                  <td className={`${pad} text-right align-middle tabular-nums`}>
                    {pct === null ? (
                      <span className="text-rose-300">?</span>
                    ) : (
                      <span className={pct === 0 ? 'text-zinc-500' : 'text-zinc-100'}>{fmtPercent(pct)}</span>
                    )}
                  </td>
                  <td className={`${pad} text-right align-middle`}>
                    <Button size="sm" variant="ghost" className="text-rose-300 hover:bg-rose-500/10" onClick={() => onRemove(e.key)} aria-label={`Remove ${e.name}`}>
                      <IconTrash className="h-4 w-4" />
                      <span className="hidden sm:inline">Remove</span>
                    </Button>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
      {anyInvalid && <p className="border-t border-zinc-800 px-4 py-2 text-xs text-rose-300">Weights must be numbers of 0 or more, hours whole numbers of 0 or more.</p>}
    </Card>
  );
}

// ---------------------------------------------------------------------------
// Add entry dialog
// ---------------------------------------------------------------------------

function AddEntryDialog({
  reel,
  existingIds,
  onClose,
  onAdd,
}: {
  reel: Reel;
  existingIds: Set<number>;
  onClose: () => void;
  onAdd: (item_id: number, name: string) => void;
}) {
  const toast = useToast();
  const [picked, setPicked] = useState<CatalogItem | null>(null);
  const [rawId, setRawId] = useState('');
  const [busy, setBusy] = useState(false);

  const rawNum = Number(rawId);
  const rawValid = rawId.trim() !== '' && Number.isInteger(rawNum) && rawNum > 0;
  const rawDuplicate = rawValid && existingIds.has(rawNum);

  const addPicked = () => {
    if (!picked) return;
    onAdd(picked.id, picked.name);
  };

  const addById = async () => {
    if (!rawValid || rawDuplicate) return;
    setBusy(true);
    try {
      // Look the name up when the catalog knows the id; otherwise add it blind.
      const item = await api.items.get(rawNum).catch(() => null);
      onAdd(rawNum, item ? item.name : '');
      if (!item) toast.info(`Item #${rawNum} is not in the catalog; added by id only`);
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal
      open
      title={`Add to ${REEL_TITLE[reel].toLowerCase()}`}
      onClose={onClose}
      footer={
        <>
          <Button variant="ghost" onClick={onClose} disabled={busy}>
            Cancel
          </Button>
          <Button variant="primary" onClick={addPicked} disabled={!picked || busy}>
            Add {picked ? picked.name : 'selected'}
          </Button>
        </>
      }
    >
      <div className="space-y-4">
        <CatalogSearch inputId="box-add-search" onPick={setPicked} selectedId={picked?.id ?? null} disabledIds={existingIds} />

        {picked && (
          <div className="rounded-md border border-accent-500/40 bg-accent-500/10 px-3 py-2 text-sm text-accent-100">
            Selected: <span className="font-medium">{picked.name}</span> <span className="text-accent-200/70">#{picked.id}</span>
          </div>
        )}

        {reel === 'bonus' && (
          <form
            onSubmit={(e) => {
              e.preventDefault();
              void addById();
            }}
            className="border-t border-zinc-800 pt-4"
          >
            <div className="flex items-end gap-2">
              <div className="flex-1">
                <Input
                  id="box-add-id"
                  label="Or add by item id"
                  type="number"
                  min={1}
                  inputMode="numeric"
                  value={rawId}
                  onChange={(e) => setRawId(e.target.value)}
                  hint={rawDuplicate ? 'Already in this reel.' : 'For vouchers and tickets that are not searchable by name.'}
                  className={rawDuplicate ? 'border-rose-500' : ''}
                />
              </div>
              <Button type="submit" disabled={!rawValid || rawDuplicate} loading={busy}>
                Add by id
              </Button>
            </div>
          </form>
        )}
      </div>
    </Modal>
  );
}
