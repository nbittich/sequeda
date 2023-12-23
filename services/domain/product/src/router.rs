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
    user_header::ExtractUserInfo, QueryIds, StoreCollection, PUBLIC_TENANT, SERVICE_COLLECTION_NAME,
};
use sequeda_store::{Repository, StoreClient, StoreRepository};
use serde_json::json;

use crate::entity::{ProductItem, ProductItemUpsert};

pub fn get_router(client: StoreClient) -> Router {
    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("product"));

    Router::new()
        .route("/find-all", get(find_all))
        .route("/find-by-ids", post(find_by_ids))
        .route("/find-one/:product_id", get(find_one))
        .route("/delete/:product_id", delete(delete_by_id))
        .route("/", post(upsert))
        .layer(Extension(client))
        .layer(Extension(StoreCollection(collection_name)))
}

/// routes

async fn find_all(
    Extension(client): Extension<StoreClient>,
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    Extension(collection): Extension<StoreCollection>,
) -> impl IntoResponse {
    tracing::debug!("ProductItem list route entered!");
    let repository: StoreRepository<ProductItem> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;
    match repository.find_all().await {
        Ok(people) => (StatusCode::OK, Json(people)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
async fn find_by_ids(
    Extension(client): Extension<StoreClient>,
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    Extension(collection): Extension<StoreCollection>,
    extract::Json(QueryIds(query_ids)): extract::Json<QueryIds>,
) -> impl IntoResponse {
    tracing::debug!("ProductItem list by ids route entered!");
    let repository: StoreRepository<ProductItem> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;
    match repository.find_by_ids(query_ids).await {
        Ok(people) => (StatusCode::OK, Json(people)).into_response(),
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
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    Path(product_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("ProductItem find one route entered!");
    let repository: StoreRepository<ProductItem> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;

    match repository.find_by_id(&product_id).await {
        Ok(product) => (StatusCode::OK, Json(product)).into_response(),
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
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    Path(product_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("ProductItem delete one route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            })),
        );
    };
    let repository: StoreRepository<ProductItem> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;

    match repository.delete_by_id(&product_id).await {
        Ok(Some(product)) => (
            StatusCode::OK,
            Json(json!({
                "result": format!("product with id {} deleted", &product.id)
            })),
        ),
        Ok(None) => (
            StatusCode::NO_CONTENT,
            Json(json!({
                "result": format!("product with id {} not found", &product_id)
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
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    extract::Json(payload): extract::Json<ProductItemUpsert>,
) -> impl IntoResponse {
    tracing::debug!("Upsert ProductItem route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            })),
        )
            .into_response();
    };
    let repository: StoreRepository<ProductItem> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;
    let product = async {
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
    let ProductItemUpsert {
        name,
        tags,
        unit_type,
        description,
        price_per_unit,
        main_picture_id,
        ..
    } = payload;

    let product = ProductItem {
        name,
        tags,
        unit_type,
        description,
        price_per_unit,
        main_picture_id,
        ..product
    };

    let result = repository.update(&product.id, &product).await;
    match result {
        Ok(_) => (StatusCode::OK, Json(product)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
