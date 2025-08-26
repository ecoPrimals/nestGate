use std::collections::HashMap;
///
/// This module generates HTML dashboards for development and quick viewing.
use crate::monitoring::{ProviderMetrics, SystemMetrics};
use std::collections::HashMap;

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
                <div class="metric-title">System CPU Usage</div>
                <div class="metric-value status-{}">{}%</div>
                <div class="metric-description">Current CPU utilization</div>
            </div>
            
            <div class="metric-card">
                <div class="metric-title">Memory Usage</div>
                <div class="metric-value status-{}">{}%</div>
                <div class="metric-description">Current memory utilization</div>
            </div>
            
            <div class="metric-card">
                <div class="metric-title">Disk Usage</div>
                <div class="metric-value status-{}">{}%</div>
                <div class="metric-description">Current disk utilization</div>
            </div>
"#,
        get_status_class(system_metrics.cpu_usage),
        system_metrics.cpu_usage,
        get_status_class(system_metrics.memory_usage as f64),
        system_metrics.memory_usage,
        get_status_class(system_metrics.disk_usage as f64),
        system_metrics.disk_usage,
    ));

    // Provider metrics
    for (provider_name, metrics) in provider_metrics {
        html.push_str(&format!(
            r#"
            <div class="metric-card">
                <div class="metric-title">Provider: {}</div>
                <div class="metric-value status-{}">{}ms</div>
                <div class="metric-description">Average response time</div>
            </div>
"#,
            provider_name,
            get_latency_status_class(metrics.avg_response_time_ms as u64),
            metrics.avg_response_time_ms,
        ));
    }

    html.push_str(
        r#"
        </div>
        
        <div class="timestamp">
            Last updated: <span id="timestamp"></span>
        </div>
    </div>
    
    <script>
        document.getElementById('timestamp').textContent = new Date().toLocaleString();
        
        // Auto-refresh every 30 seconds
        setInterval(() => {
            location.reload();
        }, 30000);
    </script>
</body>
</html>
"#,
    );

    html
}

/// Get CSS class for status based on percentage
fn get_status_class(percentage: f64) -> &'static str {
    if percentage < 70.0 {
        "healthy"
    } else if percentage < 90.0 {
        "warning"
    } else {
        "error"
    }
}

/// Get CSS class for latency status
fn get_latency_status_class(latency_ms: u64) -> &'static str {
    if latency_ms < 100 {
        "healthy"
    } else if latency_ms < 500 {
        "warning"
    } else {
        "error"
    }
}
