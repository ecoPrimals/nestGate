# 🌐 Two-Node Mesh - Basic Federation

**Time**: 10 minutes  
**Complexity**: Intermediate  
**Prerequisites**: Basic networking knowledge

---

## 🎯 WHAT YOU'LL LEARN

- ✅ How mDNS discovery works
- ✅ Automatic peer connection
- ✅ Bi-directional mesh networking
- ✅ Health monitoring and heartbeats

**After this demo**: You'll understand NestGate's discovery mechanism

---

## 🚀 QUICK START

```bash
./demo-mesh.sh
```

**Expected duration**: ~10 minutes  
**Expected result**: Two nodes connected in mesh

---

## 📋 WHAT HAPPENS

### **Phase 1: Node 1 Startup**
```
Starting Node 1...
  ✅ Bind to localhost:8080 (API)
  ✅ Bind to localhost:8081 (RPC)
  ✅ Announce via mDNS: "nestgate-node-1"
  ✅ Scan network for peers...
  ⚠️  No peers found
  Status: Standalone mode (OK)
```

### **Phase 2: Node 2 Startup**
```
Starting Node 2...
  ✅ Bind to localhost:8082 (API)
  ✅ Bind to localhost:8083 (RPC)
  ✅ Announce via mDNS: "nestgate-node-2"
  ✅ Scan network for peers...
  ✅ Found: nestgate-node-1 (localhost:8080)
```

### **Phase 3: Mesh Formation**
```
Node 2 → Node 1:
  Connecting to nestgate-node-1...
  ✅ TCP connection established
  ✅ TLS handshake complete
  ✅ Capability exchange
  ✅ Node registered
  
Node 1 → Node 2:
  Peer connected: nestgate-node-2
  ✅ Bi-directional link established
  
Mesh Status:
  Nodes: 2
  Connections: 1 (2 ↔ 1)
  Time to mesh: 847ms
```

### **Phase 4: Health Monitoring**
```
Heartbeat monitoring active:
  Node 1 → Node 2: ping (2ms)
  Node 2 → Node 1: pong (2ms)
  
  Node 1 → Node 2: ping (2ms)
  Node 2 → Node 1: pong (2ms)
  
  Status: Both nodes healthy
  Uptime: 60 seconds
```

---

## 💡 HOW IT WORKS

### **mDNS Discovery**

**What is mDNS?**
- Multicast DNS
- Zero-configuration networking
- No DNS server required
- Local network only (.local domain)

**How NestGate uses it:**
```rust
// Announce service
service = "_nestgate._tcp.local"
name = "nestgate-node-1"
port = 8080
txt_records = [
  "version=1.0.0",
  "capabilities=zfs,api,rpc",
  "node_id=abc123"
]

// Broadcast every 60 seconds
// Peers discover via multicast
```

### **Connection Establishment**

**Step 1: Discovery**
```
Node 2 hears mDNS broadcast from Node 1
Node 2 extracts: hostname, port, capabilities
```

**Step 2: Connection**
```
Node 2 → Node 1: TCP SYN (port 8081)
Node 1 → Node 2: TCP SYN-ACK
Node 2 → Node 1: TCP ACK
Connection established
```

**Step 3: Handshake**
```
Node 2 → Node 1: HELLO (node_id, capabilities)
Node 1 → Node 2: WELCOME (node_id, capabilities)
Node 2 → Node 1: ACK
Mesh link active
```

### **Health Monitoring**

**Heartbeat Protocol:**
```
Every 5 seconds:
  Sender: PING (timestamp, sequence)
  Receiver: PONG (timestamp, sequence)
  
If no PONG after 3 attempts (15s):
  Mark peer as DOWN
  Remove from active mesh
  Attempt reconnection every 30s
```

---

## 🔬 DISCOVERY DEEP DIVE

### **Why mDNS?**

**Alternatives**:
1. **Manual Configuration**: Brittle, error-prone
2. **Central Registry** (Consul/etcd): Single point of failure
3. **DNS**: Requires infrastructure
4. **mDNS**: Zero-config, resilient ✅

**mDNS Benefits**:
- No configuration files
- Works on any local network
- Automatic updates
- Broadcasts and discovers simultaneously

### **Discovery Packet Format**

```
mDNS Packet:
  Type: PTR (pointer record)
  Name: _nestgate._tcp.local
  TTL: 120 seconds
  
  SRV Record:
    Priority: 0
    Weight: 0
    Port: 8080
    Target: nestgate-node-1.local
    
  TXT Records:
    version=1.0.0
    capabilities=zfs,api,rpc
    node_id=abc123
    
  A Record:
    nestgate-node-1.local → 192.168.1.100
```

**Size**: ~200 bytes per announcement

---

## 📊 PERFORMANCE METRICS

### **Discovery Latency**
```
Node startup → mDNS broadcast: 100-200ms
Broadcast → peer reception: 10-50ms
Reception → connection: 50-100ms
Connection → mesh active: 200-500ms

Total: ~500-850ms
```

### **Heartbeat Overhead**
```
Packet size: 64 bytes
Frequency: Every 5 seconds
Bandwidth: 12.8 bytes/sec per connection

Negligible overhead!
```

### **Connection Limits**
```
Theoretical: 1000+ connections
Practical: 100-200 connections
Recommended: 5-10 connections (full mesh)
```

---

## 🎓 KEY CONCEPTS

### **1. Full Mesh = Maximum Resilience**

**With 2 nodes**:
```
Node 1 ←→ Node 2

If link fails:
  Both nodes detect within 15s
  Both revert to standalone
  No data loss
  Automatic reconnection
```

**Why this matters**: No single point of failure

### **2. Bi-directional Communication**

**Traditional**:
```
Client → Server (one way)
If server down: Client fails
```

**Mesh**:
```
Node 1 ↔ Node 2 (both ways)
Either can initiate
Either can respond
True peer-to-peer
```

### **3. Sovereignty Preserved**

**Each node can**:
- Operate standalone
- Accept connections
- Initiate connections
- Leave mesh anytime
- Rejoin mesh anytime

**No node is "special"**: True sovereignty

---

## 🔬 TRY IT YOURSELF

### Experiment 1: Watch Discovery
```bash
# In terminal 1:
./demo-mesh.sh

# Watch Node 1 start
# Watch Node 2 discover Node 1
# Watch mesh formation
```

### Experiment 2: Simulate Failure
```bash
# Start demo
./demo-mesh.sh

# Wait for mesh to form
# Kill Node 1 (Ctrl+C in Node 1 terminal)
# Watch Node 2 detect failure
# Watch Node 2 attempt reconnection
```

### Experiment 3: Manual mDNS Query
```bash
# Query for NestGate services
avahi-browse -rt _nestgate._tcp

# Or with dns-sd
dns-sd -B _nestgate._tcp local
```

---

## 💬 WHAT YOU'LL SEE

```
🌐 NestGate Two-Node Mesh Demo
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Starting simulation...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 1: Node 1 Startup
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Node 1 starting...
  ✅ Bound to localhost:8080 (API)
  ✅ Bound to localhost:8081 (RPC)
  ✅ Announced via mDNS: nestgate-node-1
  ✅ Scanning for peers...
  ⚠️  No peers found
  
Status: STANDALONE
Waiting for peers...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 2: Node 2 Startup
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Node 2 starting...
  ✅ Bound to localhost:8082 (API)
  ✅ Bound to localhost:8083 (RPC)
  ✅ Announced via mDNS: nestgate-node-2
  ✅ Scanning for peers...
  ✅ Found: nestgate-node-1 (localhost:8080)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 3: Mesh Formation
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Node 2 → Node 1:
  Initiating connection to nestgate-node-1...
  ✅ TCP connection established (3ms)
  ✅ TLS handshake complete (45ms)
  ✅ Capability exchange complete (12ms)
  ✅ Node registered in mesh

Node 1 → Node 2:
  Peer connected: nestgate-node-2
  ✅ Bi-directional link established
  ✅ Mesh activated

Mesh Status:
  Nodes: 2
  Connections: 1 (1 ↔ 2)
  Formation time: 847ms

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 4: Health Monitoring (30 seconds)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[00:00] Node 1 → Node 2: PING (2ms) ✅
[00:00] Node 2 → Node 1: PONG (2ms) ✅

[00:05] Node 1 → Node 2: PING (2ms) ✅
[00:05] Node 2 → Node 1: PONG (2ms) ✅

[00:10] Node 1 → Node 2: PING (2ms) ✅
[00:10] Node 2 → Node 1: PONG (2ms) ✅

... (continues for 30 seconds)

✅ Demo Complete!

Summary:
  Mesh formed: 847ms
  Health checks: 6/6 successful
  Latency avg: 2.1ms
  Status: HEALTHY

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🎓 What you learned:
   ✅ Automatic peer discovery (mDNS)
   ✅ Zero-configuration mesh
   ✅ Bi-directional communication
   ✅ Health monitoring

⏭️  Next: ZFS Replication
   cd ../02-replication && ./demo-zfs-replication.sh
```

---

**Ready to see mesh networking in action?** Run `./demo-mesh.sh`!

🌐 **Mesh: The foundation of federation!** 🌐

