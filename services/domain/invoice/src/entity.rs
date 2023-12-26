use chrono::{Local, NaiveDateTime};
use sequeda_service_common::common_domain_types::{BankAccount, ContactDetail, UnitType};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    #[serde(rename = "_id")]
    pub id: String,
    pub creation_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
    pub number: String,
    pub reference: String,
    pub date_of_invoice: NaiveDateTime,
    pub items: Vec<InvoiceItem>,
    pub customer: Customer,
    pub invoicer: Invoicer,
    pub notes: Vec<String>,
    pub locked: bool,
    pub processed: bool,
}

impl Invoice {
    pub fn new(customer: Customer, invoicer: Invoicer) -> Self {
        Self {
            id: Default::default(),
            creation_date: Local::now().naive_local(),
            updated_date: Default::default(),
            number: Default::default(),
            reference: Default::default(),
            date_of_invoice: Default::default(),
            items: Default::default(),
            customer,
            invoicer,
            notes: Default::default(),
            locked: false,
            processed: false,
        }
    }
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

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoicer {
    pub invoicer_id: Option<String>,
    pub is_org: bool,
    pub invoicer_name: String,
    pub vat_number: Option<String>,
    pub contact_detail: ContactDetail,
    pub bank_accounts: Vec<BankAccount>,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub customer_id: Option<String>,
    pub customer_name: String,
    pub vat_number: Option<String>,
    pub contact_detail: ContactDetail,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
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
