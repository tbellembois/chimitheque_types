#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

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

    #[test]
    fn test_match_exact_search_serde_defaults() -> Result<(), serde_json::Error> {
        let mut payload = serde_json::json!({"physical_state_label": "Liquid"});
        let value: PhysicalState = serde_json::from_value(payload.clone())?;
        assert!(!value.match_exact_search);
        assert_eq!(
            serde_json::to_value(&value)?.get("match_exact_search"),
            Some(&serde_json::json!(false))
        );

        for expected in [true, false] {
            payload["match_exact_search"] = serde_json::json!(expected);
            let value: PhysicalState = serde_json::from_value(payload.clone())?;
            assert_eq!(value.match_exact_search, expected);
            assert_eq!(
                serde_json::to_value(&value)?.get("match_exact_search"),
                Some(&serde_json::json!(expected))
            );
        }

        let mut missing_required = payload.clone();
        if let Some(object) = missing_required.as_object_mut() {
            object.remove("physical_state_label");
        }
        assert!(serde_json::from_value::<PhysicalState>(missing_required).is_err());
        Ok(())
    }

    fn assert_empty(result: Result<(), crate::error::ValidationError>, expected_field: &str) {
        let error = result.unwrap_err();
        assert!(
            matches!(error, crate::error::ValidationError::Empty { field } if field == expected_field),
            "{error:?}"
        );
    }

    #[test]
    fn test_sanitize_and_validate_label() {
        let mut value = PhysicalState {
            physical_state_label: "  liquid  ".to_string(),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.physical_state_label, "liquid");

        for empty in ["", "   "] {
            let mut value = PhysicalState {
                physical_state_label: empty.to_string(),
                ..Default::default()
            };
            assert_empty(value.sanitize_and_validate(), "physical_state_label");
        }
    }
}
