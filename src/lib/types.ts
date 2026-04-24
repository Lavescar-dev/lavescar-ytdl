export type DownloadStatus =
  | 'queued'
  | 'active'
  | 'paused'
  | 'done'
  | 'error'
  | 'cancelled';

export interface SubLang {
  code: string;
  auto: boolean;
}

export interface VideoMeta {
  url: string;
  title: string;
  uploader: string;
  duration: string;       // pre-formatted "23:14"
  bestVideo: string;      // "av01.0.08M.08 · 1080p60 · 4.2 Mbps"
  bestAudio: string;      // "opus · 160 kbps · 48 kHz"
  sizeEstimate: string;   // "≈ 116 MB"
  subtitles: string;      // "en (manual), tr (auto)"
  chapters: string;       // "7 chapters detected"
  thumbnailUrl?: string;
  availableSubs: SubLang[];
}

export interface SubtitleOpts {
  langs: string[];
  auto: boolean;
  embed: boolean;
}

export interface FormatOption {
  id: string;
  label: string;          // "av1+opus"
  spec: string;           // yt-dlp -f spec
}

export interface Preset {
  id: string;
  name: string;
  spec: string;           // yt-dlp -f format spec
  flags?: string[];       // extra flags: --split-chapters, --embed-metadata, etc.
  hotkey?: string;        // "⌘1"
  isDefault?: boolean;
}

export interface Download {
  id: string;
  url: string;
  title: string;
  status: DownloadStatus;
  codec: string;          // "av01+opus · 1080p60"
  downloadedBytes: number;
  totalBytes: number;
  speedBytesPerSec: number;
  etaSeconds: number;
  finishedAt?: number;    // epoch ms
  outputPath?: string;
  error?: string;
}

export interface DownloadRequest {
  url: string;
  formatSpec: string;
  presetId: string;
  outputDir: string;
  flags?: string[];
  subtitleOpts?: SubtitleOpts | null;
  title?: string;
  codec?: string;
}

export interface RuntimeInfo {
  ytDlpVersion: string;
  ffmpegVersion: string;
  aria2cVersion: string;
  cookiesSource: string | null;
  diskFreeGb: number;
  outputDir: string;
}

export interface PlaylistEntry {
  id: string;
  url: string;
  title: string;
  duration: string | null;
  uploader: string | null;
}

export interface PlaylistInfo {
  title: string;
  uploader: string | null;
  entries: PlaylistEntry[];
}

export type UrlInspection =
  | { kind: 'single' }
  | { kind: 'playlist'; data: PlaylistInfo };
