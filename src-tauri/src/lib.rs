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
            "navigate" => commands::navigate(connector, call),
            "go_back" => commands::go_back(connector, call),
            "go_forward" => commands::go_forward(connector, call),
            "reload" => commands::reload(connector, call),
            "get_current_url" => commands::get_current_url(connector, call),
            "get_title" => commands::get_title(connector, call),
            _ => Err(tauri::Error::InvalidCommand(call.command().into())),
        })
        .setup(|app| {
            // Enable devtools in dev builds
            #[cfg(debug_assertions)]
            {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.eval("console.log('LBP dev mode loaded');");
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
