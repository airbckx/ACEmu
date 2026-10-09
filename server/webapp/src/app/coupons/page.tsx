'use client';

import { useCallback, useEffect, useState, type FormEvent } from 'react';
import { api, type CatalogItem, type Coupon } from '@/lib/api';
import { fmtInt, fmtUtc } from '@/lib/format';
import { Badge } from '@/components/Badge';
import { Button } from '@/components/Button';
import { Card } from '@/components/Card';
import { CatalogSearch } from '@/components/CatalogSearch';
import { ConfirmDialog } from '@/components/ConfirmDialog';
import { ErrorBox, Loading, PageHeader } from '@/components/Feedback';
import { Input } from '@/components/Input';
import { DataTable, type Column } from '@/components/Table';
import { errorMessage, useToast } from '@/components/Toast';
import { ItemIcon } from '@/components/ItemIcon';

export default function CouponsPage() {
  const toast = useToast();
  const [coupons, setCoupons] = useState<Coupon[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [toDelete, setToDelete] = useState<Coupon | null>(null);

  const load = useCallback(async () => {
    setError(null);
    try {
      setCoupons(await api.coupons.list());
    } catch (err) {
      setError(errorMessage(err));
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  const copyCode = async (c: Coupon) => {
    try {
      await navigator.clipboard.writeText(normalizeCode(c.serial));
      toast.success('Code copied - use Paste in the Redeem Codes window');
    } catch {
      toast.error('Could not access the clipboard - select the code and copy it by hand');
    }
  };

  const columns: Column<Coupon>[] = [
    {
      key: 'serial',
      header: 'Code',
      render: (c) => (
        <div className="flex items-center gap-2">
          <span className="font-mono font-medium tracking-wide text-zinc-100">{normalizeCode(c.serial)}</span>
          <Button size="sm" onClick={() => void copyCode(c)}>
            Copy
          </Button>
        </div>
      ),
    },
    {
      key: 'grants',
      header: 'Grants',
      render: (c) => (
        <div className="flex flex-wrap gap-1.5">
          {c.item_id > 0 && (
            <span className="inline-flex items-center gap-1">
              <ItemIcon id={c.item_id} size="xs" hideMissing />
              <Badge tone="accent">
                {c.item_count > 1 ? `${c.item_count}x ` : ''}
                {c.item_name ?? `item ${c.item_id}`}
              </Badge>
            </span>
          )}
          {c.points > 0 && <Badge tone="success">{fmtInt(c.points)} points</Badge>}
          {c.cash > 0 && <Badge tone="warning">{fmtInt(c.cash)} W Coin</Badge>}
        </div>
      ),
    },
    { key: 'uses', header: 'Uses left', className: 'text-right', render: (c) => fmtInt(c.uses_left) },
    { key: 'used', header: 'Redeemed', className: 'text-right', hideBelow: 'sm', render: (c) => fmtInt(c.used) },
    {
      key: 'expires',
      header: 'Expires',
      hideBelow: 'md',
      render: (c) => (c.expires_at ? <span className="text-zinc-400">{fmtUtc(c.expires_at)}</span> : <span className="text-zinc-600">never</span>),
    },
    { key: 'note', header: 'Note', hideBelow: 'lg', render: (c) => <span className="text-zinc-400">{c.note}</span> },
    {
      key: 'actions',
      header: '',
      className: 'text-right',
      render: (c) => (
        <Button size="sm" variant="danger" onClick={() => setToDelete(c)}>
          Delete
        </Button>
      ),
    },
  ];

  return (
    <div>
      <PageHeader
        title="Coupons"
        subtitle={coupons ? `${fmtInt(coupons.length)} code${coupons.length === 1 ? '' : 's'} - redeemed in the Item Shop under Redeem Codes: Copy here, then Paste in the game (it splits the 32 characters into its eight boxes itself; never paste dashes)` : undefined}
      />

      {error && <ErrorBox message={error} onRetry={load} />}
      {!error && !coupons && <Loading />}

      {coupons && (
        <div className="space-y-6">
          <CreateCouponForm
            onCreated={(c) => {
              setCoupons((list) => (list ? [...list.filter((x) => x.serial !== c.serial), c].sort((a, b) => a.serial.localeCompare(b.serial)) : [c]));
              toast.success(`Coupon ${c.serial} saved`);
            }}
          />
          <Card flush title="All coupons" description="Each player can redeem a code once; 'uses left' is the total across players.">
            <DataTable columns={columns} rows={coupons} rowKey={(c) => c.serial} emptyText="No coupons yet." />
          </Card>
        </div>
      )}

      {toDelete && (
        <ConfirmDialog
          open
          danger
          title={`Delete coupon ${toDelete.serial}?`}
          message="Players who already redeemed it keep what they got."
          confirmLabel="Delete"
          onCancel={() => setToDelete(null)}
          onConfirm={async () => {
            try {
              await api.coupons.remove(toDelete.serial);
              setCoupons((list) => (list ? list.filter((c) => c.serial !== toDelete.serial) : list));
              toast.success(`Coupon ${toDelete.serial} deleted`);
            } catch (err) {
              toast.error(errorMessage(err));
            } finally {
              setToDelete(null);
            }
          }}
        />
      )}
    </div>
  );
}

/** The in-game window takes eight boxes of four characters: 32 letters or digits. */
const CODE_LEN = 32;

function randomCode(): string {
  const alphabet = 'ABCDEFGHJKLMNPQRSTUVWXYZ23456789';
  return Array.from({ length: CODE_LEN }, () => alphabet[Math.floor(Math.random() * alphabet.length)]).join('');
}

/** The 32 characters the game wants: it adds the dashes between its boxes itself. */
function normalizeCode(s: string): string {
  return s.toUpperCase().replace(/[^A-Z0-9]/g, '');
}

function CreateCouponForm({ onCreated }: { onCreated: (c: Coupon) => void }) {
  const toast = useToast();
  const [serial, setSerial] = useState('');
  const [item, setItem] = useState<CatalogItem | null>(null);
  const [itemCount, setItemCount] = useState(1);
  const [points, setPoints] = useState(0);
  const [cash, setCash] = useState(0);
  const [uses, setUses] = useState(100);
  const [expires, setExpires] = useState('');
  const [note, setNote] = useState('');
  const [busy, setBusy] = useState(false);

  const normalized = normalizeCode(serial);
  const valid = normalized.length === CODE_LEN && (item !== null || points > 0 || cash > 0);

  const submit = async (e: FormEvent) => {
    e.preventDefault();
    if (!valid) return;
    setBusy(true);
    try {
      const created = await api.coupons.create({
        serial: normalized,
        item_id: item?.id ?? 0,
        item_count: itemCount,
        points,
        cash,
        uses_left: uses,
        expires_at: expires ? expires.replace('T', ' ') + ':00' : null,
        note,
      });
      onCreated(created);
      setSerial('');
      setItem(null);
      setPoints(0);
      setCash(0);
      setNote('');
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <Card
      title="New coupon"
      description="A code can grant an item, points, W Coin, or any combination. The in-game window has eight boxes of four characters, so a code is exactly 32 letters or digits. Players paste the bare characters; the game fills its boxes and shows the dashes itself."
    >
      <form onSubmit={submit} className="space-y-4">
        <div className="grid gap-3 sm:grid-cols-3">
          <div className="sm:col-span-2">
            <Input
              id="coupon-serial"
              label="Code"
              value={serial}
              onChange={(e) => setSerial(e.target.value.toUpperCase())}
              placeholder="ACEMUWELCOME2026FREE5000POINTS00"
              autoComplete="off"
              required
              hint={normalized.length === CODE_LEN ? `Saved as: ${normalized}` : `${normalized.length} / ${CODE_LEN} characters (anything but letters and digits is dropped)`}
            />
          </div>
          <div className="flex items-end">
            <Button type="button" onClick={() => setSerial(randomCode())}>
              Generate code
            </Button>
          </div>
        </div>
        <div className="grid gap-3 sm:grid-cols-2">
          <div>
            <label className="mb-1 block text-xs font-medium text-zinc-400">Item (optional)</label>
            <CatalogSearch inputId="coupon-item" onPick={setItem} selectedId={item?.id ?? null} placeholder="Search an item to grant" />
            {item && (
              <div className="mt-1 flex items-center gap-2 text-xs text-zinc-400">
                <ItemIcon id={item.id} size="sm" />
                <Badge tone="accent">{item.name}</Badge>
                <button type="button" className="text-zinc-500 hover:text-zinc-200" onClick={() => setItem(null)}>
                  clear
                </button>
              </div>
            )}
          </div>
          <Input id="coupon-count" label="Item count" type="number" min={1} value={itemCount} onChange={(e) => setItemCount(Math.max(1, Number(e.target.value)))} disabled={!item} />
        </div>
        <div className="grid gap-3 sm:grid-cols-4">
          <Input id="coupon-points" label="Points" type="number" min={0} value={points} onChange={(e) => setPoints(Math.max(0, Number(e.target.value)))} />
          <Input id="coupon-cash" label="W Coin" type="number" min={0} value={cash} onChange={(e) => setCash(Math.max(0, Number(e.target.value)))} />
          <Input id="coupon-uses" label="Total uses" type="number" min={1} value={uses} onChange={(e) => setUses(Math.max(1, Number(e.target.value)))} />
          <Input id="coupon-expires" label="Expires (local time)" type="datetime-local" value={expires} onChange={(e) => setExpires(e.target.value)} />
        </div>
        <Input id="coupon-note" label="Note" value={note} onChange={(e) => setNote(e.target.value)} placeholder="Where this code was handed out" />
        <div className="flex justify-end">
          <Button type="submit" variant="primary" loading={busy} disabled={!valid}>
            Save coupon
          </Button>
        </div>
      </form>
    </Card>
  );
}
