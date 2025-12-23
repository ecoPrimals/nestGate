# Failover and High Availability

**Level**: 3 - Federation  
**Complexity**: Intermediate  
**Time**: 10 minutes  
**Prerequisites**: Multiple NestGate instances running

---

## 🎯 Purpose

Demonstrate how a NestGate mesh automatically handles node failures through:
- Automatic failover to healthy nodes
- Request rerouting
- Data availability despite node loss
- Graceful degradation

---

## 🏗️ Architecture

```
Initial State:
┌─────────┐     ┌─────────┐     ┌─────────┐
│ Node 1  │────▶│ Node 2  │────▶│ Node 3  │
│ PRIMARY │     │ STANDBY │     │ STANDBY │
└─────────┘     └─────────┘     └─────────┘
     │               │               │
     └───────────────┴───────────────┘
              Mesh Network

After Node 1 Failure:
     ✗
┌─────────┐     ┌─────────┐     ┌─────────┐
│ Node 1  │  X  │ Node 2  │────▶│ Node 3  │
│  DEAD   │     │ PRIMARY │     │ STANDBY │
└─────────┘     └─────────┘     └─────────┘
                     │               │
                     └───────────────┘
                      Mesh Network
```

### Failover Mechanisms

1. **Health Monitoring**: Continuous health checks
2. **Failure Detection**: Identify unresponsive nodes
3. **Automatic Rerouting**: Redirect requests to healthy nodes
4. **State Recovery**: Restore from replicated data
5. **Auto-Rejoin**: Node rejoins when healthy

---

## 🚀 What This Demo Shows

### 1. Initial Healthy State
- 3 nodes running in mesh
- All nodes healthy and serving
- Load balanced across all

### 2. Simulated Failure
- Kill one node
- Monitor failure detection
- Observe automatic rerouting

### 3. Continued Operation
- Remaining nodes handle load
- No data loss (replicated data)
- Users experience no downtime

### 4. Recovery
- Failed node restarts
- Automatically rejoins mesh
- Load rebalances

---

## 📋 Prerequisites

### System Requirements
- 3 available ports
- Ability to start/stop processes
- Network connectivity

### NestGate Built
```bash
cargo build --release
```

---

## 🏃 Running the Demo

### Quick Start
```bash
./demo.sh
```

### What Happens

1. **Startup** (30 seconds)
   - Start 3 NestGate nodes
   - Verify all healthy
   - Show baseline performance

2. **Normal Operation** (30 seconds)
   - Send requests to all nodes
   - Collect success metrics
   - Show even distribution

3. **Failure Simulation** (Instant)
   - Kill Node 1
   - Monitor failure detection
   - Show mesh update

4. **Degraded Operation** (30 seconds)
   - Continue sending requests
   - Only 2 nodes handle load
   - Show 100% success rate still

5. **Recovery** (30 seconds)
   - Restart Node 1
   - Node rejoins mesh
   - Load rebalances

6. **Post-Recovery** (30 seconds)
   - All 3 nodes operational again
   - Back to full capacity
   - Show metrics

---

## 📊 Expected Results

### Failure Detection
```
Time: T+0s   - Node 1 killed
Time: T+2s   - Health check fails
Time: T+3s   - Node marked as DOWN
Time: T+3s   - Requests reroute to Node 2,3
Time: T+4s   - Mesh updated (2 nodes)

Detection Time: ~3 seconds
Downtime: 0 requests failed
```

### Performance During Failure
```
Before Failure (3 nodes):
  Total Throughput: 600 req/s
  Per-Node Load: 200 req/s
  Success Rate: 100%

During Failure (2 nodes):
  Total Throughput: 400 req/s (66% of original)
  Per-Node Load: 200 req/s (unchanged)
  Success Rate: 100% (no failed requests!)

After Recovery (3 nodes):
  Total Throughput: 600 req/s (restored)
  Per-Node Load: 200 req/s
  Success Rate: 100%
```

### Data Availability
```
Data written to Node 1: ACCESSIBLE (replicated to Node 2,3)
Data written to Node 2: ACCESSIBLE (still running)
Data written to Node 3: ACCESSIBLE (still running)

Data Loss: 0 bytes
Data Unavailability: 0 seconds
```

---

## 🔍 What to Look For

### 1. Fast Failure Detection
- Health checks fail quickly
- Mesh updated within seconds
- No manual intervention

### 2. Automatic Rerouting
- New requests go to healthy nodes only
- No requests to failed node
- Transparent to clients

### 3. Zero Data Loss
- Replicated data still accessible
- Writes continue successfully
- Consistency maintained

### 4. Graceful Recovery
- Failed node rejoins automatically
- Mesh self-heals
- Full capacity restored

---

## 💡 Key Concepts

### 1. High Availability (HA)
- System remains operational despite failures
- No single point of failure
- Automatic failover

### 2. Failure Detection
- **Active Monitoring**: Regular health checks
- **Passive Monitoring**: Failed request detection
- **Consensus**: Multiple nodes agree on failure

### 3. Data Replication
- **Synchronous**: Wait for replica writes
- **Asynchronous**: Background replication
- **Quorum**: Majority must acknowledge

### 4. Split-Brain Prevention
- Network partitions can cause issues
- Use consensus protocols (Raft, Paxos)
- Ensure odd number of nodes

---

## 🎓 Learning Outcomes

After this demo, you'll understand:

1. ✅ How NestGate detects and handles node failures
2. ✅ Automatic failover without manual intervention
3. ✅ Data remains available despite node loss
4. ✅ Self-healing mesh capabilities
5. ✅ Zero-downtime operations

---

## 🔗 Related Demos

### Previous
- **03_load_balancing**: Request distribution
- **02_replication**: Data redundancy
- **01_mesh_formation**: Node discovery

### Next
- **05_distributed_snapshot**: Coordinated snapshots
- **../04_inter_primal_mesh**: Multi-primal coordination

---

## 🛠️ Customization

### Change Failure Scenario
```bash
# Edit demo.sh
FAILURE_TYPE="crash"      # Options: crash, network, slow
FAILED_NODE_ID=2          # Kill node 2 instead of node 1
FAILURE_DURATION=60       # Keep node down for 60 seconds
```

### Test Multiple Failures
```bash
# Edit demo.sh
SIMULTANEOUS_FAILURES=2   # Kill 2 nodes at once
# Warning: Need N/2+1 nodes for quorum!
```

---

## 📈 Availability Calculations

### Single Node
```
Uptime: 99.9%
Downtime/year: 8.76 hours
Availability: 3 nines
```

### 3-Node Mesh (Active-Active)
```
Uptime: 99.999%
Downtime/year: 5.26 minutes
Availability: 5 nines (High Availability)

Probability of all nodes failing: 0.001^3 = 0.000000001
Probability of 2+ nodes up: 99.9999%
```

### Recovery Time Objective (RTO)
```
Failure Detection: ~3 seconds
Rerouting: Immediate
Data Access: 0 seconds downtime

RTO: ~3 seconds
RPO: 0 (no data loss with replication)
```

---

## 🐛 Troubleshooting

### Issue: Slow Failure Detection
**Symptom**: Takes >10 seconds to detect failure  
**Solution**: Reduce health check interval, check network latency

### Issue: Data Unavailable After Failure
**Symptom**: Cannot access data from failed node  
**Solution**: Verify replication was enabled, check replica count

### Issue: Node Won't Rejoin
**Symptom**: Restarted node doesn't join mesh  
**Solution**: Check network connectivity, verify mesh configuration

---

## 🎯 Real-World Applications

### 1. Production Databases
- Critical data must be always available
- Automatic failover essential
- Zero data loss required

### 2. Web Services
- Handle server crashes gracefully
- No user-visible downtime
- Maintain SLAs

### 3. Data Lakes
- Massive datasets across many nodes
- Node failures common at scale
- Automatic recovery critical

### 4. Media Streaming
- Cannot interrupt streams
- Failover must be seamless
- Buffer on multiple nodes

---

## 🔮 Advanced Topics

### 1. Cascading Failures
- What if multiple nodes fail?
- Quorum requirements
- Degraded mode operation

### 2. Network Partitions
- Split-brain scenarios
- Consensus protocols
- Partition tolerance

### 3. Planned Maintenance
- Graceful node shutdown
- Rolling upgrades
- Zero-downtime updates

### 4. Geographic Failover
- Cross-datacenter redundancy
- Regional failures
- Global load balancing

---

## 📖 References

- NestGate Federation Spec: `../../specs/FEDERATION_ARCHITECTURE.md`
- High Availability Patterns
- Consensus Algorithms (Raft, Paxos)
- CAP Theorem

---

**Next**: Try **05_distributed_snapshot** for coordinated snapshots across the mesh!

