// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **DATA SOURCES MODULE**
//!
//! Data source integrations for NestGate data service.
//!
//! This module implements the concrete data source integrations specified
//! in the NestGate Data Service Specification, including Steam gaming data
//! and NCBI genomic data.

#[cfg(feature = "steam")]
pub mod steam_data_service;

#[cfg(feature = "steam")]
pub use steam_data_service::{
    Achievement, AchievementData, ConflictResolution, FederationNode, GameLibraryStorage,
    GameMetadata, LibraryStats, PlayStats, SaveDataFederation, ServiceHealth, SteamAppId,
    SteamDataProvider, SteamDataService,
};
