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
    to_value, user_header::ExtractUserInfo, StoreCollection, PUBLIC_TENANT, SERVICE_COLLECTION_NAME,
};
use sequeda_store::{doc, Repository, StoreClient, StoreRepository};
use serde_json::json;

use crate::entity::{ContactDetail, Person, PersonUpsert};

pub fn get_router(client: StoreClient) -> Router {
    //let allow_origin = var(CORS_ALLOW_ORIGIN).unwrap_or_else(|_| String::from("*"));
    //let _allow_origin_header = allow_origin.parse::<HeaderValue>().unwrap();
    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("person"));

    Router::new()
        .route("/current", get(current))
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
        .layer(Extension(StoreCollection(collection_name)))
}

/// routes

// get current user profile or insert it
async fn current(
    Extension(client): Extension<StoreClient>,
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    Extension(collection): Extension<StoreCollection>,
) -> impl IntoResponse {
    tracing::debug!("Person get current route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            }))
        );
    };
    tracing::debug!("tenant is {}", &tenant);

    let repository: StoreRepository<Person> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;
    if let Ok(Some(person)) = repository
        .find_one(Some(doc! {"userId": &x_user_info.id}))
        .await
    {
        tracing::debug!("user was found, person {:?}", &person);
        (StatusCode::OK, Json(to_value(person)))
    } else {
        let person = Person {
            user_id: Some(x_user_info.id),
            first_name: x_user_info.given_name.unwrap_or_default(),
            nick_name: x_user_info.username,
            last_name: x_user_info.family_name.unwrap_or_default(),
            middle_name: x_user_info.middle_name.unwrap_or_default(),
            contact_detail: ContactDetail {
                email_address_1: x_user_info.email.unwrap_or_default(),
                ..Default::default()
            },
            ..Default::default()
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
}

async fn find_all(
    Extension(client): Extension<StoreClient>,
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    Extension(collection): Extension<StoreCollection>,
) -> impl IntoResponse {
    tracing::debug!("Person list route entered!");
    let repository: StoreRepository<Person> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;
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
    Extension(collection): Extension<StoreCollection>,
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    Path(person_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Person find one route entered!");
    let repository: StoreRepository<Person> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;

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
    Extension(collection): Extension<StoreCollection>,
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    Path(person_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Person delete one route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            }))
        );
    };
    let repository: StoreRepository<Person> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;

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
    Extension(collection): Extension<StoreCollection>,
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    extract::Json(payload): extract::Json<PersonUpsert>,
) -> impl IntoResponse {
    tracing::debug!("Upsert person route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            }))
        );
    };
    let repository: StoreRepository<Person> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;
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
        user_id,
        first_name,
        last_name,
        date_of_birth,
        nick_name,
        gender,
        academic_title,
        contact_detail,
        marital_status,
    } = payload;

    let person = Person {
        first_name,
        user_id,
        last_name,
        date_of_birth,
        nick_name,
        gender,
        academic_title,
        marital_status,
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
