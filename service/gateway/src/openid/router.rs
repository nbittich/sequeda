use std::env;

use async_redis_session::RedisSessionStore;

use crate::{
    constant::{APP_ROOT_URL, AUTH_REDIRECT_PATH, COOKIE_NAME, REDIS_URL},
    openid::user::User,
};
use async_session::{Session, SessionStore};
use axum::{
    extract::{Path, Query},
    headers,
    response::{IntoResponse, Redirect},
    routing::get,
    routing::Router,
    Extension, TypedHeader,
};
use hyper::{header::SET_COOKIE, HeaderMap};
use openidconnect::Nonce;

use super::{
    auth_redirect::{AuthRedirect, LoginPageRedirect},
    auth_request::AuthRequest,
    client::{OpenIdClient, OpenIdToken},
    destroy_session,
};
#[derive(Clone)]
#[allow(unused)]
struct Config {
    redirect_url: String,
    root_url: String,
}

pub async fn open_id_router() -> Router {
    let redis_url = env::var(REDIS_URL)
        .expect("Missing the REDIS_URL environment variable. e.g `redis://127.0.0.1`");

    let root_url = env::var(APP_ROOT_URL).expect("Missing the APP_ROOT_URL environment variable.");

    let redirect_url = root_url.clone() + AUTH_REDIRECT_PATH;

    let store = RedisSessionStore::new(redis_url).unwrap();
    // let session_layer = SessionLayer::new(store, &secret);
    let openid_client = OpenIdClient::new().await;
    let auth_redirect = format!("{AUTH_REDIRECT_PATH}/:nonce");
    Router::new()
        .route("/login", get(login))
        .route(&auth_redirect, get(login_authorized))
        .route("/logout", get(logout))
        .route("/@me", get(user_info))
        .layer(Extension(store))
        .layer(Extension(openid_client))
        .layer(Extension(Config {
            redirect_url,
            root_url: root_url.clone(),
        }))
}

async fn login(
    Extension(client): Extension<OpenIdClient>,
    Extension(config): Extension<Config>,
) -> impl IntoResponse {
    let nonce = Nonce::new_random();
    let redirect_url = format!("{}/{}", &config.redirect_url, nonce.secret());
    AuthRedirect::new(&client, &redirect_url, nonce.clone()).await
}

async fn logout(
    Extension(client): Extension<OpenIdClient>,
    Extension(store): Extension<RedisSessionStore>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> impl IntoResponse {
    let cookie = cookies.get(COOKIE_NAME).unwrap();
    let session = match store.load_session(cookie.to_string()).await.unwrap() {
        Some(s) => s,
        None => return LoginPageRedirect,
    };
    if let Some(id_token) = session.get::<OpenIdToken>("token") {
        match client.logout(&id_token).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{e}");
            }
        }
    }

    destroy_session(&store, session).await
}

async fn user_info(user: User) -> impl IntoResponse {
    format!(
        "Welcome to the protected area :)\nHere's your info:\n{:?}",
        user
    )
}

async fn login_authorized(
    Query(query): Query<AuthRequest>,
    Path(nonce): Path<Nonce>,
    Extension(config): Extension<Config>,
    Extension(client): Extension<OpenIdClient>,
    Extension(store): Extension<RedisSessionStore>,
) -> impl IntoResponse {
    let redirect_url = format!("{}/{}", &config.redirect_url, nonce.secret());
    let client = client.with_redirect_url(&redirect_url);
    let token = client.exchange_token(query, nonce).await.unwrap();
    tracing::debug!("{:?}", &token);
    let mut session = Session::new();
    session.insert("token", token).unwrap();

    // Store session and get corresponding cookie
    let cookie = store.store_session(session).await.unwrap().unwrap();

    // Build the cookie
    let cookie = format!("{}={}; SameSite=Lax; Path=/", COOKIE_NAME, cookie);

    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());
    (headers, Redirect::to("/@me"))
}
