<script lang="ts">
  import type { Download } from '$lib/types';
  import { downloads } from '$lib/state/downloads.svelte';
  import { openInMpv, revealInFileManager } from '$lib/api/tauri';

  interface Props { download: Download; index: number; }
  let { download: d, index }: Props = $props();

  const fmtBytes = (b: number): string => {
    if (b < 1024) return `${b} B`;
    if (b < 1_048_576) return `${(b / 1024).toFixed(1)} KB`;
    if (b < 1_073_741_824) return `${(b / 1_048_576).toFixed(1)} MB`;
    return `${(b / 1_073_741_824).toFixed(2)} GB`;
  };

  const fmtSpeed = (bps: number): string => `${(bps / 1_048_576).toFixed(1)} MB/s`;

  const fmtEta = (s: number): string => {
    if (s < 60) return `eta 0:${String(Math.round(s)).padStart(2, '0')}`;
    const m = Math.floor(s / 60);
    const sec = Math.round(s % 60);
    return `eta ${m}:${String(sec).padStart(2, '0')}`;
  };

  const pct = $derived(
    d.totalBytes > 0 ? Math.min(100, (d.downloadedBytes / d.totalBytes) * 100) : 0
  );

  const num = $derived(String(index).padStart(2, '0'));
</script>

<div class="dl-row" class:queued={d.status === 'queued'} class:done={d.status === 'done'}>
  <div class="dl-num">{num}</div>

  <div class="dl-main">
    <div class="dl-title">{d.title}</div>

    <div class="dl-bar-wrap">
      <div class="dl-bar" style="width: {pct}%"></div>
    </div>

    <div class="dl-stats">
      <span class="codec">{d.codec}</span>

      {#if d.status === 'queued'}
        <span>queued</span>
        <span class="pct">—</span>
      {:else if d.status === 'done'}
        <span>{fmtBytes(d.totalBytes)}</span>
        <span class="pct">done</span>
        {#if d.outputPath}
          <button class="link" onclick={() => openInMpv(d.outputPath!)}>▸ open in mpv</button>
        {/if}
      {:else}
        <span>{fmtBytes(d.downloadedBytes)} / {fmtBytes(d.totalBytes)}</span>
        <span>{fmtSpeed(d.speedBytesPerSec)}</span>
        <span class="pct">{Math.round(pct)}%</span>
        <span class="eta">{fmtEta(d.etaSeconds)}</span>
      {/if}
    </div>
  </div>

  <div class="dl-actions">
    {#if d.status === 'done'}
      <button
        class="dl-act"
        title="Show in folder"
        onclick={() => d.outputPath && revealInFileManager(d.outputPath)}
      >⊡</button>
    {:else if d.status === 'queued'}
      <button class="dl-act" title="Move up">↑</button>
      <button class="dl-act danger" title="Remove" onclick={() => downloads.cancel(d.id)}>✕</button>
    {:else}
      <button class="dl-act" title="Pause">⏸</button>
      <button class="dl-act danger" title="Cancel" onclick={() => downloads.cancel(d.id)}>✕</button>
    {/if}
  </div>
</div>

<style>
  .dl-row {
    display: grid;
    grid-template-columns: 28px 1fr auto;
    gap: 14px;
    align-items: start;
    padding: 12px 18px;
    border-bottom: 1px solid var(--line-soft);
  }
  .dl-row:last-child { border-bottom: none; }

  .link {
    color: var(--cyan, var(--amber));
    background: transparent;
    border: 0;
    cursor: pointer;
    font-size: 10.5px;
    padding: 0;
  }
  .link:hover { text-decoration: underline; }

  .dl-num {
    color: var(--dim);
    font-size: 11px;
    padding-top: 2px;
    font-variant-numeric: tabular-nums;
  }

  .dl-main { min-width: 0; }

  .dl-title {
    color: var(--text-hi);
    font-size: 12.5px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-bottom: 6px;
  }

  .dl-bar-wrap {
    height: 14px;
    background: var(--bg);
    border: 1px solid var(--line-soft);
    position: relative;
    margin-bottom: 5px;
    overflow: hidden;
    font-size: 0;
  }
  .dl-bar {
    height: 100%;
    background: linear-gradient(90deg, var(--amber-soft), var(--amber));
    transition: width 0.6s ease;
    position: relative;
  }
  .dl-bar::after {
    content: "";
    position: absolute;
    inset: 0;
    background: repeating-linear-gradient(
      45deg,
      rgba(0, 0, 0, 0.12) 0 4px,
      transparent 4px 8px
    );
    animation: march 1.2s linear infinite;
  }
  @keyframes march {
    from { background-position: 0 0; }
    to   { background-position: 24px 0; }
  }

  .dl-stats {
    display: flex;
    gap: 14px;
    font-size: 10.5px;
    color: var(--dim);
    font-variant-numeric: tabular-nums;
  }
  .dl-stats .codec { color: var(--cyan); }
  .dl-stats .pct { color: var(--amber); font-weight: 500; }
  .dl-stats .eta { color: var(--text); }

  .dl-actions {
    display: flex;
    gap: 4px;
    padding-top: 1px;
  }
  .dl-act {
    width: 24px;
    height: 24px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--dim);
    border: 1px solid transparent;
    transition: all 0.1s ease;
    font-size: 11px;
  }
  .dl-act:hover {
    color: var(--text-hi);
    border-color: var(--line);
  }
  .dl-act.danger:hover {
    color: var(--rose);
    border-color: var(--rose);
  }

  /* status variants */
  .queued .dl-bar {
    background: var(--line);
  }
  .queued .dl-bar::after { display: none; }
  .queued .dl-title { color: var(--text); }
  .queued .pct { color: var(--dim); }

  .done .dl-bar {
    background: var(--olive);
    opacity: 0.6;
  }
  .done .dl-bar::after { display: none; }
  .done .pct { color: var(--olive); }
</style>
