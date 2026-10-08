# LBP UI Wireframes (Low-Fi)

These are low-fi concepts for the shell and key panels. They should be refined into real UI during implementation, but they establish the layout intent from REQUIREMENTS.md.

## Shell (Phase 1)

```
+--------------------------------------------------------------+
|  LBP 0.1.0   [ URL bar ................................ ]  ↻ ⬅ ➡  ★ ⬇ 🛡 🔊 ▦ ⌂ ⚙  |
+--------------------------------------------------------------+
|  [tab] [tab] [tab +]  [tab group chip]                       |
+--------------------------------------------------------------+
|                                                              |
|              WebView content area (main webview)             |
|                                                              |
+--------------------------------------------------------------+
|  status: Ready                                            Win+Linux |
+--------------------------------------------------------------+
```

- Toolbar: brand + version left, URL bar center, action buttons right.
- Action buttons are placeholders for later phases: bookmarks, downloads, ad blocker, volume, split screen, workspaces, dev tools.
- Tab bar above the content area; tab group chips appear in a collapsible left sidebar (Phase 3).

## Sidebar tabs (tab groups) — Phase 3

```
+----------------------------------------------------------+
| ≡  [ tab group sidebar ]            [ URL bar ........ ] |
+----------------------------------------------------------+
|  Home (3)                                                 |
|  Work (5)                                                 |
|  Research (2)                                            |
|  + New group                                             |
+----------------------------------------------------------+
```

- Collapsible left sidebar with group chips.
- Each chip shows name + tab count + color.
- Optionally pinnable.

## Panels (drawers) — later phases

Downloads, bookmarks, history, settings, ad blocker control open as right-side drawers / sidebar tabs, not modals where avoidable.

## Split screen — Phase 5

```
+----------------------------------------------------------+
| toolbar + tab bar (split mode active)                     |
+----------------------------------------------------------+
| pane header        | pane header                          |
| +-----------------+ +----------------------------------+ |
| | webview pane 1  | | webview pane 2                   | |
| +-----------------+ +----------------------------------+ |
+----------------------------------------------------------+
| status bar                                                 |
+----------------------------------------------------------+
```

- 2-pane default on entry; 4-pane via explicit opt-in in layout picker.
- Each pane has its own mini-header + resize handles.
- Each pane is its own webview scoped to its tab.

## Workspaces — Phase 3

Workspace switcher in the sidebar or top-of-tab-bar tray, showing workspace name + tab count. Switching is instant. Each workspace has its own tabs, groups, session state, and storage profile by default in v1.

## Dev tools panel — Phase 6

Right-side tabbed panel:

- Elements
- Console
- Network
- Application (cookies, localStorage, sessionStorage)

Same UI on Windows + Linux; backend protocol differs (CDP vs Web Inspector). Document per-platform availability.
