use std::error::Error;

use crate::{
    casnumber::CasNumber, category::Category, cenumber::CeNumber, classofcompound::ClassOfCompound,
    empiricalformula::EmpiricalFormula, entity::Entity, hazardstatement::HazardStatement,
    linearformula::LinearFormula, name::Name, person::Person, physicalstate::PhysicalState,
    precautionarystatement::PrecautionaryStatement, producerref::ProducerRef,
    producttype::ProductType, signalword::SignalWord, supplierref::SupplierRef, symbol::Symbol,
    tag::Tag, unit::Unit,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Product {
    pub product_id: Option<u64>,
    pub product_type: ProductType,
    pub product_inchi: Option<String>,
    pub product_inchikey: Option<String>,
    pub product_canonical_smiles: Option<String>,
    pub product_specificity: Option<String>,
    pub product_msds: Option<String>,
    #[serde(default)]
    pub product_restricted: bool,
    #[serde(default)]
    pub product_radioactive: bool,
    pub product_twod_formula: Option<String>,
    pub product_threed_formula: Option<String>,
    pub product_disposal_comment: Option<String>,
    pub product_remark: Option<String>,
    pub product_molecular_weight: Option<f64>,
    pub product_temperature: Option<f64>,
    pub product_sheet: Option<String>,
    pub product_number_per_carton: Option<i64>,
    pub product_number_per_bag: Option<i64>,

    #[serde(default)]
    pub person: Person,
    pub name: Name,

    pub empirical_formula: Option<EmpiricalFormula>,
    pub linear_formula: Option<LinearFormula>,
    pub physical_state: Option<PhysicalState>,
    pub signal_word: Option<SignalWord>,
    pub cas_number: Option<CasNumber>,
    pub ce_number: Option<CeNumber>,
    pub producer_ref: Option<ProducerRef>,
    pub category: Option<Category>,
    pub unit_temperature: Option<Unit>,
    pub unit_molecular_weight: Option<Unit>,

    pub classes_of_compound: Option<Vec<ClassOfCompound>>,
    pub synonyms: Option<Vec<Name>>,
    pub symbols: Option<Vec<Symbol>>,
    pub hazard_statements: Option<Vec<HazardStatement>>,
    pub precautionary_statements: Option<Vec<PrecautionaryStatement>>,
    pub supplier_refs: Option<Vec<SupplierRef>>,
    pub tags: Option<Vec<Tag>>,

    // product has bookmark for the logged user
    #[serde(default)]
    pub product_has_bookmark: bool,
    // archived storage count in the logged user entity(ies)
    pub product_asc: Option<u64>,
    // storage count in the logged user entity(ies)
    pub product_sc: Option<u64>,
    // total storage count
    pub product_tsc: Option<u64>,
    // hazard statement CMR concatenation
    pub product_hs_cmr: Option<String>,
    // store location code
    pub product_sl: Option<String>,
    // product availability in the entities
    pub product_availability: Option<Vec<Entity>>,
}

impl Product {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.cas_number.is_some() && self.cas_number.as_mut().unwrap().cas_number_id.is_none() {
            self.cas_number.as_mut().unwrap().sanitize_and_validate()?;
        }

        if self.ce_number.is_some() && self.ce_number.as_mut().unwrap().ce_number_id.is_none() {
            self.ce_number.as_mut().unwrap().sanitize_and_validate()?;
        }

        if self.empirical_formula.is_some()
            && self
                .empirical_formula
                .as_mut()
                .unwrap()
                .empirical_formula_id
                .is_none()
        {
            self.empirical_formula
                .as_mut()
                .unwrap()
                .sanitize_and_validate()?;
        }

        if self.name.name_id.is_none() {
            self.name.sanitize_and_validate()?;
        }

        Ok(())
    }
}
