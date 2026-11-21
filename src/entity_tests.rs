#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::entity::*;

    #[test]
    fn test_sanitize_and_validate_cleans_name_and_description() {
        let mut entity = Entity {
            entity_name: "  lab   one  ".to_string(),
            entity_description: Some("  first   floor  ".to_string()),
            ..Default::default()
        };
        assert!(entity.sanitize_and_validate().is_ok());
        assert_eq!(entity.entity_name, "lab one");
        assert_eq!(entity.entity_description.as_deref(), Some("first floor"));
    }

    #[test]
    fn test_sanitize_and_validate_without_description() {
        let mut entity = Entity {
            entity_name: "lab".to_string(),
            entity_description: None,
            ..Default::default()
        };
        assert!(entity.sanitize_and_validate().is_ok());
        assert_eq!(entity.entity_description, None);
    }

    #[test]
    fn test_sanitize_and_validate_empty_name() {
        for name in ["", "   "] {
            let mut entity = Entity {
                entity_name: name.to_string(),
                entity_description: Some("description".to_string()),
                ..Default::default()
            };
            let error = entity.sanitize_and_validate().unwrap_err();
            assert!(matches!(
                error.downcast_ref::<ParseError>(),
                Some(ParseError::EmptyInput)
            ));
        }
    }
}
