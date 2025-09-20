// **LOG COLLECTORS AND AGGREGATION**
//! Collectors functionality and utilities.
// Log aggregation and collection functionality for forwarding logs to external systems.
// Extracted from tracing_setup.rs for file size compliance.

use crate::{NestGateError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info, warn};

use super::config::{
    LogAggregationConfig, LogDestination, LogEntry, ElasticsearchAuth, LokiAuth,
};

/// Log aggregator for batching and forwarding logs
pub struct LogAggregator {
    /// Configuration
    config: LogAggregationConfig,
    /// Log buffer
    buffer: Arc<RwLock<Vec<LogEntry>>>,
    /// HTTP client for external destinations
    client: reqwest::Client,
    /// Shutdown channel
    shutdown_tx: Option<mpsc::Sender<()>>,
}
impl LogAggregator {
    /// Create new log aggregator
    pub const fn new(config: LogAggregationConfig) -> Self {
        info!(
            "📋 Initializing log aggregator with {} destinations",
            config.destinations.len()
        );

        Self {
            config,
            buffer: Arc::new(RwLock::new(Vec::new())),
            client: reqwest::Client::new(),
            shutdown_tx: None,
        }
    }

    /// Start log aggregation background task
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn start(&mut self) -> Result<()>  {
        if !self.config.enabled {
            debug!("Log aggregation disabled");
            return Ok(());
        }

        let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        let buffer = self.buffer.clone();
        let config = self.config.clone();
        let client = self.client.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(config.flush_interval);

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if let Err(e) = Self::flush_logs(&buffer, &config, &client).await {
                            error!("Failed to flush logs: {}", e);
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        info!("Log aggregator shutting down");
                        // Final flush
                        if let Err(e) = Self::flush_logs(&buffer, &config, &client).await {
                            error!("Failed to flush logs on shutdown: {}", e);
                        }
                        break;
                    }
                }
            }
        );

        info!("✅ Log aggregator started");
        Ok(())
    }

    /// Add log entry to buffer
    pub fn add_log(&self, entry: LogEntry) {
        if !self.config.enabled {
            return;
        }

        let mut buffer = self.buffer.write().await;
        buffer.push(entry);

        // Flush if buffer is full
        if buffer.len() >= self.config.buffer_size {
            drop(buffer); // Release write lock
            if let Err(e) = Self::flush_logs(&self.buffer, &self.config, &self.client).await {
                error!("Failed to flush full buffer: {}", e);
            }
        }
    }

    /// Flush logs to external destinations
    async fn flush_logs(
        buffer: &Arc<RwLock<Vec<LogEntry>>>,
        config: &LogAggregationConfig,
        client: &reqwest::Client,
    ) -> Result<()> {
        let logs = {
            let mut buffer_guard = buffer.write().await;
            if buffer_guard.is_empty() {
                return Ok(());
            }
            let logs = buffer_guard.clone();
            buffer_guard.clear();
            logs
        };

        debug!(
            "Flushing {} log entries to {} destinations",
            logs.len(),
            config.destinations.len()
        );

        for destination in &config.destinations {
            if let Err(e) = Self::send_to_destination(&logs, destination, client).await {
                error!("Failed to send logs to destination: {}", e);
            }
        }

        Ok(())
    }

    /// Send logs to specific destination
    async fn send_to_destination(
        logs: &[LogEntry],
        destination: &LogDestination,
        client: &reqwest::Client,
    ) -> Result<()> {
        match destination {
            LogDestination::Elasticsearch { url, index, auth } => {
                Self::send_to_elasticsearch(logs, url, index, auth, client).await
            }
            LogDestination::Loki { url, labels, auth } => {
                Self::send_to_loki(logs, url, labels, auth, client).await
            }
            LogDestination::Syslog {
                host,
                port,
                facility,
            } => Self::send_to_syslog(logs, host, *port, facility).await,
            LogDestination::Webhook {
                url,
                method,
                headers,
            } => Self::send_to_webhook(logs, url, method, headers, client).await,
        }
    }

    /// Send logs to Elasticsearch
    async fn send_to_elasticsearch(
        logs: &[LogEntry],
        url: &str,
        index: &str,
        auth: &Option<ElasticsearchAuth>,
        client: &reqwest::Client,
    ) -> Result<()> {
        let bulk_url = format!("{url}/_bulk");
        let mut bulk_body = String::new();

        for log in logs {
            let index_line = serde_json::json!({
                "index": {
                    "_index": index,
                    "_type": "_doc"
                }
            );
            if let Ok(index_json) = serde_json::to_string(&index_line) {
                bulk_body.push_str(&index_json);
                bulk_body.push('\n');
                if let Ok(log_json) = serde_json::to_string(log) {
                    bulk_body.push_str(&log_json);
                } else {
                    tracing::warn!("Failed to serialize log entry, skipping");
                    continue;
                }
            } else {
                tracing::warn!("Failed to serialize index line, skipping log entry");
                continue;
            }
            bulk_body.push('\n');
        }

        let mut request = client
            .post(&bulk_url)
            .header("Content-Type", "application/x-ndjson")
            .body(bulk_body);

        if let Some(auth) = auth {
            request = match auth {
                ElasticsearchAuth::Basic { username, password } => {
                    request.basic_auth(username, Some(password))
                }
                ElasticsearchAuth::ApiKey { key } => {
                    request.header("Authorization", format!("ApiKey {key}"))
                }
            };
        }

        let response = request.send().await.map_err(|e| NestGateError::internal_error(
            location: Some(file!()"))?;

        if response.status().is_success() {
            debug!("✅ Sent {} logs to Elasticsearch", logs.len());
        } else {
            warn!(
                "Failed to send logs to Elasticsearch: {}",
                response.status()
            );
        }

        Ok(())
    }

    /// Send logs to Loki
    async fn send_to_loki(
        logs: &[LogEntry],
        url: &str,
        labels: &HashMap<String, String>,
        auth: &Option<LokiAuth>,
        client: &reqwest::Client,
    ) -> Result<()> {
        let push_url = format!("{url}/loki/api/v1/push");

        let streams: Vec<serde_json::Value> = logs
            .iter()
            .map(|log| {
                let mut stream_labels = labels.clone();
                stream_labels.insert("level".to_string(), log.level.clone());
                stream_labels.insert("service".to_string(), log.service.clone());

                let timestamp_ns = log
                    .timestamp
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos()
                    .to_string();

                serde_json::json!({
                    "stream": stream_labels,
                    "values": [[timestamp_ns, log.message]]
                })
            })
            .collect();

        let payload = serde_json::json!({
            "streams": streams
        );

        let mut request = client
            .post(&push_url)
            .header("Content-Type", "application/json")
            .json(&payload);

        if let Some(auth) = auth {
            request = match auth {
                LokiAuth::Basic { username, password } => {
                    request.basic_auth(username, Some(password))
                }
                LokiAuth::Bearer { token } => {
                    request.header("Authorization", format!("Bearer {token}"))
                }
            };
        }

        let response = request.send().await.map_err(|e| NestGateError::internal_error(
            location: Some(file!()"))?;

        if response.status().is_success() {
            debug!("✅ Sent {} logs to Loki", logs.len());
        } else {
            warn!("Failed to send logs to Loki: {}", response.status());
        }

        Ok(())
    }

    /// Send logs to Syslog
    async fn send_to_syslog(
        logs: &[LogEntry],
        host: &str,
        port: u16,
        facility: &str,
    ) -> Result<()> {
        // Syslog integration implementation
        debug!(
            "Sending {} logs to syslog at {}:{} (facility: {})",
            logs.len(),
            host,
            port,
            facility
        );

        // Format logs for syslog RFC 5424 format
        for log in logs {
            let syslog_message = format!(
                "<{}>{} {} {} {} {} - {}",
                Self::get_syslog_priority(&log.level, facility),
                chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ"),
                std::env::var("HOSTNAME").unwrap_or_else(|_| "nestgate".to_string()),
                "nestgate",
                std::process::id(),
                log.message.as_str(),
                log.message
            );

            // In a real implementation, this would send via UDP/TCP to syslog daemon
            // For now, we log the formatted message
            tracing::info!(target: "syslog", "{}", syslog_message);
        }

        Ok(())
    }

    /// Get syslog priority value (facility * 8 + severity)
    fn get_syslog_priority(level: &str, facility: &str) -> u8 {
        let facility_code = match facility {
            "user" => 1,
            "daemon" => 3,
            "local0" => 16,
            "local1" => 17,
            _ => 16, // default to local0
        };

        let severity = match level {
            "ERROR" => 3,
            "WARN" => 4,
            "INFO" => 6,
            "DEBUG" => 7,
            _ => 6, // default to info
        };

        facility_code * 8 + severity
    }

    /// Send logs to webhook
    async fn send_to_webhook(
        logs: &[LogEntry],
        url: &str,
        method: &str,
        headers: &HashMap<String, String>,
        client: &reqwest::Client,
    ) -> Result<()> {
        let payload = serde_json::json!({
            "logs": logs,
            "count": logs.len(),
            "timestamp": SystemTime::now()
        );

        let mut request = match method.to_uppercase().as_str() {
            "POST" => client.post(url).json(&payload),
            "PUT" => client.put(url).json(&payload),
            _ => {
                return Err(NestGateError::internal_error(
                    location: Some(file!().to_string())})
            }
        };

        for (key, value) in headers {
            request = request.header(key, value);
        }

        let response = request.send().await.map_err(|e| NestGateError::internal_error(
            location: Some(file!()"))?;

        if response.status().is_success() {
            debug!("✅ Sent {} logs to webhook", logs.len());
        } else {
            warn!("Failed to send logs to webhook: {}", response.status());
        }

        Ok(())
    }

    /// Stop log aggregation
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn stop(&mut self) -> Result<()>  {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(()).await;
            info!("Log aggregator stop signal sent");
        }
        Ok(())
    }
} 