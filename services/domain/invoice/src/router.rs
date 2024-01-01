use std::env::var;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use chrono::Local;
use sequeda_service_common::{
    user_header::ExtractUserInfo, StoreCollection, SERVICE_COLLECTION_NAME,
};
use sequeda_store::{doc, FindOneAndReplaceOptions, MongoError, StoreClient};
use serde_json::json;

use crate::entity::{Invoice, InvoiceSeq, InvoiceUpsert, INVOICE_SEQ_ROW_ID};

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
    Extension(StoreCollection(collection)): Extension<StoreCollection>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
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
    let mut session = match client.start_session(None).await {
        Ok(session) => session,
        Err(e) => return handle_err(e),
    };

    if let Err(e) = session.start_transaction(None).await {
        return handle_err(e);
    }
    let invoice_collection = session
        .client()
        .database(&tenant)
        .collection::<Invoice>(&collection);
    let maybe_invoice = {
        if let Some(id) = &invoice.id {
            let i = invoice_collection
                .find_one_with_session(doc! {"_id": id}, None, &mut session)
                .await;
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

    // if this happens there will be no way to delete or modify the invoice anymore
    if invoice.locked {
        let invoice_seq_collection = session
            .client()
            .database(&tenant)
            .collection::<InvoiceSeq>("invoice_seq");

        let seq = match invoice_seq_collection
            .find_one_with_session(doc! {"_id": INVOICE_SEQ_ROW_ID}, None, &mut session)
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
            .find_one_and_replace_with_session(
                doc! {"_id": INVOICE_SEQ_ROW_ID},
                &seq,
                options.clone(),
                &mut session,
            )
            .await
        {
            return handle_err(e);
        }
    }
    if let Err(e) = invoice_collection
        .find_one_and_replace_with_session(
            doc! {"_id": &invoice.id},
            &invoice,
            options,
            &mut session,
        )
        .await
    {
        return handle_err(e);
    }

    if let Err(e) = session.commit_transaction().await {
        return handle_err(e);
    }

    (StatusCode::OK, Json(invoice)).into_response()
}
