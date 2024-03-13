use std::error::Error;

use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;


use anyhow::Result;
use danmaku_light::message::Danmaku;
use tauri::Manager;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{self, Sender, Receiver};
use tokio_tungstenite::tungstenite::Message;
use futures_util::StreamExt;


struct WsWatcher {
    sender: Sender<bool>,
}
struct WsWatcherContainer {
    watcher: Arc<Mutex<WsWatcher>>,
    // receiver: Receiver<bool>,
}
impl WsWatcherContainer {
    fn new() -> Self {
        let (tx, _rx) = broadcast::channel::<bool>(32);
        Self {
            watcher: Arc::new(Mutex::new(WsWatcher { sender: tx })),
            // receiver: rx
        }
    }
    pub fn signal(&self) {
        let _ = self.watcher.lock().sender.send(true);
    }
    pub fn get_receiver(&self) -> Receiver<bool> {
        self.watcher.lock().sender.subscribe()
    }
}
fn global_ws_watcher() -> &'static WsWatcherContainer {
    static WATCHER: OnceCell<WsWatcherContainer> = OnceCell::new();
    WATCHER.get_or_init(|| WsWatcherContainer::new())
}

async fn handle_conn(stream: TcpStream, app: Arc<Mutex<tauri::AppHandle>>) -> Result<()> {
    let wsstream = tokio_tungstenite::accept_async(stream).await?;
    
    let (_, mut wsin) = wsstream.split();
    let mut rx = global_ws_watcher().get_receiver();
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
            _ = rx.recv() => {
                break;
            }
        }
    }
    
    Ok(())
}

pub fn setup(app: &tauri::AppHandle, port: u16) -> Result<()> {
    global_ws_watcher().signal();
    // 结果还是要sleep
    sleep(Duration::from_millis(500));
    let app = Arc::new(Mutex::new(app.clone()));
    let address = format!("127.0.0.1:{}", port);
    let mut rx = global_ws_watcher().get_receiver();
    let _ = tauri::async_runtime::spawn(async move {
        let result = TcpListener::bind(address).await;
        if let Ok(server) = result {
            let server = Arc::new(server);
            
            loop {
                tokio::select! {
                    Ok((stream, _addr)) = server.accept() => {
                        tauri::async_runtime::spawn(handle_conn(stream, app.clone()));
                    }
                    _ = rx.recv() => {
                        break;
                    }
                }
            }
        } else {
            unsafe {
                log::error!("Failed to start the websocket server: {:?}", result.unwrap_err_unchecked());
            }
        }
    });
    Ok(())
}
