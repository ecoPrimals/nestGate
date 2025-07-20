//! Toadstool Compute Client
//!
//! This module provides the client implementation for interacting with
//! Toadstool compute services for hardware tuning operations.

use chrono::Utc;
use serde_json::json;
use tracing::{error, info};

use super::types::*;
use nestgate_core::NestGateError;

type Result<T> = std::result::Result<T, NestGateError>;

/// Toadstool compute organization client for hardware tuning
#[derive(Debug, Clone)]
pub struct ToadstoolComputeClient {
    /// Base URL for Toadstool compute service
    pub base_url: String,
    /// HTTP client for API requests
    client: reqwest::Client,
    /// Service authentication
    auth_token: Option<String>,
}

impl ToadstoolComputeClient {
    /// Create new Toadstool compute client
    pub fn new(base_url: String) -> Self {
        info!("🐸 Creating Toadstool Compute Client");
        info!("🐸 Toadstool URL: {}", base_url);

        Self {
            base_url,
            client: reqwest::Client::new(),
            auth_token: None,
        }
    }

    /// Create new Toadstool compute client with authentication
    pub fn new_with_auth(base_url: String, auth_token: String) -> Self {
        info!("🐸 Creating Toadstool Compute Client with authentication");
        info!("🐸 Toadstool URL: {}", base_url);

        Self {
            base_url,
            client: reqwest::Client::new(),
            auth_token: Some(auth_token),
        }
    }

    /// Add authentication header to request if token is available
    fn add_auth_header(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(token) = &self.auth_token {
            request.header("Authorization", format!("Bearer {token}"))
        } else {
            request
        }
    }

    /// Register hardware tuning service with Toadstool
    pub async fn register_tuning_service(&self, service: &TuningServiceRegistration) -> Result<()> {
        info!(
            "🐸 Registering hardware tuning service with Toadstool: {}",
            service.name
        );

        let request = self
            .client
            .post(format!("{}/compute/services/register", self.base_url))
            .json(service);

        let response = self.add_auth_header(request).send().await?;

        if response.status().is_success() {
            info!(
                "✅ Hardware tuning service registered with Toadstool: {}",
                service.name
            );
            Ok(())
        } else {
            let error = response.text().await?;
            error!("❌ Failed to register with Toadstool: {}", error);
            Err(NestGateError::Internal(format!(
                "Toadstool registration failed: {error}"
            )))
        }
    }

    /// Request compute resources from Toadstool
    pub async fn request_compute_resources(
        &self,
        request: &ComputeResourceRequest,
    ) -> Result<ComputeAllocation> {
        info!("🐸 Requesting compute resources from Toadstool");

        let response = self
            .client
            .post(format!("{}/compute/resources/allocate", self.base_url))
            .json(request)
            .send()
            .await?;

        if response.status().is_success() {
            let allocation: ComputeAllocation = response.json().await?;
            info!(
                "✅ Compute resources allocated by Toadstool: {} cores, {} GB RAM",
                allocation.cpu_cores, allocation.memory_gb
            );
            Ok(allocation)
        } else {
            let error = response.text().await?;
            error!("❌ Failed to allocate compute resources: {}", error);
            Err(NestGateError::Internal(format!(
                "Toadstool allocation failed: {error}"
            )))
        }
    }

    /// Get live hardware metrics from Toadstool
    pub async fn get_live_hardware_metrics(&self) -> Result<LiveHardwareMetrics> {
        let response = self
            .client
            .get(format!("{}/compute/metrics/live", self.base_url))
            .send()
            .await?;

        if response.status().is_success() {
            let metrics: LiveHardwareMetrics = response.json().await?;
            Ok(metrics)
        } else {
            let error = response.text().await?;
            Err(NestGateError::Internal(format!(
                "Failed to get live metrics: {error}"
            )))
        }
    }

    /// Subscribe to live hardware data feed
    pub async fn subscribe_to_hardware_feed(
        &self,
        callback: Box<dyn Fn(HardwareEvent) + Send + Sync>,
    ) -> Result<()> {
        info!("🐸 Subscribing to Toadstool hardware data feed");

        // Set up WebSocket connection for live feeds
        let _ws_url = format!(
            "{}/compute/metrics/stream",
            self.base_url.replace("http", "ws")
        );

        // In a real implementation, this would establish a WebSocket connection
        // and handle incoming hardware events
        tokio::spawn(async move {
            // This is where the live data feed would be processed
            // For now, simulate with periodic updates
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

            loop {
                interval.tick().await;

                // In production, this would be real data from Toadstool
                let event = HardwareEvent {
                    timestamp: Utc::now(),
                    event_type: HardwareEventType::MetricsUpdate,
                    data: json!({
                        "cpu_usage": 45.2,
                        "memory_usage": 67.8,
                        "temperature": 42.1
                    }),
                };

                callback(event);
            }
        });

        Ok(())
    }

    /// Release compute resources back to Toadstool
    pub async fn release_compute_resources(&self, allocation_id: &str) -> Result<()> {
        info!(
            "🐸 Releasing compute resources to Toadstool: {}",
            allocation_id
        );

        let response = self
            .client
            .delete(format!(
                "{}/compute/resources/{}",
                self.base_url, allocation_id
            ))
            .send()
            .await?;

        if response.status().is_success() {
            info!(
                "✅ Compute resources released to Toadstool: {}",
                allocation_id
            );
            Ok(())
        } else {
            let error = response.text().await?;
            Err(NestGateError::Internal(format!(
                "Failed to release resources: {error}"
            )))
        }
    }

    /// Get platform information from ToadStool
    pub async fn get_platform_info(&self) -> Result<PlatformInfo> {
        info!("🐸 Getting platform information from ToadStool");

        let response = self
            .client
            .get(format!("{}/sysinfo/platform", self.base_url))
            .send()
            .await?;

        if response.status().is_success() {
            let platform_info: PlatformInfo = response.json().await?;
            info!(
                "✅ Platform detected: {} cores, {} GB RAM, {} storage devices",
                platform_info.cpu_cores,
                platform_info.memory_gb,
                platform_info.storage_devices.len()
            );
            Ok(platform_info)
        } else {
            let error = response.text().await?;
            Err(NestGateError::Internal(format!(
                "Failed to get platform info: {error}"
            )))
        }
    }

    /// Get real-time system metrics
    pub async fn get_realtime_metrics(&self) -> Result<RealtimeMetrics> {
        let response = self
            .client
            .get(format!("{}/sysinfo/metrics/realtime", self.base_url))
            .send()
            .await?;

        if response.status().is_success() {
            let metrics: RealtimeMetrics = response.json().await?;
            Ok(metrics)
        } else {
            let error = response.text().await?;
            Err(NestGateError::Internal(format!(
                "Failed to get realtime metrics: {error}"
            )))
        }
    }

    /// Discover available compute resources
    pub async fn discover_compute_resources(&self) -> Result<ComputeDiscovery> {
        info!("🔍 Discovering available compute resources via ToadStool");

        let response = self
            .client
            .get(format!("{}/sysinfo/compute/discovery", self.base_url))
            .send()
            .await?;

        if response.status().is_success() {
            let discovery: ComputeDiscovery = response.json().await?;
            info!("✅ Discovered {} compute nodes", discovery.nodes.len());
            Ok(discovery)
        } else {
            let error = response.text().await?;
            Err(NestGateError::Internal(format!(
                "Failed to discover compute resources: {error}"
            )))
        }
    }

    /// Get system health information
    pub async fn get_system_health(&self) -> Result<SystemHealth> {
        let response = self
            .client
            .get(format!("{}/sysinfo/health", self.base_url))
            .send()
            .await?;

        if response.status().is_success() {
            let health: SystemHealth = response.json().await?;
            Ok(health)
        } else {
            let error = response.text().await?;
            Err(NestGateError::Internal(format!(
                "Failed to get system health: {error}"
            )))
        }
    }

    /// Execute storage workload
    pub async fn execute_storage_workload(
        &self,
        workload: &StorageWorkload,
    ) -> Result<WorkloadExecution> {
        info!(
            "🚀 Executing storage workload via ToadStool: {}",
            workload.workload_id
        );

        let response = self
            .client
            .post(format!("{}/compute/workload/execute", self.base_url))
            .json(workload)
            .send()
            .await?;

        if response.status().is_success() {
            let execution: WorkloadExecution = response.json().await?;
            info!("✅ Storage workload '{}' executing", workload.workload_id);
            Ok(execution)
        } else {
            let error = response.text().await?;
            Err(NestGateError::Internal(format!(
                "Failed to execute storage workload: {error}"
            )))
        }
    }

    /// Allocate storage resources
    pub async fn allocate_storage_resources(
        &self,
        allocation_request: &StorageResourceRequest,
    ) -> Result<StorageResourceAllocation> {
        info!("💾 Allocating storage resources via ToadStool");

        let response = self
            .client
            .post(format!(
                "{}/compute/resources/storage/allocate",
                self.base_url
            ))
            .json(allocation_request)
            .send()
            .await?;

        if response.status().is_success() {
            let allocation: StorageResourceAllocation = response.json().await?;
            info!(
                "✅ Storage resources allocated: {} quantity",
                allocation.allocated_quantity
            );
            Ok(allocation)
        } else {
            let error = response.text().await?;
            Err(NestGateError::Internal(format!(
                "Failed to allocate storage resources: {error}"
            )))
        }
    }

    /// Manage storage process
    pub async fn manage_storage_process(
        &self,
        process_request: &StorageProcessRequest,
    ) -> Result<ProcessManagement> {
        info!(
            "⚙️ Managing storage process via ToadStool: {}",
            process_request.process_name
        );

        let response = self
            .client
            .post(format!("{}/compute/process/manage", self.base_url))
            .json(process_request)
            .send()
            .await?;

        if response.status().is_success() {
            let management: ProcessManagement = response.json().await?;
            info!(
                "✅ Storage process '{}' managed: {}",
                process_request.process_name, management.status
            );
            Ok(management)
        } else {
            let error = response.text().await?;
            Err(NestGateError::Internal(format!(
                "Failed to manage storage process: {error}"
            )))
        }
    }

    /// Optimize storage performance
    pub async fn optimize_storage_performance(
        &self,
        optimization_request: &StorageOptimizationRequest,
    ) -> Result<StorageOptimization> {
        info!("🔧 Optimizing storage performance via ToadStool");

        let response = self
            .client
            .post(format!("{}/compute/optimization/storage", self.base_url))
            .json(optimization_request)
            .send()
            .await?;

        if response.status().is_success() {
            let optimization: StorageOptimization = response.json().await?;
            info!(
                "✅ Storage performance optimized: {}% improvement",
                optimization.performance_improvement
            );
            Ok(optimization)
        } else {
            let error = response.text().await?;
            Err(NestGateError::Internal(format!(
                "Failed to optimize storage performance: {error}"
            )))
        }
    }
}
