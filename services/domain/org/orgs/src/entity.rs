use chrono::{Local, NaiveDate, NaiveDateTime};
use sequeda_service_common::{BankAccount, ContactDetail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(rename = "_id")]
    pub id: String,
    pub creation_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
    pub name: String,
    pub description: String,
    pub vat_number: String,
    pub logo_id: Option<String>,
    pub primary_contact: ContactDetail,
    pub other_contacts: Vec<ContactDetail>,
    pub bank_accounts: Vec<BankAccount>,
    pub founded_date: NaiveDate,
    pub closed_date: Option<NaiveDate>,
    pub status: Status,
    pub current: bool,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationUpsert {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub vat_number: String,
    pub logo_id: Option<String>,
    pub primary_contact: ContactDetail,
    pub other_contacts: Vec<ContactDetail>,
    pub bank_accounts: Vec<BankAccount>,
    pub founded_date: NaiveDate,
    pub closed_date: Option<NaiveDate>,
    pub status: Status,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Active, // The company has been incorporated and exists on the register of companies
    ProposalToStrikeOff, // The company is in the process of being closed down.
    Dissolved, // The company has been closed down.
    Liquidation, //  The company’s assets are currently being sold to creditors and the business is being closed down.
}

impl Default for Organization {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            creation_date: Local::now().naive_local(),
            updated_date: Default::default(),
            status: Status::Active,
            founded_date: Local::now().date_naive(),
            name: Default::default(),
            description: Default::default(),
            vat_number: Default::default(),
            logo_id: Default::default(),
            primary_contact: Default::default(),
            other_contacts: Default::default(),
            bank_accounts: Default::default(),
            closed_date: Default::default(),
            current: Default::default(),
        }
    }
}
