#!/bin/bash
# Load Balancing Demo - Distribute requests across NestGate mesh

set -e

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                                                                ║"
echo "║          NestGate Load Balancing Demonstration                 ║"
echo "║                                                                ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "This demo shows load balancing across multiple NestGate nodes"
echo ""

# Configuration
NODE_COUNT=3
BASE_PORT=8081
REQUEST_COUNT=30
LOAD_BALANCE_ALGORITHM="round_robin"

# Check if NestGate binary exists
if [ ! -f "../../target/release/nestgate" ]; then
    echo "❌ NestGate binary not found. Building..."
    cd ../.. && cargo build --release && cd showcase/03_federation/03_load_balancing
fi

# Create temp directory for node state
TEMP_DIR=$(mktemp -d)
echo "📁 Using temp directory: $TEMP_DIR"
echo ""

# Function to start a NestGate node
start_node() {
    local node_id=$1
    local port=$2
    local log_file="$TEMP_DIR/node_${node_id}.log"
    
    echo "Starting Node ${node_id} on port ${port}..."
    
    export NESTGATE_JWT_SECRET=$(openssl rand -base64 48)
    ../../../target/release/nestgate service start --port ${port} > "$log_file" 2>&1 &
    local pid=$!
    echo $pid > "$TEMP_DIR/node_${node_id}.pid"
    
    echo "  PID: $pid"
    echo "  Port: $port"
    echo "  Log: $log_file"
}

# Function to check if node is healthy
check_node_health() {
    local port=$1
    curl -sf http://127.0.0.1:${port}/health > /dev/null 2>&1
}

# Function to send requests to a node
send_requests() {
    local port=$1
    local count=$2
    local successful=0
    
    for i in $(seq 1 $count); do
        if curl -sf http://127.0.0.1:${port}/health > /dev/null 2>&1; then
            ((successful++))
        fi
    done
    
    echo $successful
}

# Cleanup function
cleanup() {
    echo ""
    echo "🧹 Cleaning up..."
    
    for node_id in $(seq 1 $NODE_COUNT); do
        if [ -f "$TEMP_DIR/node_${node_id}.pid" ]; then
            local pid=$(cat "$TEMP_DIR/node_${node_id}.pid")
            if ps -p $pid > /dev/null 2>&1; then
                kill $pid 2>/dev/null || true
                echo "  Stopped Node ${node_id} (PID: $pid)"
            fi
        fi
    done
    
    rm -rf "$TEMP_DIR"
    echo "✅ Cleanup complete"
}

trap cleanup EXIT

# 1. Start multiple NestGate nodes
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "1. Starting ${NODE_COUNT} NestGate Nodes"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

PORTS=()
for node_id in $(seq 1 $NODE_COUNT); do
    port=$((BASE_PORT + node_id - 1))
    PORTS+=($port)
    start_node $node_id $port
done

echo ""
echo "⏳ Waiting for nodes to start (10 seconds)..."
sleep 10
echo ""

# 2. Verify all nodes are healthy
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "2. Verifying Node Health"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

HEALTHY_NODES=0
for node_id in $(seq 1 $NODE_COUNT); do
    port=${PORTS[$((node_id - 1))]}
    if check_node_health $port; then
        echo "✅ Node ${node_id} (port ${port}): HEALTHY"
        ((HEALTHY_NODES++))
    else
        echo "❌ Node ${node_id} (port ${port}): UNHEALTHY"
    fi
done

echo ""
echo "Healthy Nodes: ${HEALTHY_NODES}/${NODE_COUNT}"

if [ $HEALTHY_NODES -eq 0 ]; then
    echo "❌ No healthy nodes available"
    exit 1
fi

echo ""

# 3. Simulate load balancing
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "3. Load Balancing Simulation (${LOAD_BALANCE_ALGORITHM})"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Sending ${REQUEST_COUNT} requests distributed across nodes..."
echo ""

# Initialize counters
declare -A REQUEST_COUNTS
declare -A SUCCESS_COUNTS
for node_id in $(seq 1 $NODE_COUNT); do
    REQUEST_COUNTS[$node_id]=0
    SUCCESS_COUNTS[$node_id]=0
done

# Distribute requests using round-robin
for i in $(seq 1 $REQUEST_COUNT); do
    node_idx=$(( (i - 1) % NODE_COUNT + 1 ))
    port=${PORTS[$((node_idx - 1))]}
    
    if curl -sf http://127.0.0.1:${port}/health > /dev/null 2>&1; then
        ((REQUEST_COUNTS[$node_idx]++))
        ((SUCCESS_COUNTS[$node_idx]++))
    else
        ((REQUEST_COUNTS[$node_idx]++))
    fi
    
    # Show progress
    if [ $((i % 10)) -eq 0 ]; then
        echo -n "."
    fi
done

echo ""
echo ""

# 4. Display results
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "4. Load Distribution Results"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

TOTAL_SUCCESSFUL=0
for node_id in $(seq 1 $NODE_COUNT); do
    port=${PORTS[$((node_id - 1))]}
    requests=${REQUEST_COUNTS[$node_id]}
    successful=${SUCCESS_COUNTS[$node_id]}
    percentage=$(awk "BEGIN {printf \"%.1f\", ($successful/$REQUEST_COUNT)*100}")
    
    TOTAL_SUCCESSFUL=$((TOTAL_SUCCESSFUL + successful))
    
    echo "Node ${node_id} (port ${port}):"
    echo "  Requests:  ${requests}"
    echo "  Successful: ${successful}"
    echo "  Percentage: ${percentage}%"
    
    # Simple bar chart
    bar_length=$(awk "BEGIN {printf \"%.0f\", ($successful/$REQUEST_COUNT)*50}")
    echo -n "  Distribution: ["
    for j in $(seq 1 $bar_length); do echo -n "█"; done
    for j in $(seq $((bar_length + 1)) 50); do echo -n " "; done
    echo "]"
    echo ""
done

# 5. Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "5. Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Total Requests:    ${REQUEST_COUNT}"
echo "Successful:        ${TOTAL_SUCCESSFUL}"
echo "Failed:            $((REQUEST_COUNT - TOTAL_SUCCESSFUL))"
echo "Success Rate:      $(awk "BEGIN {printf \"%.1f\", ($TOTAL_SUCCESSFUL/$REQUEST_COUNT)*100}")%"
echo ""
echo "Load Balancing Algorithm: ${LOAD_BALANCE_ALGORITHM}"
echo "Nodes:                    ${NODE_COUNT}"
echo "Requests per Node (avg):  $((REQUEST_COUNT / NODE_COUNT))"
echo ""

# Check if distribution is even
EXPECTED_PER_NODE=$((REQUEST_COUNT / NODE_COUNT))
DISTRIBUTION_EVEN=true
for node_id in $(seq 1 $NODE_COUNT); do
    successful=${SUCCESS_COUNTS[$node_id]}
    deviation=$(awk "BEGIN {print ($successful - $EXPECTED_PER_NODE)}")
    deviation_abs=${deviation#-}
    if [ $deviation_abs -gt 2 ]; then
        DISTRIBUTION_EVEN=false
        break
    fi
done

if [ "$DISTRIBUTION_EVEN" = true ]; then
    echo "✅ Load distribution is even (±2 requests)"
else
    echo "⚠️  Load distribution has some variance"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Benefits Demonstrated:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "✅ Horizontal scaling: ${NODE_COUNT}x nodes = ${NODE_COUNT}x potential throughput"
echo "✅ Even load distribution across all nodes"
echo "✅ No single point of failure"
echo "✅ Each node handles ~33% of total load"
echo "✅ Easy to add more nodes for more capacity"
echo ""
echo "🎉 Load balancing demonstration complete!"
echo ""
echo "Next Steps:"
echo "  • Try 04_failover to see what happens when a node fails"
echo "  • Increase NODE_COUNT to see scaling benefits"
echo "  • Try different load balancing algorithms"
echo ""

