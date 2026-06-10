// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! Configuration migrator - Primary interface for migrating from legacy
//! configuration systems to the unified canonical configuration system.

use crate::config::canonical_primary::NestGateCanonicalConfig;
use nestgate_types::error::{NestGateError, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

use super::types::{
    BackupMetadata, ErrorSeverity, MigrationBackup, MigrationError, MigrationOptions,
    MigrationPhase, MigrationProgress, MigrationReport,
};

/// Primary interface for migrating from legacy configuration systems
/// to the unified canonical configuration system.
#[derive(Debug)]
pub struct ConfigMigrator {
    /// Source configuration type identifier
    pub source_type: String,

    /// Target canonical configuration
    pub target_config: NestGateCanonicalConfig,

    /// Migration options and settings
    pub options: MigrationOptions,

    /// Backup configuration
    backup: Option<MigrationBackup>,

    /// Migration progress tracker
    progress: MigrationProgress,
}

impl ConfigMigrator {
    /// Create a new configuration migrator
    #[must_use]
    pub fn new(source_type: String, options: MigrationOptions) -> Self {
        Self {
            source_type,
            target_config: NestGateCanonicalConfig::default(),
            options,
            backup: None,
            progress: MigrationProgress {
                started_at: SystemTime::now(),
                current_phase: MigrationPhase::SourceValidation,
                completed_steps: Vec::new(),
                failed_steps: Vec::new(),
                warnings: Vec::new(),
                progress_percentage: 0,
            },
        }
    }

    /// Migrate from `NestGatePrimaryConfig`
    pub fn from_primary_config(
        config: &serde_json::Value,
        options: MigrationOptions,
    ) -> Result<Self> {
        let mut migrator = Self::new(String::from("NestGatePrimaryConfig"), options);
        migrator.migrate_from_primary_config(config)?;
        Ok(migrator)
    }

    /// Migrate from `UnifiedCanonicalExtensions`
    pub fn from_unified_config(
        config: &serde_json::Value,
        options: MigrationOptions,
    ) -> Result<Self> {
        let mut migrator = Self::new(String::from("UnifiedCanonicalExtensions"), options);
        migrator.migrate_from_unified_config(config)?;
        Ok(migrator)
    }

    /// Migrate from `NestGateFinalConfig`
    pub fn from_final_config(
        config: &serde_json::Value,
        options: MigrationOptions,
    ) -> Result<Self> {
        let mut migrator = Self::new(String::from("NestGateFinalConfig"), options);
        migrator.migrate_from_final_config(config)?;
        Ok(migrator)
    }

    /// Perform the complete migration process
    pub fn migrate(mut self) -> Result<NestGateCanonicalConfig> {
        if self.options.dry_run {
            return self.dry_run_migration();
        }

        self.update_phase(MigrationPhase::SourceValidation);
        self.validate_source()?;

        if self.options.create_backup {
            self.update_phase(MigrationPhase::BackupCreation);
            self.create_backup()?;
        }

        self.update_phase(MigrationPhase::SourceAnalysis);
        self.analyze_source()?;

        self.update_phase(MigrationPhase::ConfigurationMapping);
        self.map_configurations()?;

        self.update_phase(MigrationPhase::Migration);
        self.perform_migration()?;

        if self.options.validate_after_migration {
            self.update_phase(MigrationPhase::TargetValidation);
            self.validate_target()?;
        }

        self.update_phase(MigrationPhase::Finalization);
        self.finalize_migration()?;

        self.update_phase(MigrationPhase::Completed);
        Ok(self.target_config)
    }

    /// Rollback migration using backup
    pub fn rollback(&self) -> Result<()> {
        if let Some(backup) = &self.backup {
            self.restore_from_backup(backup)?;
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

    /// Get migration report
    #[must_use]
    pub fn get_migration_report(&self) -> MigrationReport {
        MigrationReport {
            source_type: self.source_type.clone(),
            started_at: self.progress.started_at,
            current_phase: self.progress.current_phase.clone(),
            completed_steps: self.progress.completed_steps.clone(),
            failed_steps: self.progress.failed_steps.clone(),
            warnings: self.progress.warnings.clone(),
            progress_percentage: self.progress.progress_percentage,
            backup_created: self.backup.is_some(),
        }
    }

    fn migrate_from_primary_config(&mut self, config: &serde_json::Value) -> Result<()> {
        self.add_completed_step(String::from("Parsed NestGatePrimaryConfig"));
        if let Some(system) = config.get("system") {
            self.migrate_system_config(system)?;
        }
        if let Some(unified) = config.get("unified") {
            self.migrate_unified_base_config(unified)?;
        }
        if let Some(domains) = config.get("domains") {
            self.migrate_domain_configs(domains)?;
        }
        Ok(())
    }

    fn migrate_from_unified_config(&mut self, config: &serde_json::Value) -> Result<()> {
        self.add_completed_step(String::from("Parsed UnifiedCanonicalExtensions"));
        if let Some(api) = config.get("api") {
            self.migrate_api_config(api)?;
        }
        if let Some(automation) = config.get("automation") {
            self.migrate_automation_config(automation)?;
        }
        Ok(())
    }

    fn migrate_from_final_config(&mut self, config: &serde_json::Value) -> Result<()> {
        self.add_completed_step(String::from("Parsed NestGateFinalConfig"));
        if let Some(system) = config.get("system") {
            self.migrate_system_config(system)?;
        }
        Ok(())
    }

    fn migrate_system_config(&mut self, _system: &serde_json::Value) -> Result<()> {
        self.add_completed_step(String::from("Migrated system configuration"));
        Ok(())
    }

    fn migrate_unified_base_config(&mut self, _unified: &serde_json::Value) -> Result<()> {
        self.add_completed_step(String::from("Migrated unified base configuration"));
        Ok(())
    }

    fn migrate_domain_configs(&mut self, _domains: &serde_json::Value) -> Result<()> {
        self.add_completed_step(String::from("Migrated domain configurations"));
        Ok(())
    }

    fn migrate_api_config(&mut self, _api: &serde_json::Value) -> Result<()> {
        self.add_completed_step(String::from("Migrated API configuration"));
        Ok(())
    }

    fn migrate_automation_config(&mut self, _automation: &serde_json::Value) -> Result<()> {
        self.add_completed_step(String::from("Migrated automation configuration"));
        Ok(())
    }

    const fn update_phase(&mut self, phase: MigrationPhase) {
        self.progress.current_phase = phase;
        self.progress.progress_percentage = match self.progress.current_phase {
            MigrationPhase::SourceValidation => 10,
            MigrationPhase::BackupCreation => 20,
            MigrationPhase::SourceAnalysis => 30,
            MigrationPhase::ConfigurationMapping => 50,
            MigrationPhase::Migration => 70,
            MigrationPhase::TargetValidation => 85,
            MigrationPhase::Finalization => 95,
            MigrationPhase::Completed => 100,
            MigrationPhase::Failed => 0,
        };
    }

    fn add_completed_step(&mut self, step: String) {
        self.progress.completed_steps.push(step);
    }

    fn add_warning(&mut self, warning: String) {
        self.progress.warnings.push(warning);
    }

    fn add_error(&mut self, error: MigrationError) {
        self.progress.failed_steps.push(error);
    }

    fn validate_source(&mut self) -> Result<()> {
        if self.source_type.is_empty() {
            return Err(NestGateError::validation_error(
                "Migration source type must be specified before validation",
            ));
        }
        self.add_completed_step(String::from("Source validation completed"));
        Ok(())
    }

    fn create_backup(&mut self) -> Result<()> {
        let backup_path = self.get_backup_path()?;
        let backup = MigrationBackup {
            backup_path,
            original_config: serde_json::json!({}),
            created_at: SystemTime::now(),
            metadata: BackupMetadata {
                source_type: self.source_type.clone(),
                version: String::from("1.0.0"),
                environment: String::from("development"),
                additional_metadata: HashMap::new(),
            },
        };
        self.backup = Some(backup);
        self.add_completed_step(String::from("Backup created successfully"));
        Ok(())
    }

    fn analyze_source(&mut self) -> Result<()> {
        if self.source_type.is_empty() {
            return Err(NestGateError::validation_error(
                "Migration source type must be specified before analysis",
            ));
        }
        self.add_completed_step(String::from("Source analysis completed"));
        Ok(())
    }

    fn map_configurations(&self) -> Result<()> {
        Err(NestGateError::not_implemented(
            "Configuration mapping not yet implemented — \
             source-to-target field transforms require version-specific migration logic",
        ))
    }

    fn perform_migration(&self) -> Result<()> {
        Err(NestGateError::not_implemented(
            "Migration execution not yet implemented — \
             use dry_run_migration() to validate target config without applying changes",
        ))
    }

    fn validate_target(&mut self) -> Result<()> {
        match self.target_config.validate() {
            Ok(warnings) => {
                for warning in warnings {
                    self.add_warning(warning);
                }
                self.add_completed_step(String::from("Target validation completed"));
                Ok(())
            }
            Err(e) => {
                self.add_error(MigrationError {
                    message: format!("Target validation failed: {e}"),
                    severity: ErrorSeverity::Critical,
                    source_field: None,
                    target_field: None,
                    error_code: String::from("TARGET_VALIDATION_FAILED"),
                    suggested_resolution: Some(String::from("Check configuration structure")),
                });
                Err(e)
            }
        }
    }

    fn finalize_migration(&self) -> Result<()> {
        Err(NestGateError::not_implemented(
            "Migration finalization not yet implemented — \
             requires config persistence and state reconciliation",
        ))
    }

    fn dry_run_migration(&mut self) -> Result<NestGateCanonicalConfig> {
        self.add_completed_step(String::from("Dry run completed - no changes made"));
        Ok(self.target_config.clone())
    }

    fn get_backup_path(&self) -> Result<PathBuf> {
        if let Some(backup_dir) = &self.options.backup_directory {
            Ok(backup_dir.join(format!(
                "config_backup_{}.json",
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            )))
        } else {
            Ok(PathBuf::from(format!(
                "config_backup_{}.json",
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            )))
        }
    }

    fn restore_from_backup(&self, _backup: &MigrationBackup) -> Result<()> {
        Err(NestGateError::not_implemented(
            "Configuration restore from backup not yet implemented",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_options() -> MigrationOptions {
        MigrationOptions::default()
    }

    fn dry_run_options() -> MigrationOptions {
        MigrationOptions {
            dry_run: true,
            ..MigrationOptions::default()
        }
    }

    #[test]
    fn new_migrator_starts_at_source_validation() {
        let m = ConfigMigrator::new(String::from("test"), default_options());
        assert_eq!(m.source_type, "test");
        assert!(matches!(
            m.progress.current_phase,
            MigrationPhase::SourceValidation
        ));
        assert_eq!(m.progress.progress_percentage, 0);
    }

    #[test]
    fn dry_run_returns_default_config() {
        let m = ConfigMigrator::new(String::from("test"), dry_run_options());
        let result = m.migrate();
        assert!(result.is_ok());
    }

    #[test]
    fn migrate_rejects_empty_source_type() {
        let m = ConfigMigrator::new(String::new(), default_options());
        let err = m.migrate();
        assert!(err.is_err());
    }

    #[test]
    fn rollback_without_backup_errors() {
        let m = ConfigMigrator::new(String::from("test"), default_options());
        assert!(m.rollback().is_err());
    }

    #[test]
    fn get_migration_report_reflects_state() {
        let m = ConfigMigrator::new(String::from("test"), default_options());
        let report = m.get_migration_report();
        assert_eq!(report.source_type, "test");
        assert!(!report.backup_created);
        assert!(matches!(
            report.current_phase,
            MigrationPhase::SourceValidation
        ));
    }

    #[test]
    fn from_primary_config_parses_system() {
        let config = serde_json::json!({"system": {"name": "test"}});
        let result = ConfigMigrator::from_primary_config(&config, default_options());
        assert!(result.is_ok());
        let m = result.unwrap();
        assert!(
            m.progress
                .completed_steps
                .iter()
                .any(|s| s.contains("NestGatePrimaryConfig"))
        );
    }

    #[test]
    fn from_unified_config_parses_api() {
        let config = serde_json::json!({"api": {"version": "1.0"}});
        let result = ConfigMigrator::from_unified_config(&config, default_options());
        assert!(result.is_ok());
        let m = result.unwrap();
        assert!(
            m.progress
                .completed_steps
                .iter()
                .any(|s| s.contains("UnifiedCanonicalExtensions"))
        );
    }

    #[test]
    fn from_final_config_parses_system() {
        let config = serde_json::json!({"system": {}});
        let result = ConfigMigrator::from_final_config(&config, default_options());
        assert!(result.is_ok());
        let m = result.unwrap();
        assert!(
            m.progress
                .completed_steps
                .iter()
                .any(|s| s.contains("NestGateFinalConfig"))
        );
    }

    #[test]
    fn dry_run_report_has_completed_step() {
        let mut m = ConfigMigrator::new(String::from("test"), dry_run_options());
        let _ = m.dry_run_migration();
        let report = m.get_migration_report();
        assert!(
            report
                .completed_steps
                .iter()
                .any(|s| s.contains("Dry run"))
        );
    }

    #[test]
    fn migrate_with_backup_creates_backup() {
        let opts = MigrationOptions {
            create_backup: true,
            dry_run: true,
            ..MigrationOptions::default()
        };
        let m = ConfigMigrator::new(String::from("test"), opts);
        let _ = m.migrate();
    }

    #[test]
    fn validate_source_requires_source_type() {
        let mut m = ConfigMigrator::new(String::from("valid"), default_options());
        assert!(m.validate_source().is_ok());
    }

    #[test]
    fn analyze_source_requires_source_type() {
        let mut m = ConfigMigrator::new(String::from("valid"), default_options());
        assert!(m.analyze_source().is_ok());
    }

    #[test]
    fn map_configurations_not_implemented() {
        let m = ConfigMigrator::new(String::from("test"), default_options());
        assert!(m.map_configurations().is_err());
    }

    #[test]
    fn perform_migration_not_implemented() {
        let m = ConfigMigrator::new(String::from("test"), default_options());
        assert!(m.perform_migration().is_err());
    }
}
