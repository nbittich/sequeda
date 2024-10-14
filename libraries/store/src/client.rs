use tracing::info;

use crate::constants::{
    MONGO_ADMIN_DATABASE, MONGO_HOST, MONGO_PASSWORD, MONGO_PORT, MONGO_USERNAME,
};
use crate::{doc, StoreError};

use crate::{Client, ClientOptions, Database};
use std::env::var;

#[derive(Debug, Clone)]
pub struct StoreClient {
    client: Client,
}

impl StoreClient {
    pub async fn new(application_name: String) -> Result<StoreClient, StoreError> {
        let client = StoreClient::create_client(application_name.clone()).await?;

        Ok(StoreClient { client })
    }

    pub fn get_raw_client(&self) -> Client {
        self.client.clone()
    }

    pub fn get_db(&self, database_name: &str) -> Database {
        let client = self.get_raw_client();
        client.database(database_name)
    }

    #[tracing::instrument]
    async fn create_client(application_name: String) -> Result<Client, StoreError> {
        let mongo_host = var(MONGO_HOST).unwrap_or_else(|_| String::from("127.0.0.1"));
        let mongo_port = var(MONGO_PORT).unwrap_or_else(|_| String::from("27017"));
        let mongo_username = var(MONGO_USERNAME).unwrap_or_else(|_| String::from("root"));
        let mongo_password = var(MONGO_PASSWORD).unwrap_or_else(|_| String::from("root"));
        let mongo_admin_db = var(MONGO_ADMIN_DATABASE).unwrap_or_else(|_| String::from("admin"));
        let mut client_options = ClientOptions::parse(format!(
            "mongodb://{mongo_username}:{mongo_password}@{mongo_host}:{mongo_port}"
        ))
        .await
        .map_err(|e| StoreError { msg: e.to_string() })?;
        client_options.app_name = Some(application_name);
        let client =
            Client::with_options(client_options).map_err(|e| StoreError { msg: e.to_string() })?;

        let _ = client
            .database(&mongo_admin_db)
            .run_command(doc! {"ping": 1})
            .await
            .map_err(|e| StoreError { msg: e.to_string() })?;

        info!("Successfully connected");
        Ok(client)
    }
}
