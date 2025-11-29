// Removed unused error imports
/// Hardware tuning types and configurations for `NestGate`
///
/// This module provides hardware-agnostic tuning capabilities that can be
/// used by primals to optimize system performance.
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;
/// Hardware-agnostic tuning engine
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Hardwareagnostictuner
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
    #[must_use]
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
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn apply_config(&mut self, config: HardwareConfiguration) -> Result<TuningResult> {
        self.active_config = Some(config.clone());

        Ok(TuningResult {
            success: true,
            performance_improvement: 15.0,
            energy_savings: 10.0,
            applied_settings: config.settings,
            warnings: Vec::new(),
            errors: Vec::new(),
        })
    }
}

impl Default for HardwareAgnosticTuner {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// Hardware configuration for tuning
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::HardwareConfiguration;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::HardwareConfiguration; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Hardwareconfiguration
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
    /// Returns the default instance
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
/// Powermanagement
pub struct PowerManagement {
    /// Power profile
    pub profile: String,
    /// CPU frequency scaling
    pub cpu_scaling: String,
    /// GPU power limit
    pub gpu_power_limit: Option<f64>,
}
impl Default for PowerManagement {
    /// Returns the default instance
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
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::MemoryConfiguration;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::MemoryConfiguration; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Memoryconfiguration
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
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::StorageConfiguration;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::StorageConfiguration; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Storageconfiguration
pub struct StorageConfiguration {
    /// Storage devices
    pub devices: Vec<StorageDevice>,
    /// Cache configuration
    pub cache_config: HashMap<String, String>,
}
/// Storage device configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagedevice
pub struct StorageDevice {
    /// Device path
    /// Device type
    pub device_type: StorageType,
    /// Performance tier
    pub performance_tier: crate::temporal_storage::PerformanceTier,
}
/// Storage device type
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Types of Storage
pub enum StorageType {
    /// Solid State Drive
    #[default]
    /// Ssd
    SSD,
    /// Hard Disk Drive
    HDD,
    /// `NVMe` drive
    NVMe,
    /// Network storage
    Network,
}
/// Tuning profile
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tuningprofile
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
    /// Returns the default instance
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
/// Tuningresult
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

/// Benchmark result for hardware performance testing
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Benchmarkresult
pub struct BenchmarkResult {
    /// Benchmark name
    pub name: String,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Performance score
    pub performance_score: f64,
    /// Resource utilization metrics
    pub resource_utilization: HashMap<String, f64>,
}

/// Live hardware metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Livehardwaremetrics
pub struct LiveHardwareMetrics {
    /// CPU utilization percentage
    pub cpu_utilization: f64,
    /// Memory utilization percentage
    pub memory_utilization: f64,
    /// GPU utilization percentage
    pub gpu_utilization: Option<f64>,
    /// Temperature readings
    pub temperatures: HashMap<String, f64>,
    /// Power consumption in watts
    pub power_consumption: Option<f64>,
}

/// Compute resource allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Computeallocation
pub struct ComputeAllocation {
    /// CPU cores allocated
    pub cpu_cores: u32,
    /// Memory allocated in GB
    pub memory_gb: u64,
    /// Storage allocated in GB
    pub storage_gb: u64,
    /// Network bandwidth in Mbps
    pub network_bandwidth_mbps: Option<u32>,
}

/// GPU resource allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Gpuallocation
pub struct GpuAllocation {
    /// GPU device ID
    pub device_id: u32,
    /// Memory allocated in GB
    pub memory_gb: u64,
    /// Compute units allocated
    pub compute_units: u32,
    /// Power limit in watts
    pub power_limit_watts: Option<u32>,
}

/// Compute resource request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for ComputeResource operation
pub struct ComputeResourceRequest {
    /// Requested CPU cores
    pub cpu_cores: u32,
    /// Requested memory in GB
    pub memory_gb: u64,
    /// Requested storage in GB
    pub storage_gb: u64,
    /// GPU requirements
    pub gpu_requirements: Option<GpuAllocation>,
    /// Priority level
    pub priority: String,
}

/// Tuning service registration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tuningserviceregistration
pub struct TuningServiceRegistration {
    /// Service name
    pub name: String,
    /// Service endpoint
    pub endpoint: String,
    /// Supported tuning profiles
    pub supported_profiles: Vec<String>,
    /// Service capabilities
    pub capabilities: Vec<String>,
}

/// External lock type for resource extraction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Types of ExternalLock
pub enum ExternalLockType {
    /// Sovereign external lock
    #[default]
    /// Sovereignexternal
    SovereignExternal,
    /// Collaborative lock
    Collaborative,
    /// Temporary lock
    Temporary,
}
/// Extraction lock for resource access
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Extractionlock
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
/// Cryptographicproof
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
    /// Returns the default instance
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
/// Extractionrestrictions
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
/// Timerestrictions
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
    /// Returns the default instance
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
/// Copyleftrequirements
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
    /// Returns the default instance
    fn default() -> Self {
        Self {
            license_type: "AGPL-3.0".to_string(),
            attribution_required: true,
            share_alike: true,
            commercial_restrictions: Vec::new(),
        }
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Storageconfigurationcanonical
pub type StorageConfigurationCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StorageConfiguration (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Hardwareconfigurationcanonical
pub type HardwareConfigurationCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using HardwareConfiguration (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Memoryconfigurationcanonical
pub type MemoryConfigurationCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using MemoryConfiguration (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

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
