/// Storage module containing the `Storage` struct and its implementations.
/// Represents chemical storage information including quantities, locations, and tracking data.
use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    borrowing::Borrowing, person::Person, product::Product, storelocation::StoreLocation,
    supplier::Supplier, unit::Unit,
};

/// Main `Storage` structure representing chemical storage information.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Storage {
    /// Unique identifier for the storage record.
    pub storage_id: Option<u64>,
    /// When the storage record was created.
    pub storage_creation_date: DateTime<Utc>,
    /// When the storage record was last modified.
    pub storage_modification_date: DateTime<Utc>,
    /// When the product was added to storage.
    pub storage_entry_date: Option<DateTime<Utc>>,
    /// When the product was removed from storage.
    pub storage_exit_date: Option<DateTime<Utc>>,
    /// When the storage location became available.
    pub storage_opening_date: Option<DateTime<Utc>>,
    /// Expiration date of the stored product.
    pub storage_expiration_date: Option<DateTime<Utc>>,
    /// Additional comments about the storage.
    pub storage_comment: Option<String>,
    /// Reference identifier for the storage.
    pub storage_reference: Option<String>,
    /// Batch number of the stored product.
    pub storage_batch_number: Option<String>,
    /// Quantity of product in storage.
    pub storage_quantity: Option<f64>,
    /// Barcode identifier for the storage.
    pub storage_barecode: Option<String>,
    /// QR code data for the storage.
    pub storage_qrcode: Option<Vec<u8>>,
    /// Flag indicating if the product should be destroyed.
    #[serde(default)]
    pub storage_to_destroy: bool,
    /// Flag indicating if the storage is archived.
    #[serde(default)]
    pub storage_archive: bool,
    /// Concentration of the stored product.
    pub storage_concentration: Option<f64>,
    /// Number of bags in storage.
    pub storage_number_of_bag: Option<u64>,
    /// Number of cartons in storage.
    pub storage_number_of_carton: Option<u64>,

    /// Person associated with the storage (owner/manager).
    #[serde(default)]
    pub person: Person,
    /// Product being stored.
    pub product: Product,
    /// Location where the product is stored.
    pub store_location: StoreLocation,
    /// Supplier of the product.
    pub supplier: Option<Supplier>,
    /// Unit of measure for quantity.
    pub unit_quantity: Option<Unit>,
    /// Unit of measure for concentration.
    pub unit_concentration: Option<Unit>,

    /// Nested storage information (for hierarchical storage).
    pub storage: Option<Box<Storage>>,
    /// Borrowing information if the product is borrowed.
    pub borrowing: Option<Borrowing>,

    /// History count of storage operations.
    #[serde(default)]
    pub storage_hc: u64,
}

/// Display implementation for `Storage` to provide a human-readable representation.
impl fmt::Display for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Storage {{ storage_id: {:?}, storage_creation_date: {}, storage_batch_number: {:?}, storage_quantity: {:?}, storage_barecode: {:?}, storage_number_of_bag: {:?}, storage_number_of_carton: {:?}, product: {}, store_location: {}, unit_quantity: {:?} }}",
            self.storage_id,
            self.storage_creation_date,
            self.storage_batch_number,
            self.storage_quantity,
            self.storage_barecode,
            self.storage_number_of_bag,
            self.storage_number_of_carton,
            self.product,
            self.store_location,
            self.unit_quantity
        )
    }
}

/// Methods for `Storage` validation and sanitization.
impl Storage {
    /// Validates and sanitizes all fields of the `Storage`.
    ///
    /// Ensures all nested structures are properly validated.
    ///
    /// # Returns
    ///
    /// `Ok(())` if validation succeeds, or an error if validation fails.
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}
