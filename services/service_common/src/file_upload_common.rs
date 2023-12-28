use axum::async_trait;
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use tokio::fs::File;

use crate::{common_domain_types::ServiceError, IdGenerator};

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
    pub size: usize,
    pub public_resource: bool,
    pub correlation_id: Option<String>,
}

impl Default for FileUpload {
    fn default() -> Self {
        Self {
            id: IdGenerator.get(),
            content_type: Default::default(),
            original_filename: Default::default(),
            internal_name: Default::default(),
            extension: Default::default(),
            creation_date: Local::now().naive_local(),
            updated_date: Default::default(),
            thumbnail_id: Default::default(),
            size: Default::default(),
            public_resource: Default::default(),
            correlation_id: Default::default(),
        }
    }
}

impl FileUpload {
    pub fn is_image(&self) -> bool {
        self.content_type
            .as_ref()
            .filter(|ct| ct.starts_with("image"))
            .is_some()
    }
}

#[async_trait]
pub trait IFileService {
    async fn upload(
        &self,
        upl: FileUpload,
        bytes: Option<&[u8]>,
    ) -> Result<FileUpload, ServiceError>;
    async fn download(&self, upl: &FileUpload) -> Result<File, ServiceError>;
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
