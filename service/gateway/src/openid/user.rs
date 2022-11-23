use crate::{constant::COOKIE_NAME, openid::destroy_session};
use async_redis_session::RedisSessionStore;
use async_session::{async_trait, SessionStore};
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
    AllOtherClaims, AuthConfig, CustomIdTokenClaims,
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
    pub groups: Vec<String>,
    pub tenant: Option<String>,
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
        let groups = user_info
            .additional_claims()
            .groups
            .clone()
            .unwrap_or_default();
        let tenant = user_info.additional_claims().tenant.clone();
        User {
            id,
            full_name,
            given_name,
            family_name,
            email,
            roles,
            tenant,
            groups,
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
        let groups = claims
            .additional_claims()
            .groups
            .clone()
            .unwrap_or_default();
        let tenant = claims.additional_claims().tenant.clone();

        User {
            id,
            full_name,
            given_name,
            family_name,
            email,
            roles,
            tenant,
            groups,
            middle_name,
            username,
        }
    }

    pub async fn from_cookie(
        store: RedisSessionStore,
        client: OpenIdClient,
        cookies: headers::Cookie,
    ) -> Result<Self, LoginPageRedirect> {
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(LoginPageRedirect)?;

        let session = store
            .load_session(session_cookie.to_string())
            .await
            .map_err(|_| LoginPageRedirect)?
            .ok_or(LoginPageRedirect)?;

        let id_token: OpenIdToken = session.get("token").ok_or(LoginPageRedirect)?;

        let id_token = match client.refresh_token(id_token).await {
            Ok(id) => id,
            Err(e) => {
                tracing::error!("{e}");
                return Err(destroy_session(&store, session).await);
            }
        };

        let user = match &id_token.claims.as_ref() {
            Some(claims) => User::from_claims(claims),
            None => match client.exchange_access_token(&id_token, None).await {
                Ok(user_info) => User::from_user_info(&user_info),
                Err(e) => {
                    tracing::error!("{e}");
                    return Err(destroy_session(&store, session).await);
                }
            },
        };

        tracing::debug!("user {user:?}");

        Ok(user)
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

        let Extension(config) = Extension::<AuthConfig>::from_request(req)
            .await
            .expect("`Auth config` extension is missing");

        let Extension(client) = Extension::<OpenIdClient>::from_request(req)
            .await
            .expect("`OpenIdClient` extension is missing");

        if config.demo_account {
            Ok(User {
                id: "demo-16ba6cdd-59cd-4bcc-b7ef-240af07153fd".into(),
                full_name: Some("Account Demo".into()),
                given_name: Some("Account".into()),
                family_name: Some("Demo".into()),
                middle_name: Some("AD".into()),
                username: Some("demo".into()),
                email: Some("demo@random.corp".into()),
                roles: vec!["demo".into()], // todo should be configurable
                groups: vec!["demogroup".into()],
                tenant: Some("demo".into()),
            })
        } else {
            let cookies = TypedHeader::<headers::Cookie>::from_request(req)
                .await
                .map_err(|e| match *e.name() {
                    header::COOKIE => match e.reason() {
                        TypedHeaderRejectionReason::Missing => LoginPageRedirect,
                        _ => panic!("unexpected error getting Cookie header(s): {}", e),
                    },
                    _ => panic!("unexpected error getting cookies: {}", e),
                })?;

            User::from_cookie(store, client, cookies.0).await
        }
    }
}
