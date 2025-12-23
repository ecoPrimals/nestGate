# Data Federation Implementation Guide

This document outlines how to implement the `/api/v1/data/store` endpoint and related federation features in NestGate to support the data federation workflow.

## 🎯 Overview

To support the Songbird data federation workflow, NestGate needs:

1. **Data Storage API** - Accept and store arbitrary data/models
2. **Replication Receipts** - Return proof of storage for verification
3. **Sharding Support** - Distribute data across federation
4. **Health Metrics** - Report storage capacity and status

## 📋 Implementation Tasks

### Phase 1: Basic Storage API ✅ (Foundation Exists)

NestGate already has storage capabilities via ZFS. We need to expose them via HTTP API.

**New Endpoint**: `POST /api/v1/data/store`

**Request Format**:
```json
{
  "data_id": "llama-3-70b-v2024.12",
  "content": "<base64-encoded-data>",
  "metadata": {
    "type": "model",
    "size_gb": 140,
    "compression": "zstd"
  }
}
```

**Response Format**:
```json
{
  "status": "stored",
  "receipt": {
    "data_id": "llama-3-70b-v2024.12",
    "tower": "westgate",
    "stored_at": "2025-12-21T12:00:00Z",
    "checksum": "sha256:abc123...",
    "location": "zfs://pool0/models/llama-3-70b",
    "size_bytes": 150323855360
  }
}
```

**Implementation Location**: `nestgate-api/src/routes/data.rs`

```rust
// nestgate-api/src/routes/data.rs

use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct StoreRequest {
    pub data_id: String,
    pub content: String,  // base64 encoded
    #[serde(default)]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct StoreResponse {
    pub status: String,
    pub receipt: StorageReceipt,
}

#[derive(Debug, Serialize)]
pub struct StorageReceipt {
    pub data_id: String,
    pub tower: String,
    pub stored_at: String,
    pub checksum: String,
    pub location: String,
    pub size_bytes: u64,
}

pub struct DataApiState {
    storage: Arc<dyn StorageBackend>,
    tower_name: String,
}

pub fn data_routes(state: DataApiState) -> Router {
    Router::new()
        .route("/api/v1/data/store", post(store_data))
        .with_state(Arc::new(state))
}

async fn store_data(
    State(state): State<Arc<DataApiState>>,
    Json(request): Json<StoreRequest>,
) -> Result<Json<StoreResponse>, StatusCode> {
    // Decode base64 content
    let data = base64::decode(&request.content)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Store to ZFS
    let location = state.storage.store(&request.data_id, &data)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Calculate checksum
    let checksum = format!("sha256:{}", sha256_hash(&data));
    
    // Generate receipt
    let receipt = StorageReceipt {
        data_id: request.data_id,
        tower: state.tower_name.clone(),
        stored_at: chrono::Utc::now().to_rfc3339(),
        checksum,
        location,
        size_bytes: data.len() as u64,
    };
    
    Ok(Json(StoreResponse {
        status: "stored".to_string(),
        receipt,
    }))
}

trait StorageBackend: Send + Sync {
    async fn store(&self, id: &str, data: &[u8]) -> Result<String, StorageError>;
}

// TODO: Implement ZFS storage backend
```

### Phase 2: Replication API 🎯 (Next)

**New Endpoint**: `POST /api/v1/data/replicate`

Used for tower-to-tower replication.

**Request Format**:
```json
{
  "source_receipt": {
    "data_id": "llama-3-70b-v2024.12",
    "tower": "westgate",
    "checksum": "sha256:abc123..."
  },
  "replication_policy": {
    "verify_checksum": true,
    "compression": "zstd",
    "priority": "normal"
  }
}
```

**Response Format**:
```json
{
  "status": "replicated",
  "receipt": {
    "data_id": "llama-3-70b-v2024.12",
    "tower": "stradgate",
    "replicated_at": "2025-12-21T12:01:00Z",
    "source_tower": "westgate",
    "checksum_verified": true
  }
}
```

### Phase 3: Sharding Support 🎯 (Future)

Implement consistent hashing for data distribution:

```rust
// nestgate-core/src/federation/sharding.rs

pub struct ConsistentHashRing {
    nodes: Vec<Node>,
    virtual_nodes: HashMap<u64, String>,
}

impl ConsistentHashRing {
    pub fn assign_shard(&self, data_id: &str) -> Vec<String> {
        // Return 2+ tower names for redundancy
    }
}
```

### Phase 4: Federation Health Metrics 🎯 (Future)

**Enhanced Endpoint**: `GET /health`

Add federation-specific metrics:

```json
{
  "status": "healthy",
  "tower": "westgate",
  "role": "cold-storage",
  "storage": {
    "capacity_gb": 10000,
    "used_gb": 4200,
    "available_gb": 5800,
    "utilization_percent": 42
  },
  "federation": {
    "connected_towers": ["stradgate"],
    "replication_lag_ms": 120,
    "last_sync": "2025-12-21T12:00:00Z"
  }
}
```

## 🔧 Integration with Existing Code

### 1. Add to HTTP Server

In `nestgate-api/src/server.rs`:

```rust
use crate::routes::data::data_routes;

pub async fn start_server(config: ApiConfig) -> Result<()> {
    let app = Router::new()
        .merge(health_routes())
        .merge(data_routes(data_state))  // Add this
        .layer(/* ... */);
    
    // ...
}
```

### 2. Wire Up ZFS Backend

In `nestgate-core/src/storage/zfs.rs`:

```rust
impl StorageBackend for ZfsBackend {
    async fn store(&self, id: &str, data: &[u8]) -> Result<String, StorageError> {
        let path = format!("{}/models/{}", self.pool_path, id);
        tokio::fs::write(&path, data).await?;
        Ok(format!("zfs://{}/{}", self.pool_name, id))
    }
}
```

### 3. Configuration

In `nestgate.toml`:

```toml
[federation]
tower_name = "westgate"
role = "cold-storage"

[storage]
backend = "zfs"
pool_name = "pool0"
compression = "zstd"
dedup = true

[replication]
enabled = true
min_copies = 2
auto_replicate = true
```

## 🧪 Testing Strategy

### Unit Tests

```rust
#[tokio::test]
async fn test_store_data() {
    let state = setup_test_state();
    let request = StoreRequest {
        data_id: "test-model".to_string(),
        content: base64::encode("test data"),
        metadata: json!({}),
    };
    
    let response = store_data(State(state), Json(request)).await;
    assert!(response.is_ok());
}
```

### Integration Tests

```bash
# Test storage across federation
./showcase/01_nestgate_songbird_live/05-data-federation.sh

# Verify replication
curl -X POST http://localhost:7200/api/v1/data/store \
  -H "Content-Type: application/json" \
  -d '{"data_id": "test", "content": "dGVzdA=="}'

# Check it replicated to Stradgate
curl http://localhost:7202/api/v1/data/list
```

## 📊 Current Demo Status

The `05-data-federation.sh` demo is **functional** but uses:
- ✅ Real multi-node startup
- ✅ Real health checks
- ✅ Real Songbird orchestration
- 🚧 **Simulated** data storage (waiting for `/api/v1/data/store` endpoint)

Once the endpoint is implemented, the demo will be **fully live** with zero mocks.

## 🚀 Quick Implementation Path

**Fastest way to get live data federation**:

1. **5 minutes**: Add basic `/api/v1/data/store` endpoint (file write)
2. **10 minutes**: Add storage receipts with checksums
3. **15 minutes**: Test with `05-data-federation.sh`
4. **30 minutes**: Add ZFS integration
5. **1 hour**: Add replication API
6. **2 hours**: Add Songbird workflow executor

**Result**: Fully functional live data federation with real tower topology.

## 📖 References

- [Songbird Workflow](./workflows/data-federation.yaml)
- [Live Demo Script](./05-data-federation.sh)
- [NestGate API Code](../../nestgate-api/src/)
- [ZFS Backend](../../nestgate-core/src/storage/zfs.rs)

---

**Status**: 📋 IMPLEMENTATION GUIDE  
**Next Step**: Implement `/api/v1/data/store` endpoint  
**Estimated Time**: 1-2 hours for basic federation  
**Last Updated**: December 21, 2025

