#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::personentities::Personentities;

    /// Deserializes `json` into `T` and serializes it back unchanged.
    fn assert_round_trip<T: serde::Serialize + serde::de::DeserializeOwned>(
        json: &serde_json::Value,
    ) {
        let value: T = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(&serde_json::to_value(&value).unwrap(), json);
    }

    fn sample() -> serde_json::Value {
        serde_json::json!({"personentities_person_id": 2, "personentities_entity_id": 1, "personentities_person_email": "chemist@example.com", "personentities_entity_name": "lab"})
    }

    #[test]
    fn test_serde_round_trip() {
        assert_round_trip::<Personentities>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_personentities_person_id() {
        let mut json = sample();
        json.as_object_mut()
            .unwrap()
            .remove("personentities_person_id");
        assert!(serde_json::from_value::<Personentities>(json).is_err());
    }
}
