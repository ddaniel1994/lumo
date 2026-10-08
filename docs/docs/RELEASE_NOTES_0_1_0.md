# Release Notes — v0.1.0 (scaffold)

This commit is the initial scaffold for the Lumo browser repo. It is not yet a usable browser release.

## Owner / repo

- GitHub: https://github.com/ddaniel1994/lumo
- Status: scaffold complete and push-ready; first push pending.

## What this commit includes

- Repo structure for a Tauri 2.x + Svelte 5 + Vite browser project
- Rust backend shell (`src-tauri`) with store plugin wired in
- Frontend shell UI (`src/`) with toolbar, URL bar, tab bar placeholder, status bar, and panel placeholders
- Reusable UI components and theme helpers
- Requirements v1.3, decision record, README, CONTRIBUTING, MIT license, `.gitignore`, docs, TODO, release notes

## Scope note

v1 is Windows + Linux only. macOS and Chrome Web Store extension support are deferred.

## Decisions locked

- Tab groups as sidebar chips
- New-tab page = speed dial + recent, configurable
- Private/incognito in v1
- Dev tools v1 = Elements, Console, Network, Storage
- Ad blocker on by default, one starter list + custom rules + per-site whitelist
- Volume booster default ceiling 200%, up to 300% in settings, limiter/safety on
- Split screen 2-pane default, 4-pane opt-in
- Workspaces full isolation by default, 10-workspace cap, optional loosen setting
- Linux CI now; Windows deploy verification manual/VM until Windows CI runners are added

## Next step

Wire the main WebView into the content area and get the dev mode shell running on Windows and Linux.
