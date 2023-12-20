use chrono::{Local, NaiveDate, NaiveDateTime};
use sequeda_service_common::{BankAccount, ContactDetail, IdGenerator};
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "_id")]
    pub id: String,
    pub user_id: Option<String>,
    pub first_name: String,
    pub profile_picture_id: Option<String>,
    pub middle_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub creation_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
    pub nick_name: Option<String>,
    pub gender: Gender,
    pub marital_status: Option<MaritalStatus>,
    pub academic_title: Option<AcademicTitle>,
    pub contact_detail: ContactDetail,
    pub bank_account: Option<BankAccount>,
    pub signature_id: Option<String>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonUpsert {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub profile_picture_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub nick_name: Option<String>,
    pub marital_status: Option<MaritalStatus>,
    pub gender: Gender,
    pub academic_title: Option<AcademicTitle>,
    pub contact_detail: ContactDetail,
    pub bank_account: Option<BankAccount>,
    pub signature_id: Option<String>,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            id: IdGenerator.get(),
            user_id: Default::default(),
            first_name: Default::default(),
            last_name: Default::default(),
            profile_picture_id: Default::default(),
            date_of_birth: Default::default(),
            creation_date: Local::now().naive_local(),
            updated_date: Default::default(),
            marital_status: Default::default(),
            nick_name: Default::default(),
            gender: Gender::Unknown,
            academic_title: Default::default(),
            contact_detail: Default::default(),
            middle_name: Default::default(),
            bank_account: Default::default(),
            signature_id: Default::default(),
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

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AcademicTitle {
    Dr,
    Professor,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MaritalStatus {
    Single,
    Married,
    Divorced,
    Separated,
    CivilPartnership,
    Widowed,
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;
    use sequeda_service_common::{Address, BankAccount};

    use super::{Gender, Person};
    #[test]
    fn test_serialize() {
        let person: Person = Person {
            first_name: "Nordine".into(),
            last_name: "Bittich".into(),
            date_of_birth: NaiveDate::from_ymd_opt(1988, 3, 10).unwrap(),
            nick_name: Some("nbittich".into()),
            gender: Gender::Male,
            contact_detail: super::ContactDetail {
                email_address_1: "nordine@sequeda.eu".into(),
                phone_number_1: "0484/79.23.22".into(),
                address: Address {
                    street: "bekker street".into(),
                    number: "33".into(),
                    post_code: "3080".into(),
                    municipality: "Tervuren".into(),
                    country: "Belgium".into(),
                    ..Default::default()
                },
                ..Default::default()
            },
            bank_account: Some(BankAccount {
                number: "BEXXX XXX XXX XXX".into(),
                bic: Some("GEBABAB".into()),
            }),
            ..Default::default()
        };

        let json: String = serde_json::to_string_pretty(&person).unwrap();
        println!("{json}");

        let converted: Person = serde_json::from_str(&json).unwrap();
        assert_eq!(person, converted);
    }
}
