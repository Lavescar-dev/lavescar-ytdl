<script lang="ts">
  import { ui } from '$lib/state/ui.svelte';
  import { runtime } from '$lib/state/runtime.svelte';
  import { downloads } from '$lib/state/downloads.svelte';
  import { presets } from '$lib/state/presets.svelte';

  const navItems = [
    { id: 'download' as const, label: 'download', icon: '▸' },
    { id: 'queue'    as const, label: 'queue',    icon: '▣' },
    { id: 'history'  as const, label: 'history',  icon: '≡' },
    { id: 'presets'  as const, label: 'presets',  icon: '◈' },
    { id: 'cookies'  as const, label: 'cookies',  icon: '⌘' }
  ];

  function countFor(id: string): number | null {
    if (id === 'download') return downloads.active;
    if (id === 'queue')    return downloads.queued;
    if (id === 'history')
      return downloads.items.filter(
        (d) => d.status === 'done' || d.status === 'error' || d.status === 'cancelled'
      ).length;
    if (id === 'presets')  return presets.items.length;
    return null;
  }
</script>

<aside class="side">
  <nav class="nav" aria-label="Main workspace">
    <div role="tablist" class="tabs">
    <div class="nav-section-label">workspace</div>
    {#each navItems as item, i}
      {@const count = countFor(item.id)}
      <div
        class="nav-item"
        class:active={ui.activeView === item.id}
        role="tab"
        aria-selected={ui.activeView === item.id}
        aria-label={item.label}
        tabindex={ui.activeView === item.id ? 0 : -1}
        onclick={() => ui.setView(item.id)}
        onkeydown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            ui.setView(item.id);
          } else if (e.key === 'ArrowDown') {
            e.preventDefault();
            const next = navItems[(i + 1) % navItems.length];
            ui.setView(next.id);
          } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            const prev = navItems[(i - 1 + navItems.length) % navItems.length];
            ui.setView(prev.id);
          }
        }}
      >
        <span class="ico" aria-hidden="true">{item.icon}</span>
        {item.label}
        {#if count !== null}
          <span class="count" aria-label={`${count} items`}>{count}</span>
        {/if}
      </div>
    {/each}
    </div>
  </nav>

  {#if runtime.info}
    <div class="side-block">
      <div class="blk-label">runtime</div>
      <div class="dep-row">
        <span class="dep-name">yt-dlp</span>
        <span class="ok">✓ {runtime.info.ytDlpVersion}</span>
      </div>
      <div class="dep-row">
        <span class="dep-name">ffmpeg</span>
        <span class="ok">✓ {runtime.info.ffmpegVersion}</span>
      </div>
      <div class="dep-row">
        <span class="dep-name">aria2c</span>
        <span class="ok">✓ {runtime.info.aria2cVersion}</span>
      </div>
      <div class="dep-row">
        <span class="dep-name">cookies</span>
        {#if runtime.info.cookiesSource}
          <span class="warn">! {runtime.info.cookiesSource}</span>
        {:else}
          <span class="warn">! none</span>
        {/if}
      </div>
    </div>

    <div class="side-block">
      <div class="blk-label">disk</div>
      <div class="dep-row">
        <span class="dep-name">target</span>
        <span style="color: var(--text)">{runtime.info.outputDir}</span>
      </div>
      <div class="dep-row">
        <span class="dep-name">free</span>
        <span style="color: var(--text)">{runtime.info.diskFreeGb.toFixed(1)} GB</span>
      </div>
    </div>
  {/if}

</aside>

<style>
  .side {
    border-right: 1px solid var(--line);
    background: var(--bg-soft);
    display: flex;
    flex-direction: column;
  }

  .nav { padding: 14px 0 18px; }
  .nav-section-label {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.18em;
    color: var(--dim);
    padding: 0 18px 6px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 7px 18px;
    color: var(--text);
    cursor: pointer;
    border-left: 2px solid transparent;
    transition: all 0.1s ease;
    font-size: 12.5px;
  }
  .nav-item:hover {
    background: var(--surface);
    color: var(--text-hi);
  }
  .nav-item.active {
    background: var(--surface);
    color: var(--amber);
    border-left-color: var(--amber);
  }
  .nav-item .ico {
    color: var(--dim);
    width: 14px;
    display: inline-block;
    text-align: center;
  }
  .nav-item.active .ico { color: var(--amber); }
  .nav-item .count {
    margin-left: auto;
    font-size: 10px;
    color: var(--dim);
    background: var(--bg);
    padding: 1px 6px;
    border-radius: 2px;
    border: 1px solid var(--line);
  }

  .side-block {
    border-top: 1px solid var(--line);
    padding: 12px 18px;
    font-size: 11px;
  }
  .side-block .blk-label {
    color: var(--dim);
    text-transform: uppercase;
    letter-spacing: 0.16em;
    font-size: 10px;
    margin-bottom: 8px;
  }

  .dep-row {
    display: flex;
    justify-content: space-between;
    padding: 3px 0;
    color: var(--text);
  }
  .dep-row .ok   { color: var(--olive); }
  .dep-row .warn { color: var(--rose); }
  .dep-name { color: var(--dim); }
</style>
