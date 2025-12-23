# 🎯 NESTGATE + SONGBIRD INTEGRATION PLAN

**Date**: December 21, 2025  
**Focus**: Live services only - Songbird federation + NestGate storage  
**Goal**: Real multi-tower demos like Songbird's proven patterns

---

## 📊 ECOSYSTEM ANALYSIS COMPLETE

### **Songbird** 🎵 (Best Federation)
- ✅ **02-federation/**: Real multi-tower with actual mesh
- ✅ **setup-local-federation.sh**: Working scripts
- ✅ **Multi-machine support**: Actual distributed setup
- ✅ **Live discovery**: mDNS, DNS-SD working
- **Key Files**:
  - `02-federation/QUICK_START.sh`
  - `02-federation/setup-local-federation.sh`
  - `02-federation/scripts/start-tower.sh`

### **ToadStool** 🍄 (Best Compute)
- ✅ **gpu-universal/**: Real GPU workloads
- ✅ **inter-primal/**: Live integrations
- ✅ **03-nestgate-ml-pipeline/**: NestGate integration exists!
- ✅ **nestgate-compute/**: Data-triggered compute
- **Key Pattern**: Real compute tasks with actual data

### **NestGate Current** 🏰
- ✅ **demos/07_connected_live/**: Works with Songbird!
- ✅ **demos/08-10**: Live science/ML demos
- ❌ **00-local-primal/06-federation/**: Simulations (DRAFT)
- **Gap**: Need real multi-node NestGate

---

## 🎯 INTEGRATION STRATEGY

### **Phase 1: Songbird + NestGate (Priority)** 
**Goal**: Real storage discovery and data service orchestration

#### **1.1: Service Discovery (Live)** ✅ Already Works!
Located: `demos/07_connected_live/demo.sh`
```bash
# This already works:
- Songbird discovers NestGate
- NestGate registers capabilities
- Cross-primal communication verified
```

#### **1.2: Data Service Orchestration** 🔄 Build
**New Demo**: `showcase/01_nestgate_songbird_live/`
```bash
01-service-discovery.sh       # Based on demo 07
02-data-workflow.sh           # Songbird orchestrates storage ops
03-multi-node-storage.sh      # Multiple NestGate nodes
04-coordinated-backup.sh      # Songbird coordinates backups
```

**What it shows**:
- Songbird discovers multiple NestGate nodes
- Orchestrates write → replicate → verify workflow
- Load balancing across nodes
- Failover when node goes down
- All LIVE, no mocks

---

## 🏗️ BUILD PLAN

### **Sprint 1: Enhance Existing Live Demo** (2 hours)

**Task 1.1**: Expand `demos/07_connected_live/`
```bash
# Current: Shows service discovery
# Add: Actual storage operations through Songbird

# New sections:
- Write data via Songbird orchestration
- Read from discovered NestGate
- Show metrics flowing back
- Demonstrate health monitoring
```

**Task 1.2**: Create Songbird workflow YAML
```yaml
# workflow-storage-test.yaml
apiVersion: songbird/v1
kind: Workflow
steps:
  - name: discover-storage
    service: discovery
    operation: find
    params:
      service_type: storage
      
  - name: write-data
    service: ${steps.discover-storage.endpoint}
    operation: write
    params:
      path: /test/data.txt
      content: "Hello from Songbird!"
      
  - name: verify-write
    service: ${steps.discover-storage.endpoint}
    operation: read
    params:
      path: /test/data.txt
```

### **Sprint 2: Multi-Node NestGate** (3-4 hours)

**Goal**: Run 2+ NestGate instances, Songbird orchestrates between them

**Setup Script**: `showcase/01_nestgate_songbird_live/setup-multi-node.sh`
```bash
#!/bin/bash
# Start multiple NestGate nodes on different ports

# Node 1 (Westgate)
cargo run --release --bin nestgate -- \
  --port 9005 \
  --data-dir /tmp/nestgate-west \
  --name westgate &

# Node 2 (Eastgate)  
cargo run --release --bin nestgate -- \
  --port 9006 \
  --data-dir /tmp/nestgate-east \
  --name eastgate &

# Wait for startup
sleep 5

# Register with Songbird
curl -X POST http://localhost:8080/api/federation/register \
  -d '{"name": "westgate", "port": 9005, "capabilities": ["storage"]}'
curl -X POST http://localhost:8080/api/federation/register \
  -d '{"name": "eastgate", "port": 9006, "capabilities": ["storage"]}'

echo "✅ Multi-node NestGate ready!"
```

**Demo Script**: `demo-multi-node.sh`
```bash
#!/bin/bash
# Demo: Songbird orchestrates across multiple NestGate nodes

echo "🎯 Multi-Node Storage Demo"
echo ""

# 1. Show discovered nodes
echo "1. Songbird discovers NestGate nodes..."
curl -s http://localhost:8080/api/federation/services?type=storage | jq

# 2. Write to node 1 via Songbird
echo "2. Writing to westgate via Songbird..."
curl -X POST http://localhost:8080/api/workflow/run -d @workflow-write-west.yaml

# 3. Replicate to node 2 via Songbird
echo "3. Replicating westgate → eastgate..."
curl -X POST http://localhost:8080/api/workflow/run -d @workflow-replicate.yaml

# 4. Read from node 2 to verify
echo "4. Reading from eastgate to verify..."
curl http://localhost:9006/api/storage/read?path=/test/data.txt

# 5. Show load balancing
echo "5. Load balanced writes (Songbird distributes)..."
for i in {1..10}; do
  curl -X POST http://localhost:8080/api/workflow/run -d @workflow-write.yaml
done

# Show distribution
curl -s http://localhost:8080/api/metrics/distribution | jq
```

### **Sprint 3: Learn from Songbird's Federation** (2 hours)

**Research**:
1. Read `../songbird/showcase/02-federation/QUICK_START.sh`
2. Study `setup-local-federation.sh`
3. Identify what makes it "live"
4. Apply patterns to NestGate

**Adaptation**:
- Use Songbird's tower startup pattern
- Adopt their configuration approach
- Apply their testing methodology
- Mirror their documentation style

---

## 📋 NEW DIRECTORY STRUCTURE

```
showcase/
├── demos/
│   ├── 07_connected_live/          ✅ Already works
│   ├── 08_bioinformatics_live/     ✅ Already works
│   ├── 09_ml_model_serving/        ✅ Already works
│   └── 10_scientific_computing/    ✅ Already works
│
├── 01_nestgate_songbird_live/      🔄 NEW (Priority)
│   ├── README.md
│   ├── QUICK_START.sh
│   ├── setup-multi-node.sh
│   ├── 01-service-discovery.sh     (based on demo 07)
│   ├── 02-data-workflow.sh         (Songbird orchestrates)
│   ├── 03-multi-node-storage.sh    (2+ nodes)
│   ├── 04-coordinated-backup.sh    (workflow example)
│   ├── workflows/
│   │   ├── write-replicate.yaml
│   │   ├── backup-verify.yaml
│   │   └── load-balance.yaml
│   └── configs/
│       ├── westgate.toml
│       └── eastgate.toml
│
├── 02_nestgate_toadstool_live/     ⏭️  Next (after Songbird)
│   ├── 01-gpu-storage-pipeline.sh
│   ├── 02-ml-artifact-management.sh
│   └── 03-distributed-training.sh
│
└── 00-local-primal/
    ├── 01-05/                      ✅ Keep (enhance to be live)
    └── 06-local-federation/        ❌ DRAFT (educational only)
```

---

## 🎯 SUCCESS CRITERIA

### **Must Have** (Sprint 1-2)
- [x] Enhance demo 07 with actual operations
- [ ] Run 2+ NestGate instances simultaneously
- [ ] Songbird orchestrates between them
- [ ] Real data written and replicated
- [ ] Load balancing demonstrated
- [ ] All using actual running services

### **Should Have** (Sprint 3)
- [ ] Failover demonstration (kill node, see recovery)
- [ ] Health monitoring through Songbird
- [ ] Metrics collection across nodes
- [ ] Workflow YAML examples
- [ ] Clear documentation

### **Nice to Have** (Future)
- [ ] 3+ node mesh
- [ ] Multi-machine deployment
- [ ] Performance benchmarks
- [ ] Chaos testing

---

## 🚀 IMMEDIATE NEXT STEPS

### **Step 1: Test Current Demo 07** (30 min)
```bash
cd showcase/demos/07_connected_live
./demo.sh

# Verify:
- Songbird responds
- NestGate binary exists
- Discovery works
- Document what's live vs simulated
```

### **Step 2: Create Multi-Node Setup** (2 hours)
```bash
mkdir -p showcase/01_nestgate_songbird_live
cd showcase/01_nestgate_songbird_live

# Create:
- setup-multi-node.sh (start 2+ nodes)
- demo-multi-node.sh (orchestration demo)
- workflow examples (YAML)
- README with clear instructions
```

### **Step 3: Test & Validate** (1 hour)
```bash
# Start services
./setup-multi-node.sh

# Run demo
./demo-multi-node.sh

# Verify:
- Multiple NestGate processes running
- Songbird sees both nodes
- Data flows between them
- All operations are LIVE
```

---

## 💡 KEY PRINCIPLES

1. **Live Services Only** - No `echo` simulations
2. **Real Processes** - Actual `cargo run` commands
3. **Real API Calls** - Use `curl` to live endpoints
4. **Real Data** - Write and read actual files
5. **Verifiable** - Use `ps`, `netstat`, `curl` to prove it's live

---

## 📚 REFERENCES

**Learn From**:
- Songbird: `../songbird/showcase/02-federation/`
- ToadStool: `../toadstool/showcase/nestgate-compute/`
- Current: `showcase/demos/07_connected_live/`

**Patterns to Apply**:
- Songbird's multi-tower startup scripts
- ToadStool's real compute workflows
- Demo 07's service discovery approach

---

**Status**: Plan ready  
**Priority**: Songbird integration first  
**Approach**: Build on demo 07, add multi-node  
**Quality**: Live services only, no mocks

---

*Real integration. Real services. Real value.*

