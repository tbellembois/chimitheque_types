use super::*;
use chimitheque_traits::searchable::Searchable;

#[test]
fn test_sanitize_and_validate_valid() {
    let mut name = Name {
        name_label: "  test  ".to_string(),
        ..Default::default()
    };
    assert!(name.sanitize_and_validate().is_ok());
    assert_eq!(name.name_label, "TEST");
}

#[test]
fn test_sanitize_and_validate_empty() {
    let mut name = Name {
        name_label: String::new(),
        ..Default::default()
    };
    assert!(name.sanitize_and_validate().is_err());
}

#[test]
fn test_searchable_trait() {
    let mut name = Name {
        name_label: "Test".to_string(),
        ..Default::default()
    };

    // Test set/get exact search
    name.set_exact_search(true);
    assert!(name.get_exact_search());

    // Test table name
    assert_eq!(name.get_table_name(), "name");

    // Test ID field name
    assert_eq!(name.get_id_field_name(), "name_id");

    // Test set/get ID
    name.set_id_field(42);
    assert_eq!(name.get_id(), Some(42));

    // Test set/get text field
    name.set_text_field("New Test");
    assert_eq!(name.get_text(), "New Test");

    // Test create method
    let new_name = name.create();
    assert_eq!(new_name.name_label, "");
    assert_eq!(new_name.name_id, None);
    assert!(!new_name.match_exact_search);
}
