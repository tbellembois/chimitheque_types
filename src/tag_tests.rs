#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::tag::*;

    #[test]
    fn test_tag_searchable() {
        let tag = Tag {
            match_exact_search: true,
            tag_id: Some(1),
            tag_label: "test".to_string(),
        };

        let mut created_tag = tag.create();
        assert!(!created_tag.match_exact_search);
        assert_eq!(created_tag.tag_id, None);
        assert_eq!(created_tag.tag_label, String::new());

        created_tag.set_exact_search(true);
        assert!(created_tag.get_exact_search());

        assert_eq!(created_tag.get_table_name(), "tag");
        assert_eq!(created_tag.get_id_field_name(), "tag_id");
        assert_eq!(created_tag.get_text_field_name(), "tag_label");

        created_tag.set_id_field(2);
        assert_eq!(created_tag.get_id(), Some(2));

        created_tag.set_text_field("test2");
        assert_eq!(created_tag.get_text(), "test2");
    }

    #[test]
    fn test_sanitize_and_validate_tag() {
        let mut tag = Tag {
            match_exact_search: false,
            tag_id: Some(1),
            tag_label: "  tag1  ".to_string(),
        };
        assert!(tag.sanitize_and_validate().is_ok());
        assert_eq!(tag.tag_label, "tag1");

        let mut tag = Tag {
            match_exact_search: false,
            tag_id: Some(2),
            tag_label: "  tag  tag  ".to_string(),
        };
        assert!(tag.sanitize_and_validate().is_ok());
        assert_eq!(tag.tag_label, "A B");

        let mut tag = Tag {
            match_exact_search: false,
            tag_id: Some(3),
            tag_label: "   A  B   ".to_string(),
        };
        assert!(tag.sanitize_and_validate().is_ok());
        assert_eq!(tag.tag_label, "A B");
    }

    #[test]
    fn test_sanitize_and_validate_empty_tag() {
        let mut tag = Tag {
            match_exact_search: false,
            tag_id: Some(6),
            tag_label: String::default(),
        };
        assert!(tag.sanitize_and_validate().is_err());
    }
}
