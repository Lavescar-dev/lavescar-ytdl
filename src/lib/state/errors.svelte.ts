import type { DownloadErrorKind } from '$lib/api/tauri';

export interface ToastError {
  id: string;
  kind: DownloadErrorKind | 'fetch_failed';
  title: string;
  message: string;
  suggestion: string;
  downloadId?: string;
  createdAt: number;
}

const KIND_COPY: Record<ToastError['kind'], { title: string; suggestion: string }> = {
  geo_blocked: {
    title: 'Geo-blocked',
    suggestion: 'Try a VPN or load cookies via Settings → cookie source.'
  },
  auth_required: {
    title: 'Sign-in required',
    suggestion: 'Switch to the Cookies view and pick a browser with a logged-in session.'
  },
  not_found: {
    title: 'Video unavailable',
    suggestion: 'The video may have been removed, set to private, or region-locked.'
  },
  network: {
    title: 'Network error',
    suggestion: 'Check your connection. yt-dlp automatically retries; give it a moment.'
  },
  io: {
    title: 'Disk error',
    suggestion: 'Verify the output directory exists and has free space.'
  },
  parse: {
    title: 'Response parse error',
    suggestion: 'yt-dlp output was unexpected. Try updating yt-dlp from Settings.'
  },
  shell: {
    title: 'Binary launch error',
    suggestion: 'The yt-dlp sidecar could not start. Reinstall or update it.'
  },
  unknown: {
    title: 'yt-dlp error',
    suggestion: 'Check the full message below, or try again.'
  },
  fetch_failed: {
    title: 'Metadata fetch failed',
    suggestion: 'The URL could not be inspected. Double-check it is a supported site.'
  }
};

class ErrorsStore {
  items = $state<ToastError[]>([]);

  private _nextId = 0;

  push(kind: ToastError['kind'], message: string, downloadId?: string) {
    this._nextId += 1;
    const id = `err-${this._nextId}`;
    const copy = KIND_COPY[kind] ?? KIND_COPY.unknown;
    this.items = [
      ...this.items,
      {
        id,
        kind,
        title: copy.title,
        suggestion: copy.suggestion,
        message,
        downloadId,
        createdAt: Date.now()
      }
    ];
    // Auto-dismiss after 8s.
    const forget = id;
    setTimeout(() => this.dismiss(forget), 8_000);
  }

  dismiss(id: string) {
    this.items = this.items.filter((e) => e.id !== id);
  }

  clear() {
    this.items = [];
  }
}

export const errors = new ErrorsStore();
