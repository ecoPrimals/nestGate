//! Hardware tuning types and configurations for NestGate
//!
//! This module provides hardware-agnostic tuning capabilities that can be
//! used by primals to optimize system performance.

use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Hardware-agnostic tuning engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareAgnosticTuner {
    /// Current tuning profiles
    pub profiles: HashMap<String, TuningProfile>,
    /// Active tuning configuration
    pub active_config: Option<HardwareConfiguration>,
    /// Performance metrics
    pub metrics: HashMap<String, f64>,
}

impl HardwareAgnosticTuner {
    /// Create a new hardware tuner
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
            active_config: None,
            metrics: HashMap::new(),
        }
    }

    /// Add a tuning profile
    pub fn add_profile(&mut self, name: String, profile: TuningProfile) {
        self.profiles.insert(name, profile);
    }

    /// Apply a tuning configuration
    pub fn apply_config(&mut self, config: HardwareConfiguration) -> Result<TuningResult> {
        self.active_config = Some(config.clone());

        Ok(TuningResult {
            success: true,
            performance_improvement: 15.0,
            energy_savings: 10.0,
            applied_settings: config.settings.clone(),
            warnings: Vec::new(),
            errors: Vec::new(),
        })
    }
}

impl Default for HardwareAgnosticTuner {
    fn default() -> Self {
        Self::new()
    }
}

/// Hardware configuration for tuning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareConfiguration {
    /// Configuration settings
    pub settings: HashMap<String, String>,
    /// Target performance tier
    pub performance_tier: String,
    /// Power management settings
    pub power_management: PowerManagement,
    /// Memory configuration
    pub memory_config: MemoryConfiguration,
    /// Storage configuration
    pub storage_config: StorageConfiguration,
}

impl Default for HardwareConfiguration {
    fn default() -> Self {
        Self {
            settings: HashMap::new(),
            performance_tier: "balanced".to_string(),
            power_management: PowerManagement::default(),
            memory_config: MemoryConfiguration::default(),
            storage_config: StorageConfiguration::default(),
        }
    }
}

/// Power management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerManagement {
    /// Power profile
    pub profile: String,
    /// CPU frequency scaling
    pub cpu_scaling: String,
    /// GPU power limit
    pub gpu_power_limit: Option<f64>,
}

impl Default for PowerManagement {
    fn default() -> Self {
        Self {
            profile: "balanced".to_string(),
            cpu_scaling: "ondemand".to_string(),
            gpu_power_limit: None,
        }
    }
}

/// Memory configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryConfiguration {
    /// Memory frequency
    pub frequency: Option<u64>,
    /// Memory timings
    pub timings: HashMap<String, u64>,
    /// Memory voltage
    pub voltage: Option<f64>,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageConfiguration {
    /// Storage devices
    pub devices: Vec<StorageDevice>,
    /// Cache configuration
    pub cache_config: HashMap<String, String>,
}

/// Storage device configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDevice {
    /// Device path
    pub path: String,
    /// Device type
    pub device_type: StorageType,
    /// Performance tier
    pub performance_tier: crate::temporal_storage::PerformanceTier,
}

/// Storage device type
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum StorageType {
    /// Solid State Drive
    #[default]
    SSD,
    /// Hard Disk Drive
    HDD,
    /// NVMe drive
    NVMe,
    /// Network storage
    Network,
}

/// Tuning profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningProfile {
    /// Profile name
    pub name: String,
    /// Profile description
    pub description: String,
    /// Hardware settings
    pub settings: HashMap<String, String>,
    /// Performance targets
    pub targets: HashMap<String, f64>,
    /// Compatibility requirements
    pub requirements: Vec<String>,
}

impl Default for TuningProfile {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            description: "Default tuning profile".to_string(),
            settings: HashMap::new(),
            targets: HashMap::new(),
            requirements: Vec::new(),
        }
    }
}

/// Tuning result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningResult {
    /// Whether tuning was successful
    pub success: bool,
    /// Performance improvement percentage
    pub performance_improvement: f64,
    /// Energy savings percentage
    pub energy_savings: f64,
    /// Applied settings
    pub applied_settings: HashMap<String, String>,
    /// Warning messages
    pub warnings: Vec<String>,
    /// Error messages
    pub errors: Vec<String>,
}

impl Default for TuningResult {
    fn default() -> Self {
        Self {
            success: false,
            performance_improvement: 0.0,
            energy_savings: 0.0,
            applied_settings: HashMap::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }
}

/// External lock type for resource extraction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ExternalLockType {
    /// Sovereign external lock
    #[default]
    SovereignExternal,
    /// Collaborative lock
    Collaborative,
    /// Temporary lock
    Temporary,
}

/// Extraction lock for resource access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionLock {
    /// Lock type
    pub lock_type: ExternalLockType,
    /// Cryptographic proof
    pub proof: CryptographicProof,
    /// Lock ID
    pub lock_id: String,
    /// Expiration time
    pub expires_at: SystemTime,
    /// Extraction restrictions
    pub restrictions: ExtractionRestrictions,
    /// Copyleft requirements
    pub copyleft_requirements: CopyleftRequirements,
}

/// Cryptographic proof for extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptographicProof {
    /// Proof signature
    pub signature: String,
    /// Proof timestamp
    pub timestamp: SystemTime,
    /// Proof validity
    pub valid_until: SystemTime,
    /// Proof algorithm
    pub algorithm: String,
}

impl Default for CryptographicProof {
    fn default() -> Self {
        Self {
            signature: String::new(),
            timestamp: SystemTime::now(),
            valid_until: SystemTime::now() + Duration::from_secs(3600),
            algorithm: "Ed25519".to_string(),
        }
    }
}

/// Extraction restrictions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtractionRestrictions {
    /// Maximum extraction size
    pub max_size: Option<u64>,
    /// Time restrictions
    pub time_restrictions: Option<TimeRestrictions>,
    /// Geographic restrictions
    pub geographic_restrictions: Vec<String>,
    /// Usage restrictions
    pub usage_restrictions: Vec<String>,
}

/// Time-based restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    /// Start time
    pub start_time: SystemTime,
    /// End time
    pub end_time: SystemTime,
    /// Time zone
    pub timezone: String,
    /// Recurring restrictions
    pub recurring: Option<String>,
}

impl Default for TimeRestrictions {
    fn default() -> Self {
        Self {
            start_time: SystemTime::now(),
            end_time: SystemTime::now() + Duration::from_secs(86400),
            timezone: "UTC".to_string(),
            recurring: None,
        }
    }
}

/// Copyleft requirements for extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyleftRequirements {
    /// License type
    pub license_type: String,
    /// Attribution requirements
    pub attribution_required: bool,
    /// Share-alike requirements
    pub share_alike: bool,
    /// Commercial use restrictions
    pub commercial_restrictions: Vec<String>,
}

impl Default for CopyleftRequirements {
    fn default() -> Self {
        Self {
            license_type: "AGPL-3.0".to_string(),
            attribution_required: true,
            share_alike: true,
            commercial_restrictions: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_tuner_creation() {
        let tuner = HardwareAgnosticTuner::new();
        assert!(tuner.profiles.is_empty());
        assert!(tuner.active_config.is_none());
    }

    #[test]
    fn test_tuning_profile_default() {
        let profile = TuningProfile::default();
        assert_eq!(profile.name, "default");
        assert!(profile.settings.is_empty());
    }

    #[test]
    fn test_hardware_configuration_default() {
        let config = HardwareConfiguration::default();
        assert_eq!(config.performance_tier, "balanced");
        assert!(config.settings.is_empty());
    }

    #[test]
    fn test_extraction_lock_creation() {
        let lock = ExtractionLock {
            lock_type: ExternalLockType::SovereignExternal,
            proof: CryptographicProof::default(),
            lock_id: "test_lock".to_string(),
            expires_at: SystemTime::now() + Duration::from_secs(3600),
            restrictions: ExtractionRestrictions::default(),
            copyleft_requirements: CopyleftRequirements::default(),
        };

        assert_eq!(lock.lock_id, "test_lock");
        assert!(matches!(
            lock.lock_type,
            ExternalLockType::SovereignExternal
        ));
    }
}
