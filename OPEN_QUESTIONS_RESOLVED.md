# LBP — Open Questions Resolution (Decision Record)

**Date:** October 8, 2026  
**Status:** Decision record only — the live requirements are in `REQUIREMENTS.md` (v1.3). This file records *why* the final state is what it is.

---

## 1. macOS removed; focus is Windows + Linux

The user asked to remove macOS entirely and focus on Windows and Linux. All macOS references, extension/audio/dev-tools/macOS-packaging open questions, and macOS-specific decisions were removed from the live requirements and from this record.

## 2. Resolved decisions (folded into REQUIREMENTS.md v1.3)

| Topic | Decision | Where in REQUIREMENTS.md |
|-------|----------|--------------------------|
| Tab groups | Sidebar chips (collapsible left sidebar) | F-10, §6.2, §7.3 |
| New-tab page | Speed dial + recent sites, configurable | F-06 |
| Private/incognito | In v1 (P1) | F-35 |
| Linux distro targets | Ubuntu, Fedora, Arch only | §9, §12 |
| Dev tools scope (v1) | Elements, Console, Network, Storage | D-01–D-05 |
| Extensions | Deferred past v1 for both Windows and Linux | F-42, §3.8, §10 |
| Volume booster | Default ceiling 200%, limiter/safety caution on by default | F-49–F-54, §3.10, §11.2 |
| Split screen default | 2-pane default on entry, explicit opt-in to 4-pane | F-55–F-56, §3.11, §11.3 |
| Workspaces isolation | Full isolation by default (tabs + groups + session + cookies/storage profile) | F-65, F-72, §3.12, §11.4 |
| Test environment | Linux CI now; Windows deploy verification manual/VM until Windows CI runners are added | T-02, T-10, T-11, T-13, T-20, §12 |
| Workspaces storage loosening | Optional setting to loosen isolation later (P3) | F-72 |
| Platform parity | Same panel UI across Windows and Linux; backend differs (CDP vs Web Inspector); document gaps | D-01–D-05 parity note, F-21/F-22 legacy parity items removed |
| Packaging | Windows MSI/EXE; Linux AppImage + deb/rpm + AUR | §9, §12 |

## 3. Remaining open questions (resolved and folded into REQUIREMENTS.md v1.3)

These were settled and folded into the requirements:

1. **Windows CI / deploy verification timing:** Keep Windows deploy verification manual/VM for now; add Windows CI runners later when available (T-11, T-13, T-20). Linux CI runs the automated deploy-on-target smoke tests now.
2. **Workspaces full isolation default weight:** Full per-workspace storage profile isolation stays the v1 default; hard cap of 10 workspaces; optional "loosen isolation" setting (F-72). No automatic fallback to looser isolation.
3. **Ad blocker starter lists granularity:** One default starter blocklist (EasyList-derived) + custom rules + per-site whitelist; multiple built-in lists with checkboxes are a follow-up.
4. **Volume booster max ceiling upper bound:** User may raise the ceiling up to 300% in settings; default is 200%; limiter/safety caution on by default.

After this, §14 in REQUIREMENTS.md is a "Decisions (Locked)" section, not an open-questions section.

## 4. Removed topics (not in v1)

- macOS support
- Chrome Web Store extension support (deferred)
- Per-platform extension/audio/dev-tools/macOS-packaging open questions (removed because macOS and extensions are out of v1)

---

*This record exists to explain the decisions behind REQUIREMENTS.md v1.3. Edit REQUIREMENTS.md for the live requirements, not this file.*
