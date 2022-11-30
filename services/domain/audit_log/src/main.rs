mod entity;
use std::{
    collections::HashMap,
    env::var,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use entity::{AuditLog, AuditLogConfig};
use sequeda_message_client::{Exchange, MessageClient};
use sequeda_service_common::{
    setup_tracing, PUBLIC_TENANT, SERVICE_APPLICATION_NAME, SERVICE_COLLECTION_NAME,
    SERVICE_CONFIG_VOLUME, SERVICE_HOST, SERVICE_PORT,
};
use sequeda_store::{Repository, StoreClient, StoreRepository};

const CONFIG_FILE_NAME: &str = "CONFIG_FILE_NAME";

#[tokio::main]
async fn main() {
    setup_tracing();
    let host = var(SERVICE_HOST).unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = var(SERVICE_PORT).unwrap_or_else(|_| String::from("0"));

    let app_name =
        var(SERVICE_APPLICATION_NAME).unwrap_or_else(|_| String::from("sequeda-auditlog-service"));

    let collection_name: String =
        var(SERVICE_COLLECTION_NAME).unwrap_or_else(|_| String::from("auditlog"));

    let config_volume = var(SERVICE_CONFIG_VOLUME).unwrap_or_else(|_| String::from("/tmp"));

    let config_file_name = var(CONFIG_FILE_NAME).unwrap_or_else(|_| String::from("auditlog.json"));

    let path_config = PathBuf::new().join(&config_volume).join(config_file_name);

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

    let consumer_db_client = db_client.clone();
    let consumer_collection_name = collection_name.clone();

    let message_consumer = tokio::spawn(async move {
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
                                            consumer_db_client.clone(),
                                            &consumer_collection_name,
                                            &tenant,
                                        )
                                        .await;
                                    repository_cache.insert(tenant.clone(), repository);
                                }
                                let Some(repository) = repository_cache.get(&tenant)  else { panic!("repository {tenant} not found")};
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
    });
}

pub fn deserialize_file(config_path: &Path) -> Vec<AuditLogConfig> {
    let file = File::open(config_path).unwrap();
    let buf_reader = BufReader::new(file);
    serde_json::from_reader(buf_reader).unwrap()
}
