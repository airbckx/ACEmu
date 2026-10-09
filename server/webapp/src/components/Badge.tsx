import type { ReactNode } from 'react';
import type { Availability } from '@/lib/api';
import { AVAILABILITY_LABEL } from '@/lib/format';

export type BadgeTone = 'neutral' | 'accent' | 'success' | 'warning' | 'danger';

const TONE: Record<BadgeTone, string> = {
  neutral: 'bg-zinc-800 text-zinc-300 ring-zinc-700',
  accent: 'bg-accent-500/15 text-accent-300 ring-accent-500/30',
  success: 'bg-emerald-500/15 text-emerald-300 ring-emerald-500/30',
  warning: 'bg-amber-500/15 text-amber-300 ring-amber-500/30',
  danger: 'bg-rose-500/15 text-rose-300 ring-rose-500/30',
};

export function Badge({ tone = 'neutral', children, className = '' }: { tone?: BadgeTone; children: ReactNode; className?: string }) {
  return (
    <span
      className={`inline-flex items-center whitespace-nowrap rounded-full px-2 py-0.5 text-xs font-medium ring-1 ring-inset ${TONE[tone]} ${className}`}
    >
      {children}
    </span>
  );
}

const AVAILABILITY_TONE: Record<Availability, BadgeTone> = {
  shop: 'success',
  lootbox: 'warning',
  hidden: 'neutral',
};

export function AvailabilityBadge({ value }: { value: Availability }) {
  const tone = AVAILABILITY_TONE[value] ?? 'neutral';
  const label = AVAILABILITY_LABEL[value] ?? value;
  return <Badge tone={tone}>{label}</Badge>;
}
