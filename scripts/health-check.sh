#!/usr/bin/env bash
# Comprehensive health check for NestGate production deployment

set -euo pipefail

# Configuration
NESTGATE_HOST="${NESTGATE_HOST:-localhost}"
NESTGATE_PORT="${NESTGATE_PORT:-8000}"
HEALTH_ENDPOINT="http://${NESTGATE_HOST}:${NESTGATE_PORT}/health"
METRICS_ENDPOINT="http://${NESTGATE_HOST}:${NESTGATE_PORT}/metrics"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

check_service() {
    local name=$1
    local url=$2
    local expected_status=${3:-200}
    
    echo -n "Checking $name... "
    
    if response=$(curl -s -o /dev/null -w "%{http_code}" "$url" 2>/dev/null); then
        if [ "$response" = "$expected_status" ]; then
            echo -e "${GREEN}✅ OK${NC} (HTTP $response)"
            return 0
        else
            echo -e "${RED}❌ FAIL${NC} (HTTP $response, expected $expected_status)"
            return 1
        fi
    else
        echo -e "${RED}❌ UNREACHABLE${NC}"
        return 1
    fi
}

check_metrics() {
    echo -n "Checking metrics availability... "
    
    if metrics=$(curl -s "$METRICS_ENDPOINT" 2>/dev/null); then
        metric_count=$(echo "$metrics" | grep -c "^nestgate_" || true)
        if [ "$metric_count" -gt 0 ]; then
            echo -e "${GREEN}✅ OK${NC} ($metric_count metrics available)"
            return 0
        else
            echo -e "${YELLOW}⚠️ NO METRICS${NC}"
            return 1
        fi
    else
        echo -e "${RED}❌ UNREACHABLE${NC}"
        return 1
    fi
}

check_system_resources() {
    echo "📊 System Resources:"
    
    # CPU usage
    cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | sed 's/%us,//')
    echo "  CPU Usage: ${cpu_usage}%"
    
    # Memory usage
    memory_info=$(free -h | grep "Mem:")
    echo "  Memory: $memory_info"
    
    # Disk usage
    disk_usage=$(df -h / | tail -1 | awk '{print "Used: " $3 " / " $2 " (" $5 ")"}')
    echo "  Disk: $disk_usage"
}

main() {
    echo "🏥 NestGate Health Check Report"
    echo "================================"
    echo "Target: $NESTGATE_HOST:$NESTGATE_PORT"
    echo "Time: $(date)"
    echo ""
    
    # Service checks
    local all_good=true
    
    check_service "Health Endpoint" "$HEALTH_ENDPOINT" || all_good=false
    check_metrics || all_good=false
    
    echo ""
    check_system_resources
    
    echo ""
    if [ "$all_good" = true ]; then
        echo -e "${GREEN}🎉 All systems operational!${NC}"
        exit 0
    else
        echo -e "${RED}⚠️ Some issues detected${NC}"
        exit 1
    fi
}

main "$@"
