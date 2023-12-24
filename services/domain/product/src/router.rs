use std::env::var;

use axum::{
    extract::{self, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use chrono::Local;
use sequeda_service_common::{
    user_header::ExtractUserInfo, IdGenerator, QueryIds, StoreCollection, PUBLIC_TENANT,
    SERVICE_COLLECTION_NAME,
};
use sequeda_store::{doc, Repository, StoreClient, StoreRepository};
use serde::Deserialize;
use serde_json::json;

use crate::entity::{ProductItem, ProductItemUpsert, ProductTag};

pub fn get_router(client: StoreClient) -> Router {
    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("product"));

    Router::new()
        .route("/find-all", get(find_all))
        .route("/tag/search", get(search_tag))
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

#[derive(Deserialize)]
struct TagQuery {
    tag: String,
}
async fn search_tag(
    Extension(client): Extension<StoreClient>,
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    Query(tag): Query<TagQuery>,
) -> impl IntoResponse {
    tracing::debug!("Search product tag route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            })),
        )
            .into_response();
    };
    let repository: StoreRepository<ProductTag> =
        StoreRepository::get_repository(client, "product_tag", &tenant).await;
    match find_tag(&repository, &tag.tag, false).await {
        Ok(p) => Json(p.into_iter().map(|p| p.name).collect::<Vec<_>>()).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn find_tag(
    repository: &StoreRepository<ProductTag>,
    tag: &str,
    exact: bool,
) -> Result<Vec<ProductTag>, sequeda_store::StoreError> {
    if exact {
        repository.find_by_query(doc! {"name": tag}).await
    } else {
        repository
            .find_by_query(doc! {"name": &format!("/${tag}/i")})
            .await
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
        StoreRepository::get_repository(client.clone(), &collection.0, &tenant).await;
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
        tags: tags
            .as_ref()
            .map(|t| t.iter().map(|l| l.to_lowercase()).collect()),
        unit_type,
        description,
        price_per_unit,
        main_picture_id,
        ..product
    };

    // persist the tag
    tokio::spawn(async move {
        let repository: StoreRepository<ProductTag> =
            StoreRepository::get_repository(client, "product_tag", &tenant).await;
        for tag in tags.iter().flatten() {
            if find_tag(&repository, tag, true)
                .await
                .is_ok_and(|pt| pt.is_empty())
            {
                let pt = ProductTag {
                    name: tag.to_lowercase(),
                    id: IdGenerator.get(),
                };
                if let Err(e) = repository.update(&pt.id, &pt).await {
                    tracing::error!("could not write tage {tag:?}: {e}");
                }
            }
        }
    });

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
