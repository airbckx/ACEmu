'use client';

import { useState } from 'react';
import { api } from '@/lib/api';
import { getUsername } from '@/lib/auth';
import { Button } from '@/components/Button';
import { Card } from '@/components/Card';
import { PageHeader } from '@/components/Feedback';
import { Input } from '@/components/Input';
import { errorMessage, useToast } from '@/components/Toast';

export default function AccountPage() {
  const toast = useToast();
  const [current, setCurrent] = useState('');
  const [next, setNext] = useState('');
  const [repeat, setRepeat] = useState('');
  const [busy, setBusy] = useState(false);

  const valid = current !== '' && next.length >= 6 && next === repeat;

  const submit = async () => {
    if (!valid) return;
    setBusy(true);
    try {
      await api.auth.changePassword(current, next);
      toast.success('Password changed');
      setCurrent('');
      setNext('');
      setRepeat('');
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div>
      <PageHeader title="Admin account" subtitle={`Signed in as ${getUsername() ?? 'admin'}`} />
      <Card title="Change password" description="At least 6 characters. Other browsers stay signed in until their token expires." className="max-w-md">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            void submit();
          }}
          className="space-y-4"
        >
          <Input id="pw-current" label="Current password" type="password" autoComplete="current-password" value={current} onChange={(e) => setCurrent(e.target.value)} />
          <Input id="pw-new" label="New password" type="password" autoComplete="new-password" value={next} onChange={(e) => setNext(e.target.value)} />
          <Input
            id="pw-repeat"
            label="Repeat new password"
            type="password"
            autoComplete="new-password"
            value={repeat}
            onChange={(e) => setRepeat(e.target.value)}
            hint={repeat !== '' && repeat !== next ? 'The passwords differ.' : undefined}
          />
          <Button type="submit" variant="primary" disabled={!valid} loading={busy}>
            Change password
          </Button>
        </form>
      </Card>
    </div>
  );
}
