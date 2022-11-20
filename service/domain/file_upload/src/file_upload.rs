use std::{error::Error, fmt::Display, path::PathBuf};

use chrono::{Local, NaiveDateTime};
use sequeda_store::{Repository, StoreRepository};
use serde::{Deserialize, Serialize};

pub const SHARE_DRIVE_PATH: &str = "SHARE_DRIVE_PATH";

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUpload {
    #[serde(rename = "_id")]
    pub id: String,
    pub creation_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
    pub content_type: Option<String>,
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
            id: uuid::Uuid::new_v4().to_string(),
            content_type: Default::default(),
            original_filename: Default::default(),
            internal_name: Default::default(),
            extension: Default::default(),
            creation_date: Local::now().naive_local(),
            updated_date: Default::default(),
            size: Default::default(),
            public_resource: Default::default(),
            correlation_id: Default::default(),
        }
    }
}

impl FileUpload {
    pub async fn upload(
        &mut self,
        file_handle: Option<&[u8]>,
        store: &StoreRepository<FileUpload>,
    ) -> Result<(), ServiceError> {
        let share_drive_path: String =
            std::env::var(SHARE_DRIVE_PATH).map_err(|e| ServiceError::from(&e))?;
        let upload = store
            .find_by_id(&self.id)
            .await
            .map_err(|e| ServiceError::from(&e))?;

        let old_internal_name = if let Some(upload) = upload {
            if file_handle.is_some() {
                Some(upload.internal_name)
            } else {
                None
            }
        } else {
            None
        };

        if let Some(file_handle) = file_handle {
            let extension = &self.extension;
            let internal_name = format!(
                "{}.{}",
                self.id,
                extension.as_ref().cloned().unwrap_or_else(|| "".into())
            );
            tokio::fs::write(
                PathBuf::from(&share_drive_path).join(&internal_name),
                file_handle,
            )
            .await
            .map_err(|e| ServiceError::from(&e))?;
            self.internal_name = internal_name;

            store
                .update(&self.id, self)
                .await
                .map_err(|e| ServiceError::from(&e))?;

            if let Some(old_internal_name) = old_internal_name {
                self.updated_date = Some(Local::now().naive_local());
                // override file
                tracing::info!("removing old file {}", old_internal_name);
                tokio::fs::remove_file(PathBuf::from(&share_drive_path).join(old_internal_name))
                    .await
                    .map_err(|e| ServiceError::from(&e))?;
                store
                    .update(&self.id, self)
                    .await
                    .map_err(|e| ServiceError::from(&e))?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ServiceError(String);

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for ServiceError {}

impl ServiceError {
    fn from(e: &dyn Error) -> Self {
        ServiceError(e.to_string())
    }
}
