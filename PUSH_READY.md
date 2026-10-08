# Push-ready note for Lumo

This is a short handoff for the final push step when the terminal is available.

## Current state

- Scaffold is complete in `/home/ddaniel/lumo`.
- Placeholders are fixed for owner `ddaniel1994` and repo `lumo`.
- Working directory was renamed from `luma` to `lumo`; stray `luma` directory removed.
- Local `.git` exists inside `/home/ddaniel/lumo` (may need a fresh commit + remote setup).
- Target remote: `https://github.com/ddaniel1994/lumo.git`

## Files included

- README.md, LICENSE (MIT), CONTRIBUTING.md, .gitignore
- package.json, vite.config.ts, svelte.config.js, tsconfig.json
- src/ (Svelte frontend shell + lib/ui + lib/utils)
- src-tauri/ (Rust backend shell, Tauri 2.x config, capabilities, icon placeholders)
- docs/ (ARCHITECTURE.md, WIREFRAMES.md, RELEASE_NOTES_0_1_0.md)
- REQUIREMENTS.md, OPEN_QUESTIONS_RESOLVED.md, TODO.md

## Push steps (when terminal is back)

1. `cd /home/ddaniel/lumo`
2. `git config --local --add safe.directory /home/ddaniel/lumo`
3. `git remote add origin https://github.com/ddaniel1994/lumo.git` (only if not already set)
4. Check remote state: `git ls-remote origin` or `git fetch origin`
5. If the remote already has a `main` commit (for example from GitHub auto-init), either:
   - push with `git push -u origin main --force` only if that is acceptable, or
   - reconcile branches first; do not blindly force-push if the remote is not supposed to be replaced
6. Otherwise: `git push -u origin main`

## Caveats

- Rust toolchain and npm deps are not verified in this environment.
- Icon files are placeholders; replace before release packaging.
- The main WebView still needs to be wired into the content area (Phase 1).

## Deferred (not in v1)

- macOS support
- Chrome Web Store extension support
