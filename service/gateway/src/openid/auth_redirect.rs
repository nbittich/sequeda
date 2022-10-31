use axum::response::{IntoResponse, Redirect, Response};

#[derive(Debug)]
pub struct LoginPageRedirect;

impl IntoResponse for LoginPageRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/login").into_response()
    }
}
