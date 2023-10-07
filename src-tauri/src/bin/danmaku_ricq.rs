#![feature(lazy_cell)]

use chocho::prelude::*;

use std::{
    sync::{mpsc::Sender, LazyLock, Mutex},
    thread,
};

use chocho::ricq::client::event::GroupMessageEvent;
use danmaku_light::message::Danmaku;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BotConfig {
    pub uin: i64,
    pub url: String,
    pub source_groups: Vec<i64>,
}

struct Handler;

static SENDER: Mutex<Option<Sender<String>>> = Mutex::new(None);
static CONFIG: LazyLock<BotConfig> =
    LazyLock::new(|| confy::load_path("./bot-config.toml").unwrap());

#[async_trait::async_trait]
impl chocho::ricq::handler::PartlyHandler for Handler {
    async fn handle_group_message(&self, GroupMessageEvent { inner, .. }: GroupMessageEvent) {
        tracing::debug!("收到事件：{:?}", inner);

        let group_code = inner.group_code;
        if !CONFIG.source_groups.contains(&group_code) {
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
        if message.is_empty() || message.len() > 150 {
            return;
        }

        let sender = SENDER.lock().unwrap();
        if let Some(sender) = sender.as_ref() {
            if let Err(e) = sender.send(message.to_string()) {
                tracing::error!("发送弹幕失败：{}", e)
            }
        }
    }
}

#[chocho::main(handler = Handler, uin = CONFIG.uin)]
async fn main(client: RQClient) -> anyhow::Result<()> {
    tracing::info!("登录成功，账号：{}", client.uin().await);

    let url = CONFIG.url.clone();

    thread::spawn(|| {
        ws::connect(url, |out| {
            let danmaku = Danmaku {
                text: "客户端已连接".to_string(),
                size: 40,
                color: "hsl(360, 100%, 90%)".to_string(),
            };
            let danmaku = serde_json::to_string(&danmaku).unwrap();
            out.send(danmaku).unwrap();

            let (send, recv) = std::sync::mpsc::channel();
            SENDER.lock().unwrap().replace(send);
            thread::spawn(move || {
                for input in recv.iter() {
                    let danmaku = Danmaku {
                        text: input,
                        size: 30,
                        color: "#FFFFFF".to_string(),
                    };
                    let danmaku = serde_json::to_string(&danmaku).unwrap();
                    out.send(danmaku).unwrap();
                }
            });

            move |msg| {
                if let ws::Message::Text(text) = msg {
                    tracing::info!("{}", text);
                }
                Ok(())
            }
        })
        .unwrap();
    });

    Ok(())
}
