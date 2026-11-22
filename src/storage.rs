//! Storage module containing the `Storage` struct and its implementations.
//! Represents chemical storage information including quantities, locations, and tracking data.

use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::unittype::UnitType;
use crate::validation::{
    check_non_negative, check_non_zero_count, check_unit_type, clean_optional_text, require_if,
    trim_optional_text,
};

use crate::{
    borrowing::Borrowing, error::ValidationError, person::Person, product::Product,
    storelocation::StoreLocation, supplier::Supplier, unit::Unit,
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
    /// When the product container was opened.
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

    /// Person who last saved the storage, set from the request headers.
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

    /// For a history row, the storage it is an earlier version of.
    pub storage: Option<Box<Storage>>,
    /// Borrowing information if the product is borrowed.
    pub borrowing: Option<Borrowing>,

    /// Number of history rows of the storage.
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
    /// Validates a new supplier (without an id); an existing one is left
    /// unchanged. Validates the borrowing, if any.
    ///
    /// Optional text fields are cleaned; an empty value becomes `None`. The
    /// quantity and concentration must be 0 or more, each with its unit, the
    /// numbers of bags and cartons greater than 0, and the exit, opening and
    /// expiration dates not before the entry date.
    ///
    /// # Returns
    ///
    /// `Ok(())` if validation succeeds, or an error if validation fails.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        trim_optional_text(&mut self.storage_comment);
        clean_optional_text(&mut self.storage_reference);
        clean_optional_text(&mut self.storage_batch_number);
        clean_optional_text(&mut self.storage_barecode);

        check_non_negative(self.storage_quantity, "storage_quantity")?;
        check_non_negative(self.storage_concentration, "storage_concentration")?;
        check_non_zero_count(self.storage_number_of_bag, "storage_number_of_bag")?;
        check_non_zero_count(self.storage_number_of_carton, "storage_number_of_carton")?;
        require_if(
            self.storage_quantity.is_some(),
            self.unit_quantity.is_some(),
            "unit_quantity",
            "when storage_quantity is set",
        )?;
        require_if(
            self.storage_concentration.is_some(),
            self.unit_concentration.is_some(),
            "unit_concentration",
            "when storage_concentration is set",
        )?;

        // Exit, opening and expiration dates can not be before the entry date.
        if let Some(entry_date) = self.storage_entry_date {
            for (field, date) in [
                ("storage_exit_date", self.storage_exit_date),
                ("storage_opening_date", self.storage_opening_date),
                ("storage_expiration_date", self.storage_expiration_date),
            ] {
                if date.is_some_and(|date| date < entry_date) {
                    return Err(ValidationError::OutOfRange {
                        field,
                        expected: "on or after storage_entry_date",
                    });
                }
            }
        }

        if let Some(ref mut supplier) = self.supplier
            && supplier.supplier_id.is_none()
        {
            supplier.sanitize_and_validate()?;
        }

        if let Some(ref mut borrowing) = self.borrowing {
            borrowing.sanitize_and_validate()?;
        }

        Ok(())
    }

    /// Checks that `unit_quantity` is a quantity unit and `unit_concentration`
    /// a concentration unit.
    ///
    /// The unit type is looked up from the unit id with `unit_type_of`, not
    /// read from the request: a client may send only the unit id, in which case
    /// `unit_type` holds its default value. Call it after `sanitize_and_validate`,
    /// with the unit types from the database.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidUnit` for a unit of the wrong type, or a
    /// unit without an id or with an unknown id.
    pub fn validate_unit_types(
        &self,
        unit_type_of: impl Fn(u64) -> Option<UnitType>,
    ) -> Result<(), ValidationError> {
        check_unit_type(
            self.unit_quantity.as_ref(),
            "unit_quantity",
            UnitType::Quantity,
            &unit_type_of,
        )?;
        check_unit_type(
            self.unit_concentration.as_ref(),
            "unit_concentration",
            UnitType::Concentration,
            &unit_type_of,
        )
    }
}

#[cfg(test)]
#[path = "storage_tests.rs"]
mod storage_tests;
