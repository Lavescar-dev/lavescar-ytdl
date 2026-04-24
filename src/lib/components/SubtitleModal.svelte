<script lang="ts">
  import { metadata } from '$lib/state/metadata.svelte';
  import type { SubtitleOpts } from '$lib/types';

  // Grouped language list for the current video. Manual entries take precedence
  // if the same code shows up in both buckets.
  const langs = $derived.by(() => {
    const cur = metadata.current;
    if (!cur) return [] as { code: string; manual: boolean; auto: boolean }[];
    const map = new Map<string, { code: string; manual: boolean; auto: boolean }>();
    for (const s of cur.availableSubs) {
      const e = map.get(s.code) ?? { code: s.code, manual: false, auto: false };
      if (s.auto) e.auto = true;
      else e.manual = true;
      map.set(s.code, e);
    }
    return [...map.values()].sort((a, b) => a.code.localeCompare(b.code));
  });

  let selected = $state<Set<string>>(new Set());
  let autoMode = $state(false);
  let embed = $state(true);

  $effect(() => {
    if (metadata.subtitleModalOpen && metadata.subtitleOpts) {
      selected = new Set(metadata.subtitleOpts.langs);
      autoMode = metadata.subtitleOpts.auto;
      embed = metadata.subtitleOpts.embed;
    } else if (metadata.subtitleModalOpen) {
      // Fresh open: default to no selection, prefer manual subs.
      selected = new Set();
      autoMode = false;
      embed = true;
    }
  });

  function toggle(code: string) {
    const next = new Set(selected);
    if (next.has(code)) next.delete(code);
    else next.add(code);
    selected = next;
  }

  function apply() {
    const opts: SubtitleOpts | null =
      selected.size === 0
        ? null
        : { langs: [...selected], auto: autoMode, embed };
    metadata.setSubtitleOpts(opts);
    metadata.closeSubtitleModal();
  }

  function clear() {
    metadata.setSubtitleOpts(null);
    metadata.closeSubtitleModal();
  }

  function onKey(e: KeyboardEvent) {
    if (e.key === 'Escape') metadata.closeSubtitleModal();
  }
</script>

<svelte:window onkeydown={onKey} />

{#if metadata.subtitleModalOpen && metadata.current}
  <div class="scrim" onclick={() => metadata.closeSubtitleModal()} role="button" tabindex="-1" onkeydown={() => {}}>
    <div class="card" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true" tabindex="-1" onkeydown={() => {}}>
      <div class="head">
        <h2>subtitles</h2>
        <button class="x" onclick={() => metadata.closeSubtitleModal()} aria-label="Close">×</button>
      </div>

      <div class="mode">
        <label>
          <input type="radio" bind:group={autoMode} value={false} />
          manual captions (author-provided)
        </label>
        <label>
          <input type="radio" bind:group={autoMode} value={true} />
          auto-generated captions
        </label>
      </div>

      {#if langs.length === 0}
        <div class="empty">no subtitles available for this video.</div>
      {:else}
        <div class="hint">
          {selected.size} of {langs.length} language{langs.length === 1 ? '' : 's'} selected
        </div>
        <ul class="langs">
          {#each langs as l (l.code)}
            <li>
              <label class:disabled={autoMode ? !l.auto : !l.manual}>
                <input
                  type="checkbox"
                  disabled={autoMode ? !l.auto : !l.manual}
                  checked={selected.has(l.code)}
                  onchange={() => toggle(l.code)}
                />
                <span class="code">{l.code}</span>
                <span class="tags">
                  {#if l.manual}<span class="tag manual">manual</span>{/if}
                  {#if l.auto}<span class="tag auto">auto</span>{/if}
                </span>
              </label>
            </li>
          {/each}
        </ul>
      {/if}

      <div class="embed">
        <label>
          <input type="checkbox" bind:checked={embed} />
          embed into video file (otherwise saved alongside as .vtt)
        </label>
      </div>

      <div class="foot">
        <button class="danger" onclick={clear}>no subtitles</button>
        <div class="spacer"></div>
        <button onclick={() => metadata.closeSubtitleModal()}>cancel</button>
        <button class="primary" onclick={apply}>apply</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .scrim {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.55);
    display: grid;
    place-items: center;
    z-index: 112;
  }
  .card {
    background: var(--surface);
    border: 1px solid var(--line);
    padding: 18px 22px;
    width: min(500px, 94vw);
    max-height: 80vh;
    display: flex;
    flex-direction: column;
  }
  .head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }
  .head h2 {
    margin: 0;
    font-size: 13px;
    color: var(--text-hi);
    text-transform: uppercase;
    letter-spacing: 0.18em;
  }
  .x { background: transparent; border: 0; color: var(--dim); font-size: 18px; line-height: 1; cursor: pointer; }

  .mode {
    display: flex;
    flex-direction: column;
    gap: 4px;
    font-size: 12px;
    color: var(--text);
    padding-bottom: 10px;
    border-bottom: 1px solid var(--line-soft);
  }
  .mode label, .embed label {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
  }
  .hint { color: var(--dim); font-size: 11px; margin: 10px 0 4px; }
  .empty { color: var(--dim); padding: 24px 0; text-align: center; font-size: 12px; }

  .langs {
    list-style: none;
    padding: 0;
    margin: 0;
    overflow-y: auto;
    flex: 1;
    max-height: 320px;
    border: 1px solid var(--line-soft);
  }
  .langs li { border-bottom: 1px solid var(--line-soft); }
  .langs li:last-child { border-bottom: 0; }
  .langs label {
    display: grid;
    grid-template-columns: 22px 60px 1fr;
    gap: 10px;
    align-items: center;
    padding: 6px 12px;
    cursor: pointer;
    font-size: 12px;
  }
  .langs label.disabled { opacity: 0.4; cursor: not-allowed; }
  .langs label:hover:not(.disabled) { background: var(--surface-2); }
  .code {
    font-family: var(--mono, ui-monospace, monospace);
    color: var(--text-hi);
  }
  .tags { display: flex; gap: 4px; }
  .tag {
    font-size: 9px;
    text-transform: uppercase;
    letter-spacing: 0.14em;
    padding: 1px 5px;
    border: 1px solid var(--line);
    color: var(--dim);
  }
  .tag.manual { color: var(--olive); border-color: var(--olive); }
  .tag.auto   { color: var(--amber); border-color: var(--amber-soft); }

  .embed {
    margin-top: 10px;
    padding-top: 10px;
    border-top: 1px solid var(--line-soft);
    font-size: 12px;
    color: var(--text);
  }

  .foot {
    display: flex;
    gap: 8px;
    margin-top: 14px;
    align-items: center;
  }
  .spacer { flex: 1; }
  .foot button {
    padding: 5px 14px;
    font-size: 11px;
    background: transparent;
    border: 1px solid var(--line);
    color: var(--text);
  }
  .foot .primary {
    background: var(--amber);
    color: var(--bg);
    border-color: var(--amber);
  }
  .foot .danger { color: var(--rose); border-color: var(--rose); }
</style>
