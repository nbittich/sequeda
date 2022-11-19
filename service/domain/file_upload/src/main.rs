use std::collections::HashMap;
use std::path::Path;
use std::{env::var, net::SocketAddr, str::FromStr};

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::post, Extension, Router};

use axum::extract::{Multipart, Query};
use sequeda_service_common::user_header::ExtractUserInfo;
use sequeda_service_common::{
    setup_tracing, StoreCollection, BODY_SIZE_LIMIT, PUBLIC_TENANT, SERVICE_COLLECTION_NAME,
    SERVICE_HOST, SERVICE_PORT,
};
use sequeda_store::{StoreClient, StoreRepository};
use tower_http::limit::RequestBodyLimitLayer;

use crate::file_upload::FileUpload;

pub mod file_upload;

#[tokio::main]
async fn main() {
    setup_tracing();

    let host = var(SERVICE_HOST).unwrap_or_else(|_| String::from("127.0.0.1"));
    let body_size_limit = (var(BODY_SIZE_LIMIT).unwrap_or_else(|_| "1024".into()))
        .parse::<usize>()
        .unwrap_or_else(|_| panic!("could not extract {}", BODY_SIZE_LIMIT));
    let port = var(SERVICE_PORT).unwrap_or_else(|_| String::from("0"));
    let app_name =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("sequeda-upload-service"));

    let addr = SocketAddr::from_str(&format!("{host}:{port}")).unwrap();

    let client = StoreClient::new(app_name).await.unwrap();
    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("upload"));

    let app = Router::new()
        .route("/upload", post(upload))
        .layer(RequestBodyLimitLayer::new(body_size_limit))
        .layer(Extension(client))
        .layer(Extension(StoreCollection(collection_name)));

    tracing::info!("listening on {:?}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn upload(
    Extension(client): Extension<StoreClient>,
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    Extension(collection): Extension<StoreCollection>,
    mut multipart: Multipart,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    tracing::debug!("Person list route entered!");
    let repository: StoreRepository<FileUpload> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;
    let mut uploads = HashMap::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.name().unwrap().to_string();

        let mut file_upload = FileUpload {
            content_type: field.content_type().map(|ct| ct.into()).or_else(|| {
                mime_guess::from_path(&file_name)
                    .first_raw()
                    .map(|ct| ct.into())
            }),
            extension: Path::new(&file_name)
                .extension()
                .map(|s| s.to_string_lossy().to_string()),
            original_filename: file_name.to_string(),
            ..Default::default()
        };

        let data = field.bytes().await.unwrap();

        file_upload.size = data.len();

        tracing::debug!("Length of `{}` is {} bytes", file_name, data.len());

        uploads.insert(file_name, (file_upload, data));
    }

    if uploads.len() == 1 {
        let Some((_, (mut upl, data))) =  uploads.into_iter().last() else {unreachable!("should never happen")};

        if let Some(id) = query.get("id") {
            upl.id = id.clone();
        }

        upl.public_resource = query
            .get("public")
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(false);

        upl.upload(Some(&data), &repository).await.unwrap();
    } else {
        for (_, (mut upl, data)) in uploads {
            upl.upload(Some(&data), &repository).await.unwrap();
        }
    }

    (StatusCode::OK, ())
}
