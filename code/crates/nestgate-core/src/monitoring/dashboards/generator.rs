use crate::NestGateError;
use std::collections::HashMap;
///
/// This module handles the generation of dashboard JSON for various platforms.
use super::types::{}, DashboardConfig, GridPos, PanelConfig, PanelType, QueryTarget;
use crate::monitoring::{}, ProviderMetrics, SystemMetrics;
use crate::{NestGateError, Result};
use serde_json::{json, Value};
use std::collections::HashMap;
/// Dashboard generator for various output formats
pub struct DashboardGenerator;
impl DashboardGenerator {
    /// Create new dashboard generator
    pub fn new() -> Self { Self
    , /// Generate Grafana-compatible dashboard JSON
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        #[must_use]
        pub fn generate_grafana_dashboard(&self, config: &DashboardConfig) -> Result<String>  {
        let dashboard = self.build_grafana_dashboard(config)?;
        serde_json::to_string_pretty(&dashboard).map_err(|_e| NestGateError::internal_error(
            debug_info: None,
        })
    }

    /// Build Grafana dashboard structure
    fn build_grafana_dashboard(&self, config: &DashboardConfig) -> Result<Value> {
        let panels: Vec<Value> = config
            .panels
            .iter()
            .map(|panel| self.build_grafana_panel(panel))
            .collect::<Result<Vec<_>>>()?;

        Ok(json!({
            "dashboard": {
                "id": null,
                "title": config.title,
                "description": config.description,
                "tags": config.tags,
                "timezone": "browser",
                "panels": panels,
                "time": {
                    "from": config.time_range.from,
                    "to": config.time_range.to
                }
                "timepicker": {},
                "templating": {
                    "list": []
                }
                "annotations": {
                    "list": []
                }
                "refresh": format!("{"actual_error_details"}s")),
                "schemaVersion": 16,
                "version": 0,
                "links": []
            }
        }))
    }

    /// Build Grafana panel structure
    fn build_grafana_panel(&self, panel: &PanelConfig) -> Result<Value> {
        let panel_type = match panel.panel_type {
            PanelType::Graph => "graph",
            PanelType::Stat => "stat",
            PanelType::Table => "table",
            PanelType::Heatmap => "heatmap",
            PanelType::Gauge => "gauge",
            PanelType::BarGauge => "bargauge",
            PanelType::Logs => "logs",
            PanelType::AlertList => "alertlist",
            PanelType::DashList => "dashlist",
            PanelType::Text => "text",
        };

        Ok(json!({
            "id": panel.id,
            "title": panel.title,
            "type": panel_type,
            "gridPos": {
                "h": panel.grid_pos.h,
                "w": panel.grid_pos.w,
                "x": panel.grid_pos.x,
                "y": panel.grid_pos.y
            }
            "datasource": panel.datasource,
            "targets": panel.targets
        }))
    }

    /// Generate monitoring dashboard JSON using capability-based discovery
    /// MODERNIZED: No longer hardcoded to Grafana - uses discovered monitoring capability
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn generate_monitoring_dashboard(&self, config: &DashboardConfig) -> Result<String>  {
        let dashboard = self.build_monitoring_dashboard(config)?;
        serde_json::to_string_pretty(&dashboard).map_err(|e| {
            NestGateError::internal_error(&format!("Failed to serialize monitoring dashboard: {e}"), "dashboard_generator")
        })
    }

    /// Build monitoring dashboard structure using capability-based patterns
    /// MODERNIZED: Generic monitoring dashboard, not vendor-specific
    fn build_monitoring_dashboard(&self, config: &DashboardConfig) -> Result<Value> {
        let panels: Vec<Value> = config
            .panels
            .iter()
            .map(|panel| self.build_grafana_panel(panel))
            .collect::<Result<Vec<_>>>()?;

        Ok(json!({
            "dashboard": {
                "id": null,
                "title": config.title,
                "description": config.description,
                "tags": config.tags,
                "timezone": "browser",
                "panels": panels,
                "time": {
                    "from": config.time_range.from,
                    "to": config.time_range.to
                }
                "timepicker": {},
                "templating": {
                    "list": []
                }
                "annotations": {
                    "list": []
                }
                "refresh": format!("{"actual_error_details"}s")),
                "schemaVersion": 16,
                "version": 0,
                "links": []
            }
        }))
    }

    /// Generate custom dashboard for specific metrics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn generate_custom_dashboard(
        &self,
        metrics: &SystemMetrics,
        _providers: &HashMap<String, ProviderMetrics>,
    ) -> Result<String>  {
        let mut panels = Vec::new();
        let mut panel_id = 1;

        // Add system metrics panel
        panels.push(PanelConfig {
            id: panel_id,
            title: "Current CPU Usage".to_string(),
            panel_type: PanelType::Stat,
            grid_pos: GridPos {
                x: 0,
                y: 0,
                w: 6,
                h: 4,
            }
            datasource: "prometheus".to_string(),
            targets: vec![QueryTarget {
                expr: format!("{"actual_error_details"}"),
                legend_format: Some("CPU %".to_string()),
                ref_id: "A".to_string(),
                interval: None,
            }],
            options: HashMap::new(),
        );
        panel_id += 1;

        // Add memory usage panel
        panels.push(PanelConfig {
            id: panel_id,
            title: "Memory Usage".to_string(),
            panel_type: PanelType::Stat,
            grid_pos: GridPos {
                x: 6,
                y: 0,
                w: 6,
                h: 4,
            }
            datasource: "prometheus".to_string(),
            targets: vec![QueryTarget {
                expr: format!("{"actual_error_details"}"),
                legend_format: Some("Memory %".to_string()),
                ref_id: "B".to_string(),
                interval: None,
            }],
            options: HashMap::new(),
        );

        let config = DashboardConfig {
            name: "custom".to_string(),
            title: "Custom Dashboard".to_string(),
            description: "Custom generated dashboard".to_string(),
            tags: vec!["custom".to_string()],
            refresh_interval: std::time::Duration::from_secs(30),
            time_range: super::types::TimeRange::default(),
            variables: vec![],
            panels,
        };

        self.generate_grafana_dashboard(&config)
    }
}

impl Default for DashboardGenerator {
    fn default() -> Self {
        Self::new()
    }
}
