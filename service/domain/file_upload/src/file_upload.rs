use std::{error::Error, fmt::Display, path::PathBuf};

use chrono::{Local, NaiveDateTime};
use image::ImageFormat;
use sequeda_store::{Repository, StoreRepository};
use serde::{Deserialize, Serialize};
use tokio::fs::File;

pub const SHARE_DRIVE_PATH: &str = "SHARE_DRIVE_PATH";

const THUMB_HEIGHT: u32 = 300;
const THUMB_WIDTH: u32 = 300;

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
            id: uuid::Uuid::new_v4().to_string(),
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
    pub async fn upload(
        &mut self,
        share_drive_path: &str,
        file_handle: Option<&[u8]>,
        store: &StoreRepository<FileUpload>,
    ) -> Result<(), ServiceError> {
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

            let old_thumbnail_id = &self.thumbnail_id.clone();

            self.thumbnail_id = self
                .make_thumbnail(&internal_name, file_handle, store, share_drive_path)
                .await?;

            self.internal_name = internal_name;

            if let Some(old_internal_name) = old_internal_name {
                self.updated_date = Some(Local::now().naive_local());
                // override file
                tracing::info!("removing old file {}", old_internal_name);
                tokio::fs::remove_file(PathBuf::from(&share_drive_path).join(&old_internal_name))
                    .await
                    .map_err(|e| ServiceError::from(&e))?;
                if let Some(old_thumbnail_id) = old_thumbnail_id {
                    store
                        .delete_by_id(old_thumbnail_id)
                        .await
                        .map_err(|e| ServiceError::from(&e))?;

                    tracing::info!("removing old thumbnail {}", old_thumbnail_id);
                    tokio::fs::remove_file(
                        PathBuf::from(&share_drive_path).join(format!("thumb-{old_internal_name}")),
                    )
                    .await
                    .map_err(|e| ServiceError::from(&e))?;
                }
            }

            store
                .update(&self.id, self)
                .await
                .map_err(|e| ServiceError::from(&e))?;
        }

        Ok(())
    }

    pub fn is_image(&self) -> bool {
        self.content_type
            .as_ref()
            .filter(|ct| ct.starts_with("image"))
            .is_some()
    }

    async fn make_thumbnail(
        &self,
        internal_name: &str,
        file_handle: &[u8],
        store: &StoreRepository<FileUpload>,
        share_drive_path: &str,
    ) -> Result<Option<String>, ServiceError> {
        if self.is_image() {
            let image = image::load_from_memory(file_handle).map_err(|e| ServiceError::from(&e))?;
            let thumb = image::imageops::thumbnail(&image, THUMB_WIDTH, THUMB_HEIGHT);

            let Some(ct) = &self.content_type  else {
                return Err(ServiceError("No Content type! Should not happen".into()))
            };

            let Some(image_format) = ImageFormat::from_mime_type(&ct) else {
                return Err(ServiceError("Format cannot be transformed to thumbnail".into()))
            };

            tracing::debug!("generate thumbnail...");
            let thumbnail = Self {
                content_type: self.content_type.clone(),
                thumbnail_id: None,
                original_filename: format!("thumb-{internal_name}"),
                internal_name: format!("thumb-{internal_name}"),
                extension: self.extension.clone(),
                size: thumb.len(),
                public_resource: self.public_resource,
                correlation_id: Some(self.id.clone()),
                ..Default::default()
            };
            tracing::debug!("save thumbnail...");

            let path_buf = PathBuf::from(&share_drive_path).join(&thumbnail.internal_name);

            // blocking but whatever todo find a non blocking way
            let file = std::fs::File::create(path_buf).map_err(|e| ServiceError::from(&e))?;
            let mut buffer = std::io::BufWriter::new(file);
            thumb
                .write_to(&mut buffer, image_format)
                .map_err(|e| ServiceError::from(&e))?;

            store
                .update(&thumbnail.id, &thumbnail)
                .await
                .map_err(|e| ServiceError::from(&e))?;
            Ok(Some(thumbnail.id))
        } else {
            Ok(None)
        }
    }

    pub async fn download(&self, share_drive_path: &str) -> Result<File, ServiceError> {
        tokio::fs::File::open(PathBuf::from(share_drive_path).join(&self.internal_name))
            .await
            .map_err(|e| ServiceError::from(&e))
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
