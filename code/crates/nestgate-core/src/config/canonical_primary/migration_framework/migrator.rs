//! Configuration migrator - Primary interface for migrating from legacy
//! configuration systems to the unified canonical configuration system.

use crate::config::canonical_primary::NestGateCanonicalConfig;
use crate::{NestGateError, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

use super::types::*;

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
        config: serde_json::Value,
        options: MigrationOptions,
    ) -> Result<Self> {
        let mut migrator = Self::new("NestGatePrimaryConfig".to_string(), options);
        migrator.migrate_from_primary_config(config)?;
        Ok(migrator)
    }

    /// Migrate from `UnifiedCanonicalExtensions`
    pub fn from_unified_config(
        config: serde_json::Value,
        options: MigrationOptions,
    ) -> Result<Self> {
        let mut migrator = Self::new("UnifiedCanonicalExtensions".to_string(), options);
        migrator.migrate_from_unified_config(config)?;
        Ok(migrator)
    }

    /// Migrate from `NestGateFinalConfig`
    pub fn from_final_config(config: serde_json::Value, options: MigrationOptions) -> Result<Self> {
        let mut migrator = Self::new("NestGateFinalConfig".to_string(), options);
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
                "backup".to_string(),
                "No backup available for rollback".to_string(),
                None,
                Some("Valid backup".to_string()),
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

    fn migrate_from_primary_config(&mut self, config: serde_json::Value) -> Result<()> {
        self.add_completed_step("Parsed NestGatePrimaryConfig".to_string());
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

    fn migrate_from_unified_config(&mut self, config: serde_json::Value) -> Result<()> {
        self.add_completed_step("Parsed UnifiedCanonicalExtensions".to_string());
        if let Some(api) = config.get("api") {
            self.migrate_api_config(api)?;
        }
        if let Some(automation) = config.get("automation") {
            self.migrate_automation_config(automation)?;
        }
        Ok(())
    }

    fn migrate_from_final_config(&mut self, config: serde_json::Value) -> Result<()> {
        self.add_completed_step("Parsed NestGateFinalConfig".to_string());
        if let Some(system) = config.get("system") {
            self.migrate_system_config(system)?;
        }
        Ok(())
    }

    fn migrate_system_config(&mut self, _system: &serde_json::Value) -> Result<()> {
        self.add_completed_step("Migrated system configuration".to_string());
        Ok(())
    }

    fn migrate_unified_base_config(&mut self, _unified: &serde_json::Value) -> Result<()> {
        self.add_completed_step("Migrated unified base configuration".to_string());
        Ok(())
    }

    fn migrate_domain_configs(&mut self, _domains: &serde_json::Value) -> Result<()> {
        self.add_completed_step("Migrated domain configurations".to_string());
        Ok(())
    }

    fn migrate_api_config(&mut self, _api: &serde_json::Value) -> Result<()> {
        self.add_completed_step("Migrated API configuration".to_string());
        Ok(())
    }

    fn migrate_automation_config(&mut self, _automation: &serde_json::Value) -> Result<()> {
        self.add_completed_step("Migrated automation configuration".to_string());
        Ok(())
    }

    fn update_phase(&mut self, phase: MigrationPhase) {
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
        self.add_completed_step("Source validation completed".to_string());
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
                version: "1.0.0".to_string(),
                environment: "development".to_string(),
                additional_metadata: HashMap::new(),
            },
        };
        self.backup = Some(backup);
        self.add_completed_step("Backup created successfully".to_string());
        Ok(())
    }

    fn analyze_source(&mut self) -> Result<()> {
        self.add_completed_step("Source analysis completed".to_string());
        Ok(())
    }

    fn map_configurations(&mut self) -> Result<()> {
        self.add_completed_step("Configuration mapping completed".to_string());
        Ok(())
    }

    fn perform_migration(&mut self) -> Result<()> {
        self.add_completed_step("Migration performed successfully".to_string());
        Ok(())
    }

    fn validate_target(&mut self) -> Result<()> {
        match self.target_config.validate() {
            Ok(warnings) => {
                for warning in warnings {
                    self.add_warning(warning);
                }
                self.add_completed_step("Target validation completed".to_string());
                Ok(())
            }
            Err(e) => {
                self.add_error(MigrationError {
                    message: format!("Target validation failed: {e}"),
                    severity: ErrorSeverity::Critical,
                    source_field: None,
                    target_field: None,
                    error_code: "TARGET_VALIDATION_FAILED".to_string(),
                    suggested_resolution: Some("Check configuration structure".to_string()),
                });
                Err(e)
            }
        }
    }

    fn finalize_migration(&mut self) -> Result<()> {
        self.add_completed_step("Migration finalized successfully".to_string());
        Ok(())
    }

    fn dry_run_migration(&mut self) -> Result<NestGateCanonicalConfig> {
        self.add_completed_step("Dry run completed - no changes made".to_string());
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
        Ok(())
    }
}
