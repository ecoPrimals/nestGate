# ⚡ **QUICK DEPLOY GUIDE**

**For**: Multi-Tower NestGate Deployment  
**Status**: 🚀 **READY**

---

## 🎯 **ONE COMMAND DEPLOY**

```bash
# Build once
cargo build --release

# Deploy to all towers (0-touch!)
for tower in eastgate westgate strandgate northgate southgate swiftgate; do
    echo "📦 Deploying to $tower..."
    scp target/release/nestgate $tower:~/ && \
    ssh $tower "./nestgate service start --daemon --port 9001" && \
    echo "✅ $tower ready!" || echo "❌ $tower failed"
done
```

**That's it!** Each tower auto-discovers local Songbird and registers!

---

## 📋 **PER-TOWER SETUP**

### **Eastgate (Dev/Biome)**
```bash
ssh eastgate
./nestgate service start --port 9001

# Auto-discovers 127.0.0.1:8080
# Registers as biome member with Toadstool, Squirrel
# ✅ Part of local biome
```

### **Westgate (NAS - 86TB)**
```bash
ssh westgate
./nestgate service start --port 9001

# Auto-discovers local Songbird
# Registers as primary storage
# ✅ NAS for entire federation
```

### **Strandgate (Server - 64 cores)**
```bash
ssh strandgate
./nestgate service start --port 9001

# Auto-discovers local Songbird
# Provides fast local storage
# ✅ Serves parallel workloads
```

---

## ✅ **VERIFICATION**

### **Check Each Tower:**
```bash
# On each tower:
curl http://localhost:8080/api/federation/services | \
    jq '.[] | select(.service_type == "storage")'

# Should show local NestGate!
```

### **Check Federation:**
```bash
# From any tower:
curl http://localhost:8080/api/federation/status

# Shows all towers in federation
```

### **Test Cross-Tower Discovery:**
```python
# From Toadstool on any tower:
import requests

# Discover all storage services
resp = requests.get("http://localhost:8080/api/federation/services/type/storage")
storage_services = resp.json()

print(f"Found {len(storage_services)} storage services:")
for svc in storage_services:
    print(f"  - {svc['service_name']} at {svc['endpoint']}")
```

---

## 🔧 **OVERRIDE (1-Touch)**

```bash
# Force specific Songbird:
export NESTGATE_ORCHESTRATOR_URL="http://specific:8080"
./nestgate service start
```

---

## 🏗️ **ARCHITECTURE**

```
FEDERATION MESH
    ↕
┌───────┬───────┬───────┐
│Westgate│Eastgate│Strand.│
│  SB   │  SB   │  SB  │  ← Local Songbirds
└───┬───┴───┬───┴───┬───┘
    │       │       │
   NG   NG+TT+SQ    NG     ← NestGate instances
   86TB   Biome   Server

SB = Songbird (local)
NG = NestGate
TT = Toadstool
SQ = Squirrel
```

---

**🎯 Deploy once, works everywhere!**

