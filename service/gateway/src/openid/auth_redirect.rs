use axum::response::{IntoResponse, Redirect, Response};
use openidconnect::{url::Url, CsrfToken, Nonce};

use super::{client::OpenIdClient, RawOpenIdClient};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AuthRedirect {
    client: RawOpenIdClient,
    authorize_url: Url,
    csrf_state: CsrfToken,
    nonce: Nonce,
}

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary(self.authorize_url.as_str()).into_response()
    }
}

#[allow(dead_code)]
impl AuthRedirect {
    pub async fn new(client: &OpenIdClient, redirect_url: &str) -> AuthRedirect {
        let (client, authorize_url, csrf_state, nonce) = client.get_authorize_url(redirect_url);
        AuthRedirect {
            client,
            authorize_url,
            csrf_state,
            nonce,
        }
    }
}
