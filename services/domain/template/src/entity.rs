use std::ops::{Deref, DerefMut};

use chrono::Local;
use sequeda_service_common::IdGenerator;
use sequeda_template_common::{Context, Template, TemplateType};
use serde::{Deserialize, Serialize};

pub struct TemplateWrapper(pub Template);

impl Deref for TemplateWrapper {
    type Target = Template;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for TemplateWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
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

impl Default for TemplateWrapper {
    fn default() -> Self {
        Self(Template {
            id: IdGenerator.get(),
            creation_date: Local::now().naive_local(),
            updated_date: Default::default(),
            file_id: Default::default(),
            template_type: TemplateType::Html,
            template_context: Context::Invoice,
            title: Default::default(),
            description: Default::default(),
        })
    }
}
