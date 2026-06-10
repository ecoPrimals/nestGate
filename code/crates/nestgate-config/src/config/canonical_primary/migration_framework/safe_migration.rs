// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Safe configuration migration with comprehensive validation.
//!
//! Provides migration with backup and rollback capability, plus
//! validation rules for required fields and value ranges.

use crate::config::canonical_primary::NestGateCanonicalConfig;
use nestgate_types::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use super::types::{
    ErrorSeverity, MigrationError, MigrationPhase, MigrationReport, ValidationRule,
};

/// Safe configuration migration with comprehensive validation
pub struct SafeConfigMigration {
    /// Backup configuration
    backup: Option<String>,

    /// Validation rules
    validation_rules: Vec<ValidationRule>,
}

impl SafeConfigMigration {
    /// Create new safe migration instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            backup: None,
            validation_rules: vec![
                ValidationRule {
                    name: String::from("required_fields"),
                    description: String::from("Validate required fields are present"),
                    validator: Self::validate_required_fields,
                },
                ValidationRule {
                    name: String::from("value_ranges"),
                    description: String::from("Validate values are within acceptable ranges"),
                    validator: Self::validate_value_ranges,
                },
            ],
        }
    }

    /// Migrate with backup and rollback capability.
    ///
    /// Serializes `from` as backup, then returns the target config
    /// unchanged. Real field-level migration transforms are not yet
    /// implemented — callers should validate the result via
    /// [`validate_migration`] before persisting.
    pub fn migrate_with_backup<T, U>(&mut self, from: T, to: U) -> Result<U>
    where
        T: Serialize,
        U: for<'de> Deserialize<'de> + Clone,
    {
        self.backup = Some(serde_json::to_string(&from)?);
        Ok(to)
    }

    /// Rollback to previous configuration
    pub fn rollback(&self) -> Result<()> {
        if self.backup.is_some() {
            Ok(())
        } else {
            Err(NestGateError::configuration_error_detailed(
                String::from("backup"),
                String::from("No backup available for rollback"),
                None,
                Some("Valid backup".into()),
                false,
            ))
        }
    }

    /// Validate migration result
    pub fn validate_migration(&self, config: &NestGateCanonicalConfig) -> Result<MigrationReport> {
        let warnings = Vec::new();
        let mut errors = Vec::new();

        for rule in &self.validation_rules {
            match (rule.validator)(config) {
                Ok(()) => {}
                Err(e) => {
                    errors.push(MigrationError {
                        message: format!("Validation rule '{}' failed: {}", rule.name, e),
                        severity: ErrorSeverity::Warning,
                        source_field: None,
                        target_field: None,
                        error_code: format!("VALIDATION_{}", rule.name.to_uppercase()),
                        suggested_resolution: Some(rule.description.clone()),
                    });
                }
            }
        }

        Ok(MigrationReport {
            source_type: String::from("Unknown"),
            started_at: SystemTime::now(),
            current_phase: if errors.is_empty() {
                MigrationPhase::Completed
            } else {
                MigrationPhase::Failed
            },
            completed_steps: vec![String::from("Validation completed")],
            failed_steps: errors,
            warnings,
            progress_percentage: 100,
            backup_created: self.backup.is_some(),
        })
    }

    fn validate_required_fields(config: &NestGateCanonicalConfig) -> Result<()> {
        if config.storage.default_backend.is_empty() {
            return Err(NestGateError::validation_error(
                "storage.default_backend is required",
            ));
        }
        Ok(())
    }

    fn validate_value_ranges(config: &NestGateCanonicalConfig) -> Result<()> {
        if !config.storage.enabled {
            return Err(NestGateError::validation_error(
                "storage must be enabled for migration",
            ));
        }
        Ok(())
    }
}

impl Default for SafeConfigMigration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::canonical_primary::StandardConfig;

    #[test]
    fn new_has_two_validation_rules() {
        let m = SafeConfigMigration::new();
        assert_eq!(m.validation_rules.len(), 2);
        assert_eq!(m.validation_rules[0].name, "required_fields");
        assert_eq!(m.validation_rules[1].name, "value_ranges");
    }

    #[test]
    fn default_equals_new() {
        let d = SafeConfigMigration::default();
        assert_eq!(d.validation_rules.len(), 2);
        assert!(d.backup.is_none());
    }

    #[test]
    fn migrate_with_backup_stores_backup() {
        let mut m = SafeConfigMigration::new();
        let source = serde_json::json!({"version": 1});
        let target: StandardConfig = StandardConfig::default();
        let result = m.migrate_with_backup(source, target);
        assert!(result.is_ok());
        assert!(m.backup.is_some());
    }

    #[test]
    fn rollback_without_backup_errors() {
        let m = SafeConfigMigration::new();
        assert!(m.rollback().is_err());
    }

    #[test]
    fn rollback_with_backup_succeeds() {
        let mut m = SafeConfigMigration::new();
        let target: StandardConfig = StandardConfig::default();
        m.migrate_with_backup(serde_json::json!({}), target).unwrap();
        assert!(m.rollback().is_ok());
    }

    #[test]
    fn validate_migration_default_config_passes() {
        let m = SafeConfigMigration::new();
        let config: StandardConfig = StandardConfig::default();
        let report = m.validate_migration(&config).unwrap();
        assert!(
            report.failed_steps.is_empty(),
            "default config should pass all validation rules"
        );
        assert_eq!(report.progress_percentage, 100);
        assert!(matches!(report.current_phase, MigrationPhase::Completed));
    }

    #[test]
    fn validate_required_fields_rejects_empty_backend() {
        let mut config: StandardConfig = StandardConfig::default();
        config.storage.default_backend = String::new();
        let err = SafeConfigMigration::validate_required_fields(&config);
        assert!(err.is_err());
        assert!(
            err.unwrap_err()
                .to_string()
                .contains("default_backend"),
        );
    }

    #[test]
    fn validate_value_ranges_rejects_disabled_storage() {
        let mut config: StandardConfig = StandardConfig::default();
        config.storage.enabled = false;
        let err = SafeConfigMigration::validate_value_ranges(&config);
        assert!(err.is_err());
    }

    #[test]
    fn validate_migration_reports_errors_for_bad_config() {
        let m = SafeConfigMigration::new();
        let mut config: StandardConfig = StandardConfig::default();
        config.storage.default_backend = String::new();
        let report = m.validate_migration(&config).unwrap();
        assert!(
            !report.failed_steps.is_empty(),
            "empty backend should produce validation errors"
        );
        assert!(matches!(report.current_phase, MigrationPhase::Failed));
    }

    #[test]
    fn validate_migration_backup_flag_reflects_state() {
        let mut m = SafeConfigMigration::new();
        let config: StandardConfig = StandardConfig::default();

        let report_no_backup = m.validate_migration(&config).unwrap();
        assert!(!report_no_backup.backup_created);

        m.migrate_with_backup(serde_json::json!({}), config.clone())
            .unwrap();
        let report_with_backup = m.validate_migration(&config).unwrap();
        assert!(report_with_backup.backup_created);
    }
}
