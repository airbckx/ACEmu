'use client';

import { useSearchParams } from 'next/navigation';
import { Suspense, useEffect, useState } from 'react';
import { api } from '@/lib/api';
import { clearSession, currentToken, setSession } from '@/lib/auth';
import { Button } from '@/components/Button';
import { Input } from '@/components/Input';
import { errorMessage } from '@/components/Toast';

export default function LoginPage() {
  return (
    <Suspense fallback={null}>
      <Login />
    </Suspense>
  );
}

/** Only same-site paths are followed after the sign-in. */
function safeNext(raw: string | null): string {
  if (!raw || !raw.startsWith('/') || raw.startsWith('//')) return '/';
  return raw;
}

function Login() {
  const params = useSearchParams();
  const next = safeNext(params.get('next'));
  const [username, setUsername] = useState('admin');
  const [password, setPassword] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  // already signed in with a token the server still accepts: go on
  useEffect(() => {
    if (!currentToken()) return;
    api.auth
      .me()
      .then(() => window.location.replace(next))
      .catch(() => clearSession());
  }, [next]);

  const submit = async () => {
    if (busy) return;
    setBusy(true);
    setError(null);
    try {
      const s = await api.auth.login(username.trim(), password);
      setSession(s.token, s.username);
      window.location.replace(next);
    } catch (err) {
      setError(errorMessage(err));
      setBusy(false);
    }
  };

  return (
    <div className="flex min-h-screen items-center justify-center bg-zinc-950 px-4 text-zinc-100">
      <form
        onSubmit={(e) => {
          e.preventDefault();
          void submit();
        }}
        className="w-full max-w-sm rounded-2xl border border-zinc-800 bg-zinc-900/70 p-6 shadow-xl"
      >
        <div className="mb-6 flex items-center gap-3">
          <span className="flex h-9 w-9 items-center justify-center rounded-md bg-accent-500 text-sm font-bold text-zinc-950">AC</span>
          <div>
            <div className="text-base font-semibold">ACEmu Admin</div>
            <div className="text-xs text-zinc-500">Sign in to continue</div>
          </div>
        </div>
        <div className="space-y-4">
          <Input id="login-user" label="Username" autoComplete="username" value={username} onChange={(e) => setUsername(e.target.value)} autoFocus />
          <Input id="login-pass" label="Password" type="password" autoComplete="current-password" value={password} onChange={(e) => setPassword(e.target.value)} />
          {error && <div className="rounded-md border border-rose-500/40 bg-rose-500/10 px-3 py-2 text-sm text-rose-200">{error}</div>}
          <Button type="submit" variant="primary" className="w-full" loading={busy} disabled={username.trim() === '' || password === ''}>
            Sign in
          </Button>
        </div>
        <p className="mt-5 text-xs text-zinc-500">
          First start without <code className="text-zinc-400">ACEMU_ADMIN_PASSWORD</code>: the generated password is in the admin log and in{' '}
          <code className="text-zinc-400">server/save/admin-password.txt</code>.
        </p>
      </form>
    </div>
  );
}
