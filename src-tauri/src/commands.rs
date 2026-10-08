use tauri::State;

#[derive(serde::Serialize)]
pub struct AppInfo {
    pub name: &'static str,
    pub version: &'static str,
}

#[tauri::command]
pub fn shell_version() -> String {
    "LBP shell".into()
}

#[tauri::command]
pub fn get_app_info() -> AppInfo {
    AppInfo {
        name: "LBP Browser",
        version: env!("CARGO_PKG_VERSION"),
    }
}
