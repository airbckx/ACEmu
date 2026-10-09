'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { useEffect, useState, type ReactNode } from 'react';
import { clearSession, currentToken, getUsername, loginHref } from '@/lib/auth';
import {
  IconAccounts,
  IconBox,
  IconChevron,
  IconClose,
  IconDashboard,
  IconHistory,
  IconItems,
  IconMail,
  IconMenu,
  IconPlayers,
  IconQuest,
  IconTicket,
  IconTrophy,
} from './Icons';

interface NavItem {
  href: string;
  label: string;
  icon: (props: { className?: string }) => ReactNode;
}

interface NavGroup {
  /** Stable key, used to remember the collapsed state. */
  key: string;
  label: string;
  items: NavItem[];
}

// The sidebar is organised by what the admin is doing rather than by table:
// looking at people, tuning the economy, or running events. The dashboard
// stands alone at the top.
const HOME: NavItem = { href: '/', label: 'Dashboard', icon: IconDashboard };

const GROUPS: NavGroup[] = [
  {
    key: 'community',
    label: 'Players',
    items: [
      { href: '/players/', label: 'Characters', icon: IconPlayers },
      { href: '/accounts/', label: 'Accounts', icon: IconAccounts },
      { href: '/matches/', label: 'Match history', icon: IconHistory },
      { href: '/leaderboard/', label: 'Leaderboard', icon: IconTrophy },
    ],
  },
  {
    key: 'economy',
    label: 'Shop & economy',
    items: [
      { href: '/items/', label: 'Item catalogue', icon: IconItems },
      { href: '/boxes/', label: 'Loot boxes', icon: IconBox },
      { href: '/coupons/', label: 'Coupons', icon: IconTicket },
    ],
  },
  {
    key: 'engagement',
    label: 'Events & messaging',
    items: [
      { href: '/quests/', label: 'Challenges & events', icon: IconQuest },
      { href: '/mail/', label: 'Mail', icon: IconMail },
    ],
  },
];

const COLLAPSED_KEY = 'acemu-admin.sidebar.collapsed';

function isActive(pathname: string, href: string): boolean {
  if (href === '/') return pathname === '/';
  return pathname === href || pathname.startsWith(href);
}

function readCollapsed(): Record<string, boolean> {
  try {
    const raw = window.localStorage.getItem(COLLAPSED_KEY);
    return raw ? (JSON.parse(raw) as Record<string, boolean>) : {};
  } catch {
    return {};
  }
}

function NavLink({ item, pathname, onNavigate }: { item: NavItem; pathname: string; onNavigate?: () => void }) {
  const active = isActive(pathname, item.href);
  const Icon = item.icon;
  return (
    <Link
      href={item.href}
      onClick={onNavigate}
      aria-current={active ? 'page' : undefined}
      className={`flex items-center gap-3 rounded-md px-3 py-1.5 text-sm font-medium transition-colors ${
        active ? 'bg-accent-500/15 text-accent-300' : 'text-zinc-400 hover:bg-zinc-800 hover:text-zinc-100'
      }`}
    >
      <Icon className="h-[18px] w-[18px] shrink-0" />
      <span className="truncate">{item.label}</span>
    </Link>
  );
}

function Sidebar({ pathname, onNavigate }: { pathname: string; onNavigate?: () => void }) {
  const [collapsed, setCollapsed] = useState<Record<string, boolean>>({});

  // Remembered per browser; a group holding the current page is always open.
  useEffect(() => {
    setCollapsed(readCollapsed());
  }, []);

  const toggle = (key: string) => {
    setCollapsed((prev) => {
      const next = { ...prev, [key]: !prev[key] };
      try {
        window.localStorage.setItem(COLLAPSED_KEY, JSON.stringify(next));
      } catch {
        // storage unavailable: the state simply is not remembered
      }
      return next;
    });
  };

  return (
    <div className="flex h-full flex-col">
      <div className="flex h-14 items-center gap-2 border-b border-zinc-800 px-5">
        <span className="flex h-7 w-7 items-center justify-center rounded-md bg-accent-500 text-xs font-bold text-zinc-950">
          AC
        </span>
        <div className="leading-tight">
          <div className="text-sm font-semibold text-zinc-50">ACEmu Admin</div>
          <div className="text-[11px] text-zinc-500">Arctic Combat emulator</div>
        </div>
      </div>
      <nav className="flex-1 overflow-y-auto px-3 py-4" aria-label="Main">
        <NavLink item={HOME} pathname={pathname} onNavigate={onNavigate} />
        {GROUPS.map((group) => {
          const containsActive = group.items.some((i) => isActive(pathname, i.href));
          const open = containsActive || !collapsed[group.key];
          const sectionId = `nav-${group.key}`;
          return (
            <div key={group.key} className="mt-5">
              <button
                type="button"
                onClick={() => toggle(group.key)}
                aria-expanded={open}
                aria-controls={sectionId}
                className="flex w-full items-center justify-between rounded-md px-3 py-1 text-[11px] font-semibold uppercase tracking-wider text-zinc-500 hover:text-zinc-300"
              >
                {group.label}
                <IconChevron className={`h-3.5 w-3.5 transition-transform ${open ? 'rotate-90' : ''}`} />
              </button>
              {open && (
                <div id={sectionId} className="mt-1 space-y-0.5">
                  {group.items.map((item) => (
                    <NavLink key={item.href} item={item} pathname={pathname} onNavigate={onNavigate} />
                  ))}
                </div>
              )}
            </div>
          );
        })}
      </nav>
      <div className="border-t border-zinc-800 px-5 py-3 text-[11px] text-zinc-600">
        <div className="flex items-center justify-between gap-2">
          <span className="truncate text-zinc-400">
            Signed in as <b className="text-zinc-300">{getUsername() ?? 'admin'}</b>
          </span>
          <button
            type="button"
            className="shrink-0 text-zinc-500 hover:text-zinc-200"
            onClick={() => {
              clearSession();
              window.location.assign('/login/');
            }}
          >
            Sign out
          </button>
        </div>
        <div className="mt-1">
          <Link href="/account/" className="text-zinc-500 hover:text-zinc-300">
            Change password
          </Link>
          <span className="mx-1 text-zinc-700">|</span>
          <Link href="/party/" className="text-zinc-500 hover:text-zinc-300" target="_blank" rel="noreferrer">
            Party screen (TV) &rarr;
          </Link>
        </div>
      </div>
    </div>
  );
}

export function Shell({ children }: { children: ReactNode }) {
  const pathname = usePathname() ?? '/';
  const [open, setOpen] = useState(false);

  // Close the drawer whenever the route changes.
  useEffect(() => {
    setOpen(false);
  }, [pathname]);

  // Every page but the sign-in needs a session; without one, go sign in.
  const [checked, setChecked] = useState(false);
  useEffect(() => {
    if (pathname.startsWith('/login')) {
      setChecked(true);
      return;
    }
    if (!currentToken()) {
      window.location.replace(loginHref());
      return;
    }
    setChecked(true);
  }, [pathname]);

  // The sign-in page and the party screen (a fullscreen board for a TV)
  // have no chrome.
  if (pathname.startsWith('/login') || pathname.startsWith('/party')) {
    return checked ? <>{children}</> : null;
  }
  if (!checked) {
    return <div className="min-h-screen bg-zinc-950" />;
  }

  return (
    <div className="min-h-screen bg-zinc-950 text-zinc-100">
      {/* Desktop sidebar */}
      <aside className="fixed inset-y-0 left-0 hidden w-60 border-r border-zinc-800 bg-zinc-900/70 lg:block">
        <Sidebar pathname={pathname} />
      </aside>

      {/* Mobile drawer */}
      {open && (
        <div className="fixed inset-0 z-40 lg:hidden">
          <div className="absolute inset-0 bg-black/60" onClick={() => setOpen(false)} />
          <aside className="absolute inset-y-0 left-0 w-64 border-r border-zinc-800 bg-zinc-900 shadow-xl">
            <button
              type="button"
              onClick={() => setOpen(false)}
              className="absolute right-2 top-3 rounded-md p-1.5 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-100"
              aria-label="Close menu"
            >
              <IconClose className="h-5 w-5" />
            </button>
            <Sidebar pathname={pathname} onNavigate={() => setOpen(false)} />
          </aside>
        </div>
      )}

      {/* Mobile top bar */}
      <header className="sticky top-0 z-30 flex h-14 items-center gap-3 border-b border-zinc-800 bg-zinc-950/90 px-4 backdrop-blur lg:hidden">
        <button
          type="button"
          onClick={() => setOpen(true)}
          className="rounded-md p-1.5 text-zinc-300 hover:bg-zinc-800"
          aria-label="Open menu"
        >
          <IconMenu className="h-5 w-5" />
        </button>
        <span className="text-sm font-semibold">ACEmu Admin</span>
      </header>

      <main className="lg:pl-60">
        <div className="mx-auto w-full max-w-7xl px-4 py-6 sm:px-6 lg:px-8">{children}</div>
        <footer className="mx-auto w-full max-w-7xl px-4 pb-8 text-xs text-zinc-600 sm:px-6 lg:px-8">
          ACEmu - an educational project, not affiliated with or endorsed by WebZen. Arctic Combat and all related names and artwork
          belong to their owners.
        </footer>
      </main>
    </div>
  );
}
