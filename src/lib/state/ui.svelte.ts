import { getSetting, setSetting, isTauri } from '$lib/api/tauri';

type View = 'download' | 'queue' | 'history' | 'presets' | 'cookies';

class UiStore {
  activeView         = $state<View>('download');
  clipboardListening = $state(true);
  concurrentLimit    = $state(3);
  throttleEnabled    = $state(false);
  throttleMbps       = $state(5);
  settingsOpen       = $state(false);

  setView(v: View) { this.activeView = v; }
  toggleClipboard() { this.clipboardListening = !this.clipboardListening; }
  openSettings() { this.settingsOpen = true; }
  closeSettings() { this.settingsOpen = false; }

  async load() {
    if (!isTauri) return;
    const limit = await getSetting('concurrent_limit');
    if (limit) this.concurrentLimit = Number(limit) || 3;
    const thr = await getSetting('throttle_enabled');
    if (thr) this.throttleEnabled = thr === '1';
    const thrMbps = await getSetting('throttle_mbps');
    if (thrMbps) this.throttleMbps = Number(thrMbps) || 5;
    const clip = await getSetting('clipboard_listening');
    if (clip) this.clipboardListening = clip === '1';
  }

  async persist() {
    if (!isTauri) return;
    await setSetting('concurrent_limit', String(this.concurrentLimit));
    await setSetting('throttle_enabled', this.throttleEnabled ? '1' : '0');
    await setSetting('throttle_mbps', String(this.throttleMbps));
    await setSetting('clipboard_listening', this.clipboardListening ? '1' : '0');
  }
}

export const ui = new UiStore();
