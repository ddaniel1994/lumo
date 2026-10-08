use tauri::webview::Url;

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

#[tauri::command]
pub fn navigate(window: tauri::WebviewWindow, url: String) -> Result<(), String> {
    // Basic URL validation - prepend https:// if no protocol
    let final_url = if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else if url.contains(".") {
        format!("https://{}", url)
    } else {
        // Treat as search query
        format!(
            "https://www.google.com/search?q={}",
            url.replace(" ", "+")
        )
    };

    match Url::parse(&final_url) {
        Ok(parsed_url) => {
            if let Err(e) = window.navigate(parsed_url) {
                return Err(e.to_string());
            }
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn go_back(_window: tauri::WebviewWindow) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn go_forward(_window: tauri::WebviewWindow) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn reload(_window: tauri::WebviewWindow) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_current_url(_window: tauri::WebviewWindow) -> Result<String, String> {
    Ok(String::new())
}

#[tauri::command]
pub fn get_title(_window: tauri::WebviewWindow) -> Result<String, String> {
    Ok(String::new())
}
