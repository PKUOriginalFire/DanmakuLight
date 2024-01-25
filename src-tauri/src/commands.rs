use anyhow::Result;
use tauri::Manager;
use crate::config::{Config, ConfigPatch, global_config};

type CommandResult<T = ()> = Result<T, String>;

#[tauri::command]
pub async fn patch_config(app: tauri::AppHandle, patch: ConfigPatch) -> CommandResult {
    let need_restart_ws = patch.ws_port.is_some();
    global_config().get_mut().patch(patch);
    let _ = &app.emit_all("reload_config", ());
    crate::config::save_config().map_err(|e|{e.to_string()})?;
    if need_restart_ws {
        crate::ws_server::setup(&app, global_config().content().ws_port).map_err(|e|{e.to_string()})?;
    }
    Ok(())
}

#[tauri::command]
pub async fn save_config() -> CommandResult {
    crate::config::save_config().map_err(|e|{e.to_string()})
}

#[tauri::command]
pub async fn get_current_config() -> CommandResult<Config> {
    Ok(global_config().content())
}

// 重启websocket服务
// #[tauri::command]
// pub async fn reload_ws(app: tauri::AppHandle) -> CommandResult {
//     crate::ws_server::setup(app.clone(), global_config().content().ws_port);
//     Ok(())
// }