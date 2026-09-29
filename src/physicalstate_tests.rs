#[cfg(test)]
mod tests {
    use chimitheque_traits::searchable::Searchable;

    use crate::physicalstate::PhysicalState;

    #[test]
    fn test_create_defaults() {
        let physical_state = PhysicalState::default();
        let created = physical_state.create();
        assert!(!created.match_exact_search);
        assert_eq!(created.physical_state_id, None);
        assert_eq!(created.physical_state_label, String::new());
    }

    #[test]
    fn test_set_and_get_exact_search() {
        let mut physical_state = PhysicalState::default();
        physical_state.set_exact_search(true);
        assert!(physical_state.get_exact_search());
        physical_state.set_exact_search(false);
        assert!(!physical_state.get_exact_search());
    }

    #[test]
    fn test_get_table_name() {
        let physical_state = PhysicalState::default();
        assert_eq!(physical_state.get_table_name(), "physical_state");
    }

    #[test]
    fn test_get_id_field_name() {
        let physical_state = PhysicalState::default();
        assert_eq!(physical_state.get_id_field_name(), "physical_state_id");
    }

    #[test]
    fn test_set_and_get_id() {
        let mut physical_state = PhysicalState::default();
        physical_state.set_id_field(42);
        assert_eq!(physical_state.get_id(), Some(42));
        physical_state.set_id_field(0); // Edge case: zero ID
        assert_eq!(physical_state.get_id(), Some(0));
    }

    #[test]
    fn test_set_and_get_text() {
        let mut physical_state = PhysicalState::default();
        physical_state.set_text_field("Solid");
        assert_eq!(physical_state.get_text(), "Solid");
        physical_state.set_text_field(""); // Edge case: empty string
        assert_eq!(physical_state.get_text(), "");
    }

    #[test]
    fn test_get_text_field_name() {
        let physical_state = PhysicalState::default();
        assert_eq!(physical_state.get_text_field_name(), "physical_state_label");
    }
}
