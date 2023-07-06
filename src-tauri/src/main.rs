#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::{anyhow, Result};

use commands::*;
use config::{load_config, Config};
use tauri::Manager;

use tauri_plugin_log::{Builder as LoggerBuilder, LogTarget};

mod tray;
mod ws_server;
mod ricq_backend;
mod commands;
mod config;
mod message;

fn setup(app: &mut tauri::App) -> Result<()> {
    let window = app
        .get_window("main")
        .ok_or_else(|| anyhow!("failed to get window"))?;
    window.set_ignore_cursor_events(true)?;

    let config = load_config()?;

    // ws_server::setup(app, &config);
    tray::setup(app)?;
    ws_server::setup(app.handle().clone(), config.ws_port);
    ricq_backend::setup(app, &config);

    Ok(())
}

#[tauri::command]
fn get_config() -> Result<Config, String> {
    load_config().map_err(|e| e.to_string())
}

fn main() {
    let logger = LoggerBuilder::default()
        .targets([LogTarget::LogDir, LogTarget::Stdout])
        .build();

    tauri::Builder::default()
        .setup(|app| Ok(setup(app)?))
        .plugin(logger)
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            patch_config,
            get_current_config,
            reload_ws,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
