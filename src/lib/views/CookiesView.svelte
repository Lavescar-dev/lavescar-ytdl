<script lang="ts">
  import { runtime } from '$lib/state/runtime.svelte';
  import { i18n } from '$lib/i18n/index.svelte';

  const t = $derived(i18n.t);

  const sources = $derived([
    { id: 'firefox',  label: t.cookies.sourceFirefox },
    { id: 'chromium', label: t.cookies.sourceChromium },
    { id: 'brave',    label: t.cookies.sourceBrave },
    { id: 'custom',   label: t.cookies.sourceCustom }
  ]);

  let selected = $state<string>(runtime.info?.cookiesSource ?? 'firefox');
  let customPath = $state('');

  function test() {
    alert(t.cookies.test);
  }
</script>

<div class="view">
  <div class="head">
    <h2>{t.cookies.title}</h2>
  </div>

  <p class="intro">{t.cookies.intro}</p>

  <div class="sources">
    {#each sources as s}
      <label class="src" class:active={selected === s.id}>
        <input type="radio" name="cookie-src" value={s.id} bind:group={selected} />
        <span>{s.label}</span>
      </label>
    {/each}
  </div>

  {#if selected === 'custom'}
    <div class="custom">
      <input bind:value={customPath} placeholder={t.cookies.customPath} />
    </div>
  {/if}

  <div class="actions">
    <button onclick={test}>{t.cookies.test}</button>
  </div>

  <div class="hint">{t.cookies.futureNote}</div>
</div>

<style>
  .view {
    border: 1px solid var(--line);
    background: var(--surface);
    padding: 16px 18px;
  }
  .head h2 {
    font-size: 13px;
    color: var(--text-hi);
    text-transform: uppercase;
    letter-spacing: 0.2em;
    margin: 0 0 14px;
  }
  .intro {
    color: var(--text);
    font-size: 12px;
    line-height: 1.55;
    max-width: 640px;
  }
  .sources {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 8px;
    margin: 14px 0;
  }
  .src {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border: 1px solid var(--line);
    cursor: pointer;
    font-size: 12px;
    color: var(--text);
  }
  .src.active {
    border-color: var(--amber);
    color: var(--amber);
  }
  .src input { accent-color: var(--amber); }

  .custom { margin: 10px 0; }
  .custom input {
    width: 100%;
    padding: 8px 10px;
    background: var(--bg);
    border: 1px solid var(--line);
    color: var(--text-hi);
    font-size: 12px;
    font-family: var(--mono, ui-monospace, monospace);
  }

  .actions { margin-top: 10px; }
  .actions button {
    font-size: 11px;
    padding: 6px 14px;
    border: 1px solid var(--line);
    background: transparent;
    color: var(--text);
  }
  .actions button:hover {
    color: var(--amber);
    border-color: var(--amber-soft);
  }

  .hint {
    margin-top: 18px;
    padding-top: 14px;
    border-top: 1px solid var(--line);
    color: var(--dim);
    font-size: 11px;
    line-height: 1.6;
    max-width: 640px;
  }
</style>
