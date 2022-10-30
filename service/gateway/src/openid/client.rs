use std::env;
use std::fmt::Display;

use async_session::chrono::Utc;
use openidconnect::core::{
    CoreGenderClaim, CoreIdTokenVerifier, CoreResponseType, CoreRevocableToken,
};
use openidconnect::reqwest::async_http_client;
use openidconnect::url::Url;
use openidconnect::{
    AuthenticationFlow, AuthorizationCode, CsrfToken, Nonce, OAuth2TokenResponse, RedirectUrl,
    RevocationUrl, Scope, SubjectIdentifier, UserInfoClaims,
};
use openidconnect::{ClientId, ClientSecret, IssuerUrl};
use serde::{Deserialize, Serialize};

use super::{AllOtherClaims, CustomTokenResponse, OpenIdProviderMetadata, RawOpenIdClient};
use crate::constant::{OPENID_CLIENT_ID, OPENID_CLIENT_SECRET, OPENID_ISSUER_URL, OPENID_SCOPES};
use crate::openid::CustomIdTokenClaims;
use crate::Client;

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
                    let err = handle_error(&err, "Failed to discover OpenID Provider");
                    panic!("{err}");
                });

        let revocation_endpoint = provider_metadata
            .additional_metadata()
            .revocation_endpoint
            .clone();
        tracing::debug!("revocation endpoint: {}", revocation_endpoint);

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
    ) -> Result<UserInfoClaims<AllOtherClaims, CoreGenderClaim>, ClientError> {
        let token_response = &id_token.token;
        let access_token = token_response.access_token().clone();
        let request = self
            .client
            .user_info(
                access_token,
                Some(SubjectIdentifier::new(user_id.to_string())),
            )
            .map_err(|err| handle_error(&err, "exchange_access_token"))?;
        request
            .request_async(async_http_client)
            .await
            .map_err(|err| handle_error(&err, "exchange_access_token"))
    }

    pub async fn refresh_token(&self, id_token: OpenIdToken) -> Result<OpenIdToken, ClientError> {
        let claims = &id_token.claims;
        let token_response = &id_token.token;
        let diff = (Utc::now() - claims.expiration()).num_seconds();
        tracing::debug!("token diff {diff:?}");
        tracing::debug!("token expires_in {:?}", &token_response.expires_in());

        if diff >= 0 {
            tracing::debug!("token expired");
            let token = self
                .client
                .exchange_refresh_token(
                    id_token
                        .token
                        .refresh_token()
                        .ok_or(ClientError("refresh token not present".into()))?,
                )
                .request_async(async_http_client)
                .await
                .map_err(|err| handle_error(&err, "refresh_token"))?;
            Ok(OpenIdToken { token, ..id_token })
        } else {
            Ok(id_token)
        }
    }

    pub async fn exchange_token(
        &self,
        auth_request: super::auth_request::AuthRequest,
        nonce: Nonce,
    ) -> Result<OpenIdToken, ClientError> {
        //let _state = CsrfToken::new(auth_request.state);
        tracing::debug!("code: {:?}", &auth_request);
        let code = AuthorizationCode::new(auth_request.code);
        let token_response = self
            .client
            .exchange_code(code)
            .request_async(async_http_client)
            .await
            .map_err(|err| handle_error(&err, "exchange_token"))?;
        let id_token_verifier: CoreIdTokenVerifier = self.client.id_token_verifier();
        let claims: &CustomIdTokenClaims = token_response
            .extra_fields()
            .id_token()
            .ok_or(ClientError("id token missing".into()))?
            .claims(&id_token_verifier, &nonce)
            .unwrap();

        Ok(OpenIdToken {
            claims: claims.clone(),
            token: token_response,
        })
    }

    pub async fn logout(&self, id_token: &OpenIdToken) -> Result<(), ClientError> {
        let token = id_token.token.clone();
        let token_to_revoke: CoreRevocableToken = match token.refresh_token() {
            Some(token) => token.into(),
            None => token.access_token().into(),
        };
        self.client
            .revoke_token(token_to_revoke)
            .expect("no revocation_uri configured")
            .request_async(async_http_client)
            .await
            .map_err(|err| handle_error(&err, "logout"))?;
        Ok(())
    }
}

fn handle_error<T: std::error::Error>(fail: &T, msg: &'static str) -> ClientError {
    let mut err_msg = format!("ERROR: {}", msg);
    let mut cur_fail: Option<&dyn std::error::Error> = Some(fail);
    while let Some(cause) = cur_fail {
        err_msg += &format!("\n    caused by: {}", cause);
        cur_fail = cause.source();
    }
    ClientError(err_msg)
}

#[derive(Debug)]
pub struct ClientError(String);
impl std::error::Error for ClientError {}

impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
