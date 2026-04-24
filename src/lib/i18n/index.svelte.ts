/**
 * Tiny rune-based i18n store, mirrors the landing page's pattern.
 *
 * Source priority: persisted setting (DB `lang`) > localStorage > navigator.language > 'en'.
 * Browser detection runs once on `init`. Manual `set()` writes both
 * `localStorage` (sync, fast) and the SQLite `settings` row (durable).
 */

import { dictionaries, en, type Dict, type Locale } from './dict';
import { getSetting, setSetting, isTauri } from '$lib/api/tauri';

const STORAGE_KEY = 'lavescar-ytdl.lang';

function detectFromBrowser(): Locale {
  if (typeof window === 'undefined') return 'en';
  const stored = localStorage.getItem(STORAGE_KEY);
  if (stored === 'en' || stored === 'tr') return stored;
  const nav = navigator.language?.toLowerCase() ?? '';
  if (nav.startsWith('tr')) return 'tr';
  return 'en';
}

class I18nStore {
  locale = $state<Locale>('en');
  t = $derived<Dict>(dictionaries[this.locale] ?? en);

  async init() {
    // Browser-side default — runs synchronously, gives the first paint a locale.
    this.locale = detectFromBrowser();
    if (typeof document !== 'undefined') {
      document.documentElement.lang = this.locale;
    }

    // Tauri override: if the user previously persisted a choice in SQLite,
    // honour that over the browser detection.
    if (isTauri) {
      try {
        const fromDb = await getSetting('lang');
        if (fromDb === 'en' || fromDb === 'tr') {
          this.locale = fromDb;
          if (typeof document !== 'undefined') document.documentElement.lang = fromDb;
        }
      } catch {
        // Settings table not ready, ignore.
      }
    }
  }

  async set(loc: Locale) {
    this.locale = loc;
    if (typeof window !== 'undefined') {
      localStorage.setItem(STORAGE_KEY, loc);
      document.documentElement.lang = loc;
    }
    if (isTauri) {
      try {
        await setSetting('lang', loc);
      } catch {
        // best-effort
      }
    }
  }
}

export const i18n = new I18nStore();
