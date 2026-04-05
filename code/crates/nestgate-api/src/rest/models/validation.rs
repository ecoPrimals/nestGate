// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Validation utilities and custom deserializers for REST API models

use serde::{Deserialize, Deserializer};

/// Validates that a dataset name follows ZFS naming conventions
pub fn validate_dataset_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let name = String::deserialize(deserializer)?;

    // Check length constraints
    if name.is_empty() || name.len() > 255 {
        return Err(serde::de::Error::custom(
            "Dataset name must be between 1 and 255 characters",
        ));
    }

    // Check that it doesn't end with a slash
    if name.ends_with('/') {
        return Err(serde::de::Error::custom(
            "Dataset name cannot end with a forward slash",
        ));
    }

    // Check for invalid characters
    if name.chars().any(|c| matches!(c, '\0' | '\n' | '\t')) {
        return Err(serde::de::Error::custom(
            "Dataset name contains invalid characters",
        ));
    }

    Ok(name)
}

/// Validates that a snapshot name follows ZFS snapshot naming conventions
pub fn validate_snapshot_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let name = String::deserialize(deserializer)?;

    // Check length constraints
    if name.is_empty() || name.len() > 255 {
        return Err(serde::de::Error::custom(
            "Snapshot name must be between 1 and 255 characters",
        ));
    }

    // Check for invalid characters in snapshot names
    if name
        .chars()
        .any(|c| matches!(c, '\0' | '\n' | '\t' | '@' | '#'))
    {
        return Err(serde::de::Error::custom(
            "Snapshot name contains invalid characters",
        ));
    }

    // Snapshot names cannot start with a dash
    if name.starts_with('-') {
        return Err(serde::de::Error::custom(
            "Snapshot name cannot start with a dash",
        ));
    }

    Ok(name)
}

/// Validates that a floating point number is positive
pub fn validate_positive_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = f64::deserialize(deserializer)?;

    if value <= 0.0 {
        return Err(serde::de::Error::custom(
            "Value must be positive (greater than 0)",
        ));
    }

    if !value.is_finite() {
        return Err(serde::de::Error::custom(
            "Value must be a finite number (not NaN or infinity)",
        ));
    }

    Ok(value)
}

/// Validates that an unsigned integer is positive (greater than 0)
pub fn validate_positive_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = u64::deserialize(deserializer)?;

    if value == 0 {
        return Err(serde::de::Error::custom(
            "Value must be positive (greater than 0)",
        ));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct DatasetNameField {
        #[serde(deserialize_with = "super::validate_dataset_name")]
        name: String,
    }

    #[derive(Debug, Deserialize)]
    struct SnapshotNameField {
        #[serde(deserialize_with = "super::validate_snapshot_name")]
        name: String,
    }

    #[derive(Debug, Deserialize)]
    struct PositiveF64Field {
        #[serde(deserialize_with = "super::validate_positive_f64")]
        value: f64,
    }

    #[derive(Debug, Deserialize)]
    struct PositiveU64Field {
        #[serde(deserialize_with = "super::validate_positive_u64")]
        value: u64,
    }

    #[test]
    fn dataset_name_roundtrip_ok() {
        let v: DatasetNameField = serde_json::from_str(r#"{"name":"tank/data"}"#).unwrap();
        assert_eq!(v.name, "tank/data");
    }

    #[test]
    fn dataset_name_rejects_empty() {
        let err = serde_json::from_str::<DatasetNameField>(r#"{"name":""}"#).unwrap_err();
        assert!(err.to_string().contains("1 and 255"));
    }

    #[test]
    fn dataset_name_rejects_trailing_slash() {
        let err = serde_json::from_str::<DatasetNameField>(r#"{"name":"bad/"}"#).unwrap_err();
        assert!(err.to_string().contains("forward slash"));
    }

    #[test]
    fn dataset_name_rejects_nul() {
        let err = serde_json::from_str::<DatasetNameField>(r#"{"name":"a\u0000b"}"#).unwrap_err();
        assert!(err.to_string().contains("invalid characters"));
    }

    #[test]
    fn snapshot_name_rejects_at_sign() {
        let err = serde_json::from_str::<SnapshotNameField>(r#"{"name":"snap@1"}"#).unwrap_err();
        assert!(err.to_string().contains("invalid characters"));
    }

    #[test]
    fn snapshot_name_rejects_leading_dash() {
        let err = serde_json::from_str::<SnapshotNameField>(r#"{"name":"-bad"}"#).unwrap_err();
        assert!(err.to_string().contains("dash"));
    }

    #[test]
    fn positive_f64_ok() {
        let v: PositiveF64Field = serde_json::from_str(r#"{"value":1.5}"#).unwrap();
        assert!((v.value - 1.5).abs() < f64::EPSILON);
    }

    #[test]
    fn positive_f64_rejects_non_positive() {
        let err = serde_json::from_str::<PositiveF64Field>(r#"{"value":0}"#).unwrap_err();
        assert!(err.to_string().contains("positive"));
    }

    #[test]
    fn positive_f64_rejects_non_finite() {
        // JSON overflow or IEEE non-finite: serde_json may error at parse or our validator rejects inf.
        assert!(serde_json::from_str::<PositiveF64Field>(r#"{"value":1e400}"#).is_err());
    }

    #[test]
    fn positive_u64_ok() {
        let v: PositiveU64Field = serde_json::from_str(r#"{"value":42}"#).unwrap();
        assert_eq!(v.value, 42);
    }

    #[test]
    fn positive_u64_rejects_zero() {
        let err = serde_json::from_str::<PositiveU64Field>(r#"{"value":0}"#).unwrap_err();
        assert!(err.to_string().contains("positive"));
    }
}
