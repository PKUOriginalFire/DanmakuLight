use std::sync::Arc;
use parking_lot::{Mutex, MutexGuard};

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigPatch {
    pub ws_port: Option<u16>,
    pub top_padding: Option<i32>,
    pub bottom_padding: Option<i32>,
    pub left_padding: Option<i32>,
    pub right_padding: Option<i32>,
}

impl From<Config> for ConfigPatch {
    fn from(val: Config) -> Self {
        ConfigPatch {
            ws_port: Some(val.ws_port),
            top_padding: Some(val.top_padding),
            bottom_padding: Some(val.bottom_padding),
            left_padding: Some(val.left_padding),
            right_padding: Some(val.right_padding),
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
    }
}

#[derive(Debug, Clone)]
pub struct ConfigContainer {
    inner: Arc<Mutex<Config>>,
}

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
    Ok(std::path::PathBuf::from("./danmaku-light.toml"))
}

pub fn load_config() -> anyhow::Result<Config> {
    Ok(confy::load_path(get_config_file_path()?)?)
}

pub fn save_config() -> anyhow::Result<()> {
    confy::store_path(get_config_file_path()?, global_config().content())?;
    Ok(())
}