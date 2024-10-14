use std::env::var;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use base64::Engine;
use chrono::Local;
use sequeda_file_upload_client::{
    DownloadFileRequestUriParams, FileUploadClient, UploadFileRequestUriParams,
};
use sequeda_service_common::{
    user_header::ExtractUserInfo, QueryIds, StoreCollection, PUBLIC_TENANT, SERVICE_COLLECTION_NAME,
};
use sequeda_store::{
    doc, FindOneAndReplaceOptions, MongoError, Repository, StoreClient, StoreRepository,
};
use sequeda_template_client::{Context, RenderRequest, TemplateClient};
use serde_json::json;

use crate::entity::{Invoice, InvoiceSeq, InvoiceUpsert, INVOICE_SEQ_ROW_ID};

pub fn get_router(
    store_client: StoreClient,
    file_client: FileUploadClient,
    template_client: TemplateClient,
) -> Router {
    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("invoice"));

    Router::new()
        .route("/find-all", get(find_all))
        .route("/find-by-ids", post(find_by_ids))
        .route("/find-one/:id", get(find_one))
        .route("/delete/:invoice_id", delete(delete_by_id))
        .route("/", post(upsert))
        .layer(Extension(store_client))
        .layer(Extension(file_client))
        .layer(Extension(template_client))
        .layer(Extension(StoreCollection(collection_name)))
}

async fn delete_by_id(
    Extension(client): Extension<StoreClient>,
    Extension(collection): Extension<StoreCollection>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    Path(invoice_id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Invoice delete one route entered!");
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            })),
        );
    };
    let repository: StoreRepository<Invoice> =
        StoreRepository::get_repository(client, &collection.0, &tenant).await;

    match repository
        .delete_by_query(doc! {"_id":&invoice_id, "locked": false})
        .await
    {
        Ok(Some(invoice)) => (
            StatusCode::OK,
            Json(json!({
                "result": format!("invoice with id {} deleted", &invoice.id)
            })),
        ),
        Ok(None) => (
            StatusCode::NO_CONTENT,
            Json(json!({
                "result": format!("invoice with id {} not found", &invoice_id)
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

async fn find_one(
    Extension(client): Extension<StoreClient>,
    Extension(collection): Extension<StoreCollection>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    Path(id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Invoice find one route entered!");
    let repository: StoreRepository<Invoice> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;

    match repository.find_by_id(&id).await {
        Ok(invoice) => (StatusCode::OK, Json(invoice)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
async fn find_by_ids(
    Extension(client): Extension<StoreClient>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    Extension(collection): Extension<StoreCollection>,
    Json(QueryIds(query_ids)): Json<QueryIds>,
) -> impl IntoResponse {
    tracing::debug!("Invoice list by ids route entered!");
    let repository: StoreRepository<Invoice> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;
    match repository.find_by_ids(query_ids).await {
        Ok(products) => (StatusCode::OK, Json(products)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn find_all(
    Extension(client): Extension<StoreClient>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    Extension(collection): Extension<StoreCollection>,
) -> impl IntoResponse {
    tracing::debug!("Invoice list route entered!");
    let repository: StoreRepository<Invoice> = StoreRepository::get_repository(
        client,
        &collection.0,
        &x_user_info.tenant.unwrap_or_else(|| PUBLIC_TENANT.into()),
    )
    .await;
    match repository.find_all().await {
        Ok(invoices) => (StatusCode::OK, Json(invoices)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
async fn upsert(
    Extension(client): Extension<StoreClient>,
    Extension(StoreCollection(collection)): Extension<StoreCollection>,
    Extension(file_client): Extension<FileUploadClient>,
    Extension(template_client): Extension<TemplateClient>,
    ExtractUserInfo {
        user_info: x_user_info,
        header: x_user_info_header,
    }: ExtractUserInfo,
    Json(invoice): Json<InvoiceUpsert>,
) -> impl IntoResponse {
    tracing::debug!("Upsert invoice route entered! payload: {invoice:?}");

    let handle_err = |e: MongoError| {
        tracing::error!("could not proceed upsert invoice. err: {e:?}");
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    };
    let Some(tenant) = x_user_info.tenant else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "result": "tenant is missing"
            })),
        )
            .into_response();
    };
    let client = client.get_raw_client(); // todo, maybe make a SessionStoreRepository or something
    let mut session = match client.start_session().await {
        Ok(session) => session,
        Err(e) => return handle_err(e),
    };

    if let Err(e) = session.start_transaction().await {
        return handle_err(e);
    }
    let invoice_collection = session
        .client()
        .database(&tenant)
        .collection::<Invoice>(&collection);
    let maybe_invoice = {
        if let Some(id) = &invoice.id {
            let i = invoice_collection.find_one(doc! {"_id": id}).await;
            match i {
                Ok(Some(mut i)) => {
                    i.updated_date = Some(Local::now().naive_local());
                    i
                }
                Err(e) => return handle_err(e),
                _ => Default::default(),
            }
        } else {
            Default::default()
        }
    };
    if maybe_invoice.locked {
        // we cannot change a locked invoice.
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "You cannot modify a locked invoice"})),
        )
            .into_response();
    }
    let InvoiceUpsert {
        id: _,
        date_of_invoice,
        items,
        customer,
        invoicer,
        notes,
        template_id: maybe_template_id,
        locked,
    } = invoice;

    let mut invoice = Invoice {
        date_of_invoice,
        items,
        customer,
        invoicer,
        notes,
        locked,
        ..maybe_invoice
    };
    let options = FindOneAndReplaceOptions::builder()
        .upsert(Some(true))
        .build();

    if let Some(template_id) = maybe_template_id.filter(|t| !t.trim().is_empty()) {
        invoice.template_id = template_id;
    }

    // if this happens there will be no way to delete or modify the invoice anymore
    if invoice.locked {
        if invoice.template_id.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "template id is empty!!!"})),
            )
                .into_response();
        }
        let invoice_seq_collection = session
            .client()
            .database(&tenant)
            .collection::<InvoiceSeq>("invoice_seq");

        let seq = match invoice_seq_collection
            .find_one(doc! {"_id": INVOICE_SEQ_ROW_ID})
            .await
        {
            Ok(Some(mut seq)) => {
                seq.seq += 1;
                seq
            }
            Ok(None) => InvoiceSeq {
                id: INVOICE_SEQ_ROW_ID.to_string(),
                seq: 1,
            },
            Err(e) => return handle_err(e),
        };

        invoice.number = Some(format!("{}-{:03}", Local::now().format("%m%Y"), seq.seq));

        if let Err(e) = invoice_seq_collection
            .find_one_and_replace(doc! {"_id": INVOICE_SEQ_ROW_ID}, &seq)
            .with_options(options.clone())
            .await
        {
            return handle_err(e);
        }

        let logo_base64 = if let Some(logo_id) = invoice
            .invoicer
            .logo_id
            .as_ref()
            .filter(|id| !id.is_empty())
        {
            let logo_metadata = file_client
                .metadata(
                    &x_user_info_header,
                    DownloadFileRequestUriParams {
                        id: logo_id.clone(),
                    },
                )
                .await
                .map_err(|e| format!("could not download metadata for logo: {e}"))
                .unwrap();
            let ct = logo_metadata
                .content_type
                .unwrap_or_else(|| "image/png".into());
            let logo = file_client
                .download(
                    &x_user_info_header,
                    DownloadFileRequestUriParams {
                        id: logo_id.clone(),
                    },
                )
                .await
                .map_err(|e| format!("could not download logo: {e}"))
                .unwrap();
            let logo_base_64 = base64::engine::general_purpose::STANDARD.encode(logo);
            Some(format!("data:{ct};base64,{logo_base_64}"))
        } else {
            None
        };
        let invoice_file_name = format!("{}.pdf", invoice.id);

        let render_payload = json!({
           "logo": logo_base64,
           "invoice": invoice
        });
        let render_request = RenderRequest {
            template_id: invoice.template_id.clone(),
            context: render_payload,
            file_name: invoice_file_name.clone(),
            template_context: Context::Invoice,
        };

        let pdf_bytes = template_client
            .render(&x_user_info_header, &render_request)
            .await
            .unwrap();
        let upl = file_client
            .upload_bytes(
                &x_user_info_header,
                UploadFileRequestUriParams {
                    correlation_id: Some(invoice.id.clone()),
                    id: invoice.pdf_id.take(),
                    is_public: Some(false),
                },
                &invoice_file_name,
                &pdf_bytes,
            )
            .await
            .unwrap();
        invoice.pdf_id = Some(upl.id);
    }

    if let Err(e) = invoice_collection
        .find_one_and_replace(doc! {"_id": &invoice.id}, &invoice)
        .with_options(options)
        .await
    {
        return handle_err(e);
    }

    if let Err(e) = session.commit_transaction().await {
        return handle_err(e);
    }

    (StatusCode::OK, Json(invoice)).into_response()
}
