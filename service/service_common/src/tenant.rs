use axum::http::StatusCode;
use axum::Json;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};
use serde_json::json;

use crate::constants::X_TENANT_ID_HEADER;

pub struct ExtractTenantId(pub String);

#[async_trait]
impl<B> FromRequest<B> for ExtractTenantId
where
    B: Send,
{
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        if let Some(tenant_id) = req.headers().get(X_TENANT_ID_HEADER) {
            match tenant_id.to_str() {
                Ok(tenant) if !tenant.trim().is_empty() => {
                    Ok(ExtractTenantId(tenant.trim().to_string()))
                }
                _ => Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error":"X-TENANT-ID is invalid"})),
                )),
            }
        } else {
            Err((
                StatusCode::FORBIDDEN,
                Json(json!({"error":"X-TENANT-ID is missing"})),
            ))
        }
    }
}
