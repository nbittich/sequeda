use std::env::var;

use axum::{
    extract::{Multipart, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Extension, Json, Router,
};
use axum_extra::headers::ContentType;
use chrono::Local;
use sequeda_file_upload_client::{FileUploadClient, UploadFileRequestUriParams};
use sequeda_service_common::{
    user_header::ExtractUserInfo, StoreCollection, SERVICE_COLLECTION_NAME,
};
use sequeda_store::{doc, FindOneAndReplaceOptions, MongoError, StoreClient};
use serde_json::json;
use tokio::io::AsyncWriteExt;

use crate::entity::{Template, TemplateUpsert};

pub fn get_router(store_client: StoreClient, file_upload_client: FileUploadClient) -> Router {
    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("invoice"));

    Router::new()
        // .route("/find-all", get(find_all))
        // .route("/find-by-ids", post(find_by_ids))
        // .route("/find-one/:person_id", get(find_one))
        // .route("/delete/:person_id", delete(delete_by_id))
        .route("/", post(upsert))
        .layer(Extension(store_client))
        .layer(Extension(file_upload_client))
        .layer(Extension(StoreCollection(collection_name)))
}
async fn upsert(
    Extension(client): Extension<StoreClient>,
    Extension(StoreCollection(collection)): Extension<StoreCollection>,
    Extension(file_upload_client): Extension<FileUploadClient>,
    ExtractUserInfo {
        user_info: x_user_info,
        header: x_user_info_header,
    }: ExtractUserInfo,
    Query(query): Query<TemplateUpsert>,
    mut form: Multipart,
) -> impl IntoResponse {
    tracing::debug!("Upsert template route entered!");

    let handle_err = |e: MongoError| {
        tracing::error!("could not proceed upsert invoice. err: {e:?}");
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    };
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            })),
        )
            .into_response();
    };
    let client = client.get_raw_client(); // todo, maybe make a SessionStoreRepository or something
    let mut session = match client.start_session(None).await {
        Ok(session) => session,
        Err(e) => return handle_err(e),
    };

    if let Err(e) = session.start_transaction(None).await {
        return handle_err(e);
    }
    let template_collection = session
        .client()
        .database(&tenant)
        .collection::<Template>(&collection);
    let maybe_template = {
        if let Some(id) = query.id {
            let i = template_collection
                .find_one_with_session(doc! {"_id": id}, None, &mut session)
                .await;
            match i {
                Ok(Some(mut i)) => {
                    i.updated_date = Some(Local::now().naive_local());
                    i
                }
                Err(e) => return handle_err(e),
                _ => Default::default(),
            }
        } else {
            Default::default()
        }
    };

    let TemplateUpsert {
        id: _,
        title,
        description,
        template_type,
        template_context,
    } = query;

    let mut template = Template {
        title,
        description,
        template_context,
        ..maybe_template
    };
    let options = FindOneAndReplaceOptions::builder()
        .upsert(Some(true))
        .build();

    if let Some(mut field) = form.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let temp_path = std::env::temp_dir().join(&file_name);
        let mut temp_file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&temp_path)
            .await
            .unwrap();
        while let Ok(Some(chunk)) = field.chunk().await {
            temp_file.write_all(&chunk).await.unwrap();
        }
        match &template_type {
            crate::entity::TemplateType::Html => {
                if let Some(ct) = mime_guess::from_path(&temp_path).first() {
                    tokio::fs::remove_file(&temp_path).await.unwrap();
                    if ContentType::from(ct) != ContentType::html() {
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(json!({"error": "File content type doesn't match template type"})),
                        )
                            .into_response();
                    }
                }
            }
        }
        template.template_type = template_type;
        let fu = file_upload_client
            .upload_file(
                &x_user_info_header,
                UploadFileRequestUriParams {
                    correlation_id: Some(template.id.clone()),
                    id: if template.file_id.is_empty() {
                        None
                    } else {
                        Some(template.file_id)
                    },
                    is_public: Some(false),
                },
                &file_name,
                temp_file,
            )
            .await
            .unwrap();
        template.file_id = fu.id;
    } else if template.file_id.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            "you cannot save a template that doesn't have a file attached to it",
        )
            .into_response();
    }
    if let Err(e) = template_collection
        .find_one_and_replace_with_session(
            doc! {"_id": &template.id},
            &template,
            options,
            &mut session,
        )
        .await
    {
        return handle_err(e);
    }

    if let Err(e) = session.commit_transaction().await {
        return handle_err(e);
    }

    (StatusCode::OK, Json(template)).into_response()
}
