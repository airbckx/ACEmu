'use client';

import { useEffect, useState } from 'react';
import { Button } from './Button';
import { Input } from './Input';

export interface EditableNumberProps {
  label?: string;
  value: number;
  onSave: (value: number) => Promise<void>;
  min?: number;
  size?: 'sm' | 'md';
  className?: string;
  /** Table-cell mode: narrow input, Save button only rendered while dirty. */
  compact?: boolean;
}

/** Numeric field with a Save button that is enabled once the value changed. */
export function EditableNumber({ label, value, onSave, min, size = 'md', className = '', compact = false }: EditableNumberProps) {
  const [text, setText] = useState(String(value));
  const [busy, setBusy] = useState(false);

  // Follow the server value when it changes externally (after a save / reload).
  useEffect(() => {
    setText(String(value));
  }, [value]);

  const parsed = Number(text);
  const valid = text.trim() !== '' && Number.isFinite(parsed) && (min === undefined || parsed >= min);
  const dirty = valid && parsed !== value;

  const save = async () => {
    if (!dirty) return;
    setBusy(true);
    try {
      await onSave(parsed);
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className={`flex items-end gap-2 ${className}`}>
      <div className={compact ? 'w-24' : 'min-w-0 flex-1'}>
        <Input
          label={label}
          type="number"
          inputMode="numeric"
          min={min}
          inputSize={size}
          value={text}
          onChange={(e) => setText(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === 'Enter') void save();
            if (e.key === 'Escape') setText(String(value));
          }}
          className={!valid ? 'border-rose-500' : dirty ? 'border-accent-500' : ''}
        />
      </div>
      {(!compact || dirty || busy) && (
        <Button size={size} variant="primary" onClick={save} disabled={!dirty} loading={busy}>
          Save
        </Button>
      )}
    </div>
  );
}
