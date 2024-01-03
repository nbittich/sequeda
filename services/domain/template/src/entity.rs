use std::fmt::Display;

use chrono::{Local, NaiveDateTime};
use sequeda_service_common::IdGenerator;
use serde::{de, Deserialize, Serialize};

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

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateUpsert {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub template_type: TemplateType,
    pub template_context: Context,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TemplateType {
    Html,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
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
                Err(std::fmt::Error::default())
            })
    }
}

impl Default for Template {
    fn default() -> Self {
        Self {
            id: IdGenerator.get(),
            creation_date: Local::now().naive_local(),
            updated_date: Default::default(),
            file_id: Default::default(),
            template_type: TemplateType::Html,
            template_context: Context::Invoice,
            title: Default::default(),
            description: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RenderRequest {
    pub template_id: String,
    pub context: serde_json::Value,
    pub file_name: String,
    pub corelation_id: String,
}
