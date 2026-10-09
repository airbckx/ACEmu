'use client';

import { useEffect, useRef, useState } from 'react';
import { api, type ServerStatus } from '@/lib/api';
import { fmtUnix } from '@/lib/format';
import { Badge } from './Badge';
import { Card } from './Card';
import { errorMessage } from './Toast';

const POLL_MS = 5000;

/** Which services are operational, probed by the admin API every 5 seconds. */
export function ServersPanel() {
  const [servers, setServers] = useState<ServerStatus[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [updatedAt, setUpdatedAt] = useState<Date | null>(null);
  const inFlight = useRef(false);

  useEffect(() => {
    let cancelled = false;
    const tick = async () => {
      if (inFlight.current) return;
      inFlight.current = true;
      try {
        const data = await api.servers();
        if (cancelled) return;
        setServers(data);
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

  const allUp = servers ? servers.every((s) => s.operational) : false;

  return (
    <section className="space-y-3">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div className="flex items-center gap-3">
          <h2 className="text-base font-semibold text-zinc-100">Servers</h2>
          {servers && <Badge tone={allUp ? 'success' : 'danger'}>{allUp ? 'all operational' : `${servers.filter((s) => !s.operational).length} with problems`}</Badge>}
        </div>
        <div className="text-xs text-zinc-500">
          {error ? <span className="text-rose-300">{error}</span> : updatedAt ? <>Checked {updatedAt.toLocaleTimeString()} - every {POLL_MS / 1000}s</> : 'Checking...'}
        </div>
      </div>
      <div className="grid grid-cols-1 gap-4 lg:grid-cols-3">
        {(servers ?? []).map((s) => (
          <Card key={s.id} className={s.operational ? '' : 'border-rose-500/40'}>
            <div className="flex items-start justify-between gap-2">
              <div>
                <div className="flex items-center gap-2">
                  <span className={`inline-block h-2.5 w-2.5 rounded-full ${s.operational ? 'bg-emerald-400' : 'bg-rose-400'}`} />
                  <span className="text-sm font-semibold text-zinc-100">{s.name}</span>
                </div>
                <p className="mt-1 text-xs text-zinc-500">{s.description}</p>
              </div>
              <Badge tone={s.operational ? 'success' : 'danger'}>{s.operational ? 'up' : 'down'}</Badge>
            </div>
            <ul className="mt-3 space-y-1.5">
              {s.checks.map((c) => (
                <li key={c.label} className="flex items-start gap-2 text-xs">
                  <span className={`mt-0.5 inline-block h-2 w-2 flex-none rounded-full ${c.ok ? 'bg-emerald-400' : 'bg-rose-400'}`} />
                  <span className="text-zinc-300">{c.label}</span>
                  <span className="text-zinc-600">{c.target}</span>
                  <span className={`ml-auto text-right ${c.ok ? 'text-zinc-400' : 'text-rose-300'}`}>{c.detail}</span>
                </li>
              ))}
            </ul>
            {s.started_at && <div className="mt-3 text-[11px] text-zinc-600">Started {fmtUnix(s.started_at)}</div>}
          </Card>
        ))}
        {!servers && !error && <div className="text-sm text-zinc-500">Probing services...</div>}
      </div>
    </section>
  );
}
