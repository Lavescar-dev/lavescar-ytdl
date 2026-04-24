<script lang="ts">
  import { presets } from '$lib/state/presets.svelte';
</script>

<div class="panel">
  <div class="panel-h">
    <span class="dot"></span>
    <span class="title-text">preset</span>
    <span class="meta">⌘P to switch</span>
  </div>

  <div class="preset-list">
    {#each presets.items as p}
      {@const on = p.id === presets.selectedId}
      <div
        class="preset-row"
        class:on
        onclick={() => presets.select(p.id)}
        role="button"
        tabindex="0"
        onkeydown={(e) => e.key === 'Enter' && presets.select(p.id)}
      >
        <span class="pmark">{on ? '▸' : '·'}</span>
        <div>
          <div class="preset-name">{p.name}</div>
          <div class="preset-spec">{p.spec}</div>
        </div>
        <span class="preset-meta">
          {p.isDefault ? 'default' : (p.hotkey ?? '')}
        </span>
      </div>
    {/each}
    <div class="preset-add" role="button" tabindex="0">+ new preset</div>
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

  .preset-list { padding: 6px 0; }

  .preset-row {
    display: flex;
    align-items: center;
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
  }
  .preset-row.on .pmark { color: var(--amber); }

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
  }
  .preset-meta {
    margin-left: auto;
    color: var(--dim);
    font-size: 10px;
  }

  .preset-add {
    padding: 9px 18px;
    color: var(--amber);
    font-size: 11px;
    border-top: 1px dashed var(--line);
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
  }
  .preset-add:hover { background: var(--surface-2); }
</style>
