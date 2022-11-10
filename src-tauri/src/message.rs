use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Danmaku {
    pub text: String,
    pub size: i32,
    pub color: String,
}
