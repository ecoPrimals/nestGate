//! **DASHBOARD MANAGER AND TEMPLATES**
//!
//! Dashboard manager implementation with template creation and management.
//! Extracted from dashboards.rs for file size compliance.

use crate::monitoring::{ProviderMetrics, SystemMetrics};
use crate::{NestGateError, Result};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, info};

use super::types::{
    DashboardConfig, PanelConfig, PanelType, GridPos, QueryTarget, TimeRange,
    VariableConfig, VariableType,
};

/// Dashboard manager
pub struct DashboardManager {
    /// Predefined dashboard templates
    templates: HashMap<String, DashboardConfig>,
}

impl DashboardManager {
    /// Create new dashboard manager
    pub fn new() -> Self {
        let mut manager = Self {
            templates: HashMap::new(),
        };

        // Initialize default templates
        manager.init_default_templates();

        info!(
            "📊 Dashboard manager initialized with {} templates",
            manager.templates.len()
        );
        manager
    }

    /// Initialize default dashboard templates
    fn init_default_templates(&mut self) {
        // System Overview Dashboard
        self.templates.insert(
            "system-overview".to_string(),
            self.create_system_overview_template(),
        );

        // Provider Performance Dashboard
        self.templates.insert(
            "provider-performance".to_string(),
            self.create_provider_performance_template(),
        );

        // Storage Metrics Dashboard
        self.templates.insert(
            "storage-metrics".to_string(),
            self.create_storage_metrics_template(),
        );

        // Alert Dashboard
        self.templates
            .insert("alerts".to_string(), self.create_alerts_template());
    }

    /// Create system overview dashboard template
    fn create_system_overview_template(&self) -> DashboardConfig {
        DashboardConfig {
            name: "system-overview".to_string(),
            title: "NestGate System Overview".to_string(),
            description: "High-level system metrics and health status".to_string(),
            tags: vec!["nestgate".to_string(), "system".to_string()],
            refresh_interval: Duration::from_secs(30),
            time_range: TimeRange {
                from: "now-1h".to_string(),
                to: "now".to_string(),
            },
            variables: vec![
                VariableConfig {
                    name: "instance".to_string(),
                    var_type: VariableType::Query,
                    query: "label_values(up, instance)".to_string(),
                    current: None,
                    options: vec![],
                }
            ],
            panels: vec![
                // CPU Usage
                PanelConfig {
                    id: 1,
                    title: "CPU Usage".to_string(),
                    panel_type: PanelType::Graph,
                    grid_pos: GridPos { x: 0, y: 0, w: 12, h: 8 },
                    datasource: "prometheus".to_string(),
                    targets: vec![
                        QueryTarget {
                            expr: "100 - (avg(irate(node_cpu_seconds_total{mode=\"idle\",instance=\"$instance\"}[5m])) * 100)".to_string(),
                            legend_format: Some("CPU Usage %".to_string()),
                            ref_id: "A".to_string(),
                            interval: None,
                        }
                    ],
                    options: HashMap::new(),
                },
                // Memory Usage
                PanelConfig {
                    id: 2,
                    title: "Memory Usage".to_string(),
                    panel_type: PanelType::Graph,
                    grid_pos: GridPos { x: 12, y: 0, w: 12, h: 8 },
                    datasource: "prometheus".to_string(),
                    targets: vec![
                        QueryTarget {
                            expr: "(1 - (node_memory_MemAvailable_bytes{instance=\"$instance\"} / node_memory_MemTotal_bytes{instance=\"$instance\"})) * 100".to_string(),
                            legend_format: Some("Memory Usage %".to_string()),
                            ref_id: "A".to_string(),
                            interval: None,
                        }
                    ],
                    options: HashMap::new(),
                },
                // Active Connections
                PanelConfig {
                    id: 3,
                    title: "Active Connections".to_string(),
                    panel_type: PanelType::Stat,
                    grid_pos: GridPos { x: 0, y: 8, w: 8, h: 4 },
                    datasource: "prometheus".to_string(),
                    targets: vec![
                        QueryTarget {
                            expr: "nestgate_active_connections{instance=\"$instance\"}".to_string(),
                            legend_format: None,
                            ref_id: "A".to_string(),
                            interval: None,
                        }
                    ],
                    options: HashMap::new(),
                },
            ],
        }
    }

    /// Create provider performance dashboard template
    fn create_provider_performance_template(&self) -> DashboardConfig {
        DashboardConfig {
            name: "provider-performance".to_string(),
            title: "Provider Performance Metrics".to_string(),
            description: "Performance metrics for all service providers".to_string(),
            tags: vec!["nestgate".to_string(), "providers".to_string()],
            refresh_interval: Duration::from_secs(15),
            time_range: TimeRange {
                from: "now-30m".to_string(),
                to: "now".to_string(),
            },
            variables: vec![
                VariableConfig {
                    name: "provider".to_string(),
                    var_type: VariableType::Query,
                    query: "label_values(nestgate_provider_requests_total, provider_name)".to_string(),
                    current: None,
                    options: vec![],
                }
            ],
            panels: vec![
                // Request Rate
                PanelConfig {
                    id: 1,
                    title: "Request Rate".to_string(),
                    panel_type: PanelType::Graph,
                    grid_pos: GridPos { x: 0, y: 0, w: 12, h: 8 },
                    datasource: "prometheus".to_string(),
                    targets: vec![
                        QueryTarget {
                            expr: "rate(nestgate_provider_requests_total{provider_name=\"$provider\"}[5m])".to_string(),
                            legend_format: Some("{{status}} req/sec".to_string()),
                            ref_id: "A".to_string(),
                            interval: None,
                        }
                    ],
                    options: HashMap::new(),
                },
                // Response Time
                PanelConfig {
                    id: 2,
                    title: "Response Time".to_string(),
                    panel_type: PanelType::Graph,
                    grid_pos: GridPos { x: 12, y: 0, w: 12, h: 8 },
                    datasource: "prometheus".to_string(),
                    targets: vec![
                        QueryTarget {
                            expr: "histogram_quantile(0.95, rate(nestgate_provider_response_time_seconds_bucket{provider_name=\"$provider\"}[5m]))".to_string(),
                            legend_format: Some("95th percentile".to_string()),
                            ref_id: "A".to_string(),
                            interval: None,
                        }
                    ],
                    options: HashMap::new(),
                },
            ],
        }
    }

    /// Create storage metrics dashboard template
    fn create_storage_metrics_template(&self) -> DashboardConfig {
        DashboardConfig {
            name: "storage-metrics".to_string(),
            title: "Storage Performance".to_string(),
            description: "Storage backend performance and utilization metrics".to_string(),
            tags: vec!["nestgate".to_string(), "storage".to_string()],
            refresh_interval: Duration::from_secs(30),
            time_range: TimeRange {
                from: "now-1h".to_string(),
                to: "now".to_string(),
            },
            variables: vec![
                VariableConfig {
                    name: "backend".to_string(),
                    var_type: VariableType::Query,
                    query: "label_values(nestgate_storage_operations_total, backend_name)".to_string(),
                    current: None,
                    options: vec![],
                }
            ],
            panels: vec![
                // Storage Operations
                PanelConfig {
                    id: 1,
                    title: "Storage Operations".to_string(),
                    panel_type: PanelType::Graph,
                    grid_pos: GridPos { x: 0, y: 0, w: 12, h: 8 },
                    datasource: "prometheus".to_string(),
                    targets: vec![
                        QueryTarget {
                            expr: "rate(nestgate_storage_operations_total{backend_name=\"$backend\"}[5m])".to_string(),
                            legend_format: Some("{{operation}} ops/sec".to_string()),
                            ref_id: "A".to_string(),
                            interval: None,
                        }
                    ],
                    options: HashMap::new(),
                },
                // Data Transfer Rate
                PanelConfig {
                    id: 2,
                    title: "Data Transfer Rate".to_string(),
                    panel_type: PanelType::Graph,
                    grid_pos: GridPos { x: 12, y: 0, w: 12, h: 8 },
                    datasource: "prometheus".to_string(),
                    targets: vec![
                        QueryTarget {
                            expr: "rate(nestgate_storage_bytes_transferred{backend_name=\"$backend\"}[5m])".to_string(),
                            legend_format: Some("{{direction}} bytes/sec".to_string()),
                            ref_id: "A".to_string(),
                            interval: None,
                        }
                    ],
                    options: HashMap::new(),
                },
            ],
        }
    }

    /// Create alerts dashboard template
    fn create_alerts_template(&self) -> DashboardConfig {
        DashboardConfig {
            name: "alerts".to_string(),
            title: "System Alerts".to_string(),
            description: "Current system alerts and notifications".to_string(),
            tags: vec!["nestgate".to_string(), "alerts".to_string()],
            refresh_interval: Duration::from_secs(10),
            time_range: TimeRange {
                from: "now-6h".to_string(),
                to: "now".to_string(),
            },
            variables: vec![],
            panels: vec![
                // Active Alerts
                PanelConfig {
                    id: 1,
                    title: "Active Alerts".to_string(),
                    panel_type: PanelType::AlertList,
                    grid_pos: GridPos { x: 0, y: 0, w: 24, h: 12 },
                    datasource: "prometheus".to_string(),
                    targets: vec![],
                    options: HashMap::new(),
                },
            ],
        }
    }

    /// Get dashboard template by name
    pub fn get_template(&self, name: &str) -> Result<&DashboardConfig> {
        self.templates.get(name).ok_or_else(|| NestGateError::Internal {
            message: format!("Dashboard template '{}' not found", name),
            location: Some(file!().to_string()),
            context: None,
            is_bug: false,
        })
    }

    /// List available templates
    pub fn list_templates(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }

    /// Build Grafana dashboard JSON
    pub fn build_grafana_dashboard(&self, config: &DashboardConfig) -> Result<Value> {
        let mut panels = Vec::new();

        for panel in &config.panels {
            let mut targets = Vec::new();
            for target in &panel.targets {
                targets.push(json!({
                    "expr": target.expr,
                    "legendFormat": target.legend_format,
                    "refId": target.ref_id,
                    "interval": target.interval
                }));
            }

            panels.push(json!({
                "id": panel.id,
                "title": panel.title,
                "type": format!("{:?}", panel.panel_type).to_lowercase(),
                "gridPos": {
                    "x": panel.grid_pos.x,
                    "y": panel.grid_pos.y,
                    "w": panel.grid_pos.w,
                    "h": panel.grid_pos.h
                },
                "datasource": panel.datasource,
                "targets": targets,
                "options": panel.options
            }));
        }

        let mut templating = Vec::new();
        for var in &config.variables {
            templating.push(json!({
                "name": var.name,
                "type": format!("{:?}", var.var_type).to_lowercase(),
                "query": var.query,
                "current": var.current,
                "options": var.options
            }));
        }

        Ok(json!({
            "dashboard": {
                "id": null,
                "title": config.title,
                "description": config.description,
                "tags": config.tags,
                "refresh": format!("{}s", config.refresh_interval.as_secs()),
                "time": {
                    "from": config.time_range.from,
                    "to": config.time_range.to
                },
                "templating": {
                    "list": templating
                },
                "panels": panels
            }
        }))
    }

    /// Generate Grafana dashboard from template
    pub fn generate_from_template(&self, template_name: &str) -> Result<String> {
        let config = self.get_template(template_name)?;
        let dashboard = self.build_grafana_dashboard(config)?;
        
        serde_json::to_string_pretty(&dashboard).map_err(|e| NestGateError::Internal {
            message: format!("Failed to serialize dashboard: {e}"),
            location: Some(file!().to_string()),
            context: None,
            is_bug: false,
        })
    }

    /// Generate custom dashboard for specific metrics
    pub fn generate_custom_dashboard(
        &self,
        system_metrics: &SystemMetrics,
        provider_metrics: &HashMap<String, ProviderMetrics>,
    ) -> Result<String> {
        let mut panels = Vec::new();
        let mut panel_id = 1u32;

        // System metrics panel
        panels.push(PanelConfig {
            id: panel_id,
            title: "System Health".to_string(),
            panel_type: PanelType::Stat,
            grid_pos: GridPos { x: 0, y: 0, w: 8, h: 4 },
            datasource: "prometheus".to_string(),
            targets: vec![QueryTarget {
                expr: format!("vector({:.1})", system_metrics.cpu_usage),
                legend_format: Some("CPU Usage %".to_string()),
                ref_id: "A".to_string(),
                interval: None,
            }],
            options: HashMap::new(),
        });
        panel_id += 1;

        // Provider metrics panels
        for (provider_name, _metrics) in provider_metrics {
            panels.push(PanelConfig {
                id: panel_id,
                title: format!("Provider: {}", provider_name),
                panel_type: PanelType::Graph,
                grid_pos: GridPos { x: (panel_id % 3) * 8, y: (panel_id / 3) * 8, w: 8, h: 6 },
                datasource: "prometheus".to_string(),
                targets: vec![QueryTarget {
                    expr: format!("nestgate_provider_requests_total{{provider_name=\"{}\"}}", provider_name),
                    legend_format: Some("Requests".to_string()),
                    ref_id: "A".to_string(),
                    interval: None,
                }],
                options: HashMap::new(),
            });
            panel_id += 1;
        }

        let config = DashboardConfig {
            name: "custom-dashboard".to_string(),
            title: "Custom NestGate Dashboard".to_string(),
            description: "Auto-generated dashboard based on current metrics".to_string(),
            tags: vec!["nestgate".to_string(), "custom".to_string()],
            refresh_interval: Duration::from_secs(30),
            time_range: TimeRange {
                from: "now-1h".to_string(),
                to: "now".to_string(),
            },
            variables: vec![],
            panels,
        };

        let dashboard = self.build_grafana_dashboard(&config)?;
        let json =
            serde_json::to_string_pretty(&dashboard).map_err(|e| NestGateError::Internal {
                message: format!("Failed to serialize custom dashboard: {e}"),
                location: Some(file!().to_string()),
                context: None,
                is_bug: false,
            })?;

        Ok(json)
    }
}

impl Default for DashboardManager {
    fn default() -> Self {
        Self::new()
    }
} 