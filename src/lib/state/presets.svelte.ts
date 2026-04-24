import type { Preset, PresetCategory } from '$lib/types';
import { isTauri, listPresets, upsertPreset, deletePreset } from '$lib/api/tauri';

const FALLBACK: Preset[] = [
  // Video
  { id: 'video-archive-av1', name: 'archive · av1',   spec: "bv[vcodec~='av01']+ba[acodec='opus']/b", hotkey: '⌘1', isDefault: true, category: 'video' },
  { id: 'video-archive-vp9', name: 'archive · vp9',   spec: "bv[vcodec~='vp09']+ba[acodec='opus']/b", hotkey: '⌘2', category: 'video' },
  { id: 'video-mobile-720',  name: 'mobile · 720p',   spec: "bv[vcodec~='avc1'][height<=720]+ba[acodec~='mp4a']/b[ext=mp4][height<=720]", hotkey: '⌘3', category: 'video' },
  { id: 'video-small-480',   name: 'small · 480p',    spec: 'bv[height<=480]+ba/b[height<=480]', category: 'video' },
  { id: 'video-4k-av1',      name: '4K · av1',        spec: "bv[vcodec~='av01'][height<=2160]+ba[acodec='opus']/bv+ba/b", category: 'video' },
  // Audio
  { id: 'audio-opus',        name: 'opus · native',   spec: "ba[acodec='opus']/ba", hotkey: '⌘1', isDefault: true, category: 'audio' },
  { id: 'audio-m4a',         name: 'm4a · native',    spec: "ba[ext=m4a]/ba[acodec~='mp4a']", hotkey: '⌘2', category: 'audio' },
  { id: 'audio-mp3-v0',      name: 'mp3 · v0 (320k)', spec: 'ba/b', flags: ['--extract-audio', '--audio-format', 'mp3', '--audio-quality', '0'], hotkey: '⌘3', category: 'audio' },
  { id: 'audio-flac',        name: 'flac · lossless', spec: 'ba/b', flags: ['--extract-audio', '--audio-format', 'flac'], category: 'audio' },
  { id: 'audio-podcast-chapters', name: 'podcast · chapters', spec: 'ba/b', flags: ['--extract-audio', '--split-chapters', '--embed-metadata'], category: 'audio' }
];

class PresetsStore {
  items = $state<Preset[]>(isTauri ? [] : FALLBACK);
  activeCategory = $state<PresetCategory>('video');
  selectedId = $state<string>('video-archive-av1');

  /// Presets filtered to the active tab.
  visible = $derived(
    this.items.filter((p) => p.category === this.activeCategory)
  );

  selected = $derived(
    this.items.find((p) => p.id === this.selectedId) ??
    this.visible.find((p) => p.isDefault) ??
    this.visible[0]
  );

  setCategory(cat: PresetCategory) {
    if (this.activeCategory === cat) return;
    this.activeCategory = cat;
    // Jump to the default preset of the newly focused category.
    const def =
      this.items.find((p) => p.category === cat && p.isDefault) ??
      this.items.find((p) => p.category === cat);
    if (def) this.selectedId = def.id;
  }

  select(id: string) {
    const p = this.items.find((x) => x.id === id);
    if (!p) return;
    this.selectedId = id;
    // Follow the preset into its category so the UI stays coherent.
    if (this.activeCategory !== p.category) this.activeCategory = p.category;
  }

  async load() {
    const rows = await listPresets();
    if (rows.length > 0) {
      this.items = rows;
      const def =
        rows.find((p) => p.category === this.activeCategory && p.isDefault) ??
        rows.find((p) => p.isDefault) ??
        rows[0];
      if (def) {
        this.selectedId = def.id;
        this.activeCategory = def.category;
      }
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
