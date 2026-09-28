//! Product module containing the Product struct and its implementations.
//! Represents a chemical or related product with various attributes and metadata.

use std::fmt;

use crate::{
    casnumber::CasNumber, category::Category, cenumber::CeNumber, classofcompound::ClassOfCompound,
    empiricalformula::EmpiricalFormula, entity::Entity, hazardstatement::HazardStatement,
    linearformula::LinearFormula, name::Name, person::Person, physicalstate::PhysicalState,
    precautionarystatement::PrecautionaryStatement, producerref::ProducerRef,
    producttype::ProductType, signalword::SignalWord, supplierref::SupplierRef, symbol::Symbol,
    tag::Tag, unit::Unit,
};
use serde::{Deserialize, Serialize};

/// Main Product structure representing a chemical or related product
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Product {
    /// Unique identifier for the product
    pub product_id: Option<u64>,
    /// Type of product (chemical, consumable, biological)
    pub product_type: ProductType,
    /// International Chemical Identifier
    pub product_inchi: Option<String>,
    /// InChIKey identifier
    pub product_inchikey: Option<String>,
    /// Canonical SMILES representation
    pub product_canonical_smiles: Option<String>,
    /// Specificity information about the product
    pub product_specificity: Option<String>,
    /// Material Safety Data Sheet reference
    pub product_msds: Option<String>,
    /// Flag indicating if product is restricted
    #[serde(default)]
    pub product_restricted: bool,
    /// Flag indicating if product is radioactive
    #[serde(default)]
    pub product_radioactive: bool,
    /// 2D chemical formula representation
    pub product_twod_formula: Option<String>,
    /// 3D chemical formula representation
    pub product_threed_formula: Option<String>,
    /// Disposal instructions/comments
    pub product_disposal_comment: Option<String>,
    /// Additional remarks about the product
    pub product_remark: Option<String>,
    /// Molecular weight of the product
    pub product_molecular_weight: Option<f64>,
    /// Temperature specification
    pub product_temperature: Option<f64>,
    /// Product specification sheet reference
    pub product_sheet: Option<String>,
    /// Number of products per carton
    pub product_number_per_carton: Option<i64>,
    /// Number of products per bag
    pub product_number_per_bag: Option<i64>,

    /// Associated person (owner/manager)
    #[serde(default)]
    pub person: Person,
    /// Product name information
    pub name: Name,

    /// Empirical formula of the product
    pub empirical_formula: Option<EmpiricalFormula>,
    /// Linear formula representation
    pub linear_formula: Option<LinearFormula>,
    /// Physical state of the product
    pub physical_state: Option<PhysicalState>,
    /// Signal word for hazard communication
    pub signal_word: Option<SignalWord>,
    /// CAS registry number
    pub cas_number: Option<CasNumber>,
    /// CE number (European Community number)
    pub ce_number: Option<CeNumber>,
    /// Reference to the producer
    pub producer_ref: Option<ProducerRef>,
    /// Product category
    pub category: Option<Category>,
    /// Unit for temperature measurement
    pub unit_temperature: Option<Unit>,
    /// Unit for molecular weight measurement
    pub unit_molecular_weight: Option<Unit>,

    /// Classes of compounds this product belongs to
    pub classes_of_compound: Option<Vec<ClassOfCompound>>,
    /// Alternative names/synonyms for the product
    pub synonyms: Option<Vec<Name>>,
    /// Hazard symbols associated with the product
    pub symbols: Option<Vec<Symbol>>,
    /// Hazard statements describing risks
    pub hazard_statements: Option<Vec<HazardStatement>>,
    /// Precautionary statements providing safety measures
    pub precautionary_statements: Option<Vec<PrecautionaryStatement>>,
    /// References to suppliers
    pub supplier_refs: Option<Vec<SupplierRef>>,
    /// Tags/categories for the product
    pub tags: Option<Vec<Tag>>,

    /// Flag indicating if the product is bookmarked by the logged-in user
    #[serde(default)]
    pub product_has_bookmark: bool,
    /// Archived storage count in the logged-in user's entities
    pub product_asc: Option<u64>,
    /// Storage count in the logged-in user's entities
    pub product_sc: Option<u64>,
    /// Total storage count across all entities
    pub product_tsc: Option<u64>,
    /// Concatenated CMR (Carcinogenic, Mutagenic, Reproductive toxic) hazard statements
    pub product_hs_cmr: Option<String>,
    /// Store location code
    pub product_sl: Option<String>,
    /// Availability of the product in different entities
    pub product_availability: Option<Vec<Entity>>,
}

/// Display implementation for Product to provide a human-readable representation
impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Product {{ product_id: {:?}, name: {}, cas_number: {} }}",
            self.product_id,
            self.name.name_label,
            self.cas_number
                .as_ref()
                .map_or("None".to_string(), |cas| cas.cas_number_label.clone())
        )
    }
}

/// Methods for Product validation and sanitization
impl Product {
    /// Validates and sanitizes all fields of the Product
    /// Ensures all nested structures are properly validated
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(cas_number) = self.cas_number.as_ref()
            && cas_number.cas_number_id.is_none()
        {
            self.cas_number.as_mut().unwrap().sanitize_and_validate()?;
        }

        if let Some(ce_number) = self.ce_number.as_ref()
            && ce_number.ce_number_id.is_none()
        {
            self.ce_number.as_mut().unwrap().sanitize_and_validate()?;
        }

        if let Some(category) = self.category.as_ref()
            && category.category_id.is_none()
        {
            self.category.as_mut().unwrap().sanitize_and_validate()?;
        }

        if let Some(empirical_formula) = self.empirical_formula.as_ref()
            && empirical_formula.empirical_formula_id.is_none()
        {
            self.empirical_formula
                .as_mut()
                .unwrap()
                .sanitize_and_validate()?;
        }

        if let Some(linear_formula) = self.linear_formula.as_ref()
            && linear_formula.linear_formula_id.is_none()
        {
            self.linear_formula
                .as_mut()
                .unwrap()
                .sanitize_and_validate()?;
        }

        if self.name.name_id.is_none() {
            self.name.sanitize_and_validate()?;
        }

        if let Some(tags) = self.tags.as_mut() {
            for tag in tags {
                if tag.tag_id.is_none() {
                    tag.sanitize_and_validate()?;
                }
            }
        }

        Ok(())
    }
}
