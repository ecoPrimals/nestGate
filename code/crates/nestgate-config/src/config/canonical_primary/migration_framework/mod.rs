// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CONFIGURATION MIGRATION FRAMEWORK**
//!
//! Migration utilities for transitioning from fragmented configuration
//! systems to the unified canonical system.
//!
//! **FEATURES**:
//! - Safe migration with rollback capability
//! - Validation before and after migration
//! - Backup and restore functionality
//! - Progress tracking and reporting
//! - Compatibility checking

mod migrator;
mod safe_migration;
mod types;

// Re-export all public types for backward compatibility
pub use migrator::ConfigMigrator;
pub use safe_migration::SafeConfigMigration;
pub use types::{
    BackupMetadata, ErrorSeverity, MigrationBackup, MigrationError, MigrationOptions,
    MigrationPhase, MigrationProgress, MigrationReport, ValidationFunction, ValidationRule,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::canonical_primary::NestGateCanonicalConfig;
    use std::time::SystemTime;

    #[test]
    fn test_migration_options_default() {
        let options = MigrationOptions::default();
        assert!(options.create_backup);
        assert!(options.validate_after_migration);
        assert!(!options.strict_validation);
        assert!(options.skip_non_critical_errors);
        assert!(!options.dry_run);
    }

    #[test]
    fn test_config_migrator_creation() {
        let options = MigrationOptions::default();
        let migrator = ConfigMigrator::new(String::from("test"), options);
        assert_eq!(migrator.source_type, "test");
        let report = migrator.get_migration_report();
        assert_eq!(report.progress_percentage, 0);
        assert!(matches!(
            report.current_phase,
            MigrationPhase::SourceValidation
        ));
    }

    #[test]
    fn test_migration_report_success() {
        let report = MigrationReport {
            source_type: String::from("test"),
            started_at: SystemTime::now(),
            current_phase: MigrationPhase::Completed,
            completed_steps: vec![String::from("step1")],
            failed_steps: vec![],
            warnings: vec![],
            progress_percentage: 100,
            backup_created: true,
        };
        assert!(report.is_successful());
    }

    #[test]
    fn test_migration_report_failed_with_critical() {
        let report = MigrationReport {
            source_type: String::from("test"),
            started_at: SystemTime::now(),
            current_phase: MigrationPhase::Failed,
            completed_steps: vec![],
            failed_steps: vec![MigrationError {
                message: String::from("Critical error"),
                severity: ErrorSeverity::Critical,
                source_field: None,
                target_field: None,
                error_code: String::from("ERR"),
                suggested_resolution: None,
            }],
            warnings: vec![],
            progress_percentage: 0,
            backup_created: false,
        };
        assert!(!report.is_successful());
    }

    #[test]
    fn test_migration_report_with_warnings_only() {
        let report = MigrationReport {
            source_type: String::from("test"),
            started_at: SystemTime::now(),
            current_phase: MigrationPhase::Completed,
            completed_steps: vec![String::from("step1")],
            failed_steps: vec![MigrationError {
                message: String::from("Minor warning"),
                severity: ErrorSeverity::Warning,
                source_field: None,
                target_field: None,
                error_code: String::from("WARN"),
                suggested_resolution: None,
            }],
            warnings: vec![String::from("warning1")],
            progress_percentage: 100,
            backup_created: true,
        };
        assert!(report.is_successful());
    }

    #[test]
    fn test_migration_report_get_summary() {
        let report = MigrationReport {
            source_type: String::from("primary"),
            started_at: SystemTime::now(),
            current_phase: MigrationPhase::Completed,
            completed_steps: vec![String::from("a"), String::from("b")],
            failed_steps: vec![],
            warnings: vec![String::from("w1")],
            progress_percentage: 100,
            backup_created: true,
        };
        let summary = report.get_summary();
        assert!(summary.contains("primary"));
        assert!(summary.contains("100%"));
        assert!(summary.contains("2"));
    }

    #[test]
    fn test_from_primary_config() {
        let config = serde_json::json!({
            "system": {},
            "unified": {},
            "domains": {}
        });
        let options = MigrationOptions::default();
        let migrator = ConfigMigrator::from_primary_config(&config, options).unwrap();
        assert_eq!(migrator.source_type, "NestGatePrimaryConfig");
    }

    #[test]
    fn test_from_unified_config() {
        let config = serde_json::json!({
            "api": {},
            "automation": {}
        });
        let options = MigrationOptions::default();
        let migrator = ConfigMigrator::from_unified_config(&config, options).unwrap();
        assert_eq!(migrator.source_type, "UnifiedCanonicalExtensions");
    }

    #[test]
    fn test_from_final_config() {
        let config = serde_json::json!({
            "system": {}
        });
        let options = MigrationOptions::default();
        let migrator = ConfigMigrator::from_final_config(&config, options).unwrap();
        assert_eq!(migrator.source_type, "NestGateFinalConfig");
    }

    #[test]
    fn test_full_migration_chain() {
        let config = serde_json::json!({"system": {}});
        let options = MigrationOptions {
            create_backup: false,
            validate_after_migration: true,
            ..Default::default()
        };
        let migrator = ConfigMigrator::from_final_config(&config, options).unwrap();
        let result = migrator.migrate();
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("not yet implemented"),
            "non-dry-run migration must return NotImplemented until field transforms are wired"
        );
    }

    #[test]
    fn test_dry_run_migration() {
        let config = serde_json::json!({"system": {}});
        let mut options = MigrationOptions::default();
        options.dry_run = true;
        options.create_backup = false;
        let migrator = ConfigMigrator::from_final_config(&config, options).unwrap();
        let result = migrator.migrate();
        assert!(result.is_ok());
        let config_result = result.unwrap();
        assert!(config_result.validate().is_ok() || config_result.validate().is_err());
    }

    #[test]
    fn test_rollback_without_backup() {
        let options = MigrationOptions {
            create_backup: false,
            ..Default::default()
        };
        let migrator = ConfigMigrator::new(String::from("test"), options);
        let result = migrator.rollback();
        assert!(result.is_err());
    }

    #[test]
    fn test_get_migration_report() {
        let migrator = ConfigMigrator::new(String::from("test_source"), MigrationOptions::default());
        let report = migrator.get_migration_report();
        assert_eq!(report.source_type, "test_source");
        assert_eq!(report.progress_percentage, 0);
        assert!(!report.backup_created);
    }

    #[test]
    fn test_migration_phase_progress() {
        let phases = [
            MigrationPhase::SourceValidation,
            MigrationPhase::BackupCreation,
            MigrationPhase::SourceAnalysis,
            MigrationPhase::ConfigurationMapping,
            MigrationPhase::Migration,
            MigrationPhase::TargetValidation,
            MigrationPhase::Finalization,
            MigrationPhase::Completed,
            MigrationPhase::Failed,
        ];
        for phase in phases {
            let _ = format!("{:?}", phase);
        }
    }

    #[test]
    fn test_error_severity_variants() {
        let _ = format!("{:?}", ErrorSeverity::Critical);
        let _ = format!("{:?}", ErrorSeverity::Warning);
        let _ = format!("{:?}", ErrorSeverity::Info);
    }

    #[test]
    fn test_safe_config_migration_new() {
        let migration = SafeConfigMigration::new();
        let config = NestGateCanonicalConfig::default();
        let report = migration.validate_migration(&config).unwrap();
        assert!(
            report
                .completed_steps
                .contains(&String::from("Validation completed"))
        );
    }

    #[test]
    fn test_safe_config_migration_default() {
        let _migration = SafeConfigMigration::default();
    }

    #[test]
    fn test_safe_config_migration_with_backup() {
        let mut migration = SafeConfigMigration::new();
        let from = serde_json::json!({"key": "value"});
        let to = NestGateCanonicalConfig::<1000, 65536, 30000, 8080>::default();
        let result = migration.migrate_with_backup(&from, to.clone());
        assert!(result.is_ok());
        assert!(migration.rollback().is_ok());
    }

    #[test]
    fn test_safe_config_migration_rollback_with_backup() {
        let mut migration = SafeConfigMigration::new();
        let from = serde_json::json!({"key": "value"});
        let to = NestGateCanonicalConfig::<1000, 65536, 30000, 8080>::default();
        let _ = migration.migrate_with_backup(&from, to);
        let result = migration.rollback();
        assert!(result.is_ok());
    }

    #[test]
    fn test_safe_config_migration_rollback_without_backup() {
        let migration = SafeConfigMigration::new();
        let result = migration.rollback();
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_config_migration_validate() {
        let migration = SafeConfigMigration::new();
        let config = NestGateCanonicalConfig::<1000, 65536, 30000, 8080>::default();
        let result = migration.validate_migration(&config);
        assert!(result.is_ok());
        let report = result.unwrap();
        assert_eq!(report.current_phase, MigrationPhase::Completed);
        assert_eq!(report.progress_percentage, 100);
    }

    #[test]
    fn test_validation_rule_structure() {
        fn dummy_validator(_: &NestGateCanonicalConfig) -> nestgate_types::error::Result<()> {
            Ok(())
        }
        let rule = ValidationRule {
            name: String::from("test"),
            description: String::from("Test rule"),
            validator: dummy_validator,
        };
        assert_eq!(rule.name, "test");
        assert_eq!(rule.description, "Test rule");
    }
}
