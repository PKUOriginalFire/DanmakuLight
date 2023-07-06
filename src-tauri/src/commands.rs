use anyhow::Result;

use crate::config::{Config, ConfigPatch, global_config};

type CommandResult<T = ()> = Result<T, String>;

#[tauri::command]
pub async fn patch_config(patch: ConfigPatch) -> CommandResult {
    global_config().get_mut().patch(patch);
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
#[tauri::command]
pub async fn reload_ws(app: tauri::AppHandle) -> CommandResult {
    crate::ws_server::setup(app.clone(), global_config().content().ws_port);
    Ok(())
}