# Demo 1.3: Capability Discovery

**Level**: 1 (Isolated)  
**Time**: 7 minutes  
**Complexity**: Beginner  
**Status**: 🆕 New demo

---

## 🎯 WHAT THIS DEMO SHOWS

This demo demonstrates NestGate's **zero-knowledge architecture** - how it discovers its own capabilities at runtime without hardcoded configuration:

1. Auto-detect available storage backends
2. Discover installed features and capabilities
3. Runtime capability reporting
4. No hardcoded primal endpoints!
5. Self-knowledge pattern in action

**Key Concept**: NestGate doesn't "know" what it can do until runtime - it discovers itself!

---

## 🌟 WHY THIS MATTERS

### Traditional Approach (Bad):
```rust
// ❌ Hardcoded - locked to specific services
const BEARDOG_URL = "http://localhost:9000";
const SONGBIRD_URL = "http://localhost:9090";
```

### NestGate's Approach (Good):
```rust
// ✅ Runtime discovery - finds what's available
let capabilities = discover_capabilities().await;
if let Some(crypto) = capabilities.find("crypto") {
    use_crypto_service(crypto);
}
```

**Benefits**:
- ✅ No vendor lock-in
- ✅ Works in any environment
- ✅ Adapts to available services
- ✅ True sovereignty

---

## 🚀 QUICK RUN

```bash
# Make sure NestGate is running
../../scripts/start_data_service.sh

# Run the demo
./demo.sh

# Expected runtime: ~1 minute
```

---

## 📋 WHAT YOU'LL SEE

### Part 1: Storage Backend Discovery
```
Detecting storage backends...
✓ ZFS: Available (version 2.x)
✓ Local Filesystem: Available
✗ S3: Not configured
✗ GCS: Not configured
```

### Part 2: Feature Detection
```
Checking available features...
✓ Compression: Enabled
✓ Deduplication: Enabled
✓ Snapshots: Enabled
✓ Replication: Enabled
✓ Encryption: Available (via capability)
```

### Part 3: Network Capabilities
```
Network capability discovery...
✓ REST API: Listening on :8080
✓ RPC: Available on :50051
✓ mDNS: Broadcasting as nestgate._tcp.local
✗ Federation: No peers discovered
```

### Part 4: Primal Discovery
```
Scanning for ecosystem primals...
⚙ Checking for BearDog (crypto)...
  → mDNS query: beardog._tcp.local
  → Result: Not found
  
⚙ Checking for Songbird (orchestration)...
  → mDNS query: songbird._tcp.local
  → Result: Not found
  
⚙ Checking for ToadStool (compute)...
  → mDNS query: toadstool._tcp.local
  → Result: Not found

Note: This is expected - no other primals running locally
```

### Part 5: Capability Report
```json
{
  "self_knowledge": {
    "name": "nestgate",
    "version": "0.1.0",
    "capabilities": [
      "storage",
      "data-service",
      "zfs",
      "rest-api",
      "rpc"
    ],
    "backends": {
      "zfs": "available",
      "local": "available"
    },
    "discovery": {
      "mdns": "enabled",
      "dns-sd": "enabled"
    },
    "ecosystem": {
      "primals_found": 0,
      "federation_peers": 0
    }
  }
}
```

---

## 🔍 HOW CAPABILITY DISCOVERY WORKS

### Step 1: Self-Inspection
```
NestGate starts → Checks its own binaries
                → Detects compiled features
                → Validates configuration
                → Reports capabilities
```

### Step 2: Environment Scanning
```
Check local services → ZFS available?
                     → Docker available?
                     → Network services?
```

### Step 3: Network Discovery
```
mDNS broadcast → "I'm NestGate, I do storage"
mDNS listen    → "Anyone else here?"
               → Build capability map
```

### Step 4: Dynamic Routing
```
Request comes in → Check capability map
                 → Route to available service
                 → Fallback if not available
```

---

## 💡 REAL-WORLD SCENARIOS

### Scenario 1: Air-Gapped Network
```
NestGate on isolated network:
✓ Detects: Local ZFS, filesystem
✗ Won't find: Cloud storage, external primals
→ Works perfectly with what's available!
```

### Scenario 2: Cloud Deployment
```
NestGate on AWS:
✓ Detects: S3 credentials in environment
✓ Discovers: Other NestGate nodes via service mesh
→ Automatically uses cloud-native storage!
```

### Scenario 3: Friend's Laptop
```
Friend joins your LAN:
✓ NestGate discovers: Your NestGate instance
✓ NestGate discovers: BearDog if running
✓ NestGate advertises: Own capabilities
→ Instant mesh formation, zero config!
```

---

## 🧪 EXPERIMENTS TO TRY

### Experiment 1: Add Another Primal
```bash
# Terminal 1: NestGate
./demo.sh

# Terminal 2: Start BearDog (if you have it)
cd ../../../../beardog
cargo run

# Terminal 3: Re-run discovery
./demo.sh
# Now you'll see BearDog discovered!
```

### Experiment 2: Disable Features
```bash
# Run without ZFS
NESTGATE_DISABLE_ZFS=1 ./demo.sh
# Shows: ZFS: Disabled by configuration

# Run with minimal features
NESTGATE_MINIMAL=1 ./demo.sh
# Shows: Only essential capabilities
```

### Experiment 3: Multi-Node Discovery
```bash
# Machine 1
NESTGATE_NODE_NAME=node1 ./demo.sh

# Machine 2 (different machine/VM)
NESTGATE_NODE_NAME=node2 ./demo.sh

# They'll discover each other via mDNS!
```

---

## 📊 COMPARISON: TRADITIONAL VS ZERO-KNOWLEDGE

### Traditional (Hardcoded):
```yaml
# ❌ Config locked to specific deployment
storage:
  primary: "s3://my-bucket"
  backup: "gcs://my-backup"
crypto:
  provider: "http://beardog.prod.company.com:9000"
```

**Problems**:
- Breaks if services move
- Can't adapt to new environment
- Vendor lock-in
- Manual reconfiguration required

### NestGate (Discovery):
```yaml
# ✅ Just hints and preferences
preferences:
  storage: ["zfs", "s3", "local"]  # Priority order
  discovery: "auto"                  # Find what's available
```

**Benefits**:
- Works anywhere
- Adapts automatically
- No vendor lock-in
- Self-healing

---

## 🔧 UNDER THE HOOD

### Discovery Methods Used:

1. **Self-Inspection**
   - Binary feature flags
   - Compiled capabilities
   - Environment variables

2. **mDNS/DNS-SD**
   - Broadcast: `_nestgate._tcp.local`
   - Listen: `_*._tcp.local`
   - Auto-discovery on LAN

3. **Configuration**
   - Read: `~/.nestgate/config.toml`
   - Env vars: `NESTGATE_*`
   - Capability hints

4. **Health Probes**
   - Try connecting to known ports
   - Check service health
   - Validate capabilities

---

## 🆘 TROUBLESHOOTING

### "No capabilities discovered"
**Cause**: NestGate not fully initialized  
**Fix**: Wait a few seconds after startup

### "mDNS not working"
**Cause**: Firewall blocking port 5353  
**Fix**: Allow mDNS traffic
```bash
# Linux
sudo ufw allow 5353/udp

# Or disable firewall temporarily
sudo ufw disable
```

### "Can't discover other nodes"
**Cause**: Different network segments  
**Fix**: Ensure machines on same subnet
```bash
# Check your network
ip addr show
# Should be same network (e.g., 192.168.1.x)
```

---

## 📚 LEARN MORE

**Concepts**:
- **Zero-Knowledge Architecture**: `../../../docs/architecture/ZERO_KNOWLEDGE_ARCHITECTURE.md`
- **Capability Discovery**: `../../../specs/INFANT_DISCOVERY_ARCHITECTURE_SPEC.md`
- **Service Discovery**: `../../../docs/current/UNIVERSAL_ADAPTER_ARCHITECTURE.md`

**Implementation**:
- Capability system: `../../../code/crates/nestgate-core/src/capabilities/`
- Discovery: `../../../code/crates/nestgate-core/src/universal_primal_discovery/`
- mDNS: `../../../code/crates/nestgate-network/src/discovery/`

---

## ⏭️ NEXT DEMO

**Demo 1.4: Health Monitoring** (`../04_health_monitoring/`)
- Monitor system health in real-time
- Understand metrics and observability
- See performance data

---

**Status**: 🆕 New demo  
**Estimated time**: 7 minutes  
**Prerequisites**: Completed Demos 1.1-1.2

**Key Takeaway**: NestGate discovers capabilities at runtime - no hardcoding, pure sovereignty! 🏰

---

*Demo 1.3 - Capability Discovery*  
*Part of Level 1: Isolated Instance*

