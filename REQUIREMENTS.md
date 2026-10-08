# LBP (Lightweight Browser Project) — Requirements Document

**Version:** 1.3  
**Date:** October 8, 2026  
**Status:** Draft — Pending Review

---

## 1. Overview

LBP (Lightweight Browser Project) is a cross-platform (Windows and Linux) web browser built for speed, minimal resource usage, and a modern, clean user experience. It provides core browsing capabilities plus productivity features such as tabs, tab groups, history, downloads, bookmarks, and basic developer tools — all while staying lightweight and responsive.

### Guiding Principles

| Principle | Description |
|-----------|-------------|
| **Fast** | Cold start < 500ms; instant tab switching; minimal UI lag. |
| **Lightweight** | Idle RAM < 50MB; installer < 20MB; no bundled Chromium. |
| **Cross-platform** | First-class Windows 10/11 and Linux (Ubuntu, Fedora, Arch) support. |
| **Sleek & Modern** | Clean, minimal UI with subtle animations, not heavy skeuomorphic chrome. |
| **Basic but Complete** | Covers the essentials without feature bloat. |

---

## 2. Tech Stack Decision

### Recommended: Tauri 2.x + Svelte

| Dimension | Rationale |
|-----------|-----------|
| **Runtime** | Tauri uses the OS WebView (WebView2 on Windows, WebKitGTK on Linux) — no bundled browser. ~96% smaller installers and ~75% less RAM than Electron. |
| **Backend** | Rust: memory safety, no GC pauses, mature Tauri plugin ecosystem. |
| **Frontend** | Svelte/SvelteKit: compiled framework, ~1.6KB gzipped runtime, minimal DOM overhead, fast initial render — fits the "lightweight + fast" mandate. |
| **Styling** | Plain CSS + CSS variables (or Tailwind if the team prefers a utility system) — keep bundle lean. |
| **Storage** | Tauri's `tauri-plugin-store` for user data (bookmarks, history, settings) + SQLite (via `tauri-plugin-sql` or `rusqlite`) for larger datasets like history and downloads. |
| **Dev Tools** | WebView remote debugging protocol surfaced through a custom in-app dev tools panel: CDP on Windows (WebView2), Web Inspector protocol on Linux (WebKitGTK). Same panel UI across platforms; v1 documents which features are available on which platform rather than pretending full parity. |

### Alternatives Considered

| Option | Why Not Chosen |
|--------|---------------|
| **Electron** | Bundles full Chromium + Node.js: ~90MB installer, 200-300MB idle RAM — contradicts the lightweight mandate. |
| **CEF (Chromium Embedded Framework)** | Full Chromium embedded — same weight problem; heavier integration effort; not chosen. |
| **Neutralino** | Even lighter but smaller ecosystem and fewer native capabilities; less future-proof for a full browser. |
| **Wails (Go)** | Viable alternative; Tauri chosen for richer plugin ecosystem and optionality. |

---

## 3. Functional Requirements

### 3.1 Core Browsing

| ID | Requirement | Priority |
|----|-------------|----------|
| F-01 | Load web pages from user-entered URLs or clicks on links. | P0 |
| F-02 | Address bar with URL display, editing, and navigation (back/forward/reload/stop). | P0 |
| F-03 | Multi tab support: open, close, rearrange, duplicate, reload per tab. | P0 |
| F-04 | Tab bar UI: clickable tabs, close buttons, favicon/loading indicator, tab title. | P0 |
| F-05 | Middle-click / Ctrl+Click to open links in new tabs; Ctrl+T new tab; Ctrl+W close tab. | P0 |
| F-06 | Default new-tab page: speed dial + recently closed sites, configurable. | P1 |
| F-07 | Zoom controls (zoom in/out/reset per site, persisted per domain if feasible). | P2 |
| F-08 | Basic page context menu (back, forward, reload, save page, copy link, open in new tab). | P1 |

### 3.2 Tab Groups

| ID | Requirement | Priority |
|----|-------------|----------|
| F-09 | Create a named tab group; assign tabs to a group. | P1 |
| F-10 | Tab groups shown as sidebar chips in a collapsible left sidebar: group name + tab count + color. | P1 |
| F-11 | Color-tag groups for visual distinction. | P2 |
| F-12 | Move all tabs in a group together; close all tabs in a group. | P2 |
| F-13 | Persist tab groups across sessions. | P1 |

> **Design note:** Tab groups are exposed via a lightweight sidebar panel (vertical groups list) to keep the chrome minimal and reduce layout complexity. The sidebar is toggleable and optionally pinnable.

### 3.3 History

| ID | Requirement | Priority |
|----|-------------|----------|
| F-14 | Record visited URLs with timestamp, title, favicon, and referrer. | P0 |
| F-15 | History viewer UI: searchable, sortable list with date grouping. | P0 |
| F-16 | Search history by URL or page title (instant, client-side). | P0 |
| F-17 | Clear history option (all, or by time range: today, last hour, custom). | P0 |
| F-18 | "Reopen closed tab" (Ctrl+Shift+T) pulling from recent-closed stack. | P0 |
| F-19 | History retained locally; optional export of history as JSON/CSV. | P2 |

### 3.4 Downloads

| ID | Requirement | Priority |
|----|-------------|----------|
| F-20 | Download manager panel showing active + completed downloads. | P0 |
| F-21 | Show: filename, size, progress %, status, completion time. | P0 |
| F-22 | Cancel/pause downloads; open containing folder; open downloaded file. | P0 |
| F-23 | Default download folder configurable in settings; native picker dialog for "save as". | P0 |
| F-24 | Retain download history (file path, URL, date) viewable in manager. | P1 |
| F-25 | Handle common MIME types; show warning for unsafe file types where appropriate. | P1 |

### 3.5 Bookmarks

| ID | Requirement | Priority |
|----|-------------|----------|
| F-26 | Bookmark any page (single bookmark button in toolbar / via menu). | P0 |
| F-27 | Bookmark manager: tree/folder view, rename, move, delete, edit URL/title. | P0 |
| F-28 | Bookmark bar (visible/hidden toggle) showing top-level bookmarks for one-click access. | P0 |
| F-29 | Import bookmarks from HTML bookmark file (Netscape/Chrome export format). | P1 |
| F-30 | Export bookmarks to HTML. | P1 |
| F-31 | Search bookmarks by name/URL. | P1 |
| F-32 | Keyboard shortcut to bookmark current page (e.g., Ctrl+D). | P0 |

### 3.6 Navigation & UX Basics

| ID | Requirement | Priority |
|----|-------------|----------|
| F-33 | Home page / startup page configurable (blank, home URL, or new-tab page). | P0 |
| F-34 | Startup behavior: restore previous session tabs, or open fresh. | P0 |
| F-35 | Private/incognito mode: temporary session with no history, cookies, or form data persisted. | P1 |
| F-36 | Fullscreen mode (F11); picture-in-picture for supported video elements. | P2 |
| F-37 | Find in page (Ctrl+F): highlight matches, navigate between them. | P1 |
| F-38 | Basic ad-blocker toggle (optional, off by default) via hosts-style blocklist. | P3 |

### 3.7 Settings / Preferences

| ID | Requirement | Priority |
|----|-------------|----------|
| F-39 | Settings UI: appearance (theme: light/dark/system, font size, toolbar density), startup, downloads, privacy. | P0 |
| F-40 | Theme persisted; dark mode with system preference detection. | P0 |
| F-41 | Keyboard shortcut customizability (at minimum, a view of current shortcuts). | P2 |

### 3.8 Plugin / Extension Support

Extensions are **deferred past v1** for both Windows and Linux.

| ID | Requirement | Priority |
|----|-------------|----------|
| F-42 | No extension/plugin support in v1. The toolbar and UI reserve a future "Extensions" toggle slot, but no extension install/manage flow is built in v1. | P0 |

> **Scope note:** Chrome Web Store extension support is not part of v1. If added later, it will be bounded by what the OS WebViews (WebView2 on Windows, WebKitGTK on Linux) support, with compatibility notes where an API is unavailable. This keeps the v1 browser lightweight and avoids overpromising on extension parity.

### 3.9 Built-in Ad Blocker

| ID | Requirement | Priority |
|----|-------------|----------|
| F-43 | Built-in ad blocker enabled from settings; on by default for new users (opt-out), with a clear on/off toggle. | P1 |
| F-44 | Filter lists: ship one default starter blocklist (EasyList-derived); allow the user to enable/disable it and add custom filter rules. Multiple built-in lists with checkboxes are a follow-up. | P1 |
| F-45 | Filtering applied before/around page load via WebView request interception where supported; blocked requests reported in a lightweight panel or devtools-adjacent view. | P1 |
| F-46 | Whitelist per-site ("allow ads on this site") with easy toggle from the address bar or page context. | P1 |
| F-47 | Update blocklists on a configurable schedule (with a manual "check for updates" button); version the active list set in settings. | P2 |
| F-48 | Low-overhead filter matching to preserve the lightweight mandate; avoid full per-request heavy regex where a faster matching strategy is available. | P1 |

> **Implementation note:** Prefer a portable request-blocking approach that works across WebView2 and WebKitGTK rather than relying on one platform's content-blocker API only. The same starter lists and per-site whitelist apply on both platforms.

> **Performance note:** Ad blocking adds per-request work; the implementation must stay within the lightweight NFR budget (NF-03 idle, NF-06 query responsiveness). Filter matching should be optimized (e.g., Trie/domain-based matching) and benchmarked during Phase 4.

### 3.10 Built-in Volume Booster

| ID | Requirement | Priority |
|----|-------------|----------|
| F-49 | In-app volume booster control for the current tab's media audio; slider beyond the system 100% with a configurable ceiling (default 200%, user-resettable). | P1 |
| F-50 | Per-tab volume independent of system volume; mute toggle per tab; volume persisted per site/domain where feasible. | P1 |
| F-51 | Global master volume control for the browser (separate from OS volume) so the user can raise all tabs together. | P2 |
| F-52 | Visual volume indicator in the tab/toolbar area (level + mute state); click to access quick slider. | P2 |
| F-53 | Fallback behavior: where the OS/WebView audio pipeline cannot amplify beyond a certain point, clearly indicate the achievable range and avoid distorted output (soft clipping / limiter to protect the user's audio). | P1 |
| F-54 | Hearing-safety caution when boosting significantly above 100%; safety limiter on by default. | P1 |

> **Implementation note:** Volume boosting is applied at the audio/output layer the browser controls. On Windows this can use the audio session/mixer APIs (e.g., WASAPI session volume or a software gain stage). On Linux it uses the appropriate audio stack (PipeWire/PulseAudio control via the backend). Where direct per-tab audio session control is unavailable, the booster applies to the browser's audio output with per-tab mute/isolation semantics. The limiter protects against harsh clipping.

> **Default:** ceiling 200%, user-resettable up to 300% in settings, limiter/safety caution on by default.

### 3.11 Split Screen (Multi-view)

| ID | Requirement | Priority |
|----|-------------|----------|
| F-55 | Split-screen mode: display up to 4 tabs simultaneously in one window, in a configurable grid (1×2, 2×2, 4×1, 2×1, 1×2). | P1 |
| F-56 | Default entry behavior: entering split mode starts with a 2-pane layout; 4-pane is available via explicit opt-in in the layout picker. | P1 |
| F-57 | Each split pane is an independent WebView instance scoped to its tab; navigation, scroll, and zoom isolated per pane. | P1 |
| F-58 | UI to enter split mode: button/shortcut; picker to choose which tabs go into which pane; "add pane" up to the 4-pane limit; remove pane (pane's tab returns to the normal tab list). | P1 |
| F-59 | Resizable panes (drag divider) with snap/even-split option; remembered layout for the session. | P2 |
| F-60 | Each pane shows its own tab title/favicon in a small pane header; close pane (closes the tab or just un-splits). | P1 |
| F-61 | Copy link / open in new tab works per-pane; right-click context menu scoped to the pane's page. | P2 |
| F-62 | Persist split layout across session restore (where practical); on startup, optionally re-enter split mode with the prior layout or return to single-view. | P2 |
| F-63 | Performance guard: ensure 4 simultaneous WebView panes stays within the lightweight footprint budget for the target machine classes; provide a "lite" split mode that throttles off-screen panes if needed. | P2 |

> **Resource note:** Four live WebView instances per window is the heaviest feature in v1. It is P1 because the user asked for it, but it must be validated against NF-03 (idle RAM) on representative hardware. If 4-pane idles too high on low-end machines, the fallback is a 2-pane default with an explicit opt-in to 4 panes.

### 3.12 Workspaces

| ID | Requirement | Priority |
|----|-------------|----------|
| F-64 | Multiple workspaces: user can create, rename, and switch between workspaces. | P1 |
| F-65 | Each workspace is an isolated session context by default: its own tabs, tab groups, session state, and its own cookies/storage profile. | P1 |
| F-66 | Workspace switcher UI: quick switcher (Ctrl+Tab-like or a sidebar/workspace tray) showing workspace name + tab count; switch instantly. | P1 |
| F-67 | Default workspaces on first run: e.g., "Home" and a couple of empty workspaces; allow the user to add up to a reasonable cap (e.g., 10) with the ability to delete. | P2 |
| F-68 | Per-workspace settings where it makes sense (e.g., startup page, theme accent), while global settings (ad blocker lists, volume ceiling, shortcuts) remain global. | P2 |
| F-69 | Private/incognito can be a per-tab concept within a workspace without creating a new workspace; workspaces and private mode are independent features. | P2 |
| F-70 | Persist workspace state (tabs, groups, active tab, pane layout if in split mode, storage profile) across restarts; restore on launch. | P1 |
| F-71 | Move a tab between workspaces (drag or context action); close workspace (choose to close all its tabs or move them elsewhere). | P2 |
| F-72 | Optional setting to loosen workspace isolation later (share cookies/storage across workspaces) for users who do not want full per-workspace profiles. | P3 |

> **Scope interaction:** Workspaces compound the resource picture because each workspace can hold its own set of tabs and, by default in v1, its own storage profile. The implementation should keep inactive workspaces lightweight (e.g., not keep all their WebViews alive simultaneously unless the user opts into "preload"). Splitting within a workspace uses the split-screen feature (F-55) scoped to that workspace's tabs.

---

## 4. Developer Tools (Basic)

The goal is a usable, in-app dev tools panel — not a full Chrome DevTools clone. The browser surfaces what the underlying WebView's remote debugging protocol provides.

| ID | Requirement | Priority |
|----|-------------|----------|
| D-01 | Toggle dev tools panel with a toolbar button or shortcut (F12 / Ctrl+Shift+I). | P0 |
| D-02 | **Inspector/Elements:** view the rendered DOM tree, see element attributes/styles, hover to highlight on page. | P0 |
| D-03 | **Console:** JavaScript console logs (log, warn, error), interactive eval input. | P0 |
| D-04 | **Network:** list of requests with method, URL, status, size, timing; filter by type. | P1 |
| D-05 | **Application/Storage:** view cookies, localStorage, sessionStorage for the current site. | P1 |

> **Implementation note (Windows):** Use WebView2's CDP connectivity (e.g., DevTools front-end over the WebView2 CDP endpoint) to drive the panel.  
> **Implementation note (Linux):** Use WebKitGTK's inspector / Web Inspector protocol; adapt the same panel UI to the data it exposes.

> **Parity note:** The in-app dev tools panel is the same UI across Windows and Linux; the backend protocol differs (CDP on Windows, Web Inspector on Linux). v1 documents which panel features are available on which platform rather than pretending full parity.

---

## 5. Non-Functional Requirements

### 5.1 Performance

| ID | Requirement | Target |
|----|-------------|--------|
| NF-01 | Cold start (process launch to first renderable UI) | ≤ 500ms |
| NF-02 | New tab open (UI ready to type) | ≤ 100ms |
| NF-03 | Idle memory usage (browser chrome + 1 empty tab) | ≤ 50MB |
| NF-04 | Tab switch latency (visible UI change) | < 16ms (one frame) |
| NF-05 | Installer/download package size | ≤ 20MB |
| NF-06 | Address bar query responsiveness (history/bookmark search) | < 50ms for ≤ 10k entries |

### 5.2 Resource & Footprint

| ID | Requirement | Target |
|----|-------------|--------|
| NF-07 | Disk footprint for user data (bookmarks + history + settings) | minimal; SQLite with periodic compaction |
| NF-08 | Background/tab resource caps: no unused tabs spinning busily; throttle background tabs where the WebView allows. | best-effort |
| NF-09 | Multi-pane / workspace memory budget: 4-pane split should ideally idle within < 150MB on a typical modern machine; document the actual measured value on the test matrix and set an explicit cap. | target, not hard cap (see validation) |

### 5.3 Cross-Platform Consistency

| ID | Requirement |
|----|-------------|
| NF-10 | Core feature set identical on Windows and Linux; WebView differences documented and handled in code. |
| NF-11 | UI scaling/DPI awareness on Windows; proper HiDPI handling on Linux. |
| NF-12 | System theme detection (light/dark) on both platforms. |

### 5.4 Security & Privacy

| ID | Requirement |
|----|-------------|
| NF-13 | No telemetry by default; clearly documented if any optional usage stats are added later. |
| NF-14 | Cookie/storage isolation per profile (especially in private mode and per-workspace where storage profiles are enabled). |
| NF-15 | WebView security settings: disable unnecessary features by default (e.g., plugin frameworks), enable safe defaults. |
| NF-16 | Clear on-disk data on exit for private mode. |
| NF-17 | If extensions are added later, extension permissions must be reviewed and surfaced at install time; no extension granted broad access without user confirmation. |
| NF-18 | Ad-blocker filter list updates fetched over HTTPS from trusted sources; verify list integrity (hash/signature) before applying. |

### 5.5 Accessibility

| ID | Requirement |
|----|-------------|
| NF-19 | Keyboard navigable UI; focus rings visible. |
| NF-20 | Supports system font scaling / larger text. |
| NF-21 | Screen reader friendly where feasible given WebView constraints. |

---

## 6. UI/UX Design Direction

### 6.1 Aesthetic

- **Minimal chrome:** thin toolbar, unobtrusive tabs, generous content area.
- **Clean typography:** system sans-serif stack, legible sizes, comfortable line height in UI panels.
- **Subtle motion:** short, polite transitions for tab open/close, panel slide-in, hover states — no theatrical animation.
- **Monochrome base + one accent color** (configurable accent in settings; default a neutral blue/slate).

### 6.2 Layout Concept

```
+----------------------------------------------------------+
| [≡] [URL bar ..................................] [🔍][⬇][⋮]   <- top toolbar (compact)
+----------------------------------------------------------+
| [Tab1] [Tab2] [Tab3 +]  [tab group chip]                <- tab bar
+----------------------------------------------------------+
|                                                          |
|              WebView content area                        |
|              (maximizes vertical space)                  |
|                                                          |
+----------------------------------------------------------+
| status bar: loading spinner | security indicator | zoom %
+----------------------------------------------------------+
```

- Tab groups exposed via a collapsible left sidebar (toggle) showing group chips with tab counts and colors; optionally pin the sidebar.
- Downloads, bookmarks, and history each have a drawer/panel that slides in from the right or extends a sidebar — never modal where avoidable.
- Workspaces: a workspace switcher (sidebar item or top-of-tab-bar tray) for fast switching; each workspace can optionally be previewed as a thumbnail strip on hover (P2).
- Split screen: when active, the content area becomes a grid of panes, each with its own mini-header (pane title/favicon) and resize handles.

### 6.3 Design System Tokens

| Token | Suggestion |
|-------|-----------|
| Background (default) | `#FAFAFA` light / `#1A1A1E` dark |
| Surface | `#FFFFFF` / `#252529` |
| Border | `#E5E5E5` / `#333338` |
| Text primary | `#1A1A1A` / `#E8E8E8` |
| Accent | `#4F6AF5` (default blue) — user-customizable |
| Radius | small (4–6px) for a modern, crisp look |
| Font | system stack: `Segoe UI`, `Inter`, `Ubuntu`, `Noto Sans` by platform |

### 6.4 Interaction Highlights

- **URL bar:** inline autocomplete from history + bookmarks + tabs, with keyboard nav (arrow keys, Enter).
- **Tabs:** drag-to-reorder; double-click tab strip to create new tab; scroll arrows or wheel for overflow.
- **Bookmarks bar:** one-line horizontal bar under the tab bar when enabled; folders expand inline on click/hover.
- **Downloads:** a small badge on the download button showing active count; panel opens on click.
- **Workspaces:** quick-switch list; double-click a workspace to switch; right-click to rename/delete.
- **Split screen:** "Split view" button in the toolbar opens a pane layout picker; panes resize by dragging dividers.

---

## 7. Component Inventory

### 7.1 Toolbar
- Back, Forward, Reload, Stop, Home, Bookmarks toggle, Downloads toggle, Dev Tools toggle, Menu (≡), Ad blocker toggle (F-43), Volume control (F-49), Split view toggle (F-55), Workspace switcher (F-64).

### 7.2 Tab Bar
- Tab: favicon, title (truncated), close button, loading spinner, mute indicator (P2), pin/unpin (P2).
- New tab button (+), tab group chip when grouped.

### 7.3 Sidebar (optional/contextual)
- Tab groups panel, bookmarks tree, history list, download list, workspace list — one at a time or via tabs within the sidebar.

### 7.4 Panels (right-side drawers)
- Downloads panel, Bookmark manager, History search, Settings, Ad blocker whitelist/control.

### 7.5 Split Pane Header
- Per-pane mini chrome: pane title/favicon, close pane, maximize this pane (exit split for that tab), copy URL, context menu.

### 7.6 Workspace Switcher
- Dropdown/list with workspace names, tab counts, active indicator, add/rename/delete actions.

### 7.7 Dev Tools Panel
- Tabbed panel: Elements | Console | Network | Application.
- Resizable; collapsible; toggle from toolbar.

### 7.8 Volume Control
- Inline loudness slider + mute + dB/percent read-out; accessible from toolbar and per-tab mini-header.

### 7.9 Ad Blocker UI
- Settings toggle + per-site whitelist chip in the address bar; blocked-request count badge optional (P2).

### 7.10 Dialogs & Modals
- "Save As" file picker (native dialog via Tauri), confirmation dialogs (clear history, close multiple tabs, delete workspace), about/version dialog.

---

## 8. Data Model (Preliminary)

### Entities

| Entity | Key Fields | Storage |
|--------|-----------|---------|
| **Bookmark** | id, url, title, favicon, parent_folder_id, date_added, keyword (P2) | SQLite + store |
| **BookmarkFolder** | id, name, parent_id | SQLite + store |
| **HistoryEntry** | id, url, title, favicon, visit_time, referrer, duration (P2) | SQLite |
| **Download** | id, url, filename, file_path, total_size, received_bytes, status, start_time, end_time | SQLite |
| **TabGroup** | id, name, color, tab_ids (ordered), date_created | store / SQLite |
| **Workspace** | id, name, color/icon, tabs[], active_tab_id, groups[], pane_layout (optional), storage_profile_enabled (bool, default true for v1), date_created | store / SQLite |
| **Session** | active_workspace_id, windows[], window_rects | store |
| **FilterList** | id, name, url, checksum/signature, date_fetched, enabled | store / SQLite |
| **FilterRule** (custom) | id, pattern, type, enabled | store / SQLite |
| **Settings** | theme, accent, startup_action, download_path, adblock_enabled, adblock_lists, volume_ceiling, split_default_layout, workspace_cap, workspace_isolation_default, etc. | store (Tauri store plugin) |

> Persistence detail: prefer SQLite for history + downloads + filter data (queryable, compaction); use Tauri store for settings + small lists.

> **Note:** There is no Extension entity in v1, since extensions are deferred.

---

## 9. Platform Support Matrix

| Feature | Windows 10/11 | Linux (Ubuntu/Fedora/Arch) |
|---------|---------------|---------------------------|
| WebView backend | WebView2 (Chromium-based) | WebKitGTK |
| Dev Tools protocol | CDP (Chrome DevTools Protocol) | Web Inspector protocol |
| Native dialogs | Win32 / WinUI via Tauri | GTK file dialogs via Tauri |
| System theme sync | Supported | Supported (GTK theme hint) |
| HiDPI | Per-monitor DPI | GTK scaling |
| Extension support | **Deferred past v1** | **Deferred past v1** |
| Audio/volume control | WASAPI session / audio session APIs or software gain stage | PipeWire/PulseAudio control via backend |
| Minimum OS | Windows 10 (WebView2 runtime or built-in on Win11) | Linux with WebKitGTK + a modern GTK (Ubuntu 22.04+, Fedora 36+, Arch current) |

> **Installer delivery:** Windows MSI/EXE; Linux: AppImage + deb/rpm + AUR packaging target.

---

## 10. Out of Scope (for v1)

- Sync/account system across devices.
- Full Chrome DevTools clone (profiler, memory snapshot, advanced debugging).
- Built-in PDF viewer beyond what the WebView provides.
- Password manager / credential storage.
- Mobile platforms (desktop only for v1).
- Plugin/extension support (deferred past v1).
- macOS support (deferred; not part of v1).
- Full feature parity for dev tools across Windows and Linux in v1 (documented gaps are acceptable; see D-01–D-05).

---

## 11. New Feature Scope Notes

### 11.1 Ad Blocker
- Built-in, with starter lists + custom rules + per-site whitelist + periodic updates + integrity check.
- Kept low-overhead to preserve NF-03/NF-06.
- Portable request-blocking approach across WebViews; no reliance on a single platform's content-blocker API only.

### 11.2 Volume Booster
- Per-tab and global master boost, with a user-set ceiling and a limiter to avoid distortion; hearing-safety caution at high boost levels.
- Default ceiling 200%, limiter/safety caution on by default.
- Implemented per platform (WASAPI on Windows; PipeWire/PulseAudio on Linux).

### 11.3 Split Screen
- Up to 4 panes in a configurable grid; each pane is an independent WebView scoped to its tab; resize handles and persistent layout.
- 2-pane default on entry; 4-pane explicit opt-in.

### 11.4 Workspaces
- Multiple isolated session contexts with quick switching.
- **v1 default: full isolation** — each workspace has its own tabs, groups, session state, and cookies/storage profile.
- Optional setting to loosen isolation later for users who do not want per-workspace profiles.

---

## 12. Testing & Deployment Pipeline (Automated)

Automated tests must **deploy the browser** (build + install/launch on target platforms) and then exercise it. This is part of the requirements, not optional.

### 12.1 Test Types

| ID | Requirement | Priority |
|----|-------------|----------|
| T-01 | **Build + package pipeline:** CI that builds the Windows installer and Linux packages (AppImage + deb/rpm) on every commit to main; publishes artifacts to a test bucket/storage. | P0 |
| T-02 | **Deploy-on-target smoke tests (Linux CI now):** On the Linux CI runner, download the built artifact, install/run it (AppImage or installed package), launch the browser, and verify it starts without crash. | P0 |
| T-03 | **GUI/headless smoke tests:** Automated tests exercising core flows: navigate to a URL, open/close tabs, bookmark a page, add to history, start a download (to a temp dir), enable/disable ad blocker, create a workspace, switch workspaces, enter split mode with up to 4 panes, toggle dev tools. | P0 |
| T-04 | **Ad-blocker verification:** Load a page containing known ad/tracker requests and verify (via the browser's blocked-request reporting or a controlled test page) that the requests are blocked when the ad blocker is on and allowed when it is off. | P1 |
| T-05 | **Volume booster verification:** Play test audio in a tab and verify the booster slider affects output level up to the configured ceiling (using an audio-level probe/measurement on the test machine), and that mute per-tab works. | P1 |
| T-06 | **Split-screen verification:** Enter split mode, place up to 4 tabs, navigate each pane to a different URL, and verify each pane renders independently and that resize/layout persists for the session. | P1 |
| T-07 | **Workspace verification:** Create two workspaces, put different tabs in each, switch between them, and verify the tab sets and storage are isolated; restart and verify restore. | P1 |
| T-08 | **Performance budgets in CI:** For representative scenarios (idle 1 tab, idle 4-pane split, workspace switch, ad blocker on/off), measure memory and startup time on the test matrix and fail the build if a budget is violated beyond a defined tolerance. | P1 |
| T-09 | **Regression test suite:** A persistent suite (e.g., Playwright/WebDriver-style automation against the running browser, or Tauri's built-in test facilities plus a small test harness) that runs on CI and can be triggered locally for developer runs. | P1 |
| T-10 | **Platform matrix automation:** Tests run on at least one Linux CI node now; add a Windows CI node when available. Results reported per platform. | P1 |
| T-11 | **Windows deploy verification (deferred):** When Windows CI runners are available, add deploy-on-target smoke tests for Windows; until then, Windows deploy verification is manual/VM. | P2 |

### 12.2 Deployment Verification (Automated / Manual)

| ID | Requirement | Priority |
|----|-------------|----------|
| T-12 | **Fresh-install verification (Linux, automated):** On a clean Linux test image (no prior LBP install), install from the generated artifact and verify first-run behavior (welcome, default workspace, default settings). | P1 |
| T-13 | **Fresh-install verification (Windows, manual/VM):** On a clean Windows VM/image, install from the generated artifact and verify first-run behavior. | P2 |
| T-14 | **Upgrade verification:** Install vA, then install vB over it (or run the updater path if an updater exists), and verify data (bookmarks, history, workspaces) survives. | P2 |
| T-15 | **Portable/uninstall verification (Windows):** Run portable mode if supported; uninstall and verify leftover user data behavior matches settings (opt-in keep data vs. full clean). | P2 |

### 12.3 Test Infrastructure Requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| T-16 | Test fixtures: a set of controlled test pages (including ad/tracker sample pages, audio test page, multi-tab pages) validated to work with the WebView backends on both platforms. | P1 |
| T-17 | Deterministic environment: CI runners pinned to known OS versions (Linux first); WebDriver/remote-control interface for the browser exposed for automation (e.g., a local automation port or Tauri test harness) so tests can drive the UI without manual interaction. | P1 |
| T-18 | Artifact retention: built installers/packages retained for the duration of the test campaign so deploy-on-target tests can pull the exact build under test. | P2 |
| T-19 | Failure capture: when a deploy or smoke test fails, capture logs, crash dumps where available, and the exact build/version tested, and surface in CI. | P1 |
| T-20 | **Windows/macOS test deferral:** Windows and macOS deploy verification are manual/VM for now and added to CI later when runners are available; the requirements for those platforms are still captured, just not automated in CI yet. | P2 |

> **Why this is required:** The user explicitly asked for tests that deploy the browser and test it. T-01 through T-20 make that a requirement: CI builds + packages, a Linux deploy-on-target smoke step now, automated GUI/functional tests that exercise the requested features (tabs, groups, history, downloads, bookmarks, ad blocker, volume booster, split screen, workspaces, dev tools) on Linux first and Windows when runners are available.

---

## 13. Milestones (Proposed)

| Phase | Focus | Outcome |
|-------|-------|---------|
| **Phase 0 — Requirements & Design** | This document + UI wireframes | Signed-off PRD + look-and-feel reference. |
| **Phase 1 — Shell** | Tauri + Svelte scaffold; window, menu, toolbar, URL bar, one webview. | Can load a URL and navigate back/forward. |
| **Phase 2 — Tabs** | Tab model, tab bar, new/close/reorder, session restore. | Multi-tab browsing works. |
| **Phase 3 — Tab Groups + Bookmarks + Workspaces** | Groups sidebar (chips), bookmark bar/manager, workspace model + switcher (full isolation default). | Grouping, bookmarks, and workspaces usable. |
| **Phase 4 — History + Downloads + Ad Blocker** | History DB, search, download manager, file dialogs, ad blocker with starter lists + whitelist. | History, downloads, and ad blocking complete. |
| **Phase 5 — Volume Booster + Split Screen** | Volume booster, split-screen up to 4 panes (2-pane default, 4-pane opt-in). | Volume and split view functional. |
| **Phase 6 — Dev Tools + Polish + Packaging** | In-app dev tools, theme, accessibility, packaging for Windows + Linux, performance tuning. | v1 release candidate. |
| **Phase 7 — Automated Test & Deploy Pipeline** | CI build/package (Linux now, Windows when available), deploy-on-target smoke tests, GUI/functional test suite, performance budgets, platform matrix. | CI-verified Linux builds; tests that deploy and exercise the browser. |

> Phases 3 and 5 each bundle multiple user-requested features; Phase 7 is added explicitly to satisfy the "tests that actually deploy the browser and test it" requirement.

---

## 14. Decisions (Locked)

The following were settled and folded into the requirements above:

1. **Windows CI / deploy verification timing:** Keep Windows deploy verification manual/VM for now; add Windows CI runners later when available (T-11, T-13, T-20). Linux CI runs the automated deploy-on-target smoke tests now.
2. **Workspaces full isolation default weight:** Full per-workspace storage profile isolation stays the v1 default; hard cap of 10 workspaces; optional "loosen isolation" setting (F-72). No automatic fallback to looser isolation — if weight is an issue in testing, revisit a stronger auto-fallback later.
3. **Ad blocker starter lists:** One starter blocklist set by default (EasyList-derived) + custom rule support + per-site whitelist; multiple built-in lists with checkboxes are a follow-up if wanted later.
4. **Volume booster max ceiling upper bound:** User may raise the ceiling in settings up to 300%; default is 200%; limiter/safety caution on by default.

---

## 15. Acceptance Criteria (v1)

- [ ] Browser starts cold in ≤ 500ms and is idle ≤ 50MB with one empty tab.
- [ ] Open, close, switch, and reorder tabs; session restores on restart (configurable).
- [ ] Create, name, color, collapse, and persist tab groups (sidebar chips).
- [ ] Navigate, search, and clear browsing history.
- [ ] Download files with progress, pause/cancel, open folder, configurable default path.
- [ ] Bookmark pages; manage via bar + manager; import/export HTML.
- [ ] In-app dev tools: Elements, Console, Network, and Storage views functional on Windows and Linux (with documented per-platform gaps).
- [ ] Light/dark theme with system sync; modern, minimal UI.
- [ ] No extension support in v1 (deferred); UI reserves a future extensions slot but no extension flow is built.
- [ ] Built-in ad blocker: on/off, starter lists, custom rules, per-site whitelist, periodic updates with integrity check; portable across WebViews.
- [ ] Built-in volume booster: per-tab + global master boost, default ceiling 200%, mute per tab, limiter/safety caution on by default; implemented per platform (WASAPI/PipeWire).
- [ ] Split screen: up to 4 panes in a configurable grid, each pane independent, resize handles, session-persistent layout; 2-pane default on entry, 4-pane explicit opt-in.
- [ ] Workspaces: create/rename/switch, **full isolation by default** (tabs + groups + session + cookies/storage profile), persist and restore; optional loosen setting.
- [ ] Packages: Windows installer + Linux AppImage (and deb/rpm/AUR targets) build cleanly.
- [ ] CI pipeline builds packages and runs deploy-on-target + smoke + functional tests on Linux now; Windows deploy verification manual/VM until Windows CI runners are added; performance budgets checked.
- [ ] No telemetry by default.

---

*Next step: review and sign off on this document, then we proceed to UI wireframes and the Phase 1 scaffold.*
