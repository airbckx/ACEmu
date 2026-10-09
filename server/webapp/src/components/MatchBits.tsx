import { Badge } from './Badge';
import { fmtTeam } from '@/lib/format';

export function matchHref(id: number): string {
  return `/matches/view/?id=${id}`;
}

/** winner 0/1 = team, -1 = draw. */
export function WinnerBadge({ winner }: { winner: number }) {
  if (winner < 0) return <Badge tone="neutral">Draw</Badge>;
  return <Badge tone={winner === 0 ? 'accent' : 'warning'}>{fmtTeam(winner)} won</Badge>;
}
