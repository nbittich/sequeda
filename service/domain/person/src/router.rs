use std::env::var;

use axum::{
    extract::{self, Path},
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post, MethodRouter},
    Json, Router,
};
use chrono::Local;
use sequeda_service_common::{tenant::ExtractTenantId, CORS_ALLOW_ORIGIN, SERVICE_COLLECTION_NAME};
use sequeda_store::{Repository, StoreClient, StoreRepository};
use serde::Serialize;
use serde_json::{json, Value};
use tower_http::cors::CorsLayer;

use crate::entity::{Person, PersonUpsert};

pub fn get_router(client: StoreClient) -> Router {
    let allow_origin = var(CORS_ALLOW_ORIGIN).unwrap_or_else(|_| String::from("*"));
    let allow_origin_header = allow_origin.parse::<HeaderValue>().unwrap();
    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("person"));

    Router::new()
        .merge(find_all(client.clone(), collection_name.clone()))
        .merge(find_one(client.clone(), collection_name.clone()))
        .merge(delete(client.clone(), collection_name.clone()))
        .merge(upsert(client, collection_name))
        .layer(
            CorsLayer::new()
                .allow_origin(allow_origin_header)
                .allow_methods(vec![Method::GET, Method::POST, Method::DELETE]),
        )
}

fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
}

/// routes

fn find_all(client: StoreClient, collection_name: String) -> Router {
    route(
        "/find-all",
        get(|ExtractTenantId(x_tenant_id): ExtractTenantId| async move {
            tracing::debug!("Person list route entered!");
            let repository = get_repository(client, &collection_name, &x_tenant_id).await;
            match repository.find_all().await {
                Ok(people) => into_response(StatusCode::OK, to_value(people)),
                Err(e) => into_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"error": e.to_string()}),
                ),
            }
        }),
    )
}
fn find_one(client: StoreClient, collection_name: String) -> Router {
    route(
        "/find-one/:person_id",
        get(
            |Path(person_id): Path<String>, ExtractTenantId(x_tenant_id): ExtractTenantId| async move {
                tracing::debug!("Person find one route entered!");
                let repository = get_repository(client, &collection_name, &x_tenant_id).await;

                match repository.find_by_id(&person_id).await {
                    Ok(person) => into_response(StatusCode::OK, to_value(person)),
                    Err(e) => into_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        json!({"error": e.to_string()}),
                    ),
                }
            },
        ),
    )
}

fn delete(client: StoreClient, collection_name: String) -> Router {
    route(
        "/delete/:person_id",
        axum::routing::delete(
            |Path(person_id): Path<String>, ExtractTenantId(x_tenant_id): ExtractTenantId| async move {
                tracing::debug!("Person delete one route entered!");
                let repository = get_repository(client, &collection_name, &x_tenant_id).await;
                match repository.delete_by_id(&person_id).await {
                    Ok(Some(person)) => into_response(
                        StatusCode::OK,
                        json!({ "result": format!("person with id {} deleted", &person.id) }),
                    ),
                    Ok(None) => into_response(
                        StatusCode::NO_CONTENT,
                        json!({ "result": format!("person with id {} not found", &person_id) }),
                    ),
                    Err(e) => into_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        json!({ "error": e.to_string() }),
                    ),
                }
            },
        ),
    )
}

fn upsert(client: StoreClient, collection_name: String) -> Router {
    route(
        "/",
        post(
            |extract::Json(payload): extract::Json<PersonUpsert>,
             ExtractTenantId(x_tenant_id): ExtractTenantId| async move {
                tracing::debug!("Upsert person route entered!");
                let repository = get_repository(client, &collection_name, &x_tenant_id).await;
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
                    Ok(_) => into_response(StatusCode::OK, to_value(person)),
                    Err(e) => into_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        json!({ "error": e.to_string() }),
                    ),
                }
            },
        ),
    )
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

fn into_response(status_code: StatusCode, body: Value) -> impl IntoResponse {
    (status_code, Json(body))
}
