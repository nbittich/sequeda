use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductUnitType {
    Hour,
    Day,
    Unit,
    // Kg, Gr, ...
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Gender {
    Male,
    Female,
    Unknown,
}
#[derive(Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactDetail {
    pub email_address_1: String,
    pub email_address_2: Option<String>,
    pub phone_number_1: String,
    pub phone_number_2: Option<String>,
    pub website: Option<String>,
    pub address: Address,
}

#[derive(Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub street: String,
    pub number: String,
    pub box_number: Option<String>,
    pub post_code: String,
    pub municipality: String,
    pub province: Option<String>,
    pub country: String,
}

#[derive(Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankAccount {
    pub number: String,
    pub bic: Option<String>,
}
