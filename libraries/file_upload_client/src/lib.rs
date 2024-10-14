use std::{
    env,
    error::Error,
    path::Path,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use reqwest::{Body, StatusCode};
pub use sequeda_file_upload_common::{
    DownloadFileRequestUriParams, FileUpload, UploadFileRequestUriParams,
};
use tokio_util::codec::{BytesCodec, FramedRead};
const X_USER_INFO_HEADER: &str = "X-USER-INFO";
pub const FILE_UPLOAD_ENDPOINT: &str = "FILE_UPLOAD_ENDPOINT";
#[derive(Clone)]
pub struct FileUploadClient {
    url: Arc<String>,
    client: reqwest::Client,
}
impl Default for FileUploadClient {
    fn default() -> Self {
        Self::new()
    }
}

impl FileUploadClient {
    pub fn new() -> Self {
        let url = Arc::new(env::var(FILE_UPLOAD_ENDPOINT).unwrap_or("http://uploads".into()));
        let client = reqwest::Client::new();
        FileUploadClient { url, client }
    }

    pub async fn upload_bytes(
        &self,
        x_user_info_header: &str,
        param: UploadFileRequestUriParams,
        file_name: &str,
        bytes: &[u8],
    ) -> Result<FileUpload, Box<dyn Error>> {
        let temp_file_path = std::env::temp_dir().join(format!(
            "{}_{file_name}",
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()
        ));
        tokio::fs::write(&temp_file_path, bytes).await?;
        let upl = {
            let file = tokio::fs::OpenOptions::new()
                .read(true)
                .open(&temp_file_path)
                .await?;
            self.upload_file(x_user_info_header, param, file_name, file)
                .await
        }?;
        tokio::fs::remove_file(temp_file_path).await?;
        Ok(upl)
    }

    pub async fn upload_file(
        &self,
        x_user_info_header: &str,
        params: UploadFileRequestUriParams,
        file_name: &str,
        file: tokio::fs::File,
    ) -> Result<FileUpload, Box<dyn Error>> {
        let stream = FramedRead::new(file, BytesCodec::new());
        let file_body = Body::wrap_stream(stream);
        let content_type = mime_guess::from_path(Path::new(file_name))
            .first()
            .map(|m| m.to_string());
        let part = reqwest::multipart::Part::stream(file_body)
            .file_name(file_name.to_string())
            .mime_str(&content_type.unwrap_or_else(|| "application/octet-stream".to_string()))?;

        let form = reqwest::multipart::Form::new()
            .text("id", params.id.unwrap_or_else(|| "".into()))
            .text(
                "correlation_id",
                params.correlation_id.unwrap_or_else(|| "".into()),
            )
            .text(
                "is_public",
                params
                    .is_public
                    .map(|b| b.to_string())
                    .unwrap_or_else(|| "false".into()),
            )
            .part("file", part);
        let resp = self
            .client
            .post(format!("{}/upload", self.url))
            .multipart(form)
            .header(X_USER_INFO_HEADER, x_user_info_header)
            .send()
            .await?;
        if resp.status() == StatusCode::OK {
            let resp = resp.json::<FileUpload>().await?;
            Ok(resp)
        } else {
            let error_msg: Box<dyn Error> = format!(
                "could not upload file. status code {}, error {:?}",
                resp.status(),
                resp.error_for_status()
            )
            .into();
            Err(error_msg)
        }
    }

    pub async fn metadata(
        &self,
        x_user_info_header: &str,
        param: DownloadFileRequestUriParams,
    ) -> Result<FileUpload, Box<dyn Error>> {
        let resp = self
            .client
            .get(format!("{}/metadata?id={}", self.url, param.id))
            .header(X_USER_INFO_HEADER, x_user_info_header)
            .send()
            .await?;
        if resp.status() == StatusCode::OK {
            let resp = resp.json().await?;
            Ok(resp)
        } else {
            let error_msg: Box<dyn Error> = format!(
                "could not extract metadata. status code {}, error {:?}",
                resp.status(),
                resp.error_for_status()
            )
            .into();
            Err(error_msg)
        }
    }

    pub async fn download(
        &self,
        x_user_info_header: &str,
        param: DownloadFileRequestUriParams,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let resp = self
            .client
            .get(format!("{}/download?id={}", self.url, param.id))
            .header(X_USER_INFO_HEADER, x_user_info_header)
            .send()
            .await?;
        if resp.status() == StatusCode::OK {
            let resp = resp.bytes().await?;

            Ok(resp.to_vec())
        } else {
            let error_msg: Box<dyn Error> = format!(
                "could not download file. status code {}, error {:?}",
                resp.status(),
                resp.error_for_status()
            )
            .into();
            Err(error_msg)
        }
    }
}
