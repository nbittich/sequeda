use sequeda_service_common::common_domain_types::UnitType;

pub struct Invoice {}

pub struct InvoiceItem {
    pub name: String,
    pub description: Option<String>,
    pub qty: i32,
    pub vat: usize,
    pub price_per_unit: f64,
    pub unit_type: UnitType,
}

impl InvoiceItem {
    pub fn get_total_vat(&self) -> f64 {
        (self.qty * self.vat as i32) as f64 / 100.
    }

    pub fn get_total_vat_excl(&self) -> f64 {
        (self.qty * self.price_per_unit as i32) as f64 / 100.
    }
}
