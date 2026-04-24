import type { VideoMeta, FormatOption, PlaylistInfo, SubtitleOpts } from '$lib/types';
import { extractInfo, inspectUrl } from '$lib/api/tauri';
import { errors } from './errors.svelte';

const FORMAT_OPTIONS: FormatOption[] = [
  { id: 'av1-opus', label: 'av1+opus',           spec: "bv[vcodec~='av01']+ba[acodec='opus']" },
  { id: 'vp9-opus', label: 'vp9+opus',           spec: "bv[vcodec~='vp09']+ba[acodec='opus']" },
  { id: 'h264-aac', label: 'h264+aac',           spec: "bv[vcodec~='avc1']+ba[acodec~='mp4a']" },
  { id: 'aud-m4a',  label: 'audio only · m4a',   spec: 'ba[ext=m4a]' },
  { id: 'aud-mp3',  label: 'audio only · mp3 v0', spec: 'ba/b' /* + --extract-audio */ }
];

class MetadataStore {
  current     = $state<VideoMeta | null>(null);
  playlist    = $state<PlaylistInfo | null>(null);
  isLoading   = $state(false);
  error       = $state<string | null>(null);
  formatOptions = FORMAT_OPTIONS;
  selectedFormatId = $state<string>('av1-opus');
  subtitleOpts = $state<SubtitleOpts | null>(null);
  subtitleModalOpen = $state(false);

  selectedFormat = $derived(
    this.formatOptions.find((f) => f.id === this.selectedFormatId)
      ?? this.formatOptions[0]
  );

  state = $derived<'empty' | 'loading' | 'ready' | 'playlist' | 'error'>(
    this.error      ? 'error'    :
    this.isLoading  ? 'loading'  :
    this.playlist   ? 'playlist' :
    this.current    ? 'ready'    : 'empty'
  );

  async fetch(url: string) {
    if (!url.trim()) return;
    this.isLoading = true;
    this.error = null;
    this.playlist = null;
    this.current = null;
    try {
      const ins = await inspectUrl(url);
      if (ins.kind === 'playlist') {
        this.playlist = ins.data;
      } else {
        this.current = await extractInfo(url);
      }
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      this.error = msg;
      this.current = null;
      this.playlist = null;
      errors.push('fetch_failed', msg);
    } finally {
      this.isLoading = false;
    }
  }

  selectFormat(id: string) {
    this.selectedFormatId = id;
  }

  clear() {
    this.current = null;
    this.playlist = null;
    this.error = null;
    this.subtitleOpts = null;
    this.subtitleModalOpen = false;
  }

  openSubtitleModal() {
    this.subtitleModalOpen = true;
  }
  closeSubtitleModal() {
    this.subtitleModalOpen = false;
  }
  setSubtitleOpts(opts: SubtitleOpts | null) {
    this.subtitleOpts = opts;
  }
}

export const metadata = new MetadataStore();
