import { ui } from './ui.svelte';
import { presets } from './presets.svelte';
import { i18n } from '$lib/i18n/index.svelte';
import type { Dict } from '$lib/i18n/dict';

/**
 * Shortcut entry. `labelKey` is a function over the active dictionary so the
 * cheatsheet always renders in the user's chosen language without us having
 * to re-register every shortcut on switch.
 */
export interface Action {
  combo: string;
  labelKey: (t: Dict) => string;
  run: () => void;
}

const actions: Action[] = [];

export function registerShortcut(combo: string, labelKey: (t: Dict) => string, run: () => void) {
  actions.push({ combo, labelKey, run });
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

  registerShortcut('mod+l', (t) => t.shortcuts.focusUrl, () => {
    const el = document.querySelector<HTMLInputElement>('input[placeholder^="https"]');
    el?.focus();
    el?.select();
  });

  registerShortcut('mod+1', (t) => t.shortcuts.presetIndex(1), () => selectCategoryPreset(0));
  registerShortcut('mod+2', (t) => t.shortcuts.presetIndex(2), () => selectCategoryPreset(1));
  registerShortcut('mod+3', (t) => t.shortcuts.presetIndex(3), () => selectCategoryPreset(2));
  registerShortcut('mod+shift+v', (t) => t.shortcuts.switchVideo, () => presets.setCategory('video'));
  registerShortcut('mod+shift+a', (t) => t.shortcuts.switchAudio, () => presets.setCategory('audio'));
  registerShortcut('mod+,', (t) => t.shortcuts.openSettings, () => ui.openSettings());
  registerShortcut('?', (t) => t.shortcuts.showCheatsheet, () => toggleCheatsheet(true));
  registerShortcut('escape', (t) => t.shortcuts.closeOverlay, () => toggleCheatsheet(false));

  window.addEventListener('keydown', (e) => {
    const tag = (e.target as HTMLElement | null)?.tagName;
    const isEditable =
      tag === 'INPUT' || tag === 'TEXTAREA' || (e.target as HTMLElement | null)?.isContentEditable;
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

// Re-export to keep `i18n` import live in the module graph.
export const _i18n = i18n;
