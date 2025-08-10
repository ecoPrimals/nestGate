//! Dashboard Generation and Management
//!
//! Comprehensive dashboard system for generating monitoring dashboards for
//! Grafana, Prometheus, and custom web interfaces.

use crate::monitoring::{ProviderMetrics, SystemMetrics};
use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tracing::{debug, info};

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// Dashboard name
    pub name: String,
    /// Dashboard title
    pub title: String,
    /// Dashboard description
    pub description: String,
    /// Dashboard tags
    pub tags: Vec<String>,
    /// Dashboard panels
    pub panels: Vec<PanelConfig>,
    /// Refresh interval
    pub refresh_interval: Duration,
    /// Time range
    pub time_range: TimeRange,
    /// Dashboard variables
    pub variables: Vec<VariableConfig>,
}

/// Panel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelConfig {
    /// Panel ID
    pub id: u32,
    /// Panel title
    pub title: String,
    /// Panel type (graph, stat, table, etc.)
    pub panel_type: PanelType,
    /// Panel position and size
    pub grid_pos: GridPos,
    /// Data source
    pub datasource: String,
    /// Panel targets (queries)
    pub targets: Vec<QueryTarget>,
    /// Panel options
    pub options: HashMap<String, Value>,
}

/// Panel types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PanelType {
    Graph,
    Stat,
    Table,
    Heatmap,
    Gauge,
    BarGauge,
    Logs,
    AlertList,
    DashList,
    Text,
}

/// Panel grid position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridPos {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Query target for panels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryTarget {
    /// Query expression
    pub expr: String,
    /// Legend format
    pub legend_format: Option<String>,
    /// Ref ID
    pub ref_id: String,
    /// Interval
    pub interval: Option<String>,
}

/// Time range configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    /// Start time (relative, e.g., "now-1h")
    pub from: String,
    /// End time (relative, e.g., "now")
    pub to: String,
}

/// Dashboard variable configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableConfig {
    /// Variable name
    pub name: String,
    /// Variable type
    pub var_type: VariableType,
    /// Variable query
    pub query: String,
    /// Current value
    pub current: Option<String>,
    /// Available options
    pub options: Vec<String>,
}

/// Variable types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VariableType {
    Query,
    Custom,
    Constant,
    Interval,
    Datasource,
}

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
                    query: "label_values(nestgate_system_cpu_usage, instance)".to_string(),
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
                            expr: "nestgate_system_cpu_usage{instance=\"$instance\"}".to_string(),
                            legend_format: Some("CPU %".to_string()),
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
                            expr: "nestgate_system_memory_usage{instance=\"$instance\"} / (nestgate_system_memory_usage{instance=\"$instance\"} + nestgate_system_memory_available{instance=\"$instance\"}) * 100".to_string(),
                            legend_format: Some("Memory %".to_string()),
                            ref_id: "A".to_string(),
                            interval: None,
                        }
                    ],
                    options: HashMap::new(),
                },
                // Disk Usage
                PanelConfig {
                    id: 3,
                    title: "Disk Usage".to_string(),
                    panel_type: PanelType::Gauge,
                    grid_pos: GridPos { x: 0, y: 8, w: 6, h: 4 },
                    datasource: "prometheus".to_string(),
                    targets: vec![
                        QueryTarget {
                            expr: "nestgate_system_disk_usage{instance=\"$instance\"} / (nestgate_system_disk_usage{instance=\"$instance\"} + nestgate_system_disk_available{instance=\"$instance\"}) * 100".to_string(),
                            legend_format: Some("Disk %".to_string()),
                            ref_id: "A".to_string(),
                            interval: None,
                        }
                    ],
                    options: HashMap::new(),
                },
                // Active Connections
                PanelConfig {
                    id: 4,
                    title: "Active Connections".to_string(),
                    panel_type: PanelType::Stat,
                    grid_pos: GridPos { x: 6, y: 8, w: 6, h: 4 },
                    datasource: "prometheus".to_string(),
                    targets: vec![
                        QueryTarget {
                            expr: "nestgate_system_active_connections{instance=\"$instance\"}".to_string(),
                            legend_format: Some("Connections".to_string()),
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
            description: "Performance metrics for all registered providers".to_string(),
            tags: vec!["nestgate".to_string(), "providers".to_string()],
            refresh_interval: Duration::from_secs(30),
            time_range: TimeRange {
                from: "now-6h".to_string(),
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
                            legend_format: Some("{{provider_name}} requests/sec".to_string()),
                            ref_id: "A".to_string(),
                            interval: None,
                        }
                    ],
                    options: HashMap::new(),
                },
                // Success Rate
                PanelConfig {
                    id: 2,
                    title: "Success Rate".to_string(),
                    panel_type: PanelType::Graph,
                    grid_pos: GridPos { x: 12, y: 0, w: 12, h: 8 },
                    datasource: "prometheus".to_string(),
                    targets: vec![
                        QueryTarget {
                            expr: "nestgate_provider_successful_requests{provider_name=\"$provider\"} / nestgate_provider_requests_total{provider_name=\"$provider\"} * 100".to_string(),
                            legend_format: Some("{{provider_name}} success %".to_string()),
                            ref_id: "A".to_string(),
                            interval: None,
                        }
                    ],
                    options: HashMap::new(),
                },
                // Response Time
                PanelConfig {
                    id: 3,
                    title: "Average Response Time".to_string(),
                    panel_type: PanelType::Graph,
                    grid_pos: GridPos { x: 0, y: 8, w: 24, h: 8 },
                    datasource: "prometheus".to_string(),
                    targets: vec![
                        QueryTarget {
                            expr: "nestgate_provider_avg_response_time_ms{provider_name=\"$provider\"}".to_string(),
                            legend_format: Some("{{provider_name}} ms".to_string()),
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
            title: "Storage Backend Metrics".to_string(),
            description: "Performance and utilization metrics for storage backends".to_string(),
            tags: vec!["nestgate".to_string(), "storage".to_string()],
            refresh_interval: Duration::from_secs(60),
            time_range: TimeRange {
                from: "now-24h".to_string(),
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
            title: "NestGate Alerts".to_string(),
            description: "Active alerts and alert history".to_string(),
            tags: vec!["nestgate".to_string(), "alerts".to_string()],
            refresh_interval: Duration::from_secs(10),
            time_range: TimeRange {
                from: "now-24h".to_string(),
                to: "now".to_string(),
            },
            variables: vec![],
            panels: vec![
                // Active Alerts
                PanelConfig {
                    id: 1,
                    title: "Active Alerts".to_string(),
                    panel_type: PanelType::AlertList,
                    grid_pos: GridPos {
                        x: 0,
                        y: 0,
                        w: 24,
                        h: 12,
                    },
                    datasource: "alertmanager".to_string(),
                    targets: vec![],
                    options: HashMap::new(),
                },
                // Alert History
                PanelConfig {
                    id: 2,
                    title: "Alert History".to_string(),
                    panel_type: PanelType::Table,
                    grid_pos: GridPos {
                        x: 0,
                        y: 12,
                        w: 24,
                        h: 8,
                    },
                    datasource: "prometheus".to_string(),
                    targets: vec![QueryTarget {
                        expr: "increase(nestgate_alerts_total[1h])".to_string(),
                        legend_format: Some("{{alertname}} - {{severity}}".to_string()),
                        ref_id: "A".to_string(),
                        interval: None,
                    }],
                    options: HashMap::new(),
                },
            ],
        }
    }

    /// Generate Grafana dashboard JSON
    pub fn generate_grafana_dashboard(&self, template_name: &str) -> Result<String> {
        let template =
            self.templates
                .get(template_name)
                .ok_or_else(|| NestGateError::Internal {
                    message: format!("Dashboard template not found: {template_name}"),
                    location: Some(file!().to_string()),
                    debug_info: None,
                    is_bug: false,
                })?;

        let dashboard = self.build_grafana_dashboard(template)?;
        let json =
            serde_json::to_string_pretty(&dashboard).map_err(|e| NestGateError::Internal {
                message: format!("Failed to serialize dashboard: {e}"),
                location: Some(file!().to_string()),
                debug_info: None,
                is_bug: false,
            })?;

        debug!("Generated Grafana dashboard: {}", template_name);
        Ok(json)
    }

    /// Build Grafana dashboard JSON structure
    fn build_grafana_dashboard(&self, config: &DashboardConfig) -> Result<Value> {
        let panels: Vec<Value> = config
            .panels
            .iter()
            .map(|panel| self.build_grafana_panel(panel))
            .collect::<Result<Vec<_>>>()?;

        let templating = json!({
            "list": config.variables.iter()
                .map(|var| self.build_grafana_variable(var))
                .collect::<Vec<_>>()
        });

        let time_config = json!({
            "from": config.time_range.from,
            "to": config.time_range.to
        });

        let dashboard = json!({
            "dashboard": {
                "id": null,
                "title": config.title,
                "description": config.description,
                "tags": config.tags,
                "timezone": "browser",
                "panels": panels,
                "templating": templating,
                "time": time_config,
                "refresh": format!("{}s", config.refresh_interval.as_secs()),
                "schemaVersion": 27,
                "version": 1,
                "links": [],
                "graphTooltip": 0,
                "editable": true,
                "gnetId": null,
                "hideControls": false,
                "sharedCrosshair": false,
                "style": "dark",
                "uid": null
            },
            "overwrite": false
        });

        Ok(dashboard)
    }

    /// Build Grafana panel JSON
    fn build_grafana_panel(&self, panel: &PanelConfig) -> Result<Value> {
        let targets: Vec<Value> = panel
            .targets
            .iter()
            .map(|target| {
                json!({
                    "expr": target.expr,
                    "legendFormat": target.legend_format,
                    "refId": target.ref_id,
                    "interval": target.interval
                })
            })
            .collect();

        let panel_json = json!({
            "id": panel.id,
            "title": panel.title,
            "type": panel.panel_type,
            "gridPos": {
                "x": panel.grid_pos.x,
                "y": panel.grid_pos.y,
                "w": panel.grid_pos.w,
                "h": panel.grid_pos.h
            },
            "datasource": panel.datasource,
            "targets": targets,
            "options": panel.options,
            "fieldConfig": {
                "defaults": {
                    "color": {
                        "mode": "palette-classic"
                    },
                    "custom": {
                        "axisLabel": "",
                        "axisPlacement": "auto",
                        "barAlignment": 0,
                        "drawStyle": "line",
                        "fillOpacity": 10,
                        "gradientMode": "none",
                        "hideFrom": {
                            "legend": false,
                            "tooltip": false,
                            "vis": false
                        },
                        "lineInterpolation": "linear",
                        "lineWidth": 1,
                        "pointSize": 5,
                        "scaleDistribution": {
                            "type": "linear"
                        },
                        "showPoints": "never",
                        "spanNulls": false,
                        "stacking": {
                            "group": "A",
                            "mode": "none"
                        },
                        "thresholdsStyle": {
                            "mode": "off"
                        }
                    },
                    "mappings": [],
                    "thresholds": {
                        "mode": "absolute",
                        "steps": [
                            {
                                "color": "green",
                                "value": null
                            },
                            {
                                "color": "red",
                                "value": 80
                            }
                        ]
                    },
                    "unit": "short"
                },
                "overrides": []
            }
        });

        Ok(panel_json)
    }

    /// Build Grafana variable JSON
    fn build_grafana_variable(&self, variable: &VariableConfig) -> Value {
        json!({
            "name": variable.name,
            "type": variable.var_type,
            "query": variable.query,
            "current": {
                "value": variable.current,
                "text": variable.current
            },
            "options": variable.options.iter()
                .map(|opt| json!({
                    "text": opt,
                    "value": opt
                }))
                .collect::<Vec<_>>(),
            "refresh": 1,
            "includeAll": false,
            "multi": false,
            "allValue": null,
            "hide": 0
        })
    }

    /// Get available dashboard templates
    pub fn get_templates(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }

    /// Add custom dashboard template
    pub fn add_template(&mut self, template: DashboardConfig) {
        info!("Adding custom dashboard template: {}", template.name);
        self.templates.insert(template.name.clone(), template);
    }

    /// Generate dashboard for specific metrics
    pub fn generate_custom_dashboard(
        &self,
        metrics: &SystemMetrics,
        providers: &HashMap<String, ProviderMetrics>,
    ) -> Result<String> {
        let mut panels = Vec::new();
        let mut panel_id = 1;

        // Add system metrics panels
        panels.push(PanelConfig {
            id: panel_id,
            title: "Current CPU Usage".to_string(),
            panel_type: PanelType::Stat,
            grid_pos: GridPos {
                x: 0,
                y: 0,
                w: 6,
                h: 4,
            },
            datasource: "prometheus".to_string(),
            targets: vec![QueryTarget {
                expr: format!("{}", metrics.cpu_usage),
                legend_format: Some("CPU %".to_string()),
                ref_id: "A".to_string(),
                interval: None,
            }],
            options: HashMap::new(),
        });
        panel_id += 1;

        // Add provider panels
        for (provider_name, provider_metrics) in providers {
            panels.push(PanelConfig {
                id: panel_id,
                title: format!("{provider_name} Success Rate"),
                panel_type: PanelType::Gauge,
                grid_pos: GridPos {
                    x: ((panel_id - 2) % 4) * 6,
                    y: 4 + ((panel_id - 2) / 4) * 4,
                    w: 6,
                    h: 4,
                },
                datasource: "prometheus".to_string(),
                targets: vec![QueryTarget {
                    expr: format!(
                        "{}",
                        if provider_metrics.total_requests > 0 {
                            (provider_metrics.successful_requests as f64
                                / provider_metrics.total_requests as f64)
                                * 100.0
                        } else {
                            0.0
                        }
                    ),
                    legend_format: Some("Success %".to_string()),
                    ref_id: "A".to_string(),
                    interval: None,
                }],
                options: HashMap::new(),
            });
            panel_id += 1;
        }

        let config = DashboardConfig {
            name: "custom-realtime".to_string(),
            title: "Real-time NestGate Metrics".to_string(),
            description: "Current system and provider metrics".to_string(),
            tags: vec!["nestgate".to_string(), "realtime".to_string()],
            refresh_interval: Duration::from_secs(5),
            time_range: TimeRange {
                from: "now-5m".to_string(),
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
                debug_info: None,
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

/// Generate a simple HTML dashboard for development
pub fn generate_html_dashboard(
    system_metrics: &SystemMetrics,
    provider_metrics: &HashMap<String, ProviderMetrics>,
) -> String {
    let mut html = String::from(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>NestGate Monitoring Dashboard</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; background: #1a1a1a; color: #fff; }
        .container { max-width: 1200px; margin: 0 auto; }
        .header { text-align: center; margin-bottom: 30px; }
        .metrics-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
        .metric-card { background: #2a2a2a; padding: 20px; border-radius: 8px; border-left: 4px solid #007acc; }
        .metric-title { font-size: 18px; font-weight: bold; margin-bottom: 10px; color: #007acc; }
        .metric-value { font-size: 24px; font-weight: bold; margin-bottom: 5px; }
        .metric-description { font-size: 14px; color: #ccc; }
        .status-healthy { color: #28a745; }
        .status-warning { color: #ffc107; }
        .status-error { color: #dc3545; }
        .timestamp { text-align: center; margin-top: 20px; color: #666; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 NestGate Monitoring Dashboard</h1>
            <p>Real-time system and provider metrics</p>
        </div>
        
        <div class="metrics-grid">
"#,
    );

    // System metrics
    html.push_str(&format!(
        r#"
            <div class="metric-card">
                <div class="metric-title">CPU Usage</div>
                <div class="metric-value {}">{:.1}%</div>
                <div class="metric-description">Current CPU utilization</div>
            </div>
            
            <div class="metric-card">
                <div class="metric-title">Memory Usage</div>
                <div class="metric-value {}">{:.1}%</div>
                <div class="metric-description">{} MB used of {} MB total</div>
            </div>
            
            <div class="metric-card">
                <div class="metric-title">Active Connections</div>
                <div class="metric-value">{}</div>
                <div class="metric-description">Current network connections</div>
            </div>
"#,
        if system_metrics.cpu_usage > 80.0 {
            "status-error"
        } else if system_metrics.cpu_usage > 60.0 {
            "status-warning"
        } else {
            "status-healthy"
        },
        system_metrics.cpu_usage,
        if system_metrics.memory_usage > 0 && system_metrics.memory_available > 0 {
            let total = system_metrics.memory_usage + system_metrics.memory_available;
            let percent = (system_metrics.memory_usage as f64 / total as f64) * 100.0;
            if percent > 80.0 {
                "status-error"
            } else if percent > 60.0 {
                "status-warning"
            } else {
                "status-healthy"
            }
        } else {
            "status-healthy"
        },
        if system_metrics.memory_usage > 0 && system_metrics.memory_available > 0 {
            let total = system_metrics.memory_usage + system_metrics.memory_available;
            (system_metrics.memory_usage as f64 / total as f64) * 100.0
        } else {
            0.0
        },
        system_metrics.memory_usage / 1024 / 1024,
        (system_metrics.memory_usage + system_metrics.memory_available) / 1024 / 1024,
        system_metrics.active_connections
    ));

    // Provider metrics
    for (name, metrics) in provider_metrics {
        let success_rate = if metrics.total_requests > 0 {
            (metrics.successful_requests as f64 / metrics.total_requests as f64) * 100.0
        } else {
            100.0
        };

        html.push_str(&format!(
            r#"
            <div class="metric-card">
                <div class="metric-title">Provider: {}</div>
                <div class="metric-value {}">{:.1}%</div>
                <div class="metric-description">Success rate ({} requests, {:.1}ms avg)</div>
            </div>
"#,
            name,
            if success_rate < 95.0 {
                "status-error"
            } else if success_rate < 99.0 {
                "status-warning"
            } else {
                "status-healthy"
            },
            success_rate,
            metrics.total_requests,
            metrics.avg_response_time_ms
        ));
    }

    html.push_str(&format!(
        r#"
        </div>
        
        <div class="timestamp">
            Last updated: {:?}
        </div>
    </div>
</body>
</html>
"#,
        SystemTime::now()
    ));

    html
}
