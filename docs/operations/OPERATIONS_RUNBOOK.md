> **Historical**: This document was written in December 1, 2025. Current architecture
> and patterns may differ. See root-level docs and `specs/` for current specifications.

# NestGate Operations Runbook

**Version**: 4.7.0-dev  
**Grade**: A- (92/100)  
**Status**: **PRODUCTION-READY**  
**Last Updated**: March 29, 2026

---

## Table of Contents

1. [System Overview](#system-overview)
2. [Normal Operations](#normal-operations)
3. [Monitoring](#monitoring)
4. [Incident Response](#incident-response)
5. [Common Tasks](#common-tasks)
6. [Emergency Procedures](#emergency-procedures)
7. [Escalation](#escalation)

---

## System Overview

### Architecture
- **Service**: NestGate API Server
- **Port**: 8080 (configurable)
- **Language**: Rust (stable)
- **Runtime**: Native binary (no VM/interpreter)
- **Storage**: Universal adapter (ZFS, S3, GCS, Azure, MinIO)

### Key Metrics
- **Tests**: 8,177+ lib tests (100% pass rate)
- **Test Files**: 273 total, 53 comprehensive
- **Coverage**: 77.1% line (target 90%)
- **Build Time**: ~14s (release)
- **Binary Size**: ~4.7MB (musl static, optimized)

### Dependencies
- Tokio (async runtime)
- Axum (HTTP framework)
- Serde (serialization)
- ZFS (optional, for ZFS storage)

---

## Normal Operations

### Daily Checks (5 minutes)

```bash
#!/bin/bash
# Daily health check script

echo "=== NestGate Daily Check ==="
echo ""

# 1. Service status
echo "1. Service Status:"
systemctl status nestgate || docker ps | grep nestgate

# 2. Health endpoint
echo "2. Health Check:"
curl -s http://localhost:8080/health | jq .

# 3. Error logs (last hour)
echo "3. Recent Errors:"
journalctl -u nestgate --since "1 hour ago" | grep -i error | tail -10

# 4. Resource usage
echo "4. Resource Usage:"
ps aux | grep nestgate | grep -v grep

# 5. Disk space
echo "5. Disk Space:"
df -h /data/nestgate

echo ""
echo "=== Daily Check Complete ==="
```

### Weekly Tasks (30 minutes)

1. **Review Metrics**:
```bash
# Check Prometheus/Grafana dashboards
# Look for trends in:
# - Request rates
# - Error rates
# - Response times
# - Resource usage
```

2. **Log Analysis**:
```bash
# Analyze logs for patterns
journalctl -u nestgate --since "1 week ago" | \
  grep -i error | \
  sort | uniq -c | sort -rn | head -20
```

3. **Backup Verification**:
```bash
# Verify backups are running
ls -lh /backups/nestgate/ | tail -10

# Test a restore (in staging)
# ... restore procedure ...
```

4. **Security Updates**:
```bash
# Check for Rust updates
rustup update

# Check for dependency updates
cargo outdated

# Review security advisories
cargo audit
```

### Monthly Tasks (2-4 hours)

1. **Performance Review**:
   - Analyze response time percentiles
   - Review resource usage trends
   - Identify optimization opportunities

2. **Security Audit**:
   - Review access logs
   - Check for unusual patterns
   - Update security policies
   - Rotate credentials

3. **Capacity Planning**:
   - Project growth trends
   - Plan infrastructure scaling
   - Update resource allocations

4. **Dependency Updates**:
```bash
# Update dependencies
cargo update

# Run full test suite
cargo test --lib

# Deploy to staging
# ... deployment procedure ...

# Monitor for issues
# ... monitoring procedure ...

# Deploy to production
# ... deployment procedure ...
```

---

## Monitoring

### Key Metrics to Watch

1. **Service Health**:
   - OK: HTTP 200 on `/health` endpoint
   - Warning: Response time > 500ms
   - Critical: HTTP 500/503 errors

2. **Resource Usage**:
   - OK: CPU < 70%
   - Warning: CPU 70-90%
   - Critical: CPU > 90%
   - OK: Memory < 80%
   - Warning: Memory 80-95%
   - Critical: Memory > 95%

3. **Error Rates**:
   - OK: Error rate < 0.1%
   - Warning: Error rate 0.1-1%
   - Critical: Error rate > 1%

4. **Disk Space**:
   - OK: Usage < 70%
   - Warning: Usage 70-90%
   - Critical: Usage > 90%

### Alert Thresholds

```yaml
# Example Prometheus alert rules
groups:
  - name: nestgate_critical
    interval: 30s
    rules:
      - alert: NestGateDown
        expr: up{job="nestgate"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "NestGate is down"
          description: "Service has been down for 1 minute"
      
      - alert: HighErrorRate
        expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.01
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High error rate detected"
          description: "Error rate is {{ $value }} per second"
      
      - alert: HighResponseTime
        expr: histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m])) > 0.5
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "High response time"
          description: "95th percentile is {{ $value }}s"
      
      - alert: HighMemoryUsage
        expr: process_resident_memory_bytes / node_memory_MemTotal_bytes > 0.9
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High memory usage"
          description: "Memory usage is {{ $value | humanizePercentage }}"
```

---

## Incident Response

### Severity Levels

**P0 - Critical (Immediate Response)**:
- Service completely down
- Data loss occurring
- Security breach

**P1 - High (< 15 minutes)**:
- Degraded performance (>5x normal)
- Elevated error rates (>5%)
- Critical feature unavailable

**P2 - Medium (< 1 hour)**:
- Minor performance degradation
- Non-critical errors
- Monitoring alerts

**P3 - Low (< 1 day)**:
- Cosmetic issues
- Documentation needed
- Minor improvements

### Response Procedures

#### P0: Service Down

1. **Immediate Actions** (0-5 minutes):
```bash
# Check service status
systemctl status nestgate

# Check if port is listening
netstat -tulpn | grep 8080

# Check recent logs
journalctl -u nestgate -n 100 --no-pager

# Attempt restart
systemctl restart nestgate

# Verify health
curl http://localhost:8080/health
```

2. **If Restart Fails** (5-15 minutes):
```bash
# Check disk space
df -h

# Check memory
free -h

# Check for hung processes
ps aux | grep nestgate

# Check system logs
dmesg | tail -50

# Check file permissions
ls -la /data/nestgate

# Try manual start with verbose logging
RUST_LOG=debug ./target/release/nestgate-api-server
```

3. **Escalation** (15+ minutes):
   - Notify on-call engineer
   - Create incident ticket
   - Roll back recent changes if applicable
   - Consider failover to backup instance

#### P1: High Error Rate

1. **Investigate** (0-5 minutes):
```bash
# Check error logs
journalctl -u nestgate --since "15 minutes ago" | grep -i error

# Check metrics
curl http://localhost:8080/metrics | grep error

# Check specific endpoint
curl -v http://localhost:8080/api/problem-endpoint
```

2. **Mitigate** (5-15 minutes):
```bash
# If database issue, check connections
# If API issue, check external dependencies
# If resource issue, scale up or restart

# Temporary fixes:
# - Rate limiting
# - Circuit breakers
# - Graceful degradation
```

3. **Document**:
   - What happened
   - What was affected
   - How it was fixed
   - How to prevent recurrence

#### P2: Performance Degradation

1. **Baseline**:
```bash
# Measure current performance
ab -n 1000 -c 10 http://localhost:8080/health

# Compare to normal baseline
# Normal: ~100 req/sec, 10ms avg
```

2. **Profile**:
```bash
# CPU profiling
perf record -g -p $(pgrep nestgate)
perf report

# Memory profiling
# (requires heaptrack or similar)
```

3. **Optimize**:
   - Identify bottlenecks
   - Apply targeted fixes
   - Verify improvements
   - Document findings

---

## Common Tasks

### Viewing Logs

```bash
# Real-time logs
journalctl -u nestgate -f

# Last hour
journalctl -u nestgate --since "1 hour ago"

# Specific date
journalctl -u nestgate --since "2025-12-01" --until "2025-12-02"

# Error logs only
journalctl -u nestgate -p err

# JSON output
journalctl -u nestgate -o json-pretty
```

### Configuration Changes

```bash
# 1. Edit configuration
vim /etc/nestgate/config.toml

# 2. Validate syntax
cargo run --release -- --config /etc/nestgate/config.toml --validate

# 3. Restart service
systemctl restart nestgate

# 4. Verify
curl http://localhost:8080/health

# 5. Monitor logs
journalctl -u nestgate -f
```

### Scaling

```bash
# Vertical scaling (increase resources)
# 1. Stop service
systemctl stop nestgate

# 2. Adjust limits in service file
vim /etc/systemd/system/nestgate.service

# 3. Reload systemd
systemctl daemon-reload

# 4. Start service
systemctl start nestgate

# Horizontal scaling (add instances)
# (Depends on orchestration - K8s, Docker Swarm, etc.)
kubectl scale deployment nestgate --replicas=5
```

### Backup & Restore

```bash
# Manual backup
tar -czf nestgate-backup-$(date +%Y%m%d-%H%M%S).tar.gz \
  /etc/nestgate/ \
  /data/nestgate/

# Automated backup (add to cron)
0 2 * * * /usr/local/bin/nestgate-backup.sh

# Restore
tar -xzf nestgate-backup-20251201-020000.tar.gz -C /
systemctl restart nestgate
```

---

## Emergency Procedures

### Data Loss Prevention

**If data corruption detected**:

1. **STOP** - Immediately stop the service
2. **ISOLATE** - Prevent further writes
3. **SNAPSHOT** - Capture current state
4. **ASSESS** - Determine extent of damage
5. **RESTORE** - From most recent good backup
6. **VERIFY** - Ensure data integrity
7. **RESUME** - Restart service with monitoring

### Security Breach Response

**If breach suspected**:

1. **CONTAIN** - Isolate affected systems
2. **ASSESS** - Determine scope and impact
3. **ERADICATE** - Remove threat
4. **RECOVER** - Restore to known good state
5. **LESSONS** - Document and improve

### Complete System Failure

**If everything is broken**:

1. **Deploy fresh instance** from known-good build
2. **Restore configuration** from backup
3. **Restore data** from backup
4. **Verify integrity** of restored data
5. **Point traffic** to new instance
6. **Investigate** root cause offline

---

## Escalation

### On-Call Rotation

- **Primary**: Check internal schedule
- **Secondary**: Check internal schedule
- **Manager**: Check internal contacts

### Contact Methods

1. **PagerDuty**: High-priority alerts
2. **Slack**: #nestgate-ops channel
3. **Email**: ops@company.com (non-urgent)
4. **Phone**: Emergency only

### Escalation Criteria

**Escalate immediately if**:
- Service down >15 minutes
- Data loss occurring
- Security breach
- Unable to diagnose
- Multiple systems affected

**Escalate after investigation if**:
- Complex issue requiring expertise
- Needs management decision
- Requires vendor support
- Potential legal implications

---

## Appendix

### Quick Reference

```bash
# Start service
systemctl start nestgate

# Stop service
systemctl stop nestgate

# Restart service
systemctl restart nestgate

# Check status
systemctl status nestgate

# View logs
journalctl -u nestgate -f

# Check health
curl http://localhost:8080/health

# Check metrics
curl http://localhost:8080/metrics

# Run tests
cd /opt/nestgate && cargo test --lib

# Build release
cd /opt/nestgate && cargo build --release

# Check version
./target/release/nestgate-api-server --version
```

### Service Files

**Systemd** (`/etc/systemd/system/nestgate.service`):
```ini
[Unit]
Description=NestGate API Server
After=network.target

[Service]
Type=simple
User=nestgate
Group=nestgate
WorkingDirectory=/opt/nestgate
ExecStart=/opt/nestgate/target/release/nestgate-api-server
Restart=on-failure
RestartSec=10
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
```

### Useful Commands

```bash
# Find largest log files
du -sh /var/log/nestgate/* | sort -rh | head -10

# Count errors by type
grep -o 'ERROR.*' /var/log/nestgate/nestgate.log | \
  sort | uniq -c | sort -rn

# Monitor resource usage
watch -n 1 'ps aux | grep nestgate | grep -v grep'

# Network connections
netstat -an | grep :8080 | wc -l

# Find core dumps
find /var/crash -name "nestgate.*" -mtime -7
```

---

## Notes

- This runbook should be updated as procedures change
- Always test procedures in staging before production
- Document all incidents for future reference
- Keep contact information current

**Last Review**: April 3, 2026  
**Next Review**: July 1, 2026

---

*NestGate 4.7.0-dev - Production-Ready A- (92/100)*

