// **DASHBOARD TYPES**
//! Type definitions and data structures.
// Core types and configuration structures for dashboard generation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Dashboard
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
/// Configuration for Panel
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
/// Types of Panel
pub enum PanelType {
    /// Graph
    Graph,
    /// Stat
    Stat,
    /// Table
    Table,
    /// Heatmap
    Heatmap,
    /// Gauge
    Gauge,
    /// Bargauge
    BarGauge,
    /// Logs
    Logs,
    /// Text
    Text,
    /// Alert
    Alert,
    /// Piechart
    Piechart,
}
/// Grid position for panels
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Gridpos
pub struct GridPos {
    /// X
    pub x: u32,
    /// Y
    pub y: u32,
    /// W
    pub w: u32,
    /// H
    pub h: u32,
}
/// Query target for panels
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Querytarget
pub struct QueryTarget {
    /// Expr
    pub expr: String,
    /// Legend Format
    pub legend_format: Option<String>,
    /// Interval
    pub interval: Option<String>,
    /// Ref identifier
    pub ref_id: String,
}
/// Panel options
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Paneloptions
pub struct PanelOptions {
    /// Legend
    pub legend: Option<LegendConfig>,
    /// Tooltip
    pub tooltip: Option<TooltipConfig>,
    /// Axes
    pub axes: Option<AxesConfig>,
    /// Thresholds
    pub thresholds: Vec<Threshold>,
    /// Colors
    pub colors: Vec<String>,
}
/// Legend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Legend
pub struct LegendConfig {
    /// Show
    pub show: bool,
    /// As Table
    pub as_table: bool,
    /// To The Right
    pub to_the_right: bool,
    /// Values
    pub values: Vec<String>,
}
/// Tooltip configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Tooltip
pub struct TooltipConfig {
    /// Shared
    pub shared: bool,
    /// Sort
    pub sort: u32,
    /// Value Type
    pub value_type: String,
}
/// Axes configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Axes
pub struct AxesConfig {
    /// Left
    pub left: AxisConfig,
    /// Right
    pub right: Option<AxisConfig>,
}
/// Axis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Axis
pub struct AxisConfig {
    /// Show
    pub show: bool,
    /// Label
    pub label: String,
    /// Min
    pub min: Option<f64>,
    /// Max
    pub max: Option<f64>,
    /// Unit
    pub unit: String,
}
/// Threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Threshold
pub struct Threshold {
    /// Value
    pub value: f64,
    /// Color
    pub color: String,
    /// Op
    pub op: String,
}
/// Time range configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Timerange
pub struct TimeRange {
    /// From
    pub from: String,
    /// To
    pub to: String,
}
impl Default for TimeRange {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            from: "now-1h".to_string(),
            to: "now".to_string(),
        }
    }
}

/// Dashboard variable configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Variable
pub struct VariableConfig {
    /// Name
    pub name: String,
    /// Display name
    pub display_name: String,
    /// Var Type
    pub var_type: VariableType,
    /// Query
    pub query: Option<String>,
    /// Options
    pub options: Vec<VariableOption>,
    /// Multi
    pub multi: bool,
    /// Include All
    pub include_all: bool,
}
/// Variable types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Variable
pub enum VariableType {
    /// Query
    Query,
    /// Custom
    Custom,
    /// Constant
    Constant,
    /// Datasource
    Datasource,
    /// Interval
    Interval,
    /// Textbox
    Textbox,
}
/// Variable option
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Variableoption
pub struct VariableOption {
    /// Text
    pub text: String,
    /// Value
    pub value: String,
    /// Selected
    pub selected: bool,
}
/// Dashboard template
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Dashboardtemplate
pub struct DashboardTemplate {
    /// Name
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Category
    pub category: String,
    /// Configuration for 
    pub config: DashboardConfig,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// Dashboard export format
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Exportformat
pub enum ExportFormat {
    /// Json
    Json,
    /// Yaml
    Yaml,
    /// Grafana
    Grafana,
    /// Prometheus
    Prometheus,
}
/// Dashboard generation result
#[derive(Debug, Clone)]
/// Dashboardresult
pub struct DashboardResult {
    /// Configuration for 
    pub config: DashboardConfig,
    /// Json
    pub json: String,
    /// Format
    pub format: ExportFormat,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
