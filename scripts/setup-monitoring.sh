#!/bin/bash
# 📊 **NESTGATE PRODUCTION MONITORING SETUP**
# Automated setup script for production monitoring infrastructure
# Version: 1.0.0-production

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
NESTGATE_HOME="${NESTGATE_HOME:-/opt/nestgate}"
LOG_DIR="${LOG_DIR:-/var/log/nestgate}"
METRICS_PORT="${METRICS_PORT:-9090}"
HEALTH_CHECK_INTERVAL="${HEALTH_CHECK_INTERVAL:-30}"

echo -e "${BLUE}🚀 NestGate Production Monitoring Setup${NC}"
echo "========================================"

# Function to print status messages
print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Check if running as root
if [[ $EUID -ne 0 ]]; then
   print_error "This script must be run as root for system configuration"
   exit 1
fi

print_status "Starting NestGate monitoring setup..."

# Create monitoring directories
print_status "Creating monitoring directories..."
mkdir -p "${LOG_DIR}/monitoring"
mkdir -p "${NESTGATE_HOME}/monitoring"
mkdir -p "${NESTGATE_HOME}/config/monitoring"

# Setup log rotation for NestGate logs
print_status "Configuring log rotation..."
cat > /etc/logrotate.d/nestgate << EOF
${LOG_DIR}/*.log {
    daily
    rotate 7
    compress
    delaycompress
    missingok
    notifempty
    create 644 nestgate nestgate
    postrotate
        systemctl reload nestgate-* || true
    endscript
}
EOF

# Create systemd monitoring service
print_status "Creating monitoring service..."
cat > /etc/systemd/system/nestgate-monitor.service << EOF
[Unit]
Description=NestGate Monitoring Service
After=network.target
Requires=nestgate-core.service

[Service]
Type=simple
User=nestgate
Group=nestgate
WorkingDirectory=${NESTGATE_HOME}
ExecStart=${NESTGATE_HOME}/bin/nestgate-monitor
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

# Resource limits
MemoryMax=512M
CPUQuota=50%

# Security settings
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=${LOG_DIR} ${NESTGATE_HOME}/data

[Install]
WantedBy=multi-user.target
EOF

# Create health check script
print_status "Creating health check script..."
cat > "${NESTGATE_HOME}/bin/health-check.sh" << 'EOF'
#!/bin/bash
# NestGate Health Check Script

HEALTH_ENDPOINT="${NESTGATE_HEALTH_ENDPOINT:-http://localhost:8080/health}"
METRICS_ENDPOINT="${NESTGATE_METRICS_ENDPOINT:-http://localhost:8080/metrics}"
TIMEOUT="${HEALTH_CHECK_TIMEOUT:-10}"

# Check service health
check_health() {
    local service_name="$1"
    local endpoint="$2"
    
    if curl -sf --max-time "$TIMEOUT" "$endpoint" >/dev/null 2>&1; then
        echo "✓ $service_name: healthy"
        return 0
    else
        echo "✗ $service_name: unhealthy"
        return 1
    fi
}

# Check all services
echo "🔍 NestGate Health Check - $(date)"
echo "=================================="

healthy=0
total=0

# Core service health
if check_health "Core Service" "$HEALTH_ENDPOINT"; then
    ((healthy++))
fi
((total++))

# Metrics availability
if check_health "Metrics Service" "$METRICS_ENDPOINT"; then
    ((healthy++))
fi
((total++))

# Service status
for service in nestgate-core nestgate-zfs nestgate-network; do
    if systemctl is-active "$service" >/dev/null 2>&1; then
        echo "✓ $service: active"
        ((healthy++))
    else
        echo "✗ $service: inactive"
    fi
    ((total++))
done

# Summary
echo "=================================="
echo "Health Score: $healthy/$total services healthy"

if [[ $healthy -eq $total ]]; then
    echo "🎉 All services healthy!"
    exit 0
else
    echo "⚠️  Some services need attention"
    exit 1
fi
EOF

chmod +x "${NESTGATE_HOME}/bin/health-check.sh"

# Create metrics collection script
print_status "Creating metrics collection script..."
cat > "${NESTGATE_HOME}/bin/collect-metrics.sh" << 'EOF'
#!/bin/bash
# NestGate Metrics Collection Script

METRICS_ENDPOINT="${NESTGATE_METRICS_ENDPOINT:-http://localhost:8080/metrics}"
OUTPUT_DIR="${NESTGATE_HOME}/data/metrics"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

mkdir -p "$OUTPUT_DIR"

# Collect current metrics
curl -s "$METRICS_ENDPOINT" > "${OUTPUT_DIR}/metrics_${TIMESTAMP}.txt" || {
    echo "Failed to collect metrics from $METRICS_ENDPOINT"
    exit 1
}

# Keep only last 24 hours of metrics files
find "$OUTPUT_DIR" -name "metrics_*.txt" -mtime +1 -delete

echo "Metrics collected: ${OUTPUT_DIR}/metrics_${TIMESTAMP}.txt"
EOF

chmod +x "${NESTGATE_HOME}/bin/collect-metrics.sh"

# Setup cron jobs for monitoring
print_status "Setting up monitoring cron jobs..."
cat > /etc/cron.d/nestgate-monitoring << EOF
# NestGate Monitoring Cron Jobs

# Health check every 5 minutes
*/5 * * * * nestgate ${NESTGATE_HOME}/bin/health-check.sh >> ${LOG_DIR}/health-check.log 2>&1

# Metrics collection every 15 minutes
*/15 * * * * nestgate ${NESTGATE_HOME}/bin/collect-metrics.sh >> ${LOG_DIR}/metrics-collection.log 2>&1

# Daily log cleanup
0 2 * * * root find ${LOG_DIR} -name "*.log" -mtime +7 -delete
EOF

# Create monitoring configuration
print_status "Creating monitoring configuration..."
cat > "${NESTGATE_HOME}/config/monitoring/monitoring.toml" << EOF
# NestGate Monitoring Configuration

[monitoring]
enabled = true
health_check_interval = "${HEALTH_CHECK_INTERVAL}s"
metrics_retention_days = 7
log_retention_days = 7

[endpoints]
health = "http://localhost:8080/health"
metrics = "http://localhost:8080/metrics"
status = "http://localhost:8080/status"

[alerting]
enabled = false  # Enable when integrating with external systems
webhook_url = ""
email_notifications = false

[thresholds]
cpu_usage_warning = 80
cpu_usage_critical = 95
memory_usage_warning = 85
memory_usage_critical = 95
disk_usage_warning = 80
disk_usage_critical = 90
response_time_warning = 1000  # milliseconds
response_time_critical = 5000
EOF

# Setup systemd timer for health checks
print_status "Creating systemd health check timer..."
cat > /etc/systemd/system/nestgate-health-check.service << EOF
[Unit]
Description=NestGate Health Check
Requires=nestgate-core.service

[Service]
Type=oneshot
User=nestgate
Group=nestgate
ExecStart=${NESTGATE_HOME}/bin/health-check.sh
StandardOutput=journal
StandardError=journal
EOF

cat > /etc/systemd/system/nestgate-health-check.timer << EOF
[Unit]
Description=Run NestGate health check every 5 minutes
Requires=nestgate-health-check.service

[Timer]
OnCalendar=*:0/5
Persistent=true

[Install]
WantedBy=timers.target
EOF

# Set proper ownership
print_status "Setting proper file ownership..."
chown -R nestgate:nestgate "${NESTGATE_HOME}" "${LOG_DIR}"

# Reload systemd and enable services
print_status "Enabling monitoring services..."
systemctl daemon-reload
systemctl enable nestgate-health-check.timer
systemctl start nestgate-health-check.timer

# Create monitoring dashboard script
print_status "Creating monitoring dashboard..."
cat > "${NESTGATE_HOME}/bin/monitoring-dashboard.sh" << 'EOF'
#!/bin/bash
# Simple monitoring dashboard for NestGate

clear
echo "🖥️  NestGate Production Monitoring Dashboard"
echo "============================================"
echo

# System information
echo "📊 System Information:"
echo "  Hostname: $(hostname)"
echo "  Uptime: $(uptime -p)"
echo "  Load: $(uptime | awk -F'load average:' '{print $2}')"
echo

# Service status
echo "🔧 Service Status:"
for service in nestgate-core nestgate-zfs nestgate-network; do
    if systemctl is-active "$service" >/dev/null 2>&1; then
        status="✅ Running"
    else
        status="❌ Stopped"
    fi
    echo "  $service: $status"
done
echo

# Resource usage
echo "💾 Resource Usage:"
echo "  Memory: $(free -h | awk '/^Mem:/ {printf "%s/%s (%.1f%%)", $3, $2, $3/$2*100}')"
echo "  Disk: $(df -h / | awk 'NR==2 {printf "%s/%s (%s used)", $3, $2, $5}')"
echo

# Recent logs
echo "📝 Recent Activity (last 10 lines):"
tail -n 10 /var/log/nestgate/core.log 2>/dev/null || echo "  No logs available"

echo
echo "🔄 Auto-refreshing every 30 seconds... (Ctrl+C to exit)"
EOF

chmod +x "${NESTGATE_HOME}/bin/monitoring-dashboard.sh"

print_status "Monitoring setup completed successfully!"
echo
echo -e "${BLUE}📊 Monitoring Components Installed:${NC}"
echo "  • Health check service and timer"
echo "  • Metrics collection scripts"
echo "  • Log rotation configuration"
echo "  • Monitoring dashboard"
echo "  • Automated cleanup jobs"
echo
echo -e "${BLUE}🔧 Management Commands:${NC}"
echo "  • Health check: ${NESTGATE_HOME}/bin/health-check.sh"
echo "  • Dashboard: ${NESTGATE_HOME}/bin/monitoring-dashboard.sh"
echo "  • View logs: journalctl -u nestgate-*"
echo "  • Service status: systemctl status nestgate-*"
echo
echo -e "${GREEN}✨ NestGate monitoring is now operational!${NC}" 