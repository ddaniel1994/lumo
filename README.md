# Lumo

Lightweight, fast browser for **Windows** and **Linux**.

**Project status:** scaffold complete and push-ready (pending first push to `main`).

## What it is

Lumo is a lightweight browser built on [Tauri 2.x](https://tauri.app/) + [Svelte](https://svelte.dev/) + Vite. It uses the OS WebView (WebView2 on Windows, WebKitGTK on Linux) rather than bundling a full browser engine, which keeps the installer small and idle memory low.

## v1 scope

For v1, Lumo covers:

- Tabs, tab groups (sidebar chips), history, downloads, bookmarks
- Private/incognito mode
- Built-in ad blocker (on by default, with starter lists + custom rules + per-site whitelist)
- Built-in volume booster (per-tab + global master, default ceiling 200%, limiter/safety on by default, user can raise up to 300% in settings)
- Split screen (up to 4 panes; 2-pane default on entry, explicit opt-in to 4 panes)
- Workspaces (full isolation by default: tabs + groups + session + cookies/storage profile, 10-workspace cap, optional loosen setting)
- Basic in-app dev tools (Elements, Console, Network, Application/Storage)

Out of scope for v1:

- Extension/plugin support from the Chrome Web Store (deferred)
- macOS support (deferred)
- Sync/account system
- Full Chrome DevTools clone
- Password manager / credential storage

See [REQUIREMENTS.md](REQUIREMENTS.md) for the full requirements and decisions, and [OPEN_QUESTIONS_RESOLVED.md](OPEN_QUESTIONS_RESOLVED.md) for the decision record.

## Tech stack

- **Frontend:** Svelte 5 + Vite
- **Backend:** Rust + Tauri 2.x
- **Storage:** Tauri store plugin + SQLite (in later phases)
- **Webviews:** WebView2 (Windows), WebKitGTK (Linux)
- **License:** MIT

## Develop

### Prerequisites

- Node 18+
- Rust toolchain (required by Tauri)
- On Windows: WebView2 runtime
- On Linux: a modern WebKitGTK stack (e.g. Ubuntu 22.04+, Fedora 36+, Arch current)

### Setup

```bash
npm install
npm run tauri:dev
```

`npm run tauri:dev` runs the frontend via Vite in development mode and launches Tauri with the dev server. `npm run tauri:build` builds the platform packages.

### Repo status

- Scaffold is complete and push-ready.
- Icon files are still placeholders; replace them before release packaging.
- The main WebView still needs to be wired into the content area (Phase 1 core task).
- Rust toolchain and npm deps are required to build; not verified in this environment.

### Project structure

```
./src                 # Svelte frontend
./src/lib             # UI components and utils
./src-tauri           # Rust backend + Tauri config + Cargo
./docs                # architecture, wireframes, release notes
./REQUIREMENTS.md     # Requirements + decisions
./OPEN_QUESTIONS_RESOLVED.md
./TODO.md
./LICENSE            # MIT
```

## Releases

v1 targets Windows (MSI/EXE) and Linux (AppImage, deb, rpm, AUR). Signing/notarization and CI packaging details are captured in the requirements.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — see [LICENSE](LICENSE).
