use std::time::SystemTime;
use tokio::sync::RwLock;
use tracing::info;

use super::client::HttpClient;
use super::connection::ConnectionStats;
use crate::handlers::zfs::universal_zfs::config::RemoteConfig;

/// Remote ZFS service implementation
#[derive(Debug)]
pub struct RemoteZfsService {
    service_name: String,
    service_version: String,
    config: RemoteConfig,
    client: HttpClient,
    start_time: SystemTime,
    connection_stats: RwLock<ConnectionStats>,
}
impl Clone for RemoteZfsService {
    fn clone(&self) -> Self {
        Self {
            service_name: self.service_name.clone(),
            service_version: self.service_version.clone(),
            config: self.config.clone(),
            client: self.client.clone(),
            start_time: self.start_time,
            connection_stats: RwLock::new(ConnectionStats::default()),
        }
    }
}

impl RemoteZfsService {
    /// Create a new remote ZFS service with enhanced configuration
    pub fn new(config: RemoteConfig) -> Self {
        info!("Initializing remote ZFS service: {}", config.endpoint);

        let client = HttpClient::new(&config);

        Self {
            service_name: "remote-zfs".to_string(),
            service_version: "1.0.0".to_string(),
            config,
            client,
            start_time: SystemTime::now(),
            connection_stats: RwLock::new(ConnectionStats::default()),
        }
    }

    /// Get service name
    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    /// Get service version
    pub fn service_version(&self) -> &str {
        &self.service_version
    }

    /// Get configuration
    pub fn config(&self) -> &RemoteConfig {
        &self.config
    }

    /// Get HTTP client
    pub fn client(&self) -> &HttpClient {
        &self.client
    }

    /// Get connection statistics
    pub async fn connection_stats(&self) -> ConnectionStats {
        self.connection_stats.read().await.clone()
    }

    /// Update connection statistics after successful request
    pub async fn record_success(&self, response_time: std::time::Duration) {
        let mut stats = self.connection_stats.write().await;
        stats.record_success(response_time);
    }

    /// Update connection statistics after failed request
    pub async fn record_failure(&self, error: String) {
        let mut stats = self.connection_stats.write().await;
        stats.record_failure(error);
    }

    /// Check if remote service is available
    pub async fn is_available(&self) -> bool {
        self.client.health_check().await.is_ok()
    }

    /// Get service uptime
    pub fn uptime(&self) -> std::time::Duration {
        self.start_time.elapsed().unwrap_or_default()
    }
}
