// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **MESSAGE TYPES**
//!
//! Core MCP message structures and enums.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::capabilities::{
    CapabilityQueryPayload, CapabilityRegistrationPayload, CapabilityResponsePayload,
};
use super::errors::{AcknowledmentPayload, ErrorPayload};
use super::federation::{
    FederationHeartbeatPayload, FederationJoinPayload, FederationLeavePayload,
    FederationSyncPayload,
};
use super::metrics::{MetricsQueryPayload, MetricsReportPayload};
use super::orchestrator::{
    LoadBalancingPayload, OrchestratorRoutePayload, ServiceDiscoveryPayload,
    ServiceRegistrationPayload,
};
use super::services::{HealthCheckPayload, StatusUpdatePayload};
use super::volumes::{
    VolumeCreatePayload, VolumeDeletePayload, VolumeInfoPayload, VolumeListPayload,
    VolumeMountPayload, VolumeUnmountPayload,
};

/// MCP message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpMessage {
    /// Message Type
    pub message_type: String,
    /// Payload
    pub payload: serde_json::Value,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

/// Enhanced MCP Message with advanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique identifier
    pub id: String,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
    /// Source
    pub source: String,
    /// Destination
    pub destination: Option<String>,
    /// Message Type
    pub message_type: MessageType,
    /// Payload
    pub payload: MessagePayload,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

impl Message {
    /// Create new message
    #[must_use]
    pub fn new(message_type: MessageType, payload: MessagePayload) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: std::time::SystemTime::now(),
            source: "nestgate-v2".to_string(),
            destination: None,
            message_type,
            payload,
            metadata: HashMap::new(),
        }
    }

    /// Set destination
    #[must_use]
    pub fn with_destination(mut self, destination: String) -> Self {
        self.destination = Some(destination);
        self
    }

    /// Add metadata
    #[must_use]
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Enhanced Message Types with advanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    // Capability messages
    CapabilityRegistration,
    CapabilityQuery,
    CapabilityResponse,

    // Storage operations
    VolumeCreate,
    VolumeDelete,
    VolumeMount,
    VolumeUnmount,
    VolumeList,
    VolumeInfo,

    // Performance and monitoring
    MetricsReport,
    MetricsQuery,
    HealthCheck,
    StatusUpdate,

    // Federation and clustering
    FederationJoin,
    FederationLeave,
    FederationSync,
    FederationHeartbeat,

    // Orchestrator v2 specific
    OrchestratorRoute,
    ServiceRegistration,
    ServiceDiscovery,
    LoadBalancing,

    // Error handling
    Error,
    Acknowledgment,
}

/// Enhanced Message Payload with advanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePayload {
    // Capability payloads
    CapabilityRegistration(CapabilityRegistrationPayload),
    CapabilityQuery(CapabilityQueryPayload),
    CapabilityResponse(CapabilityResponsePayload),

    // Storage operation payloads
    VolumeCreate(VolumeCreatePayload),
    VolumeDelete(VolumeDeletePayload),
    VolumeMount(VolumeMountPayload),
    VolumeUnmount(VolumeUnmountPayload),
    VolumeList(VolumeListPayload),
    VolumeInfo(VolumeInfoPayload),

    // Performance and monitoring payloads
    MetricsReport(MetricsReportPayload),
    MetricsQuery(MetricsQueryPayload),
    HealthCheck(HealthCheckPayload),
    StatusUpdate(StatusUpdatePayload),

    // Federation payloads
    FederationJoin(FederationJoinPayload),
    FederationLeave(FederationLeavePayload),
    FederationSync(FederationSyncPayload),
    FederationHeartbeat(FederationHeartbeatPayload),

    // Orchestrator v2 payloads
    OrchestratorRoute(OrchestratorRoutePayload),
    ServiceRegistration(ServiceRegistrationPayload),
    ServiceDiscovery(ServiceDiscoveryPayload),
    LoadBalancing(LoadBalancingPayload),

    // Error handling payloads
    Error(ErrorPayload),
    Acknowledgment(AcknowledmentPayload),
}
