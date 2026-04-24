<script lang="ts">
  import { ui } from '$lib/state/ui.svelte';
  import { i18n } from '$lib/i18n/index.svelte';
  import { cheatsheetVisible } from '$lib/state/shortcuts.svelte';

  const t = $derived(i18n.t);
</script>

<header class="topbar">
  <div class="brand">
    <span class="logo">L</span>
    <span class="name">lavescar</span>
    <span class="sep">▸</span>
    <span class="mode">yt-dlp</span>
    <span class="ver">v1.0.0</span>
  </div>
  <div class="spacer"></div>

  <div class="lang" role="group" aria-label={t.settings.language}>
    <button
      class="lang-btn"
      class:on={i18n.locale === 'en'}
      aria-pressed={i18n.locale === 'en'}
      onclick={() => i18n.set('en')}
    >{t.langSwitch.en}</button>
    <button
      class="lang-btn"
      class:on={i18n.locale === 'tr'}
      aria-pressed={i18n.locale === 'tr'}
      onclick={() => i18n.set('tr')}
    >{t.langSwitch.tr}</button>
  </div>

  <div class="top-actions">
    <button class="top-btn" title={t.topbar.pauseAll}>{t.topbar.pauseAll}</button>
    <button class="top-btn" title={t.topbar.settings} onclick={() => ui.openSettings()}>{t.topbar.settings}</button>
    <button class="top-btn" title={t.topbar.logs}>{t.topbar.logs}</button>
    <button class="top-btn kbd" onclick={() => (cheatsheetVisible.value = true)} aria-label={t.shortcuts.title}>?</button>
  </div>
</header>

<style>
  .topbar {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 10px 18px;
    border-bottom: 1px solid var(--line);
    background: linear-gradient(180deg, var(--bg-soft), var(--bg));
  }

  .brand {
    display: flex;
    align-items: baseline;
    gap: 10px;
    font-weight: 500;
    letter-spacing: 0.02em;
  }
  .brand .logo {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    background: var(--amber);
    color: var(--bg);
    font-weight: 700;
    font-size: 12px;
    transform: skew(-8deg);
  }
  .brand .name { color: var(--text-hi); }
  .brand .sep  { color: var(--dim); }
  .brand .mode { color: var(--amber); font-weight: 400; }
  .brand .ver {
    color: var(--dim);
    font-size: 11px;
    font-style: italic;
    font-family: var(--serif);
    margin-left: 6px;
  }

  .spacer { flex: 1; }

  .lang {
    display: flex;
    border: 1px solid var(--line);
    overflow: hidden;
    margin-right: 4px;
  }
  .lang-btn {
    padding: 3px 9px;
    background: transparent;
    border: 0;
    color: var(--dim);
    font-size: 10px;
    letter-spacing: 0.16em;
    font-family: inherit;
    cursor: pointer;
  }
  .lang-btn.on { background: var(--amber); color: var(--bg); }
  .lang-btn:not(.on):hover { color: var(--text-hi); }
  .lang-btn + .lang-btn { border-left: 1px solid var(--line); }

  .top-actions { display: flex; align-items: center; gap: 4px; }
  .top-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 5px 10px;
    color: var(--dim);
    border: 1px solid transparent;
    transition: all 0.12s ease;
    font-size: 12px;
    background: transparent;
    cursor: pointer;
  }
  .top-btn:hover {
    color: var(--text-hi);
    border-color: var(--line);
    background: var(--surface);
  }
  .top-btn.kbd {
    color: var(--dim);
    font-size: 10px;
    border: 1px solid var(--line);
    padding: 2px 7px;
    border-radius: 2px;
    background: var(--surface);
  }
</style>
