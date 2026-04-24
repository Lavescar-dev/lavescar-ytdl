<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { isTauri } from '$lib/api/tauri';
  import { downloads } from '$lib/state/downloads.svelte';
  import { presets } from '$lib/state/presets.svelte';
  import { runtime } from '$lib/state/runtime.svelte';

  interface Props { children?: import('svelte').Snippet; }
  let { children }: Props = $props();

  import { ui } from '$lib/state/ui.svelte';

  onMount(() => {
    if (!isTauri) return;
    downloads.loadHistory().catch(console.error);
    presets.load().catch(console.error);
    runtime.load().catch(console.error);
    ui.load().catch(console.error);
  });
</script>

{@render children?.()}
