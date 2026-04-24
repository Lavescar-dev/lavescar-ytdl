import type { Preset } from '$lib/types';
import { isTauri, listPresets, upsertPreset, deletePreset } from '$lib/api/tauri';

const FALLBACK: Preset[] = [
  {
    id: 'archive-av1',
    name: 'archive · av1',
    spec: "bv[vcodec~='av01']+ba[acodec='opus']/b",
    isDefault: true
  },
  {
    id: 'max-audio',
    name: 'max audio',
    spec: 'ba/b',
    flags: ['--extract-audio', '--sponsorblock-remove', 'sponsor'],
    hotkey: '⌘1'
  },
  {
    id: 'mobile-720',
    name: 'mobile · 720p',
    spec: 'b[ext=mp4][height<=720]',
    hotkey: '⌘2'
  },
  {
    id: 'podcast-split',
    name: 'podcast · split',
    spec: 'ba/b',
    flags: ['--split-chapters', '--embed-metadata'],
    hotkey: '⌘3'
  }
];

class PresetsStore {
  items = $state<Preset[]>(isTauri ? [] : FALLBACK);
  selectedId = $state<string>('archive-av1');

  selected = $derived(
    this.items.find((p) => p.id === this.selectedId) ?? this.items[0]
  );

  select(id: string) {
    this.selectedId = id;
  }

  async load() {
    const rows = await listPresets();
    if (rows.length > 0) {
      this.items = rows;
      const def = rows.find((p) => p.isDefault);
      if (def) this.selectedId = def.id;
    }
  }

  async upsert(p: Preset) {
    await upsertPreset(p);
    await this.load();
  }

  async remove(id: string) {
    await deletePreset(id);
    await this.load();
  }
}

export const presets = new PresetsStore();

if (typeof window !== 'undefined' && isTauri) {
  presets.load().catch(() => {});
}
