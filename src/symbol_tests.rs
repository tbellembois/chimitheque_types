#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::symbol::*;

    #[test]
    fn test_symbol_searchable() {
        let symbol = Symbol {
            match_exact_search: true,
            symbol_id: Some(1),
            symbol_label: "test".to_string(),
        };

        let mut created_symbol = symbol.create();
        assert!(!created_symbol.match_exact_search);
        assert_eq!(created_symbol.symbol_id, None);
        assert_eq!(created_symbol.symbol_label, String::new());

        created_symbol.set_exact_search(true);
        assert!(created_symbol.get_exact_search());

        assert_eq!(created_symbol.get_table_name(), "symbol");
        assert_eq!(created_symbol.get_id_field_name(), "symbol_id");
        assert_eq!(created_symbol.get_text_field_name(), "symbol_label");

        created_symbol.set_id_field(2);
        assert_eq!(created_symbol.get_id(), Some(2));

        created_symbol.set_text_field("test2");
        assert_eq!(created_symbol.get_text(), "test2");
    }
}
