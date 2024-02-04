mod entity;
use std::{
    collections::HashMap,
    env::var,
    fs::File,
    io::BufReader,
    net::SocketAddr,
    path::{Path, PathBuf},
    str::FromStr,
};

use axum::{
    extract::Query, http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router,
};
use entity::{AuditLog, AuditLogConfig};
use sequeda_message_client::{Exchange, MessageClient};
use sequeda_service_common::{
    setup_tracing, user_header::ExtractUserInfo, StoreCollection, PUBLIC_TENANT,
    SERVICE_APPLICATION_NAME, SERVICE_COLLECTION_NAME, SERVICE_CONFIG_VOLUME, SERVICE_HOST,
    SERVICE_PORT,
};
use sequeda_store::{Pageable, Repository, StoreClient, StoreRepository};
use serde_json::json;

const CONFIG_FILE_NAME: &str = "CONFIG_FILE_NAME";

#[tokio::main]
async fn main() {
    setup_tracing();
    let host = var(SERVICE_HOST).unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = var(SERVICE_PORT).unwrap_or_else(|_| String::from("0"));
    let addr = SocketAddr::from_str(&format!("{host}:{port}")).unwrap();

    let app_name =
        var(SERVICE_APPLICATION_NAME).unwrap_or_else(|_| String::from("sequeda-auditlog-service"));

    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("auditlog"));

    let config_volume = var(SERVICE_CONFIG_VOLUME).unwrap_or_else(|_| String::from("/tmp"));

    let config_file_name = var(CONFIG_FILE_NAME).unwrap_or_else(|_| String::from("auditlog.json"));

    let path_config = PathBuf::from(config_volume).join(config_file_name);

    if !path_config.exists() {
        panic!("Missing config `{path_config:?}`");
    }

    let configs: Vec<AuditLogConfig> = deserialize_file(&path_config);

    let mut message_client = MessageClient::new(&app_name).await.unwrap();

    for AuditLogConfig {
        topic,
        header_message: _,
    } in &configs
    {
        tracing::info!("Subscribing to {topic}");
        message_client.subscribe(topic).await.unwrap();
    }
    let db_client = StoreClient::new(app_name.clone()).await.unwrap();

    let mut message_consumer = message_consumer(
        db_client.clone(),
        message_client,
        collection_name.clone(),
        configs,
    );

    let mut server = tokio::spawn(async move {
        tracing::info!("listening on {:?}", addr);
        let router = Router::new()
            .route("/find-all", get(find_all))
            .layer(Extension(StoreCollection(collection_name)))
            .layer(Extension(db_client));
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        axum::serve(listener, router.into_make_service())
            .await
            .unwrap();
    });
    tokio::select! {
        _ = (&mut server) => {
            message_consumer.abort();
        },
        _ = (&mut message_consumer) => {
            server.abort();
        },
    }
}

async fn find_all(
    pagination: Option<Query<Pageable>>,
    Extension(client): Extension<StoreClient>,
    ExtractUserInfo {
        user_info: x_user_info,
        ..
    }: ExtractUserInfo,
    Extension(collection): Extension<StoreCollection>,
) -> impl IntoResponse {
    tracing::debug!("Audit log page route entered!");
    let repository: StoreRepository<AuditLog> = StoreRepository::get_repository(
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

    match repository.find_page(None, page).await {
        Ok(logs) => (StatusCode::OK, Json(logs)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

fn deserialize_file(config_path: &Path) -> Vec<AuditLogConfig> {
    let file = File::open(config_path).unwrap();
    let buf_reader = BufReader::new(file);
    serde_json::from_reader(buf_reader).unwrap()
}

fn message_consumer(
    db_client: StoreClient,
    mut message_client: MessageClient,
    collection_name: String,
    configs: Vec<AuditLogConfig>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut repository_cache = HashMap::new();
        loop {
            if let Some(message) = message_client.recv().await {
                match message {
                    Ok(Exchange {
                        tenant,
                        message,
                        headers,
                        timestamp,
                        topic,
                    }) => {
                        let config = configs.iter().find(|c| c.topic == topic);
                        if let Some(config) = config {
                            let message = if let Some(header_message) = &config.header_message {
                                headers.get(header_message).cloned()
                            } else {
                                Some(Exchange::get_message_as_string(&message))
                            };

                            if let Some(message) = message {
                                let tenant = tenant.unwrap_or_else(|| PUBLIC_TENANT.to_string());
                                if !repository_cache.contains_key(&tenant) {
                                    let repository: StoreRepository<AuditLog> =
                                        StoreRepository::get_repository(
                                            db_client.clone(),
                                            &collection_name,
                                            &tenant,
                                        )
                                        .await;
                                    repository_cache.insert(tenant.clone(), repository);
                                }
                                let Some(repository) = repository_cache.get(&tenant) else {
                                    panic!("repository {tenant} not found")
                                };
                                let audit_log = AuditLog {
                                    message,
                                    received_date: timestamp,
                                    ..Default::default()
                                };
                                if let Err(e) = repository.insert_one(&audit_log).await {
                                    tracing::error!(
                                        "could not insert audit log {audit_log:?} to database. {e}"
                                    );
                                }
                            } else {
                                tracing::error!(
                                    r#"exchange could not be extracted: 
                                                 headers: {headers:?}, 
                                                 message: {message:?}, 
                                                 tenant: {tenant:?}, 
                                                 topic: {topic}, 
                                                 config: {config:?}"#
                                );
                            }
                        }
                    }
                    Err(e) => tracing::error!("error message received: {e}"),
                }
            }
        }
    })
}
