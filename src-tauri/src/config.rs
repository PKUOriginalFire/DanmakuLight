use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub ws_port: u16,
    pub top_padding: i32,
    pub bottom_padding: i32,
    pub left_padding: i32,
    pub right_padding: i32,
    pub bot_config: BotConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotConfig {
    pub enable: bool,
    pub uin: i64,
    pub source_groups: Vec<i64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ws_port: 3210,
            top_padding: 0,
            bottom_padding: 0,
            left_padding: 0,
            right_padding: 0,
            bot_config: BotConfig {
                enable: false,
                uin: 0,
                source_groups: vec![],
            },
        }
    }
}

pub fn get_config_file_path() -> anyhow::Result<std::path::PathBuf> {
    Ok(confy::get_configuration_file_path("danmaku-light", None)?)
}

pub fn load_config() -> anyhow::Result<Config> {
    Ok(confy::load("danmaku-light", None)?)
}
