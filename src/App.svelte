<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let appName = "LBP Browser";
  let appVersion = "0.1.0";
  let loading = true;
  let url = "";
  let statusText = "Ready";

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

  async function navigate() {
    if (!url.trim()) return;
    try {
      await invoke("navigate", { url: url });
      statusText = `Navigating to ${url}`;
    } catch (err) {
      statusText = `Navigation failed: ${err}`;
    }
  }

  async function handleBack() {
    try {
      await invoke("go_back");
    } catch (err) {
      console.error("Back failed:", err);
    }
  }

  async function handleForward() {
    try {
      await invoke("go_forward");
    } catch (err) {
      console.error("Forward failed:", err);
    }
  }

  async function handleReload() {
    try {
      await invoke("reload");
      statusText = "Reloading...";
    } catch (err) {
      console.error("Reload failed:", err);
    }
  }

  function handleUrlKeydown(event: KeyboardEvent) {
    if (event.key === "Enter") {
      navigate();
    }
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
        value={url}
        oninput={(e) => url = (e.target as HTMLInputElement).value}
        onkeydown={handleUrlKeydown}
      />
    </div>

    <nav class="toolbar-actions">
      <button class="icon-btn" title="Back" onclick={handleBack}>←</button>
      <button class="icon-btn" title="Forward" onclick={handleForward}>→</button>
      <button class="icon-btn" title="Reload" onclick={handleReload}>↻</button>
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
      <!-- Content is rendered by Tauri's webview (main window) -->
      <div class="content-area">
        <!-- The actual web content is displayed by Tauri's webview -->
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

  .content-area {
    flex: 1;
    position: relative;
    overflow: hidden;
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
