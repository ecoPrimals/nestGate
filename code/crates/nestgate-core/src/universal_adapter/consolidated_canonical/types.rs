// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Type definitions for the consolidated canonical adapter
//!
//! This module contains core types for capabilities, requests, responses,
//! and service registration used throughout the adapter system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use super::enums::{
    CapabilityCategory, DataType, RequestPriority, ResponseStatus, ScalabilityRating,
};
use super::health::ResourceRequirements;

// ==================== CAPABILITY TYPES ====================

/// Service capability definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCapability {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Category
    pub category: CapabilityCategory,
    /// Version
    pub version: String,
    /// Provider
    pub provider: String,
    /// Supported data types
    pub supported_data_types: Vec<DataType>,
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
    /// Scalability rating
    pub scalability: ScalabilityRating,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

// ==================== REQUEST/RESPONSE TYPES ====================

/// Capability request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequest {
    /// Unique identifier
    pub id: String,
    /// Capability identifier
    pub capability_id: String,
    /// Method
    pub method: String,
    /// Parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Timeout
    pub timeout: Duration,
    /// Priority
    pub priority: RequestPriority,
    /// Correlation identifier
    pub correlation_id: Option<String>,
    /// Timestamp when this was created
    pub created_at: SystemTime,
}

/// Capability response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityResponse {
    /// Request identifier
    pub request_id: String,
    /// Status
    pub status: ResponseStatus,
    /// Data
    pub data: Option<serde_json::Value>,
    /// Error
    pub error: Option<String>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
    /// Execution time
    pub execution_time: Duration,
    /// Provider
    pub provider: String,
}

// ==================== SERVICE REGISTRATION ====================

/// Service registration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    /// Service identifier
    pub service_id: String,
    /// Capabilities
    pub capabilities: Vec<ServiceCapability>,
    /// Endpoints
    pub endpoints: Vec<String>,
    /// Health status
    pub health_status: String,
    /// Last seen timestamp
    pub last_seen: SystemTime,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
