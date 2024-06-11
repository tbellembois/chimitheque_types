use log::debug;
use regex::Regex;
use serde::Serialize;
use url::Url;

#[derive(Debug, Clone, Serialize)]
pub struct RequestFilter {
    pub search: Option<String>,
    pub order_by: Option<String>,
    pub order: String,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
    pub bookmark: bool,
    pub borrowing: bool,
    pub cas_number: Option<u64>,
    pub cas_number_cmr: bool,
    pub category: Option<u64>,
    pub custom_name_part_of: Option<String>,
    pub empirical_formula: Option<u64>,
    pub entity: Option<u64>,
    pub hazard_statements: Option<Vec<u64>>,
    pub history: bool,
    pub storages: Option<Vec<u64>>,
    pub name: Option<u64>,
    pub permission: String,
    pub precautionary_statements: Option<Vec<u64>>,
    pub producer: Option<u64>,
    pub producer_ref: Option<u64>,
    pub product: Option<u64>,
    pub product_specificity: Option<String>,
    pub show_bio: bool,
    pub show_chem: bool,
    pub show_consu: bool,
    pub signal_word: Option<u64>,
    pub storage: Option<u64>,
    pub storage_archive: bool,
    pub storage_barecode: Option<String>,
    pub storage_batch_number: Option<String>,
    pub storage_to_destroy: bool,
    pub store_location: Option<u64>,
    pub store_location_can_store: bool,
    pub supplier: Option<u64>,
    pub symbols: Option<Vec<u64>>,
    pub tags: Option<Vec<u64>>,
    pub unit_type: Option<String>,
}

impl TryFrom<&str> for RequestFilter {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        debug!("value:{value}");

        // Result populated after by the request parameters.
        let mut request_filter: RequestFilter = Default::default();

        // Parse request URL.
        let url = match Url::parse(value) {
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

        // Get the query parameters.
        for query_pair in url.query_pairs() {
            debug!("query_pair:{:?}", query_pair);

            let (key, value) = query_pair;
            match key {
                std::borrow::Cow::Borrowed("search") => {
                    request_filter.search = Some(value.to_string())
                }
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
                std::borrow::Cow::Borrowed("cas_number_cmr") => match value.parse::<bool>() {
                    Ok(v) => request_filter.cas_number_cmr = v,
                    Err(e) => {
                        return Err(format!("error with cas_number_cmr query parameter: {e}"))
                    }
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
                    Ok(v) => request_filter.storage_archive = v,
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
            order_by: None,
            order: String::from("asc"),
            offset: None,
            limit: None,
            bookmark: false,
            borrowing: false,
            cas_number: None,
            cas_number_cmr: false,
            category: None,
            custom_name_part_of: None,
            empirical_formula: None,
            entity: None,
            hazard_statements: None,
            history: false,
            storages: None,
            name: None,
            permission: String::from("r"),
            precautionary_statements: None,
            producer: None,
            producer_ref: None,
            product: None,
            product_specificity: None,
            show_bio: false,
            show_chem: false,
            show_consu: false,
            signal_word: None,
            storage: None,
            storage_archive: false,
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
mod tests {

    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
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
        &cas_number_cmr=true\
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
        assert!(filter.clone().unwrap().cas_number_cmr);
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
        assert!(filter.clone().unwrap().storage_archive);
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
            "cas_number_cmr",
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
    }
}
