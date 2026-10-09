'use client';

import type { ReactNode } from 'react';

export interface Column<T> {
  key: string;
  header: ReactNode;
  render: (row: T, index: number) => ReactNode;
  /** Extra classes for both header and cells (e.g. text-right, w-24). */
  className?: string;
  /** Hide on small screens. */
  hideBelow?: 'sm' | 'md' | 'lg';
}

export interface DataTableProps<T> {
  columns: Column<T>[];
  rows: T[];
  rowKey: (row: T, index: number) => string | number;
  emptyText?: ReactNode;
  onRowClick?: (row: T) => void;
  dense?: boolean;
}

const HIDE: Record<NonNullable<Column<unknown>['hideBelow']>, string> = {
  sm: 'hidden sm:table-cell',
  md: 'hidden md:table-cell',
  lg: 'hidden lg:table-cell',
};

export function DataTable<T>({ columns, rows, rowKey, emptyText = 'Nothing here.', onRowClick, dense }: DataTableProps<T>) {
  const pad = dense ? 'px-3 py-1.5' : 'px-3 py-2.5 sm:px-4';
  return (
    <div className="w-full overflow-x-auto">
      <table className="w-full min-w-max text-left text-sm">
        <thead className="bg-zinc-900 text-xs uppercase tracking-wide text-zinc-500">
          <tr>
            {columns.map((c) => (
              <th
                key={c.key}
                scope="col"
                className={`${pad} font-medium ${c.hideBelow ? HIDE[c.hideBelow] : ''} ${c.className ?? ''}`}
              >
                {c.header}
              </th>
            ))}
          </tr>
        </thead>
        <tbody className="divide-y divide-zinc-800/80">
          {rows.length === 0 && (
            <tr>
              <td colSpan={columns.length} className="px-4 py-8 text-center text-sm text-zinc-500">
                {emptyText}
              </td>
            </tr>
          )}
          {rows.map((row, i) => (
            <tr
              key={rowKey(row, i)}
              onClick={onRowClick ? () => onRowClick(row) : undefined}
              className={`transition-colors hover:bg-zinc-800/50 ${onRowClick ? 'cursor-pointer' : ''}`}
            >
              {columns.map((c) => (
                <td
                  key={c.key}
                  className={`${pad} align-middle text-zinc-200 ${c.hideBelow ? HIDE[c.hideBelow] : ''} ${c.className ?? ''}`}
                >
                  {c.render(row, i)}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
