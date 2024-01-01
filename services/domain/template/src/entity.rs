use chrono::{Local, NaiveDateTime};
use sequeda_service_common::IdGenerator;
use serde::{Deserialize, Serialize};

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
