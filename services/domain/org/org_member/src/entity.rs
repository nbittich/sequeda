use chrono::{Local, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    #[serde(rename = "_id")]
    pub id: String,
    pub org_id: String,
    pub person_id: String,
    pub position_id: String,
    pub responsible_of: Vec<String>,
    pub managed_by: Option<String>,
    pub started: NaiveDate,
    pub ended: NaiveDate,
    pub remarks: Vec<Remark>,
    pub creation_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
}

impl Default for Member {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
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
    pub added_date: NaiveDateTime,
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
    pub responsible_of: Vec<String>,
    pub managed_by: Option<String>,
    pub started: NaiveDate,
    pub ended: NaiveDate,
    pub remarks: Vec<Remark>,
}
