use anyhow::Result;

use crate::config::Config;

use crate::config::global_config;

type CommandResult<T = ()> = Result<T, String>;

#[tauri::command]
pub async fn save_config() -> CommandResult {
    // TODO
    Ok(())
}

#[tauri::command]
pub async fn get_current_config() -> CommandResult<Config> {
    Ok(global_config().content())
}

// 重启所有服务（感觉不如直接重启应用）
#[tauri::command]
pub async fn reload_all() -> CommandResult {
    // TODO
    Ok(())
}

// 重启websocket服务
#[tauri::command]
pub async fn reload_ws() -> CommandResult {
    // TODO
    Ok(())
}

// 重启ricq服务
#[tauri::command]
pub async fn reload_ricq() -> CommandResult {
    // TODO
    Ok(())
}
