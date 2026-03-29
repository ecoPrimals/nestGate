// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Configuration, constants, and defaults for the `NestGate` ecosystem.
//!
//! Extracted from nestgate-core to enable parallel compilation.

#![warn(missing_docs)]

#[cfg(test)]
pub use nestgate_platform::env_process;

pub mod canonical_modernization;
pub mod capability_based_config;
pub mod capability_config;
pub mod config;
pub mod constants;
pub mod defaults;
pub mod defaults_v2_config;
pub mod environment;
pub mod environment_config;
pub mod sovereignty_config;

pub use config::NestGateCanonicalConfig as CanonicalConfig;
pub use config::canonical_primary::NestGateCanonicalConfig;
pub use constants::*;
