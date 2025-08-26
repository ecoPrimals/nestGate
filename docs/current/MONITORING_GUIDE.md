# 📊 **NestGate v2.0 Monitoring & Observability Guide**

**Date**: January 30, 2025  
**Version**: NestGate v2.0 - Production Monitoring  
**Status**: ✅ **PRODUCTION READY**

---

## 📋 **OVERVIEW**

NestGate v2.0 provides comprehensive monitoring and observability capabilities designed for production environments. This guide covers all aspects of monitoring, alerting, and troubleshooting your NestGate deployment.

### **Monitoring Architecture**

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   NestGate      │    │   Prometheus    │    │    Grafana      │
│   Application   │───▶│   Metrics       │───▶│   Dashboard     │
│                 │    │   Collection    │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                        │                        │
         ▼                        ▼                        ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Health        │    │   Alertmanager  │    │   Log           │
│   Endpoints     │    │   Notifications │    │   Aggregation   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

---

## 🔍 **HEALTH MONITORING**

### **Health Endpoints**

NestGate v2.0 provides multiple health check endpoints for different monitoring needs:

#### **Basic Health Check**
```bash
# Simple health status
curl http://localhost:8081/health

# Expected Response:
{
  "status": "healthy",
  "timestamp": "2025-01-30T12:00:00Z",
  "version": "2.0.0"
}
```

#### **Detailed Health Check**
```bash
# Comprehensive system status
curl http://localhost:8081/health/detailed

# Expected Response:
{
  "status": "healthy",
  "timestamp": "2025-01-30T12:00:00Z",
  "version": "2.0.0",
  "components": {
    "zfs": {
      "status": "healthy",
      "pool_status": "ONLINE",
      "available_space": "1.2TB",
      "used_space": "800GB"
    },
    "universal_adapter": {
      "status": "healthy",
      "discovered_services": 4,
      "active_connections": 3
    },
    "api": {
      "status": "healthy",
      "active_connections": 15,
      "requests_per_second": 45.2
    },
    "storage": {
      "status": "healthy",
      "tier_hot_usage": "65%",
      "tier_warm_usage": "40%",
      "tier_cold_usage": "20%"
    }
  },
  "ecosystem": {
    "beardog": "connected",
    "songbird": "connected", 
    "squirrel": "connected",
    "toadstool": "discovering"
  }
}
```

#### **Readiness Probe**
```bash
# Kubernetes readiness check
curl http://localhost:8081/ready

# Response: 200 OK when ready to serve traffic
```

#### **Liveness Probe**
```bash
# Kubernetes liveness check
curl http://localhost:8081/alive

# Response: 200 OK when process is alive
```

### **Health Check Configuration**

Configure health checks in your `production.toml`:

```toml
[monitoring.health]
enabled = true
endpoint = "/health"
detailed_endpoint = "/health/detailed"
check_interval_seconds = 10

# Component-specific health checks
[monitoring.health.components]
zfs_timeout_seconds = 5
adapter_timeout_seconds = 3
api_timeout_seconds = 2
storage_timeout_seconds = 10

# Thresholds for health status
[monitoring.health.thresholds]
max_response_time_ms = 1000
max_error_rate_percent = 5.0
min_available_space_gb = 10
max_cpu_usage_percent = 90.0
max_memory_usage_percent = 85.0
```

---

## 📈 **METRICS COLLECTION**

### **Prometheus Metrics**

NestGate v2.0 exposes comprehensive metrics via Prometheus endpoint:

```bash
# Access metrics endpoint
curl http://localhost:9090/metrics
```

#### **Core Application Metrics**

| **Metric Name** | **Type** | **Description** |
|-----------------|----------|-----------------|
| `nestgate_requests_total` | Counter | Total HTTP requests |
| `nestgate_request_duration_seconds` | Histogram | Request latency distribution |
| `nestgate_active_connections` | Gauge | Current active connections |
| `nestgate_storage_operations_total` | Counter | Storage operations count |
| `nestgate_storage_bytes_total` | Counter | Total bytes stored/retrieved |
| `nestgate_zfs_pool_usage_percent` | Gauge | ZFS pool utilization |
| `nestgate_ecosystem_connections` | Gauge | Active ecosystem connections |

#### **System Metrics**

| **Metric Name** | **Type** | **Description** |
|-----------------|----------|-----------------|
| `nestgate_memory_usage_bytes` | Gauge | Memory consumption |
| `nestgate_cpu_usage_percent` | Gauge | CPU utilization |
| `nestgate_disk_usage_bytes` | Gauge | Disk space usage |
| `nestgate_network_bytes_total` | Counter | Network I/O |
| `nestgate_goroutines_count` | Gauge | Active goroutines |
| `nestgate_gc_duration_seconds` | Histogram | Garbage collection time |

#### **Business Metrics**

| **Metric Name** | **Type** | **Description** |
|-----------------|----------|-----------------|
| `nestgate_files_stored_total` | Counter | Total files stored |
| `nestgate_data_tiers_usage` | Gauge | Usage by storage tier |
| `nestgate_compression_ratio` | Gauge | Average compression ratio |
| `nestgate_deduplication_savings` | Gauge | Space saved by dedup |
| `nestgate_snapshot_count` | Gauge | Number of snapshots |
| `nestgate_backup_success_total` | Counter | Successful backups |

### **Custom Metrics**

Add custom metrics for your specific use cases:

```toml
[monitoring.custom_metrics]
enable_business_metrics = true
enable_performance_metrics = true
enable_security_metrics = true

# Custom metric definitions
[[monitoring.custom_metrics.definitions]]
name = "nestgate_custom_operations"
type = "counter"
description = "Custom business operations"
labels = ["operation_type", "user_id", "tenant"]
```

---

## 📊 **GRAFANA DASHBOARDS**

### **Pre-built Dashboards**

NestGate v2.0 includes production-ready Grafana dashboards:

#### **1. System Overview Dashboard**
- **Purpose**: High-level system health and performance
- **Panels**:
  - System health status
  - Request rate and latency
  - Resource utilization (CPU, Memory, Disk)
  - Active connections
  - Error rates

#### **2. Storage Performance Dashboard**
- **Purpose**: Storage-specific metrics and ZFS monitoring
- **Panels**:
  - ZFS pool status and utilization
  - Storage tier distribution
  - I/O operations per second
  - Compression and deduplication ratios
  - Snapshot and backup status

#### **3. Ecosystem Integration Dashboard**
- **Purpose**: Universal adapter and ecosystem connectivity
- **Panels**:
  - Discovered services status
  - Capability utilization
  - Inter-service communication metrics
  - Failover and fallback events
  - Service discovery timeline

#### **4. Security and Compliance Dashboard**
- **Purpose**: Security events and compliance monitoring
- **Panels**:
  - Authentication attempts and failures
  - TLS certificate status
  - Security violations
  - Audit log events
  - Privacy compliance metrics

### **Dashboard Configuration**

#### **Prometheus Data Source**
```yaml
# datasources/prometheus.yml
apiVersion: 1

datasources:
  - name: Prometheus
    type: prometheus
    access: proxy
    url: http://prometheus:9090
    isDefault: true
    editable: true
```

#### **Dashboard Provisioning**
```yaml
# dashboards/nestgate.yml
apiVersion: 1

providers:
  - name: 'nestgate-dashboards'
    orgId: 1
    folder: 'NestGate'
    type: file
    disableDeletion: false
    updateIntervalSeconds: 10
    allowUiUpdates: true
    options:
      path: /etc/grafana/provisioning/dashboards
```

### **Alert Rules**

Configure alerting rules for critical metrics:

```yaml
# prometheus/rules/nestgate.yml
groups:
  - name: nestgate.rules
    rules:
      # High error rate
      - alert: NestGateHighErrorRate
        expr: rate(nestgate_requests_total{status=~"5.."}[5m]) > 0.05
        for: 2m
        labels:
          severity: warning
        annotations:
          summary: "High error rate detected"
          description: "Error rate is {{ $value }} errors per second"

      # High memory usage
      - alert: NestGateHighMemoryUsage
        expr: nestgate_memory_usage_bytes / nestgate_memory_limit_bytes > 0.85
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High memory usage"
          description: "Memory usage is {{ $value | humanizePercentage }}"

      # ZFS pool full
      - alert: NestGateZFSPoolFull
        expr: nestgate_zfs_pool_usage_percent > 90
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "ZFS pool nearly full"
          description: "ZFS pool usage is {{ $value }}%"

      # Service discovery failure
      - alert: NestGateEcosystemDisconnected
        expr: nestgate_ecosystem_connections < 2
        for: 3m
        labels:
          severity: warning
        annotations:
          summary: "Ecosystem connectivity issues"
          description: "Only {{ $value }} ecosystem services connected"
```

---

## 📝 **LOGGING**

### **Log Configuration**

Configure comprehensive logging in `production.toml`:

```toml
[monitoring.logging]
level = "info"
format = "json"
output = "file"
file_path = "/opt/nestgate/logs/nestgate.log"
max_file_size_mb = 100
max_files = 10
enable_compression = true

# Structured logging fields
[monitoring.logging.fields]
include_timestamp = true
include_level = true
include_source = true
include_trace_id = true
include_span_id = true

# Log sampling (for high-volume environments)
[monitoring.logging.sampling]
enabled = false
sample_rate = 0.1  # Sample 10% of logs
```

### **Log Levels and Content**

#### **ERROR Level**
- System failures and critical errors
- ZFS operation failures
- Authentication failures
- Ecosystem communication errors

#### **WARN Level**
- Performance degradation
- Fallback activations
- Configuration warnings
- Resource threshold breaches

#### **INFO Level**
- Service startup/shutdown
- Ecosystem service discovery
- Configuration changes
- Backup operations

#### **DEBUG Level**
- Request/response details
- Internal state changes
- Performance metrics
- Detailed error context

### **Log Aggregation**

#### **Loki Configuration**
```yaml
# monitoring/loki.yml
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

schema_config:
  configs:
    - from: 2020-10-24
      store: boltdb-shipper
      object_store: filesystem
      schema: v11
      index:
        prefix: index_
        period: 24h

storage_config:
  boltdb_shipper:
    active_index_directory: /loki/boltdb-shipper-active
    cache_location: /loki/boltdb-shipper-cache
    shared_store: filesystem
  filesystem:
    directory: /loki/chunks

limits_config:
  enforce_metric_name: false
  reject_old_samples: true
  reject_old_samples_max_age: 168h
```

#### **Promtail Configuration**
```yaml
# monitoring/promtail.yml
server:
  http_listen_port: 9080
  grpc_listen_port: 0

positions:
  filename: /tmp/positions.yaml

clients:
  - url: http://loki:3100/loki/api/v1/push

scrape_configs:
  - job_name: nestgate
    static_configs:
      - targets:
          - localhost
        labels:
          job: nestgate
          __path__: /var/log/nestgate/*.log
    pipeline_stages:
      - json:
          expressions:
            timestamp: timestamp
            level: level
            message: message
            component: component
      - timestamp:
          source: timestamp
          format: RFC3339
      - labels:
          level:
          component:
```

---

## 🚨 **ALERTING**

### **Alert Configuration**

#### **Alertmanager Setup**
```yaml
# monitoring/alertmanager.yml
global:
  smtp_smarthost: 'localhost:587'
  smtp_from: 'nestgate-alerts@yourcompany.com'

route:
  group_by: ['alertname']
  group_wait: 10s
  group_interval: 10s
  repeat_interval: 1h
  receiver: 'web.hook'

receivers:
  - name: 'web.hook'
    webhook_configs:
      - url: 'http://your-webhook-url'
        send_resolved: true

  - name: 'email'
    email_configs:
      - to: 'admin@yourcompany.com'
        subject: 'NestGate Alert: {{ .GroupLabels.alertname }}'
        body: |
          {{ range .Alerts }}
          Alert: {{ .Annotations.summary }}
          Description: {{ .Annotations.description }}
          {{ end }}

  - name: 'slack'
    slack_configs:
      - api_url: 'YOUR_SLACK_WEBHOOK_URL'
        channel: '#nestgate-alerts'
        title: 'NestGate Alert'
        text: '{{ range .Alerts }}{{ .Annotations.summary }}{{ end }}'
```

### **Alert Thresholds**

Configure appropriate thresholds for your environment:

```toml
[monitoring.alerting]
enabled = true
webhook_url = "https://your-alerting-system.com/webhook"

# Performance alerts
[monitoring.alerting.performance]
response_time_threshold_ms = 1000
error_rate_threshold_percent = 5.0
cpu_usage_threshold_percent = 80.0
memory_usage_threshold_percent = 85.0

# Storage alerts
[monitoring.alerting.storage]
disk_usage_threshold_percent = 90.0
zfs_pool_threshold_percent = 85.0
snapshot_failure_threshold = 3

# Ecosystem alerts
[monitoring.alerting.ecosystem]
min_connected_services = 2
discovery_timeout_threshold_seconds = 30
adapter_failure_threshold = 5
```

### **Alert Runbooks**

#### **High Memory Usage Alert**
```
1. Check current memory usage: curl http://localhost:9090/metrics | grep memory
2. Identify memory-consuming processes: top -p $(pgrep nestgate)
3. Review recent logs for memory leaks: journalctl -u nestgate --since "1 hour ago"
4. Consider restarting service if usage > 95%: systemctl restart nestgate
5. Scale up resources if pattern persists
```

#### **ZFS Pool Full Alert**
```
1. Check pool status: zpool status nestgate-pool
2. Check dataset usage: zfs list -o space nestgate-pool
3. Clean up old snapshots: zfs destroy nestgate-pool/data@old-snapshot
4. Enable compression if not active: zfs set compression=lz4 nestgate-pool
5. Add additional storage devices to pool
```

#### **Ecosystem Connectivity Alert**
```
1. Check ecosystem status: curl http://localhost:8080/api/v1/ecosystem/health
2. Review universal adapter logs: journalctl -u nestgate | grep adapter
3. Test individual service connectivity
4. Check network connectivity and DNS resolution
5. Restart service if connectivity issues persist
```

---

## 🔧 **TROUBLESHOOTING**

### **Common Issues and Solutions**

#### **High Response Times**
```bash
# Check system resources
top -p $(pgrep nestgate)
iostat -x 1

# Check ZFS performance
zpool iostat -v nestgate-pool 1

# Review slow queries
curl http://localhost:9090/metrics | grep duration

# Check for blocking operations
curl http://localhost:8081/health/detailed
```

#### **Memory Leaks**
```bash
# Monitor memory growth
watch -n 5 'ps aux | grep nestgate | grep -v grep'

# Check for memory pressure
cat /proc/pressure/memory

# Review memory metrics
curl http://localhost:9090/metrics | grep memory

# Analyze heap if available
curl http://localhost:8081/debug/pprof/heap
```

#### **Storage Issues**
```bash
# Check ZFS health
zpool status -v
zfs list -o space

# Check for corruption
zpool scrub nestgate-pool
zpool status -v nestgate-pool

# Review storage metrics
curl http://localhost:9090/metrics | grep storage

# Check disk space
df -h /opt/nestgate
```

#### **Ecosystem Connectivity**
```bash
# Test service discovery
curl http://localhost:8080/api/v1/ecosystem/services

# Check universal adapter status
curl http://localhost:8080/api/v1/adapter/status

# Review connection logs
journalctl -u nestgate | grep -i "connection\|discovery\|adapter"

# Test individual service endpoints
curl http://beardog-service:8080/health
curl http://songbird-service:8080/health
```

### **Performance Optimization**

#### **CPU Optimization**
```toml
[performance.threads]
worker_threads = 8  # Match CPU cores
max_blocking_threads = 16

[performance.cpu]
enable_cpu_affinity = true
cpu_mask = "0-7"  # Bind to specific cores
```

#### **Memory Optimization**
```toml
[performance.memory]
max_memory = 8589934592  # 8GB
pool_size = 2147483648   # 2GB
enable_memory_mapping = true
gc_target_percent = 10
```

#### **I/O Optimization**
```toml
[performance.io]
max_concurrent_operations = 500
buffer_size = 131072  # 128KB
enable_zero_copy = true
use_direct_io = true
```

---

## 📋 **MONITORING CHECKLIST**

### **Pre-Production**
- [ ] Health endpoints responding correctly
- [ ] Prometheus metrics collection configured
- [ ] Grafana dashboards imported and functional
- [ ] Alert rules configured and tested
- [ ] Log aggregation pipeline working
- [ ] Alertmanager notifications configured

### **Production Deployment**
- [ ] All monitoring services running
- [ ] Dashboards showing expected data
- [ ] Alert thresholds appropriate for environment
- [ ] Log retention policies configured
- [ ] Backup monitoring in place
- [ ] Runbooks documented and accessible

### **Ongoing Operations**
- [ ] Regular review of alert thresholds
- [ ] Dashboard updates for new features
- [ ] Log retention cleanup
- [ ] Performance baseline updates
- [ ] Capacity planning based on trends
- [ ] Alert fatigue management

---

## 🎯 **MONITORING BEST PRACTICES**

### **Metrics Strategy**
1. **Use the Four Golden Signals**:
   - **Latency**: Request response times
   - **Traffic**: Requests per second
   - **Errors**: Error rate percentage
   - **Saturation**: Resource utilization

2. **Implement SLIs/SLOs**:
   - Define Service Level Indicators
   - Set Service Level Objectives
   - Monitor error budgets

3. **Monitor Business Metrics**:
   - Files stored per day
   - Storage tier utilization
   - Compression effectiveness
   - User activity patterns

### **Alerting Strategy**
1. **Alert on Symptoms, Not Causes**:
   - Focus on user-impacting issues
   - Avoid noisy infrastructure alerts
   - Use severity levels appropriately

2. **Implement Alert Hierarchy**:
   - **Critical**: Immediate response required
   - **Warning**: Investigation needed
   - **Info**: Awareness only

3. **Reduce Alert Fatigue**:
   - Tune thresholds based on baselines
   - Use alert suppression for maintenance
   - Regular review and cleanup

### **Dashboard Design**
1. **Audience-Specific Dashboards**:
   - **Executive**: High-level KPIs
   - **Operations**: System health and performance
   - **Development**: Application-specific metrics

2. **Effective Visualization**:
   - Use appropriate chart types
   - Include context and baselines
   - Highlight anomalies clearly

---

## 📞 **SUPPORT RESOURCES**

### **Documentation Links**
- **Prometheus Configuration**: [Official Documentation](https://prometheus.io/docs/)
- **Grafana Dashboards**: [Dashboard Library](https://grafana.com/grafana/dashboards/)
- **ZFS Monitoring**: [OpenZFS Documentation](https://openzfs.org/)

### **Community Resources**
- **NestGate Monitoring**: GitHub repository monitoring examples
- **Ecosystem Integration**: Universal adapter monitoring patterns
- **Best Practices**: Production deployment guides

### **Emergency Contacts**
- **On-Call Engineer**: [Your on-call system]
- **Platform Team**: [Your platform team contact]
- **Vendor Support**: [Vendor support if applicable]

---

**📊 Comprehensive Monitoring Deployed**  
**🔍 Full Observability Enabled**  
**🚨 Production-Ready Alerting**  
**📈 Performance Optimization Ready**

**Your NestGate v2.0 monitoring stack is production-ready!** 