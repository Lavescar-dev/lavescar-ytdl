<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { isTauri } from '$lib/api/tauri';
  import { downloads } from '$lib/state/downloads.svelte';
  import { presets } from '$lib/state/presets.svelte';
  import { runtime } from '$lib/state/runtime.svelte';
  import { ui } from '$lib/state/ui.svelte';
  import { i18n } from '$lib/i18n/index.svelte';

  interface Props { children?: import('svelte').Snippet; }
  let { children }: Props = $props();

  onMount(() => {
    // i18n picks up navigator.language synchronously, then async-overrides
    // from the persisted DB setting if running under Tauri.
    i18n.init().catch(() => {});

    if (!isTauri) return;
    downloads.loadHistory().catch(console.error);
    presets.load().catch(console.error);
    runtime.load().catch(console.error);
    ui.load().catch(console.error);
  });
</script>

{@render children?.()}
