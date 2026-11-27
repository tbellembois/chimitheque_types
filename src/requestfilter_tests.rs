#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::requestfilter::*;
    use crate::sortorder::SortOrder;
    use std::fmt::Write as _;

    #[test]
    fn test_requestfilter_serde_defaults_match_default() {
        let filter: RequestFilter = serde_json::from_str("{}").unwrap();
        assert_eq!(filter.order, SortOrder::Asc);
        assert_eq!(
            serde_json::to_value(&filter).unwrap(),
            serde_json::to_value(RequestFilter::default()).unwrap()
        );
    }

    #[test]
    fn test_requestfilter_serde_order() {
        for (input, expected) in [
            ("asc", SortOrder::Asc),
            ("desc", SortOrder::Desc),
            ("DESC", SortOrder::Desc),
        ] {
            let payload = serde_json::json!({"order": input});
            let filter: RequestFilter = serde_json::from_value(payload).unwrap();
            assert_eq!(filter.order, expected);
            assert_eq!(
                serde_json::to_value(&filter).unwrap()["order"],
                expected.as_str()
            );
        }
    }

    #[test]
    fn test_requestfilter_serde_rejects_invalid_order() {
        for order in ["", "up", "ascending"] {
            let payload = serde_json::json!({"order": order});
            assert!(
                serde_json::from_value::<RequestFilter>(payload).is_err(),
                "{order:?}"
            );
        }
    }

    #[test]
    fn test_requestfilter_try_from_order() {
        for (value, expected) in [
            ("asc", SortOrder::Asc),
            ("desc", SortOrder::Desc),
            ("DESC", SortOrder::Desc),
            ("Asc", SortOrder::Asc),
        ] {
            let request = format!("http://localhost?order={value}");
            let filter = RequestFilter::try_from(request.as_str()).unwrap();
            assert_eq!(filter.order, expected, "{value}");
            assert_eq!(filter.to_string(), format!("?order={}", expected.as_str()));
        }
    }

    #[test]
    fn test_requestfilter_try_from_rejects_invalid_order() {
        for value in ["", "up", "ascending"] {
            let request = format!("http://localhost?order={value}");
            assert_eq!(
                RequestFilter::try_from(request.as_str())
                    .unwrap_err()
                    .to_string(),
                format!("error with order query parameter: wrong sort order: {value}")
            );
        }
    }

    #[test]
    fn test_requestfilter_display() {
        let filter = RequestFilter::default();
        assert_eq!(filter.to_string(), "?order=asc");
    }

    #[test]
    fn test_requestfilter_display_all_fields() {
        let filter = RequestFilter {
            search: Some("test".to_string()),
            id: Some(1),
            order_by: Some("name".to_string()),
            order: SortOrder::Asc,
            offset: Some(0),
            limit: Some(10),
            bookmark: true,
            borrowing: true,
            cas_number: Some(12345),
            cas_number_string: Some("ABC123".to_string()),
            is_cmr: true,
            category: Some(1),
            custom_name_part_of: Some("part1".to_string()),
            empirical_formula: Some(1),
            entity: Some(1),
            entity_name: Some("entity1".to_string()),
            hazard_statements: Some(vec![1, 2, 3]),
            history: true,
            storages: Some(vec![1, 2, 3]),
            name: Some(1),
            person: Some(1),
            person_email: Some("person@example.com".to_string()),
            precautionary_statements: Some(vec![1, 2, 3]),
            producer: Some(1),
            producer_ref: Some(1),
            product: Some(1),
            product_specificity: Some("specificity1".to_string()),
            show_bio: true,
            show_chem: true,
            show_consu: true,
            signal_word: Some(1),
            storage: Some(1),
            storage_archive: Some(true),
            storage_barecode: Some("barcode1".to_string()),
            storage_batch_number: Some("batch1".to_string()),
            storage_to_destroy: true,
            store_location: Some(1),
            store_location_can_store: true,
            supplier: Some(1),
            symbols: Some(vec![1, 2, 3]),
            tags: Some(vec![1, 2, 3]),
            unit_type: Some("quantity".to_string()),
        };
        assert_eq!(
            filter.to_string(),
            "?search=test&id=1&order_by=name&order=asc&offset=0&limit=10&bookmark=true&borrowing=true&cas_number=12345&cas_number_string=ABC123&is_cmr=true&category=1&custom_name_part_of=part1&empirical_formula=1&entity=1&entity_name=entity1&hazard_statements=1%2C2%2C3&precautionary_statements=1%2C2%2C3&history=true&storages=1%2C2%2C3&name=1&producer=1&producer_ref=1&product=1&person=1&person_email=person%40example.com&product_specificity=specificity1&show_bio=true&show_chem=true&show_consu=true&signal_word=1&storage=1&storage_archive=true&storage_barecode=barcode1&storage_batch_number=batch1&storage_to_destroy=true&store_location=1&store_location_can_store=true&supplier=1&symbols=1%2C2%2C3&tags=1%2C2%2C3&unit_type=quantity"
        );

        let request = format!("http://localhost/products{filter}");
        let parsed = RequestFilter::try_from(request.as_str()).unwrap();
        assert_eq!(
            serde_json::to_value(&parsed).unwrap(),
            serde_json::to_value(&filter).unwrap()
        );
    }

    #[test]
    fn test_requestfilter_decoded_parameter_names() {
        // Escaping every name exercises owned Cow keys for every supported field.
        let parameters = [
            ("search", "acetone%20%26%20ethanol"),
            ("id", "42"),
            ("order_by", "name"),
            ("order", "desc"),
            ("offset", "5"),
            ("limit", "10"),
            ("bookmark", "true"),
            ("borrowing", "true"),
            ("cas_number", "1"),
            ("cas_number_string", "7732-18-5"),
            ("is_cmr", "true"),
            ("category", "2"),
            ("custom_name_part_of", "part"),
            ("empirical_formula", "3"),
            ("entity", "4"),
            ("entity_name", "lab"),
            ("hazard_statements", "1%2C2"),
            ("history", "true"),
            ("storages", "3%2C4"),
            ("name", "5"),
            ("person", "6"),
            ("person_email", "chemist%2Blab%40example.com"),
            ("precautionary_statements", "5%2C6"),
            ("producer", "7"),
            ("producer_ref", "8"),
            ("product", "9"),
            ("product_specificity", "specific"),
            ("show_bio", "true"),
            ("show_chem", "true"),
            ("show_consu", "true"),
            ("signal_word", "10"),
            ("storage", "11"),
            ("storage_archive", "false"),
            ("storage_barecode", "barcode"),
            ("storage_batch_number", "batch"),
            ("storage_to_destroy", "true"),
            ("store_location", "12"),
            ("store_location_can_store", "true"),
            ("supplier", "13"),
            ("symbols", "7%2C8"),
            ("tags", "9%2C10"),
            ("unit_type", "quantity"),
        ];

        for (name, value) in parameters {
            let encoded_name = name.bytes().fold(String::new(), |mut encoded, byte| {
                write!(encoded, "%{byte:02X}").unwrap();
                encoded
            });
            let plain_url = format!("http://localhost?{name}={value}");
            let encoded_url = format!("http://localhost?{encoded_name}={value}");
            let plain = RequestFilter::try_from(plain_url.as_str()).unwrap();
            let encoded = RequestFilter::try_from(encoded_url.as_str()).unwrap();

            assert_eq!(
                serde_json::to_value(&encoded).unwrap(),
                serde_json::to_value(&plain).unwrap(),
                "encoded name must behave like {name}"
            );
        }
    }

    #[test]
    fn test_requestfilter_decoded_parameter_names_preserve_validation() {
        // Decoded names must not bypass numeric, boolean, or ID-list validation.
        for (plain_query, encoded_query) in [
            ("id=invalid", "%69d=invalid"),
            ("limit=invalid", "l%69mit=invalid"),
            ("bookmark=invalid", "book%6dark=invalid"),
            ("order=invalid", "ord%65r=invalid"),
            ("tags=1,,2", "ta%67s=1,,2"),
            ("tags=18446744073709551616", "ta%67s=18446744073709551616"),
        ] {
            let plain_url = format!("http://localhost?{plain_query}");
            let encoded_url = format!("http://localhost?{encoded_query}");
            let expected_error = RequestFilter::try_from(plain_url.as_str()).unwrap_err();
            assert_eq!(
                RequestFilter::try_from(encoded_url.as_str()).unwrap_err(),
                expected_error,
                "validation differs for {encoded_query}"
            );
        }
    }

    #[test]
    fn test_requestfilter_mixed_parameter_name_encodings() {
        // Preserve last-value-wins semantics when plain and escaped keys are mixed.
        for (query, expected) in [
            ("search=first&se%61rch=second", "second"),
            ("se%61rch=first&search=second", "second"),
            ("se%61rch=kept&unknown%5Fkey=ignored", "kept"),
            // URL decoding happens once; this key decodes to se%61rch, not search.
            ("search=kept&se%2561rch=ignored", "kept"),
        ] {
            let request = format!("http://localhost?{query}");
            let filter = RequestFilter::try_from(request.as_str()).unwrap();
            assert_eq!(filter.search.as_deref(), Some(expected), "{query}");
        }
    }

    #[test]
    fn test_requestfilter_display_encodes_special_characters() {
        let filter = RequestFilter {
            search: Some("acetone & ethanol+#50%".to_string()),
            person_email: Some("chemist+lab@example.com".to_string()),
            ..Default::default()
        };

        assert_eq!(
            filter.to_string(),
            "?search=acetone+%26+ethanol%2B%2350%25&order=asc&person_email=chemist%2Blab%40example.com"
        );
    }

    #[test]
    fn test_requestfilter_text_fields_round_trip() {
        for value in [
            "",
            "acetone&limit=1",
            "chemist+lab@example.com",
            "#fragment",
            "%26%3D",
            "two words",
            "éthanol 化学 🧪",
            "a=b?c/d",
        ] {
            let filter = RequestFilter {
                search: Some(value.to_string()),
                order_by: Some(value.to_string()),
                cas_number_string: Some(value.to_string()),
                custom_name_part_of: Some(value.to_string()),
                entity_name: Some(value.to_string()),
                person_email: Some(value.to_string()),
                product_specificity: Some(value.to_string()),
                storage_barecode: Some(value.to_string()),
                storage_batch_number: Some(value.to_string()),
                unit_type: Some(value.to_string()),
                limit: Some(25),
                ..Default::default()
            };
            let request = format!("http://localhost/products{filter}");
            let parsed = RequestFilter::try_from(request.as_str()).unwrap();

            assert_eq!(
                serde_json::to_value(&parsed).unwrap(),
                serde_json::to_value(&filter).unwrap(),
                "round trip failed for {value:?}"
            );
        }
    }

    const ID_LIST_PARAMETERS: [&str; 5] = [
        "hazard_statements",
        "storages",
        "precautionary_statements",
        "symbols",
        "tags",
    ];

    #[test]
    fn test_requestfilter_rejects_overflowing_id_lists() {
        for parameter in ID_LIST_PARAMETERS {
            for value in [
                "18446744073709551616",
                "18446744073709551616,1",
                "1,18446744073709551616,2",
                "1,18446744073709551616",
                "9999999999999999999999999999999999999999",
            ] {
                let request = format!("http://localhost?{parameter}={value}");
                let error = RequestFilter::try_from(request.as_str())
                    .expect_err("overflowing IDs must reject the entire filter")
                    .to_string();

                assert!(error.contains(parameter), "{error}");
                assert!(
                    error.contains("18446744073709551616") || error.contains(value),
                    "{error}"
                );
            }
        }
    }

    #[test]
    fn test_requestfilter_parses_id_lists_without_losing_values() {
        for parameter in ID_LIST_PARAMETERS {
            for (value, expected) in [
                ("18446744073709551615", vec![u64::MAX]),
                ("0,18446744073709551615,1,1", vec![0, u64::MAX, 1, 1]),
                ("1%2C2%2C3", vec![1, 2, 3]),
                ("1,2,", vec![1, 2]),
            ] {
                let request = format!("http://localhost?{parameter}={value}");
                let filter = RequestFilter::try_from(request.as_str()).unwrap();

                assert_eq!(
                    serde_json::to_value(&filter).unwrap()[parameter],
                    serde_json::json!(expected),
                    "incorrect IDs for {parameter}={value}"
                );
            }
        }
    }

    #[test]
    fn test_requestfilter_error_variants() {
        use crate::error::RequestFilterError;
        use std::error::Error as _;

        let error = RequestFilter::try_from("not a url").unwrap_err();
        assert!(matches!(error, RequestFilterError::InvalidUrl(_)));
        assert!(error.to_string().starts_with("can not parse url: "));
        assert!(error.source().is_some());

        let error = RequestFilter::try_from("http://localhost?limit=ten").unwrap_err();
        assert!(matches!(
            &error,
            RequestFilterError::InvalidParameter {
                parameter: "limit",
                ..
            }
        ));
        assert_eq!(
            error.to_string(),
            "error with limit query parameter: invalid digit found in string"
        );
        assert!(error.source().is_none());

        let error = RequestFilter::try_from("http://localhost?tags=1,,2").unwrap_err();
        assert_eq!(
            error,
            RequestFilterError::InvalidIdList { parameter: "tags" }
        );
        assert_eq!(error.to_string(), "invalid tags ids format");
    }

    #[test]
    fn test_requestfilter_rejects_malformed_id_lists() {
        for parameter in ID_LIST_PARAMETERS {
            for value in ["", ",1", "1,,2", "1;2", "1,a,2", "-1", "1,%202"] {
                let request = format!("http://localhost?{parameter}={value}");
                let error = RequestFilter::try_from(request.as_str()).unwrap_err();

                assert_eq!(error.to_string(), format!("invalid {parameter} ids format"));
            }
        }
    }

    #[test]
    fn test_requestfilter_try_from_id_at_end_of_url() {
        let id = |url: &str| RequestFilter::try_from(url).unwrap().id;

        // An id at the end of the path is captured.
        assert_eq!(id("http://localhost/products/12"), Some(12));
        assert_eq!(id("http://localhost/12"), Some(12));

        // No id when the path does not end with a number.
        for url in [
            "http://localhost/products",
            "http://localhost/products/",
            "http://localhost/products/12/",
            "http://localhost/products/12a",
            "http://localhost/products/1.5",
        ] {
            assert_eq!(id(url), None, "{url}");
        }

        // A query or a fragment after the path does not hide the id.
        assert_eq!(id("http://localhost/products/12?search=x"), Some(12));
        assert_eq!(id("http://localhost/products/12#top"), Some(12));

        // The id query parameter is used, and overrides an id in the path.
        assert_eq!(id("http://localhost/products?id=5"), Some(5));
        assert_eq!(id("http://localhost/products/12?id=5"), Some(5));

        // An id too big for a u64 is rejected, as in the id query parameter.
        for url in [
            "http://localhost/products/99999999999999999999",
            "http://localhost/products?id=99999999999999999999",
        ] {
            let error = RequestFilter::try_from(url).unwrap_err();
            assert!(
                matches!(
                    error,
                    RequestFilterError::InvalidParameter {
                        parameter: "id",
                        ..
                    }
                ),
                "{url}: {error:?}"
            );
        }
    }

    #[test]
    fn test_requestfilter_try_from_valid() {
        let input = "http://localhost?search=test&limit=10";
        let filter = RequestFilter::try_from(input).unwrap();
        assert_eq!(filter.search, Some("test".to_string()));
        assert_eq!(filter.limit, Some(10));
    }

    #[test]
    fn test_requestfilter_try_from_invalid() {
        let input = "http://localhost?invalid_string";
        let result = RequestFilter::try_from(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_requestfilter_default_values() {
        let default_filter = RequestFilter::default();
        assert_eq!(default_filter.search, None);
        assert_eq!(default_filter.id, None);
        assert_eq!(default_filter.order_by, None);
        assert_eq!(default_filter.order, SortOrder::Asc);
        assert_eq!(default_filter.offset, None);
        assert_eq!(default_filter.limit, None);
        assert!(!default_filter.bookmark);
        assert!(!default_filter.borrowing);
        assert_eq!(default_filter.cas_number, None);
        assert_eq!(default_filter.cas_number_string, None);
        assert!(!default_filter.is_cmr);
        assert_eq!(default_filter.category, None);
        assert_eq!(default_filter.custom_name_part_of, None);
        assert_eq!(default_filter.empirical_formula, None);
        assert_eq!(default_filter.entity, None);
        assert_eq!(default_filter.entity_name, None);
        assert_eq!(default_filter.hazard_statements, None);
        assert!(!default_filter.history);
        assert_eq!(default_filter.storages, None);
        assert_eq!(default_filter.name, None);
        assert_eq!(default_filter.person, None);
        assert_eq!(default_filter.person_email, None);
        assert_eq!(default_filter.precautionary_statements, None);
        assert_eq!(default_filter.producer, None);
        assert_eq!(default_filter.producer_ref, None);
        assert_eq!(default_filter.product, None);
        assert_eq!(default_filter.product_specificity, None);
        assert!(!default_filter.show_bio);
        assert!(!default_filter.show_chem);
        assert!(!default_filter.show_consu);
        assert_eq!(default_filter.signal_word, None);
        assert_eq!(default_filter.storage, None);
        assert_eq!(default_filter.storage_archive, None);
        assert_eq!(default_filter.storage_barecode, None);
        assert_eq!(default_filter.storage_batch_number, None);
        assert!(!default_filter.storage_to_destroy);
        assert_eq!(default_filter.store_location, None);
        assert!(!default_filter.store_location_can_store);
        assert_eq!(default_filter.supplier, None);
        assert_eq!(default_filter.symbols, None);
        assert_eq!(default_filter.tags, None);
        assert_eq!(default_filter.unit_type, None);
    }
}
