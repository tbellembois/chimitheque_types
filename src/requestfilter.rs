use chimitheque_defines::{END_OF_URL_CAPTURE_RE, IDS_CAPTURE_RE, IDS_MATCH_RE};
use log::debug;
use serde::{Deserialize, Serialize};
use std::fmt;
use url::{Url, form_urlencoded};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestFilter {
    pub search: Option<String>,
    pub id: Option<u64>,
    pub order_by: Option<String>,
    #[serde(default)]
    pub order: String,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    #[serde(default)]
    pub bookmark: bool,
    #[serde(default)]
    pub borrowing: bool,
    pub cas_number: Option<u64>,
    pub cas_number_string: Option<String>,
    #[serde(default)]
    pub is_cmr: bool,
    pub category: Option<u64>,
    pub custom_name_part_of: Option<String>,
    pub empirical_formula: Option<u64>,
    pub entity: Option<u64>,
    pub entity_name: Option<String>,
    pub hazard_statements: Option<Vec<u64>>,
    #[serde(default)]
    pub history: bool,
    pub storages: Option<Vec<u64>>,
    pub name: Option<u64>,
    pub person: Option<u64>,
    pub person_email: Option<String>,
    #[serde(default)]
    pub precautionary_statements: Option<Vec<u64>>,
    pub producer: Option<u64>,
    pub producer_ref: Option<u64>,
    pub product: Option<u64>, // a product id while selecting storages, not a product id while selecting products - use id instead
    pub product_specificity: Option<String>,
    #[serde(default)]
    pub show_bio: bool,
    #[serde(default)]
    pub show_chem: bool,
    #[serde(default)]
    pub show_consu: bool,
    pub signal_word: Option<u64>,
    pub storage: Option<u64>, // a storage id while selecting products (ie the product that has for storage this "storage" id), not a storage id while selecting storages - use id instead
    pub storage_archive: Option<bool>,
    pub storage_barecode: Option<String>,
    pub storage_batch_number: Option<String>,
    #[serde(default)]
    pub storage_to_destroy: bool,
    pub store_location: Option<u64>,
    #[serde(default)]
    pub store_location_can_store: bool,
    pub supplier: Option<u64>,
    pub symbols: Option<Vec<u64>>,
    pub tags: Option<Vec<u64>>,
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
        parameters.append_pair("order", &self.order);
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
            parameters.append_pair(
                "hazard_statements",
                &hazard_statements
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(precautionary_statements) = &self.precautionary_statements {
            parameters.append_pair(
                "precautionary_statements",
                &precautionary_statements
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if self.history {
            parameters.append_pair("history", "true");
        }
        if let Some(storages) = &self.storages {
            parameters.append_pair(
                "storages",
                &storages
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            );
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
            parameters.append_pair(
                "symbols",
                &symbols
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(tags) = &self.tags {
            parameters.append_pair(
                "tags",
                &tags
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(unit_type) = &self.unit_type {
            parameters.append_pair("unit_type", unit_type);
        }

        let result = parameters.finish();
        write!(f, "?{result}")
    }
}

impl TryFrom<&str> for RequestFilter {
    type Error = String;

    fn try_from(request: &str) -> Result<Self, Self::Error> {
        debug!("request:{request}");

        // Result populated after by the request parameters.
        let mut request_filter: RequestFilter = RequestFilter::default();

        // Parse request URL.
        let url = match Url::parse(request) {
            Ok(url) => url,
            Err(e) => return Err(format!("can not parse url: {e}")),
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
            match key {
                std::borrow::Cow::Borrowed("search") => {
                    request_filter.search = Some(value.to_string());
                }
                std::borrow::Cow::Borrowed("id") => match value.parse::<u64>() {
                    Ok(v) => request_filter.id = Some(v),
                    Err(e) => return Err(format!("error with id query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("order_by") => {
                    request_filter.order_by = Some(value.to_string());
                }
                std::borrow::Cow::Borrowed("order") => request_filter.order = value.to_string(),
                std::borrow::Cow::Borrowed("offset") => match value.parse::<usize>() {
                    Ok(v) => request_filter.offset = Some(v),
                    Err(e) => return Err(format!("error with offset query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("limit") => match value.parse::<usize>() {
                    Ok(v) => request_filter.limit = Some(v),
                    Err(e) => return Err(format!("error with limit query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("bookmark") => match value.parse::<bool>() {
                    Ok(v) => request_filter.bookmark = v,
                    Err(e) => return Err(format!("error with bookmark query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("borrowing") => match value.parse::<bool>() {
                    Ok(v) => request_filter.borrowing = v,
                    Err(e) => return Err(format!("error with borrowing query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("cas_number") => match value.parse::<u64>() {
                    Ok(v) => request_filter.cas_number = Some(v),
                    Err(e) => return Err(format!("error with cas_number query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("cas_number_string") => {
                    request_filter.cas_number_string = Some(value.to_string());
                }
                std::borrow::Cow::Borrowed("is_cmr") => match value.parse::<bool>() {
                    Ok(v) => request_filter.is_cmr = v,
                    Err(e) => return Err(format!("error with is_cmr query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("category") => match value.parse::<u64>() {
                    Ok(v) => request_filter.category = Some(v),
                    Err(e) => return Err(format!("error with category query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("custom_name_part_of") => {
                    request_filter.custom_name_part_of = Some(value.to_string());
                }
                std::borrow::Cow::Borrowed("empirical_formula") => match value.parse::<u64>() {
                    Ok(v) => request_filter.empirical_formula = Some(v),
                    Err(e) => {
                        return Err(format!("error with empirical_formula query parameter: {e}"));
                    }
                },
                std::borrow::Cow::Borrowed("entity") => match value.parse::<u64>() {
                    Ok(v) => request_filter.entity = Some(v),
                    Err(e) => return Err(format!("error with entity query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("entity_name") => {
                    request_filter.entity_name = Some(value.to_string());
                }
                std::borrow::Cow::Borrowed("hazard_statements") => {
                    if !IDS_MATCH_RE.is_match(&value) {
                        return Err("invalid hazard_statements ids format".to_string());
                    }

                    let caps = IDS_CAPTURE_RE.captures_iter(&value);
                    let mut hazard_statement_ids: Vec<u64> = Vec::new();
                    for cap in caps {
                        // Safe access - capture should exist due to is_match, but handle gracefully
                        if let Some(id_match) = cap.name("id")
                            && let Ok(id_parsed) = id_match.as_str().parse::<u64>()
                        {
                            hazard_statement_ids.push(id_parsed);
                        }
                    }
                    request_filter.hazard_statements = Some(hazard_statement_ids);
                }
                std::borrow::Cow::Borrowed("history") => match value.parse::<bool>() {
                    Ok(v) => request_filter.history = v,
                    Err(e) => return Err(format!("error with history query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("storages") => {
                    if !IDS_MATCH_RE.is_match(&value) {
                        return Err("invalid storages ids format".to_string());
                    }

                    let caps = IDS_CAPTURE_RE.captures_iter(&value);
                    let mut storage_ids: Vec<u64> = Vec::new();
                    for cap in caps {
                        // Safe access - capture should exist due to is_match, but handle gracefully
                        if let Some(id_match) = cap.name("id")
                            && let Ok(id_parsed) = id_match.as_str().parse::<u64>()
                        {
                            storage_ids.push(id_parsed);
                        }
                    }
                    request_filter.storages = Some(storage_ids);
                }
                std::borrow::Cow::Borrowed("name") => match value.parse::<u64>() {
                    Ok(v) => request_filter.name = Some(v),
                    Err(e) => return Err(format!("error with name query parameter: {e}")),
                },
                // std::borrow::Cow::Borrowed("permission") => {
                //     request_filter.permission = value.to_string();
                // }
                std::borrow::Cow::Borrowed("precautionary_statements") => {
                    if !IDS_MATCH_RE.is_match(&value) {
                        return Err("invalid precautionary_statements ids format".to_string());
                    }

                    let caps = IDS_CAPTURE_RE.captures_iter(&value);
                    let mut precautionary_statement_ids: Vec<u64> = Vec::new();
                    for cap in caps {
                        // Safe access - capture should exist due to is_match, but handle gracefully
                        if let Some(id_match) = cap.name("id")
                            && let Ok(id_parsed) = id_match.as_str().parse::<u64>()
                        {
                            precautionary_statement_ids.push(id_parsed);
                        }
                    }
                    request_filter.precautionary_statements = Some(precautionary_statement_ids);
                }
                std::borrow::Cow::Borrowed("producer") => match value.parse::<u64>() {
                    Ok(v) => request_filter.producer = Some(v),
                    Err(e) => return Err(format!("error with producer query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("producer_ref") => match value.parse::<u64>() {
                    Ok(v) => request_filter.producer_ref = Some(v),
                    Err(e) => return Err(format!("error with producer_ref query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("product") => match value.parse::<u64>() {
                    Ok(v) => request_filter.product = Some(v),
                    Err(e) => return Err(format!("error with product query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("person") => match value.parse::<u64>() {
                    Ok(v) => request_filter.person = Some(v),
                    Err(e) => return Err(format!("error with person query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("person_email") => {
                    request_filter.person_email = Some(value.to_string());
                }
                std::borrow::Cow::Borrowed("product_specificity") => {
                    request_filter.product_specificity = Some(value.to_string());
                }
                std::borrow::Cow::Borrowed("show_bio") => match value.parse::<bool>() {
                    Ok(v) => request_filter.show_bio = v,
                    Err(e) => return Err(format!("error with show_bio query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("show_chem") => match value.parse::<bool>() {
                    Ok(v) => request_filter.show_chem = v,
                    Err(e) => return Err(format!("error with show_chem query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("show_consu") => match value.parse::<bool>() {
                    Ok(v) => request_filter.show_consu = v,
                    Err(e) => return Err(format!("error with show_consu query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("signal_word") => match value.parse::<u64>() {
                    Ok(v) => request_filter.signal_word = Some(v),
                    Err(e) => return Err(format!("error with signal_word query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("storage") => match value.parse::<u64>() {
                    Ok(v) => request_filter.storage = Some(v),
                    Err(e) => return Err(format!("error with storage query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("storage_archive") => match value.parse::<bool>() {
                    Ok(v) => request_filter.storage_archive = Some(v),
                    Err(e) => {
                        return Err(format!("error with storage_archive query parameter: {e}"));
                    }
                },
                std::borrow::Cow::Borrowed("storage_barecode") => {
                    request_filter.storage_barecode = Some(value.to_string());
                }
                std::borrow::Cow::Borrowed("storage_batch_number") => {
                    request_filter.storage_batch_number = Some(value.to_string());
                }
                std::borrow::Cow::Borrowed("storage_to_destroy") => match value.parse::<bool>() {
                    Ok(v) => request_filter.storage_to_destroy = v,
                    Err(e) => {
                        return Err(format!(
                            "error with storage_to_destroy query parameter: {e}"
                        ));
                    }
                },
                std::borrow::Cow::Borrowed("store_location") => match value.parse::<u64>() {
                    Ok(v) => request_filter.store_location = Some(v),
                    Err(e) => {
                        return Err(format!("error with store_location query parameter: {e}"));
                    }
                },
                std::borrow::Cow::Borrowed("store_location_can_store") => {
                    match value.parse::<bool>() {
                        Ok(v) => request_filter.store_location_can_store = v,
                        Err(e) => {
                            return Err(format!(
                                "error with store_location_can_store query parameter: {e}"
                            ));
                        }
                    }
                }
                std::borrow::Cow::Borrowed("supplier") => match value.parse::<u64>() {
                    Ok(v) => request_filter.supplier = Some(v),
                    Err(e) => return Err(format!("error with supplier query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("symbols") => {
                    if !IDS_MATCH_RE.is_match(&value) {
                        return Err("invalid symbols ids format".to_string());
                    }

                    let caps = IDS_CAPTURE_RE.captures_iter(&value);
                    let mut symbol_ids: Vec<u64> = Vec::new();
                    for cap in caps {
                        // Safe access - capture should exist due to is_match, but handle gracefully
                        if let Some(id_match) = cap.name("id")
                            && let Ok(id_parsed) = id_match.as_str().parse::<u64>()
                        {
                            symbol_ids.push(id_parsed);
                        }
                    }
                    request_filter.symbols = Some(symbol_ids);
                }
                std::borrow::Cow::Borrowed("tags") => {
                    if !IDS_MATCH_RE.is_match(&value) {
                        return Err("invalid tags ids format".to_string());
                    }

                    let caps = IDS_CAPTURE_RE.captures_iter(&value);
                    let mut tag_ids: Vec<u64> = Vec::new();
                    for cap in caps {
                        // Safe access - capture should exist due to is_match, but handle gracefully
                        if let Some(id_match) = cap.name("id")
                            && let Ok(id_parsed) = id_match.as_str().parse::<u64>()
                        {
                            tag_ids.push(id_parsed);
                        }
                    }
                    request_filter.tags = Some(tag_ids);
                }
                std::borrow::Cow::Borrowed("unit_type") => {
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
            order: String::from("asc"),
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
            // permission: String::from("r"),
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
