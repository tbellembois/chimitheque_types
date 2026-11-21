use serde::{Deserialize, Serialize};

use crate::{product::Product, storelocation::StoreLocation, unit::Unit};

/// The total quantity of a product in a store location.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Stock {
    /// Store location holding the product.
    pub store_location: StoreLocation,
    /// Stocked product.
    pub product: Product,

    /// Total quantity, expressed in `unit`.
    pub quantity: f64,
    /// Unit of `quantity`.
    pub unit: Option<Unit>,
}

#[cfg(test)]
#[path = "stock_tests.rs"]
mod stock_tests;
