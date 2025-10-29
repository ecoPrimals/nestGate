// Removed unused error imports
/// Communication Traits
use chrono::{DateTime, Utc};
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::Result;

/// Communication layer trait
pub trait CommunicationLayer: Send + Sync {
    /// Send a message to a specific service
    async fn send_message(
        &self,
        target: ServiceAddress,
        message: ServiceMessage,
    ) -> Result<CommunicationResponse>;
    /// Broadcast a message to all services
    fn broadcast(&self, message: ServiceMessage) -> impl std::future::Future<Output = Result<Vec<CommunicationResponse>> + Send;

    /// Listen for incoming messages
    fn listen(&self) -> impl std::future::Future<Output = impl Stream<Item = (ServiceAddress, ServiceMessage)>> + Send;

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
pub struct ServiceAddress {
    pub service_id: String,
    pub instance_id: Option<String>,
    pub endpoint: Option<String>,
}
/// Message between services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMessage {
    pub id: String,
    pub message_type: MessageType,
    pub topic: Option<String>,
    pub payload: serde_json::Value,
    pub headers: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
    pub correlation_id: Option<String>,
    pub reply_to: Option<ServiceAddress>,
    pub ttl: Option<u64>,
}
/// Communication response (renamed to avoid conflict with service::ServiceResponse)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationResponse {
    pub message_id: String,
    pub success: bool,
    pub payload: Option<serde_json::Value>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}
/// Type of message
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    Request,
    Response,
    Event,
    Command,
    Notification,
}
/// Communication statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommunicationStats {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub active_connections: u64,
    pub failed_connections: u64,
    pub last_activity: Option<DateTime<Utc>>,
}
