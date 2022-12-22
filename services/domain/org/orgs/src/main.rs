use std::{env::var, net::SocketAddr, str::FromStr};

use sequeda_service_common::{setup_tracing, SERVICE_APPLICATION_NAME, SERVICE_HOST, SERVICE_PORT};
use sequeda_store::StoreClient;

mod entity;
mod router;

#[tokio::main]
async fn main() {
    setup_tracing();

    let host = var(SERVICE_HOST).unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = var(SERVICE_PORT).unwrap_or_else(|_| String::from("0"));
    let app_name =
        var(SERVICE_APPLICATION_NAME).unwrap_or_else(|_| String::from("sequeda-member-service"));

    let addr = SocketAddr::from_str(&format!("{host}:{port}")).unwrap();

    let client = StoreClient::new(app_name).await.unwrap();
    let app = router::get_router(client.clone());

    tracing::info!("listening on {:?}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
