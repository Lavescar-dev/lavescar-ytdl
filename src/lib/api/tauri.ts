/**
 * The single boundary between frontend and backend.
 *
 * In Tauri runtime: forwards to invoke() / event listeners.
 * In browser dev:   serves mock data so the UI is fully clickable
 *                   without spinning up the Rust side.
 */
import type {
  VideoMeta,
  DownloadRequest,
  Download,
  RuntimeInfo,
  Preset,
  UrlInspection
} from '$lib/types';

export const isTauri =
  typeof window !== 'undefined' &&
  ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);

async function tauriInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<T>(cmd, args);
}

async function tauriListen<T>(evt: string, cb: (p: T) => void): Promise<() => void> {
  const { listen } = await import('@tauri-apps/api/event');
  const unlisten = await listen<T>(evt, (e) => cb(e.payload));
  return unlisten;
}

/* ---------------- commands ---------------- */

export async function extractInfo(url: string): Promise<VideoMeta> {
  if (isTauri) return tauriInvoke<VideoMeta>('extract_info', { url });
  return mockExtractInfo(url);
}

export async function inspectUrl(url: string): Promise<UrlInspection> {
  if (isTauri) return tauriInvoke<UrlInspection>('inspect_url', { url });
  return { kind: 'single' };
}

export async function startDownload(req: DownloadRequest): Promise<string> {
  if (isTauri) return tauriInvoke<string>('start_download', { req });
  return mockStartDownload(req);
}

export async function cancelDownload(id: string): Promise<void> {
  if (isTauri) return tauriInvoke<void>('cancel_download', { id });
  mockCancelDownload(id);
}

export async function getRuntimeInfo(): Promise<RuntimeInfo> {
  if (isTauri) return tauriInvoke<RuntimeInfo>('runtime_info');
  return mockRuntimeInfo();
}

export async function listHistory(limit = 50, offset = 0): Promise<Download[]> {
  if (isTauri) {
    const rows = await tauriInvoke<HistoryRow[]>('list_history', { limit, offset });
    return rows.map(historyToDownload);
  }
  return [];
}

export async function listPresets(): Promise<Preset[]> {
  if (isTauri) return tauriInvoke<Preset[]>('list_presets');
  return [];
}

export async function upsertPreset(preset: Preset): Promise<void> {
  if (isTauri) return tauriInvoke<void>('upsert_preset', { preset });
}

export async function deletePreset(id: string): Promise<void> {
  if (isTauri) return tauriInvoke<void>('delete_preset', { id });
}

export async function getSetting(key: string): Promise<string | null> {
  if (isTauri) {
    const v = await tauriInvoke<string | null>('get_setting', { key });
    return v;
  }
  return null;
}

export async function setSetting(key: string, value: string): Promise<void> {
  if (isTauri) return tauriInvoke<void>('set_setting', { key, value });
}

export async function setConcurrentLimit(limit: number): Promise<void> {
  if (isTauri) return tauriInvoke<void>('set_concurrent_limit', { limit });
}

export interface YtdlpUpdateResult {
  newVersion: string;
  path: string;
  bytes: number;
  sha256: string;
}

export interface YtdlpUpdateProgress {
  phase: 'resolving' | 'downloading' | 'installing' | 'done';
  bytes: number;
  total: number | null;
}

export async function updateYtdlp(): Promise<YtdlpUpdateResult> {
  if (!isTauri) throw new Error('update_ytdlp is only available in the desktop app');
  return tauriInvoke<YtdlpUpdateResult>('update_ytdlp');
}

export async function onYtdlpUpdateProgress(
  cb: (p: YtdlpUpdateProgress) => void
): Promise<() => void> {
  if (isTauri) return tauriListen<YtdlpUpdateProgress>('ytdlp:update:progress', cb);
  return () => {};
}

export interface OrphanPart {
  path: string;
  sizeBytes: number;
  modifiedMs: number;
}

export interface OrphanScanResult {
  dir: string;
  items: OrphanPart[];
}

export async function scanOrphanParts(): Promise<OrphanScanResult> {
  if (isTauri) return tauriInvoke<OrphanScanResult>('scan_orphan_parts');
  return { dir: '', items: [] };
}

export async function deleteOrphanParts(paths: string[]): Promise<number> {
  if (isTauri) return tauriInvoke<number>('delete_orphan_parts', { paths });
  return 0;
}

export async function onOrphansFound(
  cb: (p: OrphanScanResult) => void
): Promise<() => void> {
  if (isTauri) return tauriListen<OrphanScanResult>('orphans:found', cb);
  return () => {};
}

export interface ClipboardUrlPayload {
  url: string;
  source: string;
}

export async function onClipboardUrl(
  cb: (p: ClipboardUrlPayload) => void
): Promise<() => void> {
  if (isTauri) return tauriListen<ClipboardUrlPayload>('clipboard:url', cb);
  return () => {};
}

export async function pickDirectory(): Promise<string | null> {
  if (isTauri) {
    const v = await tauriInvoke<string | null>('pick_directory');
    return v;
  }
  return null;
}

export async function detectTooling(): Promise<{
  ffmpeg: string;
  aria2c: string;
  diskFreeGb: number;
}> {
  if (isTauri) return tauriInvoke('detect_tooling');
  return { ffmpeg: '6.1.1', aria2c: '1.37.0', diskFreeGb: 412.8 };
}

export async function openInMpv(path: string): Promise<void> {
  if (isTauri) return tauriInvoke('open_in_mpv', { path });
}

export async function revealInFileManager(path: string): Promise<void> {
  if (isTauri) return tauriInvoke('reveal_in_file_manager', { path });
}

/* ---------------- events ---------------- */

export async function onProgress(
  cb: (update: Partial<Download> & { id: string }) => void
): Promise<() => void> {
  if (isTauri) return tauriListen<Partial<Download> & { id: string }>('download:progress', cb);
  return mockOnProgress(cb);
}

export async function onDone(
  cb: (payload: { id: string; path: string }) => void
): Promise<() => void> {
  if (isTauri) return tauriListen('download:done', cb);
  return () => {};
}

export type DownloadErrorKind =
  | 'geo_blocked'
  | 'auth_required'
  | 'not_found'
  | 'network'
  | 'io'
  | 'parse'
  | 'shell'
  | 'unknown';

export interface DownloadErrorPayload {
  id: string;
  kind: DownloadErrorKind;
  message: string;
}

export async function onError(
  cb: (payload: DownloadErrorPayload) => void
): Promise<() => void> {
  if (isTauri) return tauriListen<DownloadErrorPayload>('download:error', cb);
  return () => {};
}

/* ---------------- history mapping ---------------- */

interface HistoryRow {
  id: string;
  url: string;
  title: string;
  status: string;
  codec: string;
  outputPath: string | null;
  startedAt: number;
  finishedAt: number | null;
  error: string | null;
}

function historyToDownload(r: HistoryRow): Download {
  return {
    id: r.id,
    url: r.url,
    title: r.title,
    status: (r.status as Download['status']) ?? 'done',
    codec: r.codec,
    downloadedBytes: 0,
    totalBytes: 0,
    speedBytesPerSec: 0,
    etaSeconds: 0,
    finishedAt: r.finishedAt ?? undefined,
    outputPath: r.outputPath ?? undefined,
    error: r.error ?? undefined
  };
}

/* ---------------- mock implementations ---------------- */

const SAMPLES: VideoMeta[] = [
  {
    url: '',
    title: "Wube · How Factorio's Belt Compression Actually Works",
    uploader: 'Factorio Devblog',
    duration: '23:14',
    bestVideo: 'av01.0.08M.08 · 1080p60 · 4.2 Mbps',
    bestAudio: 'opus · 160 kbps · 48 kHz',
    sizeEstimate: '≈ 116 MB',
    subtitles: 'en (manual), tr (auto), de (auto)',
    chapters: '7 chapters detected',
    availableSubs: [
      { code: 'en', auto: false },
      { code: 'tr', auto: true },
      { code: 'de', auto: true }
    ]
  },
  {
    url: '',
    title: 'Andrew Kelley · The Road to Zig 1.0',
    uploader: 'Software You Can Love',
    duration: '47:02',
    bestVideo: 'vp9 · 1440p · 6.8 Mbps',
    bestAudio: 'opus · 160 kbps · 48 kHz',
    sizeEstimate: '≈ 248 MB',
    subtitles: 'en (manual)',
    chapters: '5 chapters detected',
    availableSubs: [{ code: 'en', auto: false }]
  }
];

async function mockExtractInfo(url: string): Promise<VideoMeta> {
  await sleep(550 + Math.random() * 300);
  const sample = SAMPLES[Math.floor(Math.random() * SAMPLES.length)];
  return { ...sample, url };
}

const progressListeners = new Set<
  (u: Partial<Download> & { id: string }) => void
>();
const fakeTimers = new Map<string, ReturnType<typeof setInterval>>();

async function mockStartDownload(req: DownloadRequest): Promise<string> {
  const id = crypto.randomUUID();
  const total = 80_000_000 + Math.floor(Math.random() * 200_000_000);
  let downloaded = 0;
  let lastTick = performance.now();

  const timer = setInterval(() => {
    const now = performance.now();
    const dt = (now - lastTick) / 1000;
    lastTick = now;
    const speed = 2_000_000 + Math.random() * 5_000_000;
    downloaded = Math.min(total, downloaded + speed * dt);
    const pct = downloaded / total;
    const eta = Math.max(0, (total - downloaded) / speed);

    progressListeners.forEach((cb) =>
      cb({
        id,
        downloadedBytes: downloaded,
        totalBytes: total,
        speedBytesPerSec: speed,
        etaSeconds: eta,
        status: pct >= 1 ? 'done' : 'active'
      })
    );

    if (pct >= 1) {
      clearInterval(timer);
      fakeTimers.delete(id);
    }
  }, 400);

  fakeTimers.set(id, timer);
  void req;
  return id;
}

function mockCancelDownload(id: string) {
  const t = fakeTimers.get(id);
  if (t) {
    clearInterval(t);
    fakeTimers.delete(id);
    progressListeners.forEach((cb) => cb({ id, status: 'paused' }));
  }
}

async function mockOnProgress(
  cb: (u: Partial<Download> & { id: string }) => void
): Promise<() => void> {
  progressListeners.add(cb);
  return () => progressListeners.delete(cb);
}

async function mockRuntimeInfo(): Promise<RuntimeInfo> {
  return {
    ytDlpVersion: '2026.04.18',
    ffmpegVersion: '6.1.1',
    aria2cVersion: '1.37.0',
    cookiesSource: 'firefox',
    diskFreeGb: 412.8,
    outputDir: '~/dl/yt'
  };
}

function sleep(ms: number) {
  return new Promise((r) => setTimeout(r, ms));
}
