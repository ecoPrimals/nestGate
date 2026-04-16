// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Federation-related configuration type aliases.
//!
//! Detailed configuration lives under [`crate::config::canonical_primary`].

// ==================== CANONICAL TYPE ALIAS ====================

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// Type alias for Mcpconfigcanonical
pub type McpConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// ==================== CANONICAL TYPE ALIAS ====================

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// Type alias for Federationconfigcanonical
pub type FederationConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// ==================== CANONICAL TYPE ALIAS ====================

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// Type alias for Mcpcapabilitiesconfigcanonical
pub type McpCapabilitiesConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
