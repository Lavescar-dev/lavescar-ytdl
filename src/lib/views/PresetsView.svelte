<script lang="ts">
  import type { Preset } from '$lib/types';
  import { presets } from '$lib/state/presets.svelte';

  let editing = $state<Preset | null>(null);
  let saving = $state(false);
  let saveError = $state<string | null>(null);

  function blank(): Preset {
    return {
      id: crypto.randomUUID(),
      name: 'new preset',
      spec: 'bv+ba/b',
      flags: [],
      hotkey: '',
      category: 'video'
    };
  }

  function startNew() {
    editing = blank();
  }

  function startEdit(p: Preset) {
    editing = { ...p, flags: [...(p.flags ?? [])] };
  }

  function cancel() {
    editing = null;
    saveError = null;
  }

  async function save() {
    if (!editing) return;
    saving = true;
    saveError = null;
    try {
      await presets.upsert(editing);
      editing = null;
    } catch (e) {
      saveError = e instanceof Error ? e.message : String(e);
    } finally {
      saving = false;
    }
  }

  async function remove(id: string) {
    if (!confirm('delete this preset?')) return;
    await presets.remove(id);
  }

  function updateFlags(raw: string) {
    if (!editing) return;
    editing.flags = raw
      .split(/\s+/)
      .map((s) => s.trim())
      .filter(Boolean);
  }
</script>

<div class="view">
  <div class="head">
    <h2>presets</h2>
    <button class="new" onclick={startNew}>+ new preset</button>
  </div>

  <ul class="list">
    {#each presets.items as p (p.id)}
      <li class="row">
        <div class="info">
          <div class="name">
            {p.name}
            <span class="badge cat cat-{p.category}">{p.category}</span>
            {#if p.isDefault}<span class="badge">default</span>{/if}
            {#if p.hotkey}<span class="badge hk">{p.hotkey}</span>{/if}
          </div>
          <div class="spec">{p.spec}</div>
          {#if p.flags && p.flags.length > 0}
            <div class="flags">{p.flags.join(' ')}</div>
          {/if}
        </div>
        <div class="actions">
          <button class="edit" onclick={() => startEdit(p)}>edit</button>
          <button class="del" onclick={() => remove(p.id)}>delete</button>
        </div>
      </li>
    {/each}
  </ul>

  {#if editing}
    <div class="modal">
      <div class="modal-inner">
        <div class="modal-head">
          <h3>{presets.items.some((x) => x.id === editing!.id) ? 'edit' : 'new'} preset</h3>
          <button class="x" onclick={cancel}>×</button>
        </div>
        <div class="field">
          <label for="p-name">name</label>
          <input id="p-name" bind:value={editing.name} />
        </div>
        <div class="field">
          <label for="p-cat">category</label>
          <select id="p-cat" bind:value={editing.category}>
            <option value="video">video</option>
            <option value="audio">audio</option>
          </select>
        </div>
        <div class="field">
          <label for="p-spec">format spec (yt-dlp <code>-f</code>)</label>
          <textarea id="p-spec" rows="2" bind:value={editing.spec}></textarea>
        </div>
        <div class="field">
          <label for="p-flags">extra flags (space-separated)</label>
          <input
            id="p-flags"
            value={(editing.flags ?? []).join(' ')}
            oninput={(e) => updateFlags((e.target as HTMLInputElement).value)}
            placeholder="--embed-metadata --sponsorblock-remove sponsor"
          />
        </div>
        <div class="field">
          <label for="p-hk">hotkey</label>
          <input id="p-hk" bind:value={editing.hotkey} placeholder="⌘1 · optional" />
        </div>
        <div class="field inline">
          <label>
            <input type="checkbox" bind:checked={editing.isDefault} />
            default preset
          </label>
        </div>
        {#if saveError}<div class="err">{saveError}</div>{/if}
        <div class="modal-foot">
          <button onclick={cancel}>cancel</button>
          <button class="primary" onclick={save} disabled={saving}>
            {saving ? 'saving…' : 'save'}
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .view {
    border: 1px solid var(--line);
    background: var(--surface);
    padding: 16px 18px;
  }
  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 14px;
  }
  .head h2 {
    font-size: 13px;
    color: var(--text-hi);
    text-transform: uppercase;
    letter-spacing: 0.2em;
    margin: 0;
  }
  .new {
    font-size: 11px;
    color: var(--amber);
    padding: 4px 12px;
    border: 1px solid var(--amber-soft);
    background: transparent;
  }
  .new:hover { background: var(--surface-2); }

  .list { list-style: none; padding: 0; margin: 0; }
  .row {
    display: flex;
    gap: 18px;
    padding: 10px 2px;
    border-bottom: 1px solid var(--line);
    align-items: flex-start;
  }
  .info { flex: 1; }
  .name { font-size: 13px; color: var(--text-hi); display: flex; align-items: center; gap: 8px; }
  .spec {
    font-size: 11.5px;
    color: var(--text);
    font-family: var(--mono, ui-monospace, monospace);
    margin-top: 3px;
  }
  .flags {
    font-size: 10.5px;
    color: var(--dim);
    margin-top: 2px;
  }
  .badge {
    font-size: 9px;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    padding: 1px 6px;
    border: 1px solid var(--line);
    color: var(--dim);
  }
  .badge.hk { color: var(--amber); border-color: var(--amber-soft); }
  .badge.cat-video { color: var(--olive); border-color: var(--olive); }
  .badge.cat-audio { color: var(--amber); border-color: var(--amber-soft); }

  .actions { display: flex; gap: 8px; }
  .edit, .del {
    font-size: 10px;
    padding: 3px 10px;
    border: 1px solid var(--line);
    background: transparent;
    color: var(--dim);
  }
  .edit:hover { color: var(--amber); border-color: var(--amber-soft); }
  .del:hover  { color: var(--rose);  border-color: var(--rose); }

  .modal {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.55);
    display: grid;
    place-items: center;
    z-index: 100;
  }
  .modal-inner {
    background: var(--surface);
    border: 1px solid var(--line);
    padding: 18px 22px;
    width: min(520px, 92vw);
  }
  .modal-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 14px;
  }
  .modal-head h3 {
    margin: 0;
    font-size: 13px;
    color: var(--text-hi);
    text-transform: uppercase;
    letter-spacing: 0.18em;
  }
  .x {
    background: transparent;
    border: 0;
    color: var(--dim);
    font-size: 18px;
    line-height: 1;
    cursor: pointer;
  }
  .field { margin: 10px 0; }
  .field.inline { display: flex; align-items: center; }
  .field label {
    display: block;
    color: var(--dim);
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.16em;
    margin-bottom: 4px;
  }
  .field input, .field textarea {
    width: 100%;
    padding: 7px 10px;
    background: var(--bg);
    border: 1px solid var(--line);
    color: var(--text-hi);
    font-size: 12px;
    font-family: var(--mono, ui-monospace, monospace);
  }
  .field input:focus, .field textarea:focus {
    outline: 0;
    border-color: var(--amber);
  }
  .err { color: var(--rose); font-size: 11px; margin-top: 8px; }
  .modal-foot {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 16px;
  }
  .modal-foot button {
    font-size: 11px;
    padding: 6px 14px;
    border: 1px solid var(--line);
    background: transparent;
    color: var(--text);
  }
  .modal-foot .primary {
    background: var(--amber);
    color: var(--bg);
    border-color: var(--amber);
  }
  .modal-foot .primary:disabled { opacity: 0.55; cursor: wait; }
</style>
