use axum::response::{IntoResponse, Redirect, Response};
use openidconnect::{url::Url, CsrfToken, Nonce};

use super::{client::OpenIdClient, RawOpenIdClient};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AuthRedirect {
    pub client: RawOpenIdClient,
    pub authorize_url: Url,
    pub csrf_state: CsrfToken,
    pub nonce: Nonce,
}

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary(self.authorize_url.as_str()).into_response()
    }
}

#[allow(dead_code)]
impl AuthRedirect {
    pub async fn new(client: &OpenIdClient, redirect_url: &str, nonce: Nonce) -> AuthRedirect {
        let (client, authorize_url, csrf_state, nonce) =
            client.get_authorize_url(redirect_url, nonce);
        AuthRedirect {
            client,
            authorize_url,
            csrf_state,
            nonce,
        }
    }
}
#[derive(Debug)]
pub struct LoginPageRedirect;

impl IntoResponse for LoginPageRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/login").into_response()
    }
}
