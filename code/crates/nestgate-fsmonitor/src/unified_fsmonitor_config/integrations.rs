/// **INTEGRATIONS MODULE**
/// External system integration configuration - extracted from monolithic config
/// Handles webhooks, message queues, databases, APIs, and custom integrations
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Integration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationSettings {
    /// Enable integrations
    pub enabled: bool,
    /// Webhook integrations
    pub webhooks: Vec<WebhookIntegration>,
    /// Message queue integrations
    pub message_queues: Vec<MessageQueueIntegration>,
    /// Database integrations
    pub databases: Vec<DatabaseIntegration>,
    /// API integrations
    pub apis: Vec<ApiIntegration>,
    /// Custom integrations
    pub custom: HashMap<String, CustomIntegration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookIntegration {
    /// Webhook name
    pub name: String,
    /// Webhook URL
    pub url: String,
    /// HTTP method
    pub method: String,
    /// Authentication configuration
    pub auth: WebhookAuth,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Integration enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookAuth {
    /// Authentication type (none, bearer, basic, api_key)
    pub auth_type: String,
    /// Authentication credentials
    pub credentials: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageQueueIntegration {
    /// Queue name
    pub name: String,
    /// Queue type (rabbitmq, kafka, redis, etc.)
    pub queue_type: String,
    /// Connection configuration
    pub connection: HashMap<String, serde_json::Value>,
    /// Topic or queue name
    pub topic: String,
    /// Integration enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseIntegration {
    /// Database name
    pub name: String,
    /// Database type (postgresql, mysql, mongodb, etc.)
    pub db_type: String,
    /// Connection configuration
    pub connection: HashMap<String, serde_json::Value>,
    /// Table or collection name
    pub table: String,
    /// Integration enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiIntegration {
    /// API name
    pub name: String,
    /// API base URL
    pub base_url: String,
    /// Authentication configuration
    pub auth: ApiAuth,
    /// Rate limiting configuration
    pub rate_limiting: ApiRateLimiting,
    /// Integration enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuth {
    /// Authentication type
    pub auth_type: String,
    /// Authentication configuration
    pub config: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRateLimiting {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per second limit
    pub requests_per_second: u32,
    /// Burst limit
    pub burst_limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomIntegration {
    /// Integration type
    pub integration_type: String,
    /// Integration configuration
    pub config: HashMap<String, serde_json::Value>,
    /// Integration enabled
    pub enabled: bool,
}

impl Default for IntegrationSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            webhooks: Vec::new(),
            message_queues: Vec::new(),
            databases: Vec::new(),
            apis: Vec::new(),
            custom: HashMap::new(),
        }
    }
}
