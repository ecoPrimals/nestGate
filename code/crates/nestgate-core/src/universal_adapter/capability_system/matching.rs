// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Heuristics for choosing among multiple discovered capability providers.

use super::types::DiscoveredService;

/// Prefer the service that was seen most recently (lowest elapsed time since `last_seen`).
#[must_use]
pub fn select_best_by_recency(services: &[DiscoveredService]) -> Option<&DiscoveredService> {
    services
        .iter()
        .min_by_key(|service| service.last_seen.elapsed().unwrap_or_default().as_millis())
}
