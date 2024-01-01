use std::{env::var, net::SocketAddr, str::FromStr};

use sequeda_file_upload_client::FileUploadClient;
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
        var(SERVICE_APPLICATION_NAME).unwrap_or_else(|_| String::from("sequeda-template-service"));

    let addr = SocketAddr::from_str(&format!("{host}:{port}")).unwrap();

    let store_client = StoreClient::new(app_name).await.unwrap();
    let file_client = FileUploadClient::new();
    let app = router::get_router(store_client, file_client);

    tracing::info!("listening on {:?}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
