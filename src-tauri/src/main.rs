#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::{anyhow, Result};

use tauri::Manager;

use tauri_plugin_log::{Builder as LoggerBuilder, LogTarget};

mod tray;
mod ws_server;

fn setup(app: &mut tauri::App) -> Result<()> {
    let window = app
        .get_window("main")
        .ok_or_else(|| anyhow!("failed to get window"))?;
    window.set_ignore_cursor_events(true)?;

    ws_server::setup(app);
    tray::setup(app)?;

    Ok(())
}

fn main() {
    let logger = LoggerBuilder::default()
        .targets([LogTarget::LogDir, LogTarget::Stdout])
        .build();

    tauri::Builder::default()
        .setup(|app| Ok(setup(app)?))
        .plugin(logger)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
