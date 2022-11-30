use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLog {
    #[serde(rename = "_id")]
    pub id: String,
    pub received_date: NaiveDateTime,
    pub message: String,
}

impl Default for AuditLog {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            received_date: Default::default(),
            message: Default::default(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogConfig {
    pub topic: String,
    pub header_message: Option<String>,
}
