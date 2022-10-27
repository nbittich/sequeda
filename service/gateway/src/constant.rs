
pub const REDIS_URL: &str = "SESSION_REDIS_URL";
pub const COOKIE_NAME: &str = "SEQUEDA_SESSION";
pub const AUTH_REDIRECT_PATH: &str = "/login/authorized";

pub const OPENID_CLIENT_ID: &str = "OPENID_CLIENT_ID";
pub const OPENID_CLIENT_SECRET: &str = "OPENID_CLIENT_SECRET";
pub const OPENID_ISSUER_URL: &str = "OPENID_ISSUER_URL";
pub const OPENID_SCOPES: &str = "OPENID_SCOPES";
pub const APP_ROOT_URL: &str = "APP_ROOT_URL";
pub const OPENID_ENABLED: &str = "OPENID_ENABLED";

pub use sequeda_service_common::{setup_tracing, SERVICE_CONFIG_VOLUME, SERVICE_HOST, SERVICE_PORT};