<script lang="ts">
  import { downloads } from '$lib/state/downloads.svelte';
  import { ui } from '$lib/state/ui.svelte';
  import { i18n } from '$lib/i18n/index.svelte';
  import DownloadRow from './DownloadRow.svelte';

  const t = $derived(i18n.t);

  // ACTIVE panel only shows in-flight work; completed/errored items live in History.
  const sortedItems = $derived.by(() => {
    const order: Record<string, number> = { active: 0, queued: 1, paused: 2 };
    return downloads.items
      .filter((d) => d.status === 'active' || d.status === 'queued' || d.status === 'paused')
      .slice()
      .sort((a, b) => (order[a.status] ?? 9) - (order[b.status] ?? 9));
  });
</script>

<div class="panel">
  <div class="panel-h">
    <span class="dot"></span>
    <span class="title-text">{t.download.active}</span>
    <span class="meta">
      {downloads.active} {t.download.running} · {t.download.concurrent}: {ui.concurrentLimit} · {t.download.throttle}: {ui.throttleEnabled ? t.download.throttleOn : t.download.throttleOff}
    </span>
  </div>

  <div class="dl-list">
    {#each sortedItems as d, i (d.id)}
      <DownloadRow download={d} index={d.status === 'done' ? 0 : i + 1} />
    {/each}
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

  .panel-h {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 14px;
    border-bottom: 1px solid var(--line);
    background: var(--surface-2);
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.14em;
    color: var(--dim);
  }
  .panel-h .dot {
    width: 6px;
    height: 6px;
    background: var(--amber);
    display: inline-block;
    border-radius: 50%;
  }
  .panel-h .title-text { color: var(--text-hi); font-weight: 500; }
  .panel-h .meta {
    margin-left: auto;
    color: var(--dim);
    text-transform: none;
    letter-spacing: 0;
    font-size: 11px;
  }

  .dl-list { padding: 0; }
</style>
