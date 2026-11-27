#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::borrowing::*;
    use crate::test_helpers::{assert_serde_rejects_missing, assert_serde_round_trip};
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({
            "borrowing_id": 1,
            "borrowing_comment": "back on\nmonday",
            "person": 2,
            "storage": 3,
            "borrower": {
                "person_id": 4,
                "person_email": "borrower@example.com",
                "entities": null,
                "managed_entities": null,
                "permissions": null,
                "is_admin": false
            }
        })
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        let value = assert_serde_round_trip::<Borrowing>(&sample());
        assert_eq!(value.borrower.person_id, Some(4));
    }

    #[test]
    fn test_serde_optional_fields() {
        let mut json = sample();
        for field in ["borrowing_id", "borrowing_comment"] {
            json.as_object_mut().unwrap().remove(field);
        }
        let value: Borrowing = serde_json::from_value(json).unwrap();
        assert_eq!(value.borrowing_id, None);
        assert_eq!(value.borrowing_comment, None);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Borrowing>(&sample(), &["person", "storage", "borrower"]);
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        // The comment is cleaned line by line, keeping its line breaks.
        let mut value = Borrowing {
            borrowing_comment: Some("  back   on\n monday  ".to_string()),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.borrowing_comment.as_deref(), Some("back on\nmonday"));

        // An empty comment becomes None.
        value.borrowing_comment = Some(" \n ".to_string());
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.borrowing_comment, None);
    }
}
