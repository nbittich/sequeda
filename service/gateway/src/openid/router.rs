use std::env;

use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
use axum::{
    extract::Query,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    routing::Router,
    Extension, TypedHeader,
};
use axum_sessions::{extractors::WritableSession, SessionLayer};
use openidconnect::Nonce;
use rand::Rng;
use crate::constant::{APP_ROOT_URL,REDIS_URL, AUTH_REDIRECT_PATH, COOKIE_NAME};

use super::{
    auth_redirect::AuthRedirect, auth_request::AuthRequest, client::OpenIdClient,
};
#[derive(Clone)]
struct Config {
    redirect_url: String,
}

pub async fn open_id_router() -> Router {
    let redis_url = env::var(REDIS_URL)
        .expect("Missing the REDIS_URL environment variable. e.g `redis://127.0.0.1`");

    let root_url = env::var(APP_ROOT_URL).expect("Missing the APP_ROOT_URL environment variable.");

    let redirect_url = root_url + AUTH_REDIRECT_PATH;

    let store = RedisSessionStore::new(redis_url).unwrap();
    let secret = rand::thread_rng().gen::<[u8; 128]>();
    let session_layer = SessionLayer::new(store, &secret);
    let openid_client = OpenIdClient::new().await;
    Router::new()
        .route("/login", get(login))
        .route(AUTH_REDIRECT_PATH, get(login_authorized))
        .route("/logout", get(logout))
        //.layer(Extension(store))
        .layer(Extension(openid_client))
        .layer(session_layer)
        .layer(Extension(Config { redirect_url }))
}

async fn login(
    Extension(client): Extension<OpenIdClient>,
    Extension(config): Extension<Config>,
    mut session: WritableSession,
) -> impl IntoResponse {
    let nonce = Nonce::new_random();
    let redirect = AuthRedirect::new(&client, &config.redirect_url, nonce.clone()).await;
    session
        .insert("_nonce", nonce)
        .expect("could not insert nonce");
    redirect
}

async fn logout(
    mut session: WritableSession,
) -> impl IntoResponse {
    session.destroy();

    Redirect::to("/login")
}

async fn login_authorized(
    Query(query): Query<AuthRequest>,
    Extension(client): Extension<OpenIdClient>,
    mut session: WritableSession,
) -> impl IntoResponse {
    let nonce: Nonce = session.get("_nonce").expect("nonce not in sessions");
    let token = client.exchange_token(query, nonce).await;
    session.regenerate();
    session.insert(COOKIE_NAME, token.claims).unwrap();

    Redirect::to("/")
}
