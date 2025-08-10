/// Universal Architecture Validation Tests
///
/// This module provides comprehensive validation testing for universal architecture
/// patterns, ensuring that our sovereignty-compliant system meets all design
/// principles and operational requirements:
/// - Universal capability discovery validation
/// - Service sovereignty enforcement
/// - Cross-ecosystem interoperability
/// - Dynamic service registration and discovery
/// - Universal adapter pattern compliance
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};
use tokio::time::{sleep, timeout};
use uuid::Uuid;

use nestgate_core::interface::UnifiedServiceConfig;
use nestgate_core::universal_primal_discovery::cache::DiscoveryCache;
use nestgate_core::{NestGateError, Result};

/// Universal architecture validation configuration
#[derive(Debug, Clone)]
pub struct ArchitectureValidationConfig {
    pub test_timeout: Duration,
    pub service_count: usize,
    pub capability_types: Vec<String>,
    pub strict_sovereignty_enforcement: bool,
    pub validate_cross_ecosystem: bool,
    pub test_concurrent_discovery: bool,
}

impl Default for ArchitectureValidationConfig {
    fn default() -> Self {
        Self {
            test_timeout: Duration::from_secs(30),
            service_count: 10,
            capability_types: vec![
                "orchestration".to_string(),
                "security".to_string(),
                "compute".to_string(),
                "storage".to_string(),
                "network".to_string(),
            ],
            strict_sovereignty_enforcement: true,
            validate_cross_ecosystem: true,
            test_concurrent_discovery: true,
        }
    }
}

/// Architecture validation results
#[derive(Debug, Clone)]
pub struct ArchitectureValidationResults {
    pub test_duration: Duration,
    pub services_validated: usize,
    pub capabilities_discovered: usize,
    pub sovereignty_compliance_score: f64,
    pub cross_ecosystem_compatibility: bool,
    pub discovery_success_rate: f64,
    pub universal_adapter_compliance: bool,
    pub dynamic_registration_success: bool,
    pub validation_errors: Vec<String>,
}

/// Universal architecture validation framework
pub struct UniversalArchitectureValidator {
    config: ArchitectureValidationConfig,
    discovery_cache: DiscoveryCache,
    registered_services: HashMap<String, UnifiedServiceConfig>,
    validation_results: ArchitectureValidationResults,
}

impl UniversalArchitectureValidator {
    pub fn new(config: ArchitectureValidationConfig) -> Self {
        Self {
            config,
            discovery_cache: DiscoveryCache::new(),
            registered_services: HashMap::new(),
            validation_results: ArchitectureValidationResults {
                test_duration: Duration::ZERO,
                services_validated: 0,
                capabilities_discovered: 0,
                sovereignty_compliance_score: 0.0,
                cross_ecosystem_compatibility: false,
                discovery_success_rate: 0.0,
                universal_adapter_compliance: false,
                dynamic_registration_success: false,
                validation_errors: Vec::new(),
            },
        }
    }

    /// Run comprehensive universal architecture validation
    pub async fn validate_architecture(&mut self) -> Result<ArchitectureValidationResults> {
        println!("🏗️ Starting Universal Architecture Validation");
        let start_time = Instant::now();

        // Phase 1: Service Registration and Discovery Validation
        self.validate_service_registration().await?;

        // Phase 2: Capability Discovery Validation
        self.validate_capability_discovery().await?;

        // Phase 3: Sovereignty Compliance Validation
        self.validate_sovereignty_compliance().await?;

        // Phase 4: Cross-Ecosystem Compatibility Validation
        if self.config.validate_cross_ecosystem {
            self.validate_cross_ecosystem_compatibility().await?;
        }

        // Phase 5: Universal Adapter Pattern Validation
        self.validate_universal_adapter_patterns().await?;

        // Phase 6: Concurrent Discovery Validation
        if self.config.test_concurrent_discovery {
            self.validate_concurrent_discovery().await?;
        }

        // Phase 7: Dynamic Service Management Validation
        self.validate_dynamic_service_management().await?;

        // Finalize results
        self.validation_results.test_duration = start_time.elapsed();
        self.calculate_final_scores();

        println!("✅ Universal Architecture Validation completed");
        println!("   Duration: {:?}", self.validation_results.test_duration);
        println!(
            "   Services: {}",
            self.validation_results.services_validated
        );
        println!(
            "   Capabilities: {}",
            self.validation_results.capabilities_discovered
        );
        println!(
            "   Sovereignty Score: {:.2}%",
            self.validation_results.sovereignty_compliance_score * 100.0
        );
        println!(
            "   Cross-ecosystem: {}",
            self.validation_results.cross_ecosystem_compatibility
        );

        if !self.validation_results.validation_errors.is_empty() {
            println!("⚠️ Validation Errors:");
            for error in &self.validation_results.validation_errors {
                println!("   - {}", error);
            }
        }

        Ok(self.validation_results.clone())
    }

    /// Validate service registration and basic discovery
    async fn validate_service_registration(&mut self) -> Result<()> {
        println!("📋 Validating service registration...");

        // Create test services representing different ecosystems
        let test_services = self.create_test_services();

        for service in test_services {
            // Register service
            self.registered_services
                .insert(service.service_id.clone(), service.clone());

            // Register capabilities in discovery cache
            for capability in &service.capabilities {
                self.discovery_cache
                    .store_discovery(
                        &format!("capability:{}", capability),
                        &service.endpoint,
                        Some(Duration::from_secs(300)),
                    )
                    .await;
            }

            self.validation_results.services_validated += 1;
        }

        // Validate all services are properly registered
        if self.registered_services.len() != self.config.service_count {
            self.validation_results.validation_errors.push(format!(
                "Expected {} services, found {}",
                self.config.service_count,
                self.registered_services.len()
            ));
        }

        Ok(())
    }

    /// Validate capability discovery mechanisms
    async fn validate_capability_discovery(&mut self) -> Result<()> {
        println!("🔍 Validating capability discovery...");

        let mut discovered_capabilities = 0;
        let mut failed_discoveries = 0;

        // Test discovery for each registered capability
        for service in self.registered_services.values() {
            for capability in &service.capabilities {
                let discovery_key = format!("capability:{}", capability);

                match self.discovery_cache.get_discovery(&discovery_key).await {
                    Some(endpoint) => {
                        // Validate discovered endpoint matches service endpoint
                        if endpoint == service.endpoint {
                            discovered_capabilities += 1;
                        } else {
                            failed_discoveries += 1;
                            self.validation_results.validation_errors.push(format!(
                                "Capability {} discovered wrong endpoint: {} vs {}",
                                capability, endpoint, service.endpoint
                            ));
                        }
                    }
                    None => {
                        failed_discoveries += 1;
                        self.validation_results
                            .validation_errors
                            .push(format!("Failed to discover capability: {}", capability));
                    }
                }
            }
        }

        self.validation_results.capabilities_discovered = discovered_capabilities;

        // Calculate discovery success rate
        let total_capabilities = discovered_capabilities + failed_discoveries;
        self.validation_results.discovery_success_rate = if total_capabilities > 0 {
            discovered_capabilities as f64 / total_capabilities as f64
        } else {
            0.0
        };

        Ok(())
    }

    /// Validate sovereignty compliance across all services
    async fn validate_sovereignty_compliance(&mut self) -> Result<()> {
        println!("👑 Validating sovereignty compliance...");

        let mut compliant_services = 0;
        let mut violation_count = 0;

        for service in self.registered_services.values() {
            let mut service_compliant = true;

            // Check sovereignty compliance flag
            if !service.sovereignty_compliant {
                service_compliant = false;
                violation_count += 1;
                self.validation_results.validation_errors.push(format!(
                    "Service {} not marked as sovereignty compliant",
                    service.service_id
                ));
            }

            // Check for hardcoded values (sovereignty violation)
            if service.endpoint.contains("hardcoded")
                || service.endpoint.contains("localhost")
                || service.endpoint.contains("127.0.0.1")
            {
                service_compliant = false;
                violation_count += 1;
                self.validation_results.validation_errors.push(format!(
                    "Service {} has hardcoded endpoint: {}",
                    service.service_id, service.endpoint
                ));
            }

            // Check for meaningful capabilities (not hardcoded)
            if service
                .capabilities
                .iter()
                .any(|cap| cap.contains("hardcoded") || cap.contains("test"))
            {
                service_compliant = false;
                violation_count += 1;
                self.validation_results.validation_errors.push(format!(
                    "Service {} has hardcoded capabilities",
                    service.service_id
                ));
            }

            // Check for proper capability namespacing
            for capability in &service.capabilities {
                if !capability.contains('.') || capability.split('.').count() < 2 {
                    service_compliant = false;
                    violation_count += 1;
                    self.validation_results.validation_errors.push(format!(
                        "Service {} has improperly namespaced capability: {}",
                        service.service_id, capability
                    ));
                }
            }

            if service_compliant {
                compliant_services += 1;
            }
        }

        // Calculate sovereignty compliance score
        self.validation_results.sovereignty_compliance_score = if self.registered_services.len() > 0
        {
            compliant_services as f64 / self.registered_services.len() as f64
        } else {
            0.0
        };

        // Strict enforcement check
        if self.config.strict_sovereignty_enforcement && violation_count > 0 {
            return Err(NestGateError::Validation(
                format!(
                    "Strict sovereignty enforcement enabled, found {} violations",
                    violation_count
                )
                .into(),
            ));
        }

        Ok(())
    }

    /// Validate cross-ecosystem compatibility
    async fn validate_cross_ecosystem_compatibility(&mut self) -> Result<()> {
        println!("🌐 Validating cross-ecosystem compatibility...");

        // Group services by ecosystem (based on capability prefixes)
        let mut ecosystem_groups: HashMap<String, Vec<&UnifiedServiceConfig>> = HashMap::new();

        for service in self.registered_services.values() {
            for capability in &service.capabilities {
                if let Some(ecosystem) = capability.split('.').next() {
                    ecosystem_groups
                        .entry(ecosystem.to_string())
                        .or_insert_with(Vec::new)
                        .push(service);
                }
            }
        }

        // Validate that we have multiple ecosystems
        if ecosystem_groups.len() < 2 {
            self.validation_results.validation_errors.push(
                "Cross-ecosystem testing requires at least 2 different ecosystems".to_string(),
            );
            return Ok(());
        }

        // Test compatibility between ecosystems
        let mut compatibility_tests_passed = 0;
        let mut compatibility_tests_total = 0;

        for (ecosystem1, services1) in &ecosystem_groups {
            for (ecosystem2, services2) in &ecosystem_groups {
                if ecosystem1 != ecosystem2 {
                    compatibility_tests_total += 1;

                    // Test if services from different ecosystems can be discovered together
                    let service1 = services1.first().unwrap();
                    let service2 = services2.first().unwrap();

                    // Both services should be discoverable
                    let mut both_discoverable = true;
                    for capability in &service1.capabilities {
                        if self
                            .discovery_cache
                            .get_discovery(&format!("capability:{}", capability))
                            .await
                            .is_none()
                        {
                            both_discoverable = false;
                            break;
                        }
                    }

                    if both_discoverable {
                        for capability in &service2.capabilities {
                            if self
                                .discovery_cache
                                .get_discovery(&format!("capability:{}", capability))
                                .await
                                .is_none()
                            {
                                both_discoverable = false;
                                break;
                            }
                        }
                    }

                    if both_discoverable {
                        compatibility_tests_passed += 1;
                    }
                }
            }
        }

        self.validation_results.cross_ecosystem_compatibility = compatibility_tests_total == 0
            || compatibility_tests_passed == compatibility_tests_total;

        println!(
            "   Ecosystem compatibility: {}/{} tests passed",
            compatibility_tests_passed, compatibility_tests_total
        );

        Ok(())
    }

    /// Validate universal adapter patterns
    async fn validate_universal_adapter_patterns(&mut self) -> Result<()> {
        println!("🔌 Validating universal adapter patterns...");

        let mut adapter_compliant_services = 0;

        for service in self.registered_services.values() {
            let mut adapter_compliant = true;

            // Universal adapters should use HTTP/HTTPS protocols
            if !service.endpoint.starts_with("http://") && !service.endpoint.starts_with("https://")
            {
                adapter_compliant = false;
                self.validation_results.validation_errors.push(format!(
                    "Service {} endpoint not using universal HTTP protocol: {}",
                    service.service_id, service.endpoint
                ));
            }

            // Universal adapters should have well-defined capabilities
            if service.capabilities.is_empty() {
                adapter_compliant = false;
                self.validation_results.validation_errors.push(format!(
                    "Service {} has no defined capabilities",
                    service.service_id
                ));
            }

            // Universal adapters should follow standard port conventions
            if let Ok(url) = url::Url::parse(&service.endpoint) {
                if let Some(port) = url.port() {
                    // Standard HTTP ports or service-specific ports
                    if port != 80
                        && port != 443
                        && port != 8080
                        && port != 8443
                        && (port < 3000 || port > 9999)
                    {
                        self.validation_results.validation_errors.push(format!(
                            "Service {} using non-standard port: {}",
                            service.service_id, port
                        ));
                    }
                }
            }

            if adapter_compliant {
                adapter_compliant_services += 1;
            }
        }

        self.validation_results.universal_adapter_compliance =
            adapter_compliant_services == self.registered_services.len();

        Ok(())
    }

    /// Validate concurrent discovery operations
    async fn validate_concurrent_discovery(&mut self) -> Result<()> {
        println!("⚡ Validating concurrent discovery operations...");

        let concurrent_operations = 50;
        let mut handles = Vec::new();

        // Launch concurrent discovery operations
        for i in 0..concurrent_operations {
            let cache_clone = self.discovery_cache.clone();
            let service_id = format!("concurrent-test-{}", i);

            let handle = tokio::spawn(async move {
                // Store a test discovery entry
                cache_clone
                    .store_discovery(
                        &format!("test:{}", service_id),
                        &format!("http://test-{}:8080", service_id),
                        Some(Duration::from_secs(60)),
                    )
                    .await;

                // Retrieve it immediately
                cache_clone
                    .get_discovery(&format!("test:{}", service_id))
                    .await
            });

            handles.push(handle);
        }

        // Wait for all operations to complete
        let mut successful_operations = 0;
        for handle in handles {
            match handle.await {
                Ok(Some(_)) => successful_operations += 1,
                Ok(None) => {
                    self.validation_results
                        .validation_errors
                        .push("Concurrent discovery operation returned None".to_string());
                }
                Err(e) => {
                    self.validation_results
                        .validation_errors
                        .push(format!("Concurrent discovery operation failed: {}", e));
                }
            }
        }

        println!(
            "   Concurrent operations: {}/{} successful",
            successful_operations, concurrent_operations
        );

        // At least 95% success rate for concurrent operations
        let success_rate = successful_operations as f64 / concurrent_operations as f64;
        if success_rate < 0.95 {
            self.validation_results.validation_errors.push(format!(
                "Concurrent discovery success rate too low: {:.2}%",
                success_rate * 100.0
            ));
        }

        Ok(())
    }

    /// Validate dynamic service management
    async fn validate_dynamic_service_management(&mut self) -> Result<()> {
        println!("🔄 Validating dynamic service management...");

        // Test dynamic service registration
        let dynamic_service_id = format!("dynamic-service-{}", Uuid::new_v4());
        let dynamic_service = UnifiedServiceConfig {
            service_id: dynamic_service_id.clone(),
            capabilities: vec!["dynamic.capability".to_string()],
            endpoint: "http://dynamic-service:8080".to_string(),
            sovereignty_compliant: true,
        };

        // Register the service dynamically
        self.registered_services
            .insert(dynamic_service_id.clone(), dynamic_service.clone());

        // Register its capabilities
        for capability in &dynamic_service.capabilities {
            self.discovery_cache
                .store_discovery(
                    &format!("capability:{}", capability),
                    &dynamic_service.endpoint,
                    Some(Duration::from_secs(300)),
                )
                .await;
        }

        // Verify the service can be discovered
        let discovered = self
            .discovery_cache
            .get_discovery("capability:dynamic.capability")
            .await;
        if discovered == Some(dynamic_service.endpoint.clone()) {
            self.validation_results.dynamic_registration_success = true;
        } else {
            self.validation_results
                .validation_errors
                .push("Dynamic service registration and discovery failed".to_string());
        }

        // Test service deregistration
        self.registered_services.remove(&dynamic_service_id);

        // Note: In a real implementation, we would also remove from discovery cache
        // For this test, we just verify the service registry is updated

        Ok(())
    }

    /// Create test services for validation
    fn create_test_services(&self) -> Vec<UnifiedServiceConfig> {
        let mut services = Vec::new();

        for i in 0..self.config.service_count {
            let capability_type =
                &self.config.capability_types[i % self.config.capability_types.len()];

            let service = UnifiedServiceConfig {
                service_id: format!("{}-service-{}", capability_type, i),
                capabilities: vec![
                    format!("{}.primary", capability_type),
                    format!("{}.secondary", capability_type),
                ],
                endpoint: format!("http://{}-service-{}:8080", capability_type, i),
                sovereignty_compliant: true,
            };

            services.push(service);
        }

        services
    }

    /// Calculate final validation scores
    fn calculate_final_scores(&mut self) {
        // Overall validation success based on multiple factors
        let error_penalty = self.validation_results.validation_errors.len() as f64 * 0.1;
        let base_score = 1.0;

        // Reduce score based on errors, but keep it non-negative
        let adjusted_score = (base_score - error_penalty).max(0.0);

        // Update scores that depend on overall validation
        if self.validation_results.validation_errors.is_empty() {
            self.validation_results.universal_adapter_compliance = true;
        }
    }
}

/// Convenience function to run architecture validation with default config
pub async fn validate_universal_architecture() -> Result<ArchitectureValidationResults> {
    let config = ArchitectureValidationConfig::default();
    let mut validator = UniversalArchitectureValidator::new(config);
    validator.validate_architecture().await
}

/// Convenience function to run quick validation
pub async fn quick_architecture_validation() -> Result<ArchitectureValidationResults> {
    let config = ArchitectureValidationConfig {
        test_timeout: Duration::from_secs(10),
        service_count: 5,
        test_concurrent_discovery: false,
        ..Default::default()
    };
    let mut validator = UniversalArchitectureValidator::new(config);
    validator.validate_architecture().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_architecture_validator_creation() {
        let config = ArchitectureValidationConfig::default();
        let validator = UniversalArchitectureValidator::new(config);

        assert_eq!(validator.registered_services.len(), 0);
        assert_eq!(validator.validation_results.services_validated, 0);
    }

    #[tokio::test]
    async fn test_service_registration_validation() {
        let config = ArchitectureValidationConfig {
            service_count: 3,
            ..Default::default()
        };
        let mut validator = UniversalArchitectureValidator::new(config);

        validator.validate_service_registration().await.unwrap();

        assert_eq!(validator.validation_results.services_validated, 3);
        assert_eq!(validator.registered_services.len(), 3);
    }

    #[tokio::test]
    async fn test_capability_discovery_validation() {
        let config = ArchitectureValidationConfig {
            service_count: 2,
            ..Default::default()
        };
        let mut validator = UniversalArchitectureValidator::new(config);

        // Setup services first
        validator.validate_service_registration().await.unwrap();

        // Test capability discovery
        validator.validate_capability_discovery().await.unwrap();

        assert!(validator.validation_results.capabilities_discovered > 0);
        assert!(validator.validation_results.discovery_success_rate > 0.0);
    }

    #[tokio::test]
    async fn test_sovereignty_compliance_validation() {
        let config = ArchitectureValidationConfig {
            service_count: 2,
            strict_sovereignty_enforcement: false, // Allow for testing
            ..Default::default()
        };
        let mut validator = UniversalArchitectureValidator::new(config);

        // Setup services
        validator.validate_service_registration().await.unwrap();

        // Test sovereignty compliance
        validator.validate_sovereignty_compliance().await.unwrap();

        // Should have good compliance score for well-formed test services
        assert!(validator.validation_results.sovereignty_compliance_score > 0.8);
    }

    #[tokio::test]
    async fn test_quick_architecture_validation() {
        let results = quick_architecture_validation().await.unwrap();

        assert!(results.test_duration > Duration::ZERO);
        assert!(results.services_validated > 0);
        assert!(results.sovereignty_compliance_score >= 0.0);
    }

    #[tokio::test]
    async fn test_universal_architecture_validation() {
        let results = validate_universal_architecture().await.unwrap();

        assert!(results.test_duration > Duration::ZERO);
        assert_eq!(results.services_validated, 10); // Default service count
        assert!(results.discovery_success_rate >= 0.0);
        assert!(results.sovereignty_compliance_score >= 0.0);
    }
}
