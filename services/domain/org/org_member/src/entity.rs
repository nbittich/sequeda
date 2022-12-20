use chrono::{NaiveDate, NaiveDateTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Member {
    #[serde(rename = "_id")]
    pub id : String,
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


#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Remark {
    pub added_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
    pub message: String,
}