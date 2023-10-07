use std::error::Error;

use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::Result;
use danmaku_light::config::Config;
use danmaku_light::message::Danmaku;
use tauri::Manager;

pub fn setup(app: &tauri::App, config: &Config) {
    fn wrap_ws_err(err: impl Error + Sync + Send + 'static) -> ws::Error {
        let details = err.to_string();
        ws::Error::new(ws::ErrorKind::Custom(Box::new(err)), details)
    }

    let app = Arc::new(Mutex::new(app.handle().clone()));
    let address = format!("127.0.0.1:{}", config.ws_port);
    thread::spawn(move || {
        ws::listen(address, |_| {
            let app = app.clone();
            move |msg| -> Result<(), ws::Error> {
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
