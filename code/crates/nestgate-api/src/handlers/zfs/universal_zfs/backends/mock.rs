//! Mock ZFS Backend
//!
//! Provides mock responses for testing and development without requiring actual ZFS.
//! This backend is fully deterministic and provides consistent responses.

use async_trait::async_trait;
use std::collections::HashMap;
use std::time::SystemTime;
use tokio::time::sleep;

use crate::handlers::zfs::universal_zfs::{
    traits::UniversalZfsService,
    types::{
        DatasetConfig, DatasetInfo, DatasetType, HealthCheck, HealthStatus, PoolCapacity,
        PoolConfig, PoolHealth, PoolInfo, PoolState, ScrubStatus, ServiceMetrics, ServiceStatus,
        SnapshotConfig, SnapshotInfo, UniversalZfsError, UniversalZfsResult,
    },
};
use std::time::Duration;

/// Mock ZFS service implementation
#[derive(Debug, Clone)]
pub struct MockZfsService {
    service_name: String,
    service_version: String,
    simulate_delays: bool,
    fail_operations: Vec<String>,
    pools: HashMap<String, PoolInfo>,
    datasets: HashMap<String, DatasetInfo>,
    snapshots: HashMap<String, SnapshotInfo>,
    start_time: SystemTime,
}

impl MockZfsService {
    /// Create a new mock service with default data
    pub fn new() -> Self {
        Self::with_config("mock-zfs", "1.0.0", false)
    }

    /// Create a mock service with custom configuration
    pub fn with_config(name: &str, version: &str, simulate_delays: bool) -> Self {
        let mut service = Self {
            service_name: name.to_string(),
            service_version: version.to_string(),
            simulate_delays,
            fail_operations: Vec::new(),
            pools: HashMap::new(),
            datasets: HashMap::new(),
            snapshots: HashMap::new(),
            start_time: SystemTime::now(),
        };

        service.initialize_mock_data();
        service
    }

    /// Create a mock service that fails specific operations
    pub fn with_failures(operations: Vec<String>) -> Self {
        let mut service = Self::new();
        service.fail_operations = operations;
        service
    }

    /// Initialize mock data
    fn initialize_mock_data(&mut self) {
        // Mock pools
        let pool1 = PoolInfo {
            name: "tank".to_string(),
            health: PoolHealth::Online,
            state: PoolState::Active,
            capacity: PoolCapacity {
                total_bytes: 1_000_000_000_000,   // 1TB
                used_bytes: 500_000_000_000,      // 500GB
                available_bytes: 500_000_000_000, // 500GB
                utilization_percent: 50.0,
            },
            devices: vec!["sda".to_string(), "sdb".to_string()],
            properties: {
                let mut props = HashMap::new();
                props.insert("compression".to_string(), "on".to_string());
                props.insert("deduplication".to_string(), "off".to_string());
                props
            },
            created_at: SystemTime::now() - Duration::from_secs(86400 * 30), // 30 days ago
            last_scrub: Some(SystemTime::now() - Duration::from_secs(86400 * 7)), // 7 days ago
            scrub_status: ScrubStatus::Completed { errors_found: 0 },
            errors: Vec::new(),
        };

        let pool2 = PoolInfo {
            name: "backup".to_string(),
            health: PoolHealth::Online,
            state: PoolState::Active,
            capacity: PoolCapacity {
                total_bytes: 2_000_000_000_000,     // 2TB
                used_bytes: 100_000_000_000,        // 100GB
                available_bytes: 1_900_000_000_000, // 1.9TB
                utilization_percent: 5.0,
            },
            devices: vec!["sdc".to_string(), "sdd".to_string(), "sde".to_string()],
            properties: {
                let mut props = HashMap::new();
                props.insert("compression".to_string(), "on".to_string());
                props.insert("deduplication".to_string(), "on".to_string());
                props
            },
            created_at: SystemTime::now() - Duration::from_secs(86400 * 60), // 60 days ago
            last_scrub: Some(SystemTime::now() - Duration::from_secs(86400 * 14)), // 14 days ago
            scrub_status: ScrubStatus::Completed { errors_found: 0 },
            errors: Vec::new(),
        };

        self.pools.insert("tank".to_string(), pool1);
        self.pools.insert("backup".to_string(), pool2);

        // Mock datasets
        let dataset1 = DatasetInfo {
            name: "tank/data".to_string(),
            dataset_type: DatasetType::Filesystem,
            used_space: 250_000_000_000,      // 250GB
            available_space: 750_000_000_000, // 750GB
            mount_point: Some("/tank/data".to_string()),
            properties: {
                let mut props = HashMap::new();
                props.insert("compression".to_string(), "on".to_string());
                props.insert("recordsize".to_string(), "128K".to_string());
                props
            },
            created_at: SystemTime::now() - Duration::from_secs(86400 * 20),
            parent: Some("tank".to_string()),
            children: Vec::new(),
        };

        let dataset2 = DatasetInfo {
            name: "backup/archive".to_string(),
            dataset_type: DatasetType::Filesystem,
            used_space: 50_000_000_000,         // 50GB
            available_space: 1_950_000_000_000, // 1.95TB
            mount_point: Some("/backup/archive".to_string()),
            properties: {
                let mut props = HashMap::new();
                props.insert("compression".to_string(), "on".to_string());
                props.insert("deduplication".to_string(), "on".to_string());
                props
            },
            created_at: SystemTime::now() - Duration::from_secs(86400 * 45),
            parent: Some("backup".to_string()),
            children: Vec::new(),
        };

        self.datasets.insert("tank/data".to_string(), dataset1);
        self.datasets.insert("backup/archive".to_string(), dataset2);

        // Mock snapshots
        let snapshot1 = SnapshotInfo {
            name: "tank/data@daily-2024-01-01".to_string(),
            dataset: "tank/data".to_string(),
            created_at: SystemTime::now() - Duration::from_secs(86400),
            size_bytes: 1_000_000_000, // 1GB
            properties: HashMap::new(),
            description: Some("Daily backup snapshot".to_string()),
        };

        self.snapshots
            .insert("tank/data@daily-2024-01-01".to_string(), snapshot1);
    }

    /// Simulate operation delay if enabled
    async fn simulate_delay(&self) {
        if self.simulate_delays {
            sleep(Duration::from_millis(100)).await;
        }
    }

    /// Check if operation should fail
    fn should_fail(&self, operation: &str) -> bool {
        self.fail_operations.contains(&operation.to_string())
    }
}

impl Default for MockZfsService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UniversalZfsService for MockZfsService {
    fn service_name(&self) -> &str {
        &self.service_name
    }

    fn service_version(&self) -> &str {
        &self.service_version
    }

    async fn health_check(&self) -> UniversalZfsResult<HealthStatus> {
        self.simulate_delay().await;

        if self.should_fail("health_check") {
            return Err(UniversalZfsError::service_unavailable(
                "Mock service unavailable",
            ));
        }

        let checks = vec![
            HealthCheck {
                name: "zfs_available".to_string(),
                status: ServiceStatus::Healthy,
                message: "Mock ZFS is available".to_string(),
                duration: Duration::from_millis(1),
            },
            HealthCheck {
                name: "pools_healthy".to_string(),
                status: ServiceStatus::Healthy,
                message: "All pools are healthy".to_string(),
                duration: Duration::from_millis(2),
            },
            HealthCheck {
                name: "datasets_healthy".to_string(),
                status: ServiceStatus::Healthy,
                message: "All datasets are healthy".to_string(),
                duration: Duration::from_millis(1),
            },
        ];

        Ok(HealthStatus {
            service_name: self.service_name.clone(),
            status: ServiceStatus::Healthy,
            last_check: SystemTime::now(),
            zfs_available: true,
            pools_healthy: true,
            datasets_healthy: true,
            system_healthy: true,
            checks,
            metrics: Some(self.get_metrics().await?),
        })
    }

    async fn get_metrics(&self) -> UniversalZfsResult<ServiceMetrics> {
        self.simulate_delay().await;

        Ok(ServiceMetrics {
            service_name: self.service_name.clone(),
            timestamp: SystemTime::now(),
            uptime: SystemTime::now()
                .duration_since(self.start_time)
                .unwrap_or_default(),
            requests_total: 1000,
            requests_successful: 995,
            requests_failed: 5,
            average_response_time: Duration::from_millis(50),
            error_rate: 0.5,
            circuit_breaker_state: "CLOSED".to_string(),
            active_connections: 10,
            custom_metrics: {
                let mut metrics = HashMap::new();
                metrics.insert("pools_count".to_string(), self.pools.len() as f64);
                metrics.insert("datasets_count".to_string(), self.datasets.len() as f64);
                metrics.insert("snapshots_count".to_string(), self.snapshots.len() as f64);
                metrics
            },
        })
    }

    async fn is_available(&self) -> bool {
        !self.should_fail("is_available")
    }

    async fn list_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        self.simulate_delay().await;

        if self.should_fail("list_pools") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated list_pools failure",
            ));
        }

        Ok(self.pools.values().cloned().collect())
    }

    async fn get_pool(&self, name: &str) -> UniversalZfsResult<Option<PoolInfo>> {
        self.simulate_delay().await;

        if self.should_fail("get_pool") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated get_pool failure",
            ));
        }

        if name == "nonexistent_pool" {
            return Ok(None);
        }

        Ok(self.pools.get(name).cloned())
    }

    async fn create_pool(&self, config: &PoolConfig) -> UniversalZfsResult<PoolInfo> {
        self.simulate_delay().await;

        if self.should_fail("create_pool") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated create_pool failure",
            ));
        }

        if config.name.is_empty() {
            return Err(UniversalZfsError::invalid_input(
                "name",
                "Pool name cannot be empty",
            ));
        }

        if config.devices.is_empty() {
            return Err(UniversalZfsError::invalid_input(
                "devices",
                "At least one device is required",
            ));
        }

        let pool = PoolInfo {
            name: config.name.clone(),
            health: PoolHealth::Online,
            state: PoolState::Active,
            capacity: PoolCapacity {
                total_bytes: 1_000_000_000_000, // 1TB
                used_bytes: 0,
                available_bytes: 1_000_000_000_000,
                utilization_percent: 0.0,
            },
            devices: config.devices.clone(),
            properties: config.properties.clone(),
            created_at: SystemTime::now(),
            last_scrub: None,
            scrub_status: ScrubStatus::None,
            errors: Vec::new(),
        };

        Ok(pool)
    }

    async fn destroy_pool(&self, name: &str) -> UniversalZfsResult<()> {
        self.simulate_delay().await;

        if self.should_fail("destroy_pool") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated destroy_pool failure",
            ));
        }

        if !self.pools.contains_key(name) {
            return Err(UniversalZfsError::not_found("pool", name));
        }

        Ok(())
    }

    async fn scrub_pool(&self, name: &str) -> UniversalZfsResult<()> {
        self.simulate_delay().await;

        if self.should_fail("scrub_pool") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated scrub_pool failure",
            ));
        }

        if !self.pools.contains_key(name) {
            return Err(UniversalZfsError::not_found("pool", name));
        }

        Ok(())
    }

    async fn get_pool_status(&self, name: &str) -> UniversalZfsResult<String> {
        self.simulate_delay().await;

        if self.should_fail("get_pool_status") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated get_pool_status failure",
            ));
        }

        if !self.pools.contains_key(name) {
            return Err(UniversalZfsError::not_found("pool", name));
        }

        Ok(format!("Pool '{name}' is ONLINE"))
    }

    async fn list_datasets(&self) -> UniversalZfsResult<Vec<DatasetInfo>> {
        self.simulate_delay().await;

        if self.should_fail("list_datasets") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated list_datasets failure",
            ));
        }

        Ok(self.datasets.values().cloned().collect())
    }

    async fn get_dataset(&self, name: &str) -> UniversalZfsResult<Option<DatasetInfo>> {
        self.simulate_delay().await;

        if self.should_fail("get_dataset") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated get_dataset failure",
            ));
        }

        if name == "nonexistent_dataset" {
            return Ok(None);
        }

        Ok(self.datasets.get(name).cloned())
    }

    async fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo> {
        self.simulate_delay().await;

        if self.should_fail("create_dataset") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated create_dataset failure",
            ));
        }

        if config.name.is_empty() {
            return Err(UniversalZfsError::invalid_input(
                "name",
                "Dataset name cannot be empty",
            ));
        }

        let dataset = DatasetInfo {
            name: config.name.clone(),
            dataset_type: config.dataset_type.clone(),
            used_space: 0,
            available_space: 1_000_000_000_000, // 1TB
            mount_point: Some(format!("/{}", config.name)),
            properties: config.properties.clone(),
            created_at: SystemTime::now(),
            parent: config.parent.clone(),
            children: Vec::new(),
        };

        Ok(dataset)
    }

    async fn destroy_dataset(&self, name: &str) -> UniversalZfsResult<()> {
        self.simulate_delay().await;

        if self.should_fail("destroy_dataset") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated destroy_dataset failure",
            ));
        }

        if !self.datasets.contains_key(name) {
            return Err(UniversalZfsError::not_found("dataset", name));
        }

        Ok(())
    }

    async fn get_dataset_properties(
        &self,
        name: &str,
    ) -> UniversalZfsResult<HashMap<String, String>> {
        self.simulate_delay().await;

        if self.should_fail("get_dataset_properties") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated get_dataset_properties failure",
            ));
        }

        if name == "nonexistent" {
            return Err(UniversalZfsError::not_found("dataset", name));
        }

        let mut properties = HashMap::new();
        properties.insert("compression".to_string(), "on".to_string());
        properties.insert("deduplication".to_string(), "off".to_string());
        properties.insert("encryption".to_string(), "off".to_string());
        properties.insert("recordsize".to_string(), "128K".to_string());

        Ok(properties)
    }

    async fn set_dataset_properties(
        &self,
        name: &str,
        properties: &HashMap<String, String>,
    ) -> UniversalZfsResult<()> {
        self.simulate_delay().await;

        if self.should_fail("set_dataset_properties") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated set_dataset_properties failure",
            ));
        }

        if name == "nonexistent" {
            return Err(UniversalZfsError::not_found("dataset", name));
        }

        if properties.is_empty() {
            return Err(UniversalZfsError::invalid_input(
                "properties",
                "Properties cannot be empty",
            ));
        }

        Ok(())
    }

    async fn list_snapshots(&self) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        self.simulate_delay().await;

        if self.should_fail("list_snapshots") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated list_snapshots failure",
            ));
        }

        Ok(self.snapshots.values().cloned().collect())
    }

    async fn list_dataset_snapshots(&self, dataset: &str) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        self.simulate_delay().await;

        if self.should_fail("list_dataset_snapshots") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated list_dataset_snapshots failure",
            ));
        }

        let snapshots: Vec<SnapshotInfo> = self
            .snapshots
            .values()
            .filter(|s| s.dataset == dataset)
            .cloned()
            .collect();

        Ok(snapshots)
    }

    async fn create_snapshot(&self, config: &SnapshotConfig) -> UniversalZfsResult<SnapshotInfo> {
        self.simulate_delay().await;

        if self.should_fail("create_snapshot") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated create_snapshot failure",
            ));
        }

        if config.name.is_empty() {
            return Err(UniversalZfsError::invalid_input(
                "name",
                "Snapshot name cannot be empty",
            ));
        }

        if config.dataset.is_empty() {
            return Err(UniversalZfsError::invalid_input(
                "dataset",
                "Dataset name cannot be empty",
            ));
        }

        let snapshot = SnapshotInfo {
            name: format!("{}@{}", config.dataset, config.name),
            dataset: config.dataset.clone(),
            created_at: SystemTime::now(),
            size_bytes: 1_000_000_000, // 1GB
            properties: config.properties.clone(),
            description: config.description.clone(),
        };

        Ok(snapshot)
    }

    async fn destroy_snapshot(&self, name: &str) -> UniversalZfsResult<()> {
        self.simulate_delay().await;

        if self.should_fail("destroy_snapshot") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated destroy_snapshot failure",
            ));
        }

        if !self.snapshots.contains_key(name) {
            return Err(UniversalZfsError::not_found("snapshot", name));
        }

        Ok(())
    }

    async fn optimize(&self) -> UniversalZfsResult<String> {
        self.simulate_delay().await;

        if self.should_fail("optimize") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated optimize failure",
            ));
        }

        Ok("opt-123456789".to_string())
    }

    async fn get_optimization_analytics(&self) -> UniversalZfsResult<serde_json::Value> {
        self.simulate_delay().await;

        if self.should_fail("get_optimization_analytics") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated get_optimization_analytics failure",
            ));
        }

        Ok(serde_json::json!({
            "tier_performance": {
                "hot": {"iops": 1000, "latency_ms": 1.5},
                "warm": {"iops": 500, "latency_ms": 3.0},
                "cold": {"iops": 100, "latency_ms": 10.0}
            },
            "recommendations": [
                "Consider moving cold data to warm tier",
                "Enable compression on high-volume datasets"
            ]
        }))
    }

    async fn predict_tier(&self, file_path: &str) -> UniversalZfsResult<String> {
        self.simulate_delay().await;

        if self.should_fail("predict_tier") {
            return Err(UniversalZfsError::backend(
                "mock",
                "Simulated predict_tier failure",
            ));
        }

        if file_path.is_empty() {
            return Err(UniversalZfsError::invalid_input(
                "file_path",
                "File path cannot be empty",
            ));
        }

        // Mock prediction logic based on file path
        let predicted_tier = if file_path.contains("cache") || file_path.contains("temp") {
            "hot"
        } else if file_path.contains("archive") || file_path.contains("backup") {
            "cold"
        } else {
            "warm"
        };

        Ok(predicted_tier.to_string())
    }

    async fn get_configuration(&self) -> UniversalZfsResult<serde_json::Value> {
        self.simulate_delay().await;

        Ok(serde_json::json!({
            "service_name": self.service_name,
            "service_version": self.service_version,
            "backend": "mock",
            "simulate_delays": self.simulate_delays,
            "pools_count": self.pools.len(),
            "datasets_count": self.datasets.len(),
            "snapshots_count": self.snapshots.len()
        }))
    }

    async fn update_configuration(&self, _config: serde_json::Value) -> UniversalZfsResult<()> {
        self.simulate_delay().await;
        Ok(())
    }

    async fn shutdown(&self) -> UniversalZfsResult<()> {
        self.simulate_delay().await;
        Ok(())
    }
}
