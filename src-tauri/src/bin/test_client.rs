use std::thread;

use danmaku_light::message::Danmaku;

fn main() -> anyhow::Result<()> {
    let url = std::env::var("WS_URL")?;

    ws::connect(url, |out| {
        let danmaku = Danmaku {
            text: "客户端已连接".to_string(),
            size: 40,
            color: "hsl(360, 100%, 90%)".to_string(),
        };
        let danmaku = serde_json::to_string(&danmaku).unwrap();
        out.send(danmaku).unwrap();

        let (send, recv) = std::sync::mpsc::channel();
        thread::spawn(move || {
            let mut input = String::new();
            loop {
                std::io::stdin().read_line(&mut input).unwrap();
                send.send(input.clone()).unwrap();
                input.clear();
            }
        });
        thread::spawn(move || {
            for input in recv.iter() {
                let danmaku = Danmaku {
                    text: input,
                    size: 40,
                    color: "#FFFFFF".to_string(),
                };
                let danmaku = serde_json::to_string(&danmaku).unwrap();
                out.send(danmaku).unwrap();
            }
        });

        move |msg| {
            if let ws::Message::Text(text) = msg {
                println!("{}", text);
            }
            Ok(())
        }
    })?;

    Ok(())
}
