'use client';

import { useCallback, useEffect, useState, type FormEvent } from 'react';
import { api, type CatalogItem, type Mail, type PlayerSummary } from '@/lib/api';
import { fmtInt } from '@/lib/format';
import { Badge } from '@/components/Badge';
import { Button } from '@/components/Button';
import { Card } from '@/components/Card';
import { CatalogSearch } from '@/components/CatalogSearch';
import { ErrorBox, Loading, PageHeader } from '@/components/Feedback';
import { Checkbox, Input, Select } from '@/components/Input';
import { DataTable, type Column } from '@/components/Table';
import { errorMessage, useToast } from '@/components/Toast';
import { ItemIcon } from '@/components/ItemIcon';

const MSG_LABEL: Record<number, string> = { 1: 'Text', 2: 'Item', 4: 'Points', 5: 'Clan invite', 6: 'Clan request', 7: 'Cash' };

export default function MailPage() {
  const toast = useToast();
  const [mail, setMail] = useState<Mail[] | null>(null);
  const [players, setPlayers] = useState<PlayerSummary[]>([]);
  const [filter, setFilter] = useState('');
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setError(null);
    try {
      const [m, p] = await Promise.all([api.mail.list(filter || undefined), api.players.list().catch(() => [] as PlayerSummary[])]);
      setMail(m);
      setPlayers(p);
    } catch (err) {
      setError(errorMessage(err));
    }
  }, [filter]);

  useEffect(() => {
    void load();
  }, [load]);

  const columns: Column<Mail>[] = [
    { key: 'id', header: 'ID', className: 'w-14 text-zinc-500', hideBelow: 'sm', render: (m) => m.id },
    { key: 'date', header: 'Date', hideBelow: 'md', render: (m) => <span className="text-zinc-400">{m.sent_date}</span> },
    {
      key: 'box',
      header: 'Box',
      render: (m) => (
        <span className="text-zinc-300">
          {m.owner} <span className="text-zinc-600">/ {m.box === 0 ? 'inbox' : 'sent'}</span>
        </span>
      ),
    },
    { key: 'from', header: 'From', render: (m) => m.sender },
    { key: 'to', header: 'To', hideBelow: 'sm', render: (m) => m.recipient },
    {
      key: 'kind',
      header: 'Kind',
      render: (m) => (
        <div className="flex flex-wrap gap-1">
          <Badge tone={m.msg_type === 2 || m.msg_type === 4 ? 'accent' : 'neutral'}>{MSG_LABEL[m.msg_type] ?? `type ${m.msg_type}`}</Badge>
          {m.msg_type === 2 && m.item_id > 0 && (
            <span className="inline-flex items-center gap-1">
              <ItemIcon id={m.item_id} size="xs" hideMissing />
              <Badge tone="warning">
                {m.item_count > 1 ? `${m.item_count}x ` : ''}item {m.item_id}
              </Badge>
            </span>
          )}
          {m.msg_type === 4 && m.points > 0 && <Badge tone="success">{fmtInt(m.points)} pts</Badge>}
          {m.status === 0 && m.box === 0 && <Badge tone="accent">unread</Badge>}
        </div>
      ),
    },
    { key: 'text', header: 'Text', render: (m) => <span className="line-clamp-2 text-zinc-300">{m.text}</span> },
    {
      key: 'actions',
      header: '',
      className: 'text-right',
      render: (m) => (
        <Button
          size="sm"
          variant="danger"
          onClick={async () => {
            try {
              await api.mail.remove(m.id);
              setMail((list) => (list ? list.filter((x) => x.id !== m.id) : list));
            } catch (err) {
              toast.error(errorMessage(err));
            }
          }}
        >
          Delete
        </Button>
      ),
    },
  ];

  return (
    <div>
      <PageHeader title="Mail" subtitle={mail ? `${fmtInt(mail.length)} message${mail.length === 1 ? '' : 's'} on the server` : undefined} />

      {error && <ErrorBox message={error} onRetry={load} />}
      {!error && !mail && <Loading />}

      {mail && (
        <div className="space-y-6">
          <SendMailForm
            players={players}
            onSent={(delivered, failed) => {
              if (delivered.length > 0) toast.success(`Delivered to ${delivered.length} player${delivered.length === 1 ? '' : 's'}`);
              if (failed.length > 0) toast.error(`Not delivered: ${failed.join(', ')}`);
              void load();
            }}
          />
          <Card
            flush
            title="Messages"
            description="Every mailbox on the server. Players see their inbox under Mail in the lobby; attachments are taken from the message."
            actions={
              <Select id="mail-filter" selectSize="sm" value={filter} onChange={(e) => setFilter(e.target.value)}>
                <option value="">All players</option>
                {players.map((p) => (
                  <option key={p.name} value={p.name}>
                    {p.name}
                  </option>
                ))}
              </Select>
            }
          >
            <DataTable columns={columns} rows={mail} rowKey={(m) => m.id} emptyText="No mail." dense />
          </Card>
        </div>
      )}
    </div>
  );
}

function SendMailForm({ players, onSent }: { players: PlayerSummary[]; onSent: (delivered: string[], failed: string[]) => void }) {
  const toast = useToast();
  const [everyone, setEveryone] = useState(false);
  const [to, setTo] = useState('');
  const [sender, setSender] = useState('ACEmu');
  const [text, setText] = useState('');
  const [item, setItem] = useState<CatalogItem | null>(null);
  const [itemCount, setItemCount] = useState(1);
  const [points, setPoints] = useState(0);
  const [busy, setBusy] = useState(false);

  const recipients = everyone ? ['*'] : to.split(/[,/;\s]+/).map((s) => s.trim()).filter(Boolean);
  const valid = recipients.length > 0 && text.trim() !== '';

  const submit = async (e: FormEvent) => {
    e.preventDefault();
    if (!valid) return;
    setBusy(true);
    try {
      const out = await api.mail.send({
        to: recipients,
        sender: sender.trim() || 'ACEmu',
        text: text.trim(),
        item_id: item?.id ?? 0,
        item_count: itemCount,
        points,
      });
      onSent(out.delivered, out.failed);
      setText('');
      setItem(null);
      setPoints(0);
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <Card title="Send mail" description="A system message to one player, several (comma separated) or everyone, optionally with an item or points attached.">
      <form onSubmit={submit} className="space-y-4">
        <div className="grid gap-3 sm:grid-cols-2">
          <div>
            <Input
              id="mail-to"
              label="To"
              value={to}
              onChange={(e) => setTo(e.target.value)}
              placeholder="MaxA, MaxB"
              disabled={everyone}
              list="mail-players"
              autoComplete="off"
            />
            <datalist id="mail-players">
              {players.map((p) => (
                <option key={p.name} value={p.name} />
              ))}
            </datalist>
            <div className="mt-2">
              <Checkbox checked={everyone} onChange={(e) => setEveryone(e.target.checked)} label={`Everyone (${fmtInt(players.length)} players)`} />
            </div>
          </div>
          <Input id="mail-sender" label="Sender name shown" value={sender} onChange={(e) => setSender(e.target.value)} />
        </div>
        <div>
          <label htmlFor="mail-text" className="mb-1 block text-xs font-medium text-zinc-400">
            Message
          </label>
          <textarea
            id="mail-text"
            value={text}
            onChange={(e) => setText(e.target.value)}
            rows={3}
            className="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-2 text-sm text-zinc-100 placeholder-zinc-600 focus:border-accent-500 focus:outline-none"
            placeholder="Welcome to the server!"
          />
        </div>
        <div className="grid gap-3 sm:grid-cols-3">
          <div>
            <label className="mb-1 block text-xs font-medium text-zinc-400">Attach item (optional)</label>
            <CatalogSearch inputId="mail-item" onPick={setItem} selectedId={item?.id ?? null} placeholder="Search an item" />
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
          <Input id="mail-count" label="Item count" type="number" min={1} value={itemCount} onChange={(e) => setItemCount(Math.max(1, Number(e.target.value)))} disabled={!item} />
          <Input id="mail-points" label="Points (optional)" type="number" min={0} value={points} onChange={(e) => setPoints(Math.max(0, Number(e.target.value)))} disabled={!!item} />
        </div>
        <div className="flex justify-end">
          <Button type="submit" variant="primary" loading={busy} disabled={!valid}>
            Send
          </Button>
        </div>
      </form>
    </Card>
  );
}
