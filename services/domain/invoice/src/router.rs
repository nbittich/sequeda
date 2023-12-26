use std::env::var;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use chrono::Local;
use sequeda_service_common::{
    user_header::ExtractUserInfo, StoreCollection, SERVICE_COLLECTION_NAME,
};
use sequeda_store::{StoreClient, StoreRepository};
use serde_json::json;

use crate::entity::Invoice;

pub fn get_router(client: StoreClient) -> Router {
    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("invoice"));

    Router::new()
        // .route("/find-all", get(find_all))
        // .route("/find-by-ids", post(find_by_ids))
        // .route("/find-one/:person_id", get(find_one))
        // .route("/delete/:person_id", delete(delete_by_id))
        .route("/", post(upsert))
        .layer(Extension(client))
        .layer(Extension(StoreCollection(collection_name)))
}

async fn upsert(
    Extension(client): Extension<StoreClient>,
    StoreCollection(collection): StoreCollection,
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    Json(invoice): Json<Invoice>,
) -> impl IntoResponse {
    tracing::debug!("Upsert invoice route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            })),
        )
            .into_response();
    };
    let repository: StoreRepository<Invoice> =
        StoreRepository::get_repository(client, &collection, &tenant).await;
    // let maybe_invoice = async {
    //     if let Some(id) = &invoice.id {
    //         let i = repository.find_by_id(id).await;
    //         if let Ok(Some(mut i)) = i {
    //             i.updated_date = Some(Local::now().naive_local());
    //             return i;
    //         }
    //     }
    //     Default::default()
    // }
    // .await;
    if invoice.processed {
        // we cannot change a locked invoice.
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "You cannot modify a processed invoice"})),
        )
            .into_response();
    }
    todo!()
}
