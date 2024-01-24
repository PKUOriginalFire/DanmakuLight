use std::sync::atomic::AtomicBool;

use anyhow::Result;

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

use danmaku_light::{config::get_config_file_path, message::Danmaku};

use crate::config_panel::create_config_panel;

pub fn setup(app: &tauri::App) -> Result<()> {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show_hide", "显示/隐藏").selected())
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("edit_config", "编辑配置文件"))
        .add_item(CustomMenuItem::new("reload_config", "重载配置文件"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("about", "关于"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "退出"));

    let handle = app.handle();
    SystemTray::new()
        .with_menu(tray_menu)
        .on_event(move |event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                let result = match id.as_str() {
                    "show_hide" => show_hide(&handle),
                    "edit_config" => edit_config(&handle),
                    "reload_config" => reload_config(&handle),
                    "about" => about(&handle),
                    "quit" => {
                        handle.exit(0);
                        Ok(())
                    }
                    _ => Ok(()),
                };
                if let Err(e) = result {
                    log::error!("failed to handle system tray event: {}", e);
                }
            }
            else if let SystemTrayEvent::DoubleClick { .. } = event {
                create_config_panel(&handle);
            }
        })
        .build(app)?;
    Ok(())
}

/// 托盘菜单「显示/隐藏」选项。
fn show_hide(app: &tauri::AppHandle) -> Result<()> {
    static IS_SHOWING: AtomicBool = AtomicBool::new(true);
    let item_handle = app.tray_handle().get_item("show_hide");
    if IS_SHOWING.load(std::sync::atomic::Ordering::Relaxed) {
        app.emit_all("hide", ())?;
        IS_SHOWING.store(false, std::sync::atomic::Ordering::Relaxed);
        item_handle.set_selected(false)?;
    } else {
        app.emit_all("show", ())?;
        IS_SHOWING.store(true, std::sync::atomic::Ordering::Relaxed);
        item_handle.set_selected(true)?;
    }
    Ok(())
}

/// 托盘菜单「编辑配置文件」选项。
fn edit_config(_app: &tauri::AppHandle) -> Result<()> {
    let editor = edit::get_editor()?;
    std::process::Command::new(editor)
        .arg(get_config_file_path()?)
        .spawn()?;
    Ok(())
}

/// 托盘菜单「重载配置文件」选项。
fn reload_config(app: &tauri::AppHandle) -> Result<()> {
    app.emit_all("config", ())?;
    Ok(())
}

/// 托盘菜单「关于」选项。
fn about(app: &tauri::AppHandle) -> Result<()> {
    app.emit_all(
        "danmaku",
        Danmaku {
            text: format!(
                "~~Danmaku Light v{} by Original Fire~~",
                app.package_info().version
            ),
            size: 40,
            color: "hsl(360, 100%, 90%)".to_string(),
        },
    )?;
    Ok(())
}
