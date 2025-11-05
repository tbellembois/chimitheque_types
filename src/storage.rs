use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    borrowing::Borrowing, person::Person, product::Product, storelocation::StoreLocation,
    supplier::Supplier, unit::Unit,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Storage {
    pub storage_id: Option<u64>,
    pub storage_creation_date: DateTime<Utc>,
    pub storage_modification_date: DateTime<Utc>,
    pub storage_entry_date: Option<DateTime<Utc>>,
    pub storage_exit_date: Option<DateTime<Utc>>,
    pub storage_opening_date: Option<DateTime<Utc>>,
    pub storage_expiration_date: Option<DateTime<Utc>>,
    pub storage_comment: Option<String>,
    pub storage_reference: Option<String>,
    pub storage_batch_number: Option<String>,
    pub storage_quantity: Option<f64>,
    pub storage_barecode: Option<String>,
    pub storage_qrcode: Option<Vec<u8>>,
    #[serde(default)]
    pub storage_to_destroy: bool,
    #[serde(default)]
    pub storage_archive: bool,
    pub storage_concentration: Option<f64>,
    pub storage_number_of_bag: Option<u64>,
    pub storage_number_of_carton: Option<u64>,

    #[serde(default)]
    pub person: Person,
    pub product: Product,
    pub store_location: StoreLocation,
    pub supplier: Option<Supplier>,
    pub unit_quantity: Option<Unit>,
    pub unit_concentration: Option<Unit>,

    pub storage: Option<Box<Storage>>,
    pub borrowing: Option<Borrowing>,

    // pub storage_number_of_unit: Option<u64>,

    // storage history count
    #[serde(default)]
    pub storage_hc: u64,
}
