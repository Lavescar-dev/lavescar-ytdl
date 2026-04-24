<script lang="ts">
  import UrlBar            from '$lib/components/UrlBar.svelte';
  import MetadataPanel     from '$lib/components/MetadataPanel.svelte';
  import PresetPanel       from '$lib/components/PresetPanel.svelte';
  import ActiveDownloads   from '$lib/components/ActiveDownloads.svelte';
  import OrphanPartsBanner from '$lib/components/OrphanPartsBanner.svelte';
  import ClipboardPrompt   from '$lib/components/ClipboardPrompt.svelte';
</script>

<OrphanPartsBanner />
<ClipboardPrompt />

<UrlBar />

<div class="twin">
  <MetadataPanel />
  <PresetPanel />
</div>

<ActiveDownloads />

<style>
  .twin {
    display: grid;
    grid-template-columns: 1.4fr 1fr;
    gap: 18px;
  }
  @media (max-width: 920px) {
    .twin { grid-template-columns: 1fr; }
  }
</style>
