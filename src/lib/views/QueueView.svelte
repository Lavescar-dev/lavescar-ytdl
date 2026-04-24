<script lang="ts">
  import { downloads } from '$lib/state/downloads.svelte';
  import { i18n } from '$lib/i18n/index.svelte';

  const t = $derived(i18n.t);

  const items = $derived(
    downloads.items.filter((d) => d.status === 'queued' || d.status === 'active')
  );

  function cancel(id: string) {
    downloads.cancel(id);
  }

  function statusLabel(s: string): string {
    if (s === 'queued') return t.download.queued;
    if (s === 'active') return t.download.active;
    return s;
  }
</script>

<div class="view">
  <div class="head">
    <h2>{t.queueView.title}</h2>
    <span class="dim">{t.queueView.items(items.length)}</span>
  </div>

  {#if items.length === 0}
    <div class="empty">{t.queueView.empty}</div>
  {:else}
    <ul class="list">
      {#each items as d (d.id)}
        <li class="row">
          <span class="st st-{d.status}">{statusLabel(d.status)}</span>
          <span class="ttl">{d.title}</span>
          <span class="dim codec">{d.codec}</span>
          <button class="cancel" onclick={() => cancel(d.id)}>{t.queueView.cancel}</button>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .view {
    border: 1px solid var(--line);
    background: var(--surface);
    padding: 16px 18px;
  }
  .head {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    margin-bottom: 14px;
  }
  .head h2 {
    font-size: 13px;
    color: var(--text-hi);
    text-transform: uppercase;
    letter-spacing: 0.2em;
    margin: 0;
  }
  .dim { color: var(--dim); font-size: 11px; }
  .empty { color: var(--dim); padding: 40px 0; text-align: center; font-size: 12px; }
  .list { list-style: none; padding: 0; margin: 0; }
  .row {
    display: grid;
    grid-template-columns: 72px 1fr auto auto;
    gap: 12px;
    align-items: center;
    padding: 10px 2px;
    border-bottom: 1px solid var(--line);
    font-size: 12px;
  }
  .ttl { color: var(--text-hi); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .codec { font-size: 10.5px; }
  .st {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    padding: 2px 6px;
    border: 1px solid var(--line);
    text-align: center;
  }
  .st-queued { color: var(--dim); }
  .st-active { color: var(--amber); border-color: var(--amber-soft); }
  .cancel {
    font-size: 10px;
    color: var(--dim);
    padding: 2px 10px;
    border: 1px solid var(--line);
    background: transparent;
  }
  .cancel:hover { color: var(--rose); border-color: var(--rose); }
</style>
