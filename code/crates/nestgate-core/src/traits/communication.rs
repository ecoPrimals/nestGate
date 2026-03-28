// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Communication Layer Trait for Service-to-Service Messaging
//!
//! **MIGRATED FROM**: `traits::communication` (November 7, 2025)
//! **CANONICAL**: This is the single source of truth for inter-service communication
//! **STATUS**: Production-ready, native async

use crate::Result;
use chrono::{DateTime, Utc};
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Communication layer trait for service-to-service messaging
///
/// This trait provides the interface for sending messages between services,
/// broadcasting events, and subscribing to topics in the NestGate ecosystem.
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::traits::{CommunicationLayer, ServiceAddress, ServiceMessage};
///
/// async fn send_example(comm: &impl CommunicationLayer) -> nestgate_core::Result<()> {
///     let target = ServiceAddress {
///         service_id: "my-service".to_string(),
///         instance_id: None,
///         endpoint: None,
///     };
///     
///     let message = ServiceMessage {
///         id: "msg-123".to_string(),
///         message_type: MessageType::Request,
///         topic: None,
///         payload: serde_json::json!({"data": "hello"}),
///         headers: HashMap::new(),
///         timestamp: chrono::Utc::now(),
///         correlation_id: None,
///         reply_to: None,
///         ttl: None,
///     };
///     
///     comm.send_message(target, message).await?;
///     Ok(())
/// }
/// ```
pub trait CommunicationLayer: Send + Sync {
    /// Send a message to a specific service
    fn send_message(
        &self,
        target: ServiceAddress,
        message: ServiceMessage,
    ) -> impl std::future::Future<Output = Result<CommunicationResponse>> + Send;

    /// Broadcast a message to all services
    fn broadcast(
        &self,
        message: ServiceMessage,
    ) -> impl std::future::Future<Output = Result<Vec<CommunicationResponse>>> + Send;

    /// Listen for incoming messages
    fn listen(
        &self,
    ) -> impl std::future::Future<Output = impl Stream<Item = (ServiceAddress, ServiceMessage)>> + Send;

    /// Subscribe to a topic
    fn subscribe(&self, topic: &str) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Unsubscribe from a topic
    fn unsubscribe(&self, topic: &str) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Connect to the communication layer
    fn connect(&self) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Disconnect from the communication layer
    fn disconnect(&self) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Check if connected
    fn is_connected(&self) -> impl std::future::Future<Output = bool> + Send;

    /// Get communication statistics
    fn get_stats(&self) -> impl std::future::Future<Output = Result<CommunicationStats>> + Send;
}

/// Service address for routing messages
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Serviceaddress
pub struct ServiceAddress {
    /// Service identifier
    pub service_id: String,
    /// Instance identifier
    pub instance_id: Option<String>,
    /// Endpoint
    pub endpoint: Option<String>,
}

/// Message between services
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicemessage
pub struct ServiceMessage {
    /// Unique identifier
    pub id: String,
    /// Message Type
    pub message_type: MessageType,
    /// Topic
    pub topic: Option<String>,
    /// Payload
    pub payload: serde_json::Value,
    /// Headers
    pub headers: HashMap<String, String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Correlation identifier
    pub correlation_id: Option<String>,
    /// Reply To
    pub reply_to: Option<ServiceAddress>,
    /// Ttl
    pub ttl: Option<u64>,
}

/// Communication response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Communication operation
pub struct CommunicationResponse {
    /// Message identifier
    pub message_id: String,
    /// Success
    pub success: bool,
    /// Payload
    pub payload: Option<serde_json::Value>,
    /// Error
    pub error: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Type of message
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of Message
pub enum MessageType {
    /// Request
    Request,
    /// Response
    Response,
    /// Event
    Event,
    /// Command
    Command,
    /// Notification
    Notification,
}

/// Communication statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Communicationstats
pub struct CommunicationStats {
    /// Messages Sent
    pub messages_sent: u64,
    /// Messages Received
    pub messages_received: u64,
    /// Bytes Sent
    pub bytes_sent: u64,
    /// Bytes Received
    pub bytes_received: u64,
    /// Active Connections
    pub active_connections: u64,
    /// Failed Connections
    pub failed_connections: u64,
    /// Last Activity
    pub last_activity: Option<DateTime<Utc>>,
}
