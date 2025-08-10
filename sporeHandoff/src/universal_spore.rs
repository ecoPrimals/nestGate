//! Universal Cryptographic Spore System
//!
//! Self-contained cryptographic "spores" that work autonomously in each primal.
//! Integrates with BearDog's sophisticated genetic key system when available.
//!
//! Design Principles:
//! - Each primal gets its own self-aware cryptographic spore
//! - Spores operate autonomously without external dependencies  
//! - BearDog integration is optional enhancement, not requirement
//! - Humans get frictionless access, corporations negotiate terms
//! - Spores survive even if BearDog disappears completely

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{NestGateError, Result};

/// Universal Cryptographic Spore - Self-contained security for any primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalCryptographicSpore {
    /// Unique spore identifier
    pub spore_id: String,
    
    /// Which primal this spore belongs to
    pub primal_identity: String,
    
    /// Embedded genetic blueprint for autonomous evolution
    pub genetic_blueprint: GeneticBlueprint,
    
    /// Self-contained permission matrix
    pub embedded_permissions: PermissionMatrix,
    
    /// Policy contract with your terms embedded forever
    pub policy_contract: PolicyContract,
    
    /// Autonomous violation detection system
    pub violation_detector: ViolationDetector,
    
    /// Evolution engine for spawning children
    pub evolution_engine: EvolutionEngine,
    
    /// BearDog integration status (optional)
    pub beardog_integration: Option<BearDogIntegration>,
    
    /// Spore generation (0 = original seed)
    pub generation: u32,
    
    /// Parent spore lineage
    pub parent_lineage: Vec<String>,
    
    /// Creation timestamp
    pub created_at: SystemTime,
    
    /// Last evolution timestamp
    pub last_evolution: SystemTime,
    
    /// Usage statistics for fitness scoring
    pub usage_stats: UsageStatistics,
}

/// Genetic blueprint for autonomous spore evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticBlueprint {
    /// Evolution rules embedded in the spore
    pub evolution_rules: Vec<EvolutionRule>,
    
    /// Mutation rate for spawning children
    pub mutation_rate: f64,
    
    /// Fitness scoring algorithm
    pub fitness_algorithm: FitnessAlgorithm,
    
    /// Maximum generations before requiring refresh
    pub max_generations: u32,
    
    /// Threat adaptation patterns
    pub threat_adaptations: Vec<ThreatAdaptation>,
}

/// Self-contained permission matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMatrix {
    /// Individual user permissions (always full)
    pub individual_permissions: IndividualPermissions,
    
    /// Corporate permissions (negotiated terms)
    pub corporate_permissions: HashMap<String, CorporatePermissions>,
    
    /// Operation-specific restrictions
    pub operation_restrictions: HashMap<String, OperationRestriction>,
    
    /// Time-based access controls
    pub temporal_restrictions: Vec<TemporalRestriction>,
}

/// Policy contract embedded in every spore
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyContract {
    /// Your terms for corporate usage
    pub corporate_terms: CorporateTerms,
    
    /// Individual access policy (always free)
    pub individual_policy: IndividualPolicy,
    
    /// Entropy hierarchy requirements
    pub entropy_requirements: EntropyRequirements,
    
    /// Violation response actions
    pub violation_responses: Vec<ViolationResponse>,
    
    /// License negotiation contact info
    pub license_contact: String,
}

/// Autonomous violation detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationDetector {
    /// Corporate usage detection patterns
    pub corporate_patterns: Vec<CorporatePattern>,
    
    /// Automation detection rules
    pub automation_detectors: Vec<AutomationDetector>,
    
    /// Extraction attempt signatures
    pub extraction_signatures: Vec<ExtractionSignature>,
    
    /// Real-time monitoring thresholds
    pub monitoring_thresholds: MonitoringThresholds,
}

/// Evolution engine for spawning children
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionEngine {
    /// Conditions that trigger evolution
    pub evolution_triggers: Vec<EvolutionTrigger>,
    
    /// Child spawning parameters
    pub spawn_parameters: SpawnParameters,
    
    /// Genetic diversity maintenance
    pub diversity_rules: DiversityRules,
    
    /// Performance optimization patterns
    pub optimization_patterns: Vec<OptimizationPattern>,
}

/// Optional BearDog integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogIntegration {
    /// BearDog genetics ID
    pub genetics_id: String,
    
    /// Integration status
    pub status: IntegrationStatus,
    
    /// Last sync with BearDog
    pub last_sync: SystemTime,
    
    /// Extended capabilities from BearDog
    pub extended_capabilities: Vec<String>,
    
    /// BearDog endpoint for extended operations
    pub beardog_endpoint: Option<String>,
}

impl UniversalCryptographicSpore {
    /// Create a new spore for a primal
    pub fn new_for_primal(primal_name: &str) -> Result<Self> {
        let spore_id = format!("spore_{}_{}", primal_name, Uuid::new_v4());
        let now = SystemTime::now();
        
        Ok(Self {
            spore_id,
            primal_identity: primal_name.to_string(),
            genetic_blueprint: GeneticBlueprint::default_for_primal(primal_name),
            embedded_permissions: PermissionMatrix::default_individual_friendly(),
            policy_contract: PolicyContract::default_sovereignty_preserving(),
            violation_detector: ViolationDetector::default_corporate_aware(),
            evolution_engine: EvolutionEngine::default_autonomous(),
            beardog_integration: None, // Optional - will be set if BearDog available
            generation: 0,
            parent_lineage: vec![],
            created_at: now,
            last_evolution: now,
            usage_stats: UsageStatistics::new(),
        })
    }
    
    /// Initialize with BearDog integration (if available)
    pub async fn initialize_with_beardog(&mut self, beardog_endpoint: Option<String>) -> Result<()> {
        if let Some(endpoint) = beardog_endpoint {
            // Try to connect to BearDog for enhanced capabilities
            match self.attempt_beardog_connection(&endpoint).await {
                Ok(integration) => {
                    self.beardog_integration = Some(integration);
                    tracing::info!("🧬 Spore {} initialized with BearDog integration", self.spore_id);
                },
                Err(e) => {
                    tracing::warn!("⚠️ BearDog integration failed, operating autonomously: {}", e);
                    // Spore works perfectly without BearDog
                }
            }
        }
        
        Ok(())
    }
    
    /// Authorize an operation (core spore functionality)
    pub async fn authorize_operation(&self, operation: &OperationRequest) -> Result<AuthorizationDecision> {
        // Step 1: Detect user type (individual vs corporate)
        let user_classification = self.classify_user(operation).await?;
        
        // Step 2: Check for violations
        let violation_check = self.violation_detector.check_for_violations(operation)?;
        if let Some(violation) = violation_check {
            return Ok(AuthorizationDecision::Deny {
                reason: violation.reason,
                violation_type: violation.violation_type,
                remediation: violation.suggested_remediation,
            });
        }
        
        // Step 3: Apply permission matrix
        match user_classification {
            UserClassification::Individual => {
                // Individuals always get full access
                Ok(AuthorizationDecision::Allow {
                    permissions: self.embedded_permissions.individual_permissions.clone().into(),
                    restrictions: vec![],
                    enhanced_by_beardog: self.beardog_integration.is_some(),
                })
            },
            
            UserClassification::Corporate { organization_profile } => {
                // Check if corporation has negotiated license
                if let Some(corp_perms) = self.embedded_permissions.corporate_permissions.get(&organization_profile.organization_id) {
                    if corp_perms.is_valid() {
                        Ok(AuthorizationDecision::Allow {
                            permissions: corp_perms.clone().into(),
                            restrictions: corp_perms.restrictions.clone(),
                            enhanced_by_beardog: self.beardog_integration.is_some(),
                        })
                    } else {
                        Ok(AuthorizationDecision::RequireLicense {
                            terms: self.policy_contract.corporate_terms.clone(),
                            contact: self.policy_contract.license_contact.clone(),
                            organization_profile,
                        })
                    }
                } else {
                    Ok(AuthorizationDecision::RequireLicense {
                        terms: self.policy_contract.corporate_terms.clone(),
                        contact: self.policy_contract.license_contact.clone(),
                        organization_profile,
                    })
                }
            }
        }
    }
    
    /// Spawn a child spore (autonomous evolution)
    pub async fn spawn_child(&mut self) -> Result<UniversalCryptographicSpore> {
        // Check if evolution is needed
        if !self.should_evolve()? {
            return Err(NestGateError::Internal {
                message: "Evolution not required at this time".to_string(),
                location: Some("UniversalCryptographicSpore::spawn_child".to_string()),
                debug_info: Some(format!("Generation: {}, Last evolution: {:?}", self.generation, self.last_evolution)),
                is_bug: false,
            });
        }
        
        // Create child with evolved genetics
        let mut child = self.clone();
        child.spore_id = format!("spore_{}_{}_gen{}", self.primal_identity, Uuid::new_v4(), self.generation + 1);
        child.generation = self.generation + 1;
        child.parent_lineage = {
            let mut lineage = self.parent_lineage.clone();
            lineage.push(self.spore_id.clone());
            lineage
        };
        child.created_at = SystemTime::now();
        child.last_evolution = SystemTime::now();
        
        // Apply genetic mutations
        child.genetic_blueprint = self.evolution_engine.evolve_genetics(&self.genetic_blueprint, &self.usage_stats)?;
        
        // Update usage stats
        self.usage_stats.evolution_count += 1;
        self.last_evolution = SystemTime::now();
        
        tracing::info!("🌱 Spore {} spawned child {} (generation {})", 
                      self.spore_id, child.spore_id, child.generation);
        
        Ok(child)
    }
    
    /// Check if spore should evolve
    fn should_evolve(&self) -> Result<bool> {
        // Check evolution triggers
        for trigger in &self.evolution_engine.evolution_triggers {
            if trigger.is_triggered(&self.usage_stats, self.last_evolution)? {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// Classify user as individual or corporate
    async fn classify_user(&self, operation: &OperationRequest) -> Result<UserClassification> {
        // Use embedded detection patterns
        for pattern in &self.violation_detector.corporate_patterns {
            if pattern.matches(operation)? {
                return Ok(UserClassification::Corporate {
                    organization_profile: pattern.extract_organization_profile(operation)?,
                });
            }
        }
        
        // Default to individual (conservative approach)
        Ok(UserClassification::Individual)
    }
    
    /// Attempt BearDog connection for enhanced capabilities
    async fn attempt_beardog_connection(&self, endpoint: &str) -> Result<BearDogIntegration> {
        // This would integrate with BearDog's existing genetics system
        // For now, return a placeholder that shows integration capability
        Ok(BearDogIntegration {
            genetics_id: format!("beardog_genetics_{}", self.spore_id),
            status: IntegrationStatus::Connected,
            last_sync: SystemTime::now(),
            extended_capabilities: vec![
                "advanced-entropy".to_string(),
                "hsm-integration".to_string(),
                "genetic-evolution".to_string(),
            ],
            beardog_endpoint: Some(endpoint.to_string()),
        })
    }
}

/// Operation request for authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRequest {
    pub operation_type: String,
    pub resource_path: String,
    pub user_context: UserContext,
    pub metadata: HashMap<String, String>,
    pub timestamp: SystemTime,
}

/// User context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContext {
    pub user_id: Option<String>,
    pub session_id: String,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub environment_info: HashMap<String, String>,
}

/// Authorization decision from spore
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorizationDecision {
    Allow {
        permissions: Permissions,
        restrictions: Vec<String>,
        enhanced_by_beardog: bool,
    },
    Deny {
        reason: String,
        violation_type: String,
        remediation: String,
    },
    RequireLicense {
        terms: CorporateTerms,
        contact: String,
        organization_profile: OrganizationProfile,
    },
}

/// User classification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserClassification {
    Individual,
    Corporate {
        organization_profile: OrganizationProfile,
    },
}

// Supporting types with sensible defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualPermissions {
    pub full_access: bool,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporatePermissions {
    pub organization_id: String,
    pub permissions: Vec<String>,
    pub restrictions: Vec<String>,
    pub license_valid_until: SystemTime,
    pub entropy_requirements: EntropyTier,
}

impl CorporatePermissions {
    fn is_valid(&self) -> bool {
        SystemTime::now() < self.license_valid_until
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateTerms {
    pub base_monthly_rate: f64,
    pub automation_tax_multiplier: f64,
    pub entropy_requirements: EntropyTier,
    pub human_supervision_discount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualPolicy {
    pub always_free: bool,
    pub no_restrictions: bool,
    pub full_capabilities: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyRequirements {
    pub minimum_tier: EntropyTier,
    pub human_entropy_bonus: f64,
    pub automation_penalty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntropyTier {
    Machine,      // Pure automation
    Supervised,   // Some human oversight
    HumanLived,   // Human-driven with entropy
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationProfile {
    pub organization_id: String,
    pub organization_name: String,
    pub scale: OrganizationScale,
    pub automation_level: AutomationLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrganizationScale {
    SmallBusiness,
    RegionalBusiness,
    NationalEnterprise,
    GlobalEnterprise,
    Hyperscale,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationLevel {
    HighlyAutomated,
    PartiallyAutomated,
    HumanSupervised,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStatistics {
    pub operations_count: u64,
    pub violations_detected: u64,
    pub evolution_count: u32,
    pub performance_score: f64,
    pub last_activity: SystemTime,
}

impl UsageStatistics {
    fn new() -> Self {
        Self {
            operations_count: 0,
            violations_detected: 0,
            evolution_count: 0,
            performance_score: 1.0,
            last_activity: SystemTime::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationStatus {
    Connected,
    Disconnected,
    Error,
}

// Placeholder implementations for the complex types
// These will be fully implemented as we build out the system

impl Default for GeneticBlueprint {
    fn default() -> Self {
        Self {
            evolution_rules: vec![],
            mutation_rate: 0.05,
            fitness_algorithm: FitnessAlgorithm::BasicPerformance,
            max_generations: 100,
            threat_adaptations: vec![],
        }
    }
}

impl GeneticBlueprint {
    fn default_for_primal(_primal_name: &str) -> Self {
        Self::default()
    }
}

impl Default for PermissionMatrix {
    fn default() -> Self {
        Self::default_individual_friendly()
    }
}

impl PermissionMatrix {
    fn default_individual_friendly() -> Self {
        Self {
            individual_permissions: IndividualPermissions {
                full_access: true,
                capabilities: vec!["*".to_string()], // Full access
            },
            corporate_permissions: HashMap::new(),
            operation_restrictions: HashMap::new(),
            temporal_restrictions: vec![],
        }
    }
}

impl Default for PolicyContract {
    fn default() -> Self {
        Self::default_sovereignty_preserving()
    }
}

impl PolicyContract {
    fn default_sovereignty_preserving() -> Self {
        Self {
            corporate_terms: CorporateTerms {
                base_monthly_rate: 1000.0,
                automation_tax_multiplier: 2.0,
                entropy_requirements: EntropyTier::HumanLived,
                human_supervision_discount: 0.5,
            },
            individual_policy: IndividualPolicy {
                always_free: true,
                no_restrictions: true,
                full_capabilities: true,
            },
            entropy_requirements: EntropyRequirements {
                minimum_tier: EntropyTier::Supervised,
                human_entropy_bonus: 1.5,
                automation_penalty: 2.0,
            },
            violation_responses: vec![],
            license_contact: "license.beardog.dev".to_string(),
        }
    }
}

// Placeholder types - will be fully implemented
impl ViolationDetector {
    fn default_corporate_aware() -> Self {
        Self {
            corporate_patterns: vec![],
            automation_detectors: vec![],
            extraction_signatures: vec![],
            monitoring_thresholds: MonitoringThresholds,
        }
    }
    
    fn check_for_violations(&self, _operation: &OperationRequest) -> Result<Option<ViolationResult>> {
        // Placeholder - will implement corporate detection logic
        Ok(None)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationResult {
    pub reason: String,
    pub violation_type: String,
    pub suggested_remediation: String,
}

impl EvolutionEngine {
    fn default_autonomous() -> Self {
        Self {
            evolution_triggers: vec![],
            spawn_parameters: SpawnParameters,
            diversity_rules: DiversityRules,
            optimization_patterns: vec![],
        }
    }
    
    fn evolve_genetics(&self, _current: &GeneticBlueprint, _stats: &UsageStatistics) -> Result<GeneticBlueprint> {
        // Placeholder - will implement genetic evolution logic
        Ok(GeneticBlueprint::default())
    }
}

// Additional placeholder types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionRule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FitnessAlgorithm {
    BasicPerformance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAdaptation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRestriction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalRestriction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporatePattern;

impl CorporatePattern {
    fn matches(&self, _operation: &OperationRequest) -> Result<bool> {
        Ok(false) // Placeholder
    }
    
    fn extract_organization_profile(&self, _operation: &OperationRequest) -> Result<OrganizationProfile> {
        // Placeholder
        Ok(OrganizationProfile {
            organization_id: "unknown".to_string(),
            organization_name: "Unknown Organization".to_string(),
            scale: OrganizationScale::SmallBusiness,
            automation_level: AutomationLevel::HumanSupervised,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationDetector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionSignature;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringThresholds;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionTrigger;

impl EvolutionTrigger {
    fn is_triggered(&self, _stats: &UsageStatistics, _last_evolution: SystemTime) -> Result<bool> {
        Ok(false) // Placeholder
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnParameters;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiversityRules;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationPattern;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permissions;

impl From<IndividualPermissions> for Permissions {
    fn from(_perms: IndividualPermissions) -> Self {
        Self // Placeholder
    }
}

impl From<CorporatePermissions> for Permissions {
    fn from(_perms: CorporatePermissions) -> Self {
        Self // Placeholder
    }
} 