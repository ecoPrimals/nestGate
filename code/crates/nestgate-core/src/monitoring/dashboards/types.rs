use std::collections::HashMap;
///
/// This module contains all type definitions for the dashboard system including
/// dashboard configuration, panel types, and variable definitions.
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

impl Default for TimeRange {
    fn default() -> Self {
        Self {
            from: "now-1h".to_string(),
            to: "now".to_string(),
        }
    }
}

impl Default for GridPos {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            w: 12,
            h: 8,
        }
    }
}
