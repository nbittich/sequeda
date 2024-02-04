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
    user_header::ExtractUserInfo, IdGenerator, StoreCollection, PUBLIC_TENANT,
    SERVICE_COLLECTION_NAME,
};
use sequeda_store::{doc, Pageable, Repository, StoreClient, StoreRepository};
use serde_json::json;

use crate::entity::{Communication, Customer, CustomerUpsert};

pub fn get_router(client: StoreClient) -> Router {
    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("customer"));

    Router::new()
        .route("/find-all", get(find_all))
        .route("/find-by-org/:org_id", get(find_by_org))
        .route("/find-one/:customer_id", get(find_one))
        .route("/delete/:customer_id", delete(delete_by_id))
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
    tracing::debug!("customer list route entered!");
    let repository: StoreRepository<Customer> = StoreRepository::get_repository(
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
async fn find_one(
    Extension(client): Extension<StoreClient>,
    Extension(collection): Extension<StoreCollection>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    Path(customer_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("customer find one route entered!");
    let repository: StoreRepository<Customer> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;

    match repository.find_by_id(&customer_id).await {
        Ok(customer) => (StatusCode::OK, Json(customer)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
async fn find_by_org(
    pagination: Option<Query<Pageable>>,
    Extension(client): Extension<StoreClient>,
    Extension(collection): Extension<StoreCollection>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    Path(org_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("customer find by org route entered!");
    let repository: StoreRepository<Customer> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;

    let page = if let Some(page) = pagination {
        page.0
    } else {
        Pageable {
            page: 0,
            limit: i64::MAX,
            sort: None,
        }
    };

    match repository
        .find_page(Some(doc! {"orgId": org_id}), page)
        .await
    {
        Ok(customers) => (StatusCode::OK, Json(customers)).into_response(),
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
    Path(customer_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("customer delete one route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            })),
        );
    };
    let repository: StoreRepository<Customer> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;

    match repository.delete_by_id(&customer_id).await {
        Ok(Some(customer)) => (
            StatusCode::OK,
            Json(json!({
                "result": format!("customer with id {} deleted", &customer.id)
            })),
        ),
        Ok(None) => (
            StatusCode::NO_CONTENT,
            Json(json!({
                "result": format!("customer with id {} not found", &customer_id)
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
    extract::Json(payload): extract::Json<CustomerUpsert>,
) -> impl IntoResponse {
    tracing::debug!("Upsert customer route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            })),
        )
            .into_response();
    };
    let repository: StoreRepository<Customer> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;
    let customer = async {
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
    let CustomerUpsert {
        id: _,
        recurring_product_ids,
        org_id,
        represented_by_id,
        customer_type,
        started,
        ended,
        mut communications,
        document_ids,
    } = payload;

    for communication in communications.iter_mut() {
        match customer
            .communications
            .iter()
            .find(|r| r.id == communication.id)
        {
            Some(Communication {
                id: _,
                added_by_user_id,
                updated_by_user_id: _,
                added_date,
                updated_date: _,
                message: _,
            }) => {
                communication.updated_date = Some(Local::now().naive_local());
                communication.added_date = *added_date;
                communication.added_by_user_id = added_by_user_id.clone();
                communication.updated_by_user_id = Some(x_user_info.id.clone());
            }
            None => {
                communication.id = Some(IdGenerator.get());
                communication.added_date = Some(Local::now().naive_local());
                communication.updated_date = None;
                communication.added_by_user_id = Some(x_user_info.id.clone());
            }
        }
    }

    let customer = Customer {
        org_id,
        ended,
        started,
        recurring_product_ids,
        customer_type,
        represented_by_id,
        communications,
        document_ids,
        ..customer
    };

    let result = repository.update(&customer.id, &customer).await;
    match result {
        Ok(_) => (StatusCode::OK, Json(customer)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
