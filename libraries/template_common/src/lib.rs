use std::fmt::Display;

use ::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    #[serde(rename = "_id")]
    pub id: String,
    pub creation_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
    pub file_id: String,
    pub template_type: TemplateType,
    pub template_context: Context,
    pub title: String,
    pub description: Option<String>,
}
#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TemplateType {
    Html,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Context {
    Invoice,
}
impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self)
            .map(|s| write!(f, "{s}"))
            .unwrap_or_else(|e| {
                tracing::error!("could not fmt enum {e}");
                Err(std::fmt::Error)
            })
    }
}
#[derive(Deserialize)]
pub struct ContextQuery {
    pub context: Context,
}
#[derive(Serialize, Deserialize)]
pub struct RenderRequest {
    pub template_id: String,
    pub context: serde_json::Value,
    pub file_name: String,
    pub template_context: Context,
}
