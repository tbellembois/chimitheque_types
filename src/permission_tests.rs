#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::permission::*;

    #[test]
    fn test_permission_name_from_str() {
        let test_cases = vec![
            ("n", PermissionName::None),
            ("r", PermissionName::Read),
            ("w", PermissionName::Write),
            ("b", PermissionName::Borrow),
            ("all", PermissionName::All),
        ];

        for (input, expected) in test_cases {
            assert_eq!(expected, input.parse().unwrap());
        }
    }

    #[test]
    fn test_permission_name_from_str_invalid() {
        let invalid_inputs = vec![
            "invalid", "123", "", "a", "read", "write", "borrow", "none", "All", "Read", "Write",
            "Borrow", "All",
        ];

        for input in invalid_inputs {
            assert!(input.parse::<PermissionName>().is_err());
        }
    }

    #[test]
    fn test_permission_item_from_str() {
        let test_cases = vec![
            ("all", PermissionItem::All),
            ("entities", PermissionItem::Entities),
            ("products", PermissionItem::Products),
            ("rproducts", PermissionItem::RestrictedProducts),
            ("storages", PermissionItem::Storages),
        ];

        for (input, expected) in test_cases {
            assert_eq!(expected, input.parse().unwrap());
        }
    }

    #[test]
    fn test_permission_item_from_str_invalid() {
        let invalid_inputs = vec![
            "invalid",
            "123",
            "",
            "a",
            "allitems",
            "entity",
            "product",
            "storagess",
            "rproduct",
        ];

        for input in invalid_inputs {
            assert!(input.parse::<PermissionItem>().is_err());
        }
    }
}
