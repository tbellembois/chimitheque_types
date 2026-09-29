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
            cas_number_cmr: None,
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
            cas_number_cmr: None,
        };
        assert!(cas_number.sanitize_and_validate().is_ok());
        assert_eq!(cas_number.cas_number_label, "7732-18-5");

        let mut cas_number = CasNumber {
            match_exact_search: false,
            cas_number_id: Some(2),
            cas_number_label: "  97-65-4  ".to_string(),
            cas_number_cmr: None,
        };
        assert!(cas_number.sanitize_and_validate().is_ok());
        assert_eq!(cas_number.cas_number_label, "97-65-4");

        let mut cas_number = CasNumber {
            match_exact_search: false,
            cas_number_id: Some(3),
            cas_number_label: "7732-18-5".to_string(),
            cas_number_cmr: None,
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
            cas_number_cmr: None,
        };
        assert!(cas_number.sanitize_and_validate().is_err());

        let mut cas_number = CasNumber {
            match_exact_search: false,
            cas_number_id: Some(5),
            cas_number_label: "abcdef".to_string(),
            cas_number_cmr: None,
        };
        assert!(cas_number.sanitize_and_validate().is_err());
    }

    #[test]
    fn test_sanitize_and_validate_empty_cas_number() {
        let mut cas_number = CasNumber {
            match_exact_search: false,
            cas_number_id: Some(6),
            cas_number_label: String::default(),
            cas_number_cmr: None,
        };
        assert!(cas_number.sanitize_and_validate().is_err());
    }

    #[test]
    fn test_create() {
        let cas_number = CasNumber {
            match_exact_search: true,
            cas_number_id: Some(1),
            cas_number_label: "test".to_string(),
            cas_number_cmr: Some("cmr".to_string()),
        };
        let created = cas_number.create();
        assert!(!created.match_exact_search);
        assert_eq!(created.cas_number_id, None);
        assert_eq!(created.cas_number_label, String::new());
        assert_eq!(created.cas_number_cmr, None);
    }

    #[test]
    fn test_set_and_get_exact_search() {
        let mut cas_number = CasNumber::default();
        cas_number.set_exact_search(true);
        assert!(cas_number.get_exact_search());
        cas_number.set_exact_search(false);
        assert!(!cas_number.get_exact_search());
    }

    #[test]
    fn test_get_table_name() {
        let cas_number = CasNumber::default();
        assert_eq!(cas_number.get_table_name(), "cas_number");
    }

    #[test]
    fn test_get_id_field_name() {
        let cas_number = CasNumber::default();
        assert_eq!(cas_number.get_id_field_name(), "cas_number_id");
    }

    #[test]
    fn test_set_and_get_id() {
        let mut cas_number = CasNumber::default();
        cas_number.set_id_field(42);
        assert_eq!(cas_number.get_id(), Some(42));
        cas_number.set_id_field(0);
        assert_eq!(cas_number.get_id(), Some(0));
    }

    #[test]
    fn test_set_and_get_text() {
        let mut cas_number = CasNumber::default();
        cas_number.set_text_field("123-45-6789");
        assert_eq!(cas_number.get_text(), "123-45-6789");
        cas_number.set_text_field("");
        assert_eq!(cas_number.get_text(), "");
    }
}
