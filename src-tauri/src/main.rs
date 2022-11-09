#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tauri_plugin_log::{LogTarget, LoggerBuilder};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Danmaku {
    text: String,
    size: i32,
    color: String,
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn Error>> {
    let window = app.get_window("main").ok_or("failed to get window")?;
    window.set_ignore_cursor_events(true)?;

    start_ws_server(app);

    Ok(())
}

fn start_ws_server(app: &tauri::App) {
    fn wrap_ws_err(err: impl Error + Sync + Send + 'static) -> ws::Error {
        let details = err.to_string();
        ws::Error::new(ws::ErrorKind::Custom(Box::new(err)), details)
    }

    let app = Arc::new(Mutex::new(app.handle()));
    thread::spawn(move || {
        ws::listen("127.0.0.1:3210", |_| {
            let app = app.clone();
            move |msg| {
                if let ws::Message::Text(text) = msg {
                    let danmaku: Danmaku = serde_json::from_str(&text).map_err(wrap_ws_err)?;
                    app.lock()
                        .unwrap()
                        .emit_all("danmaku", danmaku)
                        .map_err(wrap_ws_err)?;
                }
                Ok(())
            }
        })
        .unwrap()
    });
}

/// 托盘菜单「关于」选项。
fn about(app: &tauri::AppHandle) {
    app.emit_all(
        "danmaku",
        Danmaku {
            text: format!("~~Danmaku Light v{} by Original Fire~~", app.package_info().version),
            size: 40,
            color: "hsl(360, 100%, 90%)".to_string(),
        },
    )
    .unwrap();
}

fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("about", "关于"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "退出"));

    let logger = LoggerBuilder::default()
        .targets([LogTarget::LogDir, LogTarget::Stdout])
        .build();

    tauri::Builder::default()
        .setup(setup)
        .plugin(logger)
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "quit" => app.exit(0),
                    "about" => about(app),
                    _ => {}
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
