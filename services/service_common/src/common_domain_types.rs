use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Default, PartialOrd, Serialize, Deserialize)]
pub struct Displayable<'a> {
    pub short_value: Option<&'a str>,
    pub medium_value: Option<&'a str>,
    pub long_value: &'a str,
    pub i18n_key: Option<&'a str>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnitType {
    Hour,
    Day,
    Unit,
    // Kg, Gr, ...
}

impl From<&UnitType> for Displayable<'static> {
    fn from(value: &UnitType) -> Self {
        match value {
            UnitType::Hour => Displayable {
                short_value: Some("h"),
                medium_value: None,
                long_value: "hour",
                i18n_key: Some("unit_type.hour"),
            },
            UnitType::Day => Displayable {
                short_value: Some("d"),
                medium_value: None,
                long_value: "day",
                i18n_key: Some("unit_type.day"),
            },
            UnitType::Unit => Displayable {
                short_value: Some(""),
                medium_value: None,
                long_value: "",
                i18n_key: Some("unit_type.unit"),
            },
        }
    }
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
