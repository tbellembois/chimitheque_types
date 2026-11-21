/// Product module containing the `Product` struct and its implementations.
/// Represents a chemical or related product with various attributes and metadata.
use std::fmt;

use crate::validation::{
    check_finite, check_positive, check_positive_count, clean_optional_text, trim_optional_text,
};
use crate::{
    casnumber::CasNumber, category::Category, cenumber::CeNumber, classofcompound::ClassOfCompound,
    empiricalformula::EmpiricalFormula, entity::Entity, error::ValidationError,
    hazardstatement::HazardStatement, linearformula::LinearFormula, name::Name, person::Person,
    physicalstate::PhysicalState, precautionarystatement::PrecautionaryStatement,
    producerref::ProducerRef, producttype::ProductType, signalword::SignalWord,
    supplierref::SupplierRef, symbol::Symbol, tag::Tag, unit::Unit,
};
use serde::{Deserialize, Serialize};

/// Main `Product` structure representing a chemical or related product.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Product {
    /// Unique identifier for the product.
    pub product_id: Option<u64>,
    /// Type of product (chemical, consumable, biological).
    pub product_type: ProductType,
    /// International Chemical Identifier.
    pub product_inchi: Option<String>,
    /// `InChIKey` identifier.
    pub product_inchikey: Option<String>,
    /// Canonical SMILES representation.
    pub product_canonical_smiles: Option<String>,
    /// Specificity information about the product.
    pub product_specificity: Option<String>,
    /// Material Safety Data Sheet reference.
    pub product_msds: Option<String>,
    /// Flag indicating if product is restricted.
    #[serde(default)]
    pub product_restricted: bool,
    /// Flag indicating if product is radioactive.
    #[serde(default)]
    pub product_radioactive: bool,
    /// 2D chemical formula representation picture.
    pub product_twod_formula: Option<String>,
    /// 3D chemical formula representation link.
    pub product_threed_formula: Option<String>,
    /// Disposal instructions/comments.
    pub product_disposal_comment: Option<String>,
    /// Additional remarks about the product.
    pub product_remark: Option<String>,
    /// Molecular weight of the product.
    pub product_molecular_weight: Option<f64>,
    /// Temperature specification.
    pub product_temperature: Option<f64>,
    /// Product specification sheet reference.
    pub product_sheet: Option<String>,
    /// Number of products per carton.
    pub product_number_per_carton: Option<i64>,
    /// Number of products per bag.
    pub product_number_per_bag: Option<i64>,

    /// Associated person (owner/manager).
    #[serde(default)]
    pub person: Person,
    /// Product name information.
    pub name: Name,

    /// Empirical formula of the product.
    pub empirical_formula: Option<EmpiricalFormula>,
    /// Linear formula representation.
    pub linear_formula: Option<LinearFormula>,
    /// Physical state of the product.
    pub physical_state: Option<PhysicalState>,
    /// Signal word for hazard communication.
    pub signal_word: Option<SignalWord>,
    /// CAS registry number.
    pub cas_number: Option<CasNumber>,
    /// CE number (European Community number).
    pub ce_number: Option<CeNumber>,
    /// Reference to the producer.
    pub producer_ref: Option<ProducerRef>,
    /// Product category.
    pub category: Option<Category>,
    /// Unit for temperature measurement.
    pub unit_temperature: Option<Unit>,
    /// Unit for molecular weight measurement.
    pub unit_molecular_weight: Option<Unit>,

    /// Classes of compounds this product belongs to.
    pub classes_of_compound: Option<Vec<ClassOfCompound>>,
    /// Alternative names/synonyms for the product.
    pub synonyms: Option<Vec<Name>>,
    /// Hazard symbols associated with the product.
    pub symbols: Option<Vec<Symbol>>,
    /// Hazard statements describing risks.
    pub hazard_statements: Option<Vec<HazardStatement>>,
    /// Precautionary statements providing safety measures.
    pub precautionary_statements: Option<Vec<PrecautionaryStatement>>,
    /// References to suppliers.
    pub supplier_refs: Option<Vec<SupplierRef>>,
    /// Tags/categories for the product.
    pub tags: Option<Vec<Tag>>,

    /// Flag indicating if the product is bookmarked by the logged-in user.
    #[serde(default)]
    pub product_has_bookmark: bool,
    /// Archived storage count in the logged-in user's entities.
    pub product_asc: Option<u64>,
    /// Storage count in the logged-in user's entities.
    pub product_sc: Option<u64>,
    /// Total storage count across all entities.
    pub product_tsc: Option<u64>,
    /// Concatenated CMR (Carcinogenic, Mutagenic, Reproductive toxic) hazard statements.
    pub product_hs_cmr: Option<String>,
    /// Store location code.
    pub product_sl: Option<String>,
    /// Availability of the product in different entities.
    pub product_availability: Option<Vec<Entity>>,
}

/// Display implementation for `Product` to provide a human-readable representation.
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

/// Methods for `Product` validation and sanitization.
impl Product {
    /// Validates and sanitizes all fields of the `Product`.
    ///
    /// Ensures all nested structures are properly validated: the name, CAS and CE
    /// numbers, category, formulas, producer reference, supplier references,
    /// synonyms, classes of compounds and tags. Only new entries (without an id)
    /// are validated; existing ones are left unchanged.
    ///
    /// Optional text fields are cleaned; an empty value becomes `None`. The
    /// molecular weight must be greater than 0, the temperature a finite number,
    /// and the numbers per carton and per bag greater than 0.
    ///
    /// # Returns
    ///
    /// `Ok(())` if validation succeeds, or an error if validation fails.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        // Single-line identifiers: inner whitespace is collapsed.
        clean_optional_text(&mut self.product_inchi);
        clean_optional_text(&mut self.product_inchikey);
        clean_optional_text(&mut self.product_canonical_smiles);
        clean_optional_text(&mut self.product_specificity);
        // Free text, links and pictures: only trimmed, line breaks are kept.
        trim_optional_text(&mut self.product_msds);
        trim_optional_text(&mut self.product_sheet);
        trim_optional_text(&mut self.product_twod_formula);
        trim_optional_text(&mut self.product_threed_formula);
        trim_optional_text(&mut self.product_disposal_comment);
        trim_optional_text(&mut self.product_remark);

        check_positive(self.product_molecular_weight, "product_molecular_weight")?;
        check_finite(self.product_temperature, "product_temperature")?;
        check_positive_count(self.product_number_per_carton, "product_number_per_carton")?;
        check_positive_count(self.product_number_per_bag, "product_number_per_bag")?;

        if let Some(ref mut cas_number) = self.cas_number
            && cas_number.cas_number_id.is_none()
        {
            cas_number.sanitize_and_validate()?;
        }

        if let Some(ref mut ce_number) = self.ce_number
            && ce_number.ce_number_id.is_none()
        {
            ce_number.sanitize_and_validate()?;
        }

        if let Some(ref mut category) = self.category
            && category.category_id.is_none()
        {
            category.sanitize_and_validate()?;
        }

        if let Some(ref mut empirical_formula) = self.empirical_formula
            && empirical_formula.empirical_formula_id.is_none()
        {
            empirical_formula.sanitize_and_validate()?;
        }

        if let Some(ref mut linear_formula) = self.linear_formula
            && linear_formula.linear_formula_id.is_none()
        {
            linear_formula.sanitize_and_validate()?;
        }

        if self.name.name_id.is_none() {
            self.name.sanitize_and_validate()?;
        }

        if let Some(ref mut producer_ref) = self.producer_ref
            && producer_ref.producer_ref_id.is_none()
        {
            producer_ref.sanitize_and_validate()?;
        }

        if let Some(supplier_refs) = self.supplier_refs.as_mut() {
            for supplier_ref in supplier_refs {
                if supplier_ref.supplier_ref_id.is_none() {
                    supplier_ref.sanitize_and_validate()?;
                }
            }
        }

        if let Some(synonyms) = self.synonyms.as_mut() {
            for synonym in synonyms {
                if synonym.name_id.is_none() {
                    synonym.sanitize_and_validate()?;
                }
            }
        }

        if let Some(classes_of_compound) = self.classes_of_compound.as_mut() {
            for class_of_compound in classes_of_compound {
                if class_of_compound.class_of_compound_id.is_none() {
                    class_of_compound.sanitize_and_validate()?;
                }
            }
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

#[cfg(test)]
#[path = "product_tests.rs"]
mod product_tests;
