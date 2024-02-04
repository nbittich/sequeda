use std::env::var;

use axum::{
    extract::{self, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use chrono::Local;
use sequeda_service_common::{
    user_header::ExtractUserInfo, StoreCollection, PUBLIC_TENANT, SERVICE_COLLECTION_NAME,
};
use sequeda_store::{Repository, StoreClient, StoreRepository};
use serde_json::json;

use crate::position::{Position, PositionUpsert};

pub fn get_router(client: StoreClient) -> Router {
    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("position"));

    Router::new()
        .route("/find-all", get(find_all))
        .route("/find-one/:position_id", get(find_one))
        .route("/delete/:position_id", delete(delete_by_id))
        .route("/", post(upsert))
        .layer(Extension(client))
        .layer(Extension(StoreCollection(collection_name)))
}

/// routes

async fn find_all(
    Extension(client): Extension<StoreClient>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    Extension(collection): Extension<StoreCollection>,
) -> impl IntoResponse {
    tracing::debug!("Position list route entered!");
    let repository: StoreRepository<Position> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;

    match repository.find_all().await {
        Ok(positions) => (StatusCode::OK, Json(positions)).into_response(),
        Err(e) => {
            tracing::debug!("error {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
                .into_response()
        }
    }
}
async fn find_one(
    Extension(client): Extension<StoreClient>,
    Extension(collection): Extension<StoreCollection>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    Path(position_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Position find one route entered!");
    let repository: StoreRepository<Position> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;

    match repository.find_by_id(&position_id).await {
        Ok(position) => (StatusCode::OK, Json(position)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn delete_by_id(
    Extension(client): Extension<StoreClient>,
    Extension(collection): Extension<StoreCollection>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    Path(position): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Position delete one route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            })),
        );
    };
    let repository: StoreRepository<Position> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;

    match repository.delete_by_id(&position).await {
        Ok(Some(position)) => (
            StatusCode::OK,
            Json(json!({
                "result": format!("position with id {} deleted", &position.id)
            })),
        ),
        Ok(None) => (
            StatusCode::NO_CONTENT,
            Json(json!({
                "result": format!("position with id {} not found", &position)
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

async fn upsert(
    Extension(client): Extension<StoreClient>,
    Extension(collection): Extension<StoreCollection>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    extract::Json(payload): extract::Json<PositionUpsert>,
) -> impl IntoResponse {
    tracing::debug!("Upsert position route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            })),
        )
            .into_response();
    };
    let repository: StoreRepository<Position> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;
    let position = async {
        if let Some(id) = &payload.id {
            let p = repository.find_by_id(id).await;
            if let Ok(Some(mut p)) = p {
                p.updated_date = Some(Local::now().naive_local());
                return p;
            }
        }
        Default::default()
    }
    .await;

    let PositionUpsert {
        id: _,
        name,
        description,
        level,
    } = payload;

    let position = Position {
        name,
        description,
        level,
        ..position
    };

    let result = repository.update(&position.id, &position).await;
    match result {
        Ok(_) => (StatusCode::OK, Json(position)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
