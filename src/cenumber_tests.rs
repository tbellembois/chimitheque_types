#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::cenumber::*;

    #[test]
    fn test_ce_number_searchable() {
        let ce_number = CeNumber {
            match_exact_search: true,
            ce_number_id: Some(1),
            ce_number_label: "test".to_string(),
        };

        let mut created_ce_number = ce_number.create();
        assert!(!created_ce_number.match_exact_search);
        assert_eq!(created_ce_number.ce_number_id, None);
        assert_eq!(created_ce_number.ce_number_label, String::new());

        created_ce_number.set_exact_search(true);
        assert!(created_ce_number.get_exact_search());

        assert_eq!(created_ce_number.get_table_name(), "ce_number");
        assert_eq!(created_ce_number.get_id_field_name(), "ce_number_id");
        assert_eq!(created_ce_number.get_text_field_name(), "ce_number_label");

        created_ce_number.set_id_field(2);
        assert_eq!(created_ce_number.get_id(), Some(2));

        created_ce_number.set_text_field("test2");
        assert_eq!(created_ce_number.get_text(), "test2");
    }

    #[test]
    fn test_create_defaults() {
        let ce_number = CeNumber::default();
        let created = ce_number.create();
        assert!(!created.match_exact_search);
        assert_eq!(created.ce_number_id, None);
        assert_eq!(created.ce_number_label, String::new());
    }

    #[test]
    fn test_set_and_get_exact_search() {
        let mut ce_number = CeNumber::default();
        ce_number.set_exact_search(true);
        assert!(ce_number.get_exact_search());
        ce_number.set_exact_search(false);
        assert!(!ce_number.get_exact_search());
    }

    #[test]
    fn test_get_table_name() {
        let ce_number = CeNumber::default();
        assert_eq!(ce_number.get_table_name(), "ce_number");
    }

    #[test]
    fn test_get_id_field_name() {
        let ce_number = CeNumber::default();
        assert_eq!(ce_number.get_id_field_name(), "ce_number_id");
    }

    #[test]
    fn test_set_and_get_id() {
        let mut ce_number = CeNumber::default();
        ce_number.set_id_field(42);
        assert_eq!(ce_number.get_id(), Some(42));
        ce_number.set_id_field(0);
        assert_eq!(ce_number.get_id(), Some(0));
    }

    #[test]
    fn test_set_and_get_text() {
        let mut ce_number = CeNumber::default();
        ce_number.set_text_field("214-480-6");
        assert_eq!(ce_number.get_text(), "214-480-6");
        ce_number.set_text_field("");
        assert_eq!(ce_number.get_text(), "");
    }

    #[test]
    fn test_sanitize_and_validate_cenumber() {
        let mut cenumber = CeNumber {
            match_exact_search: false,
            ce_number_id: Some(1),
            ce_number_label: "  214-480-6  ".to_string(),
        };
        assert!(cenumber.sanitize_and_validate().is_ok());
        assert_eq!(cenumber.ce_number_label, "214-480-6");

        let mut cenumber = CeNumber {
            match_exact_search: false,
            ce_number_id: Some(2),
            ce_number_label: "  687-037-1  ".to_string(),
        };
        assert!(cenumber.sanitize_and_validate().is_ok());
        assert_eq!(cenumber.ce_number_label, "687-037-1");

        let mut cenumber = CeNumber {
            match_exact_search: false,
            ce_number_id: Some(3),
            ce_number_label: "813-259-7".to_string(),
        };
        assert!(cenumber.sanitize_and_validate().is_ok());
        assert_eq!(cenumber.ce_number_label, "813-259-7");
    }

    #[test]
    fn test_sanitize_and_validate_invalid_cenumber() {
        let mut cenumber = CeNumber {
            match_exact_search: false,
            ce_number_id: Some(4),
            ce_number_label: "12345".to_string(),
        };
        assert!(cenumber.sanitize_and_validate().is_err());

        let mut cenumber = CeNumber {
            match_exact_search: false,
            ce_number_id: Some(5),
            ce_number_label: "abcdef".to_string(),
        };
        assert!(cenumber.sanitize_and_validate().is_err());
    }

    #[test]
    fn test_sanitize_and_validate_empty_cenumber() {
        let mut cenumber = CeNumber {
            match_exact_search: false,
            ce_number_id: Some(6),
            ce_number_label: String::default(),
        };
        assert!(cenumber.sanitize_and_validate().is_err());
    }
}
