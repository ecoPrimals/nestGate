//! **RESOURCES MANAGEMENT**
//!
//! Resource specification and management for ecosystem services.

use std::collections::HashMap;
use super::types::{ResourceSpec, CpuSpec, MemorySpec, StorageSpec, NetworkSpec};

impl Default for ResourceSpec {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            cpu: None,
            memory: None,
            storage: None,
            network: None,
            custom: HashMap::new(),
        }
    }
}

impl Default for CpuSpec {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            min_cores: Some(1.0),
            max_cores: None,
            architecture: None,
            features: Vec::new(),
        }
    }
}

impl Default for MemorySpec {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            min_bytes: Some(512 * 1024 * 1024), // 512MB
            max_bytes: None,
            memory_type: None,
        }
    }
}

impl Default for StorageSpec {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            min_bytes: Some(1024 * 1024 * 1024), // 1GB
            storage_type: None,
            iops: None,
            durability: None,
        }
    }
}

impl Default for NetworkSpec {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            bandwidth: None,
            latency: None,
            protocols: vec!["HTTP".to_string(), "HTTPS".to_string()],
        }
    }
}

/// Resource validator
pub struct ResourceValidator;
impl ResourceValidator {
    /// Validate resource specification
    pub fn validate(spec: &ResourceSpec) -> Vec<String> {
        let mut errors = Vec::new();

        if let Some(cpu) = &spec.cpu {
            if let (Some(min), Some(max)) = (cpu.min_cores, cpu.max_cores) {
                if min > max {
                    errors.push("CPU min_cores cannot be greater than max_cores".to_string());
                }
            }
        }

        if let Some(memory) = &spec.memory {
            if let (Some(min), Some(max)) = (memory.min_bytes, memory.max_bytes) {
                if min > max {
                    errors.push("Memory min_bytes cannot be greater than max_bytes".to_string());
                }
            }
        }

        errors
    }
} 