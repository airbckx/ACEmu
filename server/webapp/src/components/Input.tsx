'use client';

import type { InputHTMLAttributes, ReactNode, SelectHTMLAttributes } from 'react';

export const fieldClass =
  'block w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 text-sm text-zinc-100 placeholder:text-zinc-500 focus:border-accent-500 focus:outline-none focus:ring-1 focus:ring-accent-500 disabled:cursor-not-allowed disabled:opacity-60';

export interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  label?: ReactNode;
  hint?: ReactNode;
  inputSize?: 'sm' | 'md';
}

export function Input({ label, hint, inputSize = 'md', className = '', id, ...rest }: InputProps) {
  const height = inputSize === 'sm' ? 'h-8' : 'h-9';
  const control = <input id={id} className={`${fieldClass} ${height} ${className}`} {...rest} />;
  if (!label && !hint) return control;
  return (
    <label className="block" htmlFor={id}>
      {label && <span className="mb-1 block text-xs font-medium uppercase tracking-wide text-zinc-400">{label}</span>}
      {control}
      {hint && <span className="mt-1 block text-xs text-zinc-500">{hint}</span>}
    </label>
  );
}

export interface SelectProps extends SelectHTMLAttributes<HTMLSelectElement> {
  label?: ReactNode;
  selectSize?: 'sm' | 'md';
}

export function Select({ label, selectSize = 'md', className = '', id, children, ...rest }: SelectProps) {
  const height = selectSize === 'sm' ? 'h-8' : 'h-9';
  const control = (
    <select id={id} className={`${fieldClass} ${height} pr-8 ${className}`} {...rest}>
      {children}
    </select>
  );
  if (!label) return control;
  return (
    <label className="block" htmlFor={id}>
      <span className="mb-1 block text-xs font-medium uppercase tracking-wide text-zinc-400">{label}</span>
      {control}
    </label>
  );
}

export interface CheckboxProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'type'> {
  label: ReactNode;
}

export function Checkbox({ label, className = '', ...rest }: CheckboxProps) {
  return (
    <label className={`inline-flex cursor-pointer items-center gap-2 text-sm text-zinc-300 ${className}`}>
      <input
        type="checkbox"
        className="h-4 w-4 rounded border-zinc-600 bg-zinc-900 text-accent-500 focus:ring-accent-500 focus:ring-offset-zinc-950"
        {...rest}
      />
      <span>{label}</span>
    </label>
  );
}
