use std::{env::var, net::SocketAddr, str::FromStr};

use sequeda_store::StoreClient;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use sequeda_service_common::{SERVICE_APPLICATION_NAME, SERVICE_HOST, SERVICE_PORT};

mod entity;
mod router;

#[tokio::main]
async fn main() {
    setup_tracing();

    let host = var(SERVICE_HOST).unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = var(SERVICE_PORT).unwrap_or_else(|_| String::from("0"));
    let app_name =
        var(SERVICE_APPLICATION_NAME).unwrap_or_else(|_| String::from("sequeda-person-service"));

    let addr = SocketAddr::from_str(&format!("{host}:{port}")).unwrap();

    let client = StoreClient::new(app_name).await.unwrap();
    let app = router::get_router(client.clone());

    tracing::info!("listening on {:?}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn setup_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
