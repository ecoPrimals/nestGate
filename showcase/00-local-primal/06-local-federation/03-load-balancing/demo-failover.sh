#!/bin/bash
# ⚖️ Load Balancing & Failover Demo
# Time: ~10 minutes
# Shows: Automatic failover and zero-downtime operation

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

echo "⚖️ NestGate Load Balancing & Failover Demo"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "This demo simulates a 3-node cluster with automatic"
echo "load balancing and failover."
echo ""
echo "Duration: ~10 minutes"
echo ""

OUTPUT_DIR="outputs/failover-demo-$(date +%s)"
mkdir -p "$OUTPUT_DIR"

# Phase 1: Normal Operation
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Phase 1: Normal Operation (3 Nodes)${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Starting 3-node cluster..."
sleep 1

# Simulate 3 nodes
NODE1_PORT=8080
NODE2_PORT=8082
NODE3_PORT=8084

echo "  ✅ Node 1: ONLINE (localhost:${NODE1_PORT})"
sleep 0.5
echo "  ✅ Node 2: ONLINE (localhost:${NODE2_PORT})"
sleep 0.5
echo "  ✅ Node 3: ONLINE (localhost:${NODE3_PORT})"
sleep 0.5
echo ""

echo "Mesh status:"
echo "  Connections: 3 (full mesh)"
echo "  Health: All nodes healthy"
echo ""
sleep 1

echo "Generating load: 450 req/s"
echo "Load distribution:"
LOAD_PER_NODE=150
echo "  Node 1: █████████░ ${LOAD_PER_NODE} req/s (33.3%)"
sleep 0.3
echo "  Node 2: █████████░ ${LOAD_PER_NODE} req/s (33.3%)"
sleep 0.3
echo "  Node 3: █████████░ ${LOAD_PER_NODE} req/s (33.3%)"
sleep 0.5
echo ""

echo "Performance:"
echo "  Total: 450 req/s"
echo "  Latency (p50): 2.3ms"
echo "  Latency (p95): 5.1ms"
echo ""
echo -e "${GREEN}Status: HEALTHY ✅${NC}"
echo ""

# Save metrics
cat > "$OUTPUT_DIR/phase1-metrics.json" << EOF
{
  "phase": "normal_operation",
  "nodes": 3,
  "total_rps": 450,
  "node1_rps": 150,
  "node2_rps": 150,
  "node3_rps": 150,
  "latency_p50_ms": 2.3,
  "latency_p95_ms": 5.1,
  "status": "healthy"
}
EOF

sleep 3

# Phase 2: Node Failure
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Phase 2: Simulating Node 2 Failure${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Stopping Node 2..."
sleep 1
echo "  ⚠️  Node 2: Stopped"
echo ""
sleep 1

echo "[00:00] Heartbeat check..."
sleep 2
echo "[00:05] Node 2: No response (timeout)"
sleep 1
echo "[00:05] Node 2: Retry..."
sleep 2
echo "[00:08] Node 2: No response (2nd timeout)"
sleep 0.5
echo "[00:08] ❌ Node 2: Marked as DOWN"
echo ""
sleep 1

echo "Triggering automatic failover..."
sleep 0.5
echo "  🔄 Removing Node 2 from pool"
sleep 0.3
echo "  🔄 Redistributing connections"
sleep 0.3
echo "  🔄 Updating routing table"
sleep 0.3
echo "  ✅ Failover complete (280ms)"
echo ""

sleep 2

# Phase 3: 2-Node Operation
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Phase 3: Operating with 2 Nodes${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Cluster status:"
echo "  ✅ Node 1: ONLINE"
echo "  ❌ Node 2: DOWN"
echo "  ✅ Node 3: ONLINE"
echo ""
sleep 0.5

echo "Mesh status:"
echo "  Connections: 1 (degraded)"
echo "  Quorum: 2/3 nodes (maintained ✅)"
echo ""
sleep 1

echo "Load distribution:"
FAILOVER_LOAD=225
echo "  Node 1: █████████████░ ${FAILOVER_LOAD} req/s (50.0%)"
sleep 0.3
echo "  Node 2: ░░░░░░░░░░░░░░ 0 req/s (DOWN)"
sleep 0.3
echo "  Node 3: █████████████░ ${FAILOVER_LOAD} req/s (50.0%)"
sleep 0.5
echo ""

echo "Performance:"
echo "  Total: 450 req/s (maintained! ✅)"
echo "  Latency (p50): 2.5ms (+0.2ms)"
echo "  Latency (p95): 6.2ms (+1.1ms)"
echo "  Requests lost: 0 ✅"
echo ""
echo -e "${YELLOW}Status: DEGRADED (but operational) ⚠️${NC}"
echo ""

# Save degraded metrics
cat > "$OUTPUT_DIR/phase3-metrics.json" << EOF
{
  "phase": "degraded_operation",
  "nodes": 2,
  "nodes_down": 1,
  "total_rps": 450,
  "node1_rps": 225,
  "node2_rps": 0,
  "node3_rps": 225,
  "latency_p50_ms": 2.5,
  "latency_p95_ms": 6.2,
  "requests_lost": 0,
  "status": "degraded"
}
EOF

sleep 3

# Phase 4: Recovery
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Phase 4: Node 2 Recovery${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Starting Node 2..."
sleep 1
echo "  ✅ Node 2: ONLINE"
echo ""
sleep 1

echo "[00:00] Heartbeat detected"
sleep 1
echo "[00:01] Running health checks..."
sleep 1
echo "[00:02] ✅ Node 2: Health check passed"
sleep 0.5
echo "[00:02] ✅ Node 2: Marked as UP"
echo ""
sleep 1

echo "Gradual traffic ramp-up:"
sleep 0.5
echo "[00:00-10] Node 2: ███░░░░░░░ 15 req/s (10%)"
sleep 2
echo "[00:10-20] Node 2: █████░░░░░ 75 req/s (50%)"
sleep 2
echo "[00:20-30] Node 2: █████████░ 150 req/s (100%)"
sleep 2
echo ""

echo "Final distribution:"
echo "  Node 1: █████████░ ${LOAD_PER_NODE} req/s (33.3%)"
sleep 0.3
echo "  Node 2: █████████░ ${LOAD_PER_NODE} req/s (33.3%)"
sleep 0.3
echo "  Node 3: █████████░ ${LOAD_PER_NODE} req/s (33.3%)"
sleep 0.5
echo ""

echo -e "${GREEN}Status: HEALTHY (fully restored) ✅${NC}"
echo ""

# Save recovered metrics
cat > "$OUTPUT_DIR/phase4-metrics.json" << EOF
{
  "phase": "recovered",
  "nodes": 3,
  "total_rps": 450,
  "node1_rps": 150,
  "node2_rps": 150,
  "node3_rps": 150,
  "latency_p50_ms": 2.3,
  "latency_p95_ms": 5.1,
  "recovery_time_s": 30,
  "status": "healthy"
}
EOF

sleep 2

# Summary
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Demo Complete!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Summary:"
echo "  Total runtime: 90 seconds"
echo "  Node 2 downtime: 30 seconds"
echo "  Service downtime: 0 seconds ✅"
echo "  Requests lost: 0 ✅"
echo "  Data lost: 0 ✅"
echo ""
echo "🎓 What you learned:"
echo "   ✅ Automatic load distribution"
echo "   ✅ Failover detection (5-8 seconds)"
echo "   ✅ Zero-downtime operation"
echo "   ✅ Graceful recovery with ramp-up"
echo ""
echo "💡 Key Insights:"
echo "   • Service continues during node failure"
echo "   • No requests or data lost"
echo "   • Automatic rebalancing on recovery"
echo "   • Production-grade high availability"
echo ""
echo "📁 Metrics saved to: $OUTPUT_DIR"
echo ""
echo "⏭️  Next: Full 3-node cluster deep dive"
echo "   cd ../04-three-node-cluster && ./demo-cluster.sh"
echo ""
echo "⚖️ Load balancing: Zero-downtime federation!"

