use serde::{Deserialize, Serialize};

pub mod exchange;

#[derive(Debug, Serialize, Deserialize)]
pub enum TextMessage {
    Connect(String),
    Subscribe(String),
}

impl TextMessage {
    pub fn deserialize(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}
