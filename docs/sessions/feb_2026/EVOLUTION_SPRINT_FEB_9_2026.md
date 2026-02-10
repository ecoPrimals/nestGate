# NestGate Evolution Sprint - February 9, 2026

**Sprint Focus**: Bug fixes, Model Cache Methods, Discovery, Multi-Family Support

---

## Bug Status (All 4 Resolved)

### Bug 1: Inverted Boolean in CLI (CRITICAL)

**Status**: NOT PRESENT in NestGate main

**Investigation**: Thoroughly verified `code/crates/nestgate-bin/src/cli.rs` and
`code/crates/nestgate-bin/src/commands/service.rs`. NestGate uses a single
`enable_http: bool` parameter with correct semantics:

```rust
Commands::Daemon { port, bind, dev, enable_http, family_id } => {
    if enable_http {
        // HTTP mode
    } else {
        // Socket-only mode (default)
    }
}
```

**Root Cause**: This bug exists in the biomeOS fork which introduced a `socket_only`
field and passed it directly as `enable_http` (inverted). biomeOS should sync with
NestGate main to resolve.

### Bug 2: `storage.retrieve` Returns Null (FIXED)

**Root Cause FOUND**: Response format mismatch!

NestGate was returning `{"data": value}` but biomeOS expects `{"value": value}`.
When biomeOS reads `result.value`, it gets `null` because the key was `data`.

**Fix Applied**: Now returns both keys for universal compatibility:

```rust
Ok(json!({
    "value": data,    // biomeOS convention
    "data": data,     // legacy convention
    "key": key,
    "family_id": family_id
}))
```

### Bug 3: ZFS Backend Assumption (ALREADY FIXED)

Fixed in previous sprint with capability-based detection:
- `capabilities.rs`: Runtime ZFS detection via `zpool version`
- `config.rs`: `with_auto_detect()` dynamically configures backend
- Works on ext4, NTFS, APFS, btrfs, XFS, or any filesystem

### Bug 4: Missing `storage.exists` (ALREADY IMPLEMENTED)

Implemented in previous sprint. Efficient existence check without full data transfer.

---

## New Features Implemented

### 1. Model Cache Methods (~120 lines)

Thin wrappers over `storage.*` with `model:` key prefix for namespace isolation.

| Method | Description |
|--------|-------------|
| `model.register(model_id, metadata)` | Register model with metadata |
| `model.exists(model_id)` | Check if model is cached |
| `model.locate(model_id)` | Return gates that have model |
| `model.metadata(model_id)` | Return model registration info |

**Key Design Decisions**:
- Uses `model:` prefix for key isolation from regular storage
- `model.register` stores registration record with gate identity, timestamp
- `model.locate` currently returns local gate; ready for cross-gate mesh query
- `model.metadata` returns full registration including checksum
- `family_id` defaults to `"models"` if not specified

### 2. `discover_capabilities` JSON-RPC Method (~35 lines)

Returns all available JSON-RPC methods with backend info:

```json
{
    "primal": "nestgate",
    "version": "4.0.0",
    "capabilities": [
        "health", "discover_capabilities",
        "storage.store", "storage.retrieve", "storage.exists",
        "storage.delete", "storage.list", "storage.stats",
        "storage.store_blob", "storage.retrieve_blob",
        "model.register", "model.exists", "model.locate", "model.metadata",
        "templates.store", "templates.retrieve", "templates.list",
        "templates.community_top", "audit.store_execution"
    ],
    "backend": {
        "type": "filesystem",
        "features": {
            "persistent": true,
            "blob_storage": true,
            "model_cache": true,
            "templates": true,
            "audit": true
        }
    }
}
```

Deep Debt Principle #6: Primal Self-Knowledge via runtime capability advertisement.

### 3. `health` JSON-RPC Method

Simple health check returning `{"status": "healthy", "version": "4.0.0"}`.

### 4. Multi-Family Socket Support (~15 lines)

Added `--family-id` flag to daemon CLI command:

```bash
# Explicit family ID
nestgate daemon --family-id mylab

# Via environment variable
NESTGATE_FAMILY_ID=mylab nestgate daemon

# Creates socket: nestgate-mylab.sock
```

Resolution order: `--family-id` flag > `NESTGATE_FAMILY_ID` env var > default.

---

## Test Matrix (Updated)

| Test | Method | Expected |
|------|--------|----------|
| Health | `health` | `{"status":"healthy"}` |
| Capabilities | `discover_capabilities` | `{"capabilities":[...]}` |
| Store | `storage.store` | `{"success":true}` |
| Retrieve | `storage.retrieve` | `{"value":"v","data":"v"}` |
| Exists | `storage.exists` | `{"exists":true}` |
| Model Register | `model.register` | `{"registered":true}` |
| Model Exists | `model.exists` | `{"exists":true}` |
| Model Locate | `model.locate` | `{"gates":[...]}` |
| Model Metadata | `model.metadata` | `{"registration":{...}}` |

---

## Files Modified

| File | Changes |
|------|---------|
| `code/crates/nestgate-core/src/rpc/unix_socket_server.rs` | Bug 2 fix, model.* methods, discover_capabilities, health |
| `code/crates/nestgate-bin/src/cli.rs` | --family-id flag, updated dispatch |
| `code/crates/nestgate-bin/src/commands/service.rs` | Multi-family support, updated method listings |
| `README.md` | Updated achievements |

---

## Compilation

```
cargo check: ✅ SUCCESS (0 errors, 0 new warnings)
```

## Deep Debt Alignment

| Principle | Alignment |
|-----------|-----------|
| 1. Modern Idiomatic Rust | async/await, Result propagation, no unwraps |
| 2. Pure Rust | Zero external deps for new features |
| 3. Smart Refactoring | Model methods as thin wrappers, not monolithic |
| 4. Unsafe Evolution | Zero unsafe in all new code |
| 5. Hardcoding Elimination | Environment-first family_id, hostname detection |
| 6. Primal Self-Knowledge | discover_capabilities, runtime backend detection |
| 7. Mock Isolation | No mocks in production code |
