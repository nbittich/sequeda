mod constants;
pub mod user_header;

pub use constants::{
    CORS_ALLOW_ORIGIN, PUBLIC_TENANT, SERVICE_APPLICATION_NAME, SERVICE_COLLECTION_NAME,
    SERVICE_CONFIG_VOLUME, SERVICE_DATA_VOLUME, SERVICE_HOST, SERVICE_PORT, X_USER_INFO_HEADER,
};
use tracing::Level;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn setup_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
