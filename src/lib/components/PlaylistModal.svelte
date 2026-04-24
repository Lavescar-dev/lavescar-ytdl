<script lang="ts">
  import { metadata } from '$lib/state/metadata.svelte';
  import { presets } from '$lib/state/presets.svelte';
  import { downloads } from '$lib/state/downloads.svelte';
  import { runtime } from '$lib/state/runtime.svelte';
  import { i18n } from '$lib/i18n/index.svelte';

  const t = $derived(i18n.t);

  const pl = $derived(metadata.playlist);

  let selected = $state(new Set<string>());
  let rangeInput = $state('');
  let queueing = $state(false);
  let queueError = $state<string | null>(null);

  // Reset selection whenever a new playlist lands.
  $effect(() => {
    if (metadata.playlist) {
      selected = new Set(metadata.playlist.entries.map((e) => e.id));
      rangeInput = '';
      queueError = null;
    }
  });

  function toggle(id: string) {
    const next = new Set(selected);
    if (next.has(id)) next.delete(id);
    else next.add(id);
    selected = next;
  }

  function selectAll() {
    if (!pl) return;
    selected = new Set(pl.entries.map((e) => e.id));
  }
  function selectNone() {
    selected = new Set();
  }
  function applyRange() {
    if (!pl) return;
    const n = pl.entries.length;
    const picks = new Set<string>();
    for (const seg of rangeInput.split(',').map((s) => s.trim()).filter(Boolean)) {
      const m = /^(\d+)(?:-(\d+))?$/.exec(seg);
      if (!m) continue;
      const start = Math.max(1, parseInt(m[1], 10));
      const end = m[2] ? Math.min(n, parseInt(m[2], 10)) : start;
      for (let i = start; i <= end; i++) {
        const e = pl.entries[i - 1];
        if (e) picks.add(e.id);
      }
    }
    selected = picks;
  }

  async function queueSelected() {
    if (!pl) return;
    queueing = true;
    queueError = null;
    try {
      const entries = pl.entries.filter((e) => selected.has(e.id));
      const codec = metadata.selectedFormat?.label ?? '';
      for (const e of entries) {
        await downloads.enqueue(
          {
            url: e.url,
            formatSpec: metadata.selectedFormat?.spec ?? '',
            presetId: presets.selected?.id ?? '',
            outputDir: runtime.info?.outputDir ?? '~/dl/yt',
            flags: presets.selected?.flags ?? [],
            title: e.title,
            codec
          },
          e.title,
          codec
        );
      }
      metadata.clear();
    } catch (err) {
      queueError = err instanceof Error ? err.message : String(err);
    } finally {
      queueing = false;
    }
  }

  function close() { metadata.clear(); }

  function onKey(e: KeyboardEvent) {
    if (e.key === 'Escape') close();
  }
</script>

<svelte:window onkeydown={onKey} />

{#if pl}
  <div class="scrim" onclick={close} role="button" tabindex="-1" onkeydown={() => {}}>
    <div class="card" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true" tabindex="-1" onkeydown={() => {}}>
      <div class="head">
        <div>
          <h2>{pl.title}</h2>
          {#if pl.uploader}
            <div class="sub">{t.playlist.by} {pl.uploader}</div>
          {/if}
        </div>
        <button class="x" onclick={close} aria-label={t.playlist.close}>×</button>
      </div>

      <div class="controls">
        <span class="count">{t.playlist.selectedOf(selected.size, pl.entries.length)}</span>
        <div class="spacer"></div>
        <button onclick={selectAll}>{t.playlist.all}</button>
        <button onclick={selectNone}>{t.playlist.none}</button>
        <input
          bind:value={rangeInput}
          placeholder={t.playlist.rangePlaceholder}
          class="range"
        />
        <button onclick={applyRange}>{t.playlist.apply}</button>
      </div>

      <ul class="list">
        {#each pl.entries as e, i (e.id)}
          <li class="entry">
            <label>
              <input
                type="checkbox"
                checked={selected.has(e.id)}
                onchange={() => toggle(e.id)}
              />
              <span class="num">{String(i + 1).padStart(3, '0')}</span>
              <span class="ttl">{e.title}</span>
              {#if e.duration}
                <span class="dur">{e.duration}</span>
              {/if}
            </label>
          </li>
        {/each}
      </ul>

      {#if queueError}<div class="err">{queueError}</div>{/if}

      <div class="foot">
        <span class="dim">{t.playlist.preset}: {presets.selected?.name ?? '—'}</span>
        <button onclick={close}>{t.playlist.cancel}</button>
        <button
          class="primary"
          onclick={queueSelected}
          disabled={queueing || selected.size === 0}
        >
          {queueing ? t.playlist.queueing : t.playlist.queueN(selected.size)}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .scrim {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.6);
    display: grid;
    place-items: center;
    z-index: 115;
  }
  .card {
    background: var(--surface);
    border: 1px solid var(--line);
    padding: 20px 22px;
    width: min(720px, 94vw);
    max-height: 90vh;
    display: flex;
    flex-direction: column;
  }
  .head {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 12px;
    border-bottom: 1px solid var(--line);
    padding-bottom: 12px;
    margin-bottom: 12px;
  }
  .head h2 {
    margin: 0;
    font-size: 13px;
    color: var(--text-hi);
    text-transform: uppercase;
    letter-spacing: 0.18em;
  }
  .sub {
    margin-top: 3px;
    color: var(--dim);
    font-size: 11px;
  }
  .x {
    background: transparent;
    border: 0;
    color: var(--dim);
    font-size: 18px;
    line-height: 1;
    cursor: pointer;
  }

  .controls {
    display: flex;
    gap: 8px;
    align-items: center;
    margin-bottom: 12px;
    font-size: 11px;
  }
  .controls .count { color: var(--dim); }
  .controls .spacer { flex: 1; }
  .controls button {
    padding: 4px 10px;
    background: transparent;
    border: 1px solid var(--line);
    color: var(--text);
    font-size: 11px;
  }
  .controls button:hover {
    color: var(--amber);
    border-color: var(--amber-soft);
  }
  .controls .range {
    padding: 4px 8px;
    background: var(--bg);
    border: 1px solid var(--line);
    color: var(--text-hi);
    font-size: 11px;
    width: 180px;
  }

  .list {
    list-style: none;
    padding: 0;
    margin: 0;
    overflow-y: auto;
    flex: 1;
    border: 1px solid var(--line-soft);
  }
  .entry {
    border-bottom: 1px solid var(--line-soft);
  }
  .entry:last-child { border-bottom: 0; }
  .entry label {
    display: grid;
    grid-template-columns: 20px 42px 1fr auto;
    gap: 10px;
    align-items: center;
    padding: 7px 12px;
    cursor: pointer;
    font-size: 12px;
  }
  .entry label:hover { background: var(--surface-2); }
  .entry input { accent-color: var(--amber); }
  .num {
    color: var(--dim);
    font-size: 10.5px;
    font-variant-numeric: tabular-nums;
  }
  .ttl {
    color: var(--text-hi);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .dur {
    color: var(--dim);
    font-size: 10.5px;
  }

  .err { color: var(--rose); font-size: 11px; margin-top: 8px; }

  .foot {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 14px;
    align-items: center;
  }
  .dim { color: var(--dim); font-size: 11px; margin-right: auto; }
  .foot button {
    padding: 6px 14px;
    font-size: 11px;
    border: 1px solid var(--line);
    background: transparent;
    color: var(--text);
  }
  .foot .primary {
    background: var(--amber);
    color: var(--bg);
    border-color: var(--amber);
  }
  .foot .primary:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
