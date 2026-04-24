<script lang="ts">
  import { downloads } from '$lib/state/downloads.svelte';
  import { i18n } from '$lib/i18n/index.svelte';

  const t = $derived(i18n.t);

  let search = $state('');

  function statusLabel(s: string): string {
    if (s === 'done') return t.history.statusDone;
    if (s === 'error') return t.history.statusError;
    if (s === 'cancelled') return t.history.statusCancelled;
    return s;
  }

  const completed = $derived(
    downloads.items
      .filter(
        (d) =>
          d.status === 'done' ||
          d.status === 'error' ||
          d.status === 'cancelled'
      )
      .filter((d) => {
        const q = search.trim().toLowerCase();
        if (!q) return true;
        return (
          d.title.toLowerCase().includes(q) ||
          d.url.toLowerCase().includes(q) ||
          (d.codec ?? '').toLowerCase().includes(q)
        );
      })
      .sort((a, b) => (b.finishedAt ?? 0) - (a.finishedAt ?? 0))
  );

  function ymd(ms?: number): string {
    if (!ms) return 'unknown';
    const d = new Date(ms);
    return d.toISOString().slice(0, 10);
  }

  const grouped = $derived.by(() => {
    const map = new Map<string, typeof completed>();
    for (const d of completed) {
      const k = ymd(d.finishedAt);
      const g = map.get(k) ?? [];
      g.push(d);
      map.set(k, g);
    }
    return Array.from(map.entries());
  });

  function reveal(path: string) {
    navigator.clipboard?.writeText(path).catch(() => {});
  }
</script>

<div class="view">
  <div class="head">
    <h2>{t.history.title}</h2>
    <input
      bind:value={search}
      placeholder={t.history.searchPlaceholder}
      class="search"
    />
  </div>

  {#if completed.length === 0}
    <div class="empty">{t.history.empty}</div>
  {:else}
    {#each grouped as [day, rows]}
      <div class="day">
        <div class="day-lbl">{day}</div>
        <ul class="list">
          {#each rows as d (d.id)}
            <li class="row">
              <span class="st st-{d.status}">{statusLabel(d.status)}</span>
              <span class="ttl" title={d.url}>{d.title}</span>
              <span class="dim codec">{d.codec}</span>
              {#if d.outputPath}
                <button class="copy" onclick={() => reveal(d.outputPath!)}>{t.history.copyPath}</button>
              {/if}
              {#if d.error}
                <span class="err" title={d.error}>!</span>
              {/if}
            </li>
          {/each}
        </ul>
      </div>
    {/each}
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
    align-items: center;
    justify-content: space-between;
    margin-bottom: 14px;
    gap: 14px;
  }
  .head h2 {
    font-size: 13px;
    color: var(--text-hi);
    text-transform: uppercase;
    letter-spacing: 0.2em;
    margin: 0;
  }
  .search {
    flex: 0 1 280px;
    padding: 6px 10px;
    background: var(--bg);
    border: 1px solid var(--line);
    color: var(--text-hi);
    font-size: 12px;
  }
  .dim { color: var(--dim); }
  .empty { color: var(--dim); padding: 40px 0; text-align: center; font-size: 12px; }
  .day { margin-bottom: 18px; }
  .day-lbl {
    font-size: 10px;
    color: var(--dim);
    text-transform: uppercase;
    letter-spacing: 0.2em;
    padding: 6px 0;
    border-bottom: 1px solid var(--line);
    margin-bottom: 4px;
  }
  .list { list-style: none; padding: 0; margin: 0; }
  .row {
    display: grid;
    grid-template-columns: 68px 1fr auto auto auto;
    gap: 12px;
    align-items: center;
    padding: 8px 2px;
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
  .st-done      { color: var(--olive); border-color: var(--olive); }
  .st-error     { color: var(--rose);  border-color: var(--rose); }
  .st-cancelled { color: var(--dim);   border-color: var(--dim); }
  .copy {
    font-size: 10px;
    color: var(--dim);
    padding: 2px 8px;
    border: 1px solid var(--line);
    background: transparent;
  }
  .copy:hover { color: var(--amber); border-color: var(--amber-soft); }
  .err { color: var(--rose); font-weight: bold; }
</style>
