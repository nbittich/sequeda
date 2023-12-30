pub mod common_domain_types;
mod constants;
pub mod user_header;
pub use constants::{
    BODY_SIZE_LIMIT, CORS_ALLOW_ORIGIN, PUBLIC_TENANT, SERVICE_APPLICATION_NAME,
    SERVICE_COLLECTION_NAME, SERVICE_CONFIG_VOLUME, SERVICE_DATA_VOLUME, SERVICE_HOST,
    SERVICE_PORT, X_USER_INFO_HEADER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::Level;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn setup_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[deprecated]
pub fn to_value<T: Serialize + core::fmt::Debug>(data: T) -> Value {
    match serde_json::to_value(&data) {
        Ok(value) => value,
        Err(e) => {
            tracing::error!("error serializing {:?}, error: {e}", &data);
            json!({})
        }
    }
}
pub fn to_json_string<T: Serialize + core::fmt::Debug>(data: T) -> String {
    match serde_json::to_string(&data) {
        Ok(value) => value,
        Err(e) => {
            tracing::error!("error serialing {:?}, error: {e}", &data);
            "{}".into()
        }
    }
}

#[derive(Debug, Clone)]
pub struct StoreCollection(pub String);

pub struct IdGenerator;

impl IdGenerator {
    pub fn get(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }
}

#[derive(Deserialize)]
pub struct QueryIds(pub Vec<String>);
