# 🌐 **CONNECT NESTGATE TO SONGBIRD**

**Enable service mesh and ecosystem integration**

---

## 🎯 **OVERVIEW**

Connecting NestGate to Songbird unlocks:
- ✅ **Service Discovery**: Auto-find other NestGate instances
- ✅ **Load Balancing**: Distribute requests across nodes
- ✅ **Health Monitoring**: Auto-failover and recovery
- ✅ **Mesh Networking**: Secure inter-node communication
- ✅ **Central Metrics**: Unified monitoring dashboardNestGate runs standalone OR with Songbird for network effects!

---

## 🚀 **QUICK SETUP** (5 minutes)

### **Step 1: Check for Songbird**

```bash
# Check if Songbird is running on any tower
for node in westgate strandgate northgate eastgate; do
    echo "Checking $node..."
    ssh $node "pgrep -a songbird" || echo "  Not running on $node"
done

# Or check locally
curl http://localhost:9090/health 2>/dev/null && echo "✓ Songbird running locally"
```

### **Step 2: Start Songbird** (if needed)

```bash
# Option A: Start locally on Eastgate
cd ~/Development/ecoPrimals/songbird
./start_service.sh

# Option B: SSH to specific tower
ssh strandgate "cd ~/songbird && ./start_service.sh"

# Wait for startup
sleep 3
curl http://localhost:9090/health
```

### **Step 3: Start NestGate with Discovery**

```bash
cd ~/Development/ecoPrimals/nestgate

# Start with Songbird integration
./target/release/nestgate service start \
    --bind 127.0.0.1 \
    --port 8080 \
    --enable-discovery \
    --discovery-service http://localhost:9090 \
    --announce-capabilities "storage,api,local-dev"
```

### **Step 4: Verify Integration**

```bash
# Check NestGate registered
curl -s http://localhost:9090/services | jq '.'

# Should show:
# {
#   "services": [
#     {
#       "name": "nestgate-eastgate-dev",
#       "address": "127.0.0.1:8080",
#       "capabilities": ["storage", "api", "local-dev"],
#       "status": "healthy"
#     }
#   ]
# }

# Test service discovery from CLI
./target/release/nestgate discover --service-type storage
```

---

## 📝 **CONFIGURATION-BASED SETUP**

Create `~/.nestgate/config-with-discovery.toml`:

```toml
[service]
name = "nestgate-eastgate-dev"
bind_address = "127.0.0.1"
port = 8080

[storage]
data_dir = "/home/eastgate/.nestgate/data"
cache_dir = "/home/eastgate/.nestgate/cache"

[logging]
level = "info"
dir = "/home/eastgate/.nestgate/logs"

[discovery]
enabled = true
service_mesh = "songbird"
discovery_url = "http://localhost:9090"

# What to announce to other services
announce_capabilities = [
    "storage",      # NestGate core capability
    "api",          # REST API available
    "local-dev",    # Development instance marker
    "zfs",          # ZFS features
]

# What services to discover
discover_services = [
    "songbird",     # Service mesh
    "nestgate",     # Other NestGate instances
    "beardog",      # Security (if available)
    "toadstool",    # AI/compute (if available)
]

[api]
enable_cors = true
enable_metrics = true
enable_health_checks = true
```

Start with config:

```bash
./target/release/nestgate service start --config ~/.nestgate/config-with-discovery.toml
```

---

## 🌐 **MULTI-NODE SETUP**

Connect NestGate across your Metal Matrix:

### **Scenario: Full Mesh**

```bash
# 1. Start Songbird on central node (Strandgate - your server tower)
ssh strandgate "cd ~/songbird && ./start_service.sh --bind 0.0.0.0 --port 9090"

# 2. Start NestGate on Westgate (86TB storage)
ssh westgate << 'EOF'
cd ~/nestgate
./target/release/nestgate service start \
    --bind 0.0.0.0 \
    --port 8080 \
    --enable-discovery \
    --discovery-service http://strandgate:9090 \
    --announce-capabilities "storage,nas,archive"
EOF

# 3. Start NestGate on Eastgate (local dev)
./target/release/nestgate service start \
    --bind 127.0.0.1 \
    --port 8080 \
    --enable-discovery \
    --discovery-service http://strandgate:9090 \
    --announce-capabilities "storage,api,local-dev"

# 4. Start NestGate on Northgate (hot cache for AI)
ssh northgate << 'EOF'
cd ~/nestgate
./target/release/nestgate service start \
    --bind 0.0.0.0 \
    --port 8080 \
    --enable-discovery \
    --discovery-service http://strandgate:9090 \
    --announce-capabilities "storage,cache,high-performance"
EOF
```

### **Verify Mesh**

```bash
# Query Songbird for all NestGate services
curl -s http://strandgate:9090/services?type=nestgate | jq '.'

# Expected output:
# {
#   "services": [
#     {"name": "nestgate-westgate", "capabilities": ["storage","nas","archive"]},
#     {"name": "nestgate-eastgate", "capabilities": ["storage","api","local-dev"]},
#     {"name": "nestgate-northgate", "capabilities": ["storage","cache","high-performance"]}
#   ]
# }
```

---

## 🔧 **TESTING THE MESH**

### **Test 1: Service Discovery**

```bash
# From any node, discover all storage services
./target/release/nestgate discover --service-type storage

# Output:
# 🔍 Discovered Storage Services:
#   • nestgate-westgate (westgate:8080) - 86TB Archive Storage
#   • nestgate-eastgate (127.0.0.1:8080) - Development Instance
#   • nestgate-northgate (northgate:8080) - 5TB Hot Cache
```

### **Test 2: Cross-Node Storage Query**

```bash
# Query storage across ALL nodes via Songbird
./target/release/nestgate storage query \
    --pattern "*.fastq" \
    --all-nodes \
    --via-discovery

# Output:
# 📊 Storage Query Results (3 nodes):
#   westgate:/archive/bioinfo/seq001.fastq (2.3GB)
#   westgate:/archive/bioinfo/seq002.fastq (2.1GB)
#   northgate:/cache/models/evo1.pth (4.5GB)
```

### **Test 3: Load Balancing**

```bash
# Write data - Songbird routes to best node
./target/release/nestgate storage write \
    --file large_model.pth \
    --prefer-capability "high-performance"

# Songbird routes to Northgate (hot cache) automatically
```

---

## 📊 **MONITORING THE MESH**

### **Songbird Dashboard**

```bash
# Get mesh status
curl -s http://strandgate:9090/mesh/status | jq '.'

# Get metrics
curl -s http://strandgate:9090/metrics
```

### **NestGate Health Checks**

```bash
# Health of local instance
curl http://localhost:8080/health

# Health of remote instance via Songbird
curl http://strandgate:9090/proxy/nestgate-westgate/health
```

### **Logs**

```bash
# View Songbird logs
ssh strandgate "tail -f ~/songbird/logs/service.log"

# View NestGate logs
tail -f ~/.nestgate/logs/nestgate.log

# View mesh events
curl -s http://strandgate:9090/events?since=1h
```

---

## 🎯 **USE CASES**

### **1. Distributed Storage**

NestGate on Westgate (archive) + NestGate on Northgate (cache):

```bash
# Write to archive
./target/release/nestgate storage write data.tar.gz --node westgate

# Cache hot data on Northgate
./target/release/nestgate storage cache data.tar.gz --from westgate --to northgate
```

### **2. Smart Tiering**

Automatic data movement based on access patterns:

```bash
# Enable smart tiering (via Songbird coordination)
./target/release/nestgate storage configure \
    --smart-tier \
    --hot-node northgate \
    --warm-node strandgate \
    --cold-node westgate
```

### **3. AI Model Serving**

NestGate + Toadstool integration:

```bash
# NestGate serves model, Toadstool runs inference
# Songbird coordinates the pipeline

# Load model from storage
curl http://northgate:8080/api/v1/storage/models/evo1.pth > /tmp/model.pth

# Toadstool picks it up via Songbird discovery
curl -X POST http://localhost:9090/route/toadstool/infer \
    -d '{"model": "nestgate://northgate/models/evo1.pth", "input": "..."}'
```

---

## 🔐 **SECURITY INTEGRATION**

If BearDog (security primal) is running:

```bash
# NestGate auto-discovers BearDog via Songbird
# Enables:
# - Automatic authentication
# - Encrypted storage
# - Access control
# - Audit logging

# Start with security
./target/release/nestgate service start \
    --config ~/.nestgate/config.toml \
    --enable-security \
    --security-service http://localhost:9191  # BearDog
```

---

## 🚀 **NEXT STEPS**

### **Option 1: Full Ecosystem**

Deploy the complete ecoPrimals stack:
1. **Songbird** - Service mesh (Strandgate)
2. **NestGate** - Storage (Westgate, Northgate, Eastgate)
3. **BearDog** - Security (Strandgate)
4. **Toadstool** - AI/Compute (Northgate, Southgate)
5. **Squirrel** - Compute orchestration (Strandgate)

### **Option 2: Standalone**

Run NestGate standalone without Songbird:
- All features work locally
- Manual node management
- No automatic discovery

### **Option 3: Hybrid**

Start standalone, add Songbird later:
- NestGate works immediately
- Add `--enable-discovery` flag anytime
- Hot-reload configuration

---

## 📚 **RELATED DOCS**

- **CLI Commands**: `CLI_COMMANDS_WORKING.md`
- **Local Setup**: `LOCAL_INSTANCE_SETUP.md`
- **Showcase**: `showcase/START_HERE.md`
- **Ecosystem**: `showcase/ECOSYSTEM_NETWORK_EFFECTS.md`

---

**🌐 Songbird: Optional but powerful!**

**🏠 NestGate works great standalone**

**🚀 Add Songbird for 10x network effects!**

