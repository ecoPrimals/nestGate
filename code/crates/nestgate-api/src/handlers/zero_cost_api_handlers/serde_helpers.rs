// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Serde helpers for `Arc`-wrapped fields in zero-cost API types.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::sync::Arc;

/// **SERDE HELPERS FOR ARC TYPES**
/// Zero-copy serialization for Arc-wrapped types
pub mod arc_string_serde {
    use super::{Arc, Deserialize, Deserializer, Serialize, Serializer};
    /// Serialize
    pub fn serialize<S>(value: &Arc<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.as_str().serialize(serializer)
    }

    /// Deserialize
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Arc<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Arc::new)
    }
}

pub mod arc_hashmap_serde {
    use super::{Arc, Deserialize, Deserializer, HashMap, Serialize, Serializer};
    /// Serialize
    pub fn serialize<S>(
        value: &Arc<HashMap<String, String>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.as_ref().serialize(serializer)
    }

    /// Deserialize
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Arc<HashMap<String, String>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        HashMap::deserialize(deserializer).map(Arc::new)
    }
}
