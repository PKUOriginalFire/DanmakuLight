use chocho::prelude::*;

use std::sync::{Arc, Mutex};

use chocho::ricq::client::event::GroupMessageEvent;
use danmaku_light::{config, message::Danmaku};
use tauri::{AppHandle, Manager};

struct Handler {
    app: Arc<Mutex<AppHandle>>,
    source_groups: Vec<i64>,
}

#[async_trait::async_trait]
impl chocho::ricq::handler::PartlyHandler for Handler {
    async fn handle_group_message(&self, GroupMessageEvent { inner, .. }: GroupMessageEvent) {
        log::debug!("收到事件：{:?}", inner);

        let group_code = inner.group_code;
        if !self.source_groups.contains(&group_code) {
            return;
        }

        let message: Message = inner.elements.into();
        let message = message
            .into_elems()
            .filter_map(|e| match e {
                RQElem::Text(t) => Some(t.content),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("");
        let message = message.trim();
        if message.is_empty() {
            return;
        }

        let app = self.app.lock().unwrap();
        if let Err(e) = app.emit_all(
            "danmaku",
            Danmaku {
                text: message.to_string(),
                size: 30,
                color: "rgb(255,255,255)".to_string(),
            },
        ) {
            log::error!("发送弹幕失败：{}", e);
        }
    }
}

pub fn setup(app: &tauri::App, config: &config::Config) {
    if !config.bot_config.enable {
        return;
    }

    let uin = config.bot_config.uin;
    let source_groups = config.bot_config.source_groups.clone();
    let mut data_folder = app
        .path_resolver()
        .app_local_data_dir()
        .unwrap_or(".".into());
    data_folder.push("bots");
    let data_folder = data_folder.to_string_lossy().to_string();

    let app = Arc::new(Mutex::new(app.handle()));

    tauri::async_runtime::spawn(async move {
        let (client, alive) =
            chocho::login(data_folder, Handler { app, source_groups }, Some(uin), None)
                .await
                .unwrap();

        log::info!("登录成功，账号：{}", client.uin().await);

        tauri::async_runtime::spawn(async {
            alive.auto_reconnect().await.unwrap();
        });
    });
}
