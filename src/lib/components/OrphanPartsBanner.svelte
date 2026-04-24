<script lang="ts">
  import { onMount } from 'svelte';
  import {
    onOrphansFound,
    scanOrphanParts,
    deleteOrphanParts,
    isTauri,
    type OrphanScanResult
  } from '$lib/api/tauri';

  let scan = $state<OrphanScanResult | null>(null);
  let dismissed = $state(false);
  let deleting = $state(false);

  onMount(() => {
    if (!isTauri) return;
    let unsub: (() => void) | null = null;
    onOrphansFound((p) => {
      if (!dismissed) scan = p;
    }).then((fn) => (unsub = fn));
    return () => unsub?.();
  });

  const totalSize = $derived(
    scan ? scan.items.reduce((s, i) => s + i.sizeBytes, 0) : 0
  );

  function fmtMb(b: number): string {
    return `${(b / 1_048_576).toFixed(1)} MB`;
  }

  async function cleanAll() {
    if (!scan) return;
    deleting = true;
    try {
      await deleteOrphanParts(scan.items.map((i) => i.path));
      const refreshed = await scanOrphanParts();
      scan = refreshed.items.length > 0 ? refreshed : null;
    } finally {
      deleting = false;
    }
  }

  function dismiss() {
    dismissed = true;
    scan = null;
  }

  import { i18n } from '$lib/i18n/index.svelte';
  const t = $derived(i18n.t);
</script>

{#if scan && scan.items.length > 0}
  <div class="banner" role="status">
    <div class="msg">{t.orphans.summary(scan.items.length, fmtMb(totalSize), scan.dir)}</div>
    <div class="actions">
      <button onclick={cleanAll} disabled={deleting}>
        {deleting ? t.orphans.cleaning : t.orphans.deleteAll}
      </button>
      <button class="secondary" onclick={dismiss}>{t.orphans.dismiss}</button>
    </div>
  </div>
{/if}

<style>
  .banner {
    border: 1px solid var(--amber-soft);
    background: var(--surface-2);
    padding: 10px 14px;
    display: flex;
    align-items: center;
    gap: 16px;
    font-size: 12px;
    color: var(--text);
  }
  .msg { flex: 1; color: var(--text); }
  .actions { display: flex; gap: 8px; }
  .actions button {
    padding: 4px 12px;
    font-size: 11px;
    background: transparent;
    border: 1px solid var(--amber-soft);
    color: var(--amber);
  }
  .actions button:disabled { opacity: 0.6; cursor: wait; }
  .actions .secondary {
    border-color: var(--line);
    color: var(--dim);
  }
  .actions button:hover:not(:disabled) {
    background: var(--bg);
  }
</style>
