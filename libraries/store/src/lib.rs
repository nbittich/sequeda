mod client;
mod constants;
mod repository;

use std::error::Error;
use std::fmt::Display;

pub use client::StoreClient;
pub use constants::{MONGO_ADMIN_DATABASE, MONGO_HOST, MONGO_PASSWORD, MONGO_PORT, MONGO_USERNAME};
pub use mongodb::{
    bson::{doc, oid::ObjectId, to_document, Document, Regex},
    error::Error as MongoError,
    options::{ClientOptions, FindOneAndReplaceOptions, FindOptions},
    results::{DeleteResult, InsertManyResult, InsertOneResult, UpdateResult},
    Client, ClientSession, Collection, Cursor, Database,
};
pub use repository::{Page, Pageable, Repository, StoreRepository};
use serde::{Deserialize, Serialize};
pub use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreError {
    msg: String,
}

impl Error for StoreError {}

impl Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
