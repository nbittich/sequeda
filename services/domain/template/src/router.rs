use std::{env::var, io::Cursor};

use axum::{
    extract::{Multipart, Path, Query},
    http::{header, StatusCode},
    response::{AppendHeaders, IntoResponse},
    routing::{delete, get, post},
    Extension, Json, Router,
};
use axum_extra::headers::ContentType;
use chrono::Local;
use mime_guess::mime::APPLICATION_PDF;
use sequeda_file_upload_client::{FileUploadClient, UploadFileRequestUriParams};
use sequeda_service_common::{
    user_header::ExtractUserInfo, QueryIds, StoreCollection, PUBLIC_TENANT, SERVICE_COLLECTION_NAME,
};
use sequeda_store::{
    doc, FindOneAndReplaceOptions, MongoError, Repository, StoreClient, StoreRepository,
};
use sequeda_template_common::{ContextQuery, RenderRequest, Template, TemplateType};
use serde_json::json;
use tokio::io::AsyncWriteExt;
use tokio_util::io::ReaderStream;

use crate::entity::{TemplateUpsert, TemplateWrapper};

pub fn get_router(store_client: StoreClient, file_upload_client: FileUploadClient) -> Router {
    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("invoice"));

    Router::new()
        .route("/find-all", get(find_all))
        .route("/find-by-ids", post(find_by_ids))
        .route("/find-by-context", get(find_by_context))
        .route("/find-one/:templ_id", get(find_one))
        .route("/delete/:templ_id", delete(delete_by_id))
        .route("/render", post(render))
        .route("/", post(upsert))
        .layer(Extension(store_client))
        .layer(Extension(file_upload_client))
        .layer(Extension(StoreCollection(collection_name)))
}

async fn render(
    Extension(client): Extension<StoreClient>,
    ExtractUserInfo {
        user_info: x_user_info,
        header,
    }: ExtractUserInfo,
    Extension(collection): Extension<StoreCollection>,
    Extension(file_upload_client): Extension<FileUploadClient>,
    Json(req): Json<RenderRequest>,
) -> impl IntoResponse {
    tracing::debug!("Template render route entered!");
    let repository: StoreRepository<Template> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;
    match repository.find_by_id(&req.template_id).await {
        Ok(Some(tpl)) => {
            if tpl.template_context != req.template_context {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json! ({"error": "invalid template context"})),
                )
                    .into_response();
            }
            let pdf = crate::render::render(&tpl, &req.context, &file_upload_client, &header)
                .await
                .unwrap();
            let cursor = Cursor::new(pdf);
            let stream = ReaderStream::new(cursor);
            let body = axum::body::Body::from_stream(stream);
            let content_header = (
                header::CONTENT_DISPOSITION,
                format!(r#"attachment; filename="{}""#, &req.file_name),
            );

            let content_type = (header::CONTENT_TYPE, APPLICATION_PDF.to_string());

            let headers = AppendHeaders([content_type, content_header]);

            (StatusCode::OK, headers, body).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "template not found"})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn find_by_context(
    Extension(client): Extension<StoreClient>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    Extension(collection): Extension<StoreCollection>,
    Query(context): Query<ContextQuery>,
) -> impl IntoResponse {
    tracing::debug!("Template find by context route entered!");
    let repository: StoreRepository<Template> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;
    match repository
        .find_by_query(doc! {"template_context": context.context.to_string()}, None)
        .await
    {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
async fn find_by_ids(
    Extension(client): Extension<StoreClient>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    Extension(collection): Extension<StoreCollection>,
    Json(QueryIds(query_ids)): Json<QueryIds>,
) -> impl IntoResponse {
    tracing::debug!("Template list by ids route entered!");
    let repository: StoreRepository<Template> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;
    match repository.find_by_ids(query_ids).await {
        Ok(templs) => (StatusCode::OK, Json(templs)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
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
    let mut session = match client.start_session().await {
        Ok(session) => session,
        Err(e) => return handle_err(e),
    };

    if let Err(e) = session.start_transaction().await {
        return handle_err(e);
    }

    let template_collection = session
        .client()
        .database(&tenant)
        .collection::<Template>(&collection);

    let maybe_template = {
        if let Some(id) = query.id {
            let i = template_collection.find_one(doc! {"_id": id}).await;
            match i {
                Ok(Some(mut i)) => {
                    i.updated_date = Some(Local::now().naive_local());
                    TemplateWrapper(i)
                }
                Err(e) => return handle_err(e),
                _ => Default::default(),
            }
        } else {
            Default::default()
        }
    };
    let maybe_template = maybe_template.0;

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
            TemplateType::Html => {
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
        .find_one_and_replace(doc! {"_id": &template.id}, &template)
        .with_options(options)
        .await
    {
        return handle_err(e);
    }

    if let Err(e) = session.commit_transaction().await {
        return handle_err(e);
    }

    (StatusCode::OK, Json(template)).into_response()
}
async fn delete_by_id(
    Extension(client): Extension<StoreClient>,
    Extension(collection): Extension<StoreCollection>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    Path(templ_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Template delete one route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            })),
        );
    };
    let repository: StoreRepository<Template> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;

    match repository.delete_by_id(&templ_id).await {
        Ok(Some(templ)) => (
            StatusCode::OK,
            Json(json!({
                "result": format!("templ with id {} deleted", &templ.id)
            })),
        ),
        Ok(None) => (
            StatusCode::NO_CONTENT,
            Json(json!({
                "result": format!("templ with id {} not found", &templ_id)
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

async fn find_all(
    Extension(client): Extension<StoreClient>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    Extension(collection): Extension<StoreCollection>,
) -> impl IntoResponse {
    tracing::debug!("Template list route entered!");
    let repository: StoreRepository<Template> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;
    match repository.find_all().await {
        Ok(templ) => (StatusCode::OK, Json(templ)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
async fn find_one(
    Extension(client): Extension<StoreClient>,
    Extension(collection): Extension<StoreCollection>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    Path(templ_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Template find one route entered!");
    let repository: StoreRepository<Template> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;

    match repository.find_by_id(&templ_id).await {
        Ok(templ) => (StatusCode::OK, Json(templ)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
