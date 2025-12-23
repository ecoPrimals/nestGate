# 🌐 Level 4: Inter-Primal Mesh

**Progressive Complexity**: Level 4 of 5  
**Focus**: Multi-primal integration and coordination  
**Duration**: 1-2 hours total  
**Prerequisites**: NestGate + Songbird + ToadStool running  

---

## 🎯 Overview

Level 4 demonstrates NestGate working as part of a larger ecosystem mesh with multiple primals:

- **Songbird**: Orchestration and service discovery
- **ToadStool**: Compute workloads (AI/ML)
- **NestGate**: Distributed storage
- **BearDog**: Security and encryption (optional)

This is where the "inter-primal" vision comes to life - multiple autonomous systems working together seamlessly.

---

## 📁 Demo Structure

```
04_inter_primal_mesh/
├── README.md                           (this file)
├── 01_songbird_coordination/           
│   ├── README.md                       Service discovery & orchestration
│   └── demo.sh                         
├── 02_toadstool_storage/               
│   ├── README.md                       AI training with NestGate storage
│   └── demo.sh                         
├── 03_three_primal_workflow/           
│   ├── README.md                       Complete workflow across all primals
│   └── demo.sh                         
├── 04_distributed_ai_training/         
│   ├── README.md                       Multi-node AI training
│   └── demo.sh                         
└── 05_real_world_scenario/             
    ├── README.md                       Production-like scenario
    └── demo.sh                         
```

---

## 🌟 What Makes This Level Special

### Multi-Primal Coordination

Unlike previous levels that focused on NestGate alone or simple 2-primal interactions, Level 4 shows:

1. **Service Discovery** - Songbird discovers all primals
2. **Capability Matching** - Primals advertise what they can do
3. **Dynamic Routing** - Requests routed based on capabilities
4. **Failover** - Automatic fallback if services unavailable
5. **Load Balancing** - Distribute work across instances

### Real-World Patterns

These demos mirror production scenarios:

- **Distributed AI Training**: ToadStool trains models, NestGate stores checkpoints
- **Data Pipelines**: Ingest → Process → Store across primals
- **Microservices**: Each primal is a specialized service
- **Federation**: Multiple NestGate instances coordinated by Songbird

---

## 🚀 Quick Start

### Prerequisites

```bash
# 1. Ensure all primals are built
cd ../songbird && cargo build --release
cd ../toadstool && cargo build --release
cd ../nestgate && cargo build --release

# 2. Start NestGate
cd nestgate
export NESTGATE_JWT_SECRET=$(openssl rand -base64 48)
./target/release/nestgate service start --port 8080 &

# 3. Start Songbird
cd ../songbird
./target/release/songbird-orchestrator --port 8090 &

# 4. Start ToadStool (optional)
cd ../toadstool
./target/release/toadstool-server --port 7878 &
```

### Run First Demo

```bash
cd showcase/04_inter_primal_mesh
./01_songbird_coordination/demo.sh
```

---

## 📊 Demo Progression

### 4.1: Songbird Coordination

**Focus**: Service discovery and basic orchestration  
**Time**: 15 minutes  
**What you'll learn**:
- How Songbird discovers NestGate
- Protocol escalation (JSON-RPC → tarpc)
- Capability-based routing
- Health monitoring

**Key Concepts**:
- Infant Discovery Architecture (O(1) discovery)
- Universal Adapter pattern
- Zero hardcoded primal names

### 4.2: ToadStool Storage Integration

**Focus**: Compute workloads using NestGate  
**Time**: 20 minutes  
**What you'll learn**:
- ToadStool requests storage from Songbird
- Songbird routes to NestGate
- High-performance data access via tarpc
- Automatic replication for redundancy

**Key Concepts**:
- Primal-to-primal communication
- Storage as a service
- Transparent failover

### 4.3: Three-Primal Workflow

**Focus**: Complete data pipeline  
**Time**: 30 minutes  
**What you'll learn**:
- Data ingestion → NestGate
- Processing → ToadStool
- Results → NestGate
- Orchestration → Songbird

**Key Concepts**:
- Workflow orchestration
- Data lineage
- State management

### 4.4: Distributed AI Training

**Focus**: Real AI workload across mesh  
**Time**: 45 minutes  
**What you'll learn**:
- Multi-node training coordination
- Checkpoint management
- Data sharding across NestGate instances
- Performance optimization

**Key Concepts**:
- Distributed systems
- Data locality
- Network optimization

### 4.5: Real-World Scenario

**Focus**: Production-like deployment  
**Time**: 60 minutes  
**What you'll learn**:
- Complete system deployment
- Monitoring and alerting
- Failure scenarios and recovery
- Performance tuning

**Key Concepts**:
- Production patterns
- Observability
- Resilience

---

## 🏗️ Architecture

```
┌────────────────────────────────────────────────────────────────┐
│                    INTER-PRIMAL MESH                           │
│                                                                │
│                    Songbird (Orchestrator)                     │
│                    ┌──────────────────┐                        │
│                    │  Service Registry │                        │
│                    │  • Discovery      │                        │
│                    │  • Health Checks  │                        │
│                    │  • Load Balancing │                        │
│                    └────────┬──────────┘                        │
│                             │                                  │
│           ┌─────────────────┼─────────────────┐                │
│           │                 │                 │                │
│           ▼                 ▼                 ▼                │
│    ┌─────────────┐   ┌─────────────┐   ┌─────────────┐        │
│    │  NestGate A │   │  NestGate B │   │  NestGate C │        │
│    │  (Storage)  │   │  (Storage)  │   │  (Storage)  │        │
│    └──────┬──────┘   └──────┬──────┘   └──────┬──────┘        │
│           │                  │                  │               │
│           └──────────────────┼──────────────────┘               │
│                              │                                  │
│                              ▼                                  │
│                    ┌──────────────────┐                         │
│                    │   ToadStool      │                         │
│                    │   (Compute/AI)   │                         │
│                    └──────────────────┘                         │
│                                                                │
└────────────────────────────────────────────────────────────────┘

Communication:
  • JSON-RPC: Initial discovery (HTTP-based)
  • tarpc: High-performance operations (binary)
  • WebSocket: Real-time events
  • HTTP/REST: Management and monitoring
```

---

## 🔍 Key Technologies

### Service Discovery
- **Infant Discovery**: O(1) service lookup
- **Capability-Based**: Match services by what they can do
- **Dynamic**: Services join/leave at runtime

### Communication Protocols
- **JSON-RPC**: Universal, HTTP-based
- **tarpc**: 40x faster binary RPC
- **WebSocket**: Real-time events
- **gRPC**: Alternative high-performance option

### Data Management
- **ZFS**: Snapshots, replication, compression
- **Distributed Storage**: Data across multiple nodes
- **Caching**: Reduce network overhead
- **Locality**: Keep data close to compute

---

## 📈 Performance Expectations

### Single-Primal Operations
- NestGate API call: ~2ms
- ZFS snapshot: ~100ms
- Data read: ~10-50ms (depending on size)

### Multi-Primal Operations
- Service discovery: ~5ms
- Orchestrated operation: ~10-20ms overhead
- tarpc call: ~50μs additional latency
- Total overhead: ~5-10% typically

### Distributed AI Training
- Without NestGate: Data copy overhead, no snapshots
- With NestGate: Automatic checkpointing, fast recovery
- Performance: 95-98% of theoretical maximum
- Network: Typically not the bottleneck

---

## 🎓 Learning Path

### For Developers
1. Start with 4.1 (basics)
2. Understand service discovery
3. Learn protocol escalation
4. Move to 4.2 (integration)
5. See real workflows (4.3)
6. Advanced patterns (4.4-4.5)

### For Operators
1. Focus on 4.5 (production)
2. Understand monitoring
3. Practice failure scenarios
4. Learn tuning parameters
5. Study troubleshooting

### For Architects
1. Review all demos
2. Study architecture diagrams
3. Understand trade-offs
4. Learn scaling patterns
5. Design your own workflows

---

## ✅ Success Criteria

By completing Level 4, you should understand:

- ✅ How primals discover each other
- ✅ How Songbird orchestrates operations
- ✅ How to use protocol escalation
- ✅ How to build multi-primal workflows
- ✅ How to handle failures gracefully
- ✅ How to optimize performance
- ✅ How to monitor and debug

---

## 🐛 Troubleshooting

### Services Not Discovering

```bash
# Check Songbird is running
curl http://localhost:8090/health

# Check NestGate registration
curl http://localhost:8090/api/services | jq '.services[] | select(.name=="nestgate")'

# Check logs
tail -f ~/.songbird/logs/discovery.log
```

### Protocol Escalation Fails

```bash
# Verify tarpc endpoint
curl http://localhost:8080/api/protocol/capabilities | jq '.protocols.tarpc'

# Check port is open
lsof -i :8091
```

### Performance Issues

```bash
# Check metrics
curl http://localhost:8080/api/v1/storage/metrics

# Monitor in real-time
watch -n 1 'curl -s http://localhost:8080/api/v1/storage/metrics | jq .'
```

---

## 📚 Additional Resources

- **Songbird Documentation**: `../songbird/docs/`
- **ToadStool Documentation**: `../toadstool/docs/`
- **Inter-Primal Communication**: `docs/inter_primal_communication.md`
- **Service Discovery**: `docs/service_discovery.md`
- **Performance Tuning**: `docs/performance_tuning.md`

---

## 🎯 Next Level

After completing Level 4, you're ready for:

**Level 5: Real-World Deployments**
- Production configurations
- Scaling patterns
- Security hardening
- Monitoring and alerting
- Disaster recovery

---

**Ready to start?** Run `./01_songbird_coordination/demo.sh` to begin!

