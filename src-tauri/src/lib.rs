// LBP Browser - Tauri 2.x backend
//
// This is the v1 shell backend. Feature backends (history, downloads,
// ad blocker, volume, split panes, workspaces, dev tools) are added
// in later phases per REQUIREMENTS.md.

pub mod commands;
pub mod store;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::shell_version,
            commands::get_app_info,
            commands::navigate,
            commands::go_back,
            commands::go_forward,
            commands::reload,
            commands::get_current_url,
            commands::get_title,
        ])
        .setup(|_app| {
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
