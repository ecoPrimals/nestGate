// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Custom serialization for `SystemTime`

use serde::de;
use serde::{Deserialize, Deserializer, Serializer};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[allow(clippy::type_complexity)]
/// Function description
///
/// # Errors
///
/// This function will return an error if the operation fails.
pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let duration = time
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0));
    serializer.serialize_u64(duration.as_secs())
}

/// Deserialize a `SystemTime` from Unix timestamp
///
/// # Errors
///
/// Returns an error if deserialization fails or timestamp is invalid
pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<SystemTime, D::Error>
where
    D: Deserializer<'de>,
{
    let secs = u64::deserialize(deserializer)
        .map_err(|_e| de::Error::custom("deserialization error: error details".to_string()))?;
    Ok(UNIX_EPOCH + std::time::Duration::from_secs(secs))
}
