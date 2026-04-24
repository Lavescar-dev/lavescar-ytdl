<script lang="ts">
  import { downloads } from '$lib/state/downloads.svelte';

  const items = $derived(
    downloads.items.filter((d) => d.status === 'queued' || d.status === 'active')
  );

  function cancel(id: string) {
    downloads.cancel(id);
  }
</script>

<div class="view">
  <div class="head">
    <h2>queue</h2>
    <span class="dim">{items.length} item{items.length === 1 ? '' : 's'}</span>
  </div>

  {#if items.length === 0}
    <div class="empty">
      queue is empty — paste a URL in the download view.
    </div>
  {:else}
    <ul class="list">
      {#each items as d (d.id)}
        <li class="row">
          <span class="st st-{d.status}">{d.status}</span>
          <span class="ttl">{d.title}</span>
          <span class="dim codec">{d.codec}</span>
          <button class="cancel" onclick={() => cancel(d.id)}>cancel</button>
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
