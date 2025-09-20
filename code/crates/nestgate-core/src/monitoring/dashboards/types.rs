// **DASHBOARD TYPES**
//! Type definitions and data structures.
// Core types and configuration structures for dashboard generation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

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
    pub options: PanelOptions,
}
/// Panel types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PanelType {
    Graph,
    Stat,
    Table,
    Heatmap,
    Gauge,
    BarGauge,
    Logs,
    Text,
    Alert,
    Piechart,
}
/// Grid position for panels
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
    pub expr: String,
    pub legend_format: Option<String>,
    pub interval: Option<String>,
    pub ref_id: String,
}
/// Panel options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelOptions {
    pub legend: Option<LegendConfig>,
    pub tooltip: Option<TooltipConfig>,
    pub axes: Option<AxesConfig>,
    pub thresholds: Vec<Threshold>,
    pub colors: Vec<String>,
}
/// Legend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegendConfig {
    pub show: bool,
    pub as_table: bool,
    pub to_the_right: bool,
    pub values: Vec<String>,
}
/// Tooltip configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TooltipConfig {
    pub shared: bool,
    pub sort: u32,
    pub value_type: String,
}
/// Axes configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxesConfig {
    pub left: AxisConfig,
    pub right: Option<AxisConfig>,
}
/// Axis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxisConfig {
    pub show: bool,
    pub label: String,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub unit: String,
}
/// Threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threshold {
    pub value: f64,
    pub color: String,
    pub op: String,
}
/// Time range configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub from: String,
    pub to: String,
}
impl Default for TimeRange {
    fn default() -> Self {
        Self {
            from: "now-1h".to_string(),
            to: "now".to_string(),
        }
    }
}

/// Dashboard variable configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableConfig {
    pub name: String,
    pub display_name: String,
    pub var_type: VariableType,
    pub query: Option<String>,
    pub options: Vec<VariableOption>,
    pub multi: bool,
    pub include_all: bool,
}
/// Variable types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableType {
    Query,
    Custom,
    Constant,
    Datasource,
    Interval,
    Textbox,
}
/// Variable option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableOption {
    pub text: String,
    pub value: String,
    pub selected: bool,
}
/// Dashboard template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardTemplate {
    pub name: String,
    pub description: String,
    pub category: String,
    pub config: DashboardConfig,
    pub metadata: HashMap<String, String>,
}
/// Dashboard export format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Json,
    Yaml,
    Grafana,
    Prometheus,
}
/// Dashboard generation result
#[derive(Debug, Clone)]
pub struct DashboardResult {
    pub config: DashboardConfig,
    pub json: String,
    pub format: ExportFormat,
    pub metadata: HashMap<String, String>,
}
