use std::env::var;

use axum::{
    extract::{self, Path},
    http::{HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use chrono::Local;
use sequeda_service_common::{tenant::ExtractTenantId, CORS_ALLOW_ORIGIN, SERVICE_COLLECTION_NAME};
use sequeda_store::{Repository, StoreClient, StoreRepository};
use serde::Serialize;
use serde_json::{json, Value};

use crate::entity::{Person, PersonUpsert};

#[derive(Debug, Clone)]
struct Collection(String);

pub fn get_router(client: StoreClient) -> Router {
    let allow_origin = var(CORS_ALLOW_ORIGIN).unwrap_or_else(|_| String::from("*"));
    let _allow_origin_header = allow_origin.parse::<HeaderValue>().unwrap();
    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("person"));

    Router::new()
        .route("/find-all", get(find_all))
        .route("/find-one/:person_id", get(find_one))
        .route("/delete/:person_id", delete(delete_by_id))
        .route("/", post(upsert))
        // .layer(
        //     CorsLayer::new()
        //         .allow_origin(allow_origin_header)
        //         .allow_methods(vec![Method::GET, Method::POST, Method::DELETE]),
        // )
        .layer(Extension(client))
        .layer(Extension(Collection(collection_name)))
}

/// routes

async fn find_all(
    Extension(client): Extension<StoreClient>,
    ExtractTenantId(x_tenant_id): ExtractTenantId,
    Extension(collection): Extension<Collection>,
) -> impl IntoResponse {
    tracing::debug!("Person list route entered!");
    let repository = get_repository(client, &collection.0, &x_tenant_id).await;
    match repository.find_all().await {
        Ok(people) => (StatusCode::OK, Json(to_value(people))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}
async fn find_one(
    Extension(client): Extension<StoreClient>,
    Extension(collection): Extension<Collection>,
    ExtractTenantId(x_tenant_id): ExtractTenantId,
    Path(person_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Person find one route entered!");
    let repository = get_repository(client, &collection.0, &x_tenant_id).await;

    match repository.find_by_id(&person_id).await {
        Ok(person) => (StatusCode::OK, Json(to_value(person))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

async fn delete_by_id(
    Extension(client): Extension<StoreClient>,
    Extension(collection): Extension<Collection>,
    ExtractTenantId(x_tenant_id): ExtractTenantId,
    Path(person_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Person delete one route entered!");
    let repository = get_repository(client, &collection.0, &x_tenant_id).await;
    match repository.delete_by_id(&person_id).await {
        Ok(Some(person)) => (
            StatusCode::OK,
            Json(json!({
                "result": format!("person with id {} deleted", &person.id)
            })),
        ),
        Ok(None) => (
            StatusCode::NO_CONTENT,
            Json(json!({
                "result": format!("person with id {} not found", &person_id)
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
    Extension(collection): Extension<Collection>,
    ExtractTenantId(x_tenant_id): ExtractTenantId,
    extract::Json(payload): extract::Json<PersonUpsert>,
) -> impl IntoResponse {
    tracing::debug!("Upsert person route entered!");
    let repository = get_repository(client, &collection.0, &x_tenant_id).await;
    let person = async {
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
    let PersonUpsert {
        id: _,
        first_name,
        last_name,
        date_of_birth,
        nick_name,
        gender,
        academic_title,
        contact_detail,
    } = payload;

    let person = Person {
        first_name,
        last_name,
        date_of_birth,
        nick_name,
        gender,
        academic_title,
        contact_detail,
        ..person
    };

    let result = repository.update(&person.id, &person).await;
    match result {
        Ok(_) => (StatusCode::OK, Json(to_value(person))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

fn to_value<T: Serialize>(data: T) -> Value {
    match serde_json::to_value(data) {
        Ok(value) => value,
        Err(_) => json!({}),
    }
}

async fn get_repository(
    client: StoreClient,
    collection_name: &str,
    tenant_id: &str,
) -> StoreRepository<Person> {
    StoreRepository::from_collection_name(&client, tenant_id, collection_name).await
}
