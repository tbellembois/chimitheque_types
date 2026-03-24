#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::signalword::*;

    #[test]
    fn test_signalword_searchable() {
        let signalword = SignalWord {
            match_exact_search: true,
            signal_word_id: Some(1),
            signal_word_label: "test".to_string(),
        };

        let mut created_signalword = signalword.create();
        assert!(!created_signalword.match_exact_search);
        assert_eq!(created_signalword.signal_word_id, None);
        assert_eq!(created_signalword.signal_word_label, String::new());

        created_signalword.set_exact_search(true);
        assert!(created_signalword.get_exact_search());

        assert_eq!(created_signalword.get_table_name(), "signal_word");
        assert_eq!(created_signalword.get_id_field_name(), "signal_word_id");
        assert_eq!(
            created_signalword.get_text_field_name(),
            "signal_word_label"
        );

        created_signalword.set_id_field(2);
        assert_eq!(created_signalword.get_id(), Some(2));

        created_signalword.set_text_field("test2");
        assert_eq!(created_signalword.get_text(), "test2");
    }
}
