'use client';

import Link from 'next/link';
import { useRouter } from 'next/navigation';
import { useCallback, useEffect, useMemo, useState, type FormEvent } from 'react';
import { api, localInputToOle, oleToLocalInput, questTypeLabel, type CatalogItem, type Quest } from '@/lib/api';
import { fmtInt } from '@/lib/format';
import { Badge } from '@/components/Badge';
import { Button } from '@/components/Button';
import { Card } from '@/components/Card';
import { CatalogSearch } from '@/components/CatalogSearch';
import { ErrorBox, Loading, PageHeader } from '@/components/Feedback';
import { useDebounced } from '@/lib/hooks';
import { Input, Select } from '@/components/Input';
import { DataTable, type Column } from '@/components/Table';
import { errorMessage, useToast } from '@/components/Toast';
import { ItemIcon } from '@/components/ItemIcon';

function questHref(id: number): string {
  return `/quests/view/?id=${id}`;
}

function fmtWindow(q: Quest): string {
  if (q.event_start <= 0 && q.event_end <= 0) return '';
  const a = oleToLocalInput(q.event_start).replace('T', ' ');
  const b = oleToLocalInput(q.event_end).replace('T', ' ');
  return `${a || '...'} → ${b || '...'}`;
}

export default function QuestsPage() {
  const router = useRouter();
  const toast = useToast();
  const [quests, setQuests] = useState<Quest[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [q, setQ] = useState('');
  const [type, setType] = useState<number | ''>('');
  const [filter, setFilter] = useState('');
  const needle = useDebounced(q, 250);

  const load = useCallback(async () => {
    setError(null);
    try {
      setQuests(await api.quests.list({ q: needle, quest_type: type, filter }));
    } catch (err) {
      setError(errorMessage(err));
    }
  }, [needle, type, filter]);

  useEffect(() => {
    void load();
  }, [load]);

  const events = useMemo(() => (quests ?? []).filter((x) => x.event_start > 0 || x.event_end > 0), [quests]);

  const toggle = async (quest: Quest) => {
    try {
      const updated = await api.quests.update(quest.id, { enabled: !quest.enabled });
      setQuests((list) => (list ? list.map((x) => (x.id === quest.id ? { ...x, enabled: updated.enabled } : x)) : list));
    } catch (err) {
      toast.error(errorMessage(err));
    }
  };

  const columns: Column<Quest>[] = [
    { key: 'id', header: 'ID', className: 'w-16 text-zinc-500', render: (x) => x.id },
    {
      key: 'type',
      header: 'Type',
      hideBelow: 'sm',
      render: (x) => <Badge tone={x.quest_type === 1 ? 'warning' : x.quest_type === 2 ? 'accent' : 'neutral'}>{questTypeLabel(x.quest_type)}</Badge>,
    },
    { key: 'category', header: 'Category', hideBelow: 'md', render: (x) => <span className="text-zinc-300">{x.category}</span> },
    {
      key: 'desc',
      header: 'Challenge',
      render: (x) => (
        <Link href={questHref(x.id)} className="font-medium text-accent-300 hover:underline" onClick={(e) => e.stopPropagation()}>
          {x.description}
        </Link>
      ),
    },
    { key: 'count', header: 'Goal', className: 'text-right', hideBelow: 'sm', render: (x) => fmtInt(x.mission_count) },
    {
      key: 'reward',
      header: 'Reward',
      hideBelow: 'lg',
      render: (x) => (
        <div className="flex flex-wrap gap-1">
          {x.reward_point > 0 && <Badge tone="success">{fmtInt(x.reward_point)} pts</Badge>}
          {x.reward_exp > 0 && <Badge tone="neutral">{fmtInt(x.reward_exp)} exp</Badge>}
          {x.reward_item_names.map((n, i) => (
            <span key={`${n}-${i}`} className="inline-flex items-center gap-1">
              <ItemIcon id={x.reward_items[i]} size="xs" hideMissing />
              <Badge tone="accent">{n}</Badge>
            </span>
          ))}
        </div>
      ),
    },
    {
      key: 'window',
      header: 'Event window',
      hideBelow: 'lg',
      render: (x) => {
        const w = fmtWindow(x);
        return w ? <span className={x.active ? 'text-emerald-300' : 'text-zinc-500'}>{w}</span> : <span className="text-zinc-600">-</span>;
      },
    },
    {
      key: 'enabled',
      header: 'Enabled',
      className: 'text-right',
      render: (x) => (
        <Button size="sm" variant={x.enabled ? 'secondary' : 'ghost'} onClick={(e) => { e.stopPropagation(); void toggle(x); }}>
          {x.enabled ? 'On' : 'Off'}
        </Button>
      ),
    },
  ];

  return (
    <div>
      <PageHeader
        title="Challenges & events"
        subtitle="The client's quest table. Enabled chain heads are handed to players at login; dailies reset each day. Give any challenge a date window to run it as a special event."
      />

      {error && <ErrorBox message={error} onRetry={load} />}
      {!error && !quests && <Loading />}

      {quests && (
        <div className="space-y-6">
          <Card
            title="Special events"
            description="Challenges with a date window. While the window is open the server assigns them to everyone at login and lists them as events in the lobby."
          >
            {events.length === 0 ? (
              <p className="text-sm text-zinc-500">No event windows set. Open a challenge and set its start and end, or create a custom one below.</p>
            ) : (
              <ul className="divide-y divide-zinc-800">
                {events.map((x) => (
                  <li key={x.id} className="flex flex-wrap items-center justify-between gap-2 py-2">
                    <div>
                      <Link href={questHref(x.id)} className="font-medium text-accent-300 hover:underline">
                        {x.description}
                      </Link>
                      <div className="text-xs text-zinc-500">
                        #{x.id} · {x.category} · goal {fmtInt(x.mission_count)}
                      </div>
                    </div>
                    <div className="flex items-center gap-2 text-xs">
                      <span className={x.active ? 'text-emerald-300' : 'text-zinc-500'}>{fmtWindow(x)}</span>
                      <Badge tone={x.active && x.enabled ? 'success' : 'neutral'}>{x.enabled ? (x.active ? 'live' : 'scheduled') : 'disabled'}</Badge>
                    </div>
                  </li>
                ))}
              </ul>
            )}
          </Card>

          <CreateEventForm
            onCreated={(id) => {
              toast.success(`Challenge #${id} created`);
              router.push(questHref(id));
            }}
          />

          <Card
            flush
            title="All challenges"
            description={`${fmtInt(quests.length)} shown`}
            actions={
              <div className="flex flex-wrap gap-2">
                <Input id="quest-q" inputSize="sm" value={q} onChange={(e) => setQ(e.target.value)} placeholder="Search text, category or id" />
                <Select id="quest-type" selectSize="sm" value={type} onChange={(e) => setType(e.target.value === '' ? '' : Number(e.target.value))}>
                  <option value="">All types</option>
                  <option value={0}>Basic training</option>
                  <option value={1}>Event</option>
                  <option value={2}>Daily</option>
                  <option value={3}>Chain</option>
                  <option value={4}>Proficiency</option>
                </Select>
                <Select id="quest-filter" selectSize="sm" value={filter} onChange={(e) => setFilter(e.target.value)}>
                  <option value="">Any state</option>
                  <option value="events">With event window</option>
                  <option value="enabled">Enabled</option>
                  <option value="disabled">Disabled</option>
                </Select>
              </div>
            }
          >
            <DataTable columns={columns} rows={quests} rowKey={(x) => x.id} onRowClick={(x) => router.push(questHref(x.id))} emptyText="No challenges match." dense />
          </Card>
        </div>
      )}
    </div>
  );
}

function CreateEventForm({ onCreated }: { onCreated: (id: number) => void }) {
  const toast = useToast();
  const [id, setId] = useState(90001);
  const [category, setCategory] = useState('Special event');
  const [description, setDescription] = useState('');
  const [count, setCount] = useState(10);
  const [points, setPoints] = useState(1000);
  const [exp, setExp] = useState(0);
  const [item, setItem] = useState<CatalogItem | null>(null);
  const [start, setStart] = useState('');
  const [end, setEnd] = useState('');
  const [busy, setBusy] = useState(false);
  const valid = id > 0 && description.trim() !== '' && count > 0;

  const submit = async (e: FormEvent) => {
    e.preventDefault();
    if (!valid) return;
    setBusy(true);
    try {
      const created = await api.quests.create({
        id,
        quest_type: 3,
        category: category.trim() || 'Special event',
        description: description.trim(),
        mission_count: count,
        reward_point: points,
        reward_exp: exp,
        reward_items: item ? [item.id] : [],
        event_start: localInputToOle(start),
        event_end: localInputToOle(end),
        enabled: true,
        link_step: 1,
      });
      onCreated(created.id);
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <Card
      title="Create a custom challenge"
      description="Progress is counted by the client for the mission types it knows, so a custom row is only useful as a reward holder or a copy of an existing challenge with a new goal and window. The client shows its own text for ids it does not know."
    >
      <form onSubmit={submit} className="space-y-4">
        <div className="grid gap-3 sm:grid-cols-4">
          <Input id="ev-id" label="ID (unused)" type="number" min={1} value={id} onChange={(e) => setId(Number(e.target.value))} />
          <Input id="ev-cat" label="Category" value={category} onChange={(e) => setCategory(e.target.value)} />
          <div className="sm:col-span-2">
            <Input id="ev-desc" label="Description" value={description} onChange={(e) => setDescription(e.target.value)} placeholder="Kill 50 enemies during the weekend" required />
          </div>
        </div>
        <div className="grid gap-3 sm:grid-cols-4">
          <Input id="ev-count" label="Goal" type="number" min={1} value={count} onChange={(e) => setCount(Math.max(1, Number(e.target.value)))} />
          <Input id="ev-points" label="Reward points" type="number" min={0} value={points} onChange={(e) => setPoints(Math.max(0, Number(e.target.value)))} />
          <Input id="ev-exp" label="Reward EXP" type="number" min={0} value={exp} onChange={(e) => setExp(Math.max(0, Number(e.target.value)))} />
          <div>
            <label className="mb-1 block text-xs font-medium text-zinc-400">Reward item</label>
            <CatalogSearch inputId="ev-item" onPick={setItem} selectedId={item?.id ?? null} placeholder="Search" />
            {item && (
              <div className="mt-1 flex items-center gap-2 text-xs text-zinc-400">
                <ItemIcon id={item.id} size="sm" />
                {item.name}
              </div>
            )}
          </div>
        </div>
        <div className="grid gap-3 sm:grid-cols-2">
          <Input id="ev-start" label="Event start (local)" type="datetime-local" value={start} onChange={(e) => setStart(e.target.value)} />
          <Input id="ev-end" label="Event end (local)" type="datetime-local" value={end} onChange={(e) => setEnd(e.target.value)} />
        </div>
        <div className="flex justify-end">
          <Button type="submit" variant="primary" loading={busy} disabled={!valid}>
            Create
          </Button>
        </div>
      </form>
    </Card>
  );
}
