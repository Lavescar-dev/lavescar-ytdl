<script lang="ts">
  import { presets } from '$lib/state/presets.svelte';
  import { ui } from '$lib/state/ui.svelte';
  import { i18n } from '$lib/i18n/index.svelte';

  const t = $derived(i18n.t);
</script>

<div class="panel">
  <div class="panel-h">
    <span class="dot"></span>
    <span class="title-text">{t.preset.title}</span>
    <span class="meta">{t.preset.hint}</span>
  </div>

  <div class="tabs" role="tablist" aria-label={t.preset.title}>
    <button
      class="tab"
      class:on={presets.activeCategory === 'video'}
      role="tab"
      aria-selected={presets.activeCategory === 'video'}
      onclick={() => presets.setCategory('video')}
    >
      {t.preset.video}
    </button>
    <button
      class="tab"
      class:on={presets.activeCategory === 'audio'}
      role="tab"
      aria-selected={presets.activeCategory === 'audio'}
      onclick={() => presets.setCategory('audio')}
    >
      {t.preset.audio}
    </button>
  </div>

  <div class="preset-list">
    {#each presets.visible as p (p.id)}
      {@const on = p.id === presets.selectedId}
      <div
        class="preset-row"
        class:on
        onclick={() => presets.select(p.id)}
        role="button"
        tabindex="0"
        onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && presets.select(p.id)}
      >
        <span class="pmark">{on ? '▸' : '·'}</span>
        <div class="info">
          <div class="preset-name">{p.name}</div>
          <div class="preset-spec">{p.spec}</div>
          {#if p.flags && p.flags.length > 0}
            <div class="preset-flags">{p.flags.join(' ')}</div>
          {/if}
        </div>
        <span class="preset-meta">
          {p.isDefault ? t.preset.default : (p.hotkey ?? '')}
        </span>
      </div>
    {/each}

    {#if presets.visible.length === 0}
      <div class="empty">{t.preset.empty}</div>
    {/if}

    <button class="preset-add" onclick={() => ui.setView('presets')}>
      {t.preset.manage}
    </button>
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

  .tabs {
    display: flex;
    border-bottom: 1px solid var(--line);
    background: var(--bg-soft);
  }
  .tab {
    flex: 1;
    padding: 8px 14px;
    background: transparent;
    border: 0;
    color: var(--dim);
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.16em;
    cursor: pointer;
    transition: all 0.12s ease;
    border-bottom: 2px solid transparent;
  }
  .tab:hover { color: var(--text); }
  .tab.on {
    color: var(--amber);
    border-bottom-color: var(--amber);
    background: var(--surface);
  }
  .tab + .tab { border-left: 1px solid var(--line); }

  .preset-list { padding: 6px 0; }

  .preset-row {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    padding: 9px 18px;
    cursor: pointer;
    border-left: 2px solid transparent;
    transition: all 0.1s ease;
  }
  .preset-row:hover { background: var(--surface-2); }
  .preset-row.on {
    background: var(--surface-2);
    border-left-color: var(--amber);
  }
  .preset-row .pmark {
    color: var(--dim);
    font-family: var(--mono);
    width: 12px;
    padding-top: 1px;
  }
  .preset-row.on .pmark { color: var(--amber); }

  .info { flex: 1; min-width: 0; }
  .preset-name {
    color: var(--text-hi);
    font-size: 12px;
    font-weight: 500;
  }
  .preset-spec {
    color: var(--dim);
    font-size: 10.5px;
    margin-top: 1px;
    font-family: var(--mono);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .preset-flags {
    color: var(--olive);
    font-size: 10px;
    margin-top: 2px;
    font-family: var(--mono);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .preset-meta {
    color: var(--dim);
    font-size: 10px;
    padding-top: 1px;
    white-space: nowrap;
  }

  .empty {
    padding: 24px 18px;
    color: var(--dim);
    font-size: 11px;
    text-align: center;
    font-style: italic;
  }

  .preset-add {
    width: 100%;
    padding: 9px 18px;
    color: var(--amber);
    font-size: 11px;
    background: transparent;
    border: 0;
    border-top: 1px dashed var(--line);
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    text-align: left;
  }
  .preset-add:hover { background: var(--surface-2); }
</style>
