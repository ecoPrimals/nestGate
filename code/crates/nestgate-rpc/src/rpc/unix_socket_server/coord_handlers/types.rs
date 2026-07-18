// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Coordination domain types — artifact metadata, manifest, and kind taxonomy.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Classification of coordination artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    Blurb,
    Frago,
    Aar,
    Wave,
    Head,
}

impl ArtifactKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Blurb => "blurb",
            Self::Frago => "frago",
            Self::Aar => "aar",
            Self::Wave => "wave",
            Self::Head => "head",
        }
    }

    pub fn from_filename(name: &str) -> Self {
        let lower = name.to_ascii_lowercase();
        if lower.contains("blurb") {
            Self::Blurb
        } else if lower.contains("frago") {
            Self::Frago
        } else if lower.contains("aar") || lower.contains("postmortem") {
            Self::Aar
        } else if lower.contains("wave") {
            Self::Wave
        } else if std::path::Path::new(name)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("toml"))
        {
            Self::Head
        } else {
            Self::Frago
        }
    }
}

/// A single coordination artifact stored in CAS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordArtifact {
    pub hash: String,
    pub kind: ArtifactKind,
    pub title: String,
    pub wave: Option<String>,
    pub gate: Option<String>,
    pub ingested_at: String,
    /// BLAKE3 hash of the raw content in CAS.
    pub content_hash: String,
    /// Optional loamSpine entry index for provenance history.
    pub spine_index: Option<u64>,
    /// Optional sweetGrass braid ID for attribution.
    pub braid_id: Option<String>,
}

/// The coordination manifest — maps logical names to artifact metadata.
///
/// Stored as `_coordination/manifest.json` under the family dataset.
/// Updated atomically on each `coord.ingest` call.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CoordManifest {
    pub version: u32,
    pub updated_at: String,
    /// Current blurb hash (most recent ingested blurb).
    pub current_blurb: Option<String>,
    /// Current wave state hash.
    pub current_wave: Option<String>,
    /// All known artifacts indexed by hash.
    pub artifacts: BTreeMap<String, CoordArtifact>,
    /// Gate HEAD hashes indexed by gate name.
    pub heads: BTreeMap<String, String>,
    /// Blurb hashes ordered by wave (newest first).
    pub blurb_history: Vec<String>,
    /// FRAGO/AAR hashes ordered by ingest time (newest first).
    pub frago_history: Vec<String>,
}

impl CoordManifest {
    pub fn new() -> Self {
        Self {
            version: 1,
            updated_at: chrono::Utc::now().to_rfc3339(),
            ..Default::default()
        }
    }
}
