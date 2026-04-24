<script lang="ts">
  import { onMount } from 'svelte';

  const isTauri =
    typeof window !== 'undefined' &&
    ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);

  type UpdateState =
    | { phase: 'idle' }
    | { phase: 'available'; version: string; body: string | undefined }
    | { phase: 'downloading'; received: number; total: number | null }
    | { phase: 'ready' }
    | { phase: 'dismissed' }
    | { phase: 'error'; message: string };

  let state = $state<UpdateState>({ phase: 'idle' });
  let updateHandle: any = null;

  onMount(() => {
    if (!isTauri) return;
    (async () => {
      try {
        const { check } = await import('@tauri-apps/plugin-updater');
        const u = await check();
        if (u) {
          updateHandle = u;
          state = { phase: 'available', version: u.version, body: u.body };
        }
      } catch (err) {
        // Silent: updater endpoint not reachable, no release published, or
        // the bundle was built without updater artifacts. Don't surface as
        // an error — this is the common case during local dev.
        console.debug('[updater] check skipped:', err);
      }
    })();
  });

  async function install() {
    if (!updateHandle) return;
    state = { phase: 'downloading', received: 0, total: null };
    try {
      let total: number | null = null;
      let received = 0;
      await updateHandle.downloadAndInstall((event: any) => {
        if (event.event === 'Started') {
          total = event.data?.contentLength ?? null;
          state = { phase: 'downloading', received: 0, total };
        } else if (event.event === 'Progress') {
          received += event.data?.chunkLength ?? 0;
          state = { phase: 'downloading', received, total };
        } else if (event.event === 'Finished') {
          state = { phase: 'ready' };
        }
      });
      // downloadAndInstall triggers an automatic relaunch on completion.
    } catch (err) {
      state = {
        phase: 'error',
        message: err instanceof Error ? err.message : String(err)
      };
    }
  }

  function dismiss() {
    state = { phase: 'dismissed' };
  }

  function fmtMb(b: number): string {
    return `${(b / 1_048_576).toFixed(1)} MB`;
  }

  // i18n
  import { i18n } from '$lib/i18n/index.svelte';
  const t = $derived(i18n.t);
</script>

{#if state.phase !== 'idle' && state.phase !== 'dismissed'}
  <div class="banner" role="status">
    {#if state.phase === 'available'}
      <div class="msg">
        <strong>{t.update.available(`v${state.version}`)}</strong>
        {#if state.body}<span class="dim"> · {state.body.slice(0, 80)}</span>{/if}
      </div>
      <div class="actions">
        <button class="primary" onclick={install}>{t.update.install}</button>
        <button class="secondary" onclick={dismiss}>{t.update.later}</button>
      </div>
    {:else if state.phase === 'downloading'}
      <div class="msg">
        <strong>{t.update.downloading}</strong>
        <span class="dim"> · {fmtMb(state.received)}{state.total ? ` / ${fmtMb(state.total)}` : ''}</span>
      </div>
    {:else if state.phase === 'ready'}
      <div class="msg">
        <strong>{t.update.installed}</strong>
        <span class="dim"> {t.update.restarting}</span>
      </div>
    {:else if state.phase === 'error'}
      <div class="msg err">
        <strong>{t.update.failed}</strong> {state.message}
      </div>
      <div class="actions">
        <button class="secondary" onclick={dismiss}>{t.update.dismiss}</button>
      </div>
    {/if}
  </div>
{/if}

<style>
  .banner {
    border: 1px solid var(--amber);
    background: var(--surface-2);
    padding: 10px 14px;
    display: flex;
    align-items: center;
    gap: 16px;
    font-size: 12px;
    color: var(--text);
  }
  .msg { flex: 1; }
  .msg.err strong { color: var(--rose); }
  .dim { color: var(--dim); }
  strong { color: var(--amber); font-weight: 500; }
  .actions { display: flex; gap: 8px; }
  .actions button {
    padding: 4px 12px;
    font-size: 11px;
    border: 1px solid var(--amber-soft);
    background: transparent;
    color: var(--amber);
  }
  .actions .secondary {
    border-color: var(--line);
    color: var(--dim);
  }
  .actions button:hover { background: var(--bg); }
</style>
