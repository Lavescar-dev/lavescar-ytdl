<script lang="ts">
  import { cheatsheetVisible, listShortcuts } from '$lib/state/shortcuts.svelte';

  function close() { cheatsheetVisible.value = false; }

  function fmt(combo: string): string {
    const isMac = typeof navigator !== 'undefined' && /Mac|iPod|iPhone|iPad/.test(navigator.platform);
    return combo
      .split('+')
      .map((k) => (k === 'mod' ? (isMac ? '⌘' : 'Ctrl') : k.toUpperCase()))
      .join(' + ');
  }

  function onWindowKey(e: KeyboardEvent) {
    if (cheatsheetVisible.value && e.key === 'Escape') close();
  }
</script>

<svelte:window onkeydown={onWindowKey} />

{#if cheatsheetVisible.value}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="overlay" onclick={close} role="presentation">
    <div
      class="card"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      aria-labelledby="sc-title"
      tabindex="-1"
    >
      <h3 id="sc-title">keyboard shortcuts</h3>
      <ul>
        {#each listShortcuts() as s}
          <li>
            <span class="key">{fmt(s.combo)}</span>
            <span class="lbl">{s.label}</span>
          </li>
        {/each}
      </ul>
      <div class="foot">press <span class="key">?</span> anytime · <span class="key">ESC</span> to close</div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.6);
    display: grid;
    place-items: center;
    z-index: 120;
  }
  .card {
    background: var(--surface);
    border: 1px solid var(--line);
    padding: 22px 26px;
    width: min(420px, 92vw);
  }
  h3 {
    margin: 0 0 16px;
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.22em;
    color: var(--text-hi);
  }
  ul { list-style: none; padding: 0; margin: 0; }
  li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 6px 0;
    border-bottom: 1px solid var(--line);
    font-size: 12px;
    color: var(--text);
  }
  .key {
    font-family: var(--mono, ui-monospace, monospace);
    font-size: 10.5px;
    color: var(--amber);
    border: 1px solid var(--line);
    padding: 2px 6px;
    min-width: 42px;
    text-align: center;
  }
  .lbl { color: var(--text); }
  .foot {
    margin-top: 14px;
    padding-top: 12px;
    border-top: 1px solid var(--line);
    font-size: 10.5px;
    color: var(--dim);
    display: flex;
    gap: 10px;
    align-items: center;
  }
</style>
