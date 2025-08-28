//! **BIOMEOS DISCOVERY AND TEMPLATE RETRIEVAL**
//!
//! Capability discovery and template retrieval functionality for BiomeOS integration.
//! Extracted from biomeos.rs for file size compliance.

use crate::Result;
use super::types::{BiomeManifest, TemplateSpec, VolumeSpec};

impl BiomeManifest {
    /// Parse biome.yaml from string
    pub fn from_yaml(yaml_content: &str) -> Result<Self> {
        serde_yaml::from_str(yaml_content).map_err(|e| crate::NestGateError::Internal {
            message: format!("Failed to parse biome.yaml: {e}"),
            component: "biomeos_discovery".to_string(),
            location: Some(format!("{}:{}", file!(), line!())),
            is_bug: false,
            context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                operation: "parse_biome_yaml".to_string(),
                component: "biomeos".to_string(),
                metadata: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("yaml_parsing_error".to_string(), format!("{:?}", e));
                    map.insert("content_length".to_string(), yaml_content.len().to_string());
                    map
                },
                timestamp: std::time::SystemTime::now(),
                retry_info: None,
                recovery_suggestions: vec!["Check YAML syntax and format".to_string()],
                    performance_metrics: None,
                    environment: None,
            }),
        })
    }

    /// Parse biome.yaml from file
    pub async fn from_file(file_path: &str) -> Result<Self> {
        let content = tokio::fs::read_to_string(file_path).await.map_err(|e| {
            crate::NestGateError::Internal {
                message: format!("Failed to read biome.yaml: {e}"),
                component: "biomeos_discovery".to_string(),
                location: Some(file!().to_string()),
                context: None,
                is_bug: false,
            }
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

    /// Get capability-based storage templates (replaces primal-specific templates)
    pub async fn get_templates_by_capability(&self, capability_type: &str) -> Vec<TemplateSpec> {
        if let Some(_templates) = &self.templates {
            match capability_type {
                // AI and Intelligence Capabilities
                "ai-runtime" => {
                    eprintln!("INFO: Using capability-based AI runtime discovery");
                    self.templates
                        .as_ref()
                        .and_then(|t| t.ai_runtime.as_ref())
                        .cloned()
                        .unwrap_or_else(|| {
                            vec![TemplateSpec {
                                name: "ai-runtime-template".to_string(),
                                resources: "cpu:2,memory:4Gi".to_string(),
                                config: std::collections::HashMap::new(),
                            }]
                        })
                }
                "agent-processing" => {
                    eprintln!("INFO: Using capability-based agent processing discovery");
                    self.templates
                        .as_ref()
                        .and_then(|t| t.agent_processing.as_ref())
                        .cloned()
                        .unwrap_or_else(|| {
                            vec![TemplateSpec {
                                name: "agent-processing-template".to_string(),
                                resources: "cpu:1,memory:2Gi".to_string(),
                                config: std::collections::HashMap::new(),
                            }]
                        })
                }
                // Route through universal adapter for capability discovery
                capability => {
                    // Use universal adapter to discover and route capability requests
                    super::adapters::route_capability_through_adapter(capability)
                        .await
                        .unwrap_or_default()
                }
            }
        } else {
            vec![]
        }
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