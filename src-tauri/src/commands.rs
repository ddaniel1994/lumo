use tauri::State;
use tauri::webview::WebviewUrl;

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
pub fn navigate(app_handle: tauri::AppHandle, url: String) -> Result<(), String> {
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

    if let Some(window) = app_handle.webview_window("main") {
        let _ = window.set_url(final_url.as_str());
        Ok(())
    } else {
        Err("Main window not found".into())
    }
}

#[tauri::command]
pub fn go_back(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.webview_window("main") {
        let _ = window.eval("history.back()");
        Ok(())
    } else {
        Err("Main window not found".into())
    }
}

#[tauri::command]
pub fn go_forward(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.webview_window("main") {
        let _ = window.eval("history.forward()");
        Ok(())
    } else {
        Err("Main window not found".into())
    }
}

#[tauri::command]
pub fn reload(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.webview_window("main") {
        let _ = window.eval("location.reload()");
        Ok(())
    } else {
        Err("Main window not found".into())
    }
}

#[tauri::command]
pub fn get_current_url(app_handle: tauri::AppHandle) -> Result<String, String> {
    if let Some(window) = app_handle.webview_window("main") {
        let result = window.eval::<String>("location.href").unwrap_or_default();
        Ok(result)
    } else {
        Err("Main window not found".into())
    }
}

#[tauri::command]
pub fn get_title(app_handle: tauri::AppHandle) -> Result<String, String> {
    if let Some(window) = app_handle.webview_window("main") {
        let result = window.eval::<String>("document.title").unwrap_or_else(|_| "Loading...".to_string());
        Ok(result)
    } else {
        Err("Main window not found".into())
    }
}
