# Contributing to LBP Browser

Thanks for helping out. This is an early project, so the main goals are keeping it lightweight, fast, and aligned with the requirements in [REQUIREMENTS.md](REQUIREMENTS.md).

## Repo scope

- v1 is **Windows + Linux only**.
- **macOS** and **Chrome Web Store extension support** are explicitly out of v1.
- Decisions are recorded in [REQUIREMENTS.md](REQUIREMENTS.md) and [OPEN_QUESTIONS_RESOLVED.md](OPEN_QUESTIONS_RESOLVED.md). If a change affects a locked decision, update those files too.

## Development setup

1. Install Node 18+ and a Rust toolchain.
2. Install frontend deps: `npm install`
3. Run dev mode: `npm run tauri:dev`

On Windows you need the WebView2 runtime. On Linux you need a modern WebKitGTK stack.

## Branching and PRs

- Work in feature branches off `main`.
- Keep PRs small and tied to a requirement ID when possible (for example, F-10, F-43, F-55).
- PRs that add, change, or remove scope should update `REQUIREMENTS.md` and/or `OPEN_QUESTIONS_RESOLVED.md`.

## Commit style

Use short, descriptive commit messages that explain the why, not just the what. Example:
- `feat: add tab group sidebar chip panel (F-10)`
- `fix: preserve workspace storage profile across restart (F-70)`

## Testing notes

The project is meant to have automated tests that **deploy the browser and test it**, not just unit tests. See the testing requirements in [REQUIREMENTS.md](REQUIREMENTS.md) §12. For now, smoke tests and deploy verification on Linux CI are the priority; Windows deploy verification is manual/VM until Windows CI runners are added.

## Code style

- Frontend: Svelte 5 conventions, kept lean; avoid pulling in heavy libs unless they earn their place.
- Backend: Rust, memory-safe, no unnecessary allocations in hot paths.
- Keep the lightweight/performance targets in mind (idle memory, startup time, package size).

## Questions

If something is unclear, open an issue rather than silently changing scope.
