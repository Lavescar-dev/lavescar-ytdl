<script lang="ts">
  import { onMount } from 'svelte';
  import TopBar    from '$lib/components/TopBar.svelte';
  import Sidebar   from '$lib/components/Sidebar.svelte';
  import StatusBar from '$lib/components/StatusBar.svelte';
  import ShortcutsOverlay from '$lib/components/ShortcutsOverlay.svelte';
  import SettingsModal from '$lib/components/SettingsModal.svelte';
  import PlaylistModal from '$lib/components/PlaylistModal.svelte';
  import SubtitleModal from '$lib/components/SubtitleModal.svelte';
  import ErrorToast from '$lib/components/ErrorToast.svelte';
  import UpdateBanner from '$lib/components/UpdateBanner.svelte';

  import DownloadView from '$lib/views/DownloadView.svelte';
  import QueueView    from '$lib/views/QueueView.svelte';
  import HistoryView  from '$lib/views/HistoryView.svelte';
  import PresetsView  from '$lib/views/PresetsView.svelte';
  import CookiesView  from '$lib/views/CookiesView.svelte';

  import { ui } from '$lib/state/ui.svelte';
  import { startShortcuts } from '$lib/state/shortcuts.svelte';

  onMount(() => startShortcuts());
</script>

<div class="app">
  <UpdateBanner />
  <TopBar />

  <main class="main">
    <Sidebar />

    <section class="content">
      {#if ui.activeView === 'download'}
        <DownloadView />
      {:else if ui.activeView === 'queue'}
        <QueueView />
      {:else if ui.activeView === 'history'}
        <HistoryView />
      {:else if ui.activeView === 'presets'}
        <PresetsView />
      {:else if ui.activeView === 'cookies'}
        <CookiesView />
      {/if}
    </section>
  </main>

  <StatusBar />
</div>

<ShortcutsOverlay />
<SettingsModal />
<PlaylistModal />
<SubtitleModal />
<ErrorToast />

<style>
  .app {
    display: grid;
    grid-template-rows: auto auto 1fr auto;
    min-height: 100vh;
    max-width: 1400px;
    margin: 0 auto;
    border-left: 1px solid var(--line-soft);
    border-right: 1px solid var(--line-soft);
  }

  .main {
    display: grid;
    grid-template-columns: 220px 1fr;
    min-height: 0;
  }

  .content {
    padding: 24px 28px 28px;
    display: flex;
    flex-direction: column;
    gap: 22px;
    min-width: 0;
  }
</style>
