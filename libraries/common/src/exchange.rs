use std::collections::HashMap;

use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Exchange {
    pub timestamp: NaiveDateTime,
    pub topic: String,
    pub tenant: Option<String>,
    pub headers: HashMap<String, String>,
    pub message: Vec<u8>,
}

impl Default for Exchange {
    fn default() -> Self {
        Self {
            timestamp: Local::now().naive_local(),
            topic: Default::default(),
            headers: Default::default(),
            message: Default::default(),
            tenant: Default::default(),
        }
    }
}

impl Exchange {
    pub fn new(message: &[u8], topic: &str, tenant: Option<String>) -> Exchange {
        Exchange {
            topic: topic.into(),
            message: message.to_vec(),
            tenant,
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

    pub fn get_message_as_string(msg: &[u8]) -> String {
        let s = String::from_utf8_lossy(msg);
        s.to_string()
    }
}
