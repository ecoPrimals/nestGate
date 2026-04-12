// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Implementation module

use serde_json::json;
use std::collections::HashMap;
use std::time::SystemTime;
use tracing::debug;

use super::service::RemoteZfsService;
use crate::handlers::zfs::universal_zfs::traits::UniversalZfsService;
use crate::handlers::zfs::universal_zfs_types::{
    DatasetConfig, DatasetInfo, HealthStatus, PoolConfig, PoolInfo, ServiceMetrics, ServiceStatus,
    SnapshotConfig, SnapshotInfo, UniversalZfsError, UniversalZfsResult,
};

impl UniversalZfsService for RemoteZfsService {
    /// Service Name
    fn service_name(&self) -> &str {
        self.service_name()
    }

    /// Service Version
    fn service_version(&self) -> &str {
        self.service_version()
    }

    /// Health Check
    async fn health_check(&self) -> UniversalZfsResult<HealthStatus> {
        debug!("Getting health status via remote service");

        // Perform actual health check by calling remote endpoint
        let overall_healthy = match self.client().get("/health").await {
            Ok(response) => {
                // Parse health response or assume healthy if endpoint responds
                response
                    .get("healthy")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(true)
            }
            Err(_) => {
                // If health endpoint fails, service is degraded
                false
            }
        };

        Ok(HealthStatus {
            service_name: self.service_name().to_string(),
            status: if overall_healthy {
                ServiceStatus::Healthy
            } else {
                ServiceStatus::Degraded
            },
            checks: vec![],
            last_check: SystemTime::now(),
            metadata: HashMap::new(),
        })
    }

    /// Gets Metrics
    async fn get_metrics(&self) -> UniversalZfsResult<ServiceMetrics> {
        debug!("Getting service metrics via remote service");

        // Collect actual metrics from remote service
        let metrics = match self.client().get("/metrics").await {
            Ok(response) => {
                // Parse metrics from remote service response
                if let Some(custom_metrics) = response.get("custom_metrics") {
                    serde_json::from_value(custom_metrics.clone()).unwrap_or_default()
                } else {
                    HashMap::new()
                }
            }
            Err(_) => HashMap::new(), // Return empty metrics if remote call fails
        };

        Ok(ServiceMetrics {
            service_name: self.service_name().to_string(),
            timestamp: SystemTime::now(),
            requests_total: 0,
            requests_failed: 0,
            error_rate: 0.0,
            latency_avg: 0.0,
            latency_p95: 0.0,
            latency_p99: 0.0,
            custom_metrics: metrics,
        })
    }

    /// Checks if Available
    async fn is_available(&self) -> bool {
        // Check availability by attempting to connect to remote service
        (self.client().get("/health").await).is_ok()
    }

    /// List Pools
    async fn list_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        debug!("Listing pools via remote service");

        match self.client().get("/api/v1/pools").await {
            Ok(response) => {
                // Parse response into Vec<PoolInfo>
                if let Some(pools_array) = response.get("pools") {
                    match serde_json::from_value(pools_array.clone()) {
                        Ok(pools) => Ok(pools),
                        Err(e) => {
                            debug!("Failed to parse pools response: {}", e);
                            Ok(vec![]) // Return empty list if parsing fails
                        }
                    }
                } else {
                    Ok(vec![]) // Return empty list if no pools key
                }
            }
            Err(e) => {
                debug!("Failed to list pools from remote service: {}", e);
                Err(e) // Propagate the error
            }
        }
    }

    /// Gets Pool
    async fn get_pool(&self, name: &str) -> UniversalZfsResult<Option<PoolInfo>> {
        debug!("Getting pool '{}' via remote service", name);

        let endpoint = format!("/api/v1/pools/{name}");
        match self.client().get(&endpoint).await {
            Ok(response) => {
                // Parse response into PoolInfo
                match serde_json::from_value(response) {
                    Ok(pool_info) => Ok(Some(pool_info)),
                    Err(e) => {
                        debug!("Failed to parse pool response for '{}': {}", name, e);
                        Ok(None) // Return None if parsing fails
                    }
                }
            }
            Err(e) => {
                debug!("Failed to get pool '{}' from remote service: {}", name, e);
                // Return None for 404-like errors, propagate other errors
                if e.to_string().contains("404") || e.to_string().contains("not found") {
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Creates  Pool
    async fn create_pool(&self, config: &PoolConfig) -> UniversalZfsResult<PoolInfo> {
        debug!("Creating pool '{}' via remote service", config.name);
        let start_time = std::time::Instant::now();

        let request_body = json!({
            "name": config.name,
            "_devices": config.devices,
            "properties": config.properties,
        });

        match self.client().post("/api/v1/pools", request_body).await {
            Ok(response) => {
                self.record_success(start_time.elapsed()).await;
                // Parse response into PoolInfo
                serde_json::from_value(response).map_err(|e| UniversalZfsError::Backend {
                    backend: "remote".to_string(),
                    message: format!("Failed to parse pool response: {e}"),
                })
            }
            Err(e) => {
                let error = format!("Failed to create pool: {e}");
                self.record_failure(error.clone()).await;
                Err(e)
            }
        }
    }

    /// Destroy Pool
    async fn destroy_pool(&self, name: &str) -> UniversalZfsResult<()> {
        debug!("Destroying pool '{}' via remote service", name);

        let endpoint = format!("/api/v1/pools/{name}");
        match self.client().delete(&endpoint).await {
            Ok(_) => {
                debug!("Successfully destroyed pool '{}'", name);
                Ok(())
            }
            Err(e) => {
                let _error_msg = format!("Failed to destroy pool: {e}");
                debug!("{}", _error_msg);
                self.record_failure(_error_msg).await;
                Err(e)
            }
        }
    }

    /// Scrub Pool
    async fn scrub_pool(&self, name: &str) -> UniversalZfsResult<()> {
        debug!("Scrubbing pool '{}' via remote service", name);

        let endpoint = format!("/api/v1/pools/{name}/scrub");
        match self.client().post(&endpoint, json!({})).await {
            Ok(_) => {
                debug!("Successfully started scrub for pool '{}'", name);
                Ok(())
            }
            Err(e) => {
                let _error_msg = format!("Failed to scrub pool: {e}");
                debug!("{}", _error_msg);
                self.record_failure(_error_msg).await;
                Err(e)
            }
        }
    }

    /// Gets Pool Status
    async fn get_pool_status(&self, name: &str) -> UniversalZfsResult<String> {
        debug!("Getting pool status for '{}' via remote service", name);

        let endpoint = format!("/api/v1/pools/{name}/status");
        match self.client().get(&endpoint).await {
            Ok(response) => {
                // Extract status from response
                if let Some(status) = response.get("status").and_then(|s| s.as_str()) {
                    Ok(status.to_string())
                } else {
                    // Fallback to parsing the entire response as status
                    Ok(response.to_string())
                }
            }
            Err(e) => {
                debug!("Failed to get pool status for '{}': {}", name, e);
                // Return error instead of fallback for better error handling
                Err(e)
            }
        }
    }

    /// List Datasets
    async fn list_datasets(&self) -> UniversalZfsResult<Vec<DatasetInfo>> {
        debug!("Listing datasets via remote service");

        match self.client().get("/api/v1/datasets").await {
            Ok(response) => {
                // Parse response into Vec<DatasetInfo>
                if let Some(datasets_array) = response.get("datasets") {
                    match serde_json::from_value(datasets_array.clone()) {
                        Ok(datasets) => Ok(datasets),
                        Err(e) => {
                            debug!("Failed to parse datasets response: {}", e);
                            Ok(vec![]) // Return empty list if parsing fails
                        }
                    }
                } else {
                    Ok(vec![]) // Return empty list if no datasets key
                }
            }
            Err(e) => {
                debug!("Failed to list datasets from remote service: {}", e);
                Err(e) // Propagate the error
            }
        }
    }

    /// Gets Dataset
    async fn get_dataset(&self, name: &str) -> UniversalZfsResult<Option<DatasetInfo>> {
        debug!("Getting dataset '{}' via remote service", name);

        let endpoint = format!("/api/v1/datasets/{name}");
        match self.client().get(&endpoint).await {
            Ok(response) => {
                // Parse response into DatasetInfo
                match serde_json::from_value(response) {
                    Ok(dataset_info) => Ok(Some(dataset_info)),
                    Err(e) => {
                        debug!("Failed to parse dataset response for '{}': {}", name, e);
                        Ok(None) // Return None if parsing fails
                    }
                }
            }
            Err(e) => {
                debug!(
                    "Failed to get dataset '{}' from remote service: {}",
                    name, e
                );
                // Return None for 404-like errors, propagate other errors
                if e.to_string().contains("404") || e.to_string().contains("not found") {
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Creates  Dataset
    async fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo> {
        debug!("Creating dataset '{}' via remote service", config.name);
        let start_time = std::time::Instant::now();

        let request_body = json!({
            "name": config.name,
            "properties": config.properties,
        });

        match self.client().post("/api/v1/datasets", request_body).await {
            Ok(response) => {
                self.record_success(start_time.elapsed()).await;
                // Parse response into DatasetInfo
                serde_json::from_value(response).map_err(|e| UniversalZfsError::Backend {
                    backend: "remote".to_string(),
                    message: format!("Failed to parse dataset response: {e}"),
                })
            }
            Err(e) => {
                let error = format!("Failed to create dataset: {e}");
                self.record_failure(error.clone()).await;
                Err(e)
            }
        }
    }

    /// Destroy Dataset
    async fn destroy_dataset(&self, name: &str) -> UniversalZfsResult<()> {
        debug!("Destroying dataset '{}' via remote service", name);

        let endpoint = format!("/api/v1/datasets/{name}");
        match self.client().delete(&endpoint).await {
            Ok(_) => {
                debug!("Successfully destroyed dataset '{}'", name);
                Ok(())
            }
            Err(e) => {
                let _error_msg = format!("Failed to destroy dataset: {e}");
                debug!("{}", _error_msg);
                self.record_failure(_error_msg).await;
                Err(e)
            }
        }
    }

    /// Gets Dataset Properties
    async fn get_dataset_properties(
        &self,
        name: &str,
    ) -> UniversalZfsResult<HashMap<String, String>> {
        debug!(
            "Getting properties for dataset '{}' via remote service",
            name
        );

        let endpoint = format!("/api/v1/datasets/{name}/properties");
        match self.client().get(&endpoint).await {
            Ok(response) => {
                // Parse response into HashMap<String, String>
                if let Some(propertiesvalue) = response.get("properties") {
                    match serde_json::from_value(propertiesvalue.clone()) {
                        Ok(properties) => Ok(properties),
                        Err(e) => {
                            debug!("Failed to parse dataset properties response: {}", e);
                            Ok(HashMap::new()) // Return empty map if parsing fails
                        }
                    }
                } else {
                    debug!("No properties field in response for dataset '{}'", name);
                    Ok(HashMap::new()) // Return empty map if no properties key
                }
            }
            Err(e) => {
                debug!(
                    "Failed to get dataset properties from remote service: {}",
                    e
                );
                Err(e) // Propagate the error
            }
        }
    }

    /// Sets Dataset Properties
    async fn set_dataset_properties(
        &self,
        name: &str,
        properties: &HashMap<String, String>,
    ) -> UniversalZfsResult<()> {
        debug!(
            "Setting properties for dataset '{}' via remote service",
            name
        );

        let endpoint = format!("/api/v1/datasets/{name}/properties");
        let request_body = json!({
            "properties": properties
        });

        match self.client().put(&endpoint, request_body).await {
            Ok(_) => {
                debug!("Successfully set properties for dataset '{}'", name);
                Ok(())
            }
            Err(e) => {
                debug!("Failed to set dataset properties via remote service: {}", e);
                Err(e) // Propagate the error
            }
        }
    }

    /// List Snapshots
    async fn list_snapshots(&self) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        debug!("Listing snapshots via remote service");

        match self.client().get("/api/v1/snapshots").await {
            Ok(response) => {
                // Parse response into Vec<SnapshotInfo>
                if let Some(snapshots_array) = response.get("snapshots") {
                    match serde_json::from_value(snapshots_array.clone()) {
                        Ok(snapshots) => Ok(snapshots),
                        Err(e) => {
                            debug!("Failed to parse snapshots response: {}", e);
                            Ok(vec![]) // Return empty list if parsing fails
                        }
                    }
                } else {
                    Ok(vec![]) // Return empty list if no snapshots key
                }
            }
            Err(e) => {
                debug!("Failed to list snapshots from remote service: {}", e);
                Err(e) // Propagate the error
            }
        }
    }

    /// List Dataset Snapshots
    async fn list_dataset_snapshots(
        &self,
        dataset: &str,
    ) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        debug!(
            "Listing snapshots for dataset '{}' via remote service",
            dataset
        );

        let endpoint = format!("/api/v1/datasets/{dataset}/snapshots");
        match self.client().get(&endpoint).await {
            Ok(response) => {
                // Parse response into Vec<SnapshotInfo>
                if let Some(snapshots_array) = response.get("snapshots") {
                    match serde_json::from_value(snapshots_array.clone()) {
                        Ok(snapshots) => Ok(snapshots),
                        Err(e) => {
                            debug!("Failed to parse dataset snapshots response: {}", e);
                            Ok(vec![]) // Return empty list if parsing fails
                        }
                    }
                } else {
                    debug!("No snapshots field in response for dataset '{}'", dataset);
                    Ok(vec![]) // Return empty list if no snapshots key
                }
            }
            Err(e) => {
                debug!(
                    "Failed to list dataset snapshots from remote service: {}",
                    e
                );
                Err(e) // Propagate the error
            }
        }
    }

    /// Creates  Snapshot
    async fn create_snapshot(
        &self,
        config: &SnapshotConfig,
    ) -> UniversalZfsResult<SnapshotInfo> {
        debug!("Creating snapshot '{}' via remote service", config.name);
        let start_time = std::time::Instant::now();

        let request_body = json!({
            "name": config.name,
            "dataset": config.dataset,
            "properties": config.properties,
        });

        match self.client().post("/api/v1/snapshots", request_body).await {
            Ok(response) => {
                self.record_success(start_time.elapsed()).await;
                // Parse response into SnapshotInfo
                serde_json::from_value(response).map_err(|e| UniversalZfsError::Backend {
                    backend: "remote".to_string(),
                    message: format!("Failed to parse snapshot response: {e}"),
                })
            }
            Err(e) => {
                let error = format!("Failed to create snapshot: {e}");
                self.record_failure(error.clone()).await;
                Err(e)
            }
        }
    }

    /// Destroy Snapshot
    async fn destroy_snapshot(&self, name: &str) -> UniversalZfsResult<()> {
        debug!("Destroying snapshot '{}' via remote service", name);

        let endpoint = format!("/api/v1/snapshots/{name}");
        match self.client().delete(&endpoint).await {
            Ok(_) => {
                debug!("Successfully destroyed snapshot '{}'", name);
                Ok(())
            }
            Err(e) => {
                let _error_msg = format!("Failed to destroy snapshot: {e}");
                debug!("{}", _error_msg);
                self.record_failure(_error_msg).await;
                Err(e)
            }
        }
    }

    /// Optimize
    async fn optimize(&self) -> UniversalZfsResult<String> {
        debug!("Running optimization via remote service");

        match self.client().post("/api/v1/optimize", json!({})).await {
            Ok(response) => {
                // Extract optimization result message
                if let Some(message) = response.get("message").and_then(|m| m.as_str()) {
                    Ok(message.to_string())
                } else if let Some(status) = response.get("status").and_then(|s| s.as_str()) {
                    Ok(format!("Status: {status}"))
                } else {
                    Ok("Optimization completed successfully".to_string())
                }
            }
            Err(e) => {
                debug!("Failed to run optimization: {}", e);
                Err(e)
            }
        }
    }

    /// Gets Optimization Analytics
    async fn get_optimization_analytics(&self) -> UniversalZfsResult<serde_json::Value> {
        debug!("Getting optimization analytics via remote service");

        match self.client().get("/api/v1/optimize/analytics").await {
            Ok(response) => {
                // Return the analytics data directly
                Ok(response)
            }
            Err(e) => {
                debug!("Failed to get optimization analytics: {}", e);
                // Return empty analytics on error instead of propagating
                Ok(json!({
                    "error": "Failed to retrieve analytics",
                    "message": e.to_string(),
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "pools_analyzed": 0,
                    "recommendations": []
                }))
            }
        }
    }

    /// Predict Tier
    async fn predict_tier(&self, file_path: &str) -> UniversalZfsResult<String> {
        debug!("Predicting tier for '{}' via remote service", file_path);

        let request_body = json!({
            "file_path": file_path,
            "analyze_access_patterns": true,
            "include_metadata": true
        });

        match self
            .client()
            .post("/api/v1/predict/tier", request_body)
            .await
        {
            Ok(response) => {
                // Extract predicted tier from response
                if let Some(tier) = response.get("predicted_tier").and_then(|t| t.as_str()) {
                    Ok(tier.to_string())
                } else if let Some(tier) = response.get("tier").and_then(|t| t.as_str()) {
                    Ok(tier.to_string())
                } else {
                    // Fallback to "unknown" if parsing fails
                    Ok("unknown".to_string())
                }
            }
            Err(e) => {
                debug!("Failed to predict tier for '{}': {}", file_path, e);
                // Return default tier instead of error for graceful degradation
                Ok("warm".to_string())
            }
        }
    }

    /// Gets Configuration
    async fn get_configuration(&self) -> UniversalZfsResult<serde_json::Value> {
        debug!("Getting configuration via remote service");

        match self.client().get("/api/v1/configuration").await {
            Ok(response) => {
                // Return the configuration data directly
                Ok(response)
            }
            Err(e) => {
                debug!("Failed to get configuration: {}", e);
                // Return default configuration structure on error
                Ok(json!({
                    "error": "Failed to retrieve configuration",
                    "message": e.to_string(),
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "service_name": self.service_name(),
                    "version": self.service_version(),
                    "default_config": {
                        "pools": [],
                        "datasets": [],
                        "snapshots": []
                    }
                }))
            }
        }
    }

    /// Updates  Configuration
    async fn update_configuration(&self, config: serde_json::Value) -> UniversalZfsResult<()> {
        debug!("Updating configuration via remote service");

        match self.client().put("/api/v1/configuration", config).await {
            Ok(_) => {
                debug!("Successfully updated configuration");
                Ok(())
            }
            Err(e) => {
                let _error_msg = format!("Failed to update configuration: {e}");
                debug!("{}", _error_msg);
                self.record_failure(_error_msg).await;
                Err(e)
            }
        }
    }

    /// Shutdown
    async fn shutdown(&self) -> UniversalZfsResult<()> {
        debug!("Shutting down remote service");

        match self.client().post("/api/v1/shutdown", json!({})).await {
            Ok(_) => {
                debug!("Successfully initiated remote service shutdown");
                Ok(())
            }
            Err(e) => {
                // Log the error but don't fail - shutdown might be expected to close connection
                debug!(
                    "Remote service shutdown call failed (this may be expected): {}",
                    e
                );
                // Return success since connection termination during shutdown is normal
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod implementation_tests {
    use super::super::service::RemoteZfsService;
    use crate::handlers::zfs::universal_zfs::config::RemoteConfig;
    use std::time::Duration;

    #[test]
    #[expect(deprecated)]
    fn remote_zfs_service_exposes_metadata() {
        let cfg = RemoteConfig {
            endpoint: "http://127.0.0.1:65530".to_string(),
            timeout: Duration::from_millis(500),
            auth: Some("token".into()),
        };
        let s = RemoteZfsService::new(cfg);
        assert_eq!(s.service_name(), "remote-zfs");
        assert_eq!(s.service_version(), "1.0.0");
        assert_eq!(s.config().endpoint, "http://127.0.0.1:65530");
        assert!(s.config().auth.is_some());
    }

    #[test]
    fn remote_config_serde_roundtrip() {
        let cfg = RemoteConfig {
            endpoint: "https://zfs.example/api".into(),
            timeout: Duration::from_secs(30),
            auth: None,
        };
        let j = serde_json::to_string(&cfg).expect("serde");
        let back: RemoteConfig = serde_json::from_str(&j).expect("de");
        assert_eq!(back.endpoint, cfg.endpoint);
    }
}
