<script lang="ts">
  import { metadata } from '$lib/state/metadata.svelte';
  import { i18n } from '$lib/i18n/index.svelte';

  const t = $derived(i18n.t);
</script>

<div class="panel">
  <div class="panel-h">
    <span class="dot"></span>
    <span class="title-text">{t.metadata.title}</span>
    <span class="meta">
      {#if metadata.state === 'empty'}{t.metadata.awaitingInput}
      {:else if metadata.state === 'loading'}{t.metadata.fetching}
      {:else if metadata.state === 'ready' && metadata.current}{t.metadata.ready} · {metadata.current.duration}
      {:else if metadata.state === 'error'}{t.metadata.error}
      {/if}
    </span>
  </div>

  <div class="meta-body">
    {#if metadata.state === 'empty' || metadata.state === 'loading'}
      <div class="meta-empty">
        {#if metadata.state === 'loading'}
          {t.metadata.fetchingFrom}
        {:else}
          {t.metadata.pasteHint}
        {/if}
        <span class="glyph">◯ ─ ─ ─ ─ ─ ─ ─ ─ ─ ◯</span>
      </div>
    {:else if metadata.state === 'error'}
      <div class="meta-empty">
        <span style="color: var(--rose)">{metadata.error}</span>
      </div>
    {:else if metadata.current}
      {@const m = metadata.current}
      <div class="meta-card">
        <div class="meta-title">{m.title}</div>
        <div class="meta-channel">{m.uploader}</div>
        <div class="meta-grid">
          <span class="k">{t.metadata.duration}</span><span class="v">{m.duration}</span>
          <span class="k">{t.metadata.bestV}</span><span class="v"><span class="hi">{m.bestVideo}</span></span>
          <span class="k">{t.metadata.bestA}</span><span class="v"><span class="hi">{m.bestAudio}</span></span>
          <span class="k">{t.metadata.sizeEst}</span><span class="v">{m.sizeEstimate}</span>
          <span class="k">{t.metadata.subs}</span><span class="v">{m.subtitles}</span>
          <span class="k">{t.metadata.chapters}</span><span class="v">{m.chapters}</span>
        </div>

        <div class="format-pick">
          {#each metadata.formatOptions as fmt}
            <button
              class="fmt-chip"
              class:on={fmt.id === metadata.selectedFormatId}
              onclick={() => metadata.selectFormat(fmt.id)}
            >
              {fmt.label}
            </button>
          {/each}
        </div>

        <div class="sub-config">
          <button class="sub-btn" onclick={() => metadata.openSubtitleModal()}>
            {t.metadata.configureSubtitles}
          </button>
          {#if metadata.subtitleOpts}
            <span class="sub-summary">
              {metadata.subtitleOpts.langs.join(', ')}
              · {metadata.subtitleOpts.auto ? t.subtitle.badgeAuto : t.subtitle.badgeManual}
              · {metadata.subtitleOpts.embed ? 'embed' : 'separate'}
            </span>
          {:else}
            <span class="sub-summary dim">{t.metadata.noSubtitlesSelected}</span>
          {/if}
        </div>
      </div>
    {/if}
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

  .meta-empty {
    padding: 36px 18px;
    text-align: center;
    color: var(--dim);
    font-family: var(--serif);
    font-style: italic;
    font-size: 13px;
  }
  .meta-empty .glyph {
    display: block;
    font-family: var(--mono);
    font-style: normal;
    color: var(--amber-soft);
    font-size: 11px;
    margin-top: 8px;
    letter-spacing: 0.2em;
  }

  .meta-card { padding: 16px 18px; }

  .meta-title {
    color: var(--text-hi);
    font-size: 14px;
    font-weight: 500;
    margin-bottom: 4px;
    line-height: 1.3;
  }
  .meta-channel {
    color: var(--amber);
    font-size: 11px;
    margin-bottom: 14px;
  }

  .meta-grid {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 6px 14px;
    font-size: 11.5px;
  }
  .meta-grid .k {
    color: var(--dim);
    text-transform: uppercase;
    letter-spacing: 0.12em;
    font-size: 10px;
    padding-top: 2px;
  }
  .meta-grid .v { color: var(--text); }
  .meta-grid .v .hi { color: var(--amber); }

  .format-pick {
    margin-top: 16px;
    padding-top: 14px;
    border-top: 1px dashed var(--line);
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }
  .fmt-chip {
    padding: 4px 10px;
    border: 1px solid var(--line);
    background: var(--bg);
    font-size: 10.5px;
    color: var(--dim);
    cursor: pointer;
    transition: all 0.12s ease;
    letter-spacing: 0.05em;
  }
  .fmt-chip:hover {
    color: var(--text-hi);
    border-color: var(--dim);
  }
  .fmt-chip.on {
    color: var(--amber);
    border-color: var(--amber);
    background: rgba(212, 160, 23, 0.06);
  }

  .sub-config {
    margin-top: 12px;
    padding-top: 10px;
    border-top: 1px dashed var(--line-soft);
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
  }
  .sub-btn {
    font-size: 10.5px;
    color: var(--dim);
    background: transparent;
    border: 1px solid var(--line);
    padding: 3px 9px;
    cursor: pointer;
  }
  .sub-btn:hover {
    color: var(--amber);
    border-color: var(--amber-soft);
  }
  .sub-summary {
    font-size: 10.5px;
    color: var(--text);
    letter-spacing: 0.04em;
  }
  .sub-summary.dim { color: var(--dim); }
</style>
