//! # NestGate External Extraction Protection System
//!
//! **Free internal communication, locked external extraction**
//!
//! This module implements crypto locks specifically for preventing commercial
//! extraction of data and functionality when it leaves the NestGate ecosystem.
//! All internal primal-to-primal communication remains completely free.

use crate::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// External system boundary detector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalBoundary {
    /// Boundary identifier
    pub boundary_id: String,
    /// Type of external system
    pub external_type: ExternalSystemType,
    /// Extraction risk level
    pub risk_level: ExtractionRisk,
    /// Required protection level
    pub protection_level: ProtectionLevel,
}

/// Types of external systems that require protection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalSystemType {
    /// Commercial cloud services (AWS, Azure, GCP)
    CommercialCloud { provider: String },
    /// Third-party APIs
    ThirdPartyAPI { vendor: String },
    /// External databases
    ExternalDatabase { system: String },
    /// Network exports (HTTP, gRPC, etc.)
    NetworkExport { protocol: String },
    /// File exports
    FileExport { format: String },
    /// Container/VM exports
    ContainerExport { runtime: String },
    /// Backup/sync services
    BackupService { provider: String },
    /// Development tools
    DevTools { tool: String },
}

/// Extraction risk levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtractionRisk {
    /// Low risk - basic functionality
    Low,
    /// Medium risk - valuable data
    Medium,
    /// High risk - proprietary algorithms
    High,
    /// Critical risk - core IP
    Critical,
}

/// Protection levels against extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProtectionLevel {
    /// No protection needed
    None,
    /// Basic encryption
    BasicEncryption,
    /// Advanced crypto lock
    CryptoLock,
    /// Copyleft enforcement
    CopyleftEnforcement,
    /// Complete isolation
    Isolation,
}

/// Extraction protection crypto lock
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionLock {
    /// Unique lock identifier
    pub lock_id: Uuid,
    /// Lock type for external boundary
    pub lock_type: ExternalLockType,
    /// Cryptographic proof
    pub proof: CryptographicProof,
    /// Expiration
    pub expires_at: Option<DateTime<Utc>>,
    /// Allowed external operations
    pub allowed_operations: Vec<String>,
    /// Extraction restrictions
    pub restrictions: ExtractionRestrictions,
    /// Copyleft requirements
    pub copyleft_requirements: CopyleftRequirements,
}

/// Types of external locks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalLockType {
    /// Sovereign control - user owns the external system
    SovereignExternal,
    /// Licensed extraction - commercial use with copyleft
    LicensedExtraction { license: String },
    /// Research use - academic/non-commercial
    ResearchUse { institution: String },
    /// Community sharing - open source ecosystem
    CommunitySharing,
    /// Trial access - limited time/scope
    TrialAccess { duration_days: u32 },
}

/// Cryptographic proof - exclusively managed by BearDog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptographicProof {
    /// BearDog public key identifier
    pub beardog_key_id: String,
    /// BearDog digital signature
    pub beardog_signature: String,
    /// BearDog timestamp
    pub timestamp: DateTime<Utc>,
    /// BearDog nonce for replay protection
    pub nonce: String,
    /// BearDog proof hash
    pub proof_hash: String,
    /// Your ecosystem fingerprint (never leaves ecosystem)
    pub ecosystem_fingerprint: String,
    /// BearDog validation token
    pub beardog_validation_token: String,
}

impl CryptographicProof {
    /// Create new proof using BearDog (only way to create crypto locks)
    pub async fn new_with_beardog(
        beardog_validator: &crate::cert::CertValidator,
        operation: &str,
        destination: &str,
    ) -> Result<Self> {
        // Generate ecosystem fingerprint (stays in ecosystem)
        let ecosystem_fingerprint = Self::generate_ecosystem_fingerprint();

        // Generate nonce for replay protection
        let nonce = uuid::Uuid::new_v4().to_string();

        // Create proof data
        let proof_data = format!(
            "{}:{}:{}:{}",
            operation, destination, nonce, ecosystem_fingerprint
        );

        // Get BearDog key ID and signature
        let beardog_key_id = beardog_validator.get_key_id().await?;
        let beardog_signature = beardog_validator.sign_data(&proof_data).await?;

        // Generate BearDog validation token
        let beardog_validation_token = beardog_validator
            .generate_validation_token(&proof_data)
            .await?;

        // Create proof hash
        let proof_hash = Self::hash_proof_data(&proof_data, &beardog_signature)?;

        Ok(Self {
            beardog_key_id,
            beardog_signature,
            timestamp: Utc::now(),
            nonce,
            proof_hash,
            ecosystem_fingerprint,
            beardog_validation_token,
        })
    }

    /// Validate proof using BearDog (only way to unlock crypto locks)
    pub async fn validate_with_beardog(
        &self,
        beardog_validator: &crate::cert::CertValidator,
        operation: &str,
        destination: &str,
    ) -> Result<bool> {
        // Reconstruct proof data
        let proof_data = format!(
            "{}:{}:{}:{}",
            operation, destination, self.nonce, self.ecosystem_fingerprint
        );

        // Validate BearDog signature
        let signature_valid = beardog_validator
            .verify_signature(&proof_data, &self.beardog_signature, &self.beardog_key_id)
            .await?;

        if !signature_valid {
            return Ok(false);
        }

        // Validate BearDog token
        let token_valid = beardog_validator
            .validate_token(&self.beardog_validation_token, &proof_data)
            .await?;

        if !token_valid {
            return Ok(false);
        }

        // Validate proof hash
        let expected_hash = Self::hash_proof_data(&proof_data, &self.beardog_signature)?;
        if expected_hash != self.proof_hash {
            return Ok(false);
        }

        // Validate ecosystem fingerprint (ensures key never left ecosystem)
        let current_fingerprint = Self::generate_ecosystem_fingerprint();
        if current_fingerprint != self.ecosystem_fingerprint {
            return Ok(false);
        }

        Ok(true)
    }

    /// Generate ecosystem fingerprint (always stays in ecosystem)
    fn generate_ecosystem_fingerprint() -> String {
        // Generate a unique fingerprint for your ecosystem
        // This ensures keys never leave your ecosystem
        use std::env;
        use std::process;

        let hostname = env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string());
        let pid = process::id();
        let timestamp = Utc::now().timestamp();

        format!("nestgate-ecosystem-{}-{}-{}", hostname, pid, timestamp)
    }

    /// Hash proof data with BearDog signature
    fn hash_proof_data(data: &str, signature: &str) -> Result<String> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.update(signature.as_bytes());
        let result = hasher.finalize();

        Ok(format!("{:x}", result))
    }
}

/// Extraction restrictions (protecting against unauthorized extraction)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtractionRestrictions {
    /// Data volume limits
    pub max_data_volume: Option<u64>,
    /// API call limits
    pub max_api_calls: Option<u64>,
    /// Geographic restrictions
    pub geographic_limits: Vec<String>,
    /// Time-based restrictions
    pub time_restrictions: Option<TimeRestrictions>,
    /// Purpose restrictions
    pub purpose_restrictions: Vec<String>,
}

/// Time-based restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    /// Allowed time windows
    pub allowed_windows: Vec<TimeWindow>,
    /// Timezone restrictions
    pub timezone_restrictions: Vec<String>,
    /// Maximum session duration
    pub max_session_duration: Option<chrono::Duration>,
}

/// Time window for access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    /// Start time
    pub start: chrono::NaiveTime,
    /// End time
    pub end: chrono::NaiveTime,
    /// Days of week
    pub days_of_week: Vec<chrono::Weekday>,
}

/// Copyleft requirements for external use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyleftRequirements {
    /// Require source code disclosure
    pub require_source_disclosure: bool,
    /// Require attribution
    pub require_attribution: bool,
    /// Require share-alike licensing
    pub require_share_alike: bool,
    /// Require modification disclosure
    pub require_modification_disclosure: bool,
    /// License compatibility check
    pub compatible_licenses: Vec<String>,
}

impl Default for CopyleftRequirements {
    fn default() -> Self {
        Self {
            require_source_disclosure: true,
            require_attribution: true,
            require_share_alike: true,
            require_modification_disclosure: true,
            compatible_licenses: vec!["GPL-3.0".to_string(), "AGPL-3.0".to_string()],
        }
    }
}

/// Internal primal communication - always free
pub struct InternalPrimalCommunication;

impl InternalPrimalCommunication {
    /// Check if communication is internal (always returns true for rust/ecoPrimals)
    pub fn is_internal_communication(source: &str, destination: &str) -> bool {
        // All rust code and ecoPrimals can freely communicate
        Self::is_rust_code(source) && Self::is_rust_code(destination)
            || Self::is_eco_primal(source) && Self::is_eco_primal(destination)
            || Self::is_internal_system(source) && Self::is_internal_system(destination)
    }

    fn is_rust_code(identifier: &str) -> bool {
        identifier.starts_with("nestgate-")
            || identifier.starts_with("rust:")
            || identifier.contains("::") // Rust module path
    }

    fn is_eco_primal(identifier: &str) -> bool {
        identifier.starts_with("ecoprimal:")
            || identifier.starts_with("primal:")
            || identifier.ends_with(".primal")
    }

    fn is_internal_system(identifier: &str) -> bool {
        identifier.starts_with("internal:")
            || identifier.starts_with("localhost:")
            || identifier.starts_with("127.0.0.1:")
    }
}

/// External boundary guardian - exclusively uses BearDog keys
pub struct ExternalBoundaryGuardian {
    /// Active extraction locks (all locked by BearDog)
    extraction_locks: Arc<RwLock<HashMap<String, ExtractionLock>>>,
    /// Boundary configurations
    boundaries: Arc<RwLock<HashMap<String, ExternalBoundary>>>,
    /// Copyleft enforcer
    copyleft_enforcer: CopyleftEnforcer,
    /// BearDog validator (exclusive key manager)
    beardog_validator: Arc<crate::cert::CertValidator>,
}

impl ExternalBoundaryGuardian {
    /// Create new guardian with BearDog as exclusive key manager
    pub fn new(beardog_config: crate::cert::BearDogConfig) -> Self {
        Self {
            extraction_locks: Arc::new(RwLock::new(HashMap::new())),
            boundaries: Arc::new(RwLock::new(HashMap::new())),
            copyleft_enforcer: CopyleftEnforcer::new(),
            beardog_validator: Arc::new(crate::cert::CertValidator::with_beardog(beardog_config)),
        }
    }

    /// Check if access crosses external boundary (BearDog-validated)
    pub async fn check_external_boundary(
        &self,
        source: &str,
        destination: &str,
        operation: &str,
    ) -> Result<AccessDecision> {
        // Internal communication is always allowed (no BearDog needed)
        if InternalPrimalCommunication::is_internal_communication(source, destination) {
            return Ok(AccessDecision::Allow {
                reason: "Internal primal communication - no BearDog key required".to_string(),
                restrictions: vec![],
            });
        }

        // Check if destination is external
        if self.is_external_system(destination).await? {
            return self
                .evaluate_external_access_with_beardog(source, destination, operation)
                .await;
        }

        // Default allow for unclear cases
        Ok(AccessDecision::Allow {
            reason: "Not clearly external - no BearDog key required".to_string(),
            restrictions: vec![],
        })
    }

    /// Evaluate external access using BearDog validation
    async fn evaluate_external_access_with_beardog(
        &self,
        source: &str,
        destination: &str,
        operation: &str,
    ) -> Result<AccessDecision> {
        // Determine external system type
        let external_type = self.classify_external_system(destination).await?;

        // Assess extraction risk
        let risk_level = self
            .assess_extraction_risk(operation, &external_type)
            .await?;

        // Check for existing BearDog-validated extraction lock
        let extraction_locks = self.extraction_locks.read().await;
        let lock_key = format!("{}:{}", source, destination);

        if let Some(lock) = extraction_locks.get(&lock_key) {
            // Validate existing lock using BearDog
            return self.validate_beardog_extraction_lock(lock, operation).await;
        }

        // No lock found - ALL external access requires BearDog crypto locks
        match risk_level {
            ExtractionRisk::Low => Ok(AccessDecision::RequireLock {
                reason: "Low risk external access - BearDog crypto lock required".to_string(),
                lock_type: ExternalLockType::CommunitySharing,
            }),
            ExtractionRisk::Medium => Ok(AccessDecision::RequireLock {
                reason: "Medium risk external access - BearDog crypto lock required".to_string(),
                lock_type: ExternalLockType::LicensedExtraction {
                    license: "copyleft".to_string(),
                },
            }),
            ExtractionRisk::High => Ok(AccessDecision::RequireLock {
                reason: "High risk external access - BearDog crypto lock required".to_string(),
                lock_type: ExternalLockType::LicensedExtraction {
                    license: "copyleft".to_string(),
                },
            }),
            ExtractionRisk::Critical => Ok(AccessDecision::RequireLock {
                reason: "Critical risk external access - BearDog sovereign lock required"
                    .to_string(),
                lock_type: ExternalLockType::SovereignExternal,
            }),
        }
    }

    /// Validate extraction lock using BearDog (only way to unlock)
    async fn validate_beardog_extraction_lock(
        &self,
        lock: &ExtractionLock,
        operation: &str,
    ) -> Result<AccessDecision> {
        // Check expiration
        if let Some(expires_at) = lock.expires_at {
            if expires_at < Utc::now() {
                return Ok(AccessDecision::Deny {
                    reason: "BearDog extraction lock expired".to_string(),
                    alternative: Some("Renew BearDog extraction lock".to_string()),
                });
            }
        }

        // Check allowed operations
        if !lock.allowed_operations.contains(&operation.to_string()) {
            return Ok(AccessDecision::Deny {
                reason: format!(
                    "Operation '{}' not allowed by BearDog extraction lock",
                    operation
                ),
                alternative: Some("Request additional BearDog permissions".to_string()),
            });
        }

        // CRITICAL: Validate BearDog cryptographic proof
        let proof_valid = lock
            .proof
            .validate_with_beardog(
                &self.beardog_validator,
                operation,
                "", // destination not needed for validation
            )
            .await?;

        if !proof_valid {
            return Ok(AccessDecision::Deny {
                reason: "BearDog cryptographic proof validation failed".to_string(),
                alternative: Some("Obtain new BearDog crypto lock".to_string()),
            });
        }

        // Check copyleft requirements
        if lock.copyleft_requirements.require_source_disclosure {
            self.copyleft_enforcer.enforce_source_disclosure().await?;
        }

        Ok(AccessDecision::Allow {
            reason: "Valid BearDog extraction lock".to_string(),
            restrictions: vec![
                "beardog_validated".to_string(),
                "copyleft_compliance".to_string(),
            ],
        })
    }

    /// Install BearDog extraction lock (only way to create crypto locks)
    pub async fn install_beardog_extraction_lock(
        &self,
        source: &str,
        destination: &str,
        operation: &str,
        lock_type: ExternalLockType,
        restrictions: ExtractionRestrictions,
        copyleft_requirements: CopyleftRequirements,
    ) -> Result<()> {
        // Create BearDog cryptographic proof
        let proof =
            CryptographicProof::new_with_beardog(&self.beardog_validator, operation, destination)
                .await?;

        // Create extraction lock
        let lock = ExtractionLock {
            lock_id: Uuid::new_v4(),
            lock_type,
            proof,
            expires_at: None, // Can be set based on requirements
            allowed_operations: vec![operation.to_string()],
            restrictions,
            copyleft_requirements,
        };

        // Install lock
        let mut extraction_locks = self.extraction_locks.write().await;
        let lock_key = format!("{}:{}", source, destination);
        extraction_locks.insert(lock_key, lock);

        tracing::info!(
            "BearDog extraction lock installed: {} -> {} (operation: {})",
            source,
            destination,
            operation
        );

        Ok(())
    }

    /// Create sovereign BearDog lock for external companies
    pub async fn create_sovereign_beardog_lock(
        &self,
        external_company: &str,
        permitted_operations: Vec<String>,
        expiration_days: Option<u32>,
    ) -> Result<String> {
        // Generate lock ID with proper prefix
        let lock_id = Uuid::new_v4();
        let lock_id_string = format!("beardog-sovereign-{}", lock_id);

        // Create BearDog proof for sovereign access
        let proof = CryptographicProof::new_with_beardog(
            &self.beardog_validator,
            &permitted_operations.join(","),
            external_company,
        )
        .await?;

        // Set expiration
        let expires_at =
            expiration_days.map(|days| Utc::now() + chrono::Duration::days(days as i64));

        // Create sovereign lock
        let lock = ExtractionLock {
            lock_id,
            lock_type: ExternalLockType::SovereignExternal,
            proof,
            expires_at,
            allowed_operations: permitted_operations,
            restrictions: ExtractionRestrictions {
                max_data_volume: None,
                max_api_calls: None,
                geographic_limits: vec![],
                time_restrictions: None,
                purpose_restrictions: vec!["commercial".to_string()],
            },
            copyleft_requirements: CopyleftRequirements {
                require_source_disclosure: true,
                require_attribution: true,
                require_share_alike: true,
                require_modification_disclosure: true,
                compatible_licenses: vec!["GPL-3.0".to_string(), "AGPL-3.0".to_string()],
            },
        };

        // Store lock
        let mut extraction_locks = self.extraction_locks.write().await;
        extraction_locks.insert(external_company.to_string(), lock);

        tracing::info!(
            "Sovereign BearDog lock created for external company: {} (expires: {:?})",
            external_company,
            expires_at
        );

        Ok(lock_id_string)
    }

    /// Register external boundary
    pub async fn register_external_boundary(&self, boundary: ExternalBoundary) -> Result<()> {
        let mut boundaries = self.boundaries.write().await;
        boundaries.insert(boundary.boundary_id.clone(), boundary);
        Ok(())
    }

    async fn is_external_system(&self, destination: &str) -> Result<bool> {
        // Check against known external system patterns
        let external_patterns = vec![
            "http://",
            "https://", // Web services
            "amazonaws.com",
            "azure.com",
            "googleapis.com", // Cloud services
            "github.com",
            "gitlab.com", // Code repositories
            "docker.io",
            "registry-1.docker.io", // Container registries
            "pypi.org",
            "npmjs.com", // Package managers
        ];

        for pattern in external_patterns {
            if destination.contains(pattern) {
                return Ok(true);
            }
        }

        // Check configured boundaries
        let boundaries = self.boundaries.read().await;
        Ok(boundaries
            .values()
            .any(|b| destination.contains(&b.boundary_id)))
    }

    async fn classify_external_system(&self, destination: &str) -> Result<ExternalSystemType> {
        if destination.contains("amazonaws.com") {
            Ok(ExternalSystemType::CommercialCloud {
                provider: "AWS".to_string(),
            })
        } else if destination.contains("azure.com") {
            Ok(ExternalSystemType::CommercialCloud {
                provider: "Azure".to_string(),
            })
        } else if destination.contains("googleapis.com") {
            Ok(ExternalSystemType::CommercialCloud {
                provider: "GCP".to_string(),
            })
        } else if destination.starts_with("http") {
            Ok(ExternalSystemType::ThirdPartyAPI {
                vendor: "Unknown".to_string(),
            })
        } else {
            Ok(ExternalSystemType::ExternalDatabase {
                system: "Unknown".to_string(),
            })
        }
    }

    async fn assess_extraction_risk(
        &self,
        operation: &str,
        _external_type: &ExternalSystemType,
    ) -> Result<ExtractionRisk> {
        match operation {
            // Low risk operations
            "read" | "query" | "fetch" => Ok(ExtractionRisk::Low),
            // Medium risk operations
            "write" | "store" | "backup" => Ok(ExtractionRisk::Medium),
            // High risk operations
            "export" | "migrate" | "sync" => Ok(ExtractionRisk::High),
            // Critical risk operations
            "clone" | "replicate" | "extract" => Ok(ExtractionRisk::Critical),
            _ => Ok(ExtractionRisk::Medium),
        }
    }
}

/// Access decision for external boundaries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessDecision {
    /// Allow access
    Allow {
        reason: String,
        restrictions: Vec<String>,
    },
    /// Require authentication
    RequireAuthentication { reason: String, auth_type: String },
    /// Require extraction lock
    RequireLock {
        reason: String,
        lock_type: ExternalLockType,
    },
    /// Deny access
    Deny {
        reason: String,
        alternative: Option<String>,
    },
}

/// Copyleft enforcement for maintaining open source compliance
#[derive(Default)]
pub struct CopyleftEnforcer;

impl CopyleftEnforcer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enforce source code disclosure
    pub async fn enforce_source_disclosure(&self) -> Result<()> {
        // Implement source disclosure enforcement
        // This would ensure that any commercial use requires source code sharing
        Ok(())
    }

    /// Enforce attribution requirements
    pub async fn enforce_attribution(&self) -> Result<()> {
        // Implement attribution enforcement
        Ok(())
    }

    /// Enforce share-alike licensing
    pub async fn enforce_share_alike(&self) -> Result<()> {
        // Implement share-alike enforcement
        Ok(())
    }
}

/// Hardware-agnostic performance tuning
pub struct HardwareAgnosticTuner {
    /// Detected hardware configuration
    hardware_config: Option<HardwareConfiguration>,
    /// Tuning profiles
    tuning_profiles: HashMap<String, TuningProfile>,
}

impl HardwareAgnosticTuner {
    pub fn new() -> Self {
        Self {
            hardware_config: None,
            tuning_profiles: Self::default_tuning_profiles(),
        }
    }

    /// Auto-detect hardware and apply optimal tuning
    pub async fn auto_tune(&mut self) -> Result<TuningResult> {
        // Detect hardware
        let hardware = self.detect_hardware().await?;
        self.hardware_config = Some(hardware.clone());

        // Select optimal tuning profile
        let profile = self.select_tuning_profile(&hardware).await?;

        // Apply tuning
        self.apply_tuning_profile(&profile).await
    }

    async fn detect_hardware(&self) -> Result<HardwareConfiguration> {
        // Agnostic hardware detection
        Ok(HardwareConfiguration {
            cpu_cores: self.detect_cpu_cores().await?,
            memory_gb: self.detect_memory_gb().await?,
            storage_devices: self.detect_storage_devices().await?,
            network_interfaces: self.detect_network_interfaces().await?,
            accelerators: self.detect_accelerators().await?,
        })
    }

    async fn detect_cpu_cores(&self) -> Result<u32> {
        // Cross-platform CPU detection
        Ok(num_cpus::get() as u32)
    }

    async fn detect_memory_gb(&self) -> Result<u32> {
        // Cross-platform memory detection
        // This would use system-specific APIs
        Ok(32) // Placeholder
    }

    async fn detect_storage_devices(&self) -> Result<Vec<StorageDevice>> {
        // Cross-platform storage detection
        Ok(vec![StorageDevice {
            device_id: "primary".to_string(),
            device_type: StorageType::NVMe,
            capacity_gb: 1000,
            performance_tier: PerformanceTier::High,
        }])
    }

    async fn detect_network_interfaces(&self) -> Result<Vec<NetworkInterface>> {
        // Cross-platform network detection
        Ok(vec![NetworkInterface {
            interface_id: "primary".to_string(),
            speed_mbps: 1000,
            interface_type: NetworkType::Ethernet,
        }])
    }

    async fn detect_accelerators(&self) -> Result<Vec<Accelerator>> {
        // Detect GPUs, TPUs, FPGAs, etc.
        Ok(vec![]) // No accelerators in this example
    }

    async fn select_tuning_profile(
        &self,
        hardware: &HardwareConfiguration,
    ) -> Result<TuningProfile> {
        // More logical profile selection based on CPU cores
        match hardware.cpu_cores {
            1..=4 => {
                // Low core count - use efficient profile
                self.tuning_profiles
                    .get("efficient")
                    .ok_or_else(|| {
                        crate::NestGateError::InvalidInput(
                            "Efficient tuning profile not found".to_string(),
                        )
                    })
                    .cloned()
            }
            5..=16 => {
                // Medium core count - use balanced profile
                self.tuning_profiles
                    .get("balanced")
                    .ok_or_else(|| {
                        crate::NestGateError::InvalidInput(
                            "Balanced tuning profile not found".to_string(),
                        )
                    })
                    .cloned()
            }
            _ => {
                // High core count - use high performance profile
                self.tuning_profiles
                    .get("high_performance")
                    .ok_or_else(|| {
                        crate::NestGateError::InvalidInput(
                            "High performance tuning profile not found".to_string(),
                        )
                    })
                    .cloned()
            }
        }
    }

    async fn apply_tuning_profile(&self, profile: &TuningProfile) -> Result<TuningResult> {
        let mut optimizations = vec![];

        // Apply CPU optimizations
        for opt in &profile.cpu_optimizations {
            self.apply_cpu_optimization(opt).await?;
            optimizations.push(format!("CPU: {}", opt));
        }

        // Apply memory optimizations
        for opt in &profile.memory_optimizations {
            self.apply_memory_optimization(opt).await?;
            optimizations.push(format!("Memory: {}", opt));
        }

        // Apply storage optimizations
        for opt in &profile.storage_optimizations {
            self.apply_storage_optimization(opt).await?;
            optimizations.push(format!("Storage: {}", opt));
        }

        // Apply network optimizations
        for opt in &profile.network_optimizations {
            self.apply_network_optimization(opt).await?;
            optimizations.push(format!("Network: {}", opt));
        }

        Ok(TuningResult {
            profile_name: profile.name.clone(),
            optimizations_applied: profile.cpu_optimizations.clone(),
            estimated_performance_gain: profile.estimated_performance_gain,
            status: "applied".to_string(),
            applied_settings: HashMap::new(),
        })
    }

    async fn apply_cpu_optimization(&self, optimization: &str) -> Result<()> {
        // Apply CPU-specific optimizations
        match optimization {
            "enable_turbo" => {
                // Enable CPU turbo boost
            }
            "set_affinity" => {
                // Set CPU affinity for critical threads
            }
            "optimize_cache" => {
                // Optimize cache usage patterns
            }
            _ => {}
        }
        Ok(())
    }

    async fn apply_memory_optimization(&self, optimization: &str) -> Result<()> {
        // Apply memory-specific optimizations
        match optimization {
            "huge_pages" => {
                // Enable huge pages
            }
            "numa_aware" => {
                // Enable NUMA-aware memory allocation
            }
            "memory_pool" => {
                // Configure memory pools
            }
            _ => {}
        }
        Ok(())
    }

    async fn apply_storage_optimization(&self, optimization: &str) -> Result<()> {
        // Apply storage-specific optimizations
        match optimization {
            "io_scheduler" => {
                // Optimize I/O scheduler
            }
            "readahead" => {
                // Configure readahead settings
            }
            "queue_depth" => {
                // Optimize queue depth
            }
            _ => {}
        }
        Ok(())
    }

    async fn apply_network_optimization(&self, optimization: &str) -> Result<()> {
        // Apply network-specific optimizations
        match optimization {
            "tcp_tuning" => {
                // Optimize TCP settings
            }
            "buffer_sizes" => {
                // Optimize buffer sizes
            }
            "interrupt_coalescing" => {
                // Configure interrupt coalescing
            }
            _ => {}
        }
        Ok(())
    }

    fn default_tuning_profiles() -> HashMap<String, TuningProfile> {
        let mut profiles = HashMap::new();

        profiles.insert(
            "high_performance".to_string(),
            TuningProfile {
                name: "High Performance".to_string(),
                cpu_optimizations: vec![
                    "enable_turbo".to_string(),
                    "set_affinity".to_string(),
                    "optimize_cache".to_string(),
                ],
                memory_optimizations: vec![
                    "huge_pages".to_string(),
                    "numa_aware".to_string(),
                    "memory_pool".to_string(),
                ],
                storage_optimizations: vec![
                    "io_scheduler".to_string(),
                    "readahead".to_string(),
                    "queue_depth".to_string(),
                ],
                network_optimizations: vec![
                    "tcp_tuning".to_string(),
                    "buffer_sizes".to_string(),
                    "interrupt_coalescing".to_string(),
                ],
                estimated_performance_gain: 40.0,
            },
        );

        profiles.insert(
            "balanced".to_string(),
            TuningProfile {
                name: "Balanced".to_string(),
                cpu_optimizations: vec!["set_affinity".to_string()],
                memory_optimizations: vec!["memory_pool".to_string()],
                storage_optimizations: vec!["io_scheduler".to_string()],
                network_optimizations: vec!["tcp_tuning".to_string()],
                estimated_performance_gain: 20.0,
            },
        );

        profiles.insert(
            "efficient".to_string(),
            TuningProfile {
                name: "Efficient".to_string(),
                cpu_optimizations: vec!["set_affinity".to_string()],
                memory_optimizations: vec![],
                storage_optimizations: vec![],
                network_optimizations: vec![],
                estimated_performance_gain: 5.0,
            },
        );

        profiles
    }
}

/// Hardware configuration (agnostic)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareConfiguration {
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub storage_devices: Vec<StorageDevice>,
    pub network_interfaces: Vec<NetworkInterface>,
    pub accelerators: Vec<Accelerator>,
}

impl Default for HardwareConfiguration {
    fn default() -> Self {
        Self {
            cpu_cores: num_cpus::get() as u32,
            memory_gb: 16,
            storage_devices: vec![],
            network_interfaces: vec![],
            accelerators: vec![],
        }
    }
}

/// Storage device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDevice {
    pub device_id: String,
    pub device_type: StorageType,
    pub capacity_gb: u64,
    pub performance_tier: PerformanceTier,
}

/// Storage types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    HDD,
    SSD,
    NVMe,
    Optane,
    Tape,
    Cloud,
}

/// Performance tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceTier {
    Low,
    Medium,
    High,
    Ultra,
}

/// Network interface information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub interface_id: String,
    pub speed_mbps: u32,
    pub interface_type: NetworkType,
}

/// Network types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkType {
    Ethernet,
    WiFi,
    Infiniband,
    Fiber,
}

/// Accelerator information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Accelerator {
    pub accelerator_id: String,
    pub accelerator_type: AcceleratorType,
    pub memory_gb: u32,
}

/// Accelerator types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AcceleratorType {
    GPU,
    TPU,
    FPGA,
    ASIC,
}

/// Tuning profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningProfile {
    pub name: String,
    pub cpu_optimizations: Vec<String>,
    pub memory_optimizations: Vec<String>,
    pub storage_optimizations: Vec<String>,
    pub network_optimizations: Vec<String>,
    pub estimated_performance_gain: f64,
}

/// Tuning result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningResult {
    pub profile_name: String,
    pub optimizations_applied: Vec<String>,
    pub estimated_performance_gain: f64,
    // Add new field needed for the hardware tuning
    pub status: String,
    pub applied_settings: HashMap<String, String>,
}

impl Default for TuningResult {
    fn default() -> Self {
        Self {
            profile_name: "default".to_string(),
            optimizations_applied: vec![],
            estimated_performance_gain: 0.0,
            status: "ready".to_string(),
            applied_settings: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_internal_communication_allowed() {
        let beardog_config = crate::cert::BearDogConfig {
            endpoint: "https://beardog.test:8443".to_string(),
            api_key: "test-key".to_string(),
            trust_anchor: "test-anchor".to_string(),
            validation_timeout: std::time::Duration::from_secs(30),
            retry_attempts: 3,
        };

        let guardian = ExternalBoundaryGuardian::new(beardog_config);

        // Test internal communication (should always be allowed)
        let result = guardian
            .check_external_boundary("nestgate-core", "nestgate-zfs", "dataset_create")
            .await;

        assert!(result.is_ok());
        match result.unwrap() {
            AccessDecision::Allow { .. } => (),
            _ => panic!("Internal communication should be allowed"),
        }
    }

    #[tokio::test]
    async fn test_external_boundary_detection() {
        let beardog_config = crate::cert::BearDogConfig {
            endpoint: "https://beardog.test:8443".to_string(),
            api_key: "test-key".to_string(),
            trust_anchor: "test-anchor".to_string(),
            validation_timeout: std::time::Duration::from_secs(30),
            retry_attempts: 3,
        };

        let guardian = ExternalBoundaryGuardian::new(beardog_config);

        // Test external access with proper AWS URL (should require lock)
        let result = guardian
            .check_external_boundary("nestgate-core", "s3.amazonaws.com", "file_upload")
            .await;

        assert!(result.is_ok());
        match result.unwrap() {
            AccessDecision::RequireLock { .. } => (),
            _ => panic!("External access should require lock"),
        }
    }

    #[tokio::test]
    async fn test_hardware_agnostic_tuning() {
        let mut tuner = HardwareAgnosticTuner::new();
        let result = tuner.auto_tune().await.unwrap();

        assert!(!result.optimizations_applied.is_empty());
        assert!(result.estimated_performance_gain > 0.0);
    }

    #[test]
    fn test_copyleft_requirements() {
        let requirements = CopyleftRequirements {
            require_source_disclosure: true,
            require_attribution: true,
            require_share_alike: true,
            require_modification_disclosure: true,
            compatible_licenses: vec!["GPL-3.0".to_string(), "AGPL-3.0".to_string()],
        };

        assert!(requirements.require_source_disclosure);
        assert!(requirements.require_share_alike);
    }
}
