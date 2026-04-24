import type { DownloadErrorKind } from '$lib/api/tauri';

/**
 * Error toast store. The kind discriminator is preserved as raw data;
 * `ErrorToast.svelte` renders kind-specific copy via the i18n dictionary.
 */
export type ErrorKind = DownloadErrorKind | 'fetch_failed';

export interface ToastError {
  id: string;
  kind: ErrorKind;
  message: string;
  downloadId?: string;
  createdAt: number;
}

class ErrorsStore {
  items = $state<ToastError[]>([]);
  private _nextId = 0;

  push(kind: ErrorKind, message: string, downloadId?: string) {
    this._nextId += 1;
    const id = `err-${this._nextId}`;
    this.items = [
      ...this.items,
      { id, kind, message, downloadId, createdAt: Date.now() }
    ];
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
