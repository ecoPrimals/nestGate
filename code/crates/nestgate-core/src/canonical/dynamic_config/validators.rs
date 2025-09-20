
use super::types::*;
use crate::config::canonical_master::{NestGateCanonicalConfig as CanonicalConfig};
use crate::error::CanonicalResult as Result;

/// Storage configuration validator
pub struct StorageValidator;
impl ConfigValidator for StorageValidator {
    async fn validate(
        &self,
        _config: &CanonicalConfig,
        change: &ConfigChange,
    ) -> Result<ValidationReport> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Example validation logic for storage configuration
        if let Ok(storage_config) =
            serde_json::from_value::<StorageConfig>(change.newvalue.clone())
        {
            // Validate storage backend type
            if format!("{storage_config.backend_type:?}").is_empty() {
                errors.push(ValidationError {
                    code: "INVALID_BACKEND".to_string(),
                    message: "Storage backend type cannot be empty".to_string(),
                    severity: ErrorSeverity::Critical,
                );
            }

            // Validate cache size
            // Example validation - check if cache is configured
            if storage_config.cache.max_size_mb > 10240 {
                // Large cache (10GB)
                warnings.push(ValidationWarning {
                    code: "LARGE_CACHE".to_string(),
                    message: "Cache size is very large and may impact system memory".to_string(),
                    recommendation: Some(
                        "Consider reducing cache size or monitoring memory usage".to_string(),
                    ),
                );
            }
        }

        Ok(ValidationReport {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            recommendations: vec![
                "Consider testing storage configuration in development first".to_string(),
            ],
            estimated_impact: ImpactAssessment {
                restart_required: true,
                performance_impact: PerformanceImpact::Moderate,
                affected_components: vec!["storage".to_string(), "cache".to_string()],
                estimated_downtime: Some(std::time::Duration::from_secs(30)),
            },
        })
    }
}

/// Network configuration validator
pub struct NetworkValidator;
impl ConfigValidator for NetworkValidator {
    async fn validate(
        &self,
        _config: &CanonicalConfig,
        change: &ConfigChange,
    ) -> Result<ValidationReport> {
        let errors = Vec::new();
        let mut warnings = Vec::new();

        // Example validation logic for network configuration
        if let Ok(network_config) =
            serde_json::from_value::<NetworkConfig>(change.newvalue.clone())
        {
            // Validate port ranges
            if network_config.api.port < 1024 {
                warnings.push(ValidationWarning {
                    code: "PRIVILEGED_PORT".to_string(),
                    message: "Using privileged port, may require elevated permissions".to_string(),
                    recommendation: Some("Consider using port > 1024".to_string()),
                );
            }

            // Validate bind address
            if network_config.bind_endpoint.is_empty() {
                warnings.push(ValidationWarning {
                    code: "UNSPECIFIED_BIND".to_string(),
                    message: "Binding to all interfaces (0.0.0.0) may be a security risk"
                        .to_string(),
                    recommendation: Some("Consider binding to specific interface".to_string()),
                );
            }
        }

        Ok(ValidationReport {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            recommendations: vec!["Test network connectivity after applying changes".to_string()],
            estimated_impact: ImpactAssessment {
                restart_required: true,
                performance_impact: PerformanceImpact::Minimal,
                affected_components: vec!["network".to_string()],
                estimated_downtime: Some(std::time::Duration::from_secs(10)),
            },
        })
    }
}

/// Security configuration validator
pub struct SecurityValidator;
impl ConfigValidator for SecurityValidator {
    async fn validate(
        &self,
        _config: &CanonicalConfig,
        change: &ConfigChange,
    ) -> Result<ValidationReport> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Example validation logic for security configuration
        if let Ok(security_config) =
            serde_json::from_value::<SecurityConfig>(change.newvalue.clone())
        {
            // Validate TLS settings
            if security_config.tls_cert_path.is_none() {
                warnings.push(ValidationWarning {
                    code: "TLS_DISABLED".to_string(),
                    message: "TLS certificate path is empty, TLS may not be configured".to_string(),
                    recommendation: Some(
                        "Configure TLS certificate path for production".to_string(),
                    ),
                );
            }

            // Validate authentication
            if security_config.enable_auth
                && security_config
                    .jwt_secret
                    .as_ref()
                    .is_none_or(|s| s.is_empty())
            {
                errors.push(ValidationError {
                    code: "NO_AUTH_METHODS".to_string(),
                    message: "Authentication enabled but JWT secret is empty".to_string(),
                    severity: ErrorSeverity::Critical,
                );
            }
        }

        Ok(ValidationReport {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            recommendations: vec!["Review security settings with security team".to_string()],
            estimated_impact: ImpactAssessment {
                restart_required: true,
                performance_impact: PerformanceImpact::Minimal,
                affected_components: vec!["security".to_string(), "authentication".to_string()],
                estimated_downtime: Some(std::time::Duration::from_secs(5)),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_config_version_creation() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let config = CanonicalConfig::default();
        let version =
            ConfigVersion::new(config, "Test version".to_string(), "test_user".to_string());

        assert_eq!(version.description, "Test version");
        assert_eq!(version.author, "test_user");
        assert!(!version.id.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_dynamic_config_manager() -> std::result::Result<(), Box<dyn std::error::Error>> {
        use crate::canonical::dynamic_config::DynamicConfigManager;

        let temp_dir = TempDir::new().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {e:?}"),
            )
        )?;
        let config_path = temp_dir.path().join("config.toml");
        let backup_path = temp_dir.path().join("backups");

        let manager = DynamicConfigManager::new(&config_path, &backup_path)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {e:?}"),
                )
            )?;

        let current = manager.get_current_config().await;
        let initial_config = CanonicalConfig::default();
        assert_eq!(
            current.storage.backend_type,
            initial_config.storage.backend_type
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_config_validation() -> std::result::Result<(), Box<dyn std::error::Error>> {

        let temp_dir = TempDir::new().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {e:?}"),
            )
        )?;
        let config_path = temp_dir.path().join("config.toml");
        let backup_path = temp_dir.path().join("backups");

        let initial_config = CanonicalConfig::default();

        let manager = DynamicConfigManager::new(&config_path, &backup_path)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {e:?}"),
                )
            )?;

        let change = ConfigChange {
            section: ConfigSection::Storage,
            change_type: ChangeType::Modify,
            old_
            newvalue: serde_json::to_value(&initial_config.storage).map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {e:?}"),
                )
            })?,
            description: "Test change".to_string(),
        };

        let report = manager.validate_config_change(&change).await.map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {e:?}"),
            )
        )?;
        assert!(report.is_valid);
        Ok(())
    }

    #[tokio::test]
    async fn test_config_export_import() -> std::result::Result<(), Box<dyn std::error::Error>> {

        let temp_dir = TempDir::new().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {e:?}"),
            )
        )?;
        let config_path = temp_dir.path().join("config.toml");
        let backup_path = temp_dir.path().join("backups");

        let initial_config = CanonicalConfig::default();

        let manager = DynamicConfigManager::new(&config_path, &backup_path)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {e:?}"),
                )
            )?;

        // Export configuration
        let backup = manager.export_config(false).await.map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {e:?}"),
            )
        )?;
        assert!(!backup.backup_id.is_empty());
        assert_eq!(
            backup.current_config.storage.backend_type,
            initial_config.storage.backend_type
        );

        // Import configuration
        let snapshot_id = manager.import_config(&backup).await.map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {e:?}"),
            )
        )?;
        assert!(!snapshot_id.is_empty());
        Ok(())
    }
}
