'use client';

import { useEffect, useRef, useState, type ReactNode } from 'react';
import { api, type Live, type LiveMember, type LiveRoom, type LiveSession } from '@/lib/api';
import { fmtTeam, fmtUnix } from '@/lib/format';
import { Badge } from './Badge';
import { Card } from './Card';
import { DataTable, type Column } from './Table';
import { errorMessage } from './Toast';

const POLL_MS = 3000;

/** Live game-server state, polled every 3 seconds. */
export function LivePanel() {
  const [live, setLive] = useState<Live | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [updatedAt, setUpdatedAt] = useState<Date | null>(null);
  const inFlight = useRef(false);

  useEffect(() => {
    let cancelled = false;

    const tick = async () => {
      if (inFlight.current) return;
      inFlight.current = true;
      try {
        const data = await api.live();
        if (cancelled) return;
        setLive(data);
        setError(null);
        setUpdatedAt(new Date());
      } catch (err) {
        if (!cancelled) setError(errorMessage(err));
      } finally {
        inFlight.current = false;
      }
    };

    void tick();
    const timer = window.setInterval(() => void tick(), POLL_MS);
    return () => {
      cancelled = true;
      window.clearInterval(timer);
    };
  }, []);

  const online = live?.online ?? false;

  return (
    <section className="space-y-4">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div className="flex flex-wrap items-center gap-3">
          <h2 className="text-base font-semibold text-zinc-100">Live</h2>
          <StatusPill live={live} error={error} />
        </div>
        <div className="text-xs text-zinc-500">
          {error ? (
            <span className="text-rose-300">{error}</span>
          ) : updatedAt ? (
            <>Updated {updatedAt.toLocaleTimeString()} - polling every {POLL_MS / 1000}s</>
          ) : (
            'Connecting...'
          )}
        </div>
      </div>

      <div className="grid grid-cols-1 gap-4 xl:grid-cols-5">
        <Card
          flush
          className="xl:col-span-2"
          title="Connected players"
          description={live ? `${live.sessions.length} session${live.sessions.length === 1 ? '' : 's'}` : undefined}
        >
          <SessionsTable sessions={live?.sessions ?? []} online={online} loaded={live !== null} />
        </Card>

        <div className="space-y-4 xl:col-span-3">
          {live && live.online && live.rooms.length > 0 ? (
            live.rooms.map((room) => <RoomCard key={room.number} live={live} room={room} />)
          ) : (
            <RoomCard live={live} room={null} />
          )}
        </div>
      </div>
    </section>
  );
}

function StatusPill({ live, error }: { live: Live | null; error: string | null }) {
  if (!live) {
    return (
      <Badge tone={error ? 'danger' : 'neutral'}>
        <Dot className={error ? 'bg-rose-400' : 'bg-zinc-500'} />
        {error ? 'Unreachable' : 'Checking'}
      </Badge>
    );
  }
  if (!live.online) {
    return (
      <Badge tone="danger">
        <Dot className="bg-rose-400" />
        Offline
      </Badge>
    );
  }
  return (
    <Badge tone="success">
      <Dot className="bg-emerald-400 animate-pulse" />
      Online{live.started_at !== null ? ` since ${fmtUnix(live.started_at)}` : ''}
      {live.heartbeat_age_secs !== null && live.heartbeat_age_secs > 10 && (
        <span className="ml-1 text-emerald-200/70">(heartbeat {Math.round(live.heartbeat_age_secs)}s ago)</span>
      )}
    </Badge>
  );
}

function Dot({ className }: { className: string }) {
  return <span className={`mr-1.5 inline-block h-2 w-2 rounded-full ${className}`} aria-hidden="true" />;
}

const SESSION_COLUMNS: Column<LiveSession>[] = [
  { key: 'uid', header: 'UID', className: 'w-16 text-zinc-500', render: (s) => s.uid },
  {
    key: 'name',
    header: 'Name',
    render: (s) => (s.name ? <span className="font-medium text-zinc-100">{s.name}</span> : <span className="italic text-zinc-500">(connecting)</span>),
  },
  {
    key: 'where',
    header: 'Where',
    render: (s) => (
      <div className="flex flex-wrap gap-1">
        {s.in_room ? <Badge tone="accent">In room</Badge> : s.in_channel ? <Badge tone="success">In lobby</Badge> : <Badge tone="neutral">Login</Badge>}
      </div>
    ),
  },
  { key: 'nat', header: 'NAT', className: 'text-right text-zinc-400', render: (s) => s.nat_type },
];

function SessionsTable({ sessions, online, loaded }: { sessions: LiveSession[]; online: boolean; loaded: boolean }) {
  return (
    <DataTable
      dense
      columns={SESSION_COLUMNS}
      rows={sessions}
      rowKey={(s) => s.uid}
      emptyText={!loaded ? 'Waiting for the server...' : online ? 'Nobody connected.' : 'Game server is offline.'}
    />
  );
}

const MEMBER_COLUMNS: Column<LiveMember>[] = [
  {
    key: 'name',
    header: 'Name',
    render: (m) => (
      <span className="font-medium text-zinc-100">
        {m.name || <span className="italic text-zinc-500">uid {m.uid}</span>}
      </span>
    ),
  },
  {
    key: 'team',
    header: 'Team',
    render: (m) => <Badge tone={m.team === 0 ? 'accent' : 'warning'}>{fmtTeam(m.team)}</Badge>,
  },
  {
    key: 'ready',
    header: 'Ready',
    render: (m) => (m.state !== 0 ? <Badge tone="success">Ready</Badge> : <span className="text-zinc-500">-</span>),
  },
  {
    key: 'host',
    header: 'Host',
    render: (m) => (m.is_owner ? <Badge tone="neutral">Host</Badge> : <span className="text-zinc-500">-</span>),
  },
  {
    key: 'udp',
    header: 'UDP',
    hideBelow: 'sm',
    render: (m) => (m.udp ? <code className="text-xs text-zinc-300">{m.udp}</code> : <span className="text-zinc-600">-</span>),
  },
];

function RoomCard({ live, room }: { live: Live | null; room: LiveRoom | null }) {
  if (!room) {
    return (
      <Card title="Rooms" className="h-full">
        <p className="text-sm text-zinc-500">{!live ? 'Waiting for the server...' : !live.online ? 'Game server is offline.' : 'No room open.'}</p>
      </Card>
    );
  }

  return (
    <Card
      flush
      title={
        <span className="flex flex-wrap items-center gap-2">
          <span>Room</span>
          <span className="text-zinc-500">#{room.number}</span>
        </span>
      }
      actions={room.playing ? <Badge tone="success">In match</Badge> : <Badge tone="accent">In lobby</Badge>}
    >
      <dl className="grid grid-cols-2 gap-x-4 gap-y-3 border-b border-zinc-800 px-4 py-3 text-sm sm:grid-cols-4 sm:px-5">
        <Field label="Name" value={room.name || <span className="text-zinc-500">(unnamed)</span>} />
        <Field label="Map" value={room.map || '-'} />
        <Field label="Mode" value={<span title={`mode ${room.mode_num}`}>{room.mode || `mode ${room.mode_num}`}</span>} />
        <Field label="Owner" value={room.owner_name || `uid ${room.owner_uid}`} />
        <Field
          label="Members"
          value={
            <>
              {room.members.length} <span className="text-zinc-500">/ {room.max_users}</span>
            </>
          }
        />
      </dl>
      <DataTable dense columns={MEMBER_COLUMNS} rows={room.members} rowKey={(m) => m.uid} emptyText="No members." />
    </Card>
  );
}

function Field({ label, value }: { label: string; value: ReactNode }) {
  return (
    <div className="min-w-0">
      <dt className="text-xs font-medium uppercase tracking-wide text-zinc-500">{label}</dt>
      <dd className="truncate text-zinc-100">{value}</dd>
    </div>
  );
}
