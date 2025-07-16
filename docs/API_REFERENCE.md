# NestGate API Reference

This document provides comprehensive reference for the NestGate REST API, including all endpoints, request/response formats, authentication methods, and error handling.

## Base URL
```
http://localhost:8080/api/v1
```

## Authentication

### Standalone Mode
In standalone mode, no authentication is required for local connections.

### BearDog Integration
When integrated with BearDog, all requests require authentication:

```bash
# Get authentication token
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "your_username",
    "password": "your_password"
  }'

# Use token in subsequent requests
curl -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  http://localhost:8080/api/v1/pools
```

---

## Pool Management

### `GET /pools`
List all ZFS pools

**Response:**
```json
{
  "data": [
    {
      "name": "datapool",
      "status": "ONLINE",
      "size": "1.81T",
      "allocated": "500G",
      "free": "1.31T",
      "health": "HEALTHY",
      "devices": ["/dev/sdb"],
      "properties": {
        "compression": "lz4",
        "dedup": "off"
      }
    }
  ],
  "metadata": {
    "total_pools": 1,
    "timestamp": "2024-01-26T10:30:00Z"
  }
}
```

### `POST /pools`
Create a new ZFS pool

**Request:**
```json
{
  "name": "newpool",
  "devices": ["/dev/sdc", "/dev/sdd"],
  "pool_type": "mirror",
  "properties": {
    "compression": "gzip",
    "atime": "off"
  }
}
```

**Response:**
```json
{
  "data": {
    "pool_id": "pool_abc123",
    "name": "newpool",
    "status": "ONLINE",
    "message": "Pool created successfully"
  },
  "metadata": {
    "operation_id": "op_xyz789",
    "timestamp": "2024-01-26T10:35:00Z"
  }
}
```

### `GET /pools/{pool_name}`
Get detailed information about a specific pool

**Response:**
```json
{
  "data": {
    "name": "datapool",
    "status": "ONLINE",
    "size": "1.81T",
    "allocated": "500G",
    "free": "1.31T",
    "health": "HEALTHY",
    "devices": [
      {
        "name": "/dev/sdb",
        "status": "ONLINE",
        "read_errors": 0,
        "write_errors": 0,
        "checksum_errors": 0
      }
    ],
    "properties": {
      "compression": "lz4",
      "dedup": "off",
      "atime": "on"
    },
    "datasets": [
      {
        "name": "datapool/research",
        "size": "100G",
        "used": "45G"
      }
    ]
  }
}
```

### `DELETE /pools/{pool_name}`
Destroy a ZFS pool

**Response:**
```json
{
  "data": {
    "message": "Pool destroyed successfully",
    "pool_name": "oldpool"
  },
  "metadata": {
    "operation_id": "op_destroy_123",
    "timestamp": "2024-01-26T10:40:00Z"
  }
}
```

---

## Dataset Management

### `GET /datasets`
List all datasets

**Query Parameters:**
- `pool` (optional): Filter by pool name
- `limit` (optional): Maximum number of results (default: 100)
- `offset` (optional): Number of results to skip (default: 0)

**Response:**
```json
{
  "data": [
    {
      "name": "datapool/research",
      "pool": "datapool",
      "size": "100G",
      "used": "45G",
      "available": "55G",
      "referenced": "45G",
      "compression_ratio": "1.5x",
      "properties": {
        "quota": "100G",
        "compression": "lz4",
        "recordsize": "1M"
      }
    }
  ],
  "metadata": {
    "total_datasets": 1,
    "page": 1,
    "per_page": 100
  }
}
```

### `POST /datasets`
Create a new dataset

**Request:**
```json
{
  "pool": "datapool",
  "name": "experiments",
  "properties": {
    "quota": "200G",
    "compression": "gzip",
    "recordsize": "1M",
    "sync": "standard"
  }
}
```

**Response:**
```json
{
  "data": {
    "dataset_id": "ds_abc123",
    "name": "datapool/experiments",
    "message": "Dataset created successfully"
  },
  "metadata": {
    "operation_id": "op_create_dataset_456",
    "timestamp": "2024-01-26T11:00:00Z"
  }
}
```

---

## BYOB Workspace Management

### `GET /byob/workspaces`
List all BYOB workspaces

**Response:**
```json
{
  "data": [
    {
      "workspace_id": "ws_123",
      "name": "ml-experiments",
      "description": "Machine learning experiments",
      "storage_tier": "warm",
      "storage_gb": 500,
      "used_gb": 245,
      "backup_frequency": "daily",
      "status": "active",
      "created_at": "2024-01-15T09:00:00Z",
      "last_backup": "2024-01-26T02:00:00Z"
    }
  ],
  "metadata": {
    "total_workspaces": 1
  }
}
```

### `POST /byob/workspaces`
Create a new BYOB workspace

**Request:**
```json
{
  "name": "genomics-lab",
  "description": "Genomics research workspace",
  "storage_tier": "hot",
  "storage_gb": 1000,
  "backup_frequency": "hourly",
  "features": {
    "gpu_support": true,
    "jupyter_enabled": true,
    "git_integration": true
  },
  "sharing": {
    "enabled": true,
    "access_level": "read-write",
    "users": ["alice", "bob"]
  }
}
```

**Response:**
```json
{
  "data": {
    "workspace_id": "ws_456",
    "name": "genomics-lab",
    "mount_point": "/mnt/nestgate/workspaces/genomics-lab",
    "message": "Workspace created successfully"
  },
  "metadata": {
    "operation_id": "op_create_workspace_789",
    "timestamp": "2024-01-26T11:15:00Z"
  }
}
```

### `POST /byob/workspaces/{workspace_id}/backup`
Create a backup of a workspace

**Request:**
```json
{
  "backup_type": "full",
  "description": "Pre-experiment backup",
  "retention_days": 30
}
```

**Response:**
```json
{
  "data": {
    "backup_id": "backup_abc123",
    "snapshot_name": "genomics-lab@2024-01-26_11-20-00",
    "size_gb": 245,
    "message": "Backup created successfully"
  },
  "metadata": {
    "operation_id": "op_backup_456",
    "timestamp": "2024-01-26T11:20:00Z"
  }
}
```

### `POST /byob/workspaces/{workspace_id}/restore`
Restore a workspace from backup

**Request:**
```json
{
  "backup_id": "backup_abc123",
  "restore_type": "in-place",
  "confirm": true
}
```

**Response:**
```json
{
  "data": {
    "restore_id": "restore_def456",
    "message": "Restore completed successfully",
    "restored_size_gb": 245
  },
  "metadata": {
    "operation_id": "op_restore_789",
    "timestamp": "2024-01-26T11:25:00Z"
  }
}
```

---

## AI-Powered Features

### `GET /ai/status`
Get AI service status

**Response:**
```json
{
  "data": {
    "ai_enabled": true,
    "squirrel_connected": true,
    "features": {
      "tier_migration": true,
      "predictive_maintenance": true,
      "capacity_planning": true
    },
    "last_analysis": "2024-01-26T10:00:00Z"
  }
}
```

### `GET /ai/recommendations`
Get AI recommendations for optimization

**Query Parameters:**
- `workspace` (optional): Get recommendations for specific workspace
- `type` (optional): Filter by recommendation type (tier, performance, maintenance)

**Response:**
```json
{
  "data": [
    {
      "recommendation_id": "rec_123",
      "type": "tier_migration",
      "workspace": "ml-experiments",
      "title": "Migrate cold data to archive tier",
      "description": "Files not accessed in 30 days can be moved to cold tier",
      "priority": "medium",
      "estimated_savings": {
        "cost": 0.15,
        "performance_impact": "minimal"
      },
      "actions": [
        {
          "action": "migrate_files",
          "files": ["/data/old-experiments/*"],
          "target_tier": "cold"
        }
      ]
    }
  ],
  "metadata": {
    "total_recommendations": 1,
    "analysis_timestamp": "2024-01-26T10:00:00Z"
  }
}
```

### `POST /ai/tier-migration/enable`
Enable AI-powered tier migration

**Request:**
```json
{
  "enabled": true,
  "hot_threshold": 0.9,
  "cold_threshold": 0.1,
  "analysis_window_days": 7,
  "dry_run": false
}
```

**Response:**
```json
{
  "data": {
    "message": "AI tier migration enabled",
    "next_analysis": "2024-01-26T12:00:00Z"
  }
}
```

---

## Data Source Integration

### `GET /data-sources/ncbi/search`
Search NCBI databases

**Query Parameters:**
- `database`: Database to search (genome, pubmed, etc.)
- `query`: Search query
- `limit`: Maximum results (default: 10)

**Response:**
```json
{
  "data": [
    {
      "accession": "GCA_000001405.29",
      "title": "Homo sapiens genome assembly",
      "organism": "Homo sapiens",
      "size_gb": 3.2,
      "last_updated": "2024-01-15T00:00:00Z"
    }
  ],
  "metadata": {
    "database": "genome",
    "total_results": 1,
    "query": "Homo sapiens[Organism]"
  }
}
```

### `POST /data-sources/ncbi/download`
Download data from NCBI

**Request:**
```json
{
  "accession": "GCA_000001405.29",
  "workspace": "genomics-lab",
  "cache_tier": "warm",
  "verify_checksum": true
}
```

**Response:**
```json
{
  "data": {
    "download_id": "dl_ncbi_123",
    "status": "started",
    "estimated_size_gb": 3.2,
    "estimated_time_minutes": 15
  },
  "metadata": {
    "operation_id": "op_download_456",
    "timestamp": "2024-01-26T11:30:00Z"
  }
}
```

### `GET /data-sources/huggingface/models`
List available HuggingFace models

**Query Parameters:**
- `task`: Filter by task (text-generation, image-classification, etc.)
- `limit`: Maximum results (default: 20)

**Response:**
```json
{
  "data": [
    {
      "model_id": "microsoft/DialoGPT-medium",
      "task": "text-generation",
      "downloads": 1000000,
      "size_gb": 1.2,
      "license": "MIT"
    }
  ],
  "metadata": {
    "total_models": 1,
    "task_filter": "text-generation"
  }
}
```

---

## Metrics and Monitoring

### `GET /metrics/system`
Get comprehensive system metrics

**Response:**
```json
{
  "data": {
    "cpu_usage": 45.2,
    "memory_usage": {
      "used_gb": 8.4,
      "total_gb": 32.0,
      "percentage": 26.25
    },
    "storage": {
      "total_tb": 1.81,
      "used_tb": 0.5,
      "free_tb": 1.31,
      "percentage": 27.6
    },
    "network": {
      "rx_mbps": 125.5,
      "tx_mbps": 89.2
    },
    "zfs": {
      "arc_size_gb": 4.2,
      "arc_hit_ratio": 0.95,
      "scrub_status": "completed"
    }
  },
  "metadata": {
    "timestamp": "2024-01-26T11:45:00Z",
    "collection_interval_seconds": 30
  }
}
```

### `GET /metrics/workspaces/{workspace_id}`
Get metrics for a specific workspace

**Response:**
```json
{
  "data": {
    "workspace_id": "ws_123",
    "name": "ml-experiments",
    "storage_utilization": {
      "used_gb": 245,
      "allocated_gb": 500,
      "percentage": 49.0
    },
    "performance": {
      "read_iops": 1250,
      "write_iops": 890,
      "read_mbps": 125.5,
      "write_mbps": 89.2,
      "latency_ms": 2.5
    },
    "tier_distribution": {
      "hot": 50,
      "warm": 150,
      "cold": 45
    }
  },
  "metadata": {
    "timestamp": "2024-01-26T11:50:00Z"
  }
}
```

---

## Error Handling

### Error Response Format
All API errors follow this format:

```json
{
  "error": {
    "code": "POOL_NOT_FOUND",
    "message": "Pool 'nonexistent' not found",
    "details": {
      "pool_name": "nonexistent",
      "available_pools": ["datapool", "backuppool"]
    }
  },
  "metadata": {
    "request_id": "req_abc123",
    "timestamp": "2024-01-26T12:00:00Z"
  }
}
```

### Common Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `POOL_NOT_FOUND` | 404 | Specified pool does not exist |
| `WORKSPACE_NOT_FOUND` | 404 | Specified workspace does not exist |
| `INSUFFICIENT_SPACE` | 400 | Not enough space for operation |
| `INVALID_POOL_TYPE` | 400 | Invalid pool type specified |
| `DEVICE_BUSY` | 409 | Device is already in use |
| `UNAUTHORIZED` | 401 | Authentication required |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `RATE_LIMITED` | 429 | Too many requests |
| `INTERNAL_ERROR` | 500 | Internal server error |

---

## WebSocket API

### Connection
```javascript
const ws = new WebSocket('ws://localhost:8080/api/v1/ws');
```

### Event Types

#### Pool Events
```json
{
  "event": "pool_status_changed",
  "data": {
    "pool_name": "datapool",
    "old_status": "ONLINE",
    "new_status": "DEGRADED",
    "timestamp": "2024-01-26T12:05:00Z"
  }
}
```

#### Workspace Events
```json
{
  "event": "workspace_backup_completed",
  "data": {
    "workspace_id": "ws_123",
    "backup_id": "backup_abc123",
    "size_gb": 245,
    "duration_seconds": 180,
    "timestamp": "2024-01-26T12:10:00Z"
  }
}
```

#### Metrics Events
```json
{
  "event": "metrics_update",
  "data": {
    "type": "system",
    "cpu_usage": 52.1,
    "memory_usage": 28.5,
    "timestamp": "2024-01-26T12:15:00Z"
  }
}
```

---

## Rate Limiting

### Default Limits
- **General API**: 1000 requests per hour
- **Metrics endpoints**: 6000 requests per hour
- **WebSocket connections**: 10 concurrent connections per IP

### Rate Limit Headers
```
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1706270400
```

---

## SDK Examples

### Python SDK
```python
import nestgate

client = nestgate.Client(base_url="http://localhost:8080")

# Create workspace
workspace = client.create_workspace(
    name="test-workspace",
    storage_gb=100,
    tier="warm"
)

# Monitor metrics
metrics = client.get_workspace_metrics(workspace.id)
print(f"Usage: {metrics.storage_utilization.percentage}%")
```

### JavaScript SDK
```javascript
const NestGate = require('nestgate-js');

const client = new NestGate.Client('http://localhost:8080');

// Create workspace
const workspace = await client.createWorkspace({
  name: 'test-workspace',
  storageGb: 100,
  tier: 'warm'
});

// Monitor metrics
const metrics = await client.getWorkspaceMetrics(workspace.id);
console.log(`Usage: ${metrics.storageUtilization.percentage}%`);
```

---

## OpenAPI Specification

The complete OpenAPI 3.0 specification is available at:
- JSON: `http://localhost:8080/api/v1/openapi.json`
- YAML: `http://localhost:8080/api/v1/openapi.yaml`
- Interactive docs: `http://localhost:8080/api/docs`

---

*This API reference is automatically generated from the OpenAPI specification. For the most up-to-date information, refer to the interactive documentation at `/api/docs`.* 