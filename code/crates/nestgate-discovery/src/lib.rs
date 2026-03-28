// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Primal discovery, capability resolution, and service registry for NestGate.
//!
//! Extracted from nestgate-core to enable parallel compilation.

#![warn(missing_docs)]

pub mod capabilities;
pub mod capability_discovery;
pub mod capability_resolver;
pub mod discovery;
pub mod discovery_mechanism;
pub mod infant_discovery;
pub mod primal_discovery;
pub mod primal_self_knowledge;
pub mod self_knowledge;
pub mod service_discovery;
pub mod unified_capabilities;
pub mod universal_primal_discovery;
