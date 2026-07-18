// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Coordination domain handlers — ecosystem state served from CAS.
//!
//! The coordination backend makes wateringHole artifacts (blurbs, FRAGOs,
//! AARs, wave state, gate heads) queryable over JSON-RPC and HTTP. Data
//! is ingested into nestGate CAS via the rootPulse pipeline (rhizoCrypt
//! DAG → bearDog sign → CAS store → loamSpine commit → sweetGrass braid)
//! and served immutably from content-addressed storage.
//!
//! ## Artifact kinds
//!
//! | Kind    | Source                              |
//! |---------|-------------------------------------|
//! | Blurb   | `wateringHole/handoffs/ECOSYSTEM_BLURB.md` |
//! | FRAGO   | `wateringHole/handoffs/*.md` (AARs, FRAGOs) |
//! | Wave    | `wateringHole/wave.toml`            |
//! | Head    | `heads/<gate>.toml`                 |
//!
//! ## CAS layout
//!
//! All coordination artifacts are stored content-addressed under family
//! `_coordination`:
//! ```text
//! {base}/datasets/{family}/_coordination/manifest.json
//! {base}/datasets/{family}/_coordination/artifacts/{hash}
//! ```
//!
//! The `manifest.json` maps logical artifact names to CAS hashes.

mod ingest;
mod query;
mod types;

pub use ingest::coord_ingest;
pub use query::{
    coord_blurbs_current, coord_blurbs_get, coord_blurbs_list, coord_depot_status,
    coord_fragos_get, coord_fragos_list, coord_heads_all, coord_heads_get, coord_provenance,
    coord_topology, coord_waves_current, coord_waves_history,
};
