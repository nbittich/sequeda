use axum::http::StatusCode;
use axum::Json;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::constants::X_USER_INFO_HEADER;

pub struct ExtractUserInfo(pub UserInfo);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub id: String,
    pub full_name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub middle_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub roles: Vec<String>,
    pub groups: Vec<String>,
    pub tenant: Option<String>,
}

#[async_trait]
impl<B> FromRequest<B> for ExtractUserInfo
where
    B: Send,
{
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        if let Some(user_info) = req.headers().get(X_USER_INFO_HEADER) {
            match user_info
                .to_str()
                .ok()
                .filter(|u| !u.trim().is_empty())
                .and_then(|u| base64::decode(u).ok())
                .and_then(|u| serde_json::from_slice::<UserInfo>(&u).ok())
            {
                Some(user_info) => Ok(ExtractUserInfo(user_info)),
                _ => Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error":"X-USER-INFO is invalid"})),
                )),
            }
        } else {
            Err((
                StatusCode::FORBIDDEN,
                Json(json!({"error":"X-USER-INFO is missing"})),
            ))
        }
    }
}