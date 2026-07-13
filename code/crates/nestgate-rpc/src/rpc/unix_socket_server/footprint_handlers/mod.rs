// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! footPrint domain handlers — CAS-backed project persistence.
//!
//! Replaces Express CRUD (`/api/projects`) with content-addressed storage.
//! Each project save creates an immutable revision in `_content/` (BLAKE3),
//! while a manifest at `_footprint/manifest.json` indexes projects and their
//! revision chains.
//!
//! ## CAS layout
//!
//! ```text
//! {base}/datasets/{family}/_footprint/manifest.json
//! {base}/datasets/{family}/_content/{hex[0:2]}/{hex}         # revision blobs
//! {base}/datasets/{family}/_content/{hex[0:2]}/{hex}.meta.json  # sidecars
//! ```
//!
//! Revision content is stored in the shared `_content/` CAS directory, giving
//! footPrint projects automatic deduplication, federation via `content.replicate`,
//! HTTP serving via `GET /content/:hash`, and provenance sidecars.
//!
//! ## Methods
//!
//! | Method | Description |
//! |--------|-------------|
//! | `footprint.save` | Create/update project with new CAS revision |
//! | `footprint.get` | Retrieve project metadata + optional content |
//! | `footprint.list` | List all projects (paginated) |
//! | `footprint.delete` | Soft-delete project from manifest (CAS content remains) |
//! | `footprint.history` | List revision history for a project |

mod ingest;
mod query;
mod types;

pub use ingest::{footprint_delete, footprint_save};
pub use query::{footprint_get, footprint_history, footprint_list};
