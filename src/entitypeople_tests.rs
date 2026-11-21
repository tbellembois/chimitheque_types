#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::entitypeople::Entitypeople;

    /// Deserializes `json` into `T` and serializes it back unchanged.
    fn assert_round_trip<T: serde::Serialize + serde::de::DeserializeOwned>(
        json: &serde_json::Value,
    ) {
        let value: T = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(&serde_json::to_value(&value).unwrap(), json);
    }

    fn sample() -> serde_json::Value {
        serde_json::json!({"entitypeople_entity_id": 1, "entitypeople_person_id": 2, "entitypeople_person_email": "manager@example.com", "entitypeople_entity_name": "lab"})
    }

    #[test]
    fn test_serde_round_trip() {
        assert_round_trip::<Entitypeople>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_entitypeople_entity_id() {
        let mut json = sample();
        json.as_object_mut()
            .unwrap()
            .remove("entitypeople_entity_id");
        assert!(serde_json::from_value::<Entitypeople>(json).is_err());
    }
}
