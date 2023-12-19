use chrono::{Local, NaiveDate, NaiveDateTime};
use sequeda_service_common::IdGenerator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    #[serde(rename = "_id")]
    pub id: String,
    pub org_id: String,
    pub person_id: String,
    pub position_id: String,
    pub responsible_of: Option<Vec<String>>,
    pub managed_by: Vec<String>,
    pub started: NaiveDate,
    pub ended: Option<NaiveDate>,
    pub remarks: Vec<Remark>,
    pub creation_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
}

impl Default for Member {
    fn default() -> Self {
        Self {
            id: IdGenerator.get(),
            org_id: Default::default(),
            person_id: Default::default(),
            position_id: Default::default(),
            responsible_of: Default::default(),
            managed_by: Default::default(),
            started: Default::default(),
            ended: Default::default(),
            remarks: Default::default(),
            creation_date: Local::now().naive_local(),
            updated_date: Default::default(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remark {
    pub id: Option<String>,
    pub added_by_user_id: Option<String>,
    pub updated_by_user_id: Option<String>,
    pub added_date: Option<NaiveDateTime>,
    pub updated_date: Option<NaiveDateTime>,
    pub message: String,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberUpsert {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub org_id: String,
    pub person_id: String,
    pub position_id: String,
    pub managed_by: Vec<String>,
    pub started: NaiveDate,
    pub ended: Option<NaiveDate>,
    pub remarks: Vec<Remark>,
}
