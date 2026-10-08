<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let appName = "LBP Browser";
  let appVersion = "0.1.0";
  let loading = true;

  onMount(async () => {
    try {
      const info = await invoke<{ name: string; version: string }>("get_app_info");
      appName = info.name;
      appVersion = info.version;
    } catch {
      // Fall back if the Rust backend is not wired up yet.
    } finally {
      loading = false;
    }
  });

  let url = "";
  let statusText = "Ready";

  function navigate() {
    if (!url.trim()) return;
    statusText = `Navigating to ${url}`;
  }
</script>

<div class="app-shell">
  <!-- Top toolbar -->
  <header class="toolbar">
    <div class="brand">
      <span class="brand-mark">LBP</span>
      <span class="brand-ver">{appVersion}</span>
    </div>

    <div class="url-bar">
      <input
        type="text"
        placeholder="Search or enter URL"
        bind:value={url}
        onkeydown={(e) => {
          if (e.key === "Enter") {
            navigate();
          }
        }}
      />
    </div>

    <nav class="toolbar-actions">
      <button class="icon-btn" title="Back">←</button>
      <button class="icon-btn" title="Forward">→</button>
      <button class="icon-btn" title="Reload">↻</button>
      <span class="toolbar-spacer"></span>
      <button class="icon-btn" title="Bookmarks (future)" disabled>★</button>
      <button class="icon-btn" title="Downloads (future)" disabled>⬇</button>
      <button class="icon-btn" title="Ad blocker (F-43)" disabled>🛡</button>
      <button class="icon-btn" title="Volume booster (F-49)" disabled>🔊</button>
      <button class="icon-btn" title="Split screen (F-55)" disabled>▦</button>
      <button class="icon-btn" title="Workspaces (F-64)" disabled>⌂</button>
      <button class="icon-btn" title="Dev tools (D-01)" disabled>⚙</button>
    </nav>
  </header>

  <!-- Tab bar placeholder -->
  <section class="tab-bar" aria-label="Tabs">
    <!-- Phase 2: implement tab model + tab bar. -->
    <div class="tab-strip-empty">
      No tabs yet. Open a link or type a URL above.
    </div>
  </section>

  <!-- Main content: webview area -->
  <main class="content">
    {#if loading}
      <div class="loading">Loading LBP shell…</div>
    {:else}
      <div class="webview-placeholder">
        <!-- Phase 1: embed the main WebView here via Tauri webview window/embed. -->
        <div class="placeholder-card">
          <h2>LBP Browser shell</h2>
          <p>Frontend shell is loaded. The main WebView will be embedded in Phase 1.</p>
          <ul>
            <li><a href="/requirements">Local preview of requirements</a></li>
          </ul>
        </div>
      </div>
    {/if}
  </main>

  <!-- Status bar -->
  <footer class="statusbar">
    <span class="status-left">{statusText}</span>
    <span class="status-right">Win + Linux · v1 scope</span>
  </footer>
</div>

<style>
  .app-shell {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--bg);
    color: var(--text);
  }

  .toolbar {
    display: flex;
    align-items: center;
    gap: var(--spacing-md);
    padding: var(--spacing-sm) var(--spacing-lg);
    background: var(--surface);
    border-bottom: 1px solid var(--border);
  }

  .brand {
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
    font-weight: 600;
  }

  .brand-mark {
    background: var(--accent);
    color: white;
    padding: 2px 6px;
    border-radius: var(--radius);
    font-size: 12px;
    letter-spacing: 0.5px;
  }

  .brand-ver {
    color: var(--text-muted);
    font-size: 12px;
  }

  .url-bar input {
    flex: 1;
    max-width: 720px;
    padding: var(--spacing-sm) var(--spacing-md);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--bg);
    color: var(--text);
    font-size: 14px;
    outline: none;
  }

  .url-bar input:focus {
    border-color: var(--accent);
  }

  .toolbar-actions {
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
  }

  .icon-btn {
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--radius);
    padding: var(--spacing-sm) var(--spacing-md);
    cursor: pointer;
    font-size: 14px;
    color: var(--text);
  }

  .icon-btn:hover {
    background: var(--bg);
    border-color: var(--border);
  }

  .icon-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .toolbar-spacer {
    flex: 1;
  }

  .tab-bar {
    display: flex;
    align-items: stretch;
    background: var(--surface);
    border-bottom: 1px solid var(--border);
  }

  .tab-strip-empty {
    padding: var(--spacing-md) var(--spacing-lg);
    color: var(--text-muted);
    font-size: 13px;
  }

  .content {
    flex: 1;
    position: relative;
    overflow: hidden;
  }

  .loading {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
  }

  .webview-placeholder {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .placeholder-card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: var(--spacing-lg) var(--spacing-xl);
    max-width: 480px;
    text-align: center;
  }

  .placeholder-card h2 {
    margin: 0 0 var(--spacing-sm);
  }

  .placeholder-card p {
    margin: 0 0 var(--spacing-md);
    color: var(--text-muted);
  }

  .statusbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--spacing-xs) var(--spacing-lg);
    background: var(--surface);
    border-top: 1px solid var(--border);
    font-size: 12px;
    color: var(--text-muted);
  }
</style>
