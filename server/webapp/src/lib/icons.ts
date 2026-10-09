import { useEffect, useState } from 'react';
import { apiBase } from './api';

/**
 * Item id -> shop icon base name (`data/icons.json`, served as GET /api/icons).
 * The PNGs live in the web app's public folder: `/icons/<name>.png` (256x64)
 * and, optionally, `/icons-hd/<name>.png` - same origin as the page.
 */
export type IconMap = Map<number, string>;

let cache: IconMap | null = null;
let pending: Promise<IconMap> | null = null;
const listeners = new Set<(m: IconMap) => void>();

export function loadIcons(): Promise<IconMap> {
  if (cache) return Promise.resolve(cache);
  if (!pending) {
    pending = fetch(`${apiBase()}/api/icons`)
      .then((r) => (r.ok ? r.json() : Promise.reject(new Error(`icons: HTTP ${r.status}`))))
      .then((raw: Record<string, string>) => {
        const m: IconMap = new Map();
        for (const [id, icon] of Object.entries(raw)) {
          const n = Number(id);
          if (Number.isFinite(n) && icon) m.set(n, icon);
        }
        cache = m;
        for (const l of listeners) l(m);
        return m;
      })
      .catch(() => {
        // no icons: every lookup falls back to the placeholder
        const m: IconMap = new Map();
        cache = m;
        for (const l of listeners) l(m);
        return m;
      });
  }
  return pending;
}

/** The icon map once loaded (empty until then, then re-rendered). */
export function useItemIcons(): IconMap {
  const [map, setMap] = useState<IconMap>(() => cache ?? new Map());
  useEffect(() => {
    if (cache) {
      setMap(cache);
      return;
    }
    const l = (m: IconMap) => setMap(m);
    listeners.add(l);
    void loadIcons();
    return () => {
      listeners.delete(l);
    };
  }, []);
  return map;
}

export function iconUrl(icon: string, hd = false): string {
  return `${hd ? '/icons-hd/' : '/icons/'}${encodeURIComponent(icon)}.png`;
}
