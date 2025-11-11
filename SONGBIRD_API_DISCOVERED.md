# 🎯 **SONGBIRD API DISCOVERED!**

**Date**: November 10, 2025  
**Status**: ✅ **ENDPOINTS FOUND**

---

## 🔍 **SONGBIRD'S ACTUAL API**

### **Found in:** `songbird/crates/songbird-orchestrator/src/server/federation_api.rs`

### ✅ **Available Endpoints**

#### **Service Registration**
```
POST /api/federation/services
```

Not `/api/v1/register` - that's for capability providers!

#### **Capability Provider Registration** (Different!)
```
POST /api/federation/register
```

#### **Heartbeat**
```
POST /api/federation/heartbeat
POST /api/federation/capability/heartbeat
```

#### **Service Discovery**
```
GET /api/federation/services
GET /api/federation/services/:service_id
GET /api/federation/services/type/:service_type
GET /api/federation/services/stats
```

#### **Node Management**
```
POST /api/federation/join
GET  /api/federation/status
GET  /api/federation/nodes
```

---

## 🔧 **WHAT NESTGATE NEEDS TO CHANGE**

### **Current (Wrong):**
```rust
// NestGate is calling:
POST http://192.168.1.144:8080/api/v1/ports/allocate  // ❌ Doesn't exist
POST http://192.168.1.144:8080/api/v1/register        // ❌ Wrong endpoint
```

### **Correct:**
```rust
// NestGate should call:
POST http://192.168.1.144:8080/api/federation/services  // ✅ Service registration
POST http://192.168.1.144:8080/api/federation/register  // ✅ Capability provider
```

---

## 📝 **SERVICE REGISTRATION FORMAT**

From `songbird-network-federation/service_registry.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    pub service_id: String,
    pub service_type: String,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub health_check_endpoint: Option<String>,
    pub registered_at: DateTime<Utc>,
}
```

---

## 🎯 **PORT ALLOCATION**

**Discovery:** Songbird **does NOT have a port allocation endpoint!**

Instead, services should:
1. Choose their own port
2. Register with Songbird providing their endpoint
3. Songbird tracks the registry

**This is actually better** - maintains service sovereignty!

---

## 🚀 **UPDATED INTEGRATION PLAN**

### **1. Remove Port Allocation**
- NestGate should NOT request ports from Songbird
- NestGate chooses its own port (sovereignty!)
- Register with chosen port

### **2. Update Registration Endpoint**
```rust
// Change from:
let url = format!("{}/api/v1/register", orchestrator_url);

// To:
let url = format!("{}/api/federation/services", orchestrator_url);
```

### **3. Update Payload Format**
```rust
let payload = serde_json::json!({
    "service_id": "nestgate-local",
    "service_type": "storage",
    "endpoint": format!("http://{}:{}", bind, port),
    "capabilities": ["storage", "zfs", "dataset_management", "snapshots"],
    "metadata": {
        "primal_name": "nestgate",
        "version": env!("CARGO_PKG_VERSION"),
        "protocol": "http",
        "node": hostname,
    },
    "health_check_endpoint": format!("http://{}:{}/health", bind, port),
    "registered_at": chrono::Utc::now().to_rfc3339(),
});
```

### **4. Optional: Capability Provider Registration**
```rust
// If registering as capability provider:
POST /api/federation/register

{
  "provider_id": "nestgate-local",
  "primal_type": "storage",
  "capabilities": ["zfs", "dataset_management"],
  "endpoint": "http://192.168.1.144:9001",
  "metadata": { ... }
}
```

---

## ✅ **WHAT'S ACTUALLY WORKING**

### **Songbird Orchestrator Running:**
- ✅ Port 8080
- ✅ Federation API mounted at `/api/federation/`
- ✅ Service registry ready
- ✅ Capability registry ready

### **NestGate Discovery Working:**
- ✅ Finds Songbird at 192.168.1.144:8080
- ✅ TCP check succeeds
- ✅ Switches to OrchestrationEnhanced mode
- ⚠️ Wrong endpoints being called

---

## 🔧 **FIXES NEEDED**

### **In NestGate:**

1. **Remove port allocation logic**
2. **Update registration endpoint**
3. **Fix payload format**
4. **Add heartbeat (optional)**

### **Quick Fix:**

```rust
// In nestgate-bin/src/commands/service.rs

async fn start_service(&self, port: u16, bind: String, daemon: bool) -> BinResult<()> {
    // ... existing code ...
    
    // REMOVE: Port allocation attempt
    // Instead: Just use the provided port!
    
    // Update: Registration endpoint
    if let Some(ref orchestrator_url) = self.orchestrator_url {
        match self.register_with_orchestrator(orchestrator_url, port, &bind).await {
            Ok(()) => {
                info!("✅ Registered with Songbird");
                println!("✅ Registered with Songbird");
            }
            Err(e) => {
                warn!("⚠️ Registration failed: {}", e);
            }
        }
    }
}

async fn register_with_orchestrator(
    &self,
    orchestrator_url: &str,
    port: u16,
    bind: &str,
) -> Result<(), String> {
    let client = reqwest::Client::new();
    
    // CORRECT endpoint
    let url = format!("{}/api/federation/services", orchestrator_url);
    
    let payload = serde_json::json!({
        "service_id": "nestgate-local",
        "service_type": "storage",
        "endpoint": format!("http://{}:{}", bind, port),
        "capabilities": ["storage", "zfs", "dataset_management", "snapshots"],
        "metadata": {
            "primal_name": "nestgate",
            "version": env!("CARGO_PKG_VERSION"),
            "protocol": "http",
        },
        "health_check_endpoint": format!("http://{}:{}/health", bind, port),
        "registered_at": chrono::Utc::now().to_rfc3339(),
    });
    
    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to register: {}", e))?;
    
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("HTTP {}", response.status()))
    }
}
```

---

## 🎊 **SUMMARY**

**✅ Found Songbird's actual API**

**✅ Endpoints are `/api/federation/*` not `/api/v1/*`**

**✅ No port allocation - services are sovereign!**

**✅ Service registration endpoint found**

**✅ Capability provider endpoint found**

**🔧 Need to update NestGate to use correct endpoints**

---

## 📝 **NEXT STEPS**

1. Update `nestgate-bin/src/commands/service.rs`
2. Remove port allocation logic
3. Fix registration endpoint to `/api/federation/services`
4. Fix payload format
5. Test registration!

---

**The systems are there - just need the right addresses!**

