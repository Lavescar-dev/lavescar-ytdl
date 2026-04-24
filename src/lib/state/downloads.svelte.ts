import type { Download, DownloadRequest } from '$lib/types';
import {
  startDownload,
  cancelDownload,
  onProgress,
  onDone,
  onError,
  listHistory,
  isTauri
} from '$lib/api/tauri';
import { errors } from './errors.svelte';

function demoSeed(): Download[] {
  if (isTauri) return [];
  return [
    {
      id: 'demo-1',
      url: 'https://example/1',
      title: "Crafting Interpreters · A Handbook for Making Programming Languages",
      status: 'active',
      codec: 'av01+opus · 1080p60',
      downloadedBytes: 84_200_000,
      totalBytes: 116_700_000,
      speedBytesPerSec: 4_200_000,
      etaSeconds: 8
    },
    {
      id: 'demo-2',
      url: 'https://example/2',
      title: 'Andrew Kelley · The Road to Zig 1.0',
      status: 'active',
      codec: 'vp9+opus · 1440p',
      downloadedBytes: 12_100_000,
      totalBytes: 68_400_000,
      speedBytesPerSec: 1_800_000,
      etaSeconds: 31
    },
    {
      id: 'demo-3',
      url: 'https://example/3',
      title: 'FactorioCon 2024 · Belt System Internals (Wube talk)',
      status: 'queued',
      codec: 'av01+opus · 2160p',
      downloadedBytes: 0,
      totalBytes: 0,
      speedBytesPerSec: 0,
      etaSeconds: 0
    },
    {
      id: 'demo-0',
      url: 'https://example/0',
      title: 'Hyprland config walkthrough · 2026 update',
      status: 'done',
      codec: 'av01+opus · 1080p',
      downloadedBytes: 89_000_000,
      totalBytes: 89_000_000,
      speedBytesPerSec: 0,
      etaSeconds: 0,
      finishedAt: Date.now() - 42_000,
      outputPath: '~/dl/yt/hyprland-2026.mkv'
    }
  ];
}

class DownloadsStore {
  items = $state<Download[]>(demoSeed());

  active = $derived(this.items.filter((d) => d.status === 'active').length);
  queued = $derived(this.items.filter((d) => d.status === 'queued').length);
  done   = $derived(this.items.filter((d) => d.status === 'done').length);

  totalSpeed = $derived(
    this.items
      .filter((d) => d.status === 'active')
      .reduce((sum, d) => sum + d.speedBytesPerSec, 0)
  );

  async enqueue(req: DownloadRequest, title: string, codec: string) {
    const id = await startDownload(req);
    this.items = [
      ...this.items,
      {
        id,
        url: req.url,
        title,
        status: 'active',
        codec,
        downloadedBytes: 0,
        totalBytes: 0,
        speedBytesPerSec: 0,
        etaSeconds: 0
      }
    ];
    return id;
  }

  async cancel(id: string) {
    await cancelDownload(id);
    // Don't drop the row — move it to History as "cancelled" so the user sees
    // what they stopped and can re-queue later from the same URL.
    this.items = this.items.map((d) =>
      d.id === id
        ? {
            ...d,
            status: 'cancelled' as const,
            finishedAt: Date.now(),
            speedBytesPerSec: 0,
            etaSeconds: 0
          }
        : d
    );
  }

  remove(id: string) {
    this.items = this.items.filter((d) => d.id !== id);
  }

  applyUpdate(u: Partial<Download> & { id: string }) {
    const idx = this.items.findIndex((d) => d.id === u.id);
    if (idx === -1) return;
    this.items[idx] = { ...this.items[idx], ...u };
  }

  markDone(id: string, path: string) {
    const idx = this.items.findIndex((d) => d.id === id);
    if (idx === -1) return;
    this.items[idx] = {
      ...this.items[idx],
      status: 'done',
      outputPath: path,
      finishedAt: Date.now(),
      speedBytesPerSec: 0,
      etaSeconds: 0
    };
  }

  markError(id: string, message: string) {
    const idx = this.items.findIndex((d) => d.id === id);
    if (idx === -1) return;
    this.items[idx] = {
      ...this.items[idx],
      status: 'error',
      error: message,
      speedBytesPerSec: 0
    };
  }

  async loadHistory() {
    if (!isTauri) return;
    const rows = await listHistory(100, 0);
    // Merge: active items stay, history (done/error) is replaced.
    const activeIds = new Set(
      this.items.filter((d) => d.status === 'active' || d.status === 'queued').map((d) => d.id)
    );
    const historyItems = rows.filter((r) => !activeIds.has(r.id));
    this.items = [
      ...this.items.filter((d) => activeIds.has(d.id)),
      ...historyItems
    ];
  }
}

export const downloads = new DownloadsStore();

const unsubs: Array<() => void> = [];
if (typeof window !== 'undefined') {
  onProgress((u) => downloads.applyUpdate(u)).then((fn) => unsubs.push(fn));
  onDone(({ id, path }) => downloads.markDone(id, path)).then((fn) => unsubs.push(fn));
  onError(({ id, kind, message }) => {
    downloads.markError(id, message);
    errors.push(kind, message, id);
  }).then((fn) => unsubs.push(fn));
}
export function disposeDownloadEvents() {
  unsubs.splice(0).forEach((fn) => fn());
}
