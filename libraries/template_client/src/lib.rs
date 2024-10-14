use std::{env, error::Error, sync::Arc};

use reqwest::StatusCode;
use sequeda_template_common::Template;
pub use sequeda_template_common::{Context, RenderRequest};
const X_USER_INFO_HEADER: &str = "X-USER-INFO";
pub const TEMPLATE_ENDPOINT: &str = "TEMPLATE_ENDPOINT";
#[derive(Clone)]
pub struct TemplateClient {
    url: Arc<String>,
    client: reqwest::Client,
}

impl Default for TemplateClient {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateClient {
    pub fn new() -> Self {
        let url = Arc::new(env::var(TEMPLATE_ENDPOINT).unwrap_or("http://template".into()));
        let client = reqwest::Client::new();
        Self { url, client }
    }
    pub async fn render(
        &self,
        x_user_info_header: &str,
        render_request: &RenderRequest,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let resp = self
            .client
            .get(format!("{}/render", self.url))
            .header(X_USER_INFO_HEADER, x_user_info_header)
            .json(render_request)
            .send()
            .await?;
        if resp.status() == StatusCode::OK {
            let resp = resp.bytes().await?;

            Ok(resp.to_vec())
        } else {
            let error_msg: Box<dyn Error> = format!(
                "could not render template. status code {}, error {:?}",
                resp.status(),
                resp.error_for_status()
            )
            .into();
            Err(error_msg)
        }
    }
    pub async fn find_by_id(
        &self,
        x_user_info_header: &str,
        id: &str,
    ) -> Result<Template, Box<dyn Error>> {
        let resp = self
            .client
            .get(format!("{}/find-one/{}", self.url, id))
            .header(X_USER_INFO_HEADER, x_user_info_header)
            .send()
            .await?;
        if resp.status() == StatusCode::OK {
            let resp = resp.json().await?;
            Ok(resp)
        } else {
            let error_msg: Box<dyn Error> = format!(
                "could not fetch template. status code {}, error {:?}",
                resp.status(),
                resp.error_for_status()
            )
            .into();
            Err(error_msg)
        }
    }
}
