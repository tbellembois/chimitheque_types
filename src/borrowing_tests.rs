#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::borrowing::*;

    #[test]
    fn test_sanitize_and_validate_trims_comment() {
        let mut value = Borrowing {
            borrowing_comment: Some("  back on\nmonday  ".to_string()),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.borrowing_comment.as_deref(), Some("back on\nmonday"));

        value.borrowing_comment = Some(" \n ".to_string());
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.borrowing_comment, None);
    }
}
