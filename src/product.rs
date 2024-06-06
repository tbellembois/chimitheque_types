use crate::{
    casnumber::Casnumber, category::Category, cenumber::Cenumber, classofcompound::Classofcompound,
    empiricalformula::Empiricalformula, hazardstatement::Hazardstatement,
    linearformula::Linearformula, name::Name, person::Person, physicalstate::Physicalstate,
    precautionarystatement::Precautionarystatement, producerref::Producerref,
    producttype::ProductType, signalword::Signalword, supplierref::Supplierref, symbol::Symbol,
    tag::Tag, unit::Unit,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub product_id: u64,
    pub product_type: ProductType,
    pub product_inchi: Option<String>,
    pub product_inchikey: Option<String>,
    pub product_canonical_smiles: Option<String>,
    pub product_specificity: Option<String>,
    pub product_msds: Option<String>,
    pub product_restricted: bool,
    pub product_radioactive: bool,
    pub product_twod_formula: Option<String>,
    pub product_threed_formula: Option<String>,
    pub product_disposal_comment: Option<String>,
    pub product_remark: Option<String>,
    pub product_molecula_weight: Option<f64>,
    pub product_temperature: Option<f64>,
    pub product_sheet: Option<String>,
    pub product_number_per_carton: Option<i64>,
    pub product_number_per_bag: Option<i64>,

    pub person: Person,
    pub name: Name,

    pub empirical_formula: Option<Empiricalformula>,
    pub linear_formula: Option<Linearformula>,
    pub physical_state: Option<Physicalstate>,
    pub signal_word: Option<Signalword>,
    pub cas_number: Option<Casnumber>,
    pub ce_number: Option<Cenumber>,
    pub producer_ref: Option<Producerref>,
    pub category: Option<Category>,
    pub unit_temperature: Option<Unit>,
    pub unit_molecular_weight: Option<Unit>,

    pub class_of_compound: Option<Vec<Classofcompound>>,
    pub synonyms: Option<Vec<Name>>,
    pub symbols: Option<Vec<Symbol>>,
    pub hazard_statements: Option<Vec<Hazardstatement>>,
    pub precautionary_statements: Option<Vec<Precautionarystatement>>,
    pub supplier_refs: Option<Vec<Supplierref>>,
    pub tags: Option<Vec<Tag>>,
}
