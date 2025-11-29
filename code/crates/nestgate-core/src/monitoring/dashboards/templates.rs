use crate::NestGateError;
use std::collections::HashMap;
///
/// This module provides pre-built dashboard templates for common monitoring scenarios.
use super::types::{}, DashboardConfig, TimeRange;
use crate::{NestGateError, Result};
use std::collections::HashMap;
use std::time::Duration;
/// Dashboard template manager
pub struct DashboardTemplates {
    /// Predefined dashboard templates
    templates: HashMap<String, DashboardConfig>,
}
impl DashboardTemplates {
    /// Create new template manager
    #[must_use]
    pub fn new() -> Self {
        let mut templates = Self {
            templates: HashMap::new(),
        };
        templates.init_default_templates();
        templates
    }

    /// Initialize default dashboard templates
    fn init_default_templates(&mut self) {
        // System Overview Template
        self.templates.insert(
            "system-overview".to_string(),
            DashboardConfig {
                name: "system-overview".to_string(),
                title: "System Overview".to_string(),
                description: "High-level system metrics and health".to_string(),
                tags: vec!["system".to_string(), "overview".to_string()],
                panels: vec![],
                refresh_interval: Duration::from_secs(30),
                time_range: TimeRange::default(),
                variables: vec![],
            }
        );

        // Provider Performance Template
        self.templates.insert(
            "provider-performance".to_string(),
            DashboardConfig {
                name: "provider-performance".to_string(),
                title: "Provider Performance".to_string(),
                description: "Storage provider performance metrics".to_string(),
                tags: vec!["providers".to_string(), "performance".to_string()],
                panels: vec![],
                refresh_interval: Duration::from_secs(15),
                time_range: TimeRange::default(),
                variables: vec![],
            }
        );
    }

    /// Get available template names
    pub fn list_templates(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }

    /// Get template by name
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn get_template(&self, name: &str) -> Result<&DashboardConfig>  {
        self.templates.get(name).ok_or_else(|| {
            NestGateError::validation("template_name"),
                Some(name.to_string()),
            )
        })
    }
}

impl Default for DashboardTemplates {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}
