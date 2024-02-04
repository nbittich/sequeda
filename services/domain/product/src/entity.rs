use chrono::{Local, NaiveDateTime};
use sequeda_service_common::{common_domain_types::UnitType, IdGenerator};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductItem {
    #[serde(rename = "_id")]
    pub id: String,
    pub label: String,
    pub name: String,
    pub main_picture_id: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub creation_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
    pub price_per_unit: f64,
    pub vat: usize,
    pub unit_type: UnitType,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductTag {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
}

impl Default for ProductItem {
    fn default() -> Self {
        Self {
            id: IdGenerator.get(),
            label: Default::default(),
            name: Default::default(),
            main_picture_id: Default::default(),
            description: Default::default(),
            tags: Default::default(),
            creation_date: Local::now().naive_local(),
            updated_date: Default::default(),
            price_per_unit: Default::default(),
            vat: 21,
            unit_type: UnitType::Hour,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductItemUpsert {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub name: String,
    pub label: String,
    pub main_picture_id: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub price_per_unit: f64,
    pub vat: Option<usize>,
    pub unit_type: UnitType,
}
