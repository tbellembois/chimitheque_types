#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::requestfilter::*;

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
            order: "asc".to_string(),
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
            // permission: "read".to_string(),
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
            "?search=test&id=1&order_by=name&order=asc&offset=0&limit=10&bookmark=true&borrowing=true&cas_number=12345&cas_number_string=ABC123&is_cmr=true&category=1&custom_name_part_of=part1&empirical_formula=1&entity=1&entity_name=entity1&hazard_statements=1,2,3&precautionary_statements=1,2,3&history=true&storages=1,2,3&name=1&producer=1&producer_ref=1&product=1&person=1&person_email=person@example.com&product_specificity=specificity1&show_bio=true&show_chem=true&show_consu=true&signal_word=1&storage=1&storage_archive=true&storage_barecode=barcode1&storage_batch_number=batch1&storage_to_destroy=true&store_location=1&store_location_can_store=true&supplier=1&symbols=1,2,3&tags=1,2,3&unit_type=quantity"
        );
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
        assert_eq!(default_filter.order, "asc");
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
        // assert_eq!(default_filter.permission, "r");
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
