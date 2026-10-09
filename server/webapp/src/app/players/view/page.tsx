'use client';

import Link from 'next/link';
import { useRouter, useSearchParams } from 'next/navigation';
import { Suspense, useCallback, useEffect, useMemo, useState } from 'react';
import { api, FULL_DURABILITY, type CatalogItem, type Item, type Player, type PlayerSocial, type QSlot, type Skill, type Stats } from '@/lib/api';
import { fmtInt, fmtSlot, fmtUntil, kdRatio } from '@/lib/format';
import { Badge } from '@/components/Badge';
import { Button } from '@/components/Button';
import { Card } from '@/components/Card';
import { CatalogSearch } from '@/components/CatalogSearch';
import { ConfirmDialog } from '@/components/ConfirmDialog';
import { EditableNumber } from '@/components/EditableNumber';
import { ErrorBox, Loading, PageHeader } from '@/components/Feedback';
import { IconArrowLeft, IconTrash } from '@/components/Icons';
import { Input } from '@/components/Input';
import { DataTable, type Column } from '@/components/Table';
import { errorMessage, useToast } from '@/components/Toast';
import { ItemIcon } from '@/components/ItemIcon';

export default function PlayerViewPage() {
  return (
    <Suspense fallback={<Loading />}>
      <PlayerView />
    </Suspense>
  );
}

function PlayerView() {
  const params = useSearchParams();
  const name = params.get('name') ?? '';
  const router = useRouter();
  const toast = useToast();

  const [player, setPlayer] = useState<Player | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [confirmDelete, setConfirmDelete] = useState(false);
  const [pendingItem, setPendingItem] = useState<Item | null>(null);
  const [pendingSkill, setPendingSkill] = useState<Skill | null>(null);

  const load = useCallback(async () => {
    if (!name) {
      setError('No player name given. Open this page from the players list.');
      return;
    }
    setError(null);
    try {
      setPlayer(await api.players.get(name));
    } catch (err) {
      setError(errorMessage(err));
    }
  }, [name]);

  useEffect(() => {
    void load();
  }, [load]);

  /** Runs an API call, applies the returned player and toasts the outcome. */
  const run = async (label: string, fn: () => Promise<unknown>, after?: () => void) => {
    try {
      const result = await fn();
      if (result && typeof result === 'object' && 'stats' in result && 'items' in result) {
        setPlayer(result as Player);
      }
      toast.success(label);
      after?.();
    } catch (err) {
      toast.error(errorMessage(err));
      throw err;
    }
  };

  const updatePlayer = (body: Parameters<typeof api.players.update>[1], label: string) =>
    run(label, () => api.players.update(name, body));

  if (error) {
    return (
      <div>
        <BackLink />
        <ErrorBox message={error} onRetry={name ? load : undefined} />
      </div>
    );
  }
  if (!player) return <Loading />;

  return (
    <div>
      <BackLink />
      <PageHeader
        title={player.name}
        subtitle={
          player.account ? (
            <>
              Account:{' '}
              <Link href="/accounts/" className="text-accent-300 hover:underline">
                {player.account}
              </Link>
            </>
          ) : (
            <span className="text-zinc-500">No account linked</span>
          )
        }
        actions={
          <Button variant="danger" onClick={() => setConfirmDelete(true)}>
            <IconTrash className="h-4 w-4" />
            Delete player
          </Button>
        }
      />

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-3">
        <Card title="Balance" description="Saved immediately per field." className="lg:col-span-1">
          <div className="space-y-4">
            <EditableNumber label="Points" value={player.points} min={0} onSave={(v) => updatePlayer({ points: v }, 'Points saved')} />
            <EditableNumber label="Cash" value={player.cash} min={0} onSave={(v) => updatePlayer({ cash: v }, 'Cash saved')} />
          </div>
        </Card>

        <StatsCard stats={player.stats} onSave={(patch) => updatePlayer({ stats: patch }, 'Stats saved')} />
      </div>

      <div className="mt-6">
        <Card
          flush
          title="Inventory"
          description={`${player.items.length} item${player.items.length === 1 ? '' : 's'}. Slot -1 means the item is in the bag. Durability ${fmtInt(FULL_DURABILITY)} = 100%; weapons wear 5 per second of use and can be repaired in-game for 1 point per 100 missing.`}
          actions={
            <Button
              size="sm"
              variant="ghost"
              disabled={!player.items.some((it) => it.durability < FULL_DURABILITY)}
              onClick={() =>
                run('All items repaired', () => api.players.repairAll(name), () => void load()).catch(() => {
                  /* toasted */
                })
              }
            >
              Repair all
            </Button>
          }
        >
          <InventoryTable
            items={player.items}
            onUpdate={(item, body) =>
              run(`${item.name} updated`, () => api.players.updateItem(name, item.unique_id, body), () => void load())
            }
            onRemove={(item) => setPendingItem(item)}
          />
        </Card>
      </div>

      <div className="mt-6 grid grid-cols-1 gap-6 lg:grid-cols-2">
        <GrantItemCard
          onGrant={async (item, stack) => {
            await run(`Granted ${item.name}`, () => api.players.grantItem(name, { item_id: item.id, stack }), () => void load());
          }}
        />

        <Card
          flush
          title="Skills"
          description={`${player.skills.length} learned`}
          actions={<AddSkillForm onAdd={(id) => run('Skill added', () => api.players.grantSkill(name, id), () => void load())} />}
        >
          <SkillsTable skills={player.skills} onRemove={(s) => setPendingSkill(s)} />
        </Card>
      </div>

      <div className="mt-6">
        <Card flush title="Quick slots" description="Loadout presets. Read-only; unique ids are resolved to item names.">
          <QSlotsTable qslots={player.qslots} items={player.items} skills={player.skills} />
        </Card>
      </div>

      <div className="mt-6">
        <SocialCard name={name} />
      </div>

      <ConfirmDialog
        open={confirmDelete}
        danger
        title={`Delete player "${player.name}"?`}
        message="This removes the character, its inventory, skills and stats permanently. A linked account keeps existing."
        confirmLabel="Delete player"
        onCancel={() => setConfirmDelete(false)}
        onConfirm={async () => {
          try {
            await api.players.remove(name);
            toast.success(`Player "${name}" deleted`);
            router.push('/players/');
          } catch (err) {
            toast.error(errorMessage(err));
            setConfirmDelete(false);
          }
        }}
      />

      <ConfirmDialog
        open={pendingItem !== null}
        danger
        title="Remove item?"
        message={
          pendingItem ? `Remove ${pendingItem.name} (unique id ${pendingItem.unique_id}) from ${player.name}'s inventory?` : undefined
        }
        confirmLabel="Remove"
        onCancel={() => setPendingItem(null)}
        onConfirm={async () => {
          if (!pendingItem) return;
          const item = pendingItem;
          try {
            await run(`${item.name} removed`, () => api.players.removeItem(name, item.unique_id), () => void load());
          } catch {
            /* toasted */
          }
          setPendingItem(null);
        }}
      />

      <ConfirmDialog
        open={pendingSkill !== null}
        danger
        title="Remove skill?"
        message={pendingSkill ? `Remove ${pendingSkill.name} (skill id ${pendingSkill.skill_id}) from ${player.name}?` : undefined}
        confirmLabel="Remove"
        onCancel={() => setPendingSkill(null)}
        onConfirm={async () => {
          if (!pendingSkill) return;
          const skill = pendingSkill;
          try {
            await run(`${skill.name} removed`, () => api.players.removeSkill(name, skill.unique_id), () => void load());
          } catch {
            /* toasted */
          }
          setPendingSkill(null);
        }}
      />
    </div>
  );
}

function BackLink() {
  return (
    <Link href="/players/" className="mb-4 inline-flex items-center gap-1 text-sm text-zinc-400 hover:text-zinc-100">
      <IconArrowLeft className="h-4 w-4" />
      All players
    </Link>
  );
}

// ---------------------------------------------------------------------------
// Stats
// ---------------------------------------------------------------------------

const STAT_FIELDS: { key: keyof Stats; label: string }[] = [
  { key: 'level', label: 'Level' },
  { key: 'exp', label: 'EXP' },
  { key: 'kills', label: 'Kills' },
  { key: 'deaths', label: 'Deaths' },
  { key: 'assists', label: 'Assists' },
  { key: 'headshots', label: 'Headshots' },
  { key: 'team_kills', label: 'Team kills' },
  { key: 'wins', label: 'Wins' },
  { key: 'losses', label: 'Losses' },
  { key: 'play_time', label: 'Play time' },
];

type StatsDraft = Record<keyof Stats, string>;

function toDraft(stats: Stats): StatsDraft {
  const draft = {} as StatsDraft;
  for (const f of STAT_FIELDS) draft[f.key] = String(stats[f.key]);
  return draft;
}

function StatsCard({ stats, onSave }: { stats: Stats; onSave: (patch: Partial<Stats>) => Promise<void> }) {
  const [draft, setDraft] = useState<StatsDraft>(() => toDraft(stats));
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    setDraft(toDraft(stats));
  }, [stats]);

  const patch: Partial<Stats> = {};
  let invalid = false;
  for (const f of STAT_FIELDS) {
    const raw = draft[f.key].trim();
    const n = Number(raw);
    if (raw === '' || !Number.isFinite(n)) {
      invalid = true;
      continue;
    }
    if (n !== stats[f.key]) patch[f.key] = n;
  }
  const dirty = Object.keys(patch).length > 0;

  const save = async () => {
    if (!dirty || invalid) return;
    setBusy(true);
    try {
      await onSave(patch);
    } catch {
      /* toasted by caller */
    } finally {
      setBusy(false);
    }
  };

  return (
    <Card
      title="Stats"
      description={`K/D ${kdRatio(stats.kills, stats.deaths)} - ${fmtInt(stats.wins + stats.losses)} matches`}
      className="lg:col-span-2"
      actions={
        <>
          <Button size="sm" variant="ghost" onClick={() => setDraft(toDraft(stats))} disabled={!dirty || busy}>
            Reset
          </Button>
          <Button size="sm" variant="primary" onClick={save} disabled={!dirty || invalid} loading={busy}>
            Save stats
          </Button>
        </>
      }
    >
      <form
        onSubmit={(e) => {
          e.preventDefault();
          void save();
        }}
        className="grid grid-cols-2 gap-3 sm:grid-cols-3 md:grid-cols-5"
      >
        {STAT_FIELDS.map((f) => {
          const changed = f.key in patch;
          return (
            <Input
              key={f.key}
              id={`stat-${f.key}`}
              label={f.label}
              type="number"
              inputMode="numeric"
              value={draft[f.key]}
              onChange={(e) => setDraft((d) => ({ ...d, [f.key]: e.target.value }))}
              className={changed ? 'border-accent-500' : ''}
            />
          );
        })}
        <button type="submit" className="hidden" aria-hidden="true" tabIndex={-1} />
      </form>
    </Card>
  );
}

// ---------------------------------------------------------------------------
// Inventory
// ---------------------------------------------------------------------------

function InventoryTable({
  items,
  onUpdate,
  onRemove,
}: {
  items: Item[];
  onUpdate: (item: Item, body: { stack?: number; durability?: number; slot?: number }) => Promise<void>;
  onRemove: (item: Item) => void;
}) {
  const sorted = useMemo(() => [...items].sort((a, b) => (a.slot === b.slot ? a.unique_id - b.unique_id : a.slot < 0 ? 1 : b.slot < 0 ? -1 : a.slot - b.slot)), [items]);

  const columns: Column<Item>[] = [
    {
      key: 'name',
      header: 'Item',
      render: (it) => (
        <div className="flex items-center gap-2">
          <ItemIcon id={it.item_id} size="sm" />
          <div>
            <Link href={`/items/view/?id=${it.item_id}`} className="font-medium text-zinc-100 hover:text-accent-300 hover:underline">
              {it.name}
            </Link>
            <div className="text-xs text-zinc-500 sm:hidden">
              id {it.item_id} - uid {it.unique_id}
            </div>
          </div>
        </div>
      ),
    },
    { key: 'item_id', header: 'Item id', hideBelow: 'sm', className: 'text-zinc-400', render: (it) => it.item_id },
    { key: 'unique_id', header: 'Unique id', hideBelow: 'sm', className: 'text-zinc-400', render: (it) => it.unique_id },
    {
      key: 'slot',
      header: 'Slot',
      render: (it) => (it.slot < 0 ? <Badge tone="neutral">Bag</Badge> : <Badge tone="accent">{fmtSlot(it.slot)}</Badge>),
    },
    {
      key: 'stack',
      header: 'Stack',
      render: (it) => (
        <EditableNumber compact size="sm" min={0} value={it.stack} onSave={(v) => onUpdate(it, { stack: v })} />
      ),
    },
    {
      key: 'durability',
      header: 'Durability',
      render: (it) => (
        <div className="flex items-center gap-2">
          <EditableNumber compact size="sm" min={0} value={it.durability} onSave={(v) => onUpdate(it, { durability: v })} />
          {it.item_type !== 0 || it.durability !== 0 ? (
            <span className={it.durability <= 0 ? 'text-xs text-red-400' : it.durability < FULL_DURABILITY / 4 ? 'text-xs text-amber-300' : 'text-xs text-zinc-500'}>
              {Math.round((it.durability / FULL_DURABILITY) * 100)}%
            </span>
          ) : null}
        </div>
      ),
    },
    {
      key: 'until',
      header: 'Expiry',
      hideBelow: 'md',
      render: (it) => <span className={it.until ? 'text-zinc-300' : 'text-zinc-500'}>{fmtUntil(it.until)}</span>,
    },
    {
      key: 'actions',
      header: '',
      className: 'text-right',
      render: (it) => (
        <Button size="sm" variant="danger" onClick={() => onRemove(it)} aria-label={`Remove ${it.name}`}>
          Remove
        </Button>
      ),
    },
  ];

  return <DataTable columns={columns} rows={sorted} rowKey={(it) => it.unique_id} emptyText="Empty inventory." />;
}

// ---------------------------------------------------------------------------
// Grant item
// ---------------------------------------------------------------------------

function GrantItemCard({ onGrant }: { onGrant: (item: CatalogItem, stack: number) => Promise<void> }) {
  const [selected, setSelected] = useState<CatalogItem | null>(null);
  const [stack, setStack] = useState('1');
  const [busy, setBusy] = useState(false);

  const stackNum = Number(stack);
  const stackValid = Number.isInteger(stackNum) && stackNum >= 1;

  const grant = async () => {
    if (!selected || !stackValid) return;
    setBusy(true);
    try {
      await onGrant(selected, stackNum);
      setStack('1');
    } catch {
      /* toasted */
    } finally {
      setBusy(false);
    }
  };

  return (
    <Card title="Grant item" description="Search the catalog, pick an item and choose how many to add.">
      <div className="space-y-3">
        <CatalogSearch inputId="grant-search" onPick={setSelected} selectedId={selected?.id ?? null} />

        <div className="flex flex-col gap-3 rounded-md border border-zinc-800 bg-zinc-900/60 p-3 sm:flex-row sm:items-end">
          <div className="min-w-0 flex-1">
            <div className="mb-1 text-xs font-medium uppercase tracking-wide text-zinc-400">Selected</div>
            {selected ? (
              <div className="flex items-center gap-2 truncate text-sm text-zinc-100">
                <ItemIcon id={selected.id} size="sm" />
                <span className="truncate">
                  {selected.name} <span className="text-zinc-500">#{selected.id}</span>
                </span>
              </div>
            ) : (
              <div className="text-sm text-zinc-500">Nothing selected</div>
            )}
          </div>
          <div className="w-28">
            <Input id="grant-stack" label="Stack" type="number" min={1} inputMode="numeric" value={stack} onChange={(e) => setStack(e.target.value)} />
          </div>
          <Button variant="primary" onClick={grant} disabled={!selected || !stackValid} loading={busy}>
            Grant
          </Button>
        </div>
      </div>
    </Card>
  );
}

// ---------------------------------------------------------------------------
// Skills
// ---------------------------------------------------------------------------

function SkillsTable({ skills, onRemove }: { skills: Skill[]; onRemove: (s: Skill) => void }) {
  const columns: Column<Skill>[] = [
    {
      key: 'name',
      header: 'Skill',
      render: (s) => (
        <span className="inline-flex items-center gap-2">
          <ItemIcon id={s.skill_id} size="sm" />
          <span className="font-medium text-zinc-100">{s.name}</span>
        </span>
      ),
    },
    { key: 'skill_id', header: 'Skill id', className: 'text-zinc-400', render: (s) => s.skill_id },
    { key: 'unique_id', header: 'Unique id', hideBelow: 'sm', className: 'text-zinc-400', render: (s) => s.unique_id },
    { key: 'slot', header: 'Slot', render: (s) => (s.slot < 0 ? <span className="text-zinc-500">-</span> : s.slot) },
    {
      key: 'actions',
      header: '',
      className: 'text-right',
      render: (s) => (
        <Button size="sm" variant="danger" onClick={() => onRemove(s)}>
          Remove
        </Button>
      ),
    },
  ];
  return <DataTable columns={columns} rows={skills} rowKey={(s) => s.unique_id} emptyText="No skills learned." />;
}

function AddSkillForm({ onAdd }: { onAdd: (skillId: number) => Promise<void> }) {
  const [value, setValue] = useState('');
  const [busy, setBusy] = useState(false);
  const id = Number(value);
  const valid = value.trim() !== '' && Number.isInteger(id) && id > 0;

  const submit = async () => {
    if (!valid) return;
    setBusy(true);
    try {
      await onAdd(id);
      setValue('');
    } catch {
      /* toasted */
    } finally {
      setBusy(false);
    }
  };

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        void submit();
      }}
      className="flex items-center gap-2"
    >
      <div className="w-28">
        <Input inputSize="sm" type="number" min={1} inputMode="numeric" placeholder="Skill id" value={value} onChange={(e) => setValue(e.target.value)} aria-label="Skill id" />
      </div>
      <Button size="sm" type="submit" disabled={!valid} loading={busy}>
        Add
      </Button>
    </form>
  );
}

// ---------------------------------------------------------------------------
// Friends and block list
// ---------------------------------------------------------------------------

function SocialCard({ name }: { name: string }) {
  const toast = useToast();
  const [social, setSocial] = useState<PlayerSocial | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [friendName, setFriendName] = useState('');
  const [blockName, setBlockName] = useState('');
  const [busy, setBusy] = useState(false);

  const load = useCallback(async () => {
    setError(null);
    try {
      setSocial(await api.players.social(name));
    } catch (err) {
      setError(errorMessage(err));
    }
  }, [name]);

  useEffect(() => {
    void load();
  }, [load]);

  const act = async (label: string, fn: () => Promise<PlayerSocial>) => {
    setBusy(true);
    try {
      setSocial(await fn());
      toast.success(label);
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setBusy(false);
    }
  };

  const friendColumns: Column<PlayerSocial['friends'][number]>[] = [
    {
      key: 'name',
      header: 'Friend',
      render: (f) => (
        <Link href={`/players/view/?name=${encodeURIComponent(f.name)}`} className="font-medium text-zinc-100 hover:text-accent-300 hover:underline">
          {f.name}
        </Link>
      ),
    },
    { key: 'level', header: 'Level', className: 'text-zinc-400', render: (f) => f.level },
    {
      key: 'flags',
      header: '',
      render: (f) => (f.blocked_you ? <Badge tone="warning">Has blocked {name}</Badge> : null),
    },
    {
      key: 'actions',
      header: '',
      className: 'text-right',
      render: (f) => (
        <Button size="sm" variant="danger" disabled={busy} onClick={() => act(`${f.name} removed from friends`, () => api.players.removeFriend(name, f.name))}>
          Remove
        </Button>
      ),
    },
  ];

  const blockColumns: Column<string>[] = [
    {
      key: 'name',
      header: 'Blocked player',
      render: (b) => (
        <Link href={`/players/view/?name=${encodeURIComponent(b)}`} className="font-medium text-zinc-100 hover:text-accent-300 hover:underline">
          {b}
        </Link>
      ),
    },
    {
      key: 'actions',
      header: '',
      className: 'text-right',
      render: (b) => (
        <Button size="sm" variant="danger" disabled={busy} onClick={() => act(`${b} unblocked`, () => api.players.removeBlock(name, b))}>
          Unblock
        </Button>
      ),
    },
  ];

  const addForm = (id: string, placeholder: string, value: string, setValue: (v: string) => void, label: string, fn: (other: string) => Promise<PlayerSocial>) => (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        const other = value.trim();
        if (!other) return;
        void act(label.replace('%s', other), () => fn(other)).then(() => setValue(''));
      }}
      className="flex items-center gap-2"
    >
      <div className="w-40">
        <Input inputSize="sm" id={id} placeholder={placeholder} value={value} onChange={(e) => setValue(e.target.value)} aria-label={placeholder} />
      </div>
      <Button size="sm" type="submit" disabled={!value.trim() || busy}>
        Add
      </Button>
    </form>
  );

  if (error) return <ErrorBox message={error} onRetry={load} />;

  return (
    <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
      <Card
        flush
        title="Friends"
        description={
          social
            ? `${social.friends.length} friend${social.friends.length === 1 ? '' : 's'}. Friendships are mutual; the game's invite flow (send, accept) ends up here too.`
            : 'Loading...'
        }
        actions={addForm('friend-name', 'Player name', friendName, setFriendName, '%s added as friend', (o) => api.players.addFriend(name, o))}
      >
        <DataTable columns={friendColumns} rows={social?.friends ?? []} rowKey={(f) => f.name} emptyText="No friends yet." />
      </Card>

      <Card
        flush
        title="Block list"
        description={
          social
            ? `${social.blocks.length} blocked. Blocked players cannot whisper, friend-invite or room-invite ${name}.${
                social.blocked_by.length > 0 ? ` Blocked by: ${social.blocked_by.join(', ')}.` : ''
              }`
            : 'Loading...'
        }
        actions={addForm('block-name', 'Player name', blockName, setBlockName, '%s blocked', (o) => api.players.addBlock(name, o))}
      >
        <DataTable columns={blockColumns} rows={social?.blocks ?? []} rowKey={(b) => b} emptyText="Nobody blocked." />
      </Card>
    </div>
  );
}

// ---------------------------------------------------------------------------
// Quick slots
// ---------------------------------------------------------------------------

function QSlotsTable({ qslots, items, skills }: { qslots: QSlot[]; items: Item[]; skills: Skill[] }) {
  const itemNames = useMemo(() => new Map(items.map((i) => [i.unique_id, i.name])), [items]);
  const skillNames = useMemo(() => new Map(skills.map((s) => [s.unique_id, s.name])), [skills]);

  const ref = (uid: number) => {
    if (!uid) return <span className="text-zinc-600">-</span>;
    const n = itemNames.get(uid);
    return n ? (
      <span title={`uid ${uid}`}>{n}</span>
    ) : (
      <span className="text-amber-300" title="Not found in inventory">
        uid {uid}
      </span>
    );
  };

  const columns: Column<QSlot>[] = [
    { key: 'id', header: '#', className: 'w-10 text-zinc-500', render: (q) => q.id },
    { key: 'name', header: 'Name', render: (q) => <span className="font-medium text-zinc-100">{q.name || <span className="text-zinc-500">(unnamed)</span>}</span> },
    { key: 'main', header: 'Main', render: (q) => ref(q.main) },
    { key: 'pistol', header: 'Pistol', render: (q) => ref(q.pistol) },
    { key: 'throw1', header: 'Throw 1', hideBelow: 'sm', render: (q) => ref(q.throw1) },
    { key: 'throw2', header: 'Throw 2', hideBelow: 'sm', render: (q) => ref(q.throw2) },
    {
      key: 'skills',
      header: 'Skills',
      hideBelow: 'md',
      render: (q) =>
        q.skills.filter((s) => s !== 0).length === 0 ? (
          <span className="text-zinc-600">-</span>
        ) : (
          <div className="flex flex-wrap gap-1">
            {q.skills
              .filter((s) => s !== 0)
              .map((s, i) => (
                <Badge key={`${s}-${i}`} tone="neutral">
                  {skillNames.get(s) ?? `uid ${s}`}
                </Badge>
              ))}
          </div>
        ),
    },
  ];

  return <DataTable columns={columns} rows={qslots} rowKey={(q) => q.id} emptyText="No quick slots defined." />;
}
