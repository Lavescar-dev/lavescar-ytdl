import { ui } from './ui.svelte';
import { presets } from './presets.svelte';

type Action = { combo: string; label: string; run: () => void };

const actions: Action[] = [];

export function registerShortcut(combo: string, label: string, run: () => void) {
  actions.push({ combo, label, run });
}

export function listShortcuts(): readonly Action[] {
  return actions;
}

function normalize(e: KeyboardEvent): string {
  const parts: string[] = [];
  if (e.metaKey || e.ctrlKey) parts.push('mod');
  if (e.shiftKey) parts.push('shift');
  if (e.altKey) parts.push('alt');
  parts.push(e.key.toLowerCase());
  return parts.join('+');
}

let started = false;

export function startShortcuts() {
  if (started || typeof window === 'undefined') return;
  started = true;

  registerShortcut('mod+l', 'focus URL input', () => {
    const el = document.querySelector<HTMLInputElement>('input[placeholder^="https"]');
    el?.focus();
    el?.select();
  });

  registerShortcut('mod+1', 'active category preset #1', () => selectCategoryPreset(0));
  registerShortcut('mod+2', 'active category preset #2', () => selectCategoryPreset(1));
  registerShortcut('mod+3', 'active category preset #3', () => selectCategoryPreset(2));
  registerShortcut('mod+shift+v', 'switch to video preset tab', () => presets.setCategory('video'));
  registerShortcut('mod+shift+a', 'switch to audio preset tab', () => presets.setCategory('audio'));

  registerShortcut('mod+,', 'open settings', () => ui.openSettings());
  registerShortcut('?', 'show shortcuts cheatsheet', () => toggleCheatsheet(true));
  registerShortcut('escape', 'close overlay', () => toggleCheatsheet(false));

  window.addEventListener('keydown', (e) => {
    const tag = (e.target as HTMLElement | null)?.tagName;
    const isEditable =
      tag === 'INPUT' || tag === 'TEXTAREA' || (e.target as HTMLElement | null)?.isContentEditable;
    // Allow mod+L (focus URL) even from editable — otherwise skip editable targets.
    const combo = normalize(e);
    if (isEditable && combo !== 'mod+l' && combo !== 'escape') return;

    for (const a of actions) {
      if (a.combo === combo) {
        e.preventDefault();
        a.run();
        return;
      }
    }
  });
}

function selectCategoryPreset(index: number) {
  const list = presets.items.filter((p) => p.category === presets.activeCategory);
  // Prefer rows carrying an explicit `⌘N` hotkey (user-chosen order), else fall
  // back to list position — works both for legacy DBs and freshly seeded ones.
  const wanted = `⌘${index + 1}`;
  const byHotkey = list.find((p) => p.hotkey === wanted);
  const target = byHotkey ?? list[index];
  if (target) presets.select(target.id);
}

function toggleCheatsheet(on: boolean) {
  cheatsheetVisible.value = on;
}

class CheatsheetStore {
  value = $state(false);
}
export const cheatsheetVisible = new CheatsheetStore();
