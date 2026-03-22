#![warn(clippy::all, clippy::pedantic)]
// #![warn(
//     clippy::unwrap_used,
//     clippy::expect_used,
//     clippy::panic,
//     clippy::unreachable
// )]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::struct_excessive_bools,
    clippy::too_many_lines
)]

pub mod bookmark;
pub mod borrowing;
pub mod casnumber;
pub mod category;
pub mod cenumber;
pub mod classofcompound;
pub mod empiricalformula;
pub mod entity;
pub mod entitypeople;
pub mod error;
pub mod hazardstatement;
pub mod linearformula;
pub mod name;
pub mod permission;
pub mod person;
pub mod personentities;
pub mod physicalstate;
pub mod precautionarystatement;
pub mod producer;
pub mod producerref;
pub mod product;
pub mod productclassesofcompounds;
pub mod producthazardstatements;
pub mod productprecautionarystatements;
pub mod productsupplierrefs;
pub mod productsymbols;
pub mod productsynonyms;
pub mod producttags;
pub mod producttype;
pub mod pubchemproduct;
pub mod requestfilter;
pub mod signalword;
pub mod stock;
pub mod storage;
pub mod storelocation;
pub mod supplier;
pub mod supplierref;
pub mod symbol;
pub mod tag;
pub mod unit;
pub mod unittype;
pub mod userinfo;
