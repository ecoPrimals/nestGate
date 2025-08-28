//! **HTML DASHBOARD GENERATION**
//!
//! HTML dashboard generation functionality for development and standalone monitoring.
//! Extracted from dashboards.rs for file size compliance.

use crate::monitoring::{ProviderMetrics, SystemMetrics};
use std::collections::HashMap;
use std::time::SystemTime;

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
                <div class="metric-description">{} MB / {} MB</div>
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
