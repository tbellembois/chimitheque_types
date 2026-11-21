//! Types of Chimithèque, a management application for chemical products,
//! biological reagents and laboratory consumables.
//!
//! Each module holds one type, usually serializable with serde. Types saved
//! by the application have a `sanitize_and_validate` method that cleans their
//! fields and returns a [`error::ValidationError`] naming the invalid field.
#![warn(missing_docs)]

/// Products bookmarked by a person.
pub mod bookmark;
/// Storages lent to a person.
pub mod borrowing;
/// CAS registry numbers.
pub mod casnumber;
/// Product categories.
pub mod category;
/// European Community (EC) numbers.
pub mod cenumber;
/// Classes of chemical compounds.
pub mod classofcompound;
/// Empirical formulas.
pub mod empiricalformula;
pub mod entity;
/// Entity managers.
pub mod entitypeople;
pub mod error;
/// GHS hazard statements.
pub mod hazardstatement;
/// Linear formulas.
pub mod linearformula;
/// Product names and synonyms.
pub mod name;
/// Permissions of people on items.
pub mod permission;
/// People (application users).
pub mod person;
/// Entities people belong to.
pub mod personentities;
/// Physical states.
pub mod physicalstate;
/// GHS precautionary statements.
pub mod precautionarystatement;
/// Product producers (manufacturers).
pub mod producer;
/// Producer catalogue references.
pub mod producerref;
pub mod product;
/// Classes of compounds linked to products.
pub mod productclassesofcompounds;
/// Hazard statements linked to products.
pub mod producthazardstatements;
/// Precautionary statements linked to products.
pub mod productprecautionarystatements;
/// Supplier references linked to products.
pub mod productsupplierrefs;
/// Hazard symbols linked to products.
pub mod productsymbols;
/// Synonyms linked to products.
pub mod productsynonyms;
/// Tags linked to products.
pub mod producttags;
/// Kinds of product: chemical, biological, consumable.
pub mod producttype;
/// `PubChem` autocomplete responses.
pub mod pubchem;
/// Products extracted from `PubChem` documents.
pub mod pubchemproduct;
/// Filtering, sorting and paging options of list requests.
pub mod requestfilter;
mod searchable;
/// GHS signal words.
pub mod signalword;
/// Sort direction of list requests.
pub mod sortorder;
/// Product stocks per store location.
pub mod stock;
pub mod storage;
pub mod storelocation;
/// Product suppliers (vendors).
pub mod supplier;
/// Supplier catalogue references.
pub mod supplierref;
/// GHS hazard pictograms.
pub mod symbol;
/// Free product tags.
pub mod tag;
/// Measurement units.
pub mod unit;
/// What a unit measures.
pub mod unittype;
mod validation;
/// Application build information.
pub mod versioninfo;
