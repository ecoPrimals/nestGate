// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! footPrint project types — CAS-backed project persistence with revision history.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A single project revision stored in CAS.
///
/// Each save creates an immutable revision. The project's `current_revision`
/// points to the latest, while `revision_history` preserves the full chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRevision {
    /// BLAKE3 hash of the revision content.
    pub hash: String,
    /// Human-readable commit message for this revision.
    pub message: String,
    /// RFC 3339 timestamp of when this revision was saved.
    pub saved_at: String,
    /// BLAKE3 hash of the parent revision (if any).
    pub parent: Option<String>,
    /// Byte size of the revision content.
    pub size: u64,
    /// Optional provenance — ledger entry index.
    pub spine_index: Option<u64>,
    /// Optional provenance — attribution braid ID.
    pub braid_id: Option<String>,
}

/// A footPrint project — metadata + pointer to current revision in CAS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootPrintProject {
    /// Unique project identifier (slug-style, e.g. `my-portfolio`).
    pub project_id: String,
    /// Human-readable display name.
    pub name: String,
    /// RFC 3339 timestamp of project creation.
    pub created_at: String,
    /// RFC 3339 timestamp of last modification.
    pub updated_at: String,
    /// BLAKE3 hash of the current (latest) revision.
    pub current_revision: Option<String>,
    /// Ordered list of revision hashes (newest first).
    pub revision_history: Vec<String>,
    /// Revision metadata indexed by hash.
    pub revisions: BTreeMap<String, ProjectRevision>,
    /// Arbitrary user metadata (tags, description, etc.).
    pub metadata: serde_json::Value,
}

impl FootPrintProject {
    /// Create a new empty project with the given ID and name.
    pub fn new(project_id: String, name: String, metadata: serde_json::Value) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            project_id,
            name,
            created_at: now.clone(),
            updated_at: now,
            current_revision: None,
            revision_history: Vec::new(),
            revisions: BTreeMap::new(),
            metadata,
        }
    }
}

/// The footPrint manifest — indexes all projects by ID.
///
/// Stored as `_footprint/manifest.json` under the family dataset.
/// Updated atomically on each `footprint.save` / `footprint.delete` call.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FootPrintManifest {
    /// Monotonically increasing version number.
    pub version: u32,
    /// RFC 3339 timestamp of last manifest update.
    pub updated_at: String,
    /// Total number of projects.
    pub project_count: u32,
    /// Projects indexed by project ID.
    pub projects: BTreeMap<String, FootPrintProject>,
}

impl FootPrintManifest {
    pub fn new() -> Self {
        Self {
            version: 1,
            updated_at: chrono::Utc::now().to_rfc3339(),
            project_count: 0,
            projects: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_project_has_no_revisions() {
        let p = FootPrintProject::new(
            "test-proj".into(),
            "Test Project".into(),
            serde_json::json!({}),
        );
        assert_eq!(p.project_id, "test-proj");
        assert_eq!(p.name, "Test Project");
        assert!(p.current_revision.is_none());
        assert!(p.revision_history.is_empty());
        assert!(p.revisions.is_empty());
    }

    #[test]
    fn new_manifest_starts_empty() {
        let m = FootPrintManifest::new();
        assert_eq!(m.version, 1);
        assert_eq!(m.project_count, 0);
        assert!(m.projects.is_empty());
    }

    #[test]
    fn project_serialization_roundtrip() {
        let p = FootPrintProject::new(
            "round-trip".into(),
            "Round Trip".into(),
            serde_json::json!({"tags": ["test"]}),
        );
        let json = serde_json::to_string(&p).expect("serialize");
        let back: FootPrintProject = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.project_id, "round-trip");
        assert_eq!(back.metadata["tags"][0], "test");
    }

    #[test]
    fn manifest_serialization_roundtrip() {
        let mut m = FootPrintManifest::new();
        m.projects.insert(
            "p1".into(),
            FootPrintProject::new("p1".into(), "Project One".into(), serde_json::json!({})),
        );
        m.project_count = 1;
        let json = serde_json::to_string(&m).expect("serialize");
        let back: FootPrintManifest = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.project_count, 1);
        assert!(back.projects.contains_key("p1"));
    }
}
