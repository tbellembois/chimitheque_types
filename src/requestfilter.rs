use axum::extract::FromRequestParts;
use http::{request::Parts, StatusCode};
use log::debug;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestFilter {
    pub search: Option<String>,
    pub id: Option<u64>,
    pub order_by: Option<String>,
    #[serde(default)]
    pub order: String,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
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
    pub permission: String,
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
        let mut parameters: Vec<String> = vec![];

        if let Some(search) = &self.search {
            parameters.push(format!("search={search}"))
        }
        if let Some(id) = &self.id {
            parameters.push(format!("id={id}"))
        }
        if let Some(order_by) = &self.order_by {
            parameters.push(format!("order_by={order_by}"))
        }
        parameters.push(format!("order={}", self.order));
        if let Some(offset) = &self.offset {
            parameters.push(format!("offset={offset}"))
        }
        if let Some(limit) = &self.limit {
            parameters.push(format!("limit={limit}"))
        }
        if self.bookmark {
            parameters.push("bookmark=true".to_string());
        }
        if self.borrowing {
            parameters.push("borrowing=true".to_string());
        }
        if let Some(cas_number) = &self.cas_number {
            parameters.push(format!("cas_number={cas_number}"))
        }
        if let Some(cas_number_string) = &self.cas_number_string {
            parameters.push(format!("cas_number_string={cas_number_string}"))
        }
        if self.is_cmr {
            parameters.push("is_cmr=true".to_string());
        }
        if let Some(category) = &self.category {
            parameters.push(format!("category={category}"))
        }
        if let Some(custom_name_part_of) = &self.custom_name_part_of {
            parameters.push(format!("custom_name_part_of={custom_name_part_of}"))
        }
        if let Some(empirical_formula) = &self.empirical_formula {
            parameters.push(format!("empirical_formula={empirical_formula}"))
        }
        if let Some(entity) = &self.entity {
            parameters.push(format!("entity={entity}"))
        }
        if let Some(entity_name) = &self.entity_name {
            parameters.push(format!("entity_name={entity_name}"))
        }
        if let Some(hazard_statements) = &self.hazard_statements {
            parameters.push(format!(
                "hazard_statements={}",
                hazard_statements
                    .iter()
                    .map(|h| h.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ));
        }
        if self.history {
            parameters.push("history=true".to_string());
        }
        if let Some(storages) = &self.storages {
            parameters.push(format!(
                "storages={}",
                storages
                    .iter()
                    .map(|h| h.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ));
        }
        if let Some(name) = &self.name {
            parameters.push(format!("name={name}"))
        }
        parameters.push(format!("permission={}", self.permission));
        if let Some(precautionary_statements) = &self.precautionary_statements {
            parameters.push(format!(
                "precautionary_statements={}",
                precautionary_statements
                    .iter()
                    .map(|h| h.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ));
        }
        if let Some(producer) = &self.producer {
            parameters.push(format!("producer={producer}"))
        }
        if let Some(producer_ref) = &self.producer_ref {
            parameters.push(format!("producer_ref={producer_ref}"))
        }
        if let Some(product) = &self.product {
            parameters.push(format!("product={product}"))
        }
        if let Some(person) = &self.person {
            parameters.push(format!("person={person}"))
        }
        if let Some(person_email) = &self.person_email {
            parameters.push(format!("person_email={person_email}"))
        }
        if let Some(product_specificity) = &self.product_specificity {
            parameters.push(format!("product_specificity={product_specificity}"))
        }
        if self.show_bio {
            parameters.push("show_bio=true".to_string());
        }
        if self.show_chem {
            parameters.push("show_chem=true".to_string());
        }
        if self.show_consu {
            parameters.push("show_consu=true".to_string());
        }
        if let Some(signal_word) = &self.signal_word {
            parameters.push(format!("signal_word={signal_word}"))
        }
        if let Some(storage) = &self.storage {
            parameters.push(format!("storage={storage}"))
        }
        if let Some(storage_archive) = self.storage_archive {
            parameters.push(format!("storage_archive={storage_archive}"));
        }
        if let Some(storage_barecode) = &self.storage_barecode {
            parameters.push(format!("storage_barecode={storage_barecode}"))
        }
        if let Some(storage_batch_number) = &self.storage_batch_number {
            parameters.push(format!("storage_batch_number={storage_batch_number}"))
        }
        if self.storage_to_destroy {
            parameters.push("storage_to_destroy=true".to_string());
        }
        if let Some(store_location) = &self.store_location {
            parameters.push(format!("store_location={store_location}"))
        }
        if self.store_location_can_store {
            parameters.push("store_location_can_store=true".to_string());
        }
        if let Some(supplier) = &self.supplier {
            parameters.push(format!("supplier={supplier}"))
        }
        if let Some(symbols) = &self.symbols {
            parameters.push(format!(
                "symbols={}",
                symbols
                    .iter()
                    .map(|h| h.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ));
        }
        if let Some(tags) = &self.tags {
            parameters.push(format!(
                "tags={}",
                tags.iter()
                    .map(|h| h.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ));
        }
        if let Some(unit_type) = &self.unit_type {
            parameters.push(format!("unit_type={unit_type}"))
        }

        let result: String = parameters.join("&");
        write!(f, "?{result}")
    }
}

impl TryFrom<&str> for RequestFilter {
    type Error = String;

    fn try_from(request: &str) -> Result<Self, Self::Error> {
        debug!("request:{request}");

        // Result populated after by the request parameters.
        let mut request_filter: RequestFilter = Default::default();

        // Parse request URL.
        let url = match Url::parse(request) {
            Ok(url) => url,
            Err(e) => return Err(format!("can not parse url: {}", e)),
        };

        // Regex to validate multi valued ids.
        let ids_match = match Regex::new(r"^((\d+),{0,1})+$") {
            Ok(ids_re) => ids_re,
            Err(e) => return Err(format!("error creating ids_match regex: {e}")),
        };

        // Regex to capture multi valued ids.
        let ids_capture = match Regex::new(r"(?<id>\d+),{0,1}") {
            Ok(ids_re) => ids_re,
            Err(e) => return Err(format!("error creating ids_capture regex: {e}")),
        };

        // Regex to capture a group of digits at the end of the URL.
        let end_of_url_id_capture = match Regex::new(r"/(?<id>\d+)$") {
            Ok(id_re) => id_re,
            Err(e) => return Err(format!("error creating end_of_url_id_capture regex: {e}")),
        };

        // Trying to capture an id at the end of the URL.
        // Can be overwritten by the ?id= URL parameter (see below).
        if end_of_url_id_capture.is_match(url.as_str()) {
            if let Some(cap) = end_of_url_id_capture.captures(url.as_str()) {
                // We can unwrap safely here because of validation (is_match) below.
                let id_str = cap.name("id").unwrap().as_str();
                request_filter.id = Some(id_str.parse::<u64>().unwrap());
            }
        }

        // Get the query parameters.
        for query_pair in url.query_pairs() {
            debug!("query_pair:{:?}", query_pair);

            let (key, value) = query_pair;
            match key {
                std::borrow::Cow::Borrowed("search") => {
                    request_filter.search = Some(value.to_string())
                }
                std::borrow::Cow::Borrowed("id") => match value.parse::<u64>() {
                    Ok(v) => request_filter.id = Some(v),
                    Err(e) => return Err(format!("error with id query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("order_by") => {
                    request_filter.order_by = Some(value.to_string())
                }
                std::borrow::Cow::Borrowed("order") => request_filter.order = value.to_string(),
                std::borrow::Cow::Borrowed("offset") => match value.parse::<u64>() {
                    Ok(v) => request_filter.offset = Some(v),
                    Err(e) => return Err(format!("error with offset query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("limit") => match value.parse::<u64>() {
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
                    request_filter.cas_number_string = Some(value.to_string())
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
                    request_filter.custom_name_part_of = Some(value.to_string())
                }
                std::borrow::Cow::Borrowed("empirical_formula") => match value.parse::<u64>() {
                    Ok(v) => request_filter.empirical_formula = Some(v),
                    Err(e) => {
                        return Err(format!("error with empirical_formula query parameter: {e}"))
                    }
                },
                std::borrow::Cow::Borrowed("entity") => match value.parse::<u64>() {
                    Ok(v) => request_filter.entity = Some(v),
                    Err(e) => return Err(format!("error with entity query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("entity_name") => {
                    request_filter.entity_name = Some(value.to_string())
                }
                std::borrow::Cow::Borrowed("hazard_statements") => {
                    if !ids_match.is_match(&value) {
                        return Err("invalid hazard_statements ids format".to_string());
                    }

                    let caps = ids_capture.captures_iter(&value);
                    let mut hazard_statement_ids: Vec<u64> = Vec::new();
                    for cap in caps {
                        // We can unwrap safely here because of validation (is_match) below.
                        let id_str = cap.name("id").unwrap().as_str();
                        let id = id_str.parse::<u64>().unwrap();

                        hazard_statement_ids.push(id);
                    }
                    request_filter.hazard_statements = Some(hazard_statement_ids);
                }
                std::borrow::Cow::Borrowed("history") => match value.parse::<bool>() {
                    Ok(v) => request_filter.history = v,
                    Err(e) => return Err(format!("error with history query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("storages") => {
                    if !ids_match.is_match(&value) {
                        return Err("invalid storages ids format".to_string());
                    }

                    let caps = ids_capture.captures_iter(&value);
                    let mut storage_ids: Vec<u64> = Vec::new();
                    for cap in caps {
                        // We can unwrap safely here because of validation (is_match) below.
                        let id_str = cap.name("id").unwrap().as_str();
                        let id = id_str.parse::<u64>().unwrap();

                        storage_ids.push(id);
                    }
                    request_filter.storages = Some(storage_ids);
                }
                std::borrow::Cow::Borrowed("name") => match value.parse::<u64>() {
                    Ok(v) => request_filter.name = Some(v),
                    Err(e) => return Err(format!("error with name query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("permission") => {
                    request_filter.permission = value.to_string()
                }
                std::borrow::Cow::Borrowed("precautionary_statements") => {
                    if !ids_match.is_match(&value) {
                        return Err("invalid precautionary_statements ids format".to_string());
                    }

                    let caps = ids_capture.captures_iter(&value);
                    let mut precautionary_statement_ids: Vec<u64> = Vec::new();
                    for cap in caps {
                        // We can unwrap safely here because of validation (is_match) below.
                        let id_str = cap.name("id").unwrap().as_str();
                        let id = id_str.parse::<u64>().unwrap();

                        precautionary_statement_ids.push(id);
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
                    request_filter.person_email = Some(value.to_string())
                }
                std::borrow::Cow::Borrowed("product_specificity") => {
                    request_filter.product_specificity = Some(value.to_string())
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
                        return Err(format!("error with storage_archive query parameter: {e}"))
                    }
                },
                std::borrow::Cow::Borrowed("storage_barecode") => {
                    request_filter.storage_barecode = Some(value.to_string())
                }
                std::borrow::Cow::Borrowed("storage_batch_number") => {
                    request_filter.storage_batch_number = Some(value.to_string())
                }
                std::borrow::Cow::Borrowed("storage_to_destroy") => match value.parse::<bool>() {
                    Ok(v) => request_filter.storage_to_destroy = v,
                    Err(e) => {
                        return Err(format!(
                            "error with storage_to_destroy query parameter: {e}"
                        ))
                    }
                },
                std::borrow::Cow::Borrowed("store_location") => match value.parse::<u64>() {
                    Ok(v) => request_filter.store_location = Some(v),
                    Err(e) => {
                        return Err(format!("error with store_location query parameter: {e}"))
                    }
                },
                std::borrow::Cow::Borrowed("store_location_can_store") => {
                    match value.parse::<bool>() {
                        Ok(v) => request_filter.store_location_can_store = v,
                        Err(e) => {
                            return Err(format!(
                                "error with store_location_can_store query parameter: {e}"
                            ))
                        }
                    }
                }
                std::borrow::Cow::Borrowed("supplier") => match value.parse::<u64>() {
                    Ok(v) => request_filter.supplier = Some(v),
                    Err(e) => return Err(format!("error with supplier query parameter: {e}")),
                },
                std::borrow::Cow::Borrowed("symbols") => {
                    if !ids_match.is_match(&value) {
                        return Err("invalid symbols ids format".to_string());
                    }

                    let caps = ids_capture.captures_iter(&value);
                    let mut symbol_ids: Vec<u64> = Vec::new();
                    for cap in caps {
                        // We can unwrap safely here because of validation (is_match) below.
                        let id_str = cap.name("id").unwrap().as_str();
                        let id = id_str.parse::<u64>().unwrap();

                        symbol_ids.push(id)
                    }
                    request_filter.symbols = Some(symbol_ids);
                }
                std::borrow::Cow::Borrowed("tags") => {
                    if !ids_match.is_match(&value) {
                        return Err("invalid tags ids format".to_string());
                    }

                    let caps = ids_capture.captures_iter(&value);
                    let mut tag_ids: Vec<u64> = Vec::new();
                    for cap in caps {
                        // We can unwrap safely here because of validation (is_match) below.
                        let id_str = cap.name("id").unwrap().as_str();
                        let id = id_str.parse::<u64>().unwrap();

                        tag_ids.push(id);
                    }
                    request_filter.tags = Some(tag_ids);
                }
                std::borrow::Cow::Borrowed("unit_type") => {
                    request_filter.unit_type = Some(value.to_string())
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
            permission: String::from("r"),
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
            storage_archive: Some(false),
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

impl<S> FromRequestParts<S> for RequestFilter
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Build full absolute URL string so RequestFilter::try_from works
        // axum Parts.uri may not contain scheme/host, so we fake it
        let uri = parts.uri.clone();
        let path_and_query = uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("");
        let fake_base = "http://localhost"; // arbitrary base

        let full_url = format!("{fake_base}{path_and_query}");

        match RequestFilter::try_from(full_url.as_str()) {
            Ok(filter) => Ok(filter),
            Err(err) => {
                // Respond with 400 + error message
                let body = format!("invalid query/filter: {}", err);
                let response = (StatusCode::BAD_REQUEST, body);
                Err(response)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_to_string() {
        init_logger();

        let filter = RequestFilter {
            search: Some("%foo%".to_string()),
            id: Some(5),
            order_by: Some("foo".to_string()),
            order: "foo".to_string(),
            offset: Some(10),
            limit: Some(10),
            bookmark: true,
            borrowing: true,
            cas_number: Some(10),
            cas_number_string: Some("1234".to_string()),
            is_cmr: true,
            category: Some(10),
            custom_name_part_of: Some("foo".to_string()),
            empirical_formula: Some(10),
            entity: Some(10),
            entity_name: Some("foo".to_string()),
            hazard_statements: Some(vec![1, 2, 3]),
            history: true,
            storages: Some(vec![1, 2, 3]),
            name: Some(10),
            permission: "foo".to_string(),
            precautionary_statements: Some(vec![1, 2, 3]),
            producer: Some(10),
            producer_ref: Some(10),
            product: Some(10),
            person: Some(10),
            person_email: Some("foo@example.com".to_string()),
            product_specificity: Some("foo".to_string()),
            show_bio: true,
            show_chem: true,
            show_consu: true,
            signal_word: Some(10),
            storage: Some(10),
            storage_archive: Some(true),
            storage_barecode: Some("foo".to_string()),
            storage_batch_number: Some("foo".to_string()),
            storage_to_destroy: true,
            store_location: Some(10),
            store_location_can_store: true,
            supplier: Some(10),
            symbols: Some(vec![1, 2, 3]),
            tags: Some(vec![1, 2, 3]),
            unit_type: Some("foo".to_string()),
        };

        assert_eq!(
            filter.to_string(),
            "?search=%foo%\
        &id=5\
        &order_by=foo\
        &order=foo\
        &offset=10\
        &limit=10\
        &bookmark=true\
        &borrowing=true\
        &cas_number=10\
        &cas_number_string=1234\
        &is_cmr=true\
        &category=10\
        &custom_name_part_of=foo\
        &empirical_formula=10\
        &entity=10\
        &entity_name=foo\
        &hazard_statements=1,2,3\
        &history=true\
        &storages=1,2,3\
        &name=10\
        &permission=foo\
        &precautionary_statements=1,2,3\
        &producer=10\
        &producer_ref=10\
        &product=10\
        &person=10\
        &person_email=foo@example.com\
        &product_specificity=foo\
        &show_bio=true\
        &show_chem=true\
        &show_consu=true\
        &signal_word=10\
        &storage=10\
        &storage_archive=true\
        &storage_barecode=foo\
        &storage_batch_number=foo\
        &storage_to_destroy=true\
        &store_location=10\
        &store_location_can_store=true\
        &supplier=10\
        &symbols=1,2,3\
        &tags=1,2,3\
        &unit_type=foo"
                .to_string()
        )
    }

    #[test]
    fn test_try_from() {
        init_logger();

        // Valid values.
        let filter = RequestFilter::try_from(
            "http://localhost/?search=%foo%\
        &order_by=foo\
        &order=foo\
        &offset=10\
        &limit=10\
        &bookmark=true\
        &borrowing=true\
        &cas_number=10\
        &is_cmr=true\
        &category=10\
        &custom_name_part_of=foo\
        &empirical_formula=10\
        &entity=10\
        &hazard_statements=1,2,3\
        &history=true\
        &storages=1,2,3\
        &name=10\
        &permission=foo\
        &precautionary_statements=1,2,3\
        &producer=10\
        &producer_ref=10\
        &product=10\
        &product_specificity=foo\
        &show_bio=true\
        &show_chem=true\
        &show_consu=true\
        &signal_word=10\
        &storage=10\
        &storage_archive=true\
        &storage_barecode=foo\
        &storage_batch_number=foo\
        &storage_to_destroy=true\
        &store_location=10\
        &store_location_can_store=true\
        &supplier=10\
        &symbols=1,2,3\
        &tags=1,2,3\
        &unit_type=foo",
        );

        assert_eq!(filter.clone().unwrap().search, Some(String::from("%foo%")));
        assert_eq!(filter.clone().unwrap().order_by, Some(String::from("foo")));
        assert_eq!(filter.clone().unwrap().order, "foo");
        assert_eq!(filter.clone().unwrap().offset, Some(10));
        assert_eq!(filter.clone().unwrap().limit, Some(10));
        assert!(filter.clone().unwrap().bookmark);
        assert!(filter.clone().unwrap().borrowing);
        assert_eq!(filter.clone().unwrap().cas_number, Some(10));
        assert!(filter.clone().unwrap().is_cmr);
        assert_eq!(filter.clone().unwrap().category, Some(10));
        assert_eq!(
            filter.clone().unwrap().custom_name_part_of,
            Some(String::from("foo"))
        );
        assert_eq!(filter.clone().unwrap().empirical_formula, Some(10));
        assert_eq!(filter.clone().unwrap().entity, Some(10));
        assert_eq!(
            filter.clone().unwrap().hazard_statements,
            Some(vec![1, 2, 3])
        );
        assert!(filter.clone().unwrap().history);
        assert_eq!(filter.clone().unwrap().storages, Some(vec![1, 2, 3]));
        assert_eq!(filter.clone().unwrap().name, Some(10));
        assert_eq!(filter.clone().unwrap().permission, "foo");
        assert_eq!(
            filter.clone().unwrap().precautionary_statements,
            Some(vec![1, 2, 3])
        );
        assert_eq!(filter.clone().unwrap().producer, Some(10));
        assert_eq!(filter.clone().unwrap().producer_ref, Some(10));
        assert_eq!(filter.clone().unwrap().product, Some(10));
        assert_eq!(
            filter.clone().unwrap().product_specificity,
            Some(String::from("foo"))
        );
        assert!(filter.clone().unwrap().show_bio);
        assert!(filter.clone().unwrap().show_chem);
        assert!(filter.clone().unwrap().show_consu);
        assert_eq!(filter.clone().unwrap().signal_word, Some(10));
        assert_eq!(filter.clone().unwrap().storage, Some(10));
        assert!(filter.clone().unwrap().storage_archive.is_some_and(|x| x));
        assert_eq!(
            filter.clone().unwrap().storage_barecode,
            Some(String::from("foo"))
        );
        assert_eq!(
            filter.clone().unwrap().storage_batch_number,
            Some(String::from("foo"))
        );
        assert!(filter.clone().unwrap().storage_to_destroy);
        assert_eq!(filter.clone().unwrap().store_location, Some(10));
        assert!(filter.clone().unwrap().store_location_can_store);
        assert_eq!(filter.clone().unwrap().supplier, Some(10));
        assert_eq!(filter.clone().unwrap().symbols, Some(vec![1, 2, 3]));
        assert_eq!(filter.clone().unwrap().tags, Some(vec![1, 2, 3]));
        assert_eq!(filter.clone().unwrap().unit_type, Some(String::from("foo")));

        // Invalid values.
        let param_int = vec![
            "offset",
            "limit",
            "cas_number",
            "category",
            "empirical_formula",
            "entity",
            "name",
            "producer",
            "producer_ref",
            "product",
            "signal_word",
            "storage",
            "store_location",
            "supplier",
        ];
        let param_bool = vec![
            "bookmark",
            "borrowing",
            "is_cmr",
            "history",
            "show_bio",
            "show_chem",
            "show_consu",
            "storage_archive",
            "storage_to_destroy",
            "store_location_can_store",
        ];
        let param_vec_int = vec![
            "hazard_statements",
            "storages",
            "precautionary_statements",
            "symbols",
            "tags",
        ];

        for param in param_int {
            // test not digit
            let filter = RequestFilter::try_from(format!("http://localhost/?{param}=ab").as_str());
            assert!(filter.is_err());
        }

        for param in param_bool {
            // test not bool
            let filter = RequestFilter::try_from(format!("http://localhost/?{param}=ab").as_str());
            assert!(filter.is_err());
        }

        for param in param_vec_int {
            // test not digit
            let filter = RequestFilter::try_from(format!("http://localhost/?{param}=A").as_str());
            assert!(filter.is_err());

            let filter =
                RequestFilter::try_from(format!("http://localhost/?{param}=1,2,A").as_str());
            assert!(filter.is_err());

            // test wrong separator
            let filter = RequestFilter::try_from(format!("http://localhost/?{param}=1;2").as_str());
            assert!(filter.is_err());
        }

        // Search with URL encoded spaces.
        let filter = RequestFilter::try_from("http://localhost/?search=acide+chlor");
        assert!(filter.is_ok());
        let filter = RequestFilter::try_from("http://localhost/?search=acide%20chlor");
        assert!(filter.is_ok());

        // ID at the end of the URL.
        let filter = RequestFilter::try_from("http://localhost/1234");
        assert_eq!(filter.unwrap().id, Some(1234));
        let filter = RequestFilter::try_from("http://localhost/1234?id=4321");
        assert_eq!(filter.unwrap().id, Some(4321));
        let filter = RequestFilter::try_from("http://localhost/id=4321");
        assert_eq!(filter.unwrap().id, None);
        let filter = RequestFilter::try_from("https://192.168.1.18:8443/back/entities/10");
        assert_eq!(filter.unwrap().id, Some(10));
    }
}
