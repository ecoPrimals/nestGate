//! Hardware Test Doubles
//!
//! Pure test mocks for hardware functionality testing.
//! These simulate hardware detection, failures, and performance characteristics.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::TestDoubleConfig;

/// Hardware test double for testing hardware detection and operations
pub struct HardwareTestDouble {
    config: TestDoubleConfig,
    detected_hardware: Arc<Mutex<HashMap<String, HardwareInfo>>>,
    operations: Arc<Mutex<Vec<String>>>,
}

#[derive(Debug, Clone)]
struct HardwareInfo {
    device_type: String,
    status: HardwareStatus,
    capabilities: Vec<String>,
}

#[derive(Debug, Clone)]
enum HardwareStatus {
    Available,
    Unavailable,
    Failed,
}

impl HardwareTestDouble {
    pub fn new(config: TestDoubleConfig) -> Self {
        Self {
            config,
            detected_hardware: Arc::new(Mutex::new(HashMap::new())),
            operations: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn fake_detect_hardware(&self, device_type: &str) -> Result<bool, HardwareTestError> {
        self.record_operation(&format!("detect_hardware:{}", device_type))
            .await?;

        // Simulate hardware detection logic
        let available = match device_type {
            "zfs" => false, // Simulate ZFS not available in test environment
            "disk" => true, // Simulate disk always available
            _ => false,
        };

        if let Ok(mut hardware) = self.detected_hardware.lock() {
            hardware.insert(
                device_type.to_string(),
                HardwareInfo {
                    device_type: device_type.to_string(),
                    status: if available {
                        HardwareStatus::Available
                    } else {
                        HardwareStatus::Unavailable
                    },
                    capabilities: vec!["test".to_string()],
                },
            );
        }

        Ok(available)
    }

    pub fn get_operations(&self) -> Vec<String> {
        self.operations.lock().unwrap().clone()
    }

    async fn record_operation(&self, operation: &str) -> Result<(), HardwareTestError> {
        if let Ok(mut ops) = self.operations.lock() {
            ops.push(operation.to_string());
        }
        Ok(())
    }
}

/// Mock hardware for testing
pub struct MockHardwareForTesting {
    test_double: HardwareTestDouble,
}

impl MockHardwareForTesting {
    pub fn new() -> Self {
        Self {
            test_double: HardwareTestDouble::new(TestDoubleConfig::default()),
        }
    }

    pub async fn fake_initialize_hardware(
        &self,
        device_type: &str,
    ) -> Result<(), HardwareTestError> {
        self.test_double
            .record_operation(&format!("initialize_hardware:{}", device_type))
            .await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HardwareTestError {
    #[error("Simulated hardware failure: {0}")]
    SimulatedFailure(String),

    #[error("Hardware not detected")]
    NotDetected,
}
