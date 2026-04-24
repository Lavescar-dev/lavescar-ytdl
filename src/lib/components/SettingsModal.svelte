<script lang="ts">
  import { ui } from '$lib/state/ui.svelte';
  import { runtime } from '$lib/state/runtime.svelte';
  import { presets } from '$lib/state/presets.svelte';
  import {
    pickDirectory,
    detectTooling,
    setSetting,
    setConcurrentLimit,
    updateYtdlp,
    onYtdlpUpdateProgress,
    isTauri,
    type YtdlpUpdateProgress
  } from '$lib/api/tauri';
  import { onMount } from 'svelte';
  import { i18n } from '$lib/i18n/index.svelte';

  const t = $derived(i18n.t);

  let outputDir = $state(runtime.info?.outputDir ?? '~/dl/yt');
  let cookiesSource = $state(runtime.info?.cookiesSource ?? 'firefox');
  let defaultPresetId = $state(presets.items.find((p) => p.isDefault)?.id ?? '');
  let ffmpegDetected = $state<string | null>(null);
  let saving = $state(false);
  let saveError = $state<string | null>(null);

  let ytdlpUpdating = $state(false);
  let ytdlpProgress = $state<YtdlpUpdateProgress | null>(null);
  let ytdlpError = $state<string | null>(null);
  let ytdlpUpdated = $state<string | null>(null);

  onMount(() => {
    if (!isTauri) return;
    let unsub: (() => void) | null = null;
    onYtdlpUpdateProgress((p) => (ytdlpProgress = p)).then((fn) => (unsub = fn));
    return () => unsub?.();
  });

  function fmtBytes(b: number): string {
    if (b < 1024) return `${b} B`;
    if (b < 1_048_576) return `${(b / 1024).toFixed(1)} KB`;
    return `${(b / 1_048_576).toFixed(1)} MB`;
  }

  async function doUpdateYtdlp() {
    ytdlpUpdating = true;
    ytdlpError = null;
    ytdlpUpdated = null;
    ytdlpProgress = { phase: 'resolving', bytes: 0, total: null };
    try {
      const r = await updateYtdlp();
      ytdlpUpdated = r.newVersion;
      await runtime.load().catch(() => {});
    } catch (e) {
      ytdlpError = e instanceof Error ? e.message : String(e);
    } finally {
      ytdlpUpdating = false;
    }
  }

  $effect(() => {
    if (ui.settingsOpen) {
      outputDir = runtime.info?.outputDir ?? outputDir;
      cookiesSource = runtime.info?.cookiesSource ?? cookiesSource;
      defaultPresetId = presets.items.find((p) => p.isDefault)?.id ?? defaultPresetId;
      if (isTauri) detectTooling().then((t) => (ffmpegDetected = t.ffmpeg)).catch(() => {});
    }
  });

  async function browse() {
    const picked = await pickDirectory();
    if (picked) outputDir = picked;
  }

  async function save() {
    saving = true;
    saveError = null;
    try {
      if (isTauri) {
        await setSetting('output_dir', outputDir);
        await setSetting('cookies_source', cookiesSource);
        if (defaultPresetId) await setSetting('default_preset_id', defaultPresetId);
        // Propagate concurrent limit to the live DownloadManager semaphore.
        await setConcurrentLimit(ui.concurrentLimit);
      }
      await ui.persist();
      await runtime.load().catch(() => {});
      ui.closeSettings();
    } catch (e) {
      saveError = e instanceof Error ? e.message : String(e);
    } finally {
      saving = false;
    }
  }

  function onKey(e: KeyboardEvent) {
    if (e.key === 'Escape') ui.closeSettings();
    if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 's') {
      e.preventDefault();
      save();
    }
  }
</script>

<svelte:window onkeydown={onKey} />

{#if ui.settingsOpen}
  <div class="scrim" onclick={() => ui.closeSettings()} role="button" tabindex="-1" onkeydown={() => {}}>
    <div class="card" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true" tabindex="-1" onkeydown={() => {}}>
      <div class="head">
        <h2>{t.settings.title}</h2>
        <span class="hint">{t.settings.saveHint}</span>
      </div>

      <div class="field">
        <label for="s-lang">{t.settings.language}</label>
        <div class="lang-row" role="group" aria-label={t.settings.language}>
          <button
            class="lang-pick"
            class:on={i18n.locale === 'en'}
            onclick={() => i18n.set('en')}
          >{t.settings.languageEn}</button>
          <button
            class="lang-pick"
            class:on={i18n.locale === 'tr'}
            onclick={() => i18n.set('tr')}
          >{t.settings.languageTr}</button>
        </div>
      </div>

      <div class="field">
        <label for="s-out">{t.settings.outputDir}</label>
        <div class="row">
          <input id="s-out" bind:value={outputDir} />
          <button onclick={browse}>{t.settings.browse}</button>
        </div>
      </div>

      <div class="field">
        <label for="s-cl">{t.settings.concurrent} · {ui.concurrentLimit}</label>
        <input
          id="s-cl"
          type="range"
          min="1"
          max="10"
          step="1"
          bind:value={ui.concurrentLimit}
        />
        <div class="sub-hint">{t.settings.concurrentHint}</div>
      </div>

      <div class="field">
        <label>
          <input type="checkbox" bind:checked={ui.throttleEnabled} />
          {t.settings.throttle}
        </label>
        {#if ui.throttleEnabled}
          <div class="sub">
            <span>{ui.throttleMbps} MB/s</span>
            <input
              type="range"
              min="1"
              max="100"
              step="1"
              bind:value={ui.throttleMbps}
            />
          </div>
        {/if}
        <div class="sub-hint">{t.settings.throttleApplyHint}</div>
      </div>

      <div class="field">
        <label for="s-pr">{t.settings.defaultPreset}</label>
        <select id="s-pr" bind:value={defaultPresetId}>
          {#each presets.items as p (p.id)}
            <option value={p.id}>{p.name}</option>
          {/each}
        </select>
      </div>

      <div class="field">
        <label for="s-ck">{t.settings.cookieSource}</label>
        <select id="s-ck" bind:value={cookiesSource}>
          <option value="firefox">Firefox</option>
          <option value="chromium">Chromium</option>
          <option value="brave">Brave</option>
          <option value="none">{t.settings.cookieNone}</option>
        </select>
      </div>

      <div class="field">
        <label>
          <input type="checkbox" bind:checked={ui.clipboardListening} />
          {t.settings.watchClipboard}
        </label>
      </div>

      <div class="tooling">
        <div><span>ffmpeg</span> <span class="val">{ffmpegDetected ?? runtime.info?.ffmpegVersion ?? '—'}</span></div>
        <div><span>yt-dlp</span> <span class="val">{runtime.info?.ytDlpVersion ?? '—'}</span></div>
        <div><span>aria2c</span> <span class="val">{runtime.info?.aria2cVersion ?? '—'}</span></div>
      </div>

      <div class="ytdlp-update">
        <div class="row">
          <button onclick={doUpdateYtdlp} disabled={ytdlpUpdating}>
            {ytdlpUpdating ? t.settings.updateYtdlpUpdating : t.settings.updateYtdlp}
          </button>
          {#if ytdlpUpdated}
            <span class="ok-note">{t.settings.updateYtdlpUpdatedTo} {ytdlpUpdated}</span>
          {/if}
        </div>
        {#if ytdlpUpdating && ytdlpProgress}
          <div class="upd-status">
            {t.settings.updatePhase[ytdlpProgress.phase]}
            {#if ytdlpProgress.phase === 'downloading'}
              · {fmtBytes(ytdlpProgress.bytes)}{ytdlpProgress.total ? ` / ${fmtBytes(ytdlpProgress.total)}` : ''}
            {/if}
          </div>
          {#if ytdlpProgress.total && ytdlpProgress.total > 0}
            <div class="upd-bar-wrap">
              <div
                class="upd-bar"
                style="width: {Math.min(100, (ytdlpProgress.bytes / ytdlpProgress.total) * 100)}%"
              ></div>
            </div>
          {/if}
        {/if}
        {#if ytdlpError}
          <div class="err">{ytdlpError}</div>
        {/if}
      </div>

      {#if saveError}<div class="err">{saveError}</div>{/if}

      <div class="foot">
        <button onclick={() => ui.closeSettings()}>{t.settings.cancel}</button>
        <button class="primary" onclick={save} disabled={saving}>
          {saving ? t.settings.saving : t.settings.save}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .scrim {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.55);
    display: grid;
    place-items: center;
    z-index: 110;
  }
  .card {
    background: var(--surface);
    border: 1px solid var(--line);
    padding: 20px 26px;
    width: min(520px, 94vw);
    max-height: 90vh;
    overflow-y: auto;
  }
  .head {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 14px;
  }
  .head h2 {
    margin: 0;
    font-size: 13px;
    color: var(--text-hi);
    text-transform: uppercase;
    letter-spacing: 0.22em;
  }
  .hint { color: var(--dim); font-size: 10px; letter-spacing: 0.14em; }

  .field { margin: 12px 0; }
  .field label {
    display: block;
    color: var(--dim);
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.16em;
    margin-bottom: 5px;
  }
  .field input:not([type]), .field select {
    width: 100%;
    padding: 7px 10px;
    background: var(--bg);
    border: 1px solid var(--line);
    color: var(--text-hi);
    font-size: 12px;
  }
  .field input[type="range"] { width: 100%; accent-color: var(--amber); }
  .field select { appearance: none; }

  .row { display: flex; gap: 8px; }
  .row input { flex: 1; }
  .row button {
    padding: 0 14px;
    border: 1px solid var(--line);
    color: var(--text);
    background: transparent;
    font-size: 11px;
  }
  .row button:hover { color: var(--amber); border-color: var(--amber-soft); }

  .sub {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-top: 6px;
    font-size: 11px;
    color: var(--text);
  }
  .sub input[type="range"] { flex: 1; }
  .sub-hint {
    font-size: 10px;
    color: var(--dim);
    margin-top: 4px;
  }

  .lang-row {
    display: flex;
    gap: 6px;
  }
  .lang-pick {
    padding: 5px 12px;
    background: transparent;
    border: 1px solid var(--line);
    color: var(--dim);
    font-family: inherit;
    font-size: 12px;
    cursor: pointer;
  }
  .lang-pick.on {
    background: var(--amber);
    color: var(--bg);
    border-color: var(--amber);
  }
  .lang-pick:not(.on):hover { color: var(--text-hi); }

  .ytdlp-update {
    margin-top: 12px;
    padding-top: 12px;
    border-top: 1px dashed var(--line);
  }
  .ytdlp-update .row {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  .ytdlp-update button {
    padding: 5px 12px;
    font-size: 11px;
    border: 1px solid var(--amber-soft);
    background: transparent;
    color: var(--amber);
  }
  .ytdlp-update button:hover:not(:disabled) {
    background: var(--surface-2);
  }
  .ytdlp-update button:disabled { opacity: 0.6; cursor: wait; }
  .ytdlp-update .ok-note {
    color: var(--olive);
    font-size: 11px;
  }
  .upd-status {
    margin-top: 8px;
    font-size: 10.5px;
    color: var(--dim);
    letter-spacing: 0.06em;
  }
  .upd-bar-wrap {
    margin-top: 5px;
    height: 3px;
    background: var(--bg);
    border: 1px solid var(--line-soft);
  }
  .upd-bar {
    height: 100%;
    background: var(--amber);
    transition: width 0.2s ease;
  }

  .tooling {
    margin-top: 18px;
    padding-top: 14px;
    border-top: 1px solid var(--line);
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
    font-size: 10.5px;
    color: var(--dim);
  }
  .tooling div { display: flex; justify-content: space-between; gap: 8px; }
  .tooling .val { color: var(--olive); }

  .err { color: var(--rose); font-size: 11px; margin-top: 8px; }

  .foot {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 18px;
  }
  .foot button {
    padding: 6px 16px;
    font-size: 11px;
    border: 1px solid var(--line);
    background: transparent;
    color: var(--text);
  }
  .foot .primary {
    background: var(--amber);
    color: var(--bg);
    border-color: var(--amber);
  }
  .foot .primary:disabled { opacity: 0.55; cursor: wait; }
</style>
