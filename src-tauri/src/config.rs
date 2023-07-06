use std::sync::Arc;

use once_cell::sync::OnceCell;
use parking_lot::{Mutex, MutexGuard};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub ws_port: u16,
    pub top_padding: i32,
    pub bottom_padding: i32,
    pub left_padding: i32,
    pub right_padding: i32,
    pub bot_config: BotConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigPatch {
    pub ws_port: Option<u16>,
    pub top_padding: Option<i32>,
    pub bottom_padding: Option<i32>,
    pub left_padding: Option<i32>,
    pub right_padding: Option<i32>,
    pub bot_config: Option<BotConfig>,
}

impl From<Config> for ConfigPatch {
    fn from(val: Config) -> Self {
        ConfigPatch {
            ws_port: Some(val.ws_port),
            top_padding: Some(val.top_padding),
            bottom_padding: Some(val.bottom_padding),
            left_padding: Some(val.left_padding),
            right_padding: Some(val.right_padding),
            bot_config: Some(val.bot_config),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl Config {
    pub fn new() -> Self {
        match load_config() {
            Ok(res) => res,
            Err(err) => {
                log::error!("Failed to load config: {}", err);
                Config::default()
            },
        }
    }
    
    #[allow(dead_code)]
    pub fn patch(&mut self, config_patch: ConfigPatch) {
        macro_rules! try_patch {
            ($key: tt) => {
                if config_patch.$key.is_some() {
                    self.$key = config_patch.$key.unwrap();
                }
            };
        }
        
        try_patch!(ws_port);
        try_patch!(top_padding);
        try_patch!(bottom_padding);
        try_patch!(left_padding);
        try_patch!(right_padding);
        try_patch!(bot_config);
    }
}

#[derive(Debug, Clone)]
pub struct ConfigContainer {
    inner: Arc<Mutex<Config>>,
}

// 感觉留着这玩意在这早晚得写出死锁来，还好这程序没有啥继续扩张的必要
impl ConfigContainer {
    pub fn content(&self) -> Config {
        self.inner.lock().clone()
    }
    pub fn get_mut(&self) -> MutexGuard<Config> { 
        self.inner.lock()
    }
}

pub fn global_config() -> &'static ConfigContainer {
    static CONFIG: OnceCell<ConfigContainer> = OnceCell::new();
    CONFIG.get_or_init(|| ConfigContainer {
        inner: Arc::new(Mutex::new(Config::new()))
    })
}

pub fn get_config_file_path() -> anyhow::Result<std::path::PathBuf> {
    Ok(confy::get_configuration_file_path("danmaku-light", None)?)
}

pub fn load_config() -> anyhow::Result<Config> {
    Ok(confy::load("danmaku-light", None)?)
}

pub fn save_config() -> anyhow::Result<()> {
    Ok(confy::store("danmaku-light", None, global_config().content())?)
}