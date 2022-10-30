use std::cmp::Ordering;
use std::env;

use async_session::chrono::Duration;
use openidconnect::core::{
    CoreGenderClaim, CoreIdTokenVerifier, CoreResponseType, CoreRevocableToken,
};
use openidconnect::reqwest::async_http_client;
use openidconnect::url::Url;
use openidconnect::{
    AccessToken, AuthenticationFlow, AuthorizationCode, CsrfToken, GenderClaim, Nonce,
    OAuth2TokenResponse, RedirectUrl, RevocationUrl, Scope, SubjectIdentifier, TokenResponse,
    UserInfoClaims,
};
use openidconnect::{ClientId, ClientSecret, IssuerUrl};
use serde::{Deserialize, Serialize};

use super::{AllOtherClaims, CustomTokenResponse, OpenIdProviderMetadata, RawOpenIdClient};
use crate::constant::{OPENID_CLIENT_ID, OPENID_CLIENT_SECRET, OPENID_ISSUER_URL, OPENID_SCOPES};
use crate::openid::CustomIdTokenClaims;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OpenIdClient {
    client: RawOpenIdClient,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenIdToken {
    pub claims: CustomIdTokenClaims,
    pub token: CustomTokenResponse,
}

#[allow(dead_code)]
impl OpenIdClient {
    fn get_scopes() -> Vec<Scope> {
        env::var(OPENID_SCOPES)
            .unwrap_or_else(|_| "roles, email, profile".into())
            .split(',')
            .into_iter()
            .map(|scope| Scope::new(scope.trim().to_string()))
            .collect()
    }
    pub fn with_redirect_url(&self, redirect_url: &str) -> Self {
        let redirect_url =
            RedirectUrl::new(redirect_url.to_string()).expect("Invalid redirect URL");
        let client = self.client.clone().set_redirect_uri(redirect_url);
        Self { client }
    }

    pub fn get_authorize_url(
        &self,
        redirect_url: &str,
        nonce: Nonce,
    ) -> (RawOpenIdClient, Url, CsrfToken, Nonce) {
        let scopes = Self::get_scopes();
        let copy = self.with_redirect_url(redirect_url);

        let mut authorize_url_req = copy.client.authorize_url(
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            CsrfToken::new_random,
            || nonce,
        );
        for scope in scopes {
            authorize_url_req = authorize_url_req.add_scope(scope);
        }
        let (url, csrf, nonce) = authorize_url_req.url();
        (copy.client, url, csrf, nonce)
    }
    pub async fn new() -> Self {
        let client_id = ClientId::new(
            env::var(OPENID_CLIENT_ID).expect("Missing the OPENID_CLIENT_ID environment variable."),
        );
        let client_secret = ClientSecret::new(
            env::var(OPENID_CLIENT_SECRET)
                .expect("Missing the OPENID_CLIENT_SECRET environment variable."),
        );
        let issuer_url = IssuerUrl::new(
            env::var(OPENID_ISSUER_URL)
                .expect("Missing the OPENID_ISSUER_URL environment variable."),
        )
        .expect("Invalid issuer URL");
        let provider_metadata =
            OpenIdProviderMetadata::discover_async(issuer_url, async_http_client)
                .await
                .unwrap_or_else(|err| {
                    handle_error(&err, "Failed to discover OpenID Provider");
                    unreachable!();
                });

        let revocation_endpoint = provider_metadata
            .additional_metadata()
            .revocation_endpoint
            .clone();

        let client = RawOpenIdClient::from_provider_metadata(
            provider_metadata,
            client_id,
            Some(client_secret),
        )
        .set_revocation_uri(
            RevocationUrl::new(revocation_endpoint).expect("Invalid revocation endpoint URL"),
        );

        OpenIdClient { client }
    }

    pub async fn exchange_access_token(
        &self,
        id_token: &OpenIdToken,
        user_id: &str,
    ) -> Option<UserInfoClaims<AllOtherClaims, CoreGenderClaim>> {
        let token_response = &id_token.token;
        let access_token = token_response.access_token().clone();
        let request = self
            .client
            .user_info(
                access_token,
                Some(SubjectIdentifier::new(user_id.to_string())),
            )
            .expect("could not create request for user info");
        match request.request_async(async_http_client).await {
            Ok(user_info) => Some(user_info),
            Err(e) => {
                tracing::error!("error: {e}");
                None
            }
        }
    }

    pub async fn refresh_token(&self, id_token: OpenIdToken) -> OpenIdToken {
        let claims = &id_token.claims;
        let token_response = &id_token.token;
        let diff = claims
            .expiration()
            .signed_duration_since(claims.issue_time());

        match token_response.expires_in() {
            Some(duration)
                if duration.cmp(&diff.to_std().expect("could not convert to std"))
                    == Ordering::Less =>
            {
                let token = self
                    .client
                    .exchange_refresh_token(
                        id_token
                            .token
                            .refresh_token()
                            .expect("refresh token not present"),
                    )
                    .request_async(async_http_client)
                    .await
                    .expect("could not refresh token");
                OpenIdToken { token, ..id_token }
            }
            _ => id_token,
        }
    }

    pub async fn exchange_token(
        &self,
        auth_request: super::auth_request::AuthRequest,
        nonce: Nonce,
    ) -> OpenIdToken {
        //let _state = CsrfToken::new(auth_request.state);
        tracing::debug!("code: {:?}", &auth_request);
        let code = AuthorizationCode::new(auth_request.code);
        let token_response = self
            .client
            .exchange_code(code)
            .request_async(async_http_client)
            .await
            .unwrap_or_else(|err| {
                handle_error(&err, "Failed to contact token endpoint");
                unreachable!();
            });
        let id_token_verifier: CoreIdTokenVerifier = self.client.id_token_verifier();
        let claims: &CustomIdTokenClaims = token_response
            .extra_fields()
            .id_token()
            .expect("id token missing")
            .claims(&id_token_verifier, &nonce)
            .unwrap();
        // let token: CoreRevocableToken = match token_response.refresh_token() {
        //     Some(token) => token.into(),
        //     None => token_response.access_token().into(),
        // };
        OpenIdToken {
            claims: claims.clone(),
            token: token_response,
        }
    }
}

fn handle_error<T: std::error::Error>(fail: &T, msg: &'static str) {
    let mut err_msg = format!("ERROR: {}", msg);
    let mut cur_fail: Option<&dyn std::error::Error> = Some(fail);
    while let Some(cause) = cur_fail {
        err_msg += &format!("\n    caused by: {}", cause);
        cur_fail = cause.source();
    }
    panic!("{}", err_msg) // todo should not panic but returns an error
}
