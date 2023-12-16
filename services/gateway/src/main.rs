mod config;
mod constant;
mod openid;
mod request_handler;
use async_redis_session_v2::RedisSessionStore;
use axum::{
    body::Body,
    extract::{Extension, State},
    http::{Request, Response},
    response::IntoResponse,
    Router,
};
use axum_extra::headers::ContentType;
pub use constant::{OPENID_ENABLED, SERVICE_CONFIG_VOLUME, SERVICE_HOST, SERVICE_PORT};
use http_body_util::Empty;
use hyper::{
    header::{CONTENT_TYPE, LOCATION},
    StatusCode,
};
use hyper_tls::HttpsConnector;

use openid::User;
use sequeda_service_common::setup_tracing;
use std::{
    env::{self, var},
    net::SocketAddr,
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};

use crate::{
    config::Config,
    constant::{APP_ROOT_URL, AUTH_REDIRECT_PATH, REDIS_URL},
    openid::{open_id_router, AuthConfig, OpenIdClient},
    request_handler::RequestHandler,
};
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};

type Client = hyper_util::client::legacy::Client<HttpsConnector<HttpConnector>, axum::body::Body>;

const DEMO_ACCOUNT: &str = "DEMO_ACCOUNT";

#[tokio::main]
async fn main() {
    setup_tracing();
    let demo_account = var(DEMO_ACCOUNT)
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
    let host = var(SERVICE_HOST).unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = var(SERVICE_PORT).unwrap_or_else(|_| String::from("0"));
    let openid_enabled = var(OPENID_ENABLED)
        .ok()
        .and_then(|enabled| enabled.parse::<bool>().ok())
        .unwrap_or(false);
    let config_volume = var(SERVICE_CONFIG_VOLUME).unwrap_or_else(|_| String::from("/tmp"));

    let path_dir = PathBuf::new().join(&config_volume);

    let config: Config = Config::from_dir(path_dir.as_path());
    let request_handler: RequestHandler = RequestHandler::from_config(config);

    let https = HttpsConnector::new();
    let client = hyper_util::client::legacy::Client::builder(TokioExecutor::new())
        .build::<_, axum::body::Body>(https);
    let mut app = Router::new()
        .fallback(handler)
        .with_state(client)
        .layer(Extension(Arc::new(request_handler)));

    if openid_enabled {
        let redis_url = env::var(REDIS_URL)
            .expect("Missing the REDIS_URL environment variable. e.g `redis://127.0.0.1`");

        let root_url =
            env::var(APP_ROOT_URL).expect("Missing the APP_ROOT_URL environment variable.");

        let redirect_url = root_url.clone() + AUTH_REDIRECT_PATH;

        let store = RedisSessionStore::new(redis_url).unwrap();
        let openid_client = OpenIdClient::new().await;
        let auth_redirect = format!("{AUTH_REDIRECT_PATH}/:nonce");
        let auth_config = AuthConfig {
            auth_redirect,
            redirect_url: redirect_url.to_string(),
            root_url: root_url.to_string(),
            demo_account,
        };
        let openid_router =
            open_id_router(auth_config.clone(), store.clone(), openid_client.clone()).await;
        app = openid_router
            .merge(app)
            .layer(Extension(store))
            .layer(Extension(openid_client))
            .layer(Extension(auth_config));
    }
    let addr = SocketAddr::from_str(&format!("{host}:{port}")).unwrap();

    tracing::info!("proxy gateway listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn handler(
    State(client): State<Client>,
    Extension(request_handler): Extension<Arc<RequestHandler>>,
    user: Option<User>,
    mut req: Request<Body>,
) -> impl IntoResponse {
    tracing::debug!("req: {req:?}");
    let handle_forbidden = |status: StatusCode| {
        tracing::error!("unauthorized access: {:?}", &status);
        Response::builder()
            .status(StatusCode::PERMANENT_REDIRECT)
            .header(LOCATION, "/logout")
            .body(Empty::new())
            .unwrap()
            .into_response()
    };
    match request_handler.handle(&mut req, user).await {
        Ok(_) => match client.request(req).await {
            Ok(response)
                if response.status() == StatusCode::UNAUTHORIZED
                    || response.status() == StatusCode::FORBIDDEN =>
            {
                handle_forbidden(response.status())
            }

            Ok(response) => response.into_response(),
            Err(er) => {
                tracing::debug!("error in request {er}");
                Response::builder()
                    .status(500)
                    .header(CONTENT_TYPE, ContentType::json().to_string())
                    .body(Body::from(format!(
                        r#"{{"error": "unexpected error: {er}"}}"#
                    )))
                    .unwrap()
            }
        },
        Err(e)
            if e.status == Some(StatusCode::FORBIDDEN)
                || e.status == Some(StatusCode::UNAUTHORIZED) =>
        {
            handle_forbidden(e.status.unwrap())
        }

        Err(e) => Response::builder()
            .status(e.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
            .header(CONTENT_TYPE, ContentType::json().to_string())
            .body(Body::from(format!(
                r#"{{"error": "unexpected error: {e}"}}"#
            )))
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
