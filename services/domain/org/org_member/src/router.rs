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

use crate::entity::{Member, MemberUpsert, Remark};

pub fn get_router(client: StoreClient) -> Router {
    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("member"));

    Router::new()
        .route("/find-all", get(find_all))
        .route("/find-by-org/:org_id", get(find_by_org))
        .route("/find-one/:member_id", get(find_one))
        .route("/delete/:member_id", delete(delete_by_id))
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
    tracing::debug!("Member list route entered!");
    let repository: StoreRepository<Member> = StoreRepository::get_repository(
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
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    Path(member_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Member find one route entered!");
    let repository: StoreRepository<Member> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;

    match repository.find_by_id(&member_id).await {
        Ok(mut member) => {
            let mut res = None;
            if let Some(mut member) = member.take() {
                let responsible_of = repository
                    .find_by_query(doc! {"managedBy": &member.id}, None)
                    .await;

                tracing::debug!("responsible of {responsible_of:?}");

                if let Ok(responsible_of) = responsible_of {
                    member.responsible_of =
                        Some(responsible_of.into_iter().map(|m| m.id).collect());
                }
                res = Some(member);
            }
            (StatusCode::OK, Json(res)).into_response()
        }
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
    ExtractUserInfo(x_user_info): ExtractUserInfo,
    Path(org_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Member find by org route entered!");
    let repository: StoreRepository<Member> = StoreRepository::get_repository(
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
        Ok(members) => (StatusCode::OK, Json(members)).into_response(),
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
    Path(member_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Member delete one route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            })),
        );
    };
    let repository: StoreRepository<Member> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;

    match repository.delete_by_id(&member_id).await {
        Ok(Some(member)) => (
            StatusCode::OK,
            Json(json!({
                "result": format!("member with id {} deleted", &member.id)
            })),
        ),
        Ok(None) => (
            StatusCode::NO_CONTENT,
            Json(json!({
                "result": format!("member with id {} not found", &member_id)
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
    extract::Json(payload): extract::Json<MemberUpsert>,
) -> impl IntoResponse {
    tracing::debug!("Upsert member route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            })),
        )
            .into_response();
    };
    let repository: StoreRepository<Member> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;
    let member = async {
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
    let MemberUpsert {
        id: _,
        org_id,
        ended,
        started,
        managed_by,
        mut remarks,
        person_id,
        position_id,
    } = payload;

    for remark in remarks.iter_mut() {
        match member.remarks.iter().find(|r| r.id == remark.id) {
            Some(Remark {
                id: _,
                added_by_user_id,
                updated_by_user_id: _,
                added_date,
                updated_date: _,
                message: _,
            }) => {
                remark.updated_date = Some(Local::now().naive_local());
                remark.added_date = *added_date;
                remark.added_by_user_id = added_by_user_id.clone();
                remark.updated_by_user_id = Some(x_user_info.id.clone());
            }
            None => {
                remark.id = Some(IdGenerator.get());
                remark.added_date = Some(Local::now().naive_local());
                remark.updated_date = None;
                remark.added_by_user_id = Some(x_user_info.id.clone());
            }
        }
    }

    let member = Member {
        org_id,
        ended,
        started,
        managed_by,
        remarks,
        person_id,
        position_id,
        ..member
    };

    let result = repository.update(&member.id, &member).await;
    match result {
        Ok(_) => (StatusCode::OK, Json(member)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
