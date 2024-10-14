use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}
