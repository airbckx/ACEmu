'use client';

import { useEffect, useState } from 'react';
import { api, type CatalogItem } from '@/lib/api';
import { useDebounced } from '@/lib/hooks';
import { Spinner } from './Feedback';
import { Input } from './Input';
import { errorMessage } from './Toast';
import { ItemIcon } from './ItemIcon';

export interface CatalogSearchProps {
  /** Called when the user clicks a result. */
  onPick: (item: CatalogItem) => void;
  /** Id of the currently selected item, for highlighting. */
  selectedId?: number | null;
  placeholder?: string;
  /** Ids to grey out (already in the list). */
  disabledIds?: Set<number>;
  limit?: number;
  inputId?: string;
}

/** Search box that queries GET /api/items?q= and lists clickable results. */
export function CatalogSearch({ onPick, selectedId, placeholder = 'Search catalog by name or id', disabledIds, limit = 20, inputId }: CatalogSearchProps) {
  const [query, setQuery] = useState('');
  const debounced = useDebounced(query, 300);
  const [results, setResults] = useState<CatalogItem[]>([]);
  const [total, setTotal] = useState(0);
  const [searching, setSearching] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const q = debounced.trim();
    if (q === '') {
      setResults([]);
      setTotal(0);
      setError(null);
      return;
    }
    let cancelled = false;
    setSearching(true);
    api.items
      .list({ q, limit })
      .then((page) => {
        if (cancelled) return;
        setResults(page.items);
        setTotal(page.total);
        setError(null);
      })
      .catch((err) => {
        if (!cancelled) setError(errorMessage(err));
      })
      .finally(() => {
        if (!cancelled) setSearching(false);
      });
    return () => {
      cancelled = true;
    };
  }, [debounced, limit]);

  const active = debounced.trim() !== '';

  return (
    <div className="space-y-2">
      <Input id={inputId} type="search" placeholder={placeholder} value={query} onChange={(e) => setQuery(e.target.value)} aria-label={placeholder} autoComplete="off" />

      {error && <div className="text-xs text-rose-300">{error}</div>}

      {(results.length > 0 || searching) && (
        <div className="max-h-56 overflow-y-auto rounded-md border border-zinc-800">
          {searching && results.length === 0 && (
            <div className="flex items-center gap-2 px-3 py-2 text-xs text-zinc-500">
              <Spinner className="h-3.5 w-3.5" /> Searching...
            </div>
          )}
          {results.map((it) => {
            const disabled = disabledIds?.has(it.id) ?? false;
            const selected = selectedId === it.id;
            return (
              <button
                key={it.id}
                type="button"
                disabled={disabled}
                onClick={() => onPick(it)}
                className={`flex w-full items-center justify-between gap-3 px-3 py-2 text-left text-sm transition-colors ${
                  disabled
                    ? 'cursor-not-allowed text-zinc-600'
                    : selected
                      ? 'bg-accent-500/15 text-accent-200'
                      : 'text-zinc-200 hover:bg-zinc-800'
                }`}
              >
                <span className="flex min-w-0 items-center gap-2">
                  <ItemIcon id={it.id} size="xs" />
                  <span className="truncate">
                    {it.name}
                    {it.category && <span className="ml-2 text-xs text-zinc-500">{it.category}</span>}
                  </span>
                </span>
                <span className="shrink-0 text-xs text-zinc-500">
                  {disabled ? 'added - ' : ''}#{it.id}
                </span>
              </button>
            );
          })}
          {total > results.length && <div className="px-3 py-1.5 text-xs text-zinc-500">{total - results.length} more - refine the search.</div>}
        </div>
      )}
      {active && !searching && results.length === 0 && !error && <div className="text-xs text-zinc-500">No catalog items match.</div>}
    </div>
  );
}
