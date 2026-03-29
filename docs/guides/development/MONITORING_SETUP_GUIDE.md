> **Historical**: This document was written in November 20, 2025. Current architecture
> and patterns may differ. See root-level docs and `specs/` for current specifications.

# 📊 MONITORING & OBSERVABILITY SETUP GUIDE

**Version**: 2.0.0  
**Date**: November 20, 2025  
**For**: NestGate Production Deployment

---

## ⚡ QUICK SETUP

```bash
# 1. Start monitoring stack
docker-compose -f docker/monitoring-stack.yml up -d

# 2. Configure Prometheus
cp config/prometheus-production.yml /etc/prometheus/prometheus.yml

# 3. Import Grafana dashboards
./scripts/import-dashboards.sh

# 4. Set up Sentry
export SENTRY_DSN="your-sentry-dsn"

# 5. Verify monitoring
./scripts/verify-monitoring.sh
```

---

## 📊 MONITORING STACK

### Core Components:

1. **Prometheus** - Metrics collection
2. **Grafana** - Visualization
3. **Loki** - Log aggregation
4. **Sentry** - Error tracking
5. **AlertManager** - Alert routing

### Architecture:

```
NestGate API (port 8080)
    ↓ metrics endpoint (/metrics)
Prometheus (port 9090)
    ↓ scrapes metrics
    → AlertManager (port 9093) → Slack/PagerDuty
    → Grafana (port 3000) → Dashboards
    
NestGate Logs
    ↓ structured JSON logs
Loki (port 3100)
    ↓ aggregates logs
Grafana → Log explorer

NestGate Errors
    ↓ panic/error events
Sentry → Error tracking & alerts
```

---

## 🔧 PROMETHEUS SETUP

### Configuration:

Create `/etc/prometheus/prometheus.yml`:

```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s
  external_labels:
    cluster: 'nestgate-production'
    environment: 'production'

# Alertmanager configuration
alerting:
  alertmanagers:
    - static_configs:
        - targets: ['localhost:9093']

# Load rules
rule_files:
  - '/etc/prometheus/rules/*.yml'

# Scrape configurations
scrape_configs:
  # NestGate API
  - job_name: 'nestgate-api'
    static_configs:
      - targets: ['localhost:8080']
    metrics_path: '/metrics'
    scrape_interval: 10s
    
  # NestGate Core Services
  - job_name: 'nestgate-core'
    static_configs:
      - targets: ['localhost:8081']
    metrics_path: '/metrics'
    
  # System Metrics
  - job_name: 'node-exporter'
    static_configs:
      - targets: ['localhost:9100']
      
  # Prometheus itself
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']
```

### Alert Rules:

Create `/etc/prometheus/rules/nestgate-alerts.yml`:

```yaml
groups:
  - name: nestgate_alerts
    interval: 30s
    rules:
      # High error rate
      - alert: HighErrorRate
        expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.05
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "High error rate detected"
          description: "Error rate is {{ $value }}% over last 5 minutes"
          
      # High latency
      - alert: HighLatency
        expr: histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m])) > 0.5
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High API latency"
          description: "P95 latency is {{ $value }}s"
          
      # Service down
      - alert: ServiceDown
        expr: up{job="nestgate-api"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "NestGate API is down"
          description: "{{ $labels.instance }} is unreachable"
          
      # High CPU usage
      - alert: HighCPUUsage
        expr: (100 - (avg by (instance) (irate(node_cpu_seconds_total{mode="idle"}[5m])) * 100)) > 80
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "High CPU usage"
          description: "CPU usage is {{ $value }}% on {{ $labels.instance }}"
          
      # High memory usage
      - alert: HighMemoryUsage
        expr: (node_memory_MemTotal_bytes - node_memory_MemAvailable_bytes) / node_memory_MemTotal_bytes * 100 > 85
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "High memory usage"
          description: "Memory usage is {{ $value }}% on {{ $labels.instance }}"
          
      # Disk space low
      - alert: DiskSpaceLow
        expr: (node_filesystem_avail_bytes{fstype!="tmpfs"} / node_filesystem_size_bytes * 100) < 20
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Disk space low"
          description: "Only {{ $value }}% disk space left on {{ $labels.mountpoint }}"
```

---

## 📈 GRAFANA DASHBOARDS

### Dashboard 1: System Overview

**Panels**:
- CPU Usage (gauge)
- Memory Usage (gauge)
- Disk I/O (graph)
- Network Traffic (graph)
- System Load (graph)

**PromQL Queries**:
```promql
# CPU Usage
100 - (avg by(instance) (irate(node_cpu_seconds_total{mode="idle"}[5m])) * 100)

# Memory Usage
(node_memory_MemTotal_bytes - node_memory_MemAvailable_bytes) / node_memory_MemTotal_bytes * 100

# Disk I/O
rate(node_disk_read_bytes_total[5m])
rate(node_disk_written_bytes_total[5m])
```

### Dashboard 2: Application Metrics

**Panels**:
- Request Rate (graph)
- Error Rate (graph)
- Response Time P50/P95/P99 (graph)
- Active Connections (gauge)
- Request by Endpoint (table)

**PromQL Queries**:
```promql
# Request Rate
rate(http_requests_total[5m])

# Error Rate
rate(http_requests_total{status=~"5.."}[5m]) / rate(http_requests_total[5m]) * 100

# Response Time
histogram_quantile(0.50, rate(http_request_duration_seconds_bucket[5m]))
histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))
histogram_quantile(0.99, rate(http_request_duration_seconds_bucket[5m]))
```

### Dashboard 3: Business Metrics

**Panels**:
- API Calls by Type
- Storage Operations
- Authentication Requests
- ZFS Pool Health
- Cache Hit Rate

---

## 📝 LOKI LOG AGGREGATION

### Configuration:

Create `/etc/loki/loki-config.yml`:

```yaml
auth_enabled: false

server:
  http_listen_port: 3100

ingester:
  lifecycler:
    address: 127.0.0.1
    ring:
      kvstore:
        store: inmemory
      replication_factor: 1
  chunk_idle_period: 5m
  chunk_retain_period: 30s

schema_config:
  configs:
    - from: 2024-01-01
      store: boltdb
      object_store: filesystem
      schema: v11
      index:
        prefix: index_
        period: 168h

storage_config:
  boltdb:
    directory: /tmp/loki/index
  filesystem:
    directory: /tmp/loki/chunks

limits_config:
  enforce_metric_name: false
  reject_old_samples: true
  reject_old_samples_max_age: 168h

chunk_store_config:
  max_look_back_period: 0s

table_manager:
  retention_deletes_enabled: false
  retention_period: 0s
```

### Promtail Configuration:

Create `/etc/promtail/promtail-config.yml`:

```yaml
server:
  http_listen_port: 9080
  grpc_listen_port: 0

positions:
  filename: /tmp/positions.yaml

clients:
  - url: http://localhost:3100/loki/api/v1/push

scrape_configs:
  - job_name: nestgate
    static_configs:
      - targets:
          - localhost
        labels:
          job: nestgate-logs
          __path__: /var/log/nestgate/*.log
    pipeline_stages:
      - json:
          expressions:
            level: level
            timestamp: timestamp
            message: message
      - labels:
          level:
```

---

## 🔔 ALERTMANAGER SETUP

### Configuration:

Create `/etc/alertmanager/alertmanager.yml`:

```yaml
global:
  resolve_timeout: 5m
  slack_api_url: 'https://hooks.slack.com/services/YOUR/SLACK/WEBHOOK'

route:
  group_by: ['alertname', 'cluster', 'service']
  group_wait: 10s
  group_interval: 10s
  repeat_interval: 12h
  receiver: 'default'
  routes:
    - match:
        severity: critical
      receiver: 'pagerduty'
      continue: true
    - match:
        severity: warning
      receiver: 'slack'

receivers:
  - name: 'default'
    slack_configs:
      - channel: '#nestgate-alerts'
        title: 'Alert: {{ .GroupLabels.alertname }}'
        text: '{{ range .Alerts }}{{ .Annotations.description }}{{ end }}'
        
  - name: 'slack'
    slack_configs:
      - channel: '#nestgate-alerts'
        title: 'Warning: {{ .GroupLabels.alertname }}'
        text: '{{ range .Alerts }}{{ .Annotations.description }}{{ end }}'
        
  - name: 'pagerduty'
    pagerduty_configs:
      - service_key: 'YOUR_PAGERDUTY_KEY'
        description: '{{ .GroupLabels.alertname }}: {{ range .Alerts }}{{ .Annotations.summary }}{{ end }}'

inhibit_rules:
  - source_match:
      severity: 'critical'
    target_match:
      severity: 'warning'
    equal: ['alertname', 'cluster', 'service']
```

---

## 🐛 SENTRY ERROR TRACKING

### Setup:

1. **Create Sentry Project**: https://sentry.io
2. **Get DSN**: Project Settings → Client Keys
3. **Configure Environment**:

```bash
export SENTRY_DSN="https://...@sentry.io/..."
export SENTRY_ENVIRONMENT="production"
export SENTRY_RELEASE="2.0.0"
```

### NestGate Integration:

Already integrated in `main.rs`:

```rust
// Sentry initialization
if let Ok(dsn) = std::env::var("SENTRY_DSN") {
    let _guard = sentry::init((
        dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some(std::env::var("SENTRY_ENVIRONMENT")
                .unwrap_or_else(|_| "production".into()).into()),
            ..Default::default()
        },
    ));
}
```

### Alert Rules:

- **Error spike**: >10 errors in 5 minutes
- **New error type**: First occurrence of new error
- **Performance regression**: P95 > 2x baseline

---

## 🎯 KEY METRICS TO MONITOR

### RED Metrics (Request-Error-Duration):

```yaml
metrics:
  request_rate:
    query: rate(http_requests_total[5m])
    threshold: 
      min: 10  # req/s
      max: 10000  # req/s
      
  error_rate:
    query: rate(http_requests_total{status=~"5.."}[5m]) / rate(http_requests_total[5m])
    threshold:
      warning: 0.01  # 1%
      critical: 0.05  # 5%
      
  duration:
    query: histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))
    threshold:
      warning: 0.5  # 500ms
      critical: 1.0  # 1s
```

### USE Metrics (Utilization-Saturation-Errors):

```yaml
system_metrics:
  cpu_utilization:
    threshold:
      warning: 70%
      critical: 85%
      
  memory_utilization:
    threshold:
      warning: 80%
      critical: 90%
      
  disk_utilization:
    threshold:
      warning: 75%
      critical: 85%
      
  network_errors:
    threshold:
      warning: 0.01%
      critical: 0.1%
```

---

## 📊 MONITORING BEST PRACTICES

### 1. **Layered Monitoring**:
- **Symptoms** (user-facing): Latency, errors, availability
- **Causes** (internal): CPU, memory, disk, network
- **Business** (metrics): API usage, feature adoption

### 2. **Alert Hygiene**:
- ✅ Every alert must be actionable
- ✅ Define severity levels clearly
- ✅ Avoid alert fatigue
- ✅ Regular alert review and tuning

### 3. **Dashboard Organization**:
- **Executive**: High-level health
- **Operations**: Detailed system metrics
- **Development**: Application metrics
- **Business**: Usage and adoption

### 4. **Log Levels**:
- **ERROR**: Something failed
- **WARN**: Something unexpected
- **INFO**: Normal operations
- **DEBUG**: Detailed troubleshooting
- **TRACE**: Very detailed (disabled in production)

---

## ✅ MONITORING CHECKLIST

### Setup Phase:
- [ ] Prometheus installed and configured
- [ ] Grafana installed and dashboards imported
- [ ] Loki and Promtail configured
- [ ] Sentry project created and configured
- [ ] AlertManager configured
- [ ] Alert destinations configured (Slack, PagerDuty)

### Validation Phase:
- [ ] Metrics being collected
- [ ] Dashboards showing data
- [ ] Logs being aggregated
- [ ] Errors tracked in Sentry
- [ ] Test alert sent successfully
- [ ] All alert channels working

### Operational Phase:
- [ ] On-call rotation established
- [ ] Runbooks created
- [ ] Alert response procedures documented
- [ ] Team trained on monitoring tools
- [ ] Regular monitoring review scheduled

---

## 🚀 QUICK START SCRIPTS

### Verify Monitoring:

```bash
#!/bin/bash
# scripts/verify-monitoring.sh

echo "🔍 Verifying monitoring stack..."

# Check Prometheus
if curl -s http://localhost:9090/-/healthy | grep -q "Prometheus is Healthy"; then
    echo "✅ Prometheus: OK"
else
    echo "❌ Prometheus: FAILED"
fi

# Check Grafana
if curl -s http://localhost:3000/api/health | grep -q "ok"; then
    echo "✅ Grafana: OK"
else
    echo "❌ Grafana: FAILED"
fi

# Check Loki
if curl -s http://localhost:3100/ready | grep -q "ready"; then
    echo "✅ Loki: OK"
else
    echo "❌ Loki: FAILED"
fi

# Check NestGate metrics endpoint
if curl -s http://localhost:8080/metrics | grep -q "http_requests_total"; then
    echo "✅ NestGate metrics: OK"
else
    echo "❌ NestGate metrics: FAILED"
fi

echo "✅ Monitoring verification complete!"
```

---

## 📚 RESOURCES

### Documentation:
- Prometheus: https://prometheus.io/docs/
- Grafana: https://grafana.com/docs/
- Loki: https://grafana.com/docs/loki/
- Sentry: https://docs.sentry.io/

### Pre-built Dashboards:
- Grafana Dashboard Library: https://grafana.com/grafana/dashboards/
- Node Exporter Dashboard: #1860
- Prometheus Stats: #3662

---

**Status**: ✅ **MONITORING SETUP GUIDE COMPLETE**  
**Next**: Configure infrastructure and start monitoring stack

**Your monitoring will provide**:
- Real-time visibility into system health
- Proactive alerting before issues become critical
- Historical data for capacity planning
- Rapid incident response capabilities

**Let's monitor it! 📊**

