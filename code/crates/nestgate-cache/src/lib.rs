// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![forbid(unsafe_code)]

//! Caching subsystem: multi-tier cache, UUID cache, and cache math utilities.

#![warn(missing_docs)]

pub mod cache;
/// Pure functions for cache sizing, eviction thresholds, and hit-ratio math (mutation-test targets).
pub mod cache_math;
/// Lock-free UUID interning and helpers for stable, reusable identifiers.
pub mod uuid_cache;
