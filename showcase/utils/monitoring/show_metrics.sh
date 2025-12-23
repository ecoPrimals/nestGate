#!/usr/bin/env bash
# Show live metrics from NestGate
# Real-time monitoring dashboard in terminal

set -euo pipefail

NESTGATE_URL="${NESTGATE_URL:-http://localhost:8080}"
REFRESH_INTERVAL="${REFRESH_INTERVAL:-5}"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

clear_screen() {
    printf "\033c"
}

get_health() {
    curl -s -f "${NESTGATE_URL}/health" 2>/dev/null && echo "HEALTHY" || echo "UNHEALTHY"
}

get_uptime() {
    # Try to get uptime from health endpoint
    curl -s "${NESTGATE_URL}/health" 2>/dev/null | \
        jq -r '.uptime_seconds // .uptime // "unknown"' 2>/dev/null || echo "unknown"
}

format_uptime() {
    local seconds=$1
    if [ "$seconds" = "unknown" ]; then
        echo "unknown"
        return
    fi
    
    local days=$((seconds / 86400))
    local hours=$(((seconds % 86400) / 3600))
    local minutes=$(((seconds % 3600) / 60))
    
    if [ $days -gt 0 ]; then
        echo "${days}d ${hours}h ${minutes}m"
    elif [ $hours -gt 0 ]; then
        echo "${hours}h ${minutes}m"
    else
        echo "${minutes}m"
    fi
}

get_system_metrics() {
    # CPU
    if command -v top &>/dev/null; then
        CPU=$(top -bn1 | grep "Cpu(s)" | sed "s/.*, *\([0-9.]*\)%* id.*/\1/" | \
              awk '{printf "%.1f", 100 - $1}' 2>/dev/null || echo "N/A")
    else
        CPU="N/A"
    fi
    
    # Memory
    if command -v free &>/dev/null; then
        MEMORY=$(free -m | awk 'NR==2{printf "%dMB / %dMB (%.1f%%)", $3, $2, $3*100/$2 }' 2>/dev/null || echo "N/A")
    else
        MEMORY="N/A"
    fi
    
    # Disk
    DISK=$(df -h . | awk 'NR==2{printf "%s / %s (%s)", $3, $2, $5}' 2>/dev/null || echo "N/A")
    
    echo "$CPU|$MEMORY|$DISK"
}

display_dashboard() {
    clear_screen
    
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    local health=$(get_health)
    local uptime_sec=$(get_uptime)
    local uptime=$(format_uptime "$uptime_sec")
    local metrics=$(get_system_metrics)
    
    IFS='|' read -r cpu memory disk <<< "$metrics"
    
    # Determine health color
    if [ "$health" = "HEALTHY" ]; then
        health_color=$GREEN
    else
        health_color=$RED
    fi
    
    # Header
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║                                                            ║"
    echo "║              NestGate Live Metrics Dashboard              ║"
    echo "║                                                            ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
    echo "Last Update: $timestamp"
    echo "Refresh: Every ${REFRESH_INTERVAL}s (Ctrl+C to stop)"
    echo ""
    
    # Service Status
    echo -e "${BLUE}━━━ Service Status ━━━${NC}"
    echo -e "Health:     ${health_color}${health}${NC}"
    echo -e "Uptime:     ${uptime}"
    echo -e "Endpoint:   ${NESTGATE_URL}"
    echo ""
    
    # System Resources
    echo -e "${BLUE}━━━ System Resources ━━━${NC}"
    echo -e "CPU:        ${cpu}%"
    echo -e "Memory:     ${memory}"
    echo -e "Disk:       ${disk}"
    echo ""
    
    # Try to get additional metrics from NestGate
    echo -e "${BLUE}━━━ Storage Metrics ━━━${NC}"
    STORAGE=$(curl -s "${NESTGATE_URL}/api/v1/metrics/storage" 2>/dev/null || \
              curl -s "${NESTGATE_URL}/api/v1/metrics" 2>/dev/null)
    
    if [ -n "$STORAGE" ] && echo "$STORAGE" | jq -e '.' &>/dev/null; then
        echo "$STORAGE" | jq -r '
            "Datasets:   \(.datasets // .storage.datasets // "N/A")",
            "Objects:    \(.total_objects // .storage.total_objects // "N/A")",
            "Size:       \(.total_size_gb // .storage.total_size_gb // "N/A") GB"
        ' 2>/dev/null || echo "  (Metrics not available)"
    else
        echo "  (Storage metrics not available)"
    fi
    
    echo ""
    
    # API Metrics
    echo -e "${BLUE}━━━ API Metrics ━━━${NC}"
    API=$(curl -s "${NESTGATE_URL}/api/v1/metrics/api" 2>/dev/null || \
          curl -s "${NESTGATE_URL}/api/v1/status" 2>/dev/null)
    
    if [ -n "$API" ] && echo "$API" | jq -e '.' &>/dev/null; then
        echo "$API" | jq -r '
            "Requests:   \(.requests_total // .api.requests_total // "N/A")",
            "Errors:     \(.requests_error // .api.requests_error // "N/A")",
            "Error Rate: \(.error_rate // .api.error_rate // "N/A")%"
        ' 2>/dev/null || echo "  (Metrics not available)"
    else
        echo "  (API metrics not available)"
    fi
    
    echo ""
    echo -e "${YELLOW}Press Ctrl+C to stop monitoring${NC}"
}

main() {
    # Check if NestGate is accessible
    if ! curl -s -f "${NESTGATE_URL}/health" > /dev/null 2>&1; then
        echo "Error: Cannot reach NestGate at ${NESTGATE_URL}"
        echo "Make sure NestGate is running."
        exit 1
    fi
    
    # Trap Ctrl+C for clean exit
    trap 'echo -e "\n\nMonitoring stopped."; exit 0' INT
    
    # Main monitoring loop
    while true; do
        display_dashboard
        sleep "$REFRESH_INTERVAL"
    done
}

main "$@"

