// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Hardware Test Doubles
//!
//! Pure test mocks for hardware functionality testing.
//! These simulate hardware detection, failures, and performance characteristics.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::TestDoubleConfig;

/// Hardware test double for testing hardware detection and operations
pub struct HardwareTestDouble {
    #[allow(dead_code)] // Test fixture field
    config: TestDoubleConfig,
    detected_hardware: Arc<Mutex<HashMap<String, HardwareInfo>>>,
    operations: Arc<Mutex<Vec<String>>>,
    pub call_count: Arc<Mutex<u32>>, // Added for compatibility
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // Test fixture
struct HardwareInfo {
    device_type: String,
    status: HardwareStatus,
    capabilities: Vec<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // Test fixture
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
            call_count: Arc::new(Mutex::new(0)),
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
        self.operations
            .lock()
            .map(|ops| ops.clone())
            .unwrap_or_default()
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

impl Default for MockHardwareForTesting {
    fn default() -> Self {
        Self::new()
    }
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

    /// Initialize the mock hardware
    pub fn initialize(&mut self) -> Result<(), HardwareTestError> {
        Ok(())
    }

    /// Cleanup the mock hardware
    pub fn cleanup(&mut self) -> Result<(), String> {
        self.reset()
    }

    /// Reset hardware mock to initial state
    pub fn reset(&mut self) -> Result<(), String> {
        // Reset the test double state
        if let Ok(mut ops) = self.test_double.operations.lock() {
            ops.clear();
        }
        if let Ok(mut calls) = self.test_double.call_count.lock() {
            *calls = 0;
        }
        Ok(())
    }
}

// Simple error type for test doubles - no thiserror needed in tests
#[derive(Debug)]
pub enum HardwareTestError {
    SimulatedFailure(String),
    NotDetected,
}

impl std::fmt::Display for HardwareTestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SimulatedFailure(msg) => write!(f, "Simulated hardware failure: {}", msg),
            Self::NotDetected => write!(f, "Hardware not detected"),
        }
    }
}

impl std::error::Error for HardwareTestError {}
