# 🏠 **LOCAL NESTGATE INSTANCE SETUP**

**Run NestGate locally and connect to Songbird service mesh**

---

## 🎯 **OVERVIEW**

This guide sets up:
1. Local NestGate service (development instance)
2. Connection to Songbird (service mesh)
3. Working CLI with full implementations
4. Ecosystem integration testing

**Why this approach?**
- ✅ Local instance for development/testing
- ✅ CLI connects to local service (full features)
- ✅ Songbird provides service discovery
- ✅ Can connect to other towers via mesh
- ✅ Test ecosystem integration safely

---

## 🚀 **QUICK START** (5 minutes)

### **Step 1: Start Local NestGate Service**

```bash
cd /path/to/nestgate

# Create local data directory
mkdir -p ~/.nestgate/data
mkdir -p ~/.nestgate/logs
mkdir -p ~/.nestgate/config

# Start NestGate service locally
./target/release/nestgate service start \
    --bind 127.0.0.1:8080 \
    --data-dir ~/.nestgate/data \
    --log-dir ~/.nestgate/logs \
    --log-level info &

NESTGATE_PID=$!
echo "NestGate started with PID: $NESTGATE_PID"
echo $NESTGATE_PID > ~/.nestgate/service.pid

# Wait for startup
sleep 3

# Test it's running
curl -s http://127.0.0.1:8080/health | jq '.'
```

### **Step 2: Test CLI Connection**

```bash
# Configure CLI to use local instance
export NESTGATE_API_URL="http://127.0.0.1:8080"

# Test CLI commands
./target/release/nestgate --help
./target/release/nestgate version
./target/release/nestgate status

# Test storage operations
./target/release/nestgate storage info
./target/release/nestgate storage list
```

---

## 🌐 **CONNECT TO SONGBIRD** (Optional - Network Effects!)

### **Step 3: Check if Songbird is Running**

```bash
# Check if Songbird is running on any tower
for node in node-a node-b node-c node-d; do
    echo "Checking $node..."
    ssh $node "pgrep songbird && echo '✓ Songbird running on $node'" || true
done
```

### **Step 4: Start Songbird (if needed)**

```bash
# If Songbird isn't running, start it locally
cd /path/to/songbird

# Start Songbird service mesh
./target/release/songbird service start \
    --bind 0.0.0.0:9090 \
    --discovery-mode auto \
    --log-level info &

SONGBIRD_PID=$!
echo "Songbird started with PID: $SONGBIRD_PID"

# Wait for startup
sleep 3
```

### **Step 5: Enable NestGate Discovery**

```bash
# Restart NestGate with Songbird integration
kill $(cat ~/.nestgate/service.pid)

# Start with discovery enabled
./target/release/nestgate service start \
    --bind 127.0.0.1:8080 \
    --data-dir ~/.nestgate/data \
    --enable-discovery \
    --discovery-service http://localhost:9090 \
    --announce-capabilities "storage,api,local-dev" &

echo $! > ~/.nestgate/service.pid

# NestGate will now register with Songbird
echo "✓ NestGate registered with Songbird"
```

### **Step 6: Verify Integration**

```bash
# Check service registration
curl -s http://localhost:9090/services | jq '.'

# Should see NestGate listed
# Test service discovery
./target/release/nestgate discover --service-type storage
```

---

## 📝 **CONFIGURATION FILE** (Recommended)

Create a config file for easier management:

```bash
cat > ~/.nestgate/config.toml << 'EOF'
[service]
name = "nestgate-node-d-dev"
bind_address = "127.0.0.1"
port = 8080

[storage]
data_dir = "$HOME/.nestgate/data"
cache_dir = "$HOME/.nestgate/cache"
temp_dir = "/tmp/nestgate"

[logging]
level = "info"
dir = "$HOME/.nestgate/logs"
format = "json"

[discovery]
enabled = true
service_mesh = "songbird"
discovery_url = "http://localhost:9090"
announce_capabilities = ["storage", "api", "local-dev"]

[api]
enable_cors = true
allowed_origins = ["http://localhost:*"]
rate_limit = 1000  # requests per minute

[features]
enable_metrics = true
enable_health_checks = true
enable_admin_api = true
EOF

# Start with config file
./target/release/nestgate service start --config ~/.nestgate/config.toml &
```

---

## 🧪 **TEST CLI FUNCTIONALITY**

### **Basic Commands**

```bash
# Set environment
export NESTGATE_API_URL="http://127.0.0.1:8080"

# Version and health
./target/release/nestgate version
./target/release/nestgate doctor

# Service info
./target/release/nestgate service status
./target/release/nestgate service info

# Storage operations
./target/release/nestgate storage create testdata
./target/release/nestgate storage list
./target/release/nestgate storage info testdata

# Write test data
echo "Hello NestGate!" | ./target/release/nestgate storage write testdata/hello.txt

# Read it back
./target/release/nestgate storage read testdata/hello.txt

# Snapshots
./target/release/nestgate snapshot create testdata@test1
./target/release/nestgate snapshot list testdata
```

### **Advanced Commands**

```bash
# Metrics
./target/release/nestgate metrics show
./target/release/nestgate metrics export --format prometheus

# Discovery (if Songbird connected)
./target/release/nestgate discover services
./target/release/nestgate discover capabilities storage

# Connect to remote towers
./target/release/nestgate connect node-a
./target/release/nestgate remote list --node node-a
```

---

## 🌐 **ECOSYSTEM INTEGRATION TESTING**

### **Scenario: Use Songbird to Find Storage**

```bash
# With Songbird running, discover all storage services
./target/release/nestgate discover --service-type storage

# Output:
# Found 3 storage services:
#   - nestgate-node-d-dev (local, 127.0.0.1:8080)
#   - nestgate-node-a (remote, node-a:8080)
#   - nestgate-node-c (remote, node-c:8080)

# Connect to remote service via Songbird
./target/release/nestgate connect node-a --via-songbird

# Now CLI commands route through Songbird mesh!
./target/release/nestgate storage list --node node-a
```

### **Scenario: Multi-Node Storage Query**

```bash
# Query storage across all nodes (via Songbird)
./target/release/nestgate storage query \
    --pattern "*.fastq" \
    --all-nodes \
    --via-discovery

# Songbird routes requests to all discovered NestGate instances
# Results aggregated and returned
```

---

## 🔧 **DEVELOPMENT WORKFLOW**

### **Typical Dev Session**

```bash
# 1. Start local services
cd /path/to/nestgate
./scripts/start_local_dev.sh

# 2. Make code changes
vim code/crates/nestgate-core/src/...

# 3. Rebuild and restart
cargo build --release
./scripts/restart_local_dev.sh

# 4. Test with CLI
export NESTGATE_API_URL="http://127.0.0.1:8080"
./target/release/nestgate test-command

# 5. Check logs
tail -f ~/.nestgate/logs/nestgate.log
```

### **Helper Scripts**

Create convenience scripts:

```bash
# ~/.nestgate/scripts/start.sh
cat > ~/.nestgate/scripts/start.sh << 'EOF'
#!/bin/bash
cd /path/to/nestgate

if [ -f ~/.nestgate/service.pid ]; then
    echo "NestGate already running (PID: $(cat ~/.nestgate/service.pid))"
    exit 1
fi

./target/release/nestgate service start \
    --config ~/.nestgate/config.toml &

echo $! > ~/.nestgate/service.pid
echo "✓ NestGate started (PID: $(cat ~/.nestgate/service.pid))"
EOF

# ~/.nestgate/scripts/stop.sh
cat > ~/.nestgate/scripts/stop.sh << 'EOF'
#!/bin/bash
if [ -f ~/.nestgate/service.pid ]; then
    PID=$(cat ~/.nestgate/service.pid)
    kill $PID 2>/dev/null && echo "✓ NestGate stopped (PID: $PID)"
    rm ~/.nestgate/service.pid
else
    echo "⚠ NestGate not running"
fi
EOF

# ~/.nestgate/scripts/restart.sh
cat > ~/.nestgate/scripts/restart.sh << 'EOF'
#!/bin/bash
~/.nestgate/scripts/stop.sh
sleep 2
~/.nestgate/scripts/start.sh
EOF

# Make executable
chmod +x ~/.nestgate/scripts/*.sh
```

---

## 🐛 **TROUBLESHOOTING**

### **Service Won't Start**

```bash
# Check if port is in use
lsof -i :8080

# Kill old instance
pkill -f "nestgate service"

# Check logs
tail -100 ~/.nestgate/logs/nestgate.log

# Try different port
./target/release/nestgate service start --port 8081
```

### **CLI Can't Connect**

```bash
# Check service is running
curl http://127.0.0.1:8080/health

# Set environment variable
export NESTGATE_API_URL="http://127.0.0.1:8080"

# Or pass explicitly
./target/release/nestgate --api-url http://127.0.0.1:8080 status
```

### **Songbird Connection Issues**

```bash
# Check Songbird is running
curl http://localhost:9090/health

# Check registration
curl http://localhost:9090/services | jq '.[] | select(.name | contains("nestgate"))'

# Force re-registration
./target/release/nestgate service register \
    --discovery-url http://localhost:9090
```

---

## 📊 **MONITORING LOCAL INSTANCE**

### **Health Checks**

```bash
# Quick health check
curl -s http://127.0.0.1:8080/health | jq '.'

# Detailed status
curl -s http://127.0.0.1:8080/api/v1/status | jq '.'

# Metrics
curl -s http://127.0.0.1:8080/metrics
```

### **Resource Usage**

```bash
# Monitor process
PID=$(cat ~/.nestgate/service.pid)
ps aux | grep $PID
top -p $PID

# Disk usage
du -sh ~/.nestgate/

# Log size
du -sh ~/.nestgate/logs/
```

---

## 🎯 **EXAMPLE: FULL LOCAL SETUP**

Complete setup script:

```bash
#!/bin/bash
# setup_local_nestgate.sh

set -e

echo "🏠 Setting up local NestGate instance..."

# 1. Create directories
mkdir -p ~/.nestgate/{data,logs,config,cache,scripts}

# 2. Create config
cat > ~/.nestgate/config.toml << 'EOF'
[service]
name = "nestgate-node-d-dev"
bind_address = "127.0.0.1"
port = 8080

[storage]
data_dir = "$HOME/.nestgate/data"

[logging]
level = "info"
dir = "$HOME/.nestgate/logs"

[discovery]
enabled = true
discovery_url = "http://localhost:9090"
EOF

# 3. Start service
cd /path/to/nestgate
./target/release/nestgate service start \
    --config ~/.nestgate/config.toml &

echo $! > ~/.nestgate/service.pid

# 4. Wait and test
sleep 3
curl -s http://127.0.0.1:8080/health

# 5. Configure CLI
cat >> ~/.bashrc << 'EOF'
# NestGate CLI
export NESTGATE_API_URL="http://127.0.0.1:8080"
alias ng='nestgate'
EOF

echo "✓ Setup complete!"
echo ""
echo "Next steps:"
echo "  1. source ~/.bashrc"
echo "  2. nestgate status"
echo "  3. nestgate storage list"
```

---

## 🚀 **NEXT STEPS**

### **After Local Setup**

1. **Test CLI Commands**: Try all commands locally
2. **Start Songbird**: Enable service mesh
3. **Connect to Towers**: Access remote NestGate instances
4. **Build Features**: Develop new CLI commands
5. **Test Integration**: Validate ecosystem features

### **Production Deployment**

Once local testing is complete:
1. Deploy to Westgate (NAS mode)
2. Deploy to Strandgate (smart tier)
3. Deploy to Northgate (hot cache)
4. Enable Songbird mesh across all nodes
5. Full ecosystem integration!

---

## 📚 **RELATED DOCS**

- **Build Guide**: `BUILD_SUCCESS_REPORT.md`
- **CLI Reference**: `docs/CLI_REFERENCE.md`
- **Service Mesh**: `showcase/ECOSYSTEM_NETWORK_EFFECTS.md`
- **Songbird Integration**: `../songbird/INTEGRATION_GUIDE.md`

---

**🏠 Local instance: Your development playground!**

**🌐 + Songbird: Production-ready service mesh!**

**🚀 Start building and testing locally, deploy globally!**

