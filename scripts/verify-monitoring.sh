#!/bin/bash
# Verify monitoring stack is operational

set -e

echo "🔍 Verifying monitoring stack..."
echo ""

# Function to check service
check_service() {
    local name=$1
    local url=$2
    local pattern=$3
    
    if curl -s -f "$url" | grep -q "$pattern"; then
        echo "✅ $name: OK"
        return 0
    else
        echo "❌ $name: FAILED"
        return 1
    fi
}

# Check Prometheus
check_service "Prometheus" "http://localhost:9090/-/healthy" "Prometheus is Healthy"

# Check Grafana  
check_service "Grafana" "http://localhost:3000/api/health" "ok"

# Check Loki
check_service "Loki" "http://localhost:3100/ready" "ready"

# Check NestGate API
check_service "NestGate API" "http://localhost:8080/health" "healthy"

# Check NestGate metrics endpoint
check_service "NestGate Metrics" "http://localhost:8080/metrics" "http_requests_total"

echo ""
echo "✅ Monitoring stack verification complete!"
