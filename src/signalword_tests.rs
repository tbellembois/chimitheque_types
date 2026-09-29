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

    #[test]
    fn test_create_defaults() {
        let signal_word = SignalWord::default();
        let created = signal_word.create();
        assert!(!created.match_exact_search);
        assert_eq!(created.signal_word_id, None);
        assert_eq!(created.signal_word_label, String::new());
    }

    #[test]
    fn test_set_and_get_exact_search() {
        let mut signal_word = SignalWord::default();
        signal_word.set_exact_search(true);
        assert!(signal_word.get_exact_search());
        signal_word.set_exact_search(false);
        assert!(!signal_word.get_exact_search());
    }

    #[test]
    fn test_get_table_name() {
        let signal_word = SignalWord::default();
        assert_eq!(signal_word.get_table_name(), "signal_word");
    }

    #[test]
    fn test_get_id_field_name() {
        let signal_word = SignalWord::default();
        assert_eq!(signal_word.get_id_field_name(), "signal_word_id");
    }

    #[test]
    fn test_set_and_get_id() {
        let mut signal_word = SignalWord::default();
        signal_word.set_id_field(42);
        assert_eq!(signal_word.get_id(), Some(42));
        signal_word.set_id_field(0);
        assert_eq!(signal_word.get_id(), Some(0));
    }

    #[test]
    fn test_set_and_get_text() {
        let mut signal_word = SignalWord::default();
        signal_word.set_text_field("Danger");
        assert_eq!(signal_word.get_text(), "Danger");
        signal_word.set_text_field("");
        assert_eq!(signal_word.get_text(), "");
    }
}
