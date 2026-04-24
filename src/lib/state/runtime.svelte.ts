import type { RuntimeInfo } from '$lib/types';
import { getRuntimeInfo } from '$lib/api/tauri';

class RuntimeStore {
  info = $state<RuntimeInfo | null>(null);

  async load() {
    this.info = await getRuntimeInfo();
  }
}

export const runtime = new RuntimeStore();

if (typeof window !== 'undefined') {
  runtime.load();
}
