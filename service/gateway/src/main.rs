mod config;
mod constant;
mod openid;
mod request_handler;
pub use constant::{OPENID_ENABLED, SERVICE_CONFIG_VOLUME, SERVICE_HOST, SERVICE_PORT};

use axum::{
    extract::Extension,
    http::{Request, Response},
    response::IntoResponse,
    routing::any,
    Router,
};
use hyper::{client::HttpConnector, Body};

use hyper_rustls::HttpsConnector;
use sequeda_service_common::setup_tracing;
use std::{env::var, net::SocketAddr, path::PathBuf, str::FromStr, sync::Arc};

use crate::{config::Config, openid::open_id_router, request_handler::RequestHandler};

type Client = hyper::client::Client<HttpsConnector<HttpConnector>, Body>;

const CONFIG_FILE_NAME: &str = "CONFIG_FILE_NAME";

#[tokio::main]
async fn main() {
    setup_tracing();
    let host = var(SERVICE_HOST).unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = var(SERVICE_PORT).unwrap_or_else(|_| String::from("0"));
    let openid_enabled = var(OPENID_ENABLED)
        .ok()
        .and_then(|enabled| enabled.parse::<bool>().ok())
        .unwrap_or(false);
    let config_volume = var(SERVICE_CONFIG_VOLUME).unwrap_or_else(|_| String::from("/tmp"));
    let config_file_name = var(CONFIG_FILE_NAME).unwrap_or_else(|_| String::from("gateway.yml"));

    let path_config = PathBuf::new().join(&config_volume).join(config_file_name);

    if !path_config.exists() {
        panic!("Missing config `{path_config:?}`");
    }

    let config: Config = Config::deserialize_file(path_config.as_path());
    let request_handler: RequestHandler = RequestHandler::from_config(config);

    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .build();

    let client: Client = hyper::client::Client::builder().build(https);

    let mut app = Router::new()
        .fallback(any(handler))
        .layer(Extension(client))
        .layer(Extension(Arc::new(request_handler)));

    if openid_enabled {
        let openid_router = open_id_router().await;
        app = openid_router.merge(app);
    }
    let addr = SocketAddr::from_str(&format!("{host}:{port}")).unwrap();

    tracing::info!("proxy gateway listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(
    Extension(client): Extension<Client>,
    Extension(request_handler): Extension<Arc<RequestHandler>>,
    mut req: Request<Body>,
) -> impl IntoResponse {
    tracing::debug!("req: {req:?}");
    match request_handler.handle(&mut req).await {
        Ok(_) => match client.request(req).await {
            Ok(response) => response,
            Err(er) => {
                tracing::debug!("error in request {er}");
                Response::builder()
                    .status(500)
                    .body(Body::from("unexpected error".as_bytes()))
                    .unwrap()
            }
        },
        Err(e) if e.status.is_some() => Response::builder()
            .status(e.status.unwrap())
            .body(Body::from(format!("{e}")))
            .unwrap(),
        Err(e) => Response::builder()
            .status(500)
            .body(Body::from(format!("unexpected error: {e}")))
            .unwrap(),
    }
}

#[cfg(test)]
mod test {
    use regex::Regex;

    #[test]
    fn test_regex() {
        let path = "/proxy/yahoo-finance/chart/woops/i/did/it/again?key=ok";
        let source = "/proxy/yahoo-finance/chart/(?P<segment>.*)";
        let dest = "/v8/finance/chart/${segment}";

        let re = Regex::new(source).unwrap();
        let c = re.captures(path).unwrap();
        let r = re.replace(path, dest);
        assert_eq!(
            Some("woops/i/did/it/again?key=ok"),
            c.name("segment").map(|c| c.as_str())
        );
        assert_eq!("/v8/finance/chart/woops/i/did/it/again?key=ok", r);

        let predicate = "/proxy/yahoo-finance/chart/**";
        let re = Regex::new(predicate).unwrap();
        assert!(re.is_match(path));
    }
}
