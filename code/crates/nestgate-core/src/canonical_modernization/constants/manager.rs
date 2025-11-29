// **CONSTANTS CONSOLIDATION MANAGER**
//! Manager functionality and utilities.
// Core management logic for constants consolidation system.

use crate::Result;
use crate::error::NestGateError;
use std::collections::HashMap;

use super::types::*;
use super::{network, storage};

/// **CONSTANTS CONSOLIDATION MANAGER**
/// Handles systematic migration of scattered constants to canonical system
#[derive(Debug)]
/// Manager for ConstantsConsolidation operations
pub struct ConstantsConsolidationManager {
    /// Consolidation statistics
    pub stats: ConsolidationStats,
    /// Consolidation warnings and issues
    pub warnings: Vec<ConsolidationWarning>,
    /// Constants registry mapping
    pub constants_registry: HashMap<String, ConstantDefinition>,
    /// Domain mappings
    pub domain_mappings: HashMap<String, Vec<String>>,
}
impl ConstantsConsolidationManager {
    /// Create new consolidation manager
    #[must_use]
    pub fn new() -> Self {
        let mut manager = Self {
            stats: ConsolidationStats::default(),
            warnings: Vec::new(),
            constants_registry: HashMap::new(),
            domain_mappings: HashMap::new(),
        };

        manager.initialize_canonical_constants();
        manager
    }

    /// Initialize canonical constants system
    fn initialize_canonical_constants(&mut self) {
        // Register network constants
        let network_constants = network::register_network_constants();
        self.register_domain_constants("network", network_constants);

        // Register storage constants
        let storage_constants = storage::register_storage_constants();
        self.register_domain_constants("storage", storage_constants);
    }

    /// Register constants for a specific domain
    fn register_domain_constants(
        &mut self,
        domain: &str,
        constants: Vec<(String, ConstantValue, String)>,
    ) {
        let mut domain_constants = Vec::new();

        for (name, value, description) in constants {
            let const_type = match &value {
                ConstantValue::String(_) => "&str",
                ConstantValue::Integer(_) => "i64",
                ConstantValue::UnsignedInteger(_) => "u64",
                ConstantValue::Float(_) => "f64",
                ConstantValue::Boolean(_) => "bool",
                ConstantValue::Duration(_) => "u64",
                ConstantValue::Size(_) => "u64",
            };

            let const_def = ConstantDefinition {
                name: name.clone(),
                value,
                const_type: const_type.to_string(),
                description,
                domain: domain.to_string(),
            };

            self.constants_registry.insert(name.clone(), const_def);
            domain_constants.push(name);
        }

        self.domain_mappings
            .insert(domain.to_string(), domain_constants);
    }

    /// Consolidate scattered constants found in the codebase
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn consolidate_scattered_constants(
        &mut self,
        scattered: &[ScatteredConstant],
    ) -> Result<ConsolidationResult>  {
        let mut consolidated = Vec::new();
        let mut duplicates = Vec::new();
        let mut hardcoded_replacements = Vec::new();
        let mut warnings = Vec::new();

        for constant in scattered {
            // Check if this constant already exists
            if let Some(existing) = self.constants_registry.get(&constant.name) {
                if self.values_match(&existing.value, &constant.value) {
                    duplicates.push(constant.clone());
                    self.stats.duplicates_eliminated += 1;
                } else {
                    warnings.push(ConsolidationWarning {
                        warning_type: WarningType::DuplicateConstant,
                        message: format!("Constant '{constant.name}' has conflicting values"),
                        location: constant.location.clone(),
                        severity: WarningSeverity::High,
                    );
                }
            } else {
                // New constant - add to registry
                let const_def = ConstantDefinition {
                    name: constant.name.clone(),
                    bvalue: constant.value.clone(),
                    const_type: constant.const_type.clone(),
                    description: constant.description.clone(),
                    domain: "custom".to_string(),
                };

                self.constants_registry
                    .insert(constant.name.clone(), const_def);
                consolidated.push(constant.clone());
                self.stats.consolidated_count += 1;

                if constant.replaces_hardcoded {
                    hardcoded_replacements.push(constant.clone());
                    self.stats.hardcodedvalues_replaced += 1;
                }
            }
        }

        self.stats.total_constants = self.constants_registry.len() as u32;
        self.stats.consolidation_progress =
            (self.stats.consolidated_count as f64 / self.stats.total_constants as f64) * 100.0;

        self.warnings.extend(warnings.clone());

        Ok(ConsolidationResult {
            consolidated_constants: consolidated,
            duplicates_found: duplicates,
            hardcoded_replacements,
            warnings,
        })
    }

    /// Generate consolidated constants module code
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn generate_constants_module(&self, domain: &str) -> Result<String>  {
        let domain_constants =
            self.domain_mappings
                .get(domain)
                .ok_or_else(|| NestGateError::internal_error(
                        "Failed to generate constants module for domain '{)': domain not found",
                        domain
                    ),
                    component: "constants_consolidation".to_string(),
                    location: Some("generate_constants_module".to_string())context: Some(Box::new(crate::error::context::ErrorContext::new(
                        "constants_consolidation",
                        "generate_constants_module",
                    ))),
                )?;

        let mut module_code = format!(
            "//! **{} CONSTANTS MODULE**\n//!\n//! Consolidated constants for {} domain\n//! Generated by ConstantsConsolidationManager\n\n",
            domain.to_uppercase(),
            domain
        );

        for const_name in domain_constants {
            if let Some(const_def) = self.constants_registry.get(const_name) {
                let value_str = match &const_def.value {
                    ConstantValue::String(s) => format!("\"{}\"", s),
                    ConstantValue::Integer(i) => i.to_string(),
                    ConstantValue::UnsignedInteger(u) => u.to_string(),
                    ConstantValue::Float(f) => f.to_string(),
                    ConstantValue::Boolean(b) => b.to_string(),
                    ConstantValue::Duration(d) => d.to_string(),
                    ConstantValue::Size(s) => s.to_string(),
                };

                module_code.push_str(&format!(
                    "/// {}\npub const {}: {} = {};\n\n",
                    const_def.description, const_name, const_def.const_type, value_str
                ));
            }
        }

        Ok(module_code)
    }

    /// Check if two constant values match
    fn values_match(&self, a: &ConstantValue, b: &ConstantValue) -> bool {
        match (a, b) {
            (ConstantValue::String(a), ConstantValue::String(b)) => a == b,
            (ConstantValue::Integer(a), ConstantValue::Integer(b)) => a == b,
            (ConstantValue::UnsignedInteger(a), ConstantValue::UnsignedInteger(b)) => a == b,
            (ConstantValue::Float(a), ConstantValue::Float(b)) => (a - b).abs() < f64::EPSILON,
            (ConstantValue::Boolean(a), ConstantValue::Boolean(b)) => a == b,
            (ConstantValue::Duration(a), ConstantValue::Duration(b)) => a == b,
            (ConstantValue::Size(a), ConstantValue::Size(b)) => a == b,
            _ => false,
        }
    }

    /// Get consolidation summary
    pub fn get_summary(&self) -> ConsolidationSummary {
        ConsolidationSummary {
            stats: self.stats.clone(),
            warnings_count: self.warnings.len(),
            total_domains: self.domain_mappings.len(),
            canonical_constants_count: self.constants_registry.len(),
            estimated_maintenance_reduction: self.calculate_maintenance_reduction(),
        }
    }

    /// Calculate estimated maintenance reduction percentage
    fn calculate_maintenance_reduction(&self) -> f64 {
        // Estimate based on duplicates eliminated and consolidation ratio
        let duplicate_reduction = self.stats.duplicates_eliminated as f64 * 0.1;
        let consolidation_benefit = self.stats.consolidation_progress * 0.01;
        (duplicate_reduction + consolidation_benefit).min(95.0)
    }
}

impl Default for ConstantsConsolidationManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}
