use async_redis_session::RedisSessionStore;

use crate::{constant::COOKIE_NAME, openid::user::User};
use async_session::{Session, SessionStore};
use axum::{
    extract::{Path, Query},
    headers,
    response::{IntoResponse, Redirect},
    routing::get,
    routing::Router,
    Extension, Json, TypedHeader,
};
use hyper::{header::SET_COOKIE, HeaderMap, StatusCode};
use openidconnect::Nonce;

use super::{
    auth_redirect::LoginPageRedirect,
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

pub async fn open_id_router(
    auth_redirect: &str,
    store: RedisSessionStore,
    openid_client: OpenIdClient,
    redirect_url: &str,
    root_url: &str,
) -> Router {
    Router::new()
        .route("/login", get(login))
        // .route("/login-credentials", post(login_credentials))
        .route(auth_redirect, get(login_authorized))
        .route("/logout", get(logout))
        .route("/@me", get(user_info))
        .layer(Extension(store))
        .layer(Extension(openid_client))
        .layer(Extension(Config {
            redirect_url: redirect_url.to_string(),
            root_url: root_url.to_string(),
        }))
}

async fn login(
    user: Option<User>,
    Extension(client): Extension<OpenIdClient>,
    Extension(config): Extension<Config>,
) -> impl IntoResponse {
    if user.is_some() {
        return Redirect::permanent("/@me").into_response();
    }
    let nonce = Nonce::new_random();
    let redirect_url = format!("{}/{}", &config.redirect_url, nonce.secret());
    let (_, authorize_url, _, _) = client.get_authorize_url(&redirect_url, nonce);
    Redirect::temporary(authorize_url.as_str()).into_response()
}

async fn logout(
    Extension(client): Extension<OpenIdClient>,
    Extension(store): Extension<RedisSessionStore>,
    optional_cookies: Option<TypedHeader<headers::Cookie>>,
) -> impl IntoResponse {
    let cookie = if let Some(cookies) = optional_cookies
        .and_then(|cookies| cookies.get(COOKIE_NAME).map(|cookie| cookie.to_string()))
    {
        cookies
    } else {
        return LoginPageRedirect;
    };
    let session = match store.load_session(cookie).await {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return LoginPageRedirect,
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
    (StatusCode::OK, Json(user))
}

// async fn login_credentials(
//     user: Option<User>,
//     Query(params): Query<HashMap<String, String>>,
//     Extension(client): Extension<OpenIdClient>,
//     Extension(store): Extension<RedisSessionStore>,
// ) -> impl IntoResponse {
//     if user.is_some() {
//         return Redirect::permanent("/@me").into_response();
//     }
//     let token = client
//         .exchange_credentials(params["username"].clone(), params["password"].clone())
//         .await
//         .unwrap();
//     let mut session = Session::new();
//     session.insert("token", token).unwrap();

//     // Store session and get corresponding cookie
//     let cookie = store.store_session(session).await.unwrap().unwrap();

//     // Build the cookie
//     let cookie = format!("{}={}; SameSite=Lax; Path=/", COOKIE_NAME, cookie);

//     // Set cookie
//     let mut headers = HeaderMap::new();
//     headers.insert(SET_COOKIE, cookie.parse().unwrap());
//     (headers, Redirect::to("/@me")).into_response() // todo change this
// }

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
    (headers, Redirect::to("/@me")) // todo change this
}
