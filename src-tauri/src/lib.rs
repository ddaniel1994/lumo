// LBP Browser - Tauri 2.x backend
//
// This is the v1 shell backend. Feature backends (history, downloads,
// ad blocker, volume, split panes, workspaces, dev tools) are added
// in later phases per REQUIREMENTS.md.

pub mod commands;
pub mod store;

use tauri::{Manager, Emitter};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(|connector, call| match call.command() {
            "shell_version" => commands::shell_version(connector, call),
            "get_app_info" => commands::get_app_info(connector, call),
            _ => Err(tauri::Error::InvalidCommand(call.command().into())),
        })
        .setup(|app| {
            // Placeholder setup. Later phases add:
            // - webview cookie/storage profile handling for workspaces
            // - request interception hooks for ad blocking
            // - audio session handling for volume booster
            // - devtools/protocol hooks

            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").ok();
                let _ = window.as_ref().and_then(|w| {
                    // Enable devtools in dev builds via the webview if supported.
                    let _ = w.eval("console.log('LBP dev mode loaded');");
                    Some(())
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
