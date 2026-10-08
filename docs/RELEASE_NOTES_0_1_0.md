# Release Notes — v0.1.0 (scaffold)

This is the initial scaffold commit for the LBP browser repo. It is not yet a usable browser release.

## What this commit includes

- Repo structure for a Tauri 2.x + Svelte 5 + Vite browser project
- Rust backend shell (`src-tauri`) with store plugin wired in
- Frontend shell UI (`src/`) with toolbar, URL bar, tab bar placeholder, status bar, and panels placeholders
- Requirements v1.3, decision record, README, CONTRIBUTING, MIT license, .gitignore

## Scope note

v1 is Windows + Linux only. macOS and Chrome Web Store extension support are deferred.

## Next step

Wire the main WebView into the content area and get the dev mode shell running on Windows and Linux.
