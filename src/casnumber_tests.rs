#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::casnumber::*;

    #[test]
    fn test_cas_number_searchable() {
        let cas_number = CasNumber {
            match_exact_search: true,
            cas_number_id: Some(1),
            cas_number_label: "test".to_string(),
        };

        let mut created_cas_number = cas_number.create();
        assert!(!created_cas_number.match_exact_search);
        assert_eq!(created_cas_number.cas_number_id, None);
        assert_eq!(created_cas_number.cas_number_label, String::new());

        created_cas_number.set_exact_search(true);
        assert!(created_cas_number.get_exact_search());

        assert_eq!(created_cas_number.get_table_name(), "cas_number");
        assert_eq!(created_cas_number.get_id_field_name(), "cas_number_id");
        assert_eq!(created_cas_number.get_text_field_name(), "cas_number_label");

        created_cas_number.set_id_field(2);
        assert_eq!(created_cas_number.get_id(), Some(2));

        created_cas_number.set_text_field("test2");
        assert_eq!(created_cas_number.get_text(), "test2");
    }

    #[test]
    fn test_sanitize_and_validate_cas_number() {
        let mut cas_number = CasNumber {
            match_exact_search: false,
            cas_number_id: Some(1),
            cas_number_label: "  7732-18-5  ".to_string(),
        };
        assert!(cas_number.sanitize_and_validate().is_ok());
        assert_eq!(cas_number.cas_number_label, "7732-18-5");

        let mut cas_number = CasNumber {
            match_exact_search: false,
            cas_number_id: Some(2),
            cas_number_label: "  97-65-4  ".to_string(),
        };
        assert!(cas_number.sanitize_and_validate().is_ok());
        assert_eq!(cas_number.cas_number_label, "97-65-4");

        let mut cas_number = CasNumber {
            match_exact_search: false,
            cas_number_id: Some(3),
            cas_number_label: "7732-18-5".to_string(),
        };
        assert!(cas_number.sanitize_and_validate().is_ok());
        assert_eq!(cas_number.cas_number_label, "7732-18-5");
    }

    #[test]
    fn test_sanitize_and_validate_invalid_cas_number() {
        let mut cas_number = CasNumber {
            match_exact_search: false,
            cas_number_id: Some(4),
            cas_number_label: "12345".to_string(),
        };
        assert!(cas_number.sanitize_and_validate().is_err());

        let mut cas_number = CasNumber {
            match_exact_search: false,
            cas_number_id: Some(5),
            cas_number_label: "abcdef".to_string(),
        };
        assert!(cas_number.sanitize_and_validate().is_err());
    }

    #[test]
    fn test_sanitize_and_validate_empty_cas_number() {
        let mut cas_number = CasNumber {
            match_exact_search: false,
            cas_number_id: Some(6),
            cas_number_label: String::default(),
        };
        assert!(cas_number.sanitize_and_validate().is_err());
    }
}
