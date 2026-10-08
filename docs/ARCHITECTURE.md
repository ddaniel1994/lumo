# LBP Architecture Notes

## Layers

- **Frontend (Svelte 5 + Vite)** — UI shell, toolbar, URL bar, panels, webview placeholder.
- **Tauri backend (Rust)** — window lifecycle, menu, devtools toggle in dev builds, store plugin, future commands for history/downloads/ad blocker/volume/workspaces.
- **Webview** — WebView2 on Windows, WebKitGTK on Linux. Renders web content and exposes the remote debugging protocol for dev tools.

## Webview backends

| Platform | Backend | DevTools protocol |
|----------|---------|-------------------|
| Windows | WebView2 (Chromium-based) | CDP |
| Linux | WebKitGTK | Web Inspector protocol |

Dev tools in v1 are the same in-app panel UI on both platforms; the backend protocol differs. Document which features are available on which platform.

## Storage

- Settings + small lists: `tauri-plugin-store`.
- Larger datasets in later phases: SQLite (history, downloads, filter lists, workspace data, future extension data).
- No Extension entity in v1 since extensions are deferred.

## Phase mapping

- **Phase 1:** Shell — window, menu, toolbar, URL bar, one webview.
- **Phase 2:** Tabs — tab model + tab bar + session restore.
- **Phase 3:** Tab groups (sidebar chips) + bookmarks + workspaces (full isolation default).
- **Phase 4:** History + downloads + ad blocker.
- **Phase 5:** Volume booster + split screen (2-pane default, 4-pane opt-in).
- **Phase 6:** Dev tools + polish + packaging (Windows + Linux).
- **Phase 7:** Automated test + deploy pipeline (Linux CI now, Windows CI later).

## Platform notes

- Keep WebView differences behind a small platform adapter layer so the frontend and most backend logic stay shared.
- Any feature that behaves differently per platform (dev tools, audio, request interception) should be surfaced and documented, not hidden.

## Performance targets to keep in mind

- Cold start ≤ 500ms
- Idle memory (chrome + 1 empty tab) ≤ 50MB
- Installer package ≤ 20MB
- Address bar query < 50ms for ≤ 10k entries
- Tab switch < 16ms

See [REQUIREMENTS.md](REQUIREMENTS.md) for the full list.
