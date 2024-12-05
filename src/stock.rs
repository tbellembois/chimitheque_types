use serde::{Deserialize, Serialize};

use crate::{product::Product, storelocation::StoreLocation, unit::Unit};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Stock {
    pub store_location: StoreLocation,
    pub product: Product,

    pub quantity: f64,
    pub unit: Option<Unit>,
}
