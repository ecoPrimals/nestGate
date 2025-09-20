#!/bin/bash
# NestGate Production Monitoring and Optimization Script
# Comprehensive system monitoring, performance optimization, and health checks

set -euo pipefail

# Configuration
NESTGATE_HOME="/opt/nestgate"
LOG_DIR="${NESTGATE_HOME}/logs"
CONFIG_DIR="${NESTGATE_HOME}/config"
METRICS_DIR="${NESTGATE_HOME}/metrics"
BACKUP_DIR="/backup/nestgate"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging function
log() {
    echo -e "${BLUE}[$(date '+%Y-%m-%d %H:%M:%S')]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Create directories if they don't exist
mkdir -p "$METRICS_DIR" "$BACKUP_DIR"

# System Health Check Function
system_health_check() {
    log "🔍 Performing comprehensive system health check..."
    
    local health_score=100
    local issues=()
    
    # Check CPU usage
    local cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1)
    if (( $(echo "$cpu_usage > 80" | bc -l) )); then
        warning "High CPU usage: ${cpu_usage}%"
        issues+=("High CPU usage")
        ((health_score-=10))
    fi
    
    # Check memory usage
    local mem_usage=$(free | grep Mem | awk '{printf("%.1f"), $3/$2 * 100.0}')
    if (( $(echo "$mem_usage > 85" | bc -l) )); then
        warning "High memory usage: ${mem_usage}%"
        issues+=("High memory usage")
        ((health_score-=10))
    fi
    
    # Check disk usage
    local disk_usage=$(df -h "${NESTGATE_HOME}" | awk 'NR==2 {print $5}' | cut -d'%' -f1)
    if [ "$disk_usage" -gt 90 ]; then
        warning "High disk usage: ${disk_usage}%"
        issues+=("High disk usage")
        ((health_score-=15))
    fi
    
    # Check NestGate service status
    if ! systemctl is-active --quiet nestgate; then
        error "NestGate service is not running"
        issues+=("Service not running")
        ((health_score-=30))
    fi
    
    # Check network connectivity
    if ! curl -f -s http://localhost:8080/health > /dev/null 2>&1; then
        warning "Health endpoint not responding"
        issues+=("Health endpoint unreachable")
        ((health_score-=20))
    fi
    
    # Report health status
    if [ ${#issues[@]} -eq 0 ]; then
        success "✅ System health: EXCELLENT (${health_score}/100)"
    else
        warning "⚠️  System health: ${health_score}/100 - Issues found:"
        printf '%s\n' "${issues[@]}" | sed 's/^/  - /'
    fi
    
    # Write health metrics
    cat > "${METRICS_DIR}/health_metrics.json" <<EOF
{
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "health_score": ${health_score},
    "cpu_usage": ${cpu_usage},
    "memory_usage": ${mem_usage},
    "disk_usage": ${disk_usage},
    "service_status": "$(systemctl is-active nestgate)",
    "issues": [$(printf '"%s",' "${issues[@]}" | sed 's/,$//')],
    "uptime": "$(uptime -p)"
}
EOF
}

# Performance Optimization Function
performance_optimization() {
    log "⚡ Running performance optimization..."
    
    # Optimize system parameters
    log "Optimizing system parameters..."
    
    # Network optimizations
    if [ -w /proc/sys/net/core/rmem_max ]; then
        echo 16777216 > /proc/sys/net/core/rmem_max
        echo 16777216 > /proc/sys/net/core/wmem_max
    fi
    
    # File descriptor limits
    if [ -w /proc/sys/fs/file-max ]; then
        echo 2097152 > /proc/sys/fs/file-max
    fi
    
    # Memory management
    if [ -w /proc/sys/vm/swappiness ]; then
        echo 10 > /proc/sys/vm/swappiness
        echo 15 > /proc/sys/vm/dirty_ratio
        echo 5 > /proc/sys/vm/dirty_background_ratio
    fi
    
    # ZFS optimizations (if ZFS is available)
    if command -v zfs &> /dev/null; then
        log "Optimizing ZFS parameters..."
        
        # Set ARC max to 8GB if system has enough memory
        local total_mem=$(free -b | awk 'NR==2{print $2}')
        if [ "$total_mem" -gt 17179869184 ]; then  # 16GB
            echo 8589934592 > /sys/module/zfs/parameters/zfs_arc_max 2>/dev/null || true
        fi
        
        # Optimize TXG timeout
        echo 2 > /sys/module/zfs/parameters/zfs_txg_timeout 2>/dev/null || true
    fi
    
    # Clear system caches (if safe to do so)
    if [ "$(free | awk 'NR==2{printf "%.0f", $7/$2*100}')" -gt 50 ]; then
        sync
        echo 1 > /proc/sys/vm/drop_caches
        log "System caches cleared"
    fi
    
    success "Performance optimization completed"
}

# Security Monitoring Function
security_monitoring() {
    log "🛡️  Running security monitoring checks..."
    
    local security_alerts=()
    
    # Check for failed login attempts
    local failed_logins=$(journalctl -u nestgate --since "1 hour ago" | grep -c "authentication_failed" || echo "0")
    if [ "$failed_logins" -gt 10 ]; then
        security_alerts+=("High number of failed logins: $failed_logins")
    fi
    
    # Check for unusual network connections
    local unusual_connections=$(ss -tuln | grep -E ":(8080|9090)" | wc -l)
    if [ "$unusual_connections" -gt 50 ]; then
        security_alerts+=("Unusual number of network connections: $unusual_connections")
    fi
    
    # Check file integrity of critical files
    local critical_files=(
        "${CONFIG_DIR}/production.toml"
        "${NESTGATE_HOME}/bin/nestgate"
        "/etc/systemd/system/nestgate.service"
    )
    
    for file in "${critical_files[@]}"; do
        if [ -f "$file" ]; then
            local current_hash=$(sha256sum "$file" | cut -d' ' -f1)
            local baseline_file="${METRICS_DIR}/baseline_$(basename "$file").sha256"
            
            if [ -f "$baseline_file" ]; then
                local baseline_hash=$(cat "$baseline_file")
                if [ "$current_hash" != "$baseline_hash" ]; then
                    security_alerts+=("File integrity violation: $file")
                fi
            else
                echo "$current_hash" > "$baseline_file"
                log "Baseline hash created for $file"
            fi
        fi
    done
    
    # Report security status
    if [ ${#security_alerts[@]} -eq 0 ]; then
        success "🔒 Security status: ALL CLEAR"
    else
        warning "⚠️  Security alerts detected:"
        printf '%s\n' "${security_alerts[@]}" | sed 's/^/  - /'
        
        # Write security alerts to file
        cat > "${METRICS_DIR}/security_alerts.json" <<EOF
{
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "alerts": [$(printf '"%s",' "${security_alerts[@]}" | sed 's/,$//')],
    "failed_logins": ${failed_logins},
    "network_connections": ${unusual_connections}
}
EOF
    fi
}

# Backup Function
backup_system() {
    log "💾 Running system backup..."
    
    local backup_date=$(date +%Y%m%d_%H%M%S)
    local backup_path="${BACKUP_DIR}/nestgate_backup_${backup_date}"
    
    mkdir -p "$backup_path"
    
    # Backup configuration
    if [ -d "$CONFIG_DIR" ]; then
        cp -r "$CONFIG_DIR" "${backup_path}/config"
        success "Configuration backed up"
    fi
    
    # Backup logs (last 7 days)
    if [ -d "$LOG_DIR" ]; then
        find "$LOG_DIR" -name "*.log" -mtime -7 -exec cp {} "${backup_path}/" \;
        success "Recent logs backed up"
    fi
    
    # Backup metrics
    if [ -d "$METRICS_DIR" ]; then
        cp -r "$METRICS_DIR" "${backup_path}/metrics"
        success "Metrics backed up"
    fi
    
    # Compress backup
    tar -czf "${backup_path}.tar.gz" -C "$BACKUP_DIR" "$(basename "$backup_path")"
    rm -rf "$backup_path"
    
    # Clean old backups (keep 30 days)
    find "$BACKUP_DIR" -name "nestgate_backup_*.tar.gz" -mtime +30 -delete
    
    success "Backup completed: ${backup_path}.tar.gz"
}

# Log Analysis Function
log_analysis() {
    log "📊 Performing log analysis..."
    
    local log_file="${LOG_DIR}/nestgate.log"
    local analysis_file="${METRICS_DIR}/log_analysis.json"
    
    if [ ! -f "$log_file" ]; then
        warning "Log file not found: $log_file"
        return
    fi
    
    # Analyze logs from last hour
    local errors=$(grep -c "ERROR" "$log_file" | tail -1000 || echo "0")
    local warnings=$(grep -c "WARN" "$log_file" | tail -1000 || echo "0")
    local info=$(grep -c "INFO" "$log_file" | tail -1000 || echo "0")
    
    # Performance metrics from logs
    local avg_response_time=$(grep "response_time" "$log_file" | tail -100 | \
        grep -o 'response_time":[0-9.]*' | cut -d':' -f2 | \
        awk '{sum+=$1; count++} END {if(count>0) print sum/count; else print 0}')
    
    # Write analysis results
    cat > "$analysis_file" <<EOF
{
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "log_analysis": {
        "errors": ${errors},
        "warnings": ${warnings},
        "info": ${info},
        "avg_response_time": ${avg_response_time:-0},
        "total_entries": $((errors + warnings + info))
    }
}
EOF
    
    if [ "$errors" -gt 50 ]; then
        warning "High error rate detected: $errors errors in recent logs"
    elif [ "$errors" -gt 10 ]; then
        warning "Moderate error rate: $errors errors in recent logs"
    else
        success "Log analysis: Error rate acceptable ($errors errors)"
    fi
}

# Resource Usage Report
resource_usage_report() {
    log "📈 Generating resource usage report..."
    
    local report_file="${METRICS_DIR}/resource_usage.json"
    
    # CPU information
    local cpu_cores=$(nproc)
    local load_avg=$(uptime | awk '{print $(NF-2)}' | cut -d',' -f1)
    
    # Memory information
    local total_mem=$(free -m | awk 'NR==2{print $2}')
    local used_mem=$(free -m | awk 'NR==2{print $3}')
    local available_mem=$(free -m | awk 'NR==2{print $7}')
    
    # Disk information
    local disk_total=$(df -BG "${NESTGATE_HOME}" | awk 'NR==2 {print $2}' | sed 's/G//')
    local disk_used=$(df -BG "${NESTGATE_HOME}" | awk 'NR==2 {print $3}' | sed 's/G//')
    local disk_available=$(df -BG "${NESTGATE_HOME}" | awk 'NR==2 {print $4}' | sed 's/G//')
    
    # Network statistics
    local network_rx=$(cat /proc/net/dev | grep -E "(eth0|ens|enp)" | head -1 | awk '{print $2}')
    local network_tx=$(cat /proc/net/dev | grep -E "(eth0|ens|enp)" | head -1 | awk '{print $10}')
    
    # NestGate process information
    local nestgate_pid=$(pgrep -f nestgate || echo "0")
    local nestgate_mem="0"
    local nestgate_cpu="0"
    
    if [ "$nestgate_pid" != "0" ]; then
        nestgate_mem=$(ps -p "$nestgate_pid" -o rss= | awk '{print $1/1024}')
        nestgate_cpu=$(ps -p "$nestgate_pid" -o %cpu= | awk '{print $1}')
    fi
    
    # Generate report
    cat > "$report_file" <<EOF
{
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "system": {
        "cpu_cores": ${cpu_cores},
        "load_average": ${load_avg},
        "uptime": "$(uptime -p)"
    },
    "memory": {
        "total_mb": ${total_mem},
        "used_mb": ${used_mem},
        "available_mb": ${available_mem},
        "usage_percent": $(echo "scale=2; $used_mem * 100 / $total_mem" | bc)
    },
    "disk": {
        "total_gb": ${disk_total},
        "used_gb": ${disk_used},
        "available_gb": ${disk_available},
        "usage_percent": $(echo "scale=2; $disk_used * 100 / $disk_total" | bc)
    },
    "network": {
        "rx_bytes": ${network_rx:-0},
        "tx_bytes": ${network_tx:-0}
    },
    "nestgate_process": {
        "pid": ${nestgate_pid},
        "memory_mb": ${nestgate_mem:-0},
        "cpu_percent": ${nestgate_cpu:-0}
    }
}
EOF
    
    success "Resource usage report generated: $report_file"
}

# Main execution function
main() {
    log "🚀 Starting NestGate Production Monitoring Suite"
    log "================================================="
    
    # Check if running as appropriate user
    if [ "$EUID" -eq 0 ] && [ "$1" != "--allow-root" ]; then
        warning "Running as root. Consider running as nestgate user for security."
    fi
    
    # Parse command line arguments
    case "${1:-all}" in
        "health")
            system_health_check
            ;;
        "performance")
            performance_optimization
            ;;
        "security")
            security_monitoring
            ;;
        "backup")
            backup_system
            ;;
        "logs")
            log_analysis
            ;;
        "resources")
            resource_usage_report
            ;;
        "all")
            system_health_check
            security_monitoring
            log_analysis
            resource_usage_report
            performance_optimization
            backup_system
            ;;
        *)
            echo "Usage: $0 [health|performance|security|backup|logs|resources|all]"
            echo ""
            echo "Commands:"
            echo "  health      - System health check"
            echo "  performance - Performance optimization"
            echo "  security    - Security monitoring"
            echo "  backup      - System backup"
            echo "  logs        - Log analysis"
            echo "  resources   - Resource usage report"
            echo "  all         - Run all monitoring tasks (default)"
            exit 1
            ;;
    esac
    
    log "================================================="
    success "🎉 NestGate monitoring suite completed successfully!"
    
    # Display summary
    if [ -f "${METRICS_DIR}/health_metrics.json" ]; then
        local health_score=$(jq -r '.health_score' "${METRICS_DIR}/health_metrics.json" 2>/dev/null || echo "N/A")
        log "📊 Current Health Score: ${health_score}/100"
    fi
}

# Script entry point
main "$@" 