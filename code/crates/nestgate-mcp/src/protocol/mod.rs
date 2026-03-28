// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **MCP PROTOCOL MODULE**
//!
//! Advanced MCP protocol handling integrating enhanced NestGate capabilities
//! with v2 orchestrator-centric architecture.
//!
//! ## Module Structure
//!
//! This module is organized by domain concern:
//! - **messages**: Core message types and enums
//! - **responses**: Response types and status
//! - **session**: Session and client information
//! - **services**: Service and health monitoring
//! - **federation**: Federation and cluster coordination
//! - **capabilities**: Capability registration and discovery
//! - **volumes**: Volume operation types
//! - **metrics**: Performance metrics and monitoring
//! - **orchestrator**: Orchestrator routing and service discovery
//! - **errors**: Error handling and acknowledgments
//! - **handler**: Protocol message handler implementation
//!
//! ## Evolution History
//!
//! - **January 2026**: Smart refactoring from monolithic 946-line file
//!   - Organized by domain concern (not mechanical splitting)
//!   - Each module is self-contained and focused
//!   - Maintains all functionality and tests

// Canonical error types
pub use nestgate_core::error::Result;

// Core protocol modules
pub mod capabilities;
pub mod errors;
pub mod federation;
pub mod handler;
pub mod messages;
pub mod metrics;
pub mod orchestrator;
pub mod responses;
pub mod services;
pub mod session;
pub mod volumes;

// Re-export commonly used types
pub use capabilities::{
    CapabilityQueryPayload, CapabilityQueryType, CapabilityRegistrationPayload,
    CapabilityResponsePayload,
};
pub use errors::{AcknowledmentPayload, AcknowledmentType, ErrorPayload, McpProtocolError};
pub use federation::{
    FederationHeartbeatPayload, FederationJoinPayload, FederationLeavePayload, FederationStatus,
    FederationSyncPayload, FederationSyncType,
};
pub use handler::ProtocolHandler;
pub use messages::{McpMessage, Message, MessagePayload, MessageType};
pub use metrics::{MetricType, MetricsQueryPayload, MetricsReportPayload, TimeRange};
pub use orchestrator::{
    LoadBalancingAlgorithm, LoadBalancingInfo, LoadBalancingPayload, OrchestratorRoutePayload,
    RouteType, ServiceDiscoveryPayload, ServiceRegistrationPayload,
};
pub use responses::{Response, ResponsePayload, ResponseStatus};
pub use services::{
    ClusterHealth, HealthCheckPayload, HealthCheckType, HealthStatus, NodeRole, ServiceInfo,
    ServiceStatus, StatusUpdatePayload,
};
pub use session::{ClientInfo, McpSession, ServerCapabilities};
pub use volumes::{
    VolumeCreatePayload, VolumeDeletePayload, VolumeInfoPayload, VolumeListPayload,
    VolumeMountPayload, VolumeUnmountPayload,
};
