use std::collections::HashMap;

use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Exchange {
    timestamp: i64,
    topic: String,
    headers: HashMap<String, String>,
    message: String,
}

impl Default for Exchange {
    fn default() -> Self {
        Self {
            timestamp: Local::now().timestamp(),
            topic: Default::default(),
            headers: Default::default(),
            message: Default::default(),
        }
    }
}

impl Exchange {
    pub fn get_topic(&self) -> &str {
        &self.topic
    }
    pub fn new(message: &str, topic: &str) -> Exchange {
        Exchange {
            topic: topic.to_string(),
            message: message.to_string(),
            ..Default::default()
        }
    }
    pub fn deserialize(s: &[u8]) -> Result<Exchange, Box<bincode::ErrorKind>> {
        bincode::deserialize(s)
    }
    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, Box<bincode::ErrorKind>> {
        bincode::serialize(&self)
    }
}
