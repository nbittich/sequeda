use std::{
    io::{Cursor, Read, Seek, SeekFrom},
    path::PathBuf,
};

use axum::async_trait;
use chrono::Local;
use image::{EncodableLayout, ImageFormat};
use mime_guess::mime::IMAGE_PNG;
use sequeda_service_common::{
    common_domain_types::ServiceError,
    file_upload_common::{FileUpload, IFileService},
};
use sequeda_store::{Repository, StoreRepository};
use tokio::fs::File;

use crate::soffice::convert_to;

pub const SHARE_DRIVE_PATH: &str = "SHARE_DRIVE_PATH";

const THUMB_HEIGHT: u32 = 300;
const THUMB_WIDTH: u32 = 300;

pub struct FileService<'a> {
    pub share_drive_path: &'a str,
    pub store: &'a StoreRepository<FileUpload>,
}

impl FileService<'_> {
    fn get_physical_path(&self, internal_name: &str) -> PathBuf {
        PathBuf::from(self.share_drive_path).join(internal_name)
    }

    async fn make_thumbnail(
        &self,
        upl: &FileUpload,
        internal_name: &str,
        file_handle: &[u8],
    ) -> Result<Option<String>, ServiceError> {
        let (extension, thumb) = {
            let (ct, image) = if !upl.is_image() {
                match convert_to(
                    self.get_physical_path(internal_name),
                    crate::soffice::ConvertType::Png,
                )
                .await
                {
                    Ok(bytes) => image::load_from_memory(&bytes)
                        .map_err(|e| ServiceError::from(&e))
                        .map(|im| (Some(IMAGE_PNG.to_string()), im)),
                    Err(e) => {
                        tracing::error!("error converting file {}: {} ", upl.original_filename, e);
                        return Ok(None);
                    }
                }
            } else {
                image::load_from_memory(file_handle)
                    .map_err(|e| ServiceError::from(&e))
                    .map(|im| (upl.content_type.clone(), im))
            }?;
            let thumb = image.thumbnail(THUMB_WIDTH, THUMB_HEIGHT);

            let Some(ct) = ct else {
                return Err(ServiceError("No Content type! Should not happen".into()));
            };

            let Some(image_format) = ImageFormat::from_mime_type(ct) else {
                return Err(ServiceError(
                    "Format cannot be transformed to thumbnail".into(),
                ));
            };

            tracing::debug!("generate thumbnail...");

            let mut cursor = Cursor::new(Vec::new());

            thumb
                .write_to(&mut cursor, image_format)
                .map_err(|e| ServiceError(format!("{e}")))?;
            cursor
                .seek(SeekFrom::Start(0))
                .map_err(|e| ServiceError(format!("{e}")))?;

            let mut thumb = Vec::new();

            cursor
                .read_to_end(&mut thumb)
                .map_err(|e| ServiceError(format!("{e}")))?;
            (upl.extension.clone(), thumb)
        };
        let thumbnail = FileUpload {
            content_type: upl.content_type.clone(),
            thumbnail_id: None,
            original_filename: format!("thumb-{internal_name}"),
            internal_name: format!("thumb-{internal_name}"),
            extension,
            size: thumb.len(),
            public_resource: upl.public_resource,
            correlation_id: Some(upl.id.clone()),
            ..Default::default()
        };
        tracing::debug!("save thumbnail...");

        let path_buf = PathBuf::from(&self.share_drive_path).join(&thumbnail.internal_name);

        tokio::fs::write(path_buf, thumb.as_bytes())
            .await
            .map_err(|e| ServiceError::from(&e))?;

        self.store
            .update(&thumbnail.id, &thumbnail)
            .await
            .map_err(|e| ServiceError::from(&e))?;
        Ok(Some(thumbnail.id))
    }
}
#[async_trait]
impl IFileService for FileService<'_> {
    async fn upload(
        &self,
        mut upl: FileUpload,
        bytes: Option<&[u8]>,
    ) -> Result<FileUpload, ServiceError> {
        if let Some(file_handle) = bytes {
            let upload = self
                .store
                .find_by_id(&upl.id)
                .await
                .map_err(|e| ServiceError::from(&e))?;
            let (old_internal_name, old_thumbnail_id) = if let Some(upload) = upload {
                (Some(upload.internal_name), upload.thumbnail_id)
            } else {
                (None, None)
            };
            let extension = &upl.extension;
            let internal_name = format!(
                "{}.{}",
                upl.id,
                extension.as_ref().cloned().unwrap_or_else(|| "".into())
            );

            if let Some(old_internal_name) = old_internal_name {
                upl.updated_date = Some(Local::now().naive_local());
                // override file
                tracing::info!("removing old file {}", old_internal_name);
                if let Err(e) = tokio::fs::remove_file(
                    PathBuf::from(self.share_drive_path).join(&old_internal_name),
                )
                .await
                {
                    tracing::error!("could not remove old file: {e}");
                }
                if let Some(old_thumbnail_id) = old_thumbnail_id {
                    self.store
                        .delete_by_id(&old_thumbnail_id)
                        .await
                        .map_err(|e| ServiceError::from(&e))?;

                    tracing::info!("removing old thumbnail {}", old_thumbnail_id);
                    if let Err(e) = tokio::fs::remove_file(
                        PathBuf::from(&self.share_drive_path)
                            .join(format!("thumb-{old_internal_name}")),
                    )
                    .await
                    {
                        tracing::error!("could not remove old thumbnail: {e}");
                    }
                }
            }

            tokio::fs::write(
                PathBuf::from(&self.share_drive_path).join(&internal_name),
                file_handle,
            )
            .await
            .map_err(|e| ServiceError::from(&e))?;

            upl.thumbnail_id = self
                .make_thumbnail(&upl, &internal_name, file_handle)
                .await?;

            upl.internal_name = internal_name;
        }

        self.store
            .update(&upl.id, &upl)
            .await
            .map_err(|e| ServiceError::from(&e))?;
        Ok(upl)
    }
    async fn download(&self, upl: &FileUpload) -> Result<File, ServiceError> {
        tokio::fs::File::open(self.get_physical_path(&upl.internal_name))
            .await
            .map_err(|e| ServiceError::from(&e))
    }
}
