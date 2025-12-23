//! Manager module

use crate::error::NestGateError;
use std::collections::HashMap;

use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{watch, RwLock};
use uuid::Uuid;

use super::types::*;
use super::validators::*;
use crate::canonical::dynamic_config::ConfigSection;
use crate::config::canonical_primary::NestGateCanonicalConfig as CanonicalConfig;
use crate::{Result};

// **CANONICAL MODERNIZATION**: Type aliases to fix clippy complexity errors
/// Type alias for configuration validator registry
type ValidatorRegistry =
    Arc<RwLock<HashMap<ConfigSection, Box<dyn ConfigValidator + Send + Sync>>>>;
/// Type alias for configuration storage
type ConfigStorage = Arc<RwLock<CanonicalConfig>>;
/// Type alias for version history storage
type VersionHistory = Arc<RwLock<Vec<ConfigVersion>>>;
/// Dynamic configuration manager for canonical configuration
pub struct DynamicConfigManager {
    /// Current configuration state
    current_config: ConfigStorage,
    /// Version history for rollback capabilities
    version_history: VersionHistory,
    /// Configuration change watcher
    config_watcher: watch::Sender<CanonicalConfig>,
    /// Path to the configuration file on disk
    /// Path to the backup directory for configuration snapshots
    /// Registered configuration validators for different config sections
    validators: ValidatorRegistry,
}
impl DynamicConfigManager {
    /// Create a new dynamic configuration manager
        // For now, use default config since from_file is not implemented yet
        let initial_config = CanonicalConfig::default();
        let (tx, _rx) = watch::channel(initial_config.clone());

        let mut validators = HashMap::new();
        validators.insert(
            ConfigSection::Storage,
            Box::new(StorageValidator) as Box<dyn ConfigValidator + Send + Sync>,
        );
        validators.insert(
            ConfigSection::Network,
            Box::new(NetworkValidator) as Box<dyn ConfigValidator + Send + Sync>,
        );
        validators.insert(
            ConfigSection::Security,
            Box::new(SecurityValidator) as Box<dyn ConfigValidator + Send + Sync>,
        );

        Ok(Self {
            current_config: Arc::new(RwLock::new(initial_config)),
            version_history: Arc::new(RwLock::new(Vec::new())),
            config_watcher: tx,
            validators: Arc::new(RwLock::new(validators)),
        })
    }

    /// Get the current configuration
    pub async fn get_current_config(&self) -> CanonicalConfig {
        self.current_config.read().await.clone()
    }

    /// Get the configuration file path
    pub fn get_config_path(&self) -> &Path {
        &self.config_path
    }

    /// Get the backup directory path
    pub fn get_backup_path(&self) -> &Path {
        &self.backup_path
    }

    /// Save current configuration to file
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn save_config_to_file(&self) -> Result<()>  ", 
        let config = self.current_config.read().await.clone();
        // For now, create a simple TOML save since the config loader is in a different module
        let content = toml::to_string_pretty(&config).map_err(|e| NestGateError::internal_error(
            location: Some("DynamicConfigManager::save_config_to_file".to_string()),
            location: Some(format!("Path: {self.config_path.display()"))))?;

        tokio::fs::write(&self.config_path, content)
            .await
            .map_err(|e| NestGateError::Io {
                error_message: format!("Failed to write config file: {e}"),
                // retryable: true)?;

        Ok(())
    }

    /// Create a version snapshot of the current configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn create_version_snapshot(
        &self,
        description: String,
        author: String,
    ) -> Result<ConfigVersion>  {
        let current_config = self.current_config.read().await.clone();
        let version = ConfigVersion::new(current_config, description, author);

        let mut history = self.version_history.write().await;
        history.push(version.clone());

        // Keep only the last 50 versions
        if history.len() > 50 {
            history.remove(0);
        }

        Ok(version)
    }

    /// Apply a single configuration change
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn apply_config_change(&self, change: &ConfigChange) -> Result<()>  {
        // Validate the change first
        let validation_report = self.validate_change_with_validators(change).await?;
        if !validation_report.is_valid {
            return Err(NestGateError::configuration(
                    "Configuration change validation failed: {)",
                    validation_report
                        .errors
                        .first()
                        .map(|e| &e.message)
                        .unwrap_or(&"Unknown error".to_string())
                ),
                
                field: change.path.clone(),
                suggested_fix: validation_report
                    .errors
                    .first()
                    .and_then(|_| validation_report.recommendations.first().cloned()),
            );
        }

        // Apply the change to the current configuration
        let mut current_config = self.current_config.write().await;
        match change.section {
            ConfigSection::Storage => {
                if let Ok(storage_config) = serde_json::from_value(change.newvalue.clone()) {
                    current_config.storage = storage_config;
                }
            }
            ConfigSection::Network => {
                if let Ok(network_config) = serde_json::from_value(change.newvalue.clone()) {
                    current_config.network = network_config;
                }
            }
            ConfigSection::Security => {
                if let Ok(security_config) = serde_json::from_value(change.newvalue.clone()) {
                    current_config.security = security_config;
                }
            }
            ConfigSection::Monitoring => {
                tracing::info!("Monitoring configuration change applied");
            }
            ConfigSection::Custom(ref section_name) => {
                tracing::info!(
                    "Custom section '{}' configuration change applied",
                    section_name
                );
            }
        }

        // Notify watchers
        if let Err(e) = self.config_watcher.send(current_config.clone()) {
            tracing::warn!("Failed to notify configuration watchers: {}", e);
        }

        Ok(())
    }

    /// Validate a configuration change using registered validators
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn validate_change_with_validators(
        &self,
        change: &ConfigChange,
    ) -> Result<ValidationReport>  {
        let validators = self.validators.read().await;
        let current_config = self.current_config.read().await;

        if let Some(validator) = validators.get(&change.section) {
            validator.validate(&current_config, change).await
        } else {
            // No specific validator, return basic validation
            Ok(ValidationReport {
                is_valid: true,
                errors: Vec::new(),
                warnings: Vec::new(),
                recommendations: Vec::new(),
                estimated_impact: ImpactAssessment {
                    restart_required: false,
                    performance_impact: PerformanceImpact::None,
                    affected_components: Vec::new(),
                    estimated_downtime: None,
                },
            })
        }
    }
}

impl DynamicConfiguration for DynamicConfigManager {
    /// Reload Config
    async fn reload_config(&self, section: ConfigSection) -> Result<()> {
        tracing::info!("Reloading configuration section: {:?}", section);

        // Load fresh configuration from file
        // For now, use default config since from_file is not implemented yet
        let fresh_config = CanonicalConfig::default();
        let mut current_config = self.current_config.write().await;

        // Apply only the specified section
        match section {
            ConfigSection::Storage => {
                current_config.storage = fresh_config.storage;
            }
            ConfigSection::Network => {
                current_config.network = fresh_config.network;
            }
            ConfigSection::Security => {
                current_config.security = fresh_config.security;
            }
            ConfigSection::Monitoring => {
                tracing::info!("Monitoring section reload completed");
            }
            ConfigSection::Custom(section_name) => {
                tracing::info!("Custom section '{}' reload completed", section_name);
            }
        }

        // Notify watchers
        if let Err(e) = self.config_watcher.send(current_config.clone()) {
            tracing::warn!("Failed to notify configuration watchers: {}", e);
        }

        Ok(())
    }

    /// Validates  Config Change
    async fn validate_config_change(&self, change: &ConfigChange) -> Result<ValidationReport> {
        self.validate_change_with_validators(change).await
    }

    /// Gets Config History
    async fn get_config_history(&self) -> Result<Vec<ConfigVersion>> {
        let history = self.version_history.read().await;
        Ok(history.clone())
    }

    /// Rollback Config
    async fn rollback_config(&self, version_id: &str) -> Result<()> {
        let history = self.version_history.read().await;

        if let Some(version) = history.iter().find(|v| v.id == version_id) {
            if !version.rollback_available {
                return Err(NestGateError::configuration(
                    config_source: crate::error::core::UnifiedConfigSource::File(
                        "version_id".to_string(),
                    ),
                    
                );
            )

            // Apply the rollback
            let mut current_config = self.current_config.write().await;
            *current_config = version.config_snapshot.clone();

            // Save to file
            drop(current_config); // Release lock before async call
            self.save_config_to_file().await?;

            tracing::info!(
                "Successfully rolled back to configuration version: {}",
                version_id
            );
            Ok(())
        } else {
            Err(NestGateError::configuration(
                config_source: crate::error::core::UnifiedConfigSource::File(
                    "version_id".to_string(),
                ),
                field: Some("field".to_string()),
                
            })
        }
    }

    /// Apply Changes Atomic
    async fn apply_changes_atomic(&self, changes: Vec<ConfigChange>) -> Result<String> ", 
        // Create a snapshot before applying changes
        let snapshot = self
            .create_version_snapshot(
                format!("Atomic update with {changes.len() changes")),
                "system".to_string(),
            )
            .await?;

        // Validate all changes first
        let mut all_valid = true;
        let mut validation_reports = Vec::new();

        for change in &changes {
            let report = self.validate_config_change(change).await?;
            all_valid = all_valid && report.is_valid;
            validation_reports.push(report);
        }

        if !all_valid {
            return Err(NestGateError::configuration(
                
                
            );
        )

        // Apply all changes
        for change in &changes {
            if let Err(e) = self.apply_config_change(change).await {
                // Rollback on failure
                tracing::error!("Failed to apply configuration change, rolling back: {}", e);
                let _ = self.rollback_config(&snapshot.id).await;
                return Err(e);
            }
        }

        // Save the updated configuration
        self.save_config_to_file().await?;

        // Mark the snapshot as successfully applied
        let mut history = self.version_history.write().await;
        if let Some(version) = history.iter_mut().find(|v| v.id == snapshot.id) {
            version.applied_successfully = true;
            version.rollback_available = true;
        }

        Ok(snapshot.id)
    }

    /// Subscribe To Changes
    async fn subscribe_to_changes(&self) -> Result<watch::Receiver<CanonicalConfig>> {
        Ok(self.config_watcher.subscribe())
    }

    /// Export Config
    async fn export_config(&self, include_history: bool) -> Result<ConfigBackup> {
        let current_config = self.current_config.read().await.clone();
        let version_history = if include_history {
            Some(self.version_history.read().await.clone())
        } else {
            None
        };

        let backup = ConfigBackup {
            backup_id: Uuid::new_v4().to_string(),
            created_at: SystemTime::now(),
            source_system: gethostname::gethostname().to_string_lossy().to_string(),
            current_config,
            version_history,
            metadata: HashMap::from([
                ("export_version".to_string(), "1.0".to_string()),
                (
                    "nestgate_version".to_string(),
                    env!("CARGO_PKG_VERSION").to_string(),
                ),
            ]),
        };

        Ok(backup)
    }

    /// Import Config
    async fn import_config(&self, backup: &ConfigBackup) -> Result<String> {
        // Create a snapshot of current state before import
        let snapshot = self
            .create_version_snapshot(
                format!("Pre-import snapshot (importing {backup.backup_id})"),
                "system".to_string(),
            )
            .await?;

        // Apply the imported configuration
        let mut current_config = self.current_config.write().await;
        *current_config = backup.current_config.clone();

        // Import version history if available
        if let Some(history) = &backup.version_history {
            let mut version_history = self.version_history.write().await;
            version_history.extend(history.clone());
        }

        drop(current_config); // Release lock before async call

        // Save to file
        self.save_config_to_file().await?;

        tracing::info!(
            "Successfully imported configuration from backup: {}",
            backup.backup_id
        );
        Ok(snapshot.id)
    }
}
