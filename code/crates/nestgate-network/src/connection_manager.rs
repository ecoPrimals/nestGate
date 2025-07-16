//! Connection Manager - ALL connections MUST go through Songbird
//!
//! This module enforces that every network connection, port allocation,
//! and service communication goes through the Songbird orchestrator.
//! NO DIRECT CONNECTIONS ALLOWED.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use crate::{Result, SongbirdClient};

/// Connection types that must be managed by Songbird
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionType {
    /// API service connection
    Api,
    /// NFS protocol connection
    Nfs,
    /// SMB protocol connection
    Smb,
    /// iSCSI protocol connection
    Iscsi,
    /// S3 protocol connection
    S3,
    /// Internal service-to-service communication
    Internal(String),
    /// Health check endpoint
    Health,
    /// Metrics endpoint
    Metrics,
}

/// Connection request to Songbird
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionRequest {
    /// Source service name
    pub source_service: String,
    /// Target service name
    pub target_service: String,
    /// Connection type
    pub connection_type: ConnectionType,
    /// Required capabilities
    pub required_capabilities: Vec<String>,
    /// Connection metadata
    pub metadata: HashMap<String, String>,
}

/// Connection response from Songbird
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionResponse {
    /// Connection ID
    pub connection_id: String,
    /// Target endpoint (host:port)
    pub endpoint: String,
    /// Connection token (if required)
    pub token: Option<String>,
    /// Connection expires at
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Connection metadata
    pub metadata: HashMap<String, String>,
}

/// Active connection tracking
#[derive(Debug, Clone)]
pub struct ActiveConnection {
    /// Connection ID
    pub connection_id: String,
    /// Connection request
    pub request: ConnectionRequest,
    /// Connection response
    pub response: ConnectionResponse,
    /// Connection established at
    pub established_at: chrono::DateTime<chrono::Utc>,
    /// Last used at
    pub last_used_at: chrono::DateTime<chrono::Utc>,
}

/// Songbird Connection Manager - NO DIRECT CONNECTIONS ALLOWED
#[derive(Debug)]
pub struct SongbirdConnectionManager {
    /// Songbird client (MANDATORY)
    songbird: SongbirdClient,
    /// Service name for this instance
    service_name: String,
    /// Active connections
    active_connections: Arc<RwLock<HashMap<String, ActiveConnection>>>,
    /// Connection pool
    connection_pool: Arc<RwLock<HashMap<String, Vec<ActiveConnection>>>>,
}

impl SongbirdConnectionManager {
    /// Create new connection manager (Songbird is MANDATORY)
    pub fn new(songbird_url: String, service_name: String) -> Self {
        info!(
            "🎼 Creating Songbird Connection Manager for service: {}",
            service_name
        );
        info!("🎼 Songbird URL: {}", songbird_url);
        info!("🚫 Direct connections are FORBIDDEN - all connections via Songbird");

        Self {
            songbird: SongbirdClient::new(songbird_url),
            service_name,
            active_connections: Arc::new(RwLock::new(HashMap::new())),
            connection_pool: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Request a connection through Songbird (MANDATORY PATH)
    pub async fn request_connection(
        &self,
        request: ConnectionRequest,
    ) -> Result<ConnectionResponse> {
        info!(
            "🔗 Requesting connection via Songbird: {} -> {}",
            request.source_service, request.target_service
        );

        // ✅ ALL CONNECTIONS MUST GO THROUGH SONGBIRD
        let response = self.songbird.request_connection(&request).await?;

        // Track the connection
        let connection = ActiveConnection {
            connection_id: response.connection_id.clone(),
            request: request.clone(),
            response: response.clone(),
            established_at: chrono::Utc::now(),
            last_used_at: chrono::Utc::now(),
        };

        // Store in active connections
        let mut active = self.active_connections.write().await;
        active.insert(response.connection_id.clone(), connection.clone());

        // Add to connection pool
        let mut pool = self.connection_pool.write().await;
        let target_connections = pool
            .entry(request.target_service.clone())
            .or_insert_with(Vec::new);
        target_connections.push(connection);

        info!(
            "✅ Connection established via Songbird: {}",
            response.connection_id
        );
        Ok(response)
    }

    /// Get connection to a service (through Songbird)
    pub async fn get_connection(
        &self,
        target_service: &str,
        connection_type: ConnectionType,
    ) -> Result<ConnectionResponse> {
        // Check if we have an existing connection
        let pool = self.connection_pool.read().await;
        if let Some(connections) = pool.get(target_service) {
            if let Some(conn) = connections.iter().find(|c| {
                matches!(c.request.connection_type, ref ct if std::mem::discriminant(ct) == std::mem::discriminant(&connection_type))
            }) {
                // Update last used time
                let mut active = self.active_connections.write().await;
                if let Some(active_conn) = active.get_mut(&conn.connection_id) {
                    active_conn.last_used_at = chrono::Utc::now();
                }

                debug!("♻️ Reusing existing connection: {}", conn.connection_id);
                return Ok(conn.response.clone());
            }
        }
        drop(pool);

        // Request new connection through Songbird
        let request = ConnectionRequest {
            source_service: self.service_name.clone(),
            target_service: target_service.to_string(),
            connection_type,
            required_capabilities: vec![],
            metadata: HashMap::new(),
        };

        self.request_connection(request).await
    }

    /// Release a connection through Songbird
    pub async fn release_connection(&self, connection_id: &str) -> Result<()> {
        info!("🔓 Releasing connection via Songbird: {}", connection_id);

        // Remove from active connections
        let mut active = self.active_connections.write().await;
        let connection = active.remove(connection_id);
        drop(active);

        if let Some(conn) = connection {
            // Remove from connection pool
            let mut pool = self.connection_pool.write().await;
            if let Some(connections) = pool.get_mut(&conn.request.target_service) {
                connections.retain(|c| c.connection_id != connection_id);
                if connections.is_empty() {
                    pool.remove(&conn.request.target_service);
                }
            }
            drop(pool);

            // Notify Songbird
            self.songbird.release_connection(connection_id).await?;
            info!("✅ Connection released via Songbird: {}", connection_id);
        } else {
            warn!("⚠️ Connection not found: {}", connection_id);
        }

        Ok(())
    }

    /// Connect to a service (ONLY through Songbird)
    pub async fn connect_to_service(
        &self,
        service_name: &str,
        connection_type: ConnectionType,
    ) -> Result<String> {
        info!(
            "🔌 Connecting to service via Songbird: {} (type: {:?})",
            service_name, connection_type
        );

        let response = self.get_connection(service_name, connection_type).await?;

        info!(
            "✅ Connected to service via Songbird: {} -> {}",
            service_name, response.endpoint
        );
        Ok(response.endpoint)
    }

    /// Get service endpoint (ONLY through Songbird)
    pub async fn get_service_endpoint(
        &self,
        service_name: &str,
        connection_type: ConnectionType,
    ) -> Result<String> {
        let response = self.get_connection(service_name, connection_type).await?;
        Ok(response.endpoint)
    }

    /// List all active connections
    pub async fn list_active_connections(&self) -> HashMap<String, ActiveConnection> {
        self.active_connections.read().await.clone()
    }

    /// Cleanup expired connections
    pub async fn cleanup_expired_connections(&self) -> Result<()> {
        let now = chrono::Utc::now();
        let mut to_remove = Vec::new();

        {
            let active = self.active_connections.read().await;
            for (connection_id, connection) in active.iter() {
                if let Some(expires_at) = connection.response.expires_at {
                    if now > expires_at {
                        to_remove.push(connection_id.clone());
                    }
                }
            }
        }

        for connection_id in to_remove {
            info!("🧹 Cleaning up expired connection: {}", connection_id);
            if let Err(e) = self.release_connection(&connection_id).await {
                error!(
                    "Failed to release expired connection {}: {}",
                    connection_id, e
                );
            }
        }

        Ok(())
    }

    /// Health check all connections
    pub async fn health_check_connections(&self) -> Result<HashMap<String, bool>> {
        let mut health_status = HashMap::new();
        let active = self.active_connections.read().await;

        for (connection_id, _connection) in active.iter() {
            // Check if connection is still valid
            let is_healthy = self
                .songbird
                .check_connection_health(connection_id)
                .await
                .unwrap_or(false);
            health_status.insert(connection_id.clone(), is_healthy);

            if !is_healthy {
                warn!("🏥 Unhealthy connection detected: {}", connection_id);
            }
        }

        Ok(health_status)
    }
}

/// Extension methods for SongbirdClient
impl SongbirdClient {
    /// Request a connection through Songbird
    pub async fn request_connection(
        &self,
        request: &ConnectionRequest,
    ) -> Result<ConnectionResponse> {
        let url = format!("{}/api/v1/connections/request", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(request)
            .send()
            .await
            .map_err(|e| {
                nestgate_core::NestGateError::Internal(format!("Failed to request connection: {e}"))
            })?;

        if response.status().is_success() {
            let connection_response: ConnectionResponse = response.json().await.map_err(|e| {
                nestgate_core::NestGateError::Internal(format!(
                    "Failed to parse connection response: {e}"
                ))
            })?;

            info!(
                "✅ Connection requested via Songbird: {}",
                connection_response.connection_id
            );
            Ok(connection_response)
        } else {
            Err(nestgate_core::NestGateError::Internal(format!(
                "Connection request failed: HTTP {}",
                response.status()
            )))
        }
    }

    /// Release a connection through Songbird
    pub async fn release_connection(&self, connection_id: &str) -> Result<()> {
        let url = format!(
            "{}/api/v1/connections/{}/release",
            self.base_url, connection_id
        );

        let response = self.client.delete(&url).send().await.map_err(|e| {
            nestgate_core::NestGateError::Internal(format!("Failed to release connection: {e}"))
        })?;

        if response.status().is_success() {
            info!("✅ Connection released via Songbird: {}", connection_id);
            Ok(())
        } else {
            warn!(
                "Failed to release connection {}: HTTP {}",
                connection_id,
                response.status()
            );
            Ok(()) // Don't fail on release errors
        }
    }

    /// Check connection health through Songbird
    pub async fn check_connection_health(&self, connection_id: &str) -> Result<bool> {
        let url = format!(
            "{}/api/v1/connections/{}/health",
            self.base_url, connection_id
        );

        let response = self.client.get(&url).send().await.map_err(|e| {
            nestgate_core::NestGateError::Internal(format!(
                "Failed to check connection health: {e}"
            ))
        })?;

        Ok(response.status().is_success())
    }
}
