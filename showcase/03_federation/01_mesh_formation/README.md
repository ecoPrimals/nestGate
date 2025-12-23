# Demo 3.1: Mesh Formation

**Level**: 3 (Federation)  
**Time**: 30 minutes  
**Complexity**: Medium  
**Status**: 🚧 Building (Week 3)

---

## 🎯 **WHAT THIS DEMO SHOWS**

This demo demonstrates how multiple NestGate nodes automatically discover each other and form a mesh:

1. **Zero-Configuration Discovery** - Nodes find each other automatically
2. **Gossip Protocol** - Peer-to-peer state sharing
3. **Mesh Topology** - Fully connected network
4. **Health Monitoring** - Continuous node health checks
5. **Dynamic Membership** - Nodes join/leave gracefully

**Key Concept**: Distributed systems without central coordination

---

## 🚀 **QUICK RUN**

```bash
# Start 3 NestGate nodes
docker-compose -f mesh-demo.yml up -d

# Run the demo
./demo.sh

# Expected runtime: ~5 minutes
```

---

## 📋 **WHAT YOU'LL SEE**

### Part 1: Node Startup
```bash
# Terminal 1: Node A starts
nestgate-server --node-id node-a --port 8080

→ NestGate Node A starting...
  Node ID: node-a
  Listen: 0.0.0.0:8080
  Gossip: 0.0.0.0:9080
  Status: STANDALONE
  
  Advertising capabilities:
    - storage (100 GB available)
    - snapshots (ZFS backend)
    - replication (configured)
  
  Waiting for peers...

# Terminal 2: Node B starts
nestgate-server --node-id node-b --port 8081

→ NestGate Node B starting...
  Node ID: node-b
  Listen: 0.0.0.0:8081
  Gossip: 0.0.0.0:9081
  
  Discovery: Found peer node-a at 192.168.1.10:8080
  Gossip: Connecting to node-a
  Handshake: Success ✓
  
  Mesh Status: 2 nodes (node-a, node-b)

# Terminal 3: Node C starts
nestgate-server --node-id node-c --port 8082

→ NestGate Node C starting...
  Node ID: node-c
  Listen: 0.0.0.0:8082
  Gossip: 0.0.0.0:9082
  
  Discovery: Found peers
    - node-a at 192.168.1.10:8080
    - node-b at 192.168.1.11:8081
  
  Gossip: Connecting to all peers
  Handshakes: All successful ✓
  
  Mesh Status: 3 nodes (node-a, node-b, node-c)
```

### Part 2: Mesh Formation
```bash
# Query mesh status from any node
curl http://localhost:8080/api/v1/cluster/status

→ Cluster Status:
  Mesh ID: mesh_abc123
  Topology: Full mesh
  Total Nodes: 3
  Healthy Nodes: 3
  
  Nodes:
    - node-a (192.168.1.10:8080)
      Status: Leader
      Uptime: 45s
      Health: Healthy ✓
      Connections: 2 peers
      
    - node-b (192.168.1.11:8081)
      Status: Follower
      Uptime: 30s
      Health: Healthy ✓
      Connections: 2 peers
      
    - node-c (192.168.1.12:8082)
      Status: Follower
      Uptime: 15s
      Health: Healthy ✓
      Connections: 2 peers
  
  Gossip Protocol:
    Messages/sec: 12
    Convergence time: 1.2s
    Network overhead: 0.3%
```

### Part 3: Gossip Protocol in Action
```bash
# Watch gossip messages
nestgate-cli gossip watch

→ Gossip Stream:

[14:45:00.123] node-a → node-b: HEARTBEAT
  Sequence: 1234
  State: healthy
  Connections: 2

[14:45:00.456] node-b → node-c: HEARTBEAT
  Sequence: 567
  State: healthy
  Connections: 2

[14:45:00.789] node-c → node-a: HEARTBEAT
  Sequence: 890
  State: healthy
  Connections: 2

[14:45:01.123] node-a → ALL: STATE_UPDATE
  Event: New data available
  Path: /datasets/new-data
  Size: 2.4 GB
  
[14:45:01.234] node-b: Received STATE_UPDATE from node-a
[14:45:01.345] node-c: Received STATE_UPDATE from node-a

  ✓ State converged across all nodes (122ms)
```

### Part 4: Node Addition
```bash
# Add a 4th node dynamically
nestgate-server --node-id node-d --port 8083

→ Node D starting...
  
  Discovery phase:
    Scanning network... ✓
    Found 3 existing nodes
    
  Joining mesh:
    node-a: Handshake ✓
    node-b: Handshake ✓
    node-c: Handshake ✓
    
  Synchronizing state:
    Receiving cluster metadata... ✓
    Receiving data catalog... ✓
    Receiving configuration... ✓
    
  Mesh updated:
    Total nodes: 4
    Status: Healthy
    Convergence: 2.3s
    
# All nodes automatically updated
curl http://localhost:8080/api/v1/cluster/status

→ Total Nodes: 4 ✓
  All nodes see node-d
  Full mesh reformed automatically
```

### Part 5: Health Monitoring
```bash
# Continuous health checking
nestgate-cli health watch --interval 5s

→ Health Monitor Active:

[14:46:00] Checking all nodes...
  node-a: ✓ Healthy (latency: 1.2ms)
  node-b: ✓ Healthy (latency: 1.5ms)
  node-c: ✓ Healthy (latency: 1.3ms)
  node-d: ✓ Healthy (latency: 1.4ms)

[14:46:05] Checking all nodes...
  node-a: ✓ Healthy (latency: 1.1ms)
  node-b: ✓ Healthy (latency: 1.6ms)
  node-c: ⚠ Slow response (latency: 45ms)
  node-d: ✓ Healthy (latency: 1.3ms)

[14:46:10] Checking all nodes...
  node-a: ✓ Healthy (latency: 1.2ms)
  node-b: ✓ Healthy (latency: 1.4ms)
  node-c: ✗ Unhealthy (timeout)
  node-d: ✓ Healthy (latency: 1.5ms)
  
  ⚠ node-c marked as suspect
  Initiating failure detection...
```

---

## 💡 **WHY MESH TOPOLOGY**

### The Problem: Central Coordination

**Traditional Approach** (Centralized):
```
      Master Node
     /     |     \
   N1     N2     N3

Problems:
- Single point of failure (master)
- Bottleneck (all through master)
- Complexity (master logic)
```

---

### The Solution: Mesh Network

**NestGate Approach** (Decentralized):
```
   N1 ←→ N2 ←→ N3
    ↕      ↕     ↕
   N4 ←→ N5 ←→ N6

Benefits:
- No single point of failure
- No bottleneck (peer-to-peer)
- Simple (gossip protocol)
- Scalable (O(log n) messages)
```

---

## 🏗️ **ARCHITECTURE**

### Mesh Formation Process

```
┌────────────────────────────────────────────────┐
│         Mesh Formation Workflow                │
├────────────────────────────────────────────────┤
│                                                │
│  1. Node starts                                │
│     ↓ Advertises on mDNS                      │
│     ↓ Listens for peers                       │
│                                                │
│  2. Discovery                                  │
│     ↓ Finds existing nodes (mDNS/seeds)       │
│     ↓ Initiates connections                    │
│                                                │
│  3. Handshake                                  │
│     ↓ Exchange node IDs                        │
│     ↓ Exchange capabilities                    │
│     ↓ Verify compatibility                     │
│                                                │
│  4. State Sync                                 │
│     ↓ Receive cluster metadata                 │
│     ↓ Receive data catalog                     │
│     ↓ Receive configuration                    │
│                                                │
│  5. Gossip                                     │
│     ↓ Start sending heartbeats                 │
│     ↓ Share state updates                      │
│     ↓ Monitor peer health                      │
│                                                │
│  6. Mesh Complete                              │
│     ✓ Fully connected                          │
│     ✓ State synchronized                       │
│     ✓ Ready for operations                     │
│                                                │
└────────────────────────────────────────────────┘
```

### Gossip Protocol

```
Every N seconds (configurable):
  1. Select random peer
  2. Send: My state + Known peers
  3. Receive: Peer state + Their known peers
  4. Merge: Update local knowledge
  5. Repeat

Result: Eventually consistent view across all nodes
Time: O(log n) rounds for full convergence
Overhead: Minimal (<1% bandwidth)
```

---

## 🧪 **EXPERIMENTS TO TRY**

### Experiment 1: Watch Convergence
```bash
# Terminal 1: Start monitoring
watch -n 1 'curl -s localhost:8080/api/v1/cluster/status | jq .nodes | wc -l'

# Terminal 2: Add nodes one by one
for i in {1..5}; do
  nestgate-server --node-id node-$i --port $((8080+i)) &
  sleep 2
done

# Watch the count increase in Terminal 1
# Observe convergence time
```

### Experiment 2: Partition Recovery
```bash
# Create 6-node mesh
# Then: Block traffic between groups

# Group A: nodes 1,2,3
iptables -A INPUT -s 192.168.1.14 -j DROP  # Block node 4
iptables -A INPUT -s 192.168.1.15 -j DROP  # Block node 5
iptables -A INPUT -s 192.168.1.16 -j DROP  # Block node 6

# Now you have two partitions
# Observe: Each group maintains internal consistency

# Restore connectivity
iptables -F  # Flush rules

# Watch: Partitions merge, state reconciles
```

### Experiment 3: Large Mesh
```bash
# Start 20 nodes
for i in {1..20}; do
  nestgate-server --node-id node-$i --port $((8080+i)) &
done

# Measure:
# - Time to full mesh formation
# - Gossip message rate
# - Network overhead
# - Convergence time for updates
```

---

## 📊 **PERFORMANCE CHARACTERISTICS**

### Mesh Formation Time

**Small Mesh** (3-5 nodes):
```
Discovery: 100-500ms
Handshakes: 50-100ms per peer
State sync: 200-500ms
Total: ~1-2 seconds
```

**Medium Mesh** (10-20 nodes):
```
Discovery: 200-800ms
Handshakes: ~2-4 seconds total
State sync: 500-1000ms
Total: ~3-5 seconds
```

**Large Mesh** (50+ nodes):
```
Discovery: 500-1500ms
Handshakes: ~10-15 seconds
State sync: 1-3 seconds
Total: ~12-20 seconds
```

### Gossip Overhead

```
3 nodes:   ~12 messages/sec,  <0.1% bandwidth
10 nodes:  ~50 messages/sec,  ~0.3% bandwidth
20 nodes:  ~120 messages/sec, ~0.5% bandwidth
50 nodes:  ~350 messages/sec, ~1.2% bandwidth
```

**Conclusion**: Scales logarithmically, overhead stays low

---

## 🆘 **TROUBLESHOOTING**

### "Nodes can't discover each other"
**Cause**: Network/mDNS issues  
**Fix**:
```bash
# Check mDNS is running
systemctl status avahi-daemon

# Test mDNS discovery
avahi-browse -a

# Use manual seeds if needed
nestgate-server --seeds "node1:8080,node2:8081"
```

### "Mesh forms but nodes disconnect"
**Cause**: Firewall or health check issues  
**Fix**:
```bash
# Check firewall rules
sudo iptables -L

# Open gossip ports
sudo ufw allow 9080:9090/tcp

# Adjust health check timeout
export NESTGATE_HEALTH_TIMEOUT=10s
```

### "Slow convergence"
**Cause**: Network latency or too many nodes  
**Fix**:
```bash
# Increase gossip frequency
export NESTGATE_GOSSIP_INTERVAL=1s

# Increase fanout (more peers per round)
export NESTGATE_GOSSIP_FANOUT=3

# Use dedicated gossip network if possible
```

---

## 📚 **LEARN MORE**

**Gossip Protocols**:
- SWIM Protocol: https://www.cs.cornell.edu/projects/Quicksilver/public_pdfs/SWIM.pdf
- Epidemic Algorithms: https://dl.acm.org/doi/10.1145/41840.41841

**Mesh Networks**:
- Mesh Topology: `../../../docs/architecture/MESH_TOPOLOGY.md`
- Gossip Implementation: `../../../code/crates/nestgate-gossip/`

**Distributed Systems**:
- CAP Theorem: `../../../docs/architecture/CAP_TRADEOFFS.md`
- Failure Detection: `../../../docs/architecture/FAILURE_DETECTION.md`

---

## ⏭️ **WHAT'S NEXT**

**Completed Demo 3.1?** Great! You now understand:
- ✅ Mesh topology
- ✅ Gossip protocol
- ✅ Zero-configuration discovery
- ✅ Peer-to-peer coordination

**Ready for Demo 3.2?** (`../02_distributed_storage/`)
- Data distribution across nodes
- Consistent hashing
- Sharding strategies

**Or explore more mesh**:
- Add/remove nodes dynamically
- Test with network partitions
- Measure convergence times
- Scale to 50+ nodes

---

**Status**: 🚧 Building (Week 3)  
**Estimated time**: 30 minutes  
**Prerequisites**: Levels 1 & 2 complete

**Key Takeaway**: Distributed coordination without central control! 🌐

---

*Demo 3.1 - Mesh Formation*  
*Part of Level 3: Federation*  
*Building Week 3 - December 2025*

