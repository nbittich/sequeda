use chrono::{Local, NaiveDateTime};
use sequeda_service_common::common_domain_types::ProductUnitType;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductItem {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub main_picture_id: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub creation_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
    pub price_per_unit: f64,
    pub unit_type: ProductUnitType,
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
            id: Default::default(),
            name: Default::default(),
            main_picture_id: Default::default(),
            description: Default::default(),
            tags: Default::default(),
            creation_date: Local::now().naive_local(),
            updated_date: Default::default(),
            price_per_unit: Default::default(),
            unit_type: ProductUnitType::Hour,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductItemUpsert {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub name: String,
    pub main_picture_id: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub price_per_unit: f64,
    pub unit_type: ProductUnitType,
}
