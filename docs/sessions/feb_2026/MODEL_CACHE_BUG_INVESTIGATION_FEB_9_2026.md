# 🔍 NestGate Model Cache Bug Investigation
## biomeOS Integration Handoff - Bug Analysis

**Date**: February 9, 2026  
**Upstream**: biomeOS model cache module complete (LIVE)  
**Status**: Investigating 3 bugs blocking cross-gate model discovery  
**Priority**: HIGH - Mesh-wide AI model distribution blocked

═══════════════════════════════════════════════════════════════════

## 📋 EXECUTIVE SUMMARY

```
╔════════════════════════════════════════════════════════════╗
║                                                             ║
║   BIOMEOS MODEL CACHE: LIVE & WAITING ON NESTGATE! 🚀    ║
║                                                             ║
║  biomeOS Side:        COMPLETE ✅                         ║
║  NestGate Bugs:       3 identified                     🔍  ║
║  Current Status:      Investigating                    🔍  ║
║                                                             ║
╚════════════════════════════════════════════════════════════╝
```

═══════════════════════════════════════════════════════════════════

## 🎯 UPSTREAM CONTEXT

### **biomeOS Model Cache Module** (COMPLETE ✅)

**Location**: `biomeos-core::model_cache`

**Features Implemented**:
- ✅ HuggingFace import (auto-discovers `~/.cache/huggingface/hub/`)
- ✅ Symlink-aware scanning (follows HF blob symlinks)
- ✅ Manifest persistence (`~/.biomeos/model-cache/manifest.json`)
- ✅ Format detection (safetensors, GGUF, PyTorch)
- ✅ Resolution pipeline: Local → NestGate mesh → NotFound
- ✅ Graceful degradation (works offline when NestGate unavailable)
- ✅ 4 unit tests passing

**CLI Commands** (`biomeos model-cache`):
- `import-hf` - Import all models from HuggingFace cache
- `list` - List all cached models with size and format
- `resolve <model-id>` - Check local and mesh for a model
- `register <model-id> <path>` - Register an arbitrary model directory
- `status` - Show cache status including NestGate connection

**Current Deployment**:
- **Tower (pop-os)**: TinyLlama/1.1B (2.1 GB) + stable-diffusion-v1-5 (4.1 GB) = 6.0 GB
- **gate2 (pop-os)**: TinyLlama/1.1B (2.1 GB) + Mistral-7B-Instruct (13.8 GB) = 15.5 GB

### **Integration Architecture**:

```
biomeos model-cache resolve "TinyLlama/..."
    |
    v
ModelCache::resolve()
    |
    +-- Check local manifest (filesystem)
    |   +-- Found? Return ModelResolution::Local(path)
    |
    +-- Check NestGate via AtomicClient
    |   +-- AtomicClient::discover("nestgate")
    |   +-- client.call("storage.retrieve", {
    |       family_id: "nat0",
    |       key: "model-cache:TinyLlama/..."
    |   })
    |   +-- Found? Return ModelResolution::Remote(gate_id)
    |
    +-- Return ModelResolution::NotFound
```

═══════════════════════════════════════════════════════════════════

## 🐛 BUGS IDENTIFIED BY UPSTREAM

### **Bug 1: Inverted Boolean in Socket-Only Mode** (REPORTED AS CRITICAL)

**Upstream Report**:
```
File: nestgate/code/crates/nestgate-bin/src/cli.rs line ~337

// CURRENT (BUG):
crate::commands::service::run_daemon(port, &bind, dev, use_socket_only)

// The 4th parameter is named `enable_http` in run_daemon():
pub async fn run_daemon(port: u16, bind: &str, dev: bool, enable_http: bool)

// When use_socket_only = true, enable_http receives true -> starts HTTP mode!
// FIX:
crate::commands::service::run_daemon(port, &bind, dev, !use_socket_only)
```

**INVESTIGATION RESULT**: ❌ **FALSE POSITIVE - BUG DOES NOT EXIST**

**Actual Code** (lines 91-107, 322-328):
```rust
// CLI definition:
Daemon {
    /// Enable HTTP server (socket-only is default for TRUE ecoBin compliance)
    #[arg(long)]
    enable_http: bool,
}

// Command handling:
Commands::Daemon { port, bind, dev, enable_http } => {
    if enable_http {
        tracing::info!("🌐 Starting NestGate with HTTP server enabled");
    } else {
        tracing::info!("🔌 Starting NestGate in socket-only mode (TRUE ecoBin - default)");
    }
    crate::commands::service::run_daemon(port, &bind, dev, enable_http)
        .await
```

**Analysis**:
- ✅ CLI parameter is named `enable_http` (not `use_socket_only`)
- ✅ Passed correctly to `run_daemon(enable_http: bool)`
- ✅ Logic is correct: `enable_http=false` → socket-only mode
- ✅ Default is `false` (socket-only), HTTP requires explicit `--enable-http`

**Conclusion**: No inversion bug exists. The code is correct!

**Possible Confusion**: Upstream may be running an old version or misread the parameter name.

---

### **Bug 2: `storage.retrieve` Returns Null** (CONFIRMED - INVESTIGATING)

**Upstream Report**:
```bash
# storage.store works:
{"jsonrpc":"2.0","result":{"key":"test:key","success":true},"id":1}

# storage.list works:
{"jsonrpc":"2.0","result":{"keys":["test:key"]},"id":2}

# storage.retrieve FAILS:
{"jsonrpc":"2.0","result":{"data":null},"id":3}
```

**Hypothesis (Upstream)**: 
> "store writes to one internal structure (index/transaction log) while retrieve 
> reads from a different path (filesystem) that was never written to."

**INVESTIGATION STATUS**: 🔍 **IN PROGRESS**

**Code Flow Analysis**:

1. **storage.store** (unix_socket_server.rs:378-425):
   ```rust
   async fn storage_store(params: &Option<Value>, state: &StorageState) -> Result<Value> {
       // Parse params
       let key = params["key"].as_str()?;
       let value = &params["value"];
       let family_id = params["family_id"].as_str()?;
       
       // Serialize value to bytes
       let data_bytes = serde_json::to_vec(value)?;
       
       // Store via StorageManagerService
       state.storage_manager
           .store_object(dataset, object_id, data_bytes)
           .await?;
   }
   ```

2. **storage.retrieve** (unix_socket_server.rs:455-489):
   ```rust
   async fn storage_retrieve(params: &Option<Value>, state: &StorageState) -> Result<Value> {
       // Parse params
       let key = params["key"].as_str()?;
       let family_id = params["family_id"].as_str()?;
       
       // Retrieve from StorageManagerService
       let (data_bytes, _info) = state.storage_manager
           .retrieve_object(dataset, object_id)
           .await?;
       
       // Deserialize bytes to JSON
       let data: Value = serde_json::from_slice(&data_bytes)?;
       
       Ok(json!({"data": data}))
   }
   ```

**Key Questions**:
- ❓ Does `store_object` actually write to persistent storage?
- ❓ Does `retrieve_object` read from the same location?
- ❓ Is there an error being swallowed that returns null instead?

**Next Steps**:
- [ ] Check `StorageManagerService::store_object` implementation
- [ ] Check `StorageManagerService::retrieve_object` implementation
- [ ] Verify filesystem backend is properly initialized
- [ ] Check if errors are being caught and returning null

---

### **Bug 3: ZFS Backend Assumed** (CONFIRMED - KNOWN ISSUE)

**Upstream Report**:
```
ERROR Command failed: zpool list -H -o name,size,alloc,free,health (exit code: 1)
ERROR Error output: The ZFS modules cannot be auto-loaded.
```

**Status**: ⚠️ **KNOWN ISSUE** - Non-fatal but noisy

**Impact**:
- Doesn't break functionality
- Causes error logs on non-ZFS systems
- Confusing for users without ZFS

**Current Behavior**:
- NestGate HTTP mode tries to execute `zpool list` on startup
- Fails gracefully but logs errors
- Should skip ZFS detection when not available

**Solution Required**:
- ✅ Check if ZFS is available before attempting commands
- ✅ Gracefully fallback to filesystem backend
- ✅ Only log errors if ZFS was explicitly configured

**Deep Debt Alignment**:
- Principle #5: Hardcoding Elimination
- Don't assume ZFS is available
- Use capability-based detection

---

### **Missing Feature: `storage.exists` Method** (ENHANCEMENT)

**Upstream Request**:
> Currently returns "Method not found". The model cache can work without it 
> (uses `retrieve` + null check) but a proper `exists` method would be more efficient.

**Status**: ✅ **EASY FIX** - Simple addition

**Implementation**:
```rust
// Add to unix_socket_server.rs JSON-RPC handler
"storage.exists" => storage_exists(&params, &state).await,

async fn storage_exists(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params.as_ref()?;
    let key = params["key"].as_str()?;
    let family_id = params["family_id"].as_str()?;
    
    let dataset = family_id;
    let object_id = key;
    
    // Check if object exists
    let exists = state.storage_manager
        .object_exists(dataset, object_id)
        .await?;
    
    Ok(json!({"exists": exists}))
}
```

**Benefits**:
- More efficient than retrieve + null check
- Standard storage API pattern
- Aligns with Deep Debt Principle #1 (Modern Idiomatic Rust)

═══════════════════════════════════════════════════════════════════

## 🎯 INVESTIGATION PLAN

### **Priority 1: Fix Bug 2 (`storage.retrieve` returns null)** 🔍

**Steps**:
1. ✅ Map storage.retrieve code flow
2. ⏳ Investigate `StorageManagerService::retrieve_object`
3. ⏳ Check filesystem backend implementation
4. ⏳ Verify store/retrieve use same path
5. ⏳ Test with actual biomeOS model cache data
6. ⏳ Fix and verify

### **Priority 2: Add `storage.exists` Method** ✅

**Steps**:
1. ⏳ Implement `storage_exists` handler
2. ⏳ Add to JSON-RPC router
3. ⏳ Test with biomeOS model cache
4. ⏳ Document in API

### **Priority 3: Fix Bug 3 (ZFS assumption)** ⚠️

**Steps**:
1. ⏳ Find ZFS detection code
2. ⏳ Add capability check before ZFS commands
3. ⏳ Implement graceful fallback to filesystem backend
4. ⏳ Test on non-ZFS system
5. ⏳ Update documentation

### **Priority 4: Clarify Bug 1 with Upstream** ℹ️

**Steps**:
1. ⏳ Document that bug does not exist in current code
2. ⏳ Ask upstream to verify their NestGate version
3. ⏳ Provide correct CLI usage examples

═══════════════════════════════════════════════════════════════════

## 📊 IMPACT ANALYSIS

### **Current State (Without Fixes)**:

```
❌ Cross-gate model discovery: BROKEN
   - storage.retrieve returns null
   - biomeOS can't find models on other gates
   
⚠️  ZFS errors: NOISY
   - Non-fatal but confusing
   - Logs errors on every startup
   
⚠️  storage.exists: MISSING
   - Works but inefficient
   - Uses retrieve + null check workaround
```

### **After Fixes**:

```
✅ Cross-gate model discovery: WORKING
   - storage.retrieve returns actual data
   - biomeOS can find models across mesh
   
✅ ZFS graceful fallback: CLEAN
   - No errors on non-ZFS systems
   - Automatic filesystem backend
   
✅ storage.exists: IMPLEMENTED
   - Efficient existence checks
   - Standard storage API
```

### **Evolution Opportunities** (Post-Fix):

1. **Mesh Discovery Works**: `resolve` on Tower finds Mistral-7B on gate2
2. **Model Transfer**: Add Songbird-based model file transfer
3. **Blob Storage**: Use `storage.store_blob` for smaller models
4. **Deduplication**: SHA256-based deduplication across gates
5. **Auto-sync**: Background task syncing model manifests

═══════════════════════════════════════════════════════════════════

## 🔄 DEEP DEBT ALIGNMENT

### **Principle #1: Modern Idiomatic Rust**
- ✅ Fix storage.retrieve (proper error handling)
- ✅ Add storage.exists (standard API pattern)
- ✅ Async/await throughout

### **Principle #5: Hardcoding Elimination**
- ✅ Don't assume ZFS is available
- ✅ Capability-based backend detection
- ✅ Graceful degradation

### **Principle #6: Primal Self-Knowledge**
- ✅ NestGate discovers its own capabilities
- ✅ Advertises available backends
- ✅ biomeOS discovers NestGate via AtomicClient

═══════════════════════════════════════════════════════════════════

## 📝 NEXT ACTIONS

### **Immediate** (This Session):
1. 🔍 Complete Bug 2 investigation (storage.retrieve)
2. 🔧 Implement fixes for all 3 bugs
3. ✅ Add storage.exists method
4. 🧪 Test with biomeOS model cache integration
5. 📚 Document findings and solutions

### **Follow-up** (Next Session):
1. 📮 Report findings to upstream (Bug 1 false positive)
2. 🤝 Coordinate with biomeOS team for testing
3. 🚀 Enable mesh-wide model discovery
4. 📊 Monitor production usage

═══════════════════════════════════════════════════════════════════

**Investigation Started**: February 9, 2026  
**Status**: 🔍 **IN PROGRESS** - Bug 2 investigation ongoing  
**Priority**: 🔥 **HIGH** - Blocking biomeOS model distribution

**🔍🐛🔧 INVESTIGATION CONTINUES...** 🔧🐛🔍
