<script lang="ts">
  import { errors } from '$lib/state/errors.svelte';
  import { ui } from '$lib/state/ui.svelte';
</script>

<div class="stack" role="region" aria-label="Notifications">
  {#each errors.items as e (e.id)}
    <div class="toast kind-{e.kind}" role="alert">
      <div class="body">
        <div class="row">
          <span class="tag">{e.kind.replace('_', ' ')}</span>
          <strong>{e.title}</strong>
          <button class="x" onclick={() => errors.dismiss(e.id)} aria-label="Dismiss">×</button>
        </div>
        <div class="msg">{e.message}</div>
        <div class="sug">{e.suggestion}</div>
        {#if e.kind === 'auth_required'}
          <div class="actions">
            <button onclick={() => { ui.setView('cookies'); errors.dismiss(e.id); }}>
              → open cookies view
            </button>
          </div>
        {/if}
      </div>
    </div>
  {/each}
</div>

<style>
  .stack {
    position: fixed;
    bottom: 18px;
    right: 18px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    z-index: 130;
    max-width: min(420px, 90vw);
  }
  .toast {
    background: var(--surface);
    border: 1px solid var(--line);
    border-left: 3px solid var(--dim);
    padding: 10px 14px;
    box-shadow: 0 4px 18px rgba(0,0,0,0.35);
    font-size: 12px;
    color: var(--text);
  }
  .toast.kind-geo_blocked  { border-left-color: var(--rose); }
  .toast.kind-auth_required{ border-left-color: var(--amber); }
  .toast.kind-not_found    { border-left-color: var(--rose); }
  .toast.kind-network      { border-left-color: var(--amber); }
  .toast.kind-io           { border-left-color: var(--rose); }
  .toast.kind-parse        { border-left-color: var(--amber); }
  .toast.kind-shell        { border-left-color: var(--rose); }

  .row {
    display: flex;
    align-items: baseline;
    gap: 8px;
  }
  .tag {
    font-size: 9.5px;
    color: var(--dim);
    text-transform: uppercase;
    letter-spacing: 0.14em;
    border: 1px solid var(--line);
    padding: 1px 5px;
  }
  strong {
    color: var(--text-hi);
    font-weight: 500;
  }
  .x {
    margin-left: auto;
    background: transparent;
    border: 0;
    color: var(--dim);
    font-size: 14px;
    cursor: pointer;
    line-height: 1;
  }
  .msg {
    margin-top: 4px;
    color: var(--text);
    word-break: break-word;
  }
  .sug {
    margin-top: 5px;
    color: var(--dim);
    font-size: 11px;
    line-height: 1.45;
  }
  .actions {
    margin-top: 8px;
  }
  .actions button {
    font-size: 10.5px;
    padding: 3px 9px;
    background: transparent;
    border: 1px solid var(--amber-soft);
    color: var(--amber);
  }
  .actions button:hover {
    background: var(--surface-2);
  }
</style>
