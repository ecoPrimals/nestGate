// Management manifest discovery and parsing
// Handles biome.yaml manifest files for ecosystem integration

use crate::error::{Result, NestGateError};
// CLEANED: Removed unused serde imports as part of canonical modernization
// use serde::{Deserialize, Serialize};
use super::types::{BiomeManifest, TemplateSpec, VolumeSpec};

impl BiomeManifest {
    /// Parse Management manifest from YAML content
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn from_yaml(yaml_content: &str) -> Result<Self>  {
        serde_yaml_ng::from_str(yaml_content).map_err(|e| NestGateError::internal_error(&format!("Failed to parse biome.yaml: {e}"), "component"management_discovery"))
    }

    /// Load Management manifest from file
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn from_file(file_path: &str) -> Result<Self>  {
        let content = tokio::fs::read_to_string(file_path).await.map_err(|e| {
            NestGateError::Internal(Box::new(crate::error::variants::core_errors::InternalErrorDetails {
                message: format!("Failed to read biome.yaml: {e}"),
                component: "management_discovery".to_string(),
                location: Some(format!("{file!(}:{file!(}"), line!())),
                is_bug: false,
                context: None,
            }))
        })?;

        Self::from_yaml(&content)
    }

    /// Get storage volumes for NestGate provisioning
    pub fn get_nestgate_volumes(&self) -> Vec<&VolumeSpec> {
        self.storage
            .volumes
            .iter()
            .collect()
    }

    /// Get network configuration from manifest
    pub fn get_network_config(&self) -> Option<&super::types::BiomeNetworking> {
        Some(&self.networking)
    }

    /// Get service definitions from manifest
    pub fn get_services(&self) -> &std::collections::HashMap<String, super::types::ServiceConfig> {
        &self.services
    }

    /// Check if manifest is valid for NestGate integration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn validate_for_nestgate(&self) -> Result<()>  {
        if self.metadata.name.is_empty() {
            return Err(NestGateError::validation_error("Biome name cannot be empty"));
        }

        if self.metadata.version.is_empty() {
            return Err(NestGateError::validation_error("Biome version cannot be empty"));
        }

        Ok(())
    }

    /// Get templates filtered by capability
    pub fn get_templates_by_capability(&self, capability: &str) -> Vec<&TemplateSpec> {
        let mut result = Vec::new();
        if let Some(templates) = &self.templates {
            // Check AI runtime templates
            if let Some(ai_templates) = &templates.ai_runtime {
                result.extend(ai_templates.iter().filter(|template| {
                    template.config
                        .get("capabilities")
                        .and_then(|caps| caps.as_array())
                        .map(|caps| caps.iter().any(|cap| cap.as_str() == Some(capability)))
                        .unwrap_or(false)
                }));
            }
            // Check other template types similarly if needed
        }
        result
    }

    /// Get all available capabilities from templates
    pub fn get_available_capabilities(&self) -> Vec<String> {
        let mut capabilities = Vec::new();
        if let Some(templates) = &self.templates {
            if let Some(ai_templates) = &templates.ai_runtime {
                for template in ai_templates {
                    if let Some(caps) = template.config
                        .get("capabilities")
                        .and_then(|c| c.as_array())
                    {
                        for cap in caps {
                            if let Some(cap_str) = cap.as_str() {
                                if !capabilities.contains(&cap_str.to_string()) {
                                    capabilities.push(cap_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        capabilities
    }

    /// Get capability-based templates through universal adapter routing
    pub async fn get_templates_by_universal_adapter(&self, capability: &str) -> Vec<TemplateSpec> {
        // Route all template requests through universal adapter for sovereignty compliance
        match super::adapters::route_capability_through_adapter(capability).await {
            Ok(templates) => templates,
            Err(e) => {
                eprintln!("Universal adapter routing failed for capability '{capability}': {e}");
                vec![]
            }
        }
    }
} 