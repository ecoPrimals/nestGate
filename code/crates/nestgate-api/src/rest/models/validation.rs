// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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
