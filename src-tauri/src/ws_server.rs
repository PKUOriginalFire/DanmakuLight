use std::error::Error;

use std::sync::Arc;
use parking_lot::Mutex;
// use std::time::Duration;

use anyhow::Result;
use danmaku_light::message::Danmaku;
use tauri::Manager;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
use futures_util::StreamExt;

async fn handle_conn(stream: TcpStream, app: Arc<Mutex<tauri::AppHandle>>) -> Result<()> {
    let wsstream = tokio_tungstenite::accept_async(stream).await?;
    
    let (_, mut wsin) = wsstream.split();
    // let mut interval = tokio::time::interval(Duration::from_millis(500));
    
    fn wrap_ws_err(err: impl Error + Sync + Send + 'static) -> ws::Error {
        let details = err.to_string();
        ws::Error::new(ws::ErrorKind::Custom(Box::new(err)), details)
    }
    loop {
        tokio::select! {
            msg = wsin.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if let Message::Text(text) = msg {
                            let danmaku: Danmaku = serde_json::from_str(&text).map_err(wrap_ws_err)?;
                            app.lock()
                                .emit_all("danmaku", danmaku)
                                .map_err(wrap_ws_err)?;
                        }
                    },
                    None => break,
                }
            }
            // _ = interval.tick() => {
            //     if check_stop() {
            //         break;
            //     }
            // }
        }
    }
    
    Ok(())
}

pub fn setup(app: &tauri::App, port: u16) {
    let app = Arc::new(Mutex::new(app.handle()));
    let address = format!("127.0.0.1:{}", port);
    
    let _ = tauri::async_runtime::spawn(async move {
        let result = TcpListener::bind(address).await;
        if let Ok(server) = result {
            let server = Arc::new(server);
            
            // let mut interval = tokio::time::interval(Duration::from_millis(500));
            loop {
                tokio::select! {
                    Ok((stream, _addr)) = server.accept() => {
                        tauri::async_runtime::spawn(handle_conn(stream, app.clone()));
                    }
                    // _ = interval.tick() => {
                    //     if check_stop() {
                    //         break;
                    //     }
                    // }
                }
            }
        } else {
            unsafe {
                log::error!("Failed to start the websocket server: {:?}", result.unwrap_err_unchecked());
            }
        }
    });
}
