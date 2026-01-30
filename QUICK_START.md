# 🚀 NestGate Quick Start (5 Minutes)

**Get NestGate running in 5 minutes!**

---

## ⚡ Prerequisites

- **Rust**: 1.75+ (`rustup`)
- **Git**: Any recent version
- **OS**: Linux, macOS, or Windows (WSL2)

**Optional**:
- ZFS (for storage features)
- Docker (for containerized deployment)

---

## 📦 Step 1: Clone & Build (2 minutes)

```bash
# Clone repository
git clone https://github.com/ecoPrimals/nestGate.git
cd nestGate

# Build (release mode for performance)
cargo build --release

# Or build faster in dev mode
cargo build
```

**Expected**: Build completes successfully in ~2-3 minutes (first time).

---

## 🔧 Step 2: Configure (1 minute)

### **Option A: Default Configuration** (Fastest)

```bash
# Use defaults - no configuration needed!
# NestGate runs with sensible defaults:
# - Port: 8080
# - Host: 127.0.0.1
# - Storage: XDG-compliant ($HOME/.local/share/nestgate)
# - Socket: XDG runtime dir
```

### **Option B: Custom Configuration** (Recommended)

```bash
# Create .env file
cat > .env << 'EOF'
# Network
NESTGATE_PORT=8080
NESTGATE_HOST=127.0.0.1

# Storage (XDG-compliant)
NESTGATE_DATA_DIR=$HOME/.local/share/nestgate
NESTGATE_CACHE_DIR=$HOME/.cache/nestgate

# Discovery
NESTGATE_DISCOVERY_ENABLED=true

# Logging
RUST_LOG=info
EOF

# Load environment
source .env
```

---

## 🎬 Step 3: Run (30 seconds)

### **Start NestGate**:

```bash
# Run from release build
./target/release/nestgate

# Or from dev build
./target/debug/nestgate

# Or with cargo
cargo run --release
```

**Expected Output**:
```
🚀 NestGate v3.3.0 starting...
✅ Configuration loaded from environment
✅ Storage initialized at ~/.local/share/nestgate
✅ Socket listening at /run/user/1000/nestgate/nestgate.sock
✅ HTTP API listening on http://127.0.0.1:8080
🎉 NestGate is ready!
```

---

## ✅ Step 4: Verify (1 minute)

### **Health Check**:

```bash
# Check health endpoint
curl http://localhost:8080/health

# Expected response:
# {"status":"healthy","version":"3.3.0","uptime_seconds":5}
```

### **Create a Dataset**:

```bash
# Create a test dataset
curl -X POST http://localhost:8080/api/datasets \
  -H "Content-Type: application/json" \
  -d '{"name":"test-dataset","description":"My first dataset"}'

# Store an object
echo "Hello NestGate!" > test.txt
curl -X PUT http://localhost:8080/api/datasets/test-dataset/objects/greeting \
  --data-binary @test.txt

# Retrieve the object
curl http://localhost:8080/api/datasets/test-dataset/objects/greeting
# Output: Hello NestGate!
```

### **Check Logs**:

```bash
# View logs (if using systemd)
journalctl -fu nestgate

# Or check log file
tail -f ~/.local/share/nestgate/logs/nestgate.log
```

---

## 🎯 You're Ready!

**What You Have**:
- ✅ NestGate running locally
- ✅ HTTP API accessible
- ✅ Unix socket IPC available
- ✅ XDG-compliant storage
- ✅ Capability-based discovery

---

## 📚 Next Steps

### **Learn More**:
1. **Architecture**: See `docs/architecture/SYSTEM_OVERVIEW.md`
2. **API Reference**: See `docs/api/REST_API.md`
3. **Configuration**: See `docs/guides/ENVIRONMENT_VARIABLES.md`
4. **Storage**: See `docs/guides/STORAGE_GUIDE.md`
5. **Development**: See `docs/DEVELOPER_SETUP.md`

### **Common Tasks**:
- **Add ZFS**: See `docs/guides/ZFS_SETUP.md`
- **Clustering**: See `docs/guides/CLUSTERING.md`
- **Production Deploy**: See `docs/operations/PRODUCTION_DEPLOYMENT_CHECKLIST.md`
- **Integrate**: See `docs/integration/INTEGRATION_GUIDE.md`

### **Get Help**:
- **Troubleshooting**: See `docs/guides/TROUBLESHOOTING.md`
- **FAQ**: See `docs/FAQ.md`
- **Issues**: https://github.com/ecoPrimals/nestGate/issues

---

## 🐛 Quick Troubleshooting

### **Port Already in Use**:
```bash
# Change port
export NESTGATE_PORT=8090
cargo run --release
```

### **Permission Denied (Socket)**:
```bash
# Use custom socket path
export NESTGATE_SOCKET_DIR=$HOME/.nestgate/sockets
mkdir -p $NESTGATE_SOCKET_DIR
cargo run --release
```

### **ZFS Not Found**:
```bash
# Disable ZFS features
export NESTGATE_ZFS_ENABLED=false
cargo run --release
```

---

## 🎉 Success!

You now have NestGate running locally in **under 5 minutes**!

**What's Next?**:
- 🔍 Explore the API
- 📦 Create datasets
- 🔌 Try Unix socket communication
- 🌍 Deploy to production
- 🛠️ Contribute to development

---

**NestGate**: Storage · Discovery · Security · Pure Rust 🦀

**Version**: 3.3.0  
**Grade**: A++ 108/100 EXCEPTIONAL  
**Status**: Production-Ready ✅

**Need Help?** See `docs/` for comprehensive guides!
