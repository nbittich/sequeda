use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUpload {
    #[serde(rename = "_id")]
    pub id: String,
    pub creation_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
    pub content_type: Option<String>,
    pub thumbnail_id: Option<String>,
    pub original_filename: String,
    pub internal_name: String,
    pub extension: Option<String>,
    pub size: u64,
    pub public_resource: bool,
    pub correlation_id: Option<String>,
}

impl FileUpload {
    pub fn is_image(&self) -> bool {
        self.content_type
            .as_ref()
            .filter(|ct| ct.starts_with("image"))
            .is_some()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadFileRequestUriParams {
    pub correlation_id: Option<String>,
    pub id: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadFileRequestUriParams {
    pub id: String,
}
