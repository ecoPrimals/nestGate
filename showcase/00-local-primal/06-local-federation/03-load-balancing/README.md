# ⚖️ Load Balancing & Failover - Production HA

**Time**: 10 minutes  
**Complexity**: Advanced  
**Prerequisites**: Completed 01-two-node-mesh and 02-replication

---

## 🎯 WHAT YOU'LL LEARN

- ✅ Distributed load balancing
- ✅ Automatic failover detection
- ✅ Zero-downtime recovery
- ✅ Request distribution algorithms
- ✅ Why HA matters for sovereignty

**After this demo**: You'll understand production-grade high availability

---

## 🚀 QUICK START

```bash
./demo-failover.sh
```

**Expected duration**: ~10 minutes  
**Expected result**: 3 nodes with automatic failover demonstrated

---

## 📋 WHAT HAPPENS

### **Phase 1: Normal Operation (3 nodes)**
```
Client requests: 450 req/s total

Load distribution:
  Node 1: 150 req/s (33.3%)
  Node 2: 150 req/s (33.3%)
  Node 3: 150 req/s (33.3%)
  
Algorithm: Round-robin
Latency avg: 2.3ms
Status: HEALTHY
```

### **Phase 2: Node Failure**
```
Simulating Node 2 failure...
  ⚠️  Node 2: Stopped responding
  ⚠️  Heartbeat timeout (5 seconds)
  ✅ Node 2: Marked as DOWN
  
Automatic failover:
  Requests redistributed
  Node 2 traffic → Node 1 & 3
  Time to failover: 5.2 seconds
```

### **Phase 3: 2-Node Operation**
```
Client requests: 450 req/s (maintained!)

Load distribution:
  Node 1: 225 req/s (50.0%)
  Node 2: DOWN (0%)
  Node 3: 225 req/s (50.0%)
  
Algorithm: Round-robin (2 nodes)
Latency avg: 2.5ms (+0.2ms)
Requests lost: 0 ✅
Status: DEGRADED (but operational)
```

### **Phase 4: Node Recovery**
```
Node 2 coming back online...
  ✅ Node 2: Heartbeat detected
  ✅ Node 2: Health check passed
  ✅ Node 2: Marked as UP
  
Automatic rebalancing:
  Traffic gradually shifted back
  Rebalance time: 3 seconds
  
Load distribution:
  Node 1: 150 req/s (33.3%)
  Node 2: 150 req/s (33.3%)
  Node 3: 150 req/s (33.3%)
  
Status: HEALTHY (fully restored)
```

**Total downtime**: 0 seconds!

---

## 💡 LOAD BALANCING ALGORITHMS

### **1. Round-Robin** (Default)
```
Request 1 → Node 1
Request 2 → Node 2
Request 3 → Node 3
Request 4 → Node 1
Request 5 → Node 2
...

Pros:
  ✅ Simple
  ✅ Fair distribution
  ✅ Predictable
  
Cons:
  ❌ Ignores node load
  ❌ Ignores latency
```

### **2. Least Connections**
```
Node 1: 50 active connections → Selected
Node 2: 75 active connections
Node 3: 80 active connections

Next request → Node 1 (least loaded)

Pros:
  ✅ Accounts for load
  ✅ Better utilization
  
Cons:
  ❌ More complex
  ❌ Requires state tracking
```

### **3. Latency-Based**
```
Node 1: 2.1ms avg latency → Selected
Node 2: 5.3ms avg latency
Node 3: 3.2ms avg latency

Next request → Node 1 (fastest)

Pros:
  ✅ Best user experience
  ✅ Adapts to network conditions
  
Cons:
  ❌ Complex
  ❌ Can cause load imbalance
```

### **4. Consistent Hashing**
```
Hash(request_id) % num_nodes → Selected node

Same request_id → Same node (sticky sessions)

Pros:
  ✅ Session affinity
  ✅ Cache-friendly
  
Cons:
  ❌ Can cause imbalance
  ❌ More complex failover
```

**NestGate uses**: Round-robin with health-aware failover

---

## 🔬 FAILOVER MECHANICS

### **Detection**
```
Every 5 seconds:
  For each node:
    Send: PING
    Wait: 3 seconds for PONG
    
    If PONG received:
      ✅ Node is healthy
    Else:
      ⚠️  Retry (1 more attempt)
      
    If 2nd retry fails:
      ❌ Mark node as DOWN
      🔄 Trigger failover
```

**Detection time**: 5-8 seconds (2 heartbeat intervals)

### **Failover Process**
```
Step 1: Remove node from active pool (10ms)
Step 2: Redistribute active connections (50-200ms)
Step 3: Update routing table (10ms)
Step 4: Notify other nodes (50ms)
Step 5: Log failover event (10ms)

Total: ~200-300ms

New requests: Immediately go to healthy nodes
Active connections: Gracefully migrated
Data: No loss (replicated)
```

### **Recovery Process**
```
Node comes back online:
  
Step 1: Detect via heartbeat (5s)
Step 2: Verify health (1s)
Step 3: Sync data if needed (varies)
Step 4: Mark as available (10ms)
Step 5: Gradual traffic ramp (10s)

Ramp-up:
  0-10s: 10% traffic
  10-20s: 50% traffic
  20-30s: 100% traffic

Prevents overwhelming recovered node
```

---

## 📊 PERFORMANCE IMPACT

### **3 Nodes vs 2 Nodes**

| Metric | 3 Nodes | 2 Nodes (1 down) | Impact |
|--------|---------|------------------|--------|
| **Throughput** | 450 req/s | 450 req/s | 0% loss ✅ |
| **Latency (p50)** | 2.3ms | 2.5ms | +9% |
| **Latency (p95)** | 5.1ms | 6.2ms | +22% |
| **Node Load** | 150 req/s | 225 req/s | +50% per node |
| **Availability** | 99.999% | 99.99% | -0.009% |

**Key Insight**: System remains operational, slight performance degradation

### **Failover Cost**

```
Detection: 5-8 seconds
Failover: 0.2-0.3 seconds
Recovery: 10-30 seconds (gradual)

Requests lost: 0
Data lost: 0
Downtime: 0 seconds

Cost: Negligible ✅
```

---

## 🎓 KEY CONCEPTS

### **1. N+1 Redundancy**

**What**: Always have 1 more node than minimum needed

**Example**:
```
Minimum capacity: 300 req/s
Each node handles: 150 req/s
Needed: 2 nodes
Deploy: 3 nodes (N=2, +1=3)

If 1 fails:
  2 nodes remaining
  2 × 150 = 300 req/s
  Still meets minimum ✅
```

### **2. Split-Brain Prevention**

**Problem**:
```
Network partition:
  Node 1: Alone
  Node 2+3: Together
  
Without quorum:
  Both sides think they're primary
  Data divergence!
  Split-brain! ❌
```

**Solution (Quorum)**:
```
Require majority (2/3):
  Node 1: 1/3 nodes (no quorum, read-only)
  Node 2+3: 2/3 nodes (has quorum, read-write)
  
Only one side can write ✅
Data consistency maintained ✅
```

### **3. Graceful Degradation**

**Philosophy**: System should degrade gracefully, not catastrophically

**Levels**:
```
Level 1: Full capacity (3 nodes)
  ✅ 100% throughput
  ✅ Best latency
  ✅ Full redundancy
  
Level 2: Reduced capacity (2 nodes)
  ✅ 100% throughput (higher latency)
  ⚠️  Good latency
  ⚠️  Reduced redundancy
  
Level 3: Minimal capacity (1 node)
  ⚠️  66% throughput
  ⚠️  Higher latency
  ❌ No redundancy
  Status: CRITICAL
  
Level 4: No capacity (0 nodes)
  ❌ Service unavailable
  Status: DOWN
```

NestGate aims to never reach Level 4

---

## 💬 WHAT YOU'LL SEE

```
⚖️ NestGate Load Balancing & Failover Demo
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 1: Normal Operation (3 Nodes)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Starting 3-node cluster...
  ✅ Node 1: ONLINE (localhost:8080)
  ✅ Node 2: ONLINE (localhost:8082)
  ✅ Node 3: ONLINE (localhost:8084)

Mesh status:
  Connections: 3 (full mesh)
  Health: All nodes healthy

Generating load: 450 req/s
Load distribution:
  Node 1: █████████░ 150 req/s (33.3%)
  Node 2: █████████░ 150 req/s (33.3%)
  Node 3: █████████░ 150 req/s (33.3%)
  
Performance:
  Total: 450 req/s
  Latency (p50): 2.3ms
  Latency (p95): 5.1ms
  
Status: HEALTHY ✅

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 2: Simulating Node 2 Failure
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Stopping Node 2...
  ⚠️  Node 2: Stopped

[00:00] Heartbeat check...
[00:05] Node 2: No response (timeout)
[00:05] Node 2: Retry...
[00:08] Node 2: No response (2nd timeout)
[00:08] ❌ Node 2: Marked as DOWN

Triggering automatic failover...
  🔄 Removing Node 2 from pool
  🔄 Redistributing connections
  🔄 Updating routing table
  ✅ Failover complete (280ms)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 3: Operating with 2 Nodes
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Cluster status:
  ✅ Node 1: ONLINE
  ❌ Node 2: DOWN
  ✅ Node 3: ONLINE
  
Mesh status:
  Connections: 1 (degraded)
  Quorum: 2/3 nodes (maintained ✅)

Load distribution:
  Node 1: █████████████░ 225 req/s (50.0%)
  Node 2: ░░░░░░░░░░░░░░ 0 req/s (DOWN)
  Node 3: █████████████░ 225 req/s (50.0%)
  
Performance:
  Total: 450 req/s (maintained! ✅)
  Latency (p50): 2.5ms (+0.2ms)
  Latency (p95): 6.2ms (+1.1ms)
  Requests lost: 0 ✅
  
Status: DEGRADED (but operational) ⚠️

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 4: Node 2 Recovery
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Starting Node 2...
  ✅ Node 2: ONLINE

[00:00] Heartbeat detected
[00:01] Running health checks...
[00:02] ✅ Node 2: Health check passed
[00:02] ✅ Node 2: Marked as UP

Gradual traffic ramp-up:
[00:00-10] Node 2: ███░░░░░░░ 15 req/s (10%)
[00:10-20] Node 2: █████░░░░░ 75 req/s (50%)
[00:20-30] Node 2: █████████░ 150 req/s (100%)

Final distribution:
  Node 1: █████████░ 150 req/s (33.3%)
  Node 2: █████████░ 150 req/s (33.3%)
  Node 3: █████████░ 150 req/s (33.3%)

Status: HEALTHY (fully restored) ✅

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Demo Complete!

Summary:
  Total runtime: 90 seconds
  Node 2 downtime: 30 seconds
  Service downtime: 0 seconds ✅
  Requests lost: 0 ✅
  Data lost: 0 ✅
  
⏭️  Next: Three-node cluster details
   cd ../04-three-node-cluster && ./demo-cluster.sh
```

---

## 🔬 TRY IT YOURSELF

### Experiment 1: Kill Multiple Nodes
```bash
# Start 3 nodes
# Kill 2 nodes simultaneously
# Watch quorum handling
# Only 1/3 nodes = no quorum
# System goes read-only
```

### Experiment 2: Network Partition
```bash
# Simulate network split
# 1 node isolated
# 2 nodes together
# Quorum maintained on 2-node side
# Data consistency preserved
```

### Experiment 3: Load Testing
```bash
# Generate high load (1000+ req/s)
# Kill nodes randomly
# Measure impact on latency
# Verify zero data loss
```

---

**Ready for high availability?** Run `./demo-failover.sh`!

⚖️ **Load balancing: Zero-downtime federation!** ⚖️

