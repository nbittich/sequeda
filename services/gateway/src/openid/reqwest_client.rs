use std::env::var;

use openidconnect::{reqwest::Error, HttpRequest, HttpResponse};

use crate::constant::OPENID_DISABLE_SSL_ISSUER;

/// Custom async_http_client (code copied from oauth2 crate)
/// FIXME this doesn't allow to bump reqwest to the latest version
pub async fn async_http_client(
    request: HttpRequest,
) -> Result<HttpResponse, Error<reqwest::Error>> {
    let client = {
        let ssl_disabled = var(OPENID_DISABLE_SSL_ISSUER)
            .ok()
            .and_then(|disabled| disabled.parse::<bool>().ok())
            .unwrap_or(false);
        let builder = reqwest::Client::builder()
            .danger_accept_invalid_certs(ssl_disabled)
            .use_rustls_tls();

        // Following redirects opens the client up to SSRF vulnerabilities.
        // but this is not possible to prevent on wasm targets
        #[cfg(not(target_arch = "wasm32"))]
        let builder = builder.redirect(reqwest::redirect::Policy::none());

        builder.build().map_err(Error::Reqwest)?
    };

    let mut request_builder = client
        .request(request.method, request.url.as_str())
        .body(request.body);
    for (name, value) in &request.headers {
        request_builder = request_builder.header(name.as_str(), value.as_bytes());
    }
    let request = request_builder.build().map_err(Error::Reqwest)?;

    let response = client.execute(request).await.map_err(Error::Reqwest)?;

    let status_code = response.status();
    let headers = response.headers().to_owned();
    let chunks = response.bytes().await.map_err(Error::Reqwest)?;
    Ok(HttpResponse {
        status_code,
        headers,
        body: chunks.to_vec(),
    })
}
