use chrono::{Local, NaiveDateTime};
use rand::{distributions::Alphanumeric, Rng};
use sequeda_service_common::{
    common_domain_types::{BankAccount, ContactDetail, UnitType},
    IdGenerator,
};

use serde::{Deserialize, Serialize};

pub const INVOICE_SEQ_ROW_ID: &str = "1a6b5960-f531-4327-b529-a655b11e38f8";

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceSeq {
    #[serde(rename = "_id")]
    pub id: String,
    pub seq: u64,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    #[serde(rename = "_id")]
    pub id: String,
    pub pdf_id: Option<String>,
    pub creation_date: NaiveDateTime,
    pub template_id: String,
    pub updated_date: Option<NaiveDateTime>,
    pub number: Option<String>,
    pub reference: String,
    pub date_of_invoice: NaiveDateTime,
    pub items: Vec<InvoiceItem>,
    pub customer: Customer,
    pub invoicer: Invoicer,
    pub notes: Vec<String>,
    pub locked: bool,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceUpsert {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub template_id: Option<String>,
    pub date_of_invoice: NaiveDateTime,
    pub items: Vec<InvoiceItem>,
    pub customer: Customer,
    pub invoicer: Invoicer,
    pub notes: Vec<String>,
    pub locked: bool,
}

impl Default for Invoice {
    fn default() -> Self {
        Self {
            pdf_id: None,
            id: IdGenerator.get(),
            creation_date: Local::now().naive_local(),
            updated_date: Default::default(),
            number: Default::default(),
            template_id: Default::default(),
            reference: format!(
                "{}-{}",
                Local::now().format("%m%Y"),
                rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(5)
                    .map(char::from)
                    .map(|c| c.to_ascii_uppercase())
                    .collect::<String>()
            ),
            date_of_invoice: Default::default(),
            items: Default::default(),
            customer: Default::default(),
            invoicer: Default::default(),
            notes: Default::default(),
            locked: false,
        }
    }
}
#[allow(unused)]
impl Invoice {
    pub fn get_sub_total(&self) -> f64 {
        round(self.items.iter().map(|i| i.get_sub_total()).sum(), 2)
    }
    pub fn get_total(&self) -> f64 {
        round(self.items.iter().map(|i| i.get_total()).sum(), 2)
    }
    pub fn get_total_vat(&self) -> f64 {
        round(self.items.iter().map(|i| i.get_total_vat()).sum(), 2)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Invoicer {
    pub logo_id: Option<String>,
    pub invoicer_id: Option<String>,
    pub is_org: bool,
    pub invoicer_name: String,
    pub vat_number: Option<String>,
    pub contact_detail: ContactDetail,
    pub bank_accounts: Vec<BankAccount>,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub customer_id: Option<String>,
    pub customer_name: String,
    pub vat_number: Option<String>,
    pub contact_detail: ContactDetail,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceItem {
    pub name: String,
    pub description: Option<String>,
    pub qty: f64,
    pub vat: usize,
    pub price_per_unit: f64,
    pub unit_type: UnitType,
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}

impl InvoiceItem {
    pub fn get_total_vat(&self) -> f64 {
        round(self.get_sub_total() * self.vat as f64 / 100., 2)
    }
    pub fn get_sub_total(&self) -> f64 {
        round(self.qty * self.price_per_unit, 2)
    }
    pub fn get_total(&self) -> f64 {
        round(self.get_sub_total() + self.get_total_vat(), 2)
    }
}

#[cfg(test)]
mod test {
    use super::InvoiceItem;

    #[test]
    fn test_calc() {
        let item = InvoiceItem {
            name: "a".into(),
            description: None,
            qty: 88.,
            vat: 21,
            price_per_unit: 85.05,
            unit_type: sequeda_service_common::common_domain_types::UnitType::Hour,
        };
        assert_eq!(item.get_sub_total(), 7484.40);
        assert_eq!(item.get_total_vat(), 1571.72);
        assert_eq!(item.get_total(), 9056.12);

        let item = InvoiceItem {
            name: "b".into(),
            description: None,
            qty: 12.,
            vat: 21,
            price_per_unit: 63.38 * 8.,
            unit_type: sequeda_service_common::common_domain_types::UnitType::Day,
        };
        assert_eq!(item.get_sub_total(), 6084.48);
        assert_eq!(item.get_total_vat(), 1277.74);
        assert_eq!(item.get_total(), 7362.22);
    }
}
