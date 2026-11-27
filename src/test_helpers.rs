//! Assertions shared by the unit tests of the types.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chimitheque_traits::searchable::Searchable;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::error::ValidationError;

/// Deserializes `json` into `T`, checks that serializing it back gives the
/// same JSON and returns the deserialized value.
pub(crate) fn assert_serde_round_trip<T: Serialize + DeserializeOwned>(json: &Value) -> T {
    let value: T = serde_json::from_value(json.clone()).unwrap();
    assert_eq!(&serde_json::to_value(&value).unwrap(), json);
    value
}

/// Serializes `value`, deserializes the JSON and checks that serializing it
/// again gives the same JSON. Returns the JSON of `value`.
///
/// Used for types too nested to write their whole JSON in a test.
pub(crate) fn assert_value_round_trip<T: Serialize + DeserializeOwned>(value: &T) -> Value {
    let json = serde_json::to_value(value).unwrap();
    assert_serde_round_trip::<T>(&json);
    json
}

/// Checks that `json` can not be deserialized into `T` when any one of the
/// `required` fields is removed.
pub(crate) fn assert_serde_rejects_missing<T: DeserializeOwned>(json: &Value, required: &[&str]) {
    for field in required {
        let mut missing = json.clone();
        assert!(
            missing.as_object_mut().unwrap().remove(*field).is_some(),
            "{field} is not in the JSON"
        );
        assert!(
            serde_json::from_value::<T>(missing).is_err(),
            "accepted without {field}"
        );
    }
}

/// Checks that `result` is an error for `field` and returns the error, so the
/// caller can check its variant.
pub(crate) fn assert_validation_error(
    result: Result<(), ValidationError>,
    field: &str,
) -> ValidationError {
    let error = result.unwrap_err();
    assert_eq!(error.field(), field, "{error:?}");
    error
}

/// Checks the `Searchable` implementation of `T` against its table, id field
/// and text field names.
pub(crate) fn assert_searchable<T: Searchable + Default>(
    table: &str,
    id_field: &str,
    text_field: &str,
) {
    let mut item = T::default();
    assert_eq!(item.get_table_name(), table);
    assert_eq!(item.get_id_field_name(), id_field);
    assert_eq!(item.get_text_field_name(), text_field);

    assert!(!item.get_exact_search());
    item.set_exact_search(true);
    assert!(item.get_exact_search());
    item.set_exact_search(false);
    assert!(!item.get_exact_search());

    assert_eq!(item.get_id(), None);
    item.set_id_field(42);
    assert_eq!(item.get_id(), Some(42));

    assert_eq!(item.get_text(), "");
    item.set_text_field("text");
    assert_eq!(item.get_text(), "text");

    // `create` returns an empty item, whatever the values of `item`.
    item.set_exact_search(true);
    let created = item.create();
    assert!(!created.get_exact_search());
    assert_eq!(created.get_id(), None);
    assert_eq!(created.get_text(), "");
}
