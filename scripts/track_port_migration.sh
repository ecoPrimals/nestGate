#!/bin/bash
# Port Migration Progress Tracker
# Tracks hardcoded port instances in the codebase

echo "╔════════════════════════════════════════════════════════════╗"
echo "║        PORT MIGRATION PROGRESS TRACKER                    ║"
echo "║        NestGate - December 2, 2025                        ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

cd "$(dirname "$0")/.." || exit 1

# Function to count instances
count_port() {
    local port=$1
    local count
    count=$(rg "\b$port\b" --type rust code/ 2>/dev/null | wc -l)
    echo "$count"
}

# Function to count in production code (exclude tests)
count_prod() {
    local port=$1
    local count
    count=$(rg "\b$port\b" --type rust code/crates/*/src 2>/dev/null | grep -v test | wc -l)
    echo "$count"
}

echo "Priority Ports:"
echo "==============="
echo ""

# API Server
api_total=$(count_port "8080")
api_prod=$(count_prod "8080")
echo "📍 8080 (API Server):     $api_total instances ($api_prod in production)"

# Discovery Ports
for port in 3000 3001 3002 3010; do
    count=$(count_port "$port")
    count_prod=$(count_prod "$port")
    echo "📍 $port (Discovery):      $count instances ($count_prod in production)"
done

# Metrics
metrics_total=$(count_port "9090")
metrics_prod=$(count_prod "9090")
echo "📍 9090 (Metrics):        $metrics_total instances ($metrics_prod in production)"

# External Services (should be removed from NestGate config)
echo ""
echo "External Services (Document, Don't Migrate):"
echo "============================================"
postgres=$(count_port "5432")
echo "📍 5432 (PostgreSQL):     $postgres instances"
redis=$(count_port "6379")
echo "📍 6379 (Redis):          $redis instances"
mongo=$(count_port "27017")
echo "📍 27017 (MongoDB):       $mongo instances"

echo ""
echo "Summary:"
echo "========"

# Calculate totals
total_priority=$((api_total + $(count_port "3000") + $(count_port "3001") + $(count_port "3002") + $(count_port "3010") + metrics_total))
total_external=$((postgres + redis + mongo))
total_all=$((total_priority + total_external))

echo "Priority ports to migrate: $total_priority"
echo "External service refs:     $total_external"
echo "Total hardcoded ports:     $total_all"
echo ""

# Calculate percentage if we have a baseline
baseline=1083
if [ -n "$baseline" ]; then
    migrated=$((baseline - total_priority))
    percent=$((migrated * 100 / baseline))
    echo "Migration Progress:"
    echo "=================="
    echo "Baseline:   $baseline instances"
    echo "Remaining:  $total_priority instances"
    echo "Migrated:   $migrated instances"
    echo "Progress:   $percent%"
    echo ""
    
    # Progress bar
    completed=$((percent / 5))
    remaining=$((20 - completed))
    printf "["
    for ((i=0; i<completed; i++)); do printf "█"; done
    for ((i=0; i<remaining; i++)); do printf "░"; done
    printf "] $percent%%\n"
fi

echo ""
echo "Next Steps:"
echo "==========="
if [ "$api_total" -gt 200 ]; then
    echo "✓ Priority 1: Migrate API server port (8080) - $api_total instances"
elif [ "$total_priority" -gt 100 ]; then
    echo "✓ Priority 2: Migrate discovery ports (3000-3010)"
elif [ "$total_priority" -gt 50 ]; then
    echo "✓ Priority 3: Migrate metrics port (9090)"
else
    echo "✓ Final cleanup: Migrate remaining instances"
fi

echo ""
echo "Run './scripts/find_port_instances.sh <port>' to see details"
echo ""

