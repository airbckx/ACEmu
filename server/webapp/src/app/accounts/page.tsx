'use client';

import Link from 'next/link';
import { useCallback, useEffect, useState, type FormEvent } from 'react';
import { api, type Account, type PlayerSummary } from '@/lib/api';
import { fmtDate, fmtInt } from '@/lib/format';
import { Badge } from '@/components/Badge';
import { Button } from '@/components/Button';
import { Card } from '@/components/Card';
import { ConfirmDialog } from '@/components/ConfirmDialog';
import { ErrorBox, Loading, PageHeader } from '@/components/Feedback';
import { Checkbox, Input, Select } from '@/components/Input';
import { Modal } from '@/components/Modal';
import { DataTable, type Column } from '@/components/Table';
import { errorMessage, useToast } from '@/components/Toast';

type Dialog =
  | { kind: 'password'; account: Account }
  | { kind: 'rename'; account: Account }
  | { kind: 'link'; account: Account }
  | { kind: 'delete'; account: Account }
  | null;

export default function AccountsPage() {
  const toast = useToast();
  const [accounts, setAccounts] = useState<Account[] | null>(null);
  const [players, setPlayers] = useState<PlayerSummary[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [dialog, setDialog] = useState<Dialog>(null);

  const load = useCallback(async () => {
    setError(null);
    try {
      const [acc, pl] = await Promise.all([api.accounts.list(), api.players.list().catch(() => [] as PlayerSummary[])]);
      setAccounts(acc);
      setPlayers(pl);
    } catch (err) {
      setError(errorMessage(err));
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  const replaceAccount = (updated: Account) => {
    setAccounts((list) => (list ? list.map((a) => (a.id === updated.id ? updated : a)) : list));
  };

  const columns: Column<Account>[] = [
    { key: 'id', header: 'ID', className: 'w-14 text-zinc-500', hideBelow: 'sm', render: (a) => a.id },
    { key: 'username', header: 'Username', render: (a) => <span className="font-medium text-zinc-100">{a.username}</span> },
    {
      key: 'password',
      header: 'Password',
      render: (a) => (a.has_password ? <Badge tone="success">Set</Badge> : <Badge tone="neutral">None</Badge>),
    },
    {
      key: 'character',
      header: 'Character',
      render: (a) =>
        a.character ? (
          <Link href={`/players/view/?name=${encodeURIComponent(a.character)}`} className="text-accent-300 hover:underline">
            {a.character}
          </Link>
        ) : (
          <span className="text-zinc-600">-</span>
        ),
    },
    { key: 'created', header: 'Created', hideBelow: 'md', render: (a) => <span className="text-zinc-400">{fmtDate(a.created_at)}</span> },
    {
      key: 'actions',
      header: '',
      className: 'text-right',
      render: (a) => (
        <div className="flex flex-wrap justify-end gap-1.5">
          <Button size="sm" onClick={() => setDialog({ kind: 'password', account: a })}>
            Set password
          </Button>
          <Button size="sm" onClick={() => setDialog({ kind: 'rename', account: a })}>
            Rename
          </Button>
          <Button size="sm" onClick={() => setDialog({ kind: 'link', account: a })}>
            Link character
          </Button>
          <Button size="sm" variant="danger" onClick={() => setDialog({ kind: 'delete', account: a })}>
            Delete
          </Button>
        </div>
      ),
    },
  ];

  return (
    <div>
      <PageHeader
        title="Accounts"
        subtitle={accounts ? `${fmtInt(accounts.length)} login account${accounts.length === 1 ? '' : 's'}` : undefined}
      />

      {error && <ErrorBox message={error} onRetry={load} />}
      {!error && !accounts && <Loading />}

      {accounts && (
        <div className="space-y-6">
          <CreateAccountForm
            onCreated={(a) => {
              setAccounts((list) => (list ? [...list, a] : [a]));
              toast.success(`Account "${a.username}" created`);
            }}
          />

          <Card flush title="All accounts">
            <DataTable columns={columns} rows={accounts} rowKey={(a) => a.id} emptyText="No accounts yet." />
          </Card>
        </div>
      )}

      {dialog?.kind === 'password' && (
        <PasswordDialog
          account={dialog.account}
          onClose={() => setDialog(null)}
          onSaved={(a, cleared) => {
            replaceAccount(a);
            setDialog(null);
            toast.success(cleared ? `Password cleared for "${a.username}"` : `Password updated for "${a.username}"`);
          }}
        />
      )}
      {dialog?.kind === 'rename' && (
        <RenameDialog
          account={dialog.account}
          onClose={() => setDialog(null)}
          onSaved={(a) => {
            replaceAccount(a);
            setDialog(null);
            toast.success(`Renamed to "${a.username}"`);
          }}
        />
      )}
      {dialog?.kind === 'link' && (
        <LinkDialog
          account={dialog.account}
          players={players}
          onClose={() => setDialog(null)}
          onSaved={(a) => {
            replaceAccount(a);
            setDialog(null);
            toast.success(a.character ? `"${a.username}" now plays ${a.character}` : `Character unlinked from "${a.username}"`);
          }}
        />
      )}
      {dialog?.kind === 'delete' && (
        <DeleteDialog
          account={dialog.account}
          onClose={() => setDialog(null)}
          onDeleted={(id, withPlayer) => {
            setAccounts((list) => (list ? list.filter((a) => a.id !== id) : list));
            setDialog(null);
            toast.success(withPlayer ? 'Account and player record deleted' : 'Account deleted');
            if (withPlayer) void load();
          }}
        />
      )}
    </div>
  );
}

// ---------------------------------------------------------------------------

function CreateAccountForm({ onCreated }: { onCreated: (a: Account) => void }) {
  const toast = useToast();
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [busy, setBusy] = useState(false);

  const submit = async (e: FormEvent) => {
    e.preventDefault();
    const name = username.trim();
    if (!name) return;
    setBusy(true);
    try {
      const account = await api.accounts.create({ username: name, password });
      onCreated(account);
      setUsername('');
      setPassword('');
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <Card title="Create account" description="Leave the password empty to allow logging in without one.">
      <form onSubmit={submit} className="flex flex-col gap-3 sm:flex-row sm:items-end">
        <div className="flex-1">
          <Input id="new-username" label="Username" value={username} onChange={(e) => setUsername(e.target.value)} required autoComplete="off" />
        </div>
        <div className="flex-1">
          <Input id="new-password" label="Password" type="password" value={password} onChange={(e) => setPassword(e.target.value)} autoComplete="new-password" placeholder="(none)" />
        </div>
        <Button type="submit" variant="primary" loading={busy} disabled={!username.trim()}>
          Create
        </Button>
      </form>
    </Card>
  );
}

function PasswordDialog({ account, onClose, onSaved }: { account: Account; onClose: () => void; onSaved: (a: Account, cleared: boolean) => void }) {
  const toast = useToast();
  const [password, setPassword] = useState('');
  const [busy, setBusy] = useState(false);

  const save = async () => {
    setBusy(true);
    try {
      const updated = await api.accounts.update(account.id, { password });
      onSaved(updated, password === '');
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal
      open
      title={`Set password for "${account.username}"`}
      onClose={onClose}
      footer={
        <>
          <Button variant="ghost" onClick={onClose} disabled={busy}>
            Cancel
          </Button>
          <Button variant="primary" onClick={save} loading={busy}>
            {password === '' ? 'Clear password' : 'Save password'}
          </Button>
        </>
      }
    >
      <form
        onSubmit={(e) => {
          e.preventDefault();
          void save();
        }}
      >
        <Input
          id="set-password"
          label="New password"
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          autoComplete="new-password"
          autoFocus
          hint="Leave empty to remove the password."
        />
      </form>
    </Modal>
  );
}

function RenameDialog({ account, onClose, onSaved }: { account: Account; onClose: () => void; onSaved: (a: Account) => void }) {
  const toast = useToast();
  const [username, setUsername] = useState(account.username);
  const [busy, setBusy] = useState(false);
  const name = username.trim();
  const dirty = name !== '' && name !== account.username;

  const save = async () => {
    if (!dirty) return;
    setBusy(true);
    try {
      onSaved(await api.accounts.update(account.id, { username: name }));
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal
      open
      title={`Rename "${account.username}"`}
      onClose={onClose}
      footer={
        <>
          <Button variant="ghost" onClick={onClose} disabled={busy}>
            Cancel
          </Button>
          <Button variant="primary" onClick={save} loading={busy} disabled={!dirty}>
            Rename
          </Button>
        </>
      }
    >
      <form
        onSubmit={(e) => {
          e.preventDefault();
          void save();
        }}
      >
        <Input id="rename-username" label="Username" value={username} onChange={(e) => setUsername(e.target.value)} autoFocus autoComplete="off" />
      </form>
    </Modal>
  );
}

function LinkDialog({
  account,
  players,
  onClose,
  onSaved,
}: {
  account: Account;
  players: PlayerSummary[];
  onClose: () => void;
  onSaved: (a: Account) => void;
}) {
  const toast = useToast();
  const [value, setValue] = useState(account.character ?? '');
  const [busy, setBusy] = useState(false);
  const trimmed = value.trim();
  const dirty = trimmed !== (account.character ?? '');
  const knownNames = players.map((p) => p.name);
  const selectValue = knownNames.includes(trimmed) ? trimmed : trimmed === '' ? '' : '__custom__';

  const save = async () => {
    if (!dirty) return;
    setBusy(true);
    try {
      onSaved(await api.accounts.update(account.id, { character: trimmed === '' ? null : trimmed }));
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal
      open
      title={`Link character to "${account.username}"`}
      onClose={onClose}
      footer={
        <>
          <Button variant="ghost" onClick={onClose} disabled={busy}>
            Cancel
          </Button>
          <Button variant="primary" onClick={save} loading={busy} disabled={!dirty}>
            {trimmed === '' ? 'Unlink' : 'Link'}
          </Button>
        </>
      }
    >
      <div className="space-y-3">
        <Select
          id="link-select"
          label="Existing player"
          value={selectValue}
          onChange={(e) => {
            const v = e.target.value;
            if (v === '__custom__') return;
            setValue(v);
          }}
        >
          <option value="">(none - unlink)</option>
          {knownNames.map((n) => (
            <option key={n} value={n}>
              {n}
            </option>
          ))}
          {selectValue === '__custom__' && <option value="__custom__">(custom name)</option>}
        </Select>
        <Input
          id="link-name"
          label="Or type a character name"
          value={value}
          onChange={(e) => setValue(e.target.value)}
          autoComplete="off"
          hint="A name that does not exist yet is created when the account first logs in."
        />
      </div>
    </Modal>
  );
}

function DeleteDialog({ account, onClose, onDeleted }: { account: Account; onClose: () => void; onDeleted: (id: number, withPlayer: boolean) => void }) {
  const toast = useToast();
  const [withPlayer, setWithPlayer] = useState(false);

  return (
    <ConfirmDialog
      open
      danger
      title={`Delete account "${account.username}"?`}
      message="The login is removed permanently. Players keep their data unless you also tick the box below."
      confirmLabel="Delete"
      onCancel={onClose}
      onConfirm={async () => {
        try {
          await api.accounts.remove(account.id, withPlayer);
          onDeleted(account.id, withPlayer);
        } catch (err) {
          toast.error(errorMessage(err));
        }
      }}
    >
      <Checkbox
        checked={withPlayer}
        onChange={(e) => setWithPlayer(e.target.checked)}
        disabled={!account.character}
        label={
          account.character ? (
            <>
              Also delete the player record <span className="font-medium text-zinc-100">{account.character}</span>
            </>
          ) : (
            'Also delete the player record (no character linked)'
          )
        }
      />
    </ConfirmDialog>
  );
}
