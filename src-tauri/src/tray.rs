use std::sync::atomic::AtomicBool;

use anyhow::Result;

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

use danmaku_light::message::Danmaku;

pub fn setup(app: &tauri::App) -> Result<()> {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show_hide", "显示/隐藏").selected())
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
                    "quit" => {
                        handle.exit(0);
                        Ok(())
                    }
                    "about" => about(&handle),
                    "show_hide" => show_hide(&handle),
                    _ => Ok(()),
                };
                if let Err(e) = result {
                    log::error!("failed to handle system tray event: {}", e);
                }
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
