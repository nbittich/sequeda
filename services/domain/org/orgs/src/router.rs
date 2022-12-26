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

use crate::entity::{Organization, OrganizationUpsert};

pub fn get_router(client: StoreClient) -> Router {
    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("organization"));

    Router::new()
        .route("/current", get(current))
        .route("/find-all", get(find_all))
        .route("/find-one/:organization_id", get(find_one))
        .route("/delete/:organization_id", delete(delete_by_id))
        .route("/", post(upsert))
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
    tracing::debug!("Organization get current route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            }))
        );
    };
    tracing::debug!("tenant is {}", &tenant);
    let repository: StoreRepository<Organization> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;
    if let Ok(Some(organization)) = repository.find_one(Some(doc! {"current": true})).await {
        tracing::debug!("current was found, organization {:?}", &organization);
        (StatusCode::OK, Json(to_value(organization)))
    } else {
        let organization = Organization {
            name: tenant,
            current: true,
            ..Default::default()
        };
        let result = repository.update(&organization.id, &organization).await;
        match result {
            Ok(_) => (StatusCode::OK, Json(to_value(organization))),
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
    tracing::debug!("Organization list route entered!");
    let repository: StoreRepository<Organization> = StoreRepository::get_repository(
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
    Path(organization_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Organization find one route entered!");
    let repository: StoreRepository<Organization> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;

    match repository.find_by_id(&organization_id).await {
        Ok(organization) => (StatusCode::OK, Json(to_value(organization))),
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
    Path(organization_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Organization delete one route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            }))
        );
    };
    let repository: StoreRepository<Organization> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;

    match repository.delete_by_id(&organization_id).await {
        Ok(Some(organization)) => (
            StatusCode::OK,
            Json(json!({
                "result": format!("organization with id {} deleted", &organization.id)
            })),
        ),
        Ok(None) => (
            StatusCode::NO_CONTENT,
            Json(json!({
                "result": format!("organization with id {} not found", &organization_id)
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
    extract::Json(payload): extract::Json<OrganizationUpsert>,
) -> impl IntoResponse {
    tracing::debug!("Upsert organization route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            }))
        );
    };
    let repository: StoreRepository<Organization> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;
    let organization = async {
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
    let OrganizationUpsert {
        id: _,
        name,
        description,
        vat_number,
        logo_id,
        primary_contact,
        other_contacts,
        primary_bank_account,
        other_bank_accounts,
        founded_date,
        closed_date,
        status,
    } = payload;

    let organization = Organization {
        name,
        description,
        vat_number,
        logo_id,
        primary_contact,
        other_contacts,
        primary_bank_account,
        other_bank_accounts,
        founded_date,
        closed_date,
        status,
        ..organization
    };

    let result = repository.update(&organization.id, &organization).await;
    match result {
        Ok(_) => (StatusCode::OK, Json(to_value(organization))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}
