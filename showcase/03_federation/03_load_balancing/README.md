# Load Balancing Across NestGate Mesh

**Level**: 3 - Federation  
**Complexity**: Intermediate  
**Time**: 10 minutes  
**Prerequisites**: Multiple NestGate instances running

---

## 🎯 Purpose

Demonstrate how a NestGate mesh distributes incoming requests across multiple nodes for:
- Improved throughput
- Better resource utilization
- Horizontal scalability
- Reduced latency

---

## 🏗️ Architecture

```
                    Load Balancer
                         │
            ┌────────────┼────────────┐
            │            │            │
       ┌────▼───┐   ┌────▼───┐   ┌────▼───┐
       │ Node 1 │   │ Node 2 │   │ Node 3 │
       │Port8080│   │Port8081│   │Port8082│
       └────┬───┘   └────┬───┘   └────┬───┘
            │            │            │
            └────────────┴────────────┘
                   Shared State
```

### Load Balancing Strategies

1. **Round Robin**: Distribute requests evenly
2. **Least Connections**: Route to least busy node
3. **Response Time**: Route to fastest node
4. **Capability-Based**: Route by node capabilities
5. **Geographic**: Route by proximity

---

## 🚀 What This Demo Shows

### 1. Multiple Node Setup
- 3 NestGate instances on different ports
- Each with its own storage pool
- Coordinated via mesh protocol

### 2. Request Distribution
- Health checks distributed
- Data operations balanced
- Metrics collection from all nodes

### 3. Performance Benefits
- 3x throughput with 3 nodes
- Reduced per-node load
- Graceful handling of load spikes

### 4. Monitoring
- Per-node request counts
- Response time tracking
- Load distribution visualization

---

## 📋 Prerequisites

### System Requirements
- 3 available ports (8080, 8081, 8082)
- Sufficient memory for 3 instances
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

1. **Node Startup** (30 seconds)
   - Start 3 NestGate instances
   - Each on different port
   - Form mesh automatically

2. **Load Balancer Setup**
   - Configure HAProxy or similar
   - Register all 3 backends
   - Enable health checks

3. **Load Generation** (60 seconds)
   - Send 300 requests
   - Distributed across nodes
   - Monitor distribution

4. **Results Display**
   - Per-node request counts
   - Average response times
   - Load distribution graph

5. **Cleanup**
   - Stop all nodes
   - Clean up state

---

## 📊 Expected Results

### Request Distribution (Round Robin)
```
Node 1 (8080): 100 requests (33.3%)
Node 2 (8081): 100 requests (33.3%)
Node 3 (8082): 100 requests (33.4%)

Total: 300 requests
Distribution: Even (±1%)
```

### Performance Metrics
```
Single Node Baseline: 200 req/s
3-Node Mesh:          600 req/s (3x improvement)

Per-Node Load:        200 req/s (optimal)
Latency:              ~5ms (unchanged)
Throughput:           3x increase
```

### Load Balancer Health
```
Backend 1: UP
Backend 2: UP
Backend 3: UP

Health Check: 3/3 passing
Failover Ready: Yes
```

---

## 🔍 What to Look For

### 1. Even Distribution
- Requests split evenly across nodes
- No single node overwhelmed
- Balanced resource usage

### 2. Performance Scaling
- Linear throughput scaling with nodes
- Consistent per-request latency
- No contention bottlenecks

### 3. Health Monitoring
- Regular health checks
- Automatic backend updates
- Failed node detection

### 4. Coordination
- Mesh state synchronized
- All nodes aware of each other
- Consistent capability advertising

---

## 💡 Key Concepts

### 1. Horizontal Scaling
- Add nodes to increase capacity
- Linear throughput scaling
- No single point of failure

### 2. Load Distribution Algorithms
- **Round Robin**: Simple, predictable
- **Least Connections**: Dynamic load balancing
- **Response Time**: Performance-based routing
- **Consistent Hashing**: Session affinity

### 3. Health Checking
- Active: Periodic health probes
- Passive: Monitor request failures
- Adaptive: Adjust based on response times

### 4. Backend Management
- Dynamic backend registration
- Automatic removal of failed nodes
- Weight-based routing

---

## 🎓 Learning Outcomes

After this demo, you'll understand:

1. ✅ How to distribute load across NestGate nodes
2. ✅ Performance benefits of horizontal scaling
3. ✅ Different load balancing strategies
4. ✅ Health checking and failover preparation
5. ✅ Monitoring distributed request handling

---

## 🔗 Related Demos

### Previous
- **02_replication**: Data distribution across nodes
- **01_mesh_formation**: How nodes discover each other

### Next
- **04_failover**: What happens when a node fails
- **05_distributed_snapshot**: Coordinated snapshots

### Integration
- **04_inter_primal_mesh/01_simple_coordination**: Multi-primal load balancing

---

## 🛠️ Customization

### Change Load Balancing Algorithm
```bash
# Edit demo.sh
LOAD_BALANCE_ALGORITHM="least_connections"
# Options: round_robin, least_connections, response_time, hash
```

### Adjust Node Count
```bash
# Edit demo.sh
NODE_COUNT=5  # Start 5 nodes instead of 3
```

### Increase Load
```bash
# Edit demo.sh
REQUEST_COUNT=1000  # Send 1000 requests instead of 300
```

---

## 📈 Performance Expectations

### Scaling Characteristics
```
1 Node:  200 req/s
2 Nodes: 400 req/s (2.0x)
3 Nodes: 600 req/s (3.0x)
4 Nodes: 800 req/s (4.0x)
5 Nodes: 950 req/s (4.75x - slight overhead)

Scaling Efficiency: 95%
Network Overhead: ~5% at 5 nodes
```

### Latency Impact
```
Single Node:     5ms
With LB (3 nodes): 5.5ms (+0.5ms overhead)

Load Balancer Overhead: ~500μs
Network Hop: Negligible (same host)
```

---

## 🐛 Troubleshooting

### Issue: Uneven Distribution
**Symptom**: One node receives more requests  
**Solution**: Check load balancer algorithm, verify health checks

### Issue: High Latency
**Symptom**: Requests slower than single node  
**Solution**: Check network, verify nodes not overloaded

### Issue: Node Not Receiving Requests
**Symptom**: One node at 0 requests  
**Solution**: Check health check status, verify port accessibility

---

## 📚 Technical Details

### Load Balancer Configuration (HAProxy Example)
```
frontend nestgate_lb
    bind *:8080
    default_backend nestgate_nodes

backend nestgate_nodes
    balance roundrobin
    option httpchk GET /health
    server node1 127.0.0.1:8081 check
    server node2 127.0.0.1:8082 check
    server node3 127.0.0.1:8083 check
```

### Session Affinity (Optional)
```
# Stick to same backend for session
stick-table type ip size 1m expire 30m
stick on src
```

---

## 🎯 Use Cases

### 1. High-Traffic Data Services
- Many clients accessing data
- Distribute read operations
- Scale horizontally

### 2. Multi-Tenant Environments
- Isolate tenants across nodes
- Balance based on tenant activity
- Prevent noisy neighbors

### 3. Geographic Distribution
- Route to nearest node
- Reduce latency
- Comply with data locality

### 4. Workload Separation
- Route different operations to different nodes
- Specialize nodes for specific workloads
- Optimize resource usage

---

## 🔮 Future Enhancements

1. **Intelligent Routing**
   - ML-based load prediction
   - Adaptive algorithms
   - Workload-aware routing

2. **Auto-Scaling**
   - Spawn nodes based on load
   - Automatic deregistration
   - Cost optimization

3. **Advanced Health Checks**
   - Application-level checks
   - Custom health metrics
   - Predictive failure detection

4. **Global Load Balancing**
   - Cross-datacenter routing
   - DNS-based load balancing
   - Geographic optimization

---

## 📖 References

- [HAProxy Load Balancing Guide](https://www.haproxy.org/documentation/)
- [Nginx Load Balancing](https://nginx.org/en/docs/http/load_balancing.html)
- NestGate Architecture: `../../specs/SPECS_MASTER_INDEX.md`
- Mesh Protocol: `../../specs/FEDERATION_ARCHITECTURE.md`

---

**Next**: Try **04_failover** to see what happens when a node goes down!

