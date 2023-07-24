use tauri::{AppHandle, Manager};

pub fn create_config_panel(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_window("config_panel") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }
    
    let builder = tauri::window::WindowBuilder::new(
        app_handle,
        String::from("config_panel"),
        tauri::WindowUrl::App("config_panel".into())
    )
    .center()
    .title("DanmakuLight Config Panel")
    // .decorations(false)
    // .transparent(true)
    .min_inner_size(500.0, 400.0);

    match builder.visible(false).build() {
        Ok(_) => {
            let app_handle = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                
                if let Some(window) = app_handle.get_window("config_panel") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            });
        }
        Err(err) => {
            log::error!("Failed to create config panel: {}", err);
        }
    }
}