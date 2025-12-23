# Demo 1.4: Health Monitoring

**Level**: 1 (Isolated)  
**Time**: 5 minutes  
**Complexity**: Beginner  
**Status**: 🆕 New demo

---

## 🎯 WHAT THIS DEMO SHOWS

This demo demonstrates NestGate's health monitoring and observability features:

1. System health checks
2. Storage metrics and monitoring
3. Performance metrics
4. Resource usage tracking
5. Real-time status reporting

**Key Concept**: Observability is critical for production systems - know your system's state!

---

## 🚀 QUICK RUN

```bash
# Make sure NestGate is running
../../scripts/start_data_service.sh

# Run the demo
./demo.sh

# Expected runtime: ~1 minute
```

---

## 📋 WHAT YOU'LL SEE

### Part 1: Basic Health Check
```
GET /health

Response:
{
  "status": "healthy",
  "uptime_seconds": 3600,
  "version": "0.1.0",
  "timestamp": "2025-12-17T20:30:00Z"
}
```

### Part 2: Detailed System Status
```
GET /api/v1/status

Response:
{
  "storage": {
    "backend": "zfs",
    "available": true,
    "capacity_gb": 500,
    "used_gb": 125,
    "free_gb": 375,
    "usage_percent": 25
  },
  "api": {
    "requests_total": 1234,
    "requests_success": 1229,
    "requests_error": 5,
    "error_rate": 0.4
  },
  "system": {
    "cpu_percent": 12.5,
    "memory_mb": 1024,
    "memory_total_mb": 8192,
    "memory_percent": 12.5
  }
}
```

### Part 3: Storage Metrics
```
GET /api/v1/metrics/storage

Response:
{
  "datasets": 15,
  "total_objects": 1234,
  "total_size_gb": 45.2,
  "operations": {
    "read": 8234,
    "write": 1234,
    "delete": 45
  },
  "performance": {
    "avg_read_ms": 2.5,
    "avg_write_ms": 5.2,
    "p95_read_ms": 8.1,
    "p95_write_ms": 15.3
  }
}
```

### Part 4: Real-Time Monitoring
```
Watching metrics in real-time...

[00:00] CPU: 12.5% | Memory: 1024MB | Requests: 1234
[00:05] CPU: 15.2% | Memory: 1056MB | Requests: 1289
[00:10] CPU: 11.8% | Memory: 1048MB | Requests: 1345
```

---

## 💡 WHY HEALTH MONITORING MATTERS

### Production Requirements:
1. **Uptime Tracking** - Know when service is available
2. **Performance Monitoring** - Detect slowdowns early
3. **Resource Management** - Prevent resource exhaustion
4. **Error Detection** - Catch issues before users do
5. **Capacity Planning** - Know when to scale

### Health Check Levels:

**Basic Health** (`/health`):
- Simple yes/no: Is service alive?
- Fast response (<10ms)
- Used by load balancers

**Detailed Status** (`/api/v1/status`):
- Component-level health
- Resource utilization
- Slower but comprehensive

**Metrics** (`/api/v1/metrics/*`):
- Historical data
- Performance trends
- Deep insights

---

## 🔍 MONITORING BEST PRACTICES

### 1. Health Endpoint Design
```
Good Health Endpoint:
✓ Fast (<10ms)
✓ No heavy operations
✓ Returns 200 or 503
✓ Includes version

Bad Health Endpoint:
✗ Queries database
✗ Complex calculations
✗ Returns 500 on any error
✗ Missing version info
```

### 2. Metric Collection
```
Key Metrics to Track:
- Request rate (requests/sec)
- Error rate (errors/requests)
- Latency (p50, p95, p99)
- Resource usage (CPU, memory, disk)
- Storage capacity
```

### 3. Alert Thresholds
```
Typical Thresholds:
- Error rate > 5%: WARNING
- Error rate > 10%: CRITICAL
- Latency p95 > 500ms: WARNING
- Latency p95 > 1000ms: CRITICAL
- Disk usage > 80%: WARNING
- Disk usage > 90%: CRITICAL
```

---

## 🧪 EXPERIMENTS TO TRY

### Experiment 1: Load Testing
```bash
# Generate load
for i in {1..100}; do
  curl -X POST ${NESTGATE_URL}/api/v1/datasets/load-test/objects/obj${i} \
    -d "data ${i}" &
done
wait

# Check metrics impact
curl ${NESTGATE_URL}/api/v1/metrics/storage
# Should see increased request count and latency
```

### Experiment 2: Resource Monitoring
```bash
# Watch metrics in real-time
watch -n 5 "curl -s ${NESTGATE_URL}/api/v1/status | jq '.system'"

# In another terminal, generate activity
while true; do
  curl -X PUT ${NESTGATE_URL}/api/v1/datasets/test/objects/data \
    -d "$(date)"
  sleep 1
done
```

### Experiment 3: Error Rate Tracking
```bash
# Generate some errors (invalid requests)
for i in {1..20}; do
  curl -X POST ${NESTGATE_URL}/invalid/endpoint &
done
wait

# Check error rate
curl ${NESTGATE_URL}/api/v1/metrics/api | jq '.error_rate'
```

---

## 📊 METRICS DEEP DIVE

### Storage Metrics
```
What to Monitor:
- Total datasets
- Total objects
- Total size (GB)
- Growth rate (GB/day)
- Operations per second
- Operation latency
```

### API Metrics
```
What to Monitor:
- Request rate (req/sec)
- Success rate (%)
- Error rate (%)
- Latency distribution (p50, p95, p99)
- Endpoint-specific metrics
```

### System Metrics
```
What to Monitor:
- CPU utilization (%)
- Memory usage (MB)
- Disk I/O (MB/s)
- Network I/O (MB/s)
- Open file descriptors
```

---

## 🛠️ MONITORING STACK INTEGRATION

### Prometheus Integration
```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'nestgate'
    static_configs:
      - targets: ['localhost:9090']
    metrics_path: '/metrics'
```

### Grafana Dashboard
```
Typical Panels:
1. Request Rate (line graph)
2. Error Rate (line graph)
3. Latency p95 (line graph)
4. Storage Capacity (gauge)
5. Active Datasets (counter)
6. System Resources (multi-line graph)
```

### Alert Manager
```yaml
# alerts.yml
groups:
  - name: nestgate
    rules:
      - alert: HighErrorRate
        expr: error_rate > 0.05
        for: 5m
        annotations:
          summary: "High error rate detected"
      
      - alert: DiskSpaceLow
        expr: disk_usage_percent > 80
        for: 10m
        annotations:
          summary: "Disk space running low"
```

---

## 🆘 TROUBLESHOOTING

### "Health endpoint returns 503"
**Cause**: Service degraded or starting up  
**Check**:
```bash
# Check logs
journalctl -u nestgate -f

# Check system resources
top
df -h

# Check dependencies
systemctl status nestgate
```

### "Metrics show high latency"
**Cause**: System under load or resource constrained  
**Check**:
```bash
# Check system load
uptime

# Check disk I/O
iostat -x 1

# Check for slow queries
curl ${NESTGATE_URL}/api/v1/metrics/slow-operations
```

### "Memory usage growing"
**Cause**: Possible memory leak or cache growth  
**Check**:
```bash
# Monitor memory over time
watch -n 10 "curl -s ${NESTGATE_URL}/api/v1/status | jq '.system.memory_mb'"

# Check for leaks
valgrind --leak-check=full ./nestgate
```

---

## 📈 DASHBOARD DESIGN

### Essential Dashboard Sections:

**1. Overview (Top)**
- Current status: Healthy/Degraded/Down
- Uptime percentage
- Request rate (current)
- Error rate (current)

**2. Performance (Middle)**
- Latency graphs (p50, p95, p99)
- Request rate over time
- Error rate over time
- Operation success rate

**3. Resources (Bottom)**
- CPU usage
- Memory usage
- Disk usage
- Storage capacity

**4. Alerts (Sidebar)**
- Active alerts
- Recent alerts
- Alert history

---

## 📚 LEARN MORE

**Concepts**:
- **Observability**: `../../../docs/MONITORING_GUIDE.md`
- **Metrics**: `../../../specs/PRODUCTION_READINESS_ROADMAP.md`
- **Health Checks**: `../../../docs/DEPLOYMENT_GUIDE.md`

**Implementation**:
- Health checks: `../../../code/crates/nestgate-api/src/handlers/health.rs`
- Metrics: `../../../code/crates/nestgate-core/src/observability/metrics.rs`
- Monitoring: `../../../code/crates/nestgate-core/src/monitoring/`

---

## ⏭️ NEXT DEMO

**Demo 1.5: ZFS Advanced** (`../05_zfs_advanced/`)
- Learn advanced ZFS features
- See snapshots in action
- Understand compression and deduplication

---

**Status**: 🆕 New demo  
**Estimated time**: 5 minutes  
**Prerequisites**: Completed Demos 1.1-1.3

**Key Takeaway**: Monitor everything! You can't fix what you can't measure. 📊

---

*Demo 1.4 - Health Monitoring*  
*Part of Level 1: Isolated Instance*

