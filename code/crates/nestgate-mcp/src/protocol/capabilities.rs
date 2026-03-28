// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **CAPABILITY TYPES**
//!
//! Capability registration, query, and response types.

use crate::types::ProviderCapabilities;
use serde::{Deserialize, Serialize};

/// Capability Registration Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRegistrationPayload {
    /// Node ID
    pub node_id: String,
    /// Capabilities
    pub capabilities: ProviderCapabilities,
}

/// Capability Query Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityQueryPayload {
    /// Query type
    pub query_type: CapabilityQueryType,
}

/// Capability Query Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapabilityQueryType {
    All,
    ByNodeId(String),
    ByType(String),
}

/// Capability Response Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityResponsePayload {
    /// Capabilities
    pub capabilities: Vec<ProviderCapabilities>,
}
