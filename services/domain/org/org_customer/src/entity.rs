use chrono::{Local, NaiveDate, NaiveDateTime};
use sequeda_service_common::IdGenerator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    #[serde(rename = "_id")]
    pub id: String,
    pub customer_type: CustomerType,
    pub org_id: String,
    pub represented_by_id: String,
    pub started: NaiveDate,
    pub ended: Option<NaiveDate>,
    pub document_ids: Vec<String>,
    pub communications: Vec<Communication>,
    pub creation_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomerType {
    Person,
    Organization,
}

impl Default for CustomerType {
    fn default() -> Self {
        CustomerType::Organization
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Communication {
    pub id: Option<String>,
    pub added_by_user_id: Option<String>,
    pub updated_by_user_id: Option<String>,
    pub message: String,
    pub added_date: Option<NaiveDateTime>,
    pub updated_date: Option<NaiveDateTime>,
}

impl Default for Customer {
    fn default() -> Self {
        Customer {
            id: IdGenerator.get(),
            started: Default::default(),
            ended: Default::default(),
            org_id: Default::default(),
            customer_type: Default::default(),
            represented_by_id: Default::default(),
            document_ids: Default::default(),
            communications: Default::default(),
            creation_date: Local::now().naive_local(),
            updated_date: Default::default(),
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerUpsert {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub org_id: String,
    pub customer_type: CustomerType,
    pub started: NaiveDate,
    pub ended: Option<NaiveDate>,
    pub represented_by_id: String,
    pub document_ids: Vec<String>,
    pub communications: Vec<Communication>,
}
