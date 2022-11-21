use std::collections::HashMap;
use std::path::Path;
use std::{env::var, net::SocketAddr, str::FromStr};

use axum::body::StreamBody;
use axum::http::{header, StatusCode};
use axum::response::{AppendHeaders, IntoResponse};
use axum::routing::get;
use axum::Json;
use axum::{routing::post, Extension, Router};

use axum::extract::{Multipart, Query};
use mime_guess::mime::APPLICATION_OCTET_STREAM;
use sequeda_service_common::user_header::ExtractUserInfo;
use sequeda_service_common::{
    setup_tracing, to_value, StoreCollection, BODY_SIZE_LIMIT, PUBLIC_TENANT,
    SERVICE_COLLECTION_NAME, SERVICE_HOST, SERVICE_PORT,
};
use sequeda_store::{Repository, StoreClient, StoreRepository};
use serde_json::json;
use tokio_util::io::ReaderStream;
use tower_http::limit::RequestBodyLimitLayer;

use crate::file_upload::{FileUpload, SHARE_DRIVE_PATH};

pub mod file_upload;
#[derive(Clone, Debug)]
struct ShareDrive(String);

#[tokio::main]
async fn main() {
    setup_tracing();
    let share_drive_path: String = std::env::var(SHARE_DRIVE_PATH).unwrap();
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
        .route("/download/:id", get(download))
        .layer(RequestBodyLimitLayer::new(body_size_limit))
        .layer(Extension(client))
        .layer(Extension(ShareDrive(share_drive_path)))
        .layer(Extension(StoreCollection(collection_name)));

    tracing::info!("listening on {:?}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn download(
    Extension(client): Extension<StoreClient>,
    Extension(collection): Extension<StoreCollection>,
    Extension(ShareDrive(share_drive_path)): Extension<ShareDrive>,
    x_user_info: Option<ExtractUserInfo>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    async fn make_download_response(
        share_drive_path: &str,
        id: &str,
        repository: StoreRepository<FileUpload>,
    ) -> Option<impl IntoResponse> {
        if let Ok(Some(file)) = repository.find_by_id(id).await {
            let file_handle = file.download(share_drive_path).await.unwrap();
            let ct = file
                .content_type
                .unwrap_or_else(|| APPLICATION_OCTET_STREAM.to_string());
            let stream = ReaderStream::new(file_handle);
            let body = StreamBody::new(stream);
            let content_type = (header::CONTENT_TYPE, ct);
            let content_disposition = (
                header::CONTENT_TYPE,
                format!(r#"attachment; filename="{}"#, &file.original_filename),
            );
            let headers = AppendHeaders([content_type, content_disposition]);
            Some((headers, body))
        } else {
            None
        }
    }

    let public_repository: StoreRepository<FileUpload> =
        StoreRepository::get_repository(client.clone(), &collection.0, PUBLIC_TENANT).await;

    if let Some(response) = make_download_response(&share_drive_path, &id, public_repository).await
    {
        return response.into_response();
    } else if let Some(tenant) = x_user_info.map(|u| u.0).and_then(|u| u.tenant) {
        let private_repository: StoreRepository<FileUpload> =
            StoreRepository::get_repository(client, &collection.0, &tenant).await;
        if let Some(response) =
            make_download_response(&share_drive_path, &id, private_repository).await
        {
            return response.into_response();
        }
    }
    (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"}))).into_response()
}

async fn upload(
    Extension(client): Extension<StoreClient>,
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    Extension(collection): Extension<StoreCollection>,
    Extension(ShareDrive(share_drive_path)): Extension<ShareDrive>,
    mut multipart: Multipart,
    Query(mut query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    tracing::debug!("Person list route entered!");

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

        if let Some(id) = query.remove("id") {
            upl.id = id;
        }

        upl.public_resource = query
            .get("public")
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(false);

        let tenant = if upl.public_resource {
            PUBLIC_TENANT.into()
        } else {
            x_user_info.tenant.unwrap()
        };

        let repository: StoreRepository<FileUpload> =
            StoreRepository::get_repository(client, &collection.0, &tenant).await;
        upl.upload(&share_drive_path, Some(&data), &repository)
            .await
            .unwrap();
        (StatusCode::OK, Json(to_value(upl)))
    } else {
        let mut uploads_resp = Vec::with_capacity(uploads.len());
        let tenant = &x_user_info.tenant.unwrap();
        let repository: StoreRepository<FileUpload> =
            StoreRepository::get_repository(client, &collection.0, tenant).await;
        for (_, (mut upl, data)) in uploads {
            upl.upload(&share_drive_path, Some(&data), &repository)
                .await
                .unwrap();
            uploads_resp.push(upl);
        }
        (StatusCode::OK, Json(to_value(uploads_resp)))
    }
}
