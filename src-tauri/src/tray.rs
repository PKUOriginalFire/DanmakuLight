use anyhow::Result;

use tauri::{menu::MenuBuilder, tray::TrayIconBuilder, Manager};

use danmaku_light::{config::get_config_file_path, message::Danmaku};

pub fn setup(app: &tauri::App) -> Result<()> {
    let handle = app.handle();
    let tray_menu = MenuBuilder::new(handle)
        .check("show_hide", "显示/隐藏")
        .separator()
        .text("edit_config", "编辑配置文件")
        .text("reload_config", "重载配置文件")
        .separator()
        .text("about", "关于")
        .separator()
        .text("quit", "退出")
        .build()?;

    TrayIconBuilder::new()
        .menu(&tray_menu)
        .on_menu_event(move |handle, event| {
            let id = event.id();
            if let Some(item) = tray_menu.get(id) {
                let result = match id.as_ref() {
                    "show_hide" => show_hide(
                        handle,
                        item.as_check_menuitem()
                            .unwrap()
                            .is_checked()
                            .unwrap_or_default(),
                    ),
                    "edit_config" => edit_config(handle),
                    "reload_config" => reload_config(handle),
                    "about" => about(handle),
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
        })
        .build(app)?;
    Ok(())
}

/// 托盘菜单「显示/隐藏」选项。
fn show_hide(app: &tauri::AppHandle, checked: bool) -> Result<()> {
    if !checked {
        app.emit_all("hide", ())?;
    } else {
        app.emit_all("show", ())?;
    }
    Ok(())
}

/// 托盘菜单「编辑配置文件」选项。
fn edit_config(_app: &tauri::AppHandle) -> Result<()> {
    edit::edit_file(get_config_file_path()?)?;
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
