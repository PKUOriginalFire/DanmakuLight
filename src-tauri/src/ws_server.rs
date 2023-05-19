use std::error::Error;

use std::sync::{Arc, Mutex};
use std::thread;

use tauri::Manager;

use danmaku_light::message::Danmaku;

pub fn setup(app: &tauri::App) {
    fn wrap_ws_err(err: impl Error + Sync + Send + 'static) -> ws::Error {
        let details = err.to_string();
        ws::Error::new(ws::ErrorKind::Custom(Box::new(err)), details)
    }

    let app = Arc::new(Mutex::new(app.handle()));
    thread::spawn(move || {
        ws::listen("127.0.0.1:3210", |_| {
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
