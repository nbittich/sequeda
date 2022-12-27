use chrono::NaiveDateTime;
use sequeda_service_common::IdGenerator;
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
            id: IdGenerator.get(),
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
