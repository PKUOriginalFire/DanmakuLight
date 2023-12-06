use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub ws_port: u16,
    pub top_padding: i32,
    pub bottom_padding: i32,
    pub left_padding: i32,
    pub right_padding: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ws_port: 3210,
            top_padding: 0,
            bottom_padding: 0,
            left_padding: 0,
            right_padding: 0,
        }
    }
}

pub fn get_config_file_path() -> anyhow::Result<std::path::PathBuf> {
    Ok(std::path::PathBuf::from("./danmaku-light.toml"))
}

pub fn load_config() -> anyhow::Result<Config> {
    Ok(confy::load_path(get_config_file_path()?)?)
}
