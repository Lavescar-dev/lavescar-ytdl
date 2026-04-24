<script lang="ts">
  import { onMount } from 'svelte';
  import { ui } from '$lib/state/ui.svelte';
  import { metadata } from '$lib/state/metadata.svelte';
  import { onClipboardUrl, type ClipboardUrlPayload, isTauri } from '$lib/api/tauri';

  let pending = $state<ClipboardUrlPayload | null>(null);

  onMount(() => {
    if (!isTauri) return;
    let unsub: (() => void) | null = null;
    onClipboardUrl((p) => {
      if (!ui.clipboardListening) return;
      // Don't prompt if the user is mid-fetch or already looking at metadata
      // for a different URL — they probably already know.
      if (metadata.state === 'loading') return;
      pending = p;
    }).then((fn) => (unsub = fn));
    return () => unsub?.();
  });

  function truncate(s: string, n = 72): string {
    return s.length > n ? s.slice(0, n - 1) + '…' : s;
  }

  function accept() {
    if (!pending) return;
    metadata.fetch(pending.url);
    pending = null;
  }

  function dismiss() {
    pending = null;
  }

  import { i18n } from '$lib/i18n/index.svelte';
  const t = $derived(i18n.t);
</script>

{#if pending}
  <div class="prompt" role="status">
    <span class="src">{pending.source}</span>
    <span class="url" title={pending.url}>{truncate(pending.url)}</span>
    <div class="actions">
      <button class="primary" onclick={accept}>{t.clipboard.fetch}</button>
      <button onclick={dismiss}>{t.clipboard.dismiss}</button>
    </div>
  </div>
{/if}

<style>
  .prompt {
    border: 1px solid var(--amber-soft);
    background: var(--surface-2);
    padding: 8px 12px;
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 12px;
  }
  .src {
    font-size: 9.5px;
    color: var(--amber);
    text-transform: uppercase;
    letter-spacing: 0.14em;
    border: 1px solid var(--amber-soft);
    padding: 1px 6px;
  }
  .url {
    color: var(--text-hi);
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-family: var(--mono, ui-monospace, monospace);
    font-size: 11.5px;
  }
  .actions {
    display: flex;
    gap: 6px;
  }
  .actions button {
    font-size: 11px;
    padding: 3px 10px;
    background: transparent;
    border: 1px solid var(--line);
    color: var(--text);
  }
  .actions .primary {
    border-color: var(--amber-soft);
    color: var(--amber);
  }
  .actions button:hover { background: var(--bg); }
</style>
