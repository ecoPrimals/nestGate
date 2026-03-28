// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **DATA SOURCES MODULE**
//!
//! Data source integrations for NestGate data service.
//!
//! This module implements the concrete data source integrations specified
//! in the NestGate Data Service Specification, including Steam gaming data
//! and NCBI genomic data.

pub mod steam_data_service;

// Re-export main types for convenience
pub use steam_data_service::{
    Achievement, AchievementData, ConflictResolution, FederationNode, GameLibraryStorage,
    GameMetadata, LibraryStats, PlayStats, SaveDataFederation, ServiceHealth, SteamAppId,
    SteamDataProvider, SteamDataService,
};
