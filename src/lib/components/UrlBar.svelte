<script lang="ts">
  import { metadata } from '$lib/state/metadata.svelte';
  import { presets } from '$lib/state/presets.svelte';
  import { downloads } from '$lib/state/downloads.svelte';
  import { runtime } from '$lib/state/runtime.svelte';
  import { ui } from '$lib/state/ui.svelte';

  let url = $state('');
  let queueing = $state(false);
  let queueError = $state<string | null>(null);

  async function trigger() {
    if (!url.trim()) return;
    await metadata.fetch(url);
  }

  async function queue() {
    const meta = metadata.current;
    if (!meta) return;
    queueing = true;
    queueError = null;
    try {
      await downloads.enqueue(
        {
          url: meta.url,
          formatSpec: metadata.selectedFormat?.spec ?? '',
          presetId: presets.selected?.id ?? '',
          outputDir: runtime.info?.outputDir ?? '~/dl/yt',
          flags: presets.selected?.flags ?? [],
          subtitleOpts: metadata.subtitleOpts
        },
        meta.title,
        metadata.selectedFormat?.label ?? ''
      );
      url = '';
      metadata.clear();
    } catch (e) {
      queueError = e instanceof Error ? e.message : String(e);
    } finally {
      queueing = false;
    }
  }

  function onKey(e: KeyboardEvent) {
    if (e.key === 'Enter') trigger();
  }
</script>

<div class="panel">
  <div class="urlbar">
    <div class="urlbar-label">
      <span>source url</span>
      {#if ui.clipboardListening}
        <span class="clip">
          <span class="pulse"></span>
          clipboard listener active
        </span>
      {/if}
    </div>

    <div class="url-input-wrap" class:focused={metadata.isLoading}>
      <span class="url-prompt">$</span>
      <input
        bind:value={url}
        placeholder="https://… or paste from clipboard"
        autocomplete="off"
        onkeydown={onKey}
      />
      <button class="url-go" onclick={trigger} disabled={metadata.isLoading}>
        <span>{metadata.isLoading ? 'fetching…' : 'fetch'}</span>
        <span class="key">↵</span>
      </button>
      {#if metadata.state === 'ready'}
        <button class="url-queue" onclick={queue} disabled={queueing}>
          <span>{queueing ? 'queueing…' : '▸ queue'}</span>
        </button>
      {/if}
    </div>

    {#if queueError}
      <div class="err">{queueError}</div>
    {/if}
  </div>
</div>

<style>
  .panel {
    border: 1px solid var(--line);
    background: var(--surface);
    position: relative;
  }
  .panel::before, .panel::after {
    content: "";
    position: absolute;
    width: 8px;
    height: 8px;
    border-color: var(--amber-soft);
    opacity: 0.5;
  }
  .panel::before { top: -1px; left: -1px; border-top: 1px solid; border-left: 1px solid; }
  .panel::after  { bottom: -1px; right: -1px; border-bottom: 1px solid; border-right: 1px solid; }

  .urlbar { padding: 18px; }

  .urlbar-label {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.2em;
    color: var(--dim);
    margin-bottom: 8px;
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .urlbar-label .clip {
    color: var(--olive);
    display: inline-flex;
    align-items: center;
    gap: 5px;
    font-size: 10px;
    letter-spacing: 0.1em;
  }

  .clip .pulse {
    width: 5px;
    height: 5px;
    background: var(--olive);
    border-radius: 50%;
    animation: pulse 1.6s ease-in-out infinite;
  }
  @keyframes pulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50%      { opacity: 0.3; transform: scale(0.7); }
  }

  .url-input-wrap {
    display: flex;
    align-items: center;
    border: 1px solid var(--line);
    background: var(--bg);
    transition: border-color 0.15s ease;
  }
  .url-input-wrap:focus-within { border-color: var(--amber); }

  .url-prompt {
    padding: 10px 12px;
    color: var(--amber);
    font-weight: 500;
    border-right: 1px solid var(--line);
    line-height: 1;
  }

  input {
    padding: 10px 12px;
    color: var(--text-hi);
    font-size: 13px;
  }
  input::placeholder { color: var(--dim); }

  .url-go {
    padding: 0 14px;
    color: var(--dim);
    border-left: 1px solid var(--line);
    height: 34px;
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    transition: all 0.12s ease;
  }
  .url-go:hover:not(:disabled) {
    color: var(--amber);
    background: var(--surface-2);
  }
  .url-go:disabled { color: var(--dim); cursor: wait; }

  .url-go .key {
    font-size: 9px;
    border: 1px solid var(--line);
    padding: 1px 4px;
    border-radius: 2px;
  }

  .url-queue {
    padding: 0 18px;
    color: var(--bg);
    background: var(--amber);
    border: 0;
    border-left: 1px solid var(--amber);
    height: 34px;
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11.5px;
    font-weight: 500;
    letter-spacing: 0.04em;
    transition: all 0.12s ease;
    cursor: pointer;
  }
  .url-queue:hover:not(:disabled) {
    background: var(--text-hi, #fff);
    color: var(--bg);
  }
  .url-queue:disabled { opacity: 0.5; cursor: wait; }

  .err {
    margin-top: 8px;
    color: var(--red, #d66);
    font-size: 11px;
  }
</style>
