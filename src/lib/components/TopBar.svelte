<script lang="ts">
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
  <div class="top-actions">
    <button class="top-btn" title="Pause all">⏸ pause all</button>
    <button class="top-btn" title="Settings">⚙ settings</button>
    <button class="top-btn" title="Logs">≡ logs</button>
    <span class="top-btn kbd">?</span>
  </div>
</header>

<style>
  .topbar {
    display: flex;
    align-items: center;
    gap: 18px;
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
    padding: 2px 5px;
    border-radius: 2px;
    background: var(--surface);
  }
</style>
