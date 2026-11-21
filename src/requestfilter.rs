use chimitheque_defines::{END_OF_URL_CAPTURE_RE, IDS_CAPTURE_RE, IDS_MATCH_RE};
use log::debug;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use url::{Url, form_urlencoded};

use crate::{error::RequestFilterError, sortorder::SortOrder};

/// Filtering, sorting and paging options of a list request.
///
/// Built from the query parameters of a request URL with `TryFrom<&str>`,
/// and turned back into query parameters with `Display`. Fields that do not
/// apply to the listed items are ignored.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestFilter {
    /// Text to search in the listed items, e.g. part of a label.
    pub search: Option<String>,
    /// Id of a single item to return.
    pub id: Option<u64>,
    /// Column to sort by.
    pub order_by: Option<String>,
    /// Sort direction; ascending by default.
    #[serde(default)]
    pub order: SortOrder,
    /// Number of items to skip, for paging.
    pub offset: Option<usize>,
    /// Maximum number of items to return, for paging.
    pub limit: Option<usize>,
    /// Only products bookmarked by the current person.
    #[serde(default)]
    pub bookmark: bool,
    /// Only borrowed storages.
    #[serde(default)]
    pub borrowing: bool,
    /// Only items with this CAS number id.
    pub cas_number: Option<u64>,
    /// Only items with this CAS number, e.g. `7732-18-5`.
    pub cas_number_string: Option<String>,
    /// Only CMR products (carcinogenic, mutagenic or reprotoxic).
    #[serde(default)]
    pub is_cmr: bool,
    /// Only products of this category id.
    pub category: Option<u64>,
    /// Only products whose name contains this text.
    pub custom_name_part_of: Option<String>,
    /// Only products with this empirical formula id.
    pub empirical_formula: Option<u64>,
    /// Only items of this entity id.
    pub entity: Option<u64>,
    /// Only the entity with exactly this name.
    pub entity_name: Option<String>,
    /// Only products with these hazard statement ids.
    pub hazard_statements: Option<Vec<u64>>,
    /// Include storage history. Without it, only current storages are
    /// returned; with `id`, the storage and its history are returned.
    #[serde(default)]
    pub history: bool,
    /// When selecting storages: only storages with these ids.
    pub storages: Option<Vec<u64>>,
    /// Only products with this name id, as name or synonym.
    pub name: Option<u64>,
    /// Only items of this person id.
    pub person: Option<u64>,
    /// Only the person with exactly this email.
    pub person_email: Option<String>,
    /// Only products with these precautionary statement ids.
    #[serde(default)]
    pub precautionary_statements: Option<Vec<u64>>,
    /// Only items of this producer id.
    pub producer: Option<u64>,
    /// Only products with this producer reference id.
    pub producer_ref: Option<u64>,
    /// When selecting storages: only storages of this product id. When
    /// selecting products, use `id` instead.
    pub product: Option<u64>,
    /// Only products whose specificity contains this text.
    pub product_specificity: Option<String>,
    /// Include biological reagents.
    #[serde(default)]
    pub show_bio: bool,
    /// Include chemical products.
    #[serde(default)]
    pub show_chem: bool,
    /// Include laboratory consumables.
    #[serde(default)]
    pub show_consu: bool,
    /// Only products with this signal word id.
    pub signal_word: Option<u64>,
    /// When selecting products: only the product of this storage id. When
    /// selecting storages, use `id` instead.
    pub storage: Option<u64>,
    /// Only archived storages if `true`, only non-archived ones if `false`.
    pub storage_archive: Option<bool>,
    /// Only storages whose barcode contains this text.
    pub storage_barecode: Option<String>,
    /// Only storages whose batch number contains this text.
    pub storage_batch_number: Option<String>,
    /// Only storages marked to destroy.
    #[serde(default)]
    pub storage_to_destroy: bool,
    /// Only items in this store location id.
    pub store_location: Option<u64>,
    /// Only store locations that can hold storages.
    #[serde(default)]
    pub store_location_can_store: bool,
    /// Only items of this supplier id.
    pub supplier: Option<u64>,
    /// Only products with these hazard symbol ids.
    pub symbols: Option<Vec<u64>>,
    /// Only products with these tag ids.
    pub tags: Option<Vec<u64>>,
    /// Only units of this type, e.g. `quantity`.
    pub unit_type: Option<String>,
}

impl fmt::Display for RequestFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parameters = form_urlencoded::Serializer::new(String::new());

        if let Some(search) = &self.search {
            parameters.append_pair("search", search);
        }
        if let Some(id) = &self.id {
            parameters.append_pair("id", &id.to_string());
        }
        if let Some(order_by) = &self.order_by {
            parameters.append_pair("order_by", order_by);
        }
        parameters.append_pair("order", self.order.as_str());
        if let Some(offset) = &self.offset {
            parameters.append_pair("offset", &offset.to_string());
        }
        if let Some(limit) = &self.limit {
            parameters.append_pair("limit", &limit.to_string());
        }
        if self.bookmark {
            parameters.append_pair("bookmark", "true");
        }
        if self.borrowing {
            parameters.append_pair("borrowing", "true");
        }
        if let Some(cas_number) = &self.cas_number {
            parameters.append_pair("cas_number", &cas_number.to_string());
        }
        if let Some(cas_number_string) = &self.cas_number_string {
            parameters.append_pair("cas_number_string", cas_number_string);
        }
        if self.is_cmr {
            parameters.append_pair("is_cmr", "true");
        }
        if let Some(category) = &self.category {
            parameters.append_pair("category", &category.to_string());
        }
        if let Some(custom_name_part_of) = &self.custom_name_part_of {
            parameters.append_pair("custom_name_part_of", custom_name_part_of);
        }
        if let Some(empirical_formula) = &self.empirical_formula {
            parameters.append_pair("empirical_formula", &empirical_formula.to_string());
        }
        if let Some(entity) = &self.entity {
            parameters.append_pair("entity", &entity.to_string());
        }
        if let Some(entity_name) = &self.entity_name {
            parameters.append_pair("entity_name", entity_name);
        }
        if let Some(hazard_statements) = &self.hazard_statements {
            parameters.append_pair("hazard_statements", &join_ids(hazard_statements));
        }
        if let Some(precautionary_statements) = &self.precautionary_statements {
            parameters.append_pair(
                "precautionary_statements",
                &join_ids(precautionary_statements),
            );
        }
        if self.history {
            parameters.append_pair("history", "true");
        }
        if let Some(storages) = &self.storages {
            parameters.append_pair("storages", &join_ids(storages));
        }
        if let Some(name) = &self.name {
            parameters.append_pair("name", &name.to_string());
        }
        if let Some(producer) = &self.producer {
            parameters.append_pair("producer", &producer.to_string());
        }
        if let Some(producer_ref) = &self.producer_ref {
            parameters.append_pair("producer_ref", &producer_ref.to_string());
        }
        if let Some(product) = &self.product {
            parameters.append_pair("product", &product.to_string());
        }
        if let Some(person) = &self.person {
            parameters.append_pair("person", &person.to_string());
        }
        if let Some(person_email) = &self.person_email {
            parameters.append_pair("person_email", person_email);
        }
        if let Some(product_specificity) = &self.product_specificity {
            parameters.append_pair("product_specificity", product_specificity);
        }
        if self.show_bio {
            parameters.append_pair("show_bio", "true");
        }
        if self.show_chem {
            parameters.append_pair("show_chem", "true");
        }
        if self.show_consu {
            parameters.append_pair("show_consu", "true");
        }
        if let Some(signal_word) = &self.signal_word {
            parameters.append_pair("signal_word", &signal_word.to_string());
        }
        if let Some(storage) = &self.storage {
            parameters.append_pair("storage", &storage.to_string());
        }
        if let Some(storage_archive) = self.storage_archive {
            parameters.append_pair("storage_archive", &storage_archive.to_string());
        }
        if let Some(storage_barecode) = &self.storage_barecode {
            parameters.append_pair("storage_barecode", storage_barecode);
        }
        if let Some(storage_batch_number) = &self.storage_batch_number {
            parameters.append_pair("storage_batch_number", storage_batch_number);
        }
        if self.storage_to_destroy {
            parameters.append_pair("storage_to_destroy", "true");
        }
        if let Some(store_location) = &self.store_location {
            parameters.append_pair("store_location", &store_location.to_string());
        }
        if self.store_location_can_store {
            parameters.append_pair("store_location_can_store", "true");
        }
        if let Some(supplier) = &self.supplier {
            parameters.append_pair("supplier", &supplier.to_string());
        }
        if let Some(symbols) = &self.symbols {
            parameters.append_pair("symbols", &join_ids(symbols));
        }
        if let Some(tags) = &self.tags {
            parameters.append_pair("tags", &join_ids(tags));
        }
        if let Some(unit_type) = &self.unit_type {
            parameters.append_pair("unit_type", unit_type);
        }

        let result = parameters.finish();
        write!(f, "?{result}")
    }
}

/// Joins ids into a comma-separated string, e.g. `[1, 2]` -> `"1,2"`.
fn join_ids(ids: &[u64]) -> String {
    ids.iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(",")
}

/// Parses a single query parameter value, naming the parameter in the error.
fn parse_param<T>(parameter: &'static str, value: &str) -> Result<T, RequestFilterError>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    value
        .parse::<T>()
        .map_err(|e| RequestFilterError::InvalidParameter {
            parameter,
            message: e.to_string(),
        })
}

fn parse_id_list(value: &str, parameter: &'static str) -> Result<Vec<u64>, RequestFilterError> {
    if !IDS_MATCH_RE.is_match(value) {
        return Err(RequestFilterError::InvalidIdList { parameter });
    }

    IDS_CAPTURE_RE
        .captures_iter(value)
        .map(|capture| {
            let id = capture
                .name("id")
                .ok_or(RequestFilterError::InvalidIdList { parameter })?
                .as_str();
            id.parse::<u64>()
                .map_err(|error| RequestFilterError::InvalidParameter {
                    parameter,
                    message: format!("invalid id {id}: {error}"),
                })
        })
        .collect()
}

impl TryFrom<&str> for RequestFilter {
    type Error = RequestFilterError;

    fn try_from(request: &str) -> Result<Self, Self::Error> {
        debug!("request:{request}");

        // Result populated after by the request parameters.
        let mut request_filter: RequestFilter = RequestFilter::default();

        // Parse request URL.
        let url = match Url::parse(request) {
            Ok(url) => url,
            Err(e) => return Err(RequestFilterError::InvalidUrl(e)),
        };

        // Trying to capture an id at the end of the URL.
        // Can be overwritten by the ?id= URL parameter (see below).
        if END_OF_URL_CAPTURE_RE.is_match(url.as_str())
            && let Some(cap) = END_OF_URL_CAPTURE_RE.captures(url.as_str())
        {
            // Safe access - capture exists due to regex match, but handle parse error
            if let Some(id_match) = cap.name("id")
                && let Ok(id_parsed) = id_match.as_str().parse::<u64>()
            {
                request_filter.id = Some(id_parsed);
            }
        }

        // Get the query parameters.
        for query_pair in url.query_pairs() {
            debug!("query_pair:{query_pair:?}");

            let (key, value) = query_pair;
            // Percent-decoded names may be Cow::Owned; match their contents so they
            // behave exactly like unescaped Cow::Borrowed names without allocating.
            match key.as_ref() {
                "search" => {
                    request_filter.search = Some(value.to_string());
                }
                "id" => request_filter.id = Some(parse_param("id", &value)?),
                "order_by" => {
                    request_filter.order_by = Some(value.to_string());
                }
                "order" => request_filter.order = parse_param("order", &value)?,
                "offset" => request_filter.offset = Some(parse_param("offset", &value)?),
                "limit" => request_filter.limit = Some(parse_param("limit", &value)?),
                "bookmark" => request_filter.bookmark = parse_param("bookmark", &value)?,
                "borrowing" => request_filter.borrowing = parse_param("borrowing", &value)?,
                "cas_number" => {
                    request_filter.cas_number = Some(parse_param("cas_number", &value)?);
                }
                "cas_number_string" => {
                    request_filter.cas_number_string = Some(value.to_string());
                }
                "is_cmr" => request_filter.is_cmr = parse_param("is_cmr", &value)?,
                "category" => request_filter.category = Some(parse_param("category", &value)?),
                "custom_name_part_of" => {
                    request_filter.custom_name_part_of = Some(value.to_string());
                }
                "empirical_formula" => {
                    request_filter.empirical_formula =
                        Some(parse_param("empirical_formula", &value)?);
                }
                "entity" => request_filter.entity = Some(parse_param("entity", &value)?),
                "entity_name" => {
                    request_filter.entity_name = Some(value.to_string());
                }
                "hazard_statements" => {
                    request_filter.hazard_statements =
                        Some(parse_id_list(&value, "hazard_statements")?);
                }
                "history" => request_filter.history = parse_param("history", &value)?,
                "storages" => {
                    request_filter.storages = Some(parse_id_list(&value, "storages")?);
                }
                "name" => request_filter.name = Some(parse_param("name", &value)?),
                "precautionary_statements" => {
                    request_filter.precautionary_statements =
                        Some(parse_id_list(&value, "precautionary_statements")?);
                }
                "producer" => request_filter.producer = Some(parse_param("producer", &value)?),
                "producer_ref" => {
                    request_filter.producer_ref = Some(parse_param("producer_ref", &value)?);
                }
                "product" => request_filter.product = Some(parse_param("product", &value)?),
                "person" => request_filter.person = Some(parse_param("person", &value)?),
                "person_email" => {
                    request_filter.person_email = Some(value.to_string());
                }
                "product_specificity" => {
                    request_filter.product_specificity = Some(value.to_string());
                }
                "show_bio" => request_filter.show_bio = parse_param("show_bio", &value)?,
                "show_chem" => request_filter.show_chem = parse_param("show_chem", &value)?,
                "show_consu" => request_filter.show_consu = parse_param("show_consu", &value)?,
                "signal_word" => {
                    request_filter.signal_word = Some(parse_param("signal_word", &value)?);
                }
                "storage" => request_filter.storage = Some(parse_param("storage", &value)?),
                "storage_archive" => {
                    request_filter.storage_archive = Some(parse_param("storage_archive", &value)?);
                }
                "storage_barecode" => {
                    request_filter.storage_barecode = Some(value.to_string());
                }
                "storage_batch_number" => {
                    request_filter.storage_batch_number = Some(value.to_string());
                }
                "storage_to_destroy" => {
                    request_filter.storage_to_destroy = parse_param("storage_to_destroy", &value)?;
                }
                "store_location" => {
                    request_filter.store_location = Some(parse_param("store_location", &value)?);
                }
                "store_location_can_store" => {
                    request_filter.store_location_can_store =
                        parse_param("store_location_can_store", &value)?;
                }
                "supplier" => request_filter.supplier = Some(parse_param("supplier", &value)?),
                "symbols" => {
                    request_filter.symbols = Some(parse_id_list(&value, "symbols")?);
                }
                "tags" => {
                    request_filter.tags = Some(parse_id_list(&value, "tags")?);
                }
                "unit_type" => {
                    request_filter.unit_type = Some(value.to_string());
                }
                _ => (),
            }
        }

        Ok(request_filter)
    }
}

impl Default for RequestFilter {
    fn default() -> RequestFilter {
        RequestFilter {
            search: None,
            id: None,
            order_by: None,
            order: SortOrder::Asc,
            offset: None,
            limit: None,
            bookmark: false,
            borrowing: false,
            cas_number: None,
            cas_number_string: None,
            is_cmr: false,
            category: None,
            custom_name_part_of: None,
            empirical_formula: None,
            entity: None,
            entity_name: None,
            hazard_statements: None,
            history: false,
            storages: None,
            name: None,
            precautionary_statements: None,
            producer: None,
            producer_ref: None,
            person: None,
            person_email: None,
            product: None,
            product_specificity: None,
            show_bio: false,
            show_chem: false,
            show_consu: false,
            signal_word: None,
            storage: None,
            storage_archive: None,
            storage_barecode: None,
            storage_batch_number: None,
            storage_to_destroy: false,
            store_location: None,
            store_location_can_store: false,
            supplier: None,
            symbols: None,
            tags: None,
            unit_type: None,
        }
    }
}

#[cfg(test)]
#[path = "requestfilter_tests.rs"]
mod requestfilter_tests;
