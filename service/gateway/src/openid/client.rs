use std::env;

use openidconnect::core::{
    CoreAuthDisplay, CoreClaimName, CoreClaimType, CoreClient, CoreClientAuthMethod, CoreGrantType,
    CoreJsonWebKey, CoreJsonWebKeyType, CoreJsonWebKeyUse, CoreJweContentEncryptionAlgorithm,
    CoreJweKeyManagementAlgorithm, CoreJwsSigningAlgorithm, CoreResponseMode, CoreResponseType,
    CoreSubjectIdentifierType,
};
use openidconnect::reqwest::async_http_client;
use openidconnect::url::Url;
use openidconnect::{
    AdditionalProviderMetadata, AuthenticationFlow, CsrfToken, Nonce, ProviderMetadata,
    RedirectUrl, RevocationUrl, Scope,
};
use openidconnect::{ClientId, ClientSecret, IssuerUrl};
use serde::{Deserialize, Serialize};

use super::{
    RawOpenIdClient, OPENID_CLIENT_ID, OPENID_CLIENT_SECRET, OPENID_ISSUER_URL, OPENID_SCOPES,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RevocationEndpointProviderMetadata {
    revocation_endpoint: String,
}

impl AdditionalProviderMetadata for RevocationEndpointProviderMetadata {}
type OpenIdProviderMetadata = ProviderMetadata<
    RevocationEndpointProviderMetadata,
    CoreAuthDisplay,
    CoreClientAuthMethod,
    CoreClaimName,
    CoreClaimType,
    CoreGrantType,
    CoreJweContentEncryptionAlgorithm,
    CoreJweKeyManagementAlgorithm,
    CoreJwsSigningAlgorithm,
    CoreJsonWebKeyType,
    CoreJsonWebKeyUse,
    CoreJsonWebKey,
    CoreResponseMode,
    CoreResponseType,
    CoreSubjectIdentifierType,
>;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OpenIdClient {
    client: RawOpenIdClient,
}

#[allow(dead_code)]
impl OpenIdClient {
    fn get_scopes() -> Vec<Scope> {
        env::var(OPENID_SCOPES)
            .unwrap_or("roles, email, profile".into())
            .split(",")
            .into_iter()
            .map(|scope| Scope::new(scope.trim().to_string()))
            .collect()
    }
    pub fn get_authorize_url(
        &self,
        redirect_url: &str,
    ) -> (RawOpenIdClient, Url, CsrfToken, Nonce) {
        let scopes = Self::get_scopes();

        let redirect_url =
            RedirectUrl::new(redirect_url.to_string()).expect("Invalid redirect URL");

        let client = self.client.clone().set_redirect_uri(redirect_url);

        let mut authorize_url_req = client.authorize_url(
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        );
        for scope in scopes {
            authorize_url_req = authorize_url_req.add_scope(scope);
        }
        let (url, csrf, nonce) = authorize_url_req.url();
        (client, url, csrf, nonce)
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

        let client =
            CoreClient::from_provider_metadata(provider_metadata, client_id, Some(client_secret))
                .set_revocation_uri(
                    RevocationUrl::new(revocation_endpoint)
                        .expect("Invalid revocation endpoint URL"),
                );

        OpenIdClient { client }
    }
}

fn handle_error<T: std::error::Error>(fail: &T, msg: &'static str) {
    let mut err_msg = format!("ERROR: {}", msg);
    let mut cur_fail: Option<&dyn std::error::Error> = Some(fail);
    while let Some(cause) = cur_fail {
        err_msg += &format!("\n    caused by: {}", cause);
        cur_fail = cause.source();
    }
    panic!("{}", err_msg)
}
