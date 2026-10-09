'use client';

import Link from 'next/link';
import { useRouter, useSearchParams } from 'next/navigation';
import { Suspense, useCallback, useEffect, useState } from 'react';
import { api, localInputToOle, oleToLocalInput, questTypeLabel, type CatalogItem, type QuestDetail } from '@/lib/api';
import { fmtInt } from '@/lib/format';
import { Badge } from '@/components/Badge';
import { Button } from '@/components/Button';
import { Card } from '@/components/Card';
import { CatalogSearch } from '@/components/CatalogSearch';
import { ConfirmDialog } from '@/components/ConfirmDialog';
import { ErrorBox, Loading, PageHeader } from '@/components/Feedback';
import { IconArrowLeft } from '@/components/Icons';
import { Checkbox, Input, Select } from '@/components/Input';
import { DataTable, type Column } from '@/components/Table';
import { errorMessage, useToast } from '@/components/Toast';
import { ItemIcon } from '@/components/ItemIcon';

export default function QuestViewPage() {
  return (
    <Suspense fallback={<Loading />}>
      <QuestView />
    </Suspense>
  );
}

function QuestView() {
  const params = useSearchParams();
  const router = useRouter();
  const toast = useToast();
  const id = Number(params.get('id') ?? 0);
  const [quest, setQuest] = useState<QuestDetail | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [confirmDelete, setConfirmDelete] = useState(false);

  const load = useCallback(async () => {
    setError(null);
    try {
      setQuest(await api.quests.get(id));
    } catch (err) {
      setError(errorMessage(err));
    }
  }, [id]);

  useEffect(() => {
    void load();
  }, [load]);

  const progressColumns: Column<QuestDetail['progress'][number]>[] = [
    {
      key: 'player',
      header: 'Player',
      render: (p) => (
        <Link href={`/players/view/?name=${encodeURIComponent(p.player)}`} className="text-accent-300 hover:underline">
          {p.player}
        </Link>
      ),
    },
    {
      key: 'progress',
      header: 'Progress',
      className: 'text-right',
      render: (p) => `${fmtInt(p.progress)} / ${fmtInt(quest?.mission_count ?? 0)}`,
    },
    {
      key: 'state',
      header: 'State',
      render: (p) => (p.complete ? <Badge tone="success">{p.claimed ? 'completed, reward claimed' : 'completed'}</Badge> : <Badge tone="neutral">in progress</Badge>),
    },
    {
      key: 'actions',
      header: '',
      className: 'text-right',
      render: (p) => (
        <div className="flex justify-end gap-1.5">
          {!p.complete && (
            <Button
              size="sm"
              onClick={async () => {
                try {
                  await api.quests.patchPlayer(p.player, id, { progress: quest?.mission_count ?? 0, complete: true });
                  toast.success(`Marked complete for ${p.player}`);
                  void load();
                } catch (err) {
                  toast.error(errorMessage(err));
                }
              }}
            >
              Mark complete
            </Button>
          )}
          <Button
            size="sm"
            onClick={async () => {
              try {
                await api.quests.patchPlayer(p.player, id, { progress: 0, complete: false });
                void load();
              } catch (err) {
                toast.error(errorMessage(err));
              }
            }}
          >
            Reset
          </Button>
          <Button
            size="sm"
            variant="danger"
            onClick={async () => {
              try {
                await api.quests.unassign(p.player, id);
                void load();
              } catch (err) {
                toast.error(errorMessage(err));
              }
            }}
          >
            Remove
          </Button>
        </div>
      ),
    },
  ];

  return (
    <div>
      <div className="mb-4">
        <Link href="/quests/" className="inline-flex items-center gap-1 text-sm text-zinc-400 hover:text-zinc-100">
          <IconArrowLeft className="h-4 w-4" /> All challenges
        </Link>
      </div>

      {error && <ErrorBox message={error} onRetry={load} />}
      {!error && !quest && <Loading />}

      {quest && (
        <div className="space-y-6">
          <PageHeader
            title={
              <span>
                #{quest.id} <span className="text-zinc-400">{quest.description}</span>
              </span>
            }
            subtitle={
              <span className="flex flex-wrap items-center gap-2">
                <Badge tone={quest.quest_type === 1 ? 'warning' : 'neutral'}>{questTypeLabel(quest.quest_type)}</Badge>
                <span>{quest.category}</span>
                {quest.link_id > 0 && (
                  <span className="text-zinc-500">
                    chain {quest.link_id}, step {quest.link_step}
                  </span>
                )}
                <Badge tone={quest.enabled ? 'success' : 'neutral'}>{quest.enabled ? 'enabled' : 'disabled'}</Badge>
              </span>
            }
            actions={
              <Button variant="danger" onClick={() => setConfirmDelete(true)}>
                Delete challenge
              </Button>
            }
          />

          <EditForm quest={quest} onSaved={(q) => { setQuest(q); toast.success('Challenge saved'); }} />

          <AssignCard questId={quest.id} onAssigned={() => void load()} />

          <Card flush title="Player progress" description={`${fmtInt(quest.players)} player${quest.players === 1 ? '' : 's'} have this challenge, ${fmtInt(quest.completed)} completed.`}>
            <DataTable columns={progressColumns} rows={quest.progress} rowKey={(p) => p.player} emptyText="Nobody has this challenge yet." dense />
          </Card>
        </div>
      )}

      {quest && confirmDelete && (
        <ConfirmDialog
          open
          danger
          title={`Delete challenge #${quest.id}?`}
          message="The definition and every player's progress on it are removed. Deleting a row from the client's own table means the server can never hand it out again."
          confirmLabel="Delete"
          onCancel={() => setConfirmDelete(false)}
          onConfirm={async () => {
            try {
              await api.quests.remove(quest.id);
              toast.success('Challenge deleted');
              router.push('/quests/');
            } catch (err) {
              toast.error(errorMessage(err));
              setConfirmDelete(false);
            }
          }}
        />
      )}
    </div>
  );
}

function EditForm({ quest, onSaved }: { quest: QuestDetail; onSaved: (q: QuestDetail) => void }) {
  const toast = useToast();
  const [enabled, setEnabled] = useState(quest.enabled);
  const [type, setType] = useState(quest.quest_type);
  const [category, setCategory] = useState(quest.category);
  const [description, setDescription] = useState(quest.description);
  const [count, setCount] = useState(quest.mission_count);
  const [points, setPoints] = useState(quest.reward_point);
  const [exp, setExp] = useState(quest.reward_exp);
  const [level, setLevel] = useState(quest.create_level);
  const [items, setItems] = useState<number[]>(quest.reward_items.filter((i) => i > 0));
  const [next, setNext] = useState(quest.next_quests.join(', '));
  const [start, setStart] = useState(oleToLocalInput(quest.event_start));
  const [end, setEnd] = useState(oleToLocalInput(quest.event_end));
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    setEnabled(quest.enabled);
    setType(quest.quest_type);
    setCategory(quest.category);
    setDescription(quest.description);
    setCount(quest.mission_count);
    setPoints(quest.reward_point);
    setExp(quest.reward_exp);
    setLevel(quest.create_level);
    setItems(quest.reward_items.filter((i) => i > 0));
    setNext(quest.next_quests.join(', '));
    setStart(oleToLocalInput(quest.event_start));
    setEnd(oleToLocalInput(quest.event_end));
  }, [quest]);

  const save = async () => {
    setBusy(true);
    try {
      const updated = await api.quests.update(quest.id, {
        enabled,
        quest_type: type,
        category: category.trim(),
        description: description.trim(),
        mission_count: count,
        reward_point: points,
        reward_exp: exp,
        create_level: level,
        reward_items: items,
        next_quests: next.split(/[,\s]+/).map((s) => Number(s)).filter((n) => Number.isInteger(n) && n > 0),
        event_start: localInputToOle(start),
        event_end: localInputToOle(end),
      });
      onSaved(updated);
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setBusy(false);
    }
  };

  const addItem = (item: CatalogItem) => {
    if (items.includes(item.id) || items.length >= 2) return;
    setItems([...items, item.id]);
  };

  return (
    <Card
      title="Definition"
      description="The goal, rewards and chain are what the server enforces and pays. The client keeps its own copy of the text and goal for display, so a changed goal is reached when the server says so, not when the client's bar fills."
      actions={
        <Button variant="primary" onClick={save} loading={busy}>
          Save
        </Button>
      }
    >
      <div className="space-y-4">
        <div className="flex flex-wrap items-center gap-4">
          <Checkbox checked={enabled} onChange={(e) => setEnabled(e.target.checked)} label="Enabled (handed out to players)" />
        </div>
        <div className="grid gap-3 sm:grid-cols-4">
          <Select id="q-type" label="Type" value={type} onChange={(e) => setType(Number(e.target.value))}>
            <option value={0}>Basic training</option>
            <option value={1}>Event</option>
            <option value={2}>Daily (resets each day)</option>
            <option value={3}>Chain</option>
            <option value={4}>Proficiency</option>
          </Select>
          <Input id="q-cat" label="Category" value={category} onChange={(e) => setCategory(e.target.value)} />
          <div className="sm:col-span-2">
            <Input id="q-desc" label="Description" value={description} onChange={(e) => setDescription(e.target.value)} />
          </div>
        </div>
        <div className="grid gap-3 sm:grid-cols-4">
          <Input id="q-count" label="Goal (mission count)" type="number" min={1} value={count} onChange={(e) => setCount(Math.max(1, Number(e.target.value)))} />
          <Input id="q-points" label="Reward points" type="number" min={0} value={points} onChange={(e) => setPoints(Math.max(0, Number(e.target.value)))} />
          <Input id="q-exp" label="Reward EXP" type="number" min={0} value={exp} onChange={(e) => setExp(Math.max(0, Number(e.target.value)))} />
          <Input id="q-level" label="Minimum level" type="number" min={0} value={level} onChange={(e) => setLevel(Math.max(0, Number(e.target.value)))} />
        </div>
        <div className="grid gap-3 sm:grid-cols-2">
          <div>
            <label className="mb-1 block text-xs font-medium text-zinc-400">Reward items (up to 2)</label>
            <CatalogSearch inputId="q-item" onPick={addItem} placeholder="Add a reward item" />
            <div className="mt-2 flex flex-wrap gap-2">
              {items.map((i, idx) => (
                <span key={i} className="inline-flex items-center gap-1">
                  <ItemIcon id={i} size="sm" />
                  <Badge tone="accent">{quest.reward_item_names[idx] ?? `item ${i}`}</Badge>
                  <button type="button" className="text-xs text-zinc-500 hover:text-zinc-200" onClick={() => setItems(items.filter((x) => x !== i))}>
                    remove
                  </button>
                </span>
              ))}
              {items.length === 0 && <span className="text-xs text-zinc-600">none</span>}
            </div>
          </div>
          <Input id="q-next" label="Unlocks on completion (quest ids)" value={next} onChange={(e) => setNext(e.target.value)} placeholder="4022, 4030" hint="Comma separated ids of the challenges handed out when this one completes." />
        </div>
        <div className="grid gap-3 sm:grid-cols-2">
          <Input id="q-start" label="Event start (local)" type="datetime-local" value={start} onChange={(e) => setStart(e.target.value)} hint="Leave both empty for a permanent challenge." />
          <Input id="q-end" label="Event end (local)" type="datetime-local" value={end} onChange={(e) => setEnd(e.target.value)} />
        </div>
      </div>
    </Card>
  );
}

function AssignCard({ questId, onAssigned }: { questId: number; onAssigned: () => void }) {
  const toast = useToast();
  const [name, setName] = useState('');
  const [busy, setBusy] = useState(false);
  return (
    <Card title="Hand out to a player" description="Adds this challenge to one player's list right away (normally it is handed out at login by type and level).">
      <form
        onSubmit={async (e) => {
          e.preventDefault();
          if (!name.trim()) return;
          setBusy(true);
          try {
            await api.quests.assign(name.trim(), questId);
            toast.success(`Assigned to ${name.trim()}`);
            setName('');
            onAssigned();
          } catch (err) {
            toast.error(errorMessage(err));
          } finally {
            setBusy(false);
          }
        }}
        className="flex flex-col gap-3 sm:flex-row sm:items-end"
      >
        <div className="flex-1">
          <Input id="assign-name" label="Character name" value={name} onChange={(e) => setName(e.target.value)} autoComplete="off" />
        </div>
        <Button type="submit" loading={busy} disabled={!name.trim()}>
          Assign
        </Button>
      </form>
    </Card>
  );
}
