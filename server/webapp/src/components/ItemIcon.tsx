'use client';

import { useState } from 'react';
import { iconUrl, useItemIcons } from '@/lib/icons';

export type ItemIconSize = 'xs' | 'sm' | 'md' | 'hero';

/** The client's shop icons are 4:1 (256x64); these keep that ratio. */
const SIZE: Record<ItemIconSize, string> = {
  xs: 'h-4 w-16',
  sm: 'h-6 w-24',
  md: 'h-10 w-40',
  hero: 'h-40 w-full max-w-5xl sm:h-56 lg:h-64',
};

export interface ItemIconProps {
  /** Catalogue item id (0 / undefined shows the placeholder). */
  id: number | null | undefined;
  size?: ItemIconSize;
  /** Alt text; defaults to nothing (the name is usually next to it). */
  alt?: string;
  className?: string;
  /** Render nothing instead of the placeholder when there is no icon. */
  hideMissing?: boolean;
}

/**
 * An item's shop icon, looked up by id in the icon map. Items without an
 * entry (or whose file is missing) show a neutral placeholder of the same
 * footprint, so table rows keep their alignment.
 */
export function ItemIcon({ id, size = 'sm', alt = '', className = '', hideMissing = false }: ItemIconProps) {
  const icons = useItemIcons();
  const [broken, setBroken] = useState<string | null>(null);
  const icon = id ? icons.get(id) : undefined;
  const box = `${SIZE[size]} shrink-0 rounded ${className}`;
  if (!icon || broken === icon) {
    if (hideMissing) return null;
    return (
      <span className={`${box} inline-flex items-center justify-center border border-dashed border-zinc-700/70 bg-zinc-900/60 text-zinc-700`} aria-hidden="true" title={id ? `item ${id}: no picture` : undefined}>
        <svg viewBox="0 0 24 24" className={size === 'hero' ? 'h-8 w-8' : 'h-3 w-3'} fill="none" stroke="currentColor" strokeWidth="1.5">
          <rect x="3" y="5" width="18" height="14" rx="2" />
          <path d="M3 15l5-5 4 4 3-3 6 6" />
        </svg>
      </span>
    );
  }
  return (
    // eslint-disable-next-line @next/next/no-img-element
    <img
      src={iconUrl(icon, size === 'hero')}
      alt={alt}
      loading="lazy"
      decoding="async"
      onError={() => setBroken(icon)}
      className={`${box} object-contain ${size === 'hero' ? 'mx-auto bg-zinc-900/60 p-4' : ''}`}
    />
  );
}
