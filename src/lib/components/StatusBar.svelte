<script lang="ts">
  import { downloads } from '$lib/state/downloads.svelte';
  import { presets } from '$lib/state/presets.svelte';
  import { runtime } from '$lib/state/runtime.svelte';
  import { i18n } from '$lib/i18n/index.svelte';

  const t = $derived(i18n.t);
  const speedMb = $derived((downloads.totalSpeed / 1_048_576).toFixed(1));
</script>

<footer class="statusbar">
  <div class="seg">
    <span class="pulse-dot"></span>
    <span class="live">{t.status.ready}</span>
  </div>
  <div class="seg">
    <span class="lbl">{t.status.active}</span>
    <span class="val">{downloads.active}</span>
  </div>
  <div class="seg">
    <span class="lbl">{t.status.queued}</span>
    <span class="val">{downloads.queued}</span>
  </div>
  <div class="seg">
    <span class="lbl">↓</span>
    <span class="val">{speedMb} MB/s</span>
  </div>
  <div class="seg">
    <span class="lbl">{t.status.preset}</span>
    <span class="val">{presets.selected?.name ?? '—'}</span>
  </div>

  <div class="spacer"></div>

  {#if runtime.info}
    <div class="seg">
      <span class="lbl">{t.status.target}</span>
      <span class="path">{runtime.info.outputDir}</span>
    </div>
  {/if}
  <div class="seg">
    <span class="blink"></span>
  </div>
</footer>

<style>
  .statusbar {
    display: flex;
    align-items: center;
    gap: 18px;
    padding: 7px 18px;
    border-top: 1px solid var(--line);
    background: var(--bg-soft);
    font-size: 11px;
    color: var(--dim);
    font-variant-numeric: tabular-nums;
  }
  .seg { display: flex; align-items: center; gap: 6px; }
  .seg .lbl {
    color: var(--dim);
    text-transform: uppercase;
    letter-spacing: 0.1em;
    font-size: 10px;
  }
  .seg .val { color: var(--text-hi); }
  .seg .live { color: var(--olive); }
  .spacer { flex: 1; }
  .path { font-family: var(--mono); color: var(--text); }

  .pulse-dot {
    width: 7px;
    height: 7px;
    background: var(--olive);
    border-radius: 50%;
    animation: pulse 1.6s ease-in-out infinite;
    box-shadow: 0 0 6px var(--olive);
  }
  @keyframes pulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50%      { opacity: 0.3; transform: scale(0.7); }
  }

  .blink {
    display: inline-block;
    width: 6px;
    height: 11px;
    background: var(--amber);
    animation: blink 1s steps(2) infinite;
    vertical-align: -1px;
    margin-left: 2px;
  }
  @keyframes blink {
    50% { opacity: 0; }
  }
</style>
