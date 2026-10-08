# Lumo Todo

## Done

- Requirements v1.3 (Windows + Linux only; macOS and Chrome Web Store extensions deferred)
- Decisions locked:
  - Tab groups as sidebar chips
  - New-tab page = speed dial + recent, configurable
  - Private/incognito in v1
  - Dev tools v1 = Elements, Console, Network, Storage
  - Ad blocker on by default, one starter list + custom rules + per-site whitelist
  - Volume booster default ceiling 200%, up to 300% in settings, limiter/safety on
  - Split screen 2-pane default, 4-pane opt-in
  - Workspaces full isolation by default, 10-workspace cap, optional loosen setting
  - Linux CI now; Windows deploy verification manual/VM until Windows CI runners are added
- Repo scaffold complete and push-ready
- Placeholders fixed for ddaniel1994 / lumo

## Pending

- Push scaffold to https://github.com/ddaniel1994/lumo.git (pending terminal availability / push by owner)

## Phase 1 (Shell)

- [ ] Wire the main WebView into the content area via Tauri
- [ ] Toolbar navigation: back/forward/reload
- [ ] URL bar navigation + basic autocomplete placeholder
- [ ] Status bar feedback for navigation
- [ ] Verify dev mode builds and launches on Linux and Windows

## Phase 2 (Tabs)

- [ ] Tab model: open/close/reorder/duplicate/reload
- [ ] Tab bar UI
- [ ] Session restore on startup

## Phase 3 (Tab groups + bookmarks + workspaces)

- [ ] Tab groups sidebar chips (F-10)
- [ ] Bookmarks: bar + manager + import/export
- [ ] Workspaces: create/rename/switch, full isolation default (F-65)

## Phase 4 (History + downloads + ad blocker)

- [ ] History DB + search + clear
- [ ] Downloads panel + file dialogs
- [ ] Ad blocker with starter list + whitelist (F-43)

## Phase 5 (Volume + split screen)

- [ ] Volume booster per-tab + global master (F-49)
- [ ] Split screen up to 4 panes, 2-pane default (F-55/F-56)

## Phase 6 (Dev tools + polish + packaging)

- [ ] In-app dev tools panel (Elements, Console, Network, Storage)
- [ ] Theme polish + settings
- [ ] Packaging for Windows + Linux

## Phase 7 (Test + deploy pipeline)

- [ ] CI build + package (Linux now, Windows when available)
- [ ] Deploy-on-target smoke tests (Linux CI now)
- [ ] GUI smoke tests for core flows
- [ ] Performance budgets in CI

## Out of scope for v1

- Chrome Web Store extensions
- macOS support
- Sync/account system
- Full Chrome DevTools clone
- Password manager
