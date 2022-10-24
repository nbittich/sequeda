use std::env;

use axum::response::{IntoResponse, Redirect, Response};
use openidconnect::core::{
    CoreAuthDisplay, CoreClaimName, CoreClaimType, CoreClient, CoreClientAuthMethod, CoreGrantType,
    CoreJsonWebKey, CoreJsonWebKeyType, CoreJsonWebKeyUse, CoreJweContentEncryptionAlgorithm,
    CoreJweKeyManagementAlgorithm, CoreJwsSigningAlgorithm, CoreResponseMode, CoreResponseType,
    CoreSubjectIdentifierType, CoreAuthPrompt,
};
use openidconnect::reqwest::async_http_client;
use openidconnect::url::Url;
use openidconnect::{
    AdditionalProviderMetadata, AuthenticationFlow, CsrfToken, Nonce, ProviderMetadata,
    RedirectUrl, RevocationUrl, Scope, AuthorizationRequest,
};
use openidconnect::{ClientId, ClientSecret, IssuerUrl};
use serde::{Deserialize, Serialize};

const OPENID_CLIENT_ID: &str = "OPENID_CLIENT_ID";
const OPENID_CLIENT_SECRET: &str = "OPENID_CLIENT_SECRET";
const OPENID_ISSUER_URL: &str = "OPENID_ISSUER_URL";
//const OPENID_REDIRECT_URL: &str = "OPENID_REDIRECT_URL";
const OPENID_SCOPES: &str = "OPENID_SCOPES";

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RevocationEndpointProviderMetadata {
    revocation_endpoint: String,
}
type OpenIdClient<'a> = AuthorizationRequest<'a,  CoreAuthDisplay, CoreAuthPrompt, CoreResponseType>;

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
pub struct AuthRedirect {
    authorize_url: Url,
    csrf_state: CsrfToken,
    nonce: Nonce,
    redirect_url: String,
}

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary(self.authorize_url.as_str()).into_response()
    }
}

impl AuthRedirect {
    #[allow(dead_code)]
    pub async fn new(redirect_url: &str) -> AuthRedirect {
        let redirect_url =
            RedirectUrl::new(redirect_url.to_string()).expect("Invalid redirect URL");

        let scopes: Vec<Scope> = env::var(OPENID_SCOPES)
            .unwrap_or("roles, email, profile".into())
            .split(",")
            .into_iter()
            .map(|scope| Scope::new(scope.trim().to_string()))
            .collect();

        // let redirect_url = RedirectUrl::new(
        //     env::var(OPENID_REDIRECT_URL)
        //         .expect("Missing the OPENID_REDIRECT_URL environment variable."),
        // )
        // .expect("Invalid redirect URL");

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

        tracing::info!("Discovered Openid revocation endpoint: {revocation_endpoint}");

        let client =
            CoreClient::from_provider_metadata(provider_metadata, client_id, Some(client_secret))
                .set_redirect_uri(redirect_url.clone())
                .set_revocation_uri(
                    RevocationUrl::new(revocation_endpoint)
                        .expect("Invalid revocation endpoint URL"),
                );

        let mut client = client.authorize_url(
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        );
        for scope in scopes {
            client = client.add_scope(scope);
        }

        let (authorize_url, csrf_state, nonce) = client.url();

        AuthRedirect {
            authorize_url,
            csrf_state,
            nonce,
            redirect_url: redirect_url.to_string(),
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
    panic!("{}", err_msg)
}
