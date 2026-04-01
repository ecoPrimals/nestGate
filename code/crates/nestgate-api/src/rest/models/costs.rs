// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Cost estimation and analysis models

use serde::{Deserialize, Serialize};

// Cost estimation and financial analysis structures

/// Cost estimation for storage and infrastructure solutions
///
/// Provides detailed cost breakdown including setup, recurring monthly costs,
/// and per-GB pricing analysis for comprehensive financial planning.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Costestimate
pub struct CostEstimate {
    /// Total estimated cost including setup and first year operation
    pub total_cost: f64,
    /// One-time setup and installation costs
    pub setup_cost: f64,
    /// Recurring monthly operational costs
    pub monthly_cost: f64,
    /// Cost per gigabyte per month for capacity planning
    pub cost_per_gb_monthly: f64,
    /// Detailed cost breakdown by category (hardware, software, maintenance, etc.)
    pub breakdown: std::collections::HashMap<String, f64>,
}
