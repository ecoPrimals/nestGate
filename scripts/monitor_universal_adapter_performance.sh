#!/bin/bash

# 📊 UNIVERSAL ADAPTER PERFORMANCE MONITORING SCRIPT
# Real-time monitoring of capability discovery performance and RPC routing metrics
# Date: September 12, 2025
# Status: Universal Adapter Migration - Performance Validation

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
NESTGATE_ENDPOINT=${NESTGATE_ENDPOINT:-"http://localhost:8080"}
MONITORING_INTERVAL=${MONITORING_INTERVAL:-5}
PERFORMANCE_LOG="universal-adapter-performance.log"
ALERT_THRESHOLD_LATENCY_MS=${ALERT_THRESHOLD_LATENCY_MS:-1000}
ALERT_THRESHOLD_ERROR_RATE=${ALERT_THRESHOLD_ERROR_RATE:-5}

# Performance tracking variables
declare -A capability_latencies
declare -A rpc_success_counts
declare -A rpc_error_counts
declare -A capability_health_status

echo -e "${BLUE}📊 UNIVERSAL ADAPTER PERFORMANCE MONITOR${NC}"
echo -e "${BLUE}=========================================${NC}"
echo -e "${CYAN}Endpoint: $NESTGATE_ENDPOINT${NC}"
echo -e "${CYAN}Interval: ${MONITORING_INTERVAL}s${NC}"
echo -e "${CYAN}Log File: $PERFORMANCE_LOG${NC}"
echo ""

# Function to log performance data
log_performance() {
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    local message=$1
    echo "[$timestamp] $message" >> "$PERFORMANCE_LOG"
}

# Function to test capability discovery latency
test_capability_discovery() {
    local capability=$1
    local start_time=$(date +%s%3N)
    
    # Test capability discovery endpoint
    local response=$(curl -s -w "%{http_code}" -o /tmp/capability_response.json "$NESTGATE_ENDPOINT/universal-adapter/capabilities/$capability" 2>/dev/null || echo "000")
    
    local end_time=$(date +%s%3N)
    local latency=$((end_time - start_time))
    
    if [[ "$response" == "200" ]]; then
        capability_latencies[$capability]=$latency
        log_performance "CAPABILITY_DISCOVERY $capability latency=${latency}ms status=success"
        return 0
    else
        capability_latencies[$capability]=-1
        log_performance "CAPABILITY_DISCOVERY $capability latency=timeout status=error code=$response"
        return 1
    fi
}

# Function to test RPC routing performance
test_rpc_routing() {
    local capability=$1
    local method=${2:-"health_check"}
    local start_time=$(date +%s%3N)
    
    # Test RPC routing through universal adapter
    local response=$(curl -s -w "%{http_code}" -X POST \
        -H "Content-Type: application/json" \
        -d "{\"method\":\"$method\",\"capability\":\"$capability\",\"params\":{}}" \
        -o /tmp/rpc_response.json \
        "$NESTGATE_ENDPOINT/api/v1/rpc/universal" 2>/dev/null || echo "000")
    
    local end_time=$(date +%s%3N)
    local latency=$((end_time - start_time))
    
    if [[ "$response" == "200" ]]; then
        rpc_success_counts[$capability]=$((${rpc_success_counts[$capability]:-0} + 1))
        log_performance "RPC_ROUTING $capability method=$method latency=${latency}ms status=success"
        return 0
    else
        rpc_error_counts[$capability]=$((${rpc_error_counts[$capability]:-0} + 1))
        log_performance "RPC_ROUTING $capability method=$method latency=${latency}ms status=error code=$response"
        return 1
    fi
}

# Function to check capability health
check_capability_health() {
    local capability=$1
    
    # Check if capability is available and responding
    local health_response=$(curl -s -w "%{http_code}" \
        -o /tmp/health_response.json \
        "$NESTGATE_ENDPOINT/universal-adapter/capabilities/$capability/health" 2>/dev/null || echo "000")
    
    if [[ "$health_response" == "200" ]]; then
        capability_health_status[$capability]="healthy"
        log_performance "CAPABILITY_HEALTH $capability status=healthy"
        return 0
    else
        capability_health_status[$capability]="unhealthy"
        log_performance "CAPABILITY_HEALTH $capability status=unhealthy code=$health_response"
        return 1
    fi
}

# Function to display performance dashboard
display_performance_dashboard() {
    clear
    echo -e "${BLUE}📊 UNIVERSAL ADAPTER PERFORMANCE DASHBOARD${NC}"
    echo -e "${BLUE}============================================${NC}"
    echo -e "${CYAN}Timestamp: $(date)${NC}"
    echo ""
    
    # Capability Discovery Performance
    echo -e "${GREEN}🔍 CAPABILITY DISCOVERY PERFORMANCE${NC}"
    echo -e "${GREEN}====================================${NC}"
    for capability in orchestration security artificial_intelligence compute; do
        local latency=${capability_latencies[$capability]:-"N/A"}
        local status_color=$GREEN
        local status_icon="✅"
        
        if [[ "$latency" == "-1" ]]; then
            status_color=$RED
            status_icon="❌"
            latency="TIMEOUT"
        elif [[ "$latency" != "N/A" && "$latency" -gt "$ALERT_THRESHOLD_LATENCY_MS" ]]; then
            status_color=$YELLOW
            status_icon="⚠️"
        fi
        
        printf "${status_color}${status_icon} %-20s: %10s ms${NC}\n" "$capability" "$latency"
    done
    echo ""
    
    # RPC Routing Performance
    echo -e "${GREEN}🔀 RPC ROUTING PERFORMANCE${NC}"
    echo -e "${GREEN}==========================${NC}"
    for capability in orchestration security artificial_intelligence compute; do
        local success=${rpc_success_counts[$capability]:-0}
        local errors=${rpc_error_counts[$capability]:-0}
        local total=$((success + errors))
        local success_rate=0
        
        if [[ $total -gt 0 ]]; then
            success_rate=$((success * 100 / total))
        fi
        
        local status_color=$GREEN
        local status_icon="✅"
        
        if [[ $success_rate -lt $((100 - ALERT_THRESHOLD_ERROR_RATE)) ]]; then
            status_color=$RED
            status_icon="❌"
        elif [[ $success_rate -lt 95 ]]; then
            status_color=$YELLOW
            status_icon="⚠️"
        fi
        
        printf "${status_color}${status_icon} %-20s: %3d%% (%d/%d)${NC}\n" "$capability" "$success_rate" "$success" "$total"
    done
    echo ""
    
    # Capability Health Status
    echo -e "${GREEN}🏥 CAPABILITY HEALTH STATUS${NC}"
    echo -e "${GREEN}============================${NC}"
    for capability in orchestration security artificial_intelligence compute; do
        local health=${capability_health_status[$capability]:-"unknown"}
        local status_color=$GREEN
        local status_icon="✅"
        
        case $health in
            "healthy")
                status_color=$GREEN
                status_icon="✅"
                ;;
            "unhealthy")
                status_color=$RED
                status_icon="❌"
                ;;
            "unknown")
                status_color=$YELLOW
                status_icon="❓"
                ;;
        esac
        
        printf "${status_color}${status_icon} %-20s: %10s${NC}\n" "$capability" "$health"
    done
    echo ""
    
    # Performance Alerts
    echo -e "${YELLOW}⚠️  PERFORMANCE ALERTS${NC}"
    echo -e "${YELLOW}======================${NC}"
    local alert_count=0
    
    for capability in orchestration security artificial_intelligence compute; do
        local latency=${capability_latencies[$capability]:-0}
        if [[ "$latency" != "N/A" && "$latency" -gt "$ALERT_THRESHOLD_LATENCY_MS" ]]; then
            echo -e "${RED}🚨 HIGH LATENCY: $capability discovery took ${latency}ms (threshold: ${ALERT_THRESHOLD_LATENCY_MS}ms)${NC}"
            alert_count=$((alert_count + 1))
        fi
        
        local success=${rpc_success_counts[$capability]:-0}
        local errors=${rpc_error_counts[$capability]:-0}
        local total=$((success + errors))
        if [[ $total -gt 0 ]]; then
            local error_rate=$((errors * 100 / total))
            if [[ $error_rate -gt $ALERT_THRESHOLD_ERROR_RATE ]]; then
                echo -e "${RED}🚨 HIGH ERROR RATE: $capability RPC error rate is ${error_rate}% (threshold: ${ALERT_THRESHOLD_ERROR_RATE}%)${NC}"
                alert_count=$((alert_count + 1))
            fi
        fi
        
        local health=${capability_health_status[$capability]:-"unknown"}
        if [[ "$health" == "unhealthy" ]]; then
            echo -e "${RED}🚨 CAPABILITY UNHEALTHY: $capability is not responding to health checks${NC}"
            alert_count=$((alert_count + 1))
        fi
    done
    
    if [[ $alert_count -eq 0 ]]; then
        echo -e "${GREEN}✅ No performance alerts - all systems operating within thresholds${NC}"
    fi
    
    echo ""
    echo -e "${CYAN}📋 Press Ctrl+C to stop monitoring | Log: $PERFORMANCE_LOG${NC}"
}

# Function to generate performance report
generate_performance_report() {
    echo "Generating performance report..."
    
    local report_file="universal-adapter-performance-report.md"
    local timestamp=$(date)
    
    cat > "$report_file" << EOF
# 📊 Universal Adapter Performance Report

**Generated**: $timestamp  
**Monitoring Period**: Last monitoring session  
**Endpoint**: $NESTGATE_ENDPOINT  

## 🔍 Capability Discovery Performance

| Capability | Latest Latency (ms) | Status |
|------------|-------------------|--------|
EOF

    for capability in orchestration security artificial_intelligence compute; do
        local latency=${capability_latencies[$capability]:-"N/A"}
        local status="✅ Good"
        
        if [[ "$latency" == "-1" ]]; then
            status="❌ Timeout"
            latency="TIMEOUT"
        elif [[ "$latency" != "N/A" && "$latency" -gt "$ALERT_THRESHOLD_LATENCY_MS" ]]; then
            status="⚠️ High Latency"
        fi
        
        echo "| $capability | $latency | $status |" >> "$report_file"
    done
    
    cat >> "$report_file" << EOF

## 🔀 RPC Routing Performance

| Capability | Success Rate | Total Requests | Status |
|------------|-------------|----------------|--------|
EOF

    for capability in orchestration security artificial_intelligence compute; do
        local success=${rpc_success_counts[$capability]:-0}
        local errors=${rpc_error_counts[$capability]:-0}
        local total=$((success + errors))
        local success_rate=0
        local status="✅ Good"
        
        if [[ $total -gt 0 ]]; then
            success_rate=$((success * 100 / total))
            if [[ $success_rate -lt $((100 - ALERT_THRESHOLD_ERROR_RATE)) ]]; then
                status="❌ High Error Rate"
            elif [[ $success_rate -lt 95 ]]; then
                status="⚠️ Degraded"
            fi
        else
            status="❓ No Data"
        fi
        
        echo "| $capability | ${success_rate}% | $total | $status |" >> "$report_file"
    done
    
    cat >> "$report_file" << EOF

## 🏥 Capability Health Summary

| Capability | Health Status |
|------------|---------------|
EOF

    for capability in orchestration security artificial_intelligence compute; do
        local health=${capability_health_status[$capability]:-"unknown"}
        echo "| $capability | $health |" >> "$report_file"
    done
    
    cat >> "$report_file" << EOF

## 📈 Performance Recommendations

### Optimization Opportunities
- Monitor capabilities with latency > ${ALERT_THRESHOLD_LATENCY_MS}ms
- Investigate capabilities with error rates > ${ALERT_THRESHOLD_ERROR_RATE}%
- Ensure all capabilities maintain healthy status

### Alert Thresholds
- **Discovery Latency**: > ${ALERT_THRESHOLD_LATENCY_MS}ms
- **Error Rate**: > ${ALERT_THRESHOLD_ERROR_RATE}%
- **Health Check**: Unhealthy status

## 📊 Raw Performance Data

See detailed logs in: \`$PERFORMANCE_LOG\`

EOF

    echo -e "${GREEN}✅ Performance report generated: $report_file${NC}"
}

# Function to run performance monitoring loop
run_monitoring_loop() {
    echo "Starting performance monitoring..."
    log_performance "MONITORING_START endpoint=$NESTGATE_ENDPOINT interval=${MONITORING_INTERVAL}s"
    
    # Initialize counters
    for capability in orchestration security artificial_intelligence compute; do
        rpc_success_counts[$capability]=0
        rpc_error_counts[$capability]=0
    done
    
    while true; do
        # Test capability discovery for all capabilities
        for capability in orchestration security artificial_intelligence compute; do
            test_capability_discovery "$capability" &
            test_rpc_routing "$capability" &
            check_capability_health "$capability" &
        done
        
        # Wait for all background tests to complete
        wait
        
        # Display dashboard
        display_performance_dashboard
        
        # Sleep for monitoring interval
        sleep "$MONITORING_INTERVAL"
    done
}

# Signal handler for clean exit
cleanup() {
    echo ""
    echo -e "${YELLOW}📊 Stopping performance monitoring...${NC}"
    log_performance "MONITORING_STOP"
    generate_performance_report
    echo -e "${GREEN}✅ Performance monitoring stopped${NC}"
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Main execution
main() {
    # Check if NestGate is accessible
    if ! curl -f "$NESTGATE_ENDPOINT/health" >/dev/null 2>&1; then
        echo -e "${RED}❌ Cannot connect to NestGate at $NESTGATE_ENDPOINT${NC}"
        echo -e "${YELLOW}💡 Make sure NestGate is running and accessible${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ Connected to NestGate at $NESTGATE_ENDPOINT${NC}"
    echo -e "${CYAN}🚀 Starting performance monitoring...${NC}"
    echo ""
    
    # Initialize performance log
    echo "Universal Adapter Performance Monitoring Log" > "$PERFORMANCE_LOG"
    echo "Started: $(date)" >> "$PERFORMANCE_LOG"
    echo "Endpoint: $NESTGATE_ENDPOINT" >> "$PERFORMANCE_LOG"
    echo "==========================================" >> "$PERFORMANCE_LOG"
    
    # Start monitoring loop
    run_monitoring_loop
}

# Execute main function
main "$@" 