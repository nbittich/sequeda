use crate::{constant::COOKIE_NAME, openid::destroy_session};
use async_redis_session::RedisSessionStore;
use async_session::{async_trait, Session, SessionStore};
use axum::{
    extract::{rejection::TypedHeaderRejectionReason, FromRequest, RequestParts},
    headers, Extension, TypedHeader,
};
use hyper::header::{self};
use openidconnect::{core::CoreGenderClaim, UserInfoClaims};
use serde::{Deserialize, Serialize};

use super::{
    auth_redirect::LoginPageRedirect,
    client::{OpenIdClient, OpenIdToken},
    AllOtherClaims, CustomIdTokenClaims,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub full_name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub middle_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub roles: Vec<String>,
}

impl User {
    pub fn from_user_info(user_info: &UserInfoClaims<AllOtherClaims, CoreGenderClaim>) -> Self {
        let id = user_info.subject().to_string();
        let full_name = user_info
            .name()
            .and_then(|n| n.get(user_info.locale()))
            .cloned()
            .map(|name| name.to_string());
        let given_name = user_info
            .given_name()
            .and_then(|n| n.get(user_info.locale()))
            .cloned()
            .map(|name| name.to_string());
        let family_name = user_info
            .given_name()
            .and_then(|n| n.get(user_info.locale()))
            .cloned()
            .map(|name| name.to_string());
        let middle_name = user_info
            .middle_name()
            .and_then(|n| n.get(user_info.locale()))
            .cloned()
            .map(|name| name.to_string());
        let username = user_info
            .preferred_username()
            .cloned()
            .map(|name| name.to_string());
        let email = user_info.email().cloned().map(|name| name.to_string());
        let roles = user_info.additional_claims().realm_access.roles.clone();
        User {
            id,
            full_name,
            given_name,
            family_name,
            email,
            roles,
            middle_name,
            username,
        }
    }

    pub fn from_claims(claims: &CustomIdTokenClaims) -> Self {
        let id = claims.subject().to_string();
        let full_name = claims
            .name()
            .and_then(|n| n.get(claims.locale()))
            .cloned()
            .map(|name| name.to_string());
        let given_name = claims
            .given_name()
            .and_then(|n| n.get(claims.locale()))
            .cloned()
            .map(|name| name.to_string());
        let family_name = claims
            .given_name()
            .and_then(|n| n.get(claims.locale()))
            .cloned()
            .map(|name| name.to_string());
        let middle_name = claims
            .middle_name()
            .and_then(|n| n.get(claims.locale()))
            .cloned()
            .map(|name| name.to_string());
        let username = claims
            .preferred_username()
            .cloned()
            .map(|name| name.to_string());
        let email = claims.email().cloned().map(|name| name.to_string());
        let roles = claims.additional_claims().realm_access.roles.clone();
        User {
            id,
            full_name,
            given_name,
            family_name,
            email,
            roles,
            middle_name,
            username,
        }
    }
}

#[async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = LoginPageRedirect;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(store) = Extension::<RedisSessionStore>::from_request(req)
            .await
            .expect("`RedisSessionStore` extension is missing");
        let Extension(client) = Extension::<OpenIdClient>::from_request(req)
            .await
            .expect("`OpenIdClient` extension is missing");

        let cookies = TypedHeader::<headers::Cookie>::from_request(req)
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => LoginPageRedirect,
                    _ => panic!("unexpected error getting Cookie header(s): {}", e),
                },
                _ => panic!("unexpected error getting cookies: {}", e),
            })?;
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(LoginPageRedirect)?;

        let session = store
            .load_session(session_cookie.to_string())
            .await
            .map_err(|_| LoginPageRedirect)?
            .ok_or(LoginPageRedirect)?;

        let id_token: OpenIdToken = session.get("token").ok_or(LoginPageRedirect)?;

        let user = User::from_claims(&id_token.claims);

        tracing::debug!("user {user:?}");

        let id_token = match client.refresh_token(id_token).await {
            Ok(id) => id,
            Err(e) => {
                tracing::error!("{e}");
                return Err(destroy_session(&store, session).await)
            }
        };

        let user = match client.exchange_access_token(&id_token, &user.id).await {
            Ok(user) => Self::from_user_info(&user),
            Err(e) => {
                tracing::error!("{e}");
                return Err(destroy_session(&store, session).await)
            }
        };
        let user_info = match client.exchange_access_token(&id_token, &user.id).await {
            Ok(user_info) => user_info,
            Err(e) => {
                tracing::error!("{e}");
               return Err(destroy_session(&store, session).await)
            }
        };

        Ok(User::from_user_info(&user_info))
    }
}


