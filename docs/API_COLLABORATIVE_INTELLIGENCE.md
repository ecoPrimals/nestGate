# 🤝 Collaborative Intelligence API Reference

**Complete API documentation for NestGate's Collaborative Intelligence features**

**Version**: 0.2.0  
**Status**: ✅ Production Ready (Phases 1-3 Complete)  
**Protocol**: JSON-RPC 2.0 over Unix Sockets

---

## 📋 **Overview**

NestGate provides 5 JSON-RPC methods for Collaborative Intelligence:

**Template Storage** (4 methods):
- `templates.store` - Save graph templates
- `templates.retrieve` - Get template by ID
- `templates.list` - List templates with filtering
- `templates.community_top` - Get top-ranked community templates

**Audit Trails** (1 method):
- `audit.store_execution` - Store execution audit data

All methods follow JSON-RPC 2.0 specification and support family-based isolation.

---

## 🔌 **Connection**

### **Unix Socket Path**

```
/run/user/{uid}/nestgate-{family_id}.sock
```

- `{uid}` - User's UID (discovered at runtime)
- `{family_id}` - From `$NESTGATE_FAMILY_ID` environment variable

### **Example Client Setup**

```rust
use tokio::net::UnixStream;

let family_id = std::env::var("NESTGATE_FAMILY_ID")?;
let uid = unsafe { libc::getuid() };
let socket_path = format!("/run/user/{}/nestgate-{}.sock", uid, family_id);

let stream = UnixStream::connect(&socket_path).await?;
```

---

## 📊 **Method: templates.store**

Store a new graph template with version control.

### **Request**

```json
{
  "jsonrpc": "2.0",
  "method": "templates.store",
  "params": {
    "name": "FastAPI CRUD Service",
    "description": "REST API with PostgreSQL database and Redis cache",
    "graph_data": {
      "nodes": [
        {"id": "api", "type": "fastapi", "config": {"port": 8000}},
        {"id": "db", "type": "postgres", "config": {"pool_size": 20}},
        {"id": "cache", "type": "redis"}
      ],
      "edges": [
        {"from": "api", "to": "db"},
        {"from": "api", "to": "cache"}
      ]
    },
    "user_id": "user_abc123",
    "family_id": "myapp",
    "metadata": {
      "tags": ["api", "rest", "database", "cache"],
      "niche_type": "web_service",
      "is_community": false
    }
  },
  "id": 1
}
```

### **Parameters**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | ✅ | Human-readable template name |
| `description` | string | ✅ | Template description |
| `graph_data` | object | ✅ | Full graph structure (JSON) |
| `user_id` | string | ✅ | Owner user ID |
| `family_id` | string | ✅ | Family identifier for isolation |
| `metadata` | object | ❌ | Template metadata (see below) |

**Metadata Fields** (all optional):

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `tags` | string[] | `[]` | Search tags |
| `niche_type` | string | `""` | Niche type (e.g., "web_service") |
| `usage_count` | integer | `0` | Number of times used |
| `success_rate` | float | `0.0` | Success rate (0.0 - 1.0) |
| `is_community` | boolean | `false` | Available to community |
| `community_rating` | float | `null` | Community rating (0.0 - 5.0) |
| `rating_count` | integer | `0` | Number of ratings |

### **Response**

```json
{
  "jsonrpc": "2.0",
  "result": {
    "template_id": "tmpl_a1b2c3d4e5f6",
    "version": 1,
    "created_at": "2026-01-10T15:30:00Z",
    "success": true
  },
  "id": 1
}
```

### **Errors**

- `-32602` Invalid params (missing required field)
- `-32603` Internal error (validation failed)

---

## 📖 **Method: templates.retrieve**

Retrieve a template by ID.

### **Request**

```json
{
  "jsonrpc": "2.0",
  "method": "templates.retrieve",
  "params": {
    "template_id": "tmpl_a1b2c3d4e5f6",
    "family_id": "myapp"
  },
  "id": 2
}
```

### **Parameters**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template_id` | string | ✅ | Template identifier |
| `family_id` | string | ✅ | Family identifier for isolation |

### **Response**

```json
{
  "jsonrpc": "2.0",
  "result": {
    "id": "tmpl_a1b2c3d4e5f6",
    "name": "FastAPI CRUD Service",
    "description": "REST API with PostgreSQL database and Redis cache",
    "graph_data": { ... },
    "user_id": "user_abc123",
    "family_id": "myapp",
    "version": 1,
    "created_at": "2026-01-10T15:30:00Z",
    "updated_at": "2026-01-10T15:30:00Z",
    "metadata": {
      "tags": ["api", "rest", "database", "cache"],
      "niche_type": "web_service",
      "usage_count": 0,
      "success_rate": 0.0,
      "is_community": false,
      "rating_count": 0
    }
  },
  "id": 2
}
```

### **Errors**

- `-32602` Invalid params
- `-32603` Not found (template doesn't exist or wrong family)

---

## 📝 **Method: templates.list**

List templates with optional filtering.

### **Request**

```json
{
  "jsonrpc": "2.0",
  "method": "templates.list",
  "params": {
    "family_id": "myapp",
    "user_id": "user_abc123",
    "tags": ["api", "rest"],
    "niche_type": "web_service",
    "is_community": false
  },
  "id": 3
}
```

### **Parameters**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `family_id` | string | ✅ | Family identifier |
| `user_id` | string | ❌ | Filter by user ID |
| `tags` | string[] | ❌ | Filter by tags (any match) |
| `niche_type` | string | ❌ | Filter by niche type |
| `is_community` | boolean | ❌ | Filter by community status |

### **Response**

```json
{
  "jsonrpc": "2.0",
  "result": {
    "templates": [
      {
        "id": "tmpl_a1b2c3d4e5f6",
        "name": "FastAPI CRUD Service",
        "description": "REST API with PostgreSQL database and Redis cache",
        "user_id": "user_abc123",
        "family_id": "myapp",
        "version": 1,
        "created_at": "2026-01-10T15:30:00Z",
        "updated_at": "2026-01-10T15:30:00Z",
        "metadata": { ... }
      }
    ],
    "total": 1
  },
  "id": 3
}
```

**Note**: Results sorted by `updated_at` (most recent first)

---

## 🏆 **Method: templates.community_top**

Get top-ranked community templates.

### **Request**

```json
{
  "jsonrpc": "2.0",
  "method": "templates.community_top",
  "params": {
    "niche_type": "web_service",
    "limit": 10,
    "min_usage": 5
  },
  "id": 4
}
```

### **Parameters**

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `niche_type` | string | ❌ | `null` | Filter by niche type |
| `limit` | integer | ❌ | `10` | Maximum results |
| `min_usage` | integer | ❌ | `0` | Minimum usage threshold |

### **Ranking Algorithm**

```
score = 0.4 × normalized_usage + 0.3 × success_rate + 0.3 × (rating / 5.0)
```

Where:
- `normalized_usage` = usage_count / max_usage_in_results
- `success_rate` = 0.0 to 1.0
- `rating` = community_rating (0.0 to 5.0)

### **Response**

```json
{
  "jsonrpc": "2.0",
  "result": {
    "templates": [
      {
        "id": "tmpl_popular123",
        "name": "Production FastAPI",
        "description": "Battle-tested API template",
        "score": 0.92,
        "usage_count": 150,
        "success_rate": 0.98,
        "community_rating": 4.9,
        "rating_count": 75,
        "metadata": {
          "tags": ["api", "production"],
          "niche_type": "web_service"
        }
      }
    ]
  },
  "id": 4
}
```

**Note**: Results sorted by score (highest first)

---

## 📊 **Method: audit.store_execution**

Store execution audit trail for AI learning.

### **Request**

```json
{
  "jsonrpc": "2.0",
  "method": "audit.store_execution",
  "params": {
    "id": "",
    "execution_id": "exec_xyz789",
    "graph_id": "graph_abc123",
    "template_id": "tmpl_a1b2c3d4e5f6",
    "user_id": "user_abc123",
    "family_id": "myapp",
    "started_at": "2026-01-10T16:00:00Z",
    "completed_at": "2026-01-10T16:05:30Z",
    "status": "completed",
    "modifications": [
      {
        "timestamp": "2026-01-10T16:01:00Z",
        "modification_type": "add_node",
        "node_id": "cache_redis",
        "data": {"type": "redis", "reason": "User added caching"}
      }
    ],
    "outcomes": [
      {
        "node_id": "api",
        "status": "success",
        "started_at": "2026-01-10T16:00:05Z",
        "completed_at": "2026-01-10T16:00:15Z",
        "duration_ms": 10000,
        "metrics": {"requests_handled": 1000}
      }
    ],
    "metadata": {}
  },
  "id": 5
}
```

### **Parameters**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | ❌ | Auto-generated if empty |
| `execution_id` | string | ✅ | Unique execution identifier |
| `graph_id` | string | ✅ | Graph identifier |
| `template_id` | string | ❌ | Template used (if any) |
| `user_id` | string | ✅ | User who executed |
| `family_id` | string | ✅ | Family identifier |
| `started_at` | string | ✅ | ISO 8601 timestamp |
| `completed_at` | string | ❌ | ISO 8601 timestamp |
| `status` | string | ✅ | running, completed, failed, cancelled |
| `modifications` | array | ❌ | Graph modifications (see below) |
| `outcomes` | array | ❌ | Node outcomes (see below) |
| `metadata` | object | ❌ | Additional metadata |

**Modification Object**:

| Field | Type | Description |
|-------|------|-------------|
| `timestamp` | string | ISO 8601 timestamp |
| `modification_type` | string | add_node, remove_node, modify_node, add_edge, remove_edge |
| `node_id` | string | Affected node ID (optional) |
| `data` | object | Modification data |

**Outcome Object**:

| Field | Type | Description |
|-------|------|-------------|
| `node_id` | string | Node identifier |
| `status` | string | success, failed, skipped |
| `started_at` | string | ISO 8601 timestamp |
| `completed_at` | string | ISO 8601 timestamp (optional) |
| `duration_ms` | integer | Duration in milliseconds |
| `error` | string | Error message (if failed) |
| `metrics` | object | Node-specific metrics |

### **Response**

```json
{
  "jsonrpc": "2.0",
  "result": {
    "audit_id": "audit_m9n8b7v6c5",
    "stored_at": "2026-01-10T16:06:00Z",
    "success": true
  },
  "id": 5
}
```

---

## 🔒 **Security & Isolation**

### **Family-Based Isolation**

All methods enforce **family-based isolation**:

```rust
// ✅ Can only access own family's data
templates.list(family_id: "myapp")  // ✅ Returns myapp templates

// ❌ Cannot access other family's data
templates.retrieve(template_id: "tmpl_xyz", family_id: "other_family")
// Returns error: "Template not found"
```

### **Multi-Tenant Safety**

- Each family has completely isolated storage
- No cross-family queries possible
- Template IDs are globally unique but family-scoped
- Audit trails isolated by family

---

## 📊 **Error Handling**

### **Standard JSON-RPC Errors**

| Code | Message | Description |
|------|---------|-------------|
| `-32700` | Parse error | Invalid JSON |
| `-32600` | Invalid Request | Not JSON-RPC 2.0 |
| `-32601` | Method not found | Unknown method |
| `-32602` | Invalid params | Missing/invalid parameters |
| `-32603` | Internal error | Server-side error |

### **Example Error Response**

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32602,
    "message": "Invalid params",
    "data": {
      "error": "Validation error: name (string) required"
    }
  },
  "id": 1
}
```

---

## 🚀 **Usage Examples**

### **Example 1: Store and Retrieve Template**

```rust
// Store template
let store_request = json!({
    "jsonrpc": "2.0",
    "method": "templates.store",
    "params": {
        "name": "ML Pipeline",
        "description": "TensorFlow training pipeline",
        "graph_data": {...},
        "user_id": "data_scientist_1",
        "family_id": "ml_team",
        "metadata": {
            "tags": ["ml", "tensorflow"],
            "niche_type": "ml_pipeline"
        }
    },
    "id": 1
});

let response = send_request(store_request).await?;
let template_id = response["result"]["template_id"].as_str().unwrap();

// Retrieve it back
let retrieve_request = json!({
    "jsonrpc": "2.0",
    "method": "templates.retrieve",
    "params": {
        "template_id": template_id,
        "family_id": "ml_team"
    },
    "id": 2
});

let template = send_request(retrieve_request).await?;
println!("Template: {}", template["result"]["name"]);
```

### **Example 2: Discover Community Templates**

```rust
let request = json!({
    "jsonrpc": "2.0",
    "method": "templates.community_top",
    "params": {
        "niche_type": "web_service",
        "limit": 5,
        "min_usage": 10
    },
    "id": 3
});

let response = send_request(request).await?;
for template in response["result"]["templates"].as_array().unwrap() {
    println!("{} - Score: {}", 
        template["name"], 
        template["score"]);
}
```

### **Example 3: Store Execution Audit**

```rust
let audit_request = json!({
    "jsonrpc": "2.0",
    "method": "audit.store_execution",
    "params": {
        "id": "",
        "execution_id": format!("exec_{}", uuid::Uuid::new_v4()),
        "graph_id": "graph_123",
        "user_id": "user_abc",
        "family_id": "myapp",
        "started_at": chrono::Utc::now().to_rfc3339(),
        "status": "running",
        "modifications": [],
        "outcomes": [],
        "metadata": {}
    },
    "id": 4
});

let response = send_request(audit_request).await?;
println!("Audit ID: {}", response["result"]["audit_id"]);
```

---

## 📚 **Further Reading**

- **[COLLABORATIVE_INTELLIGENCE_TRACKER.md](../COLLABORATIVE_INTELLIGENCE_TRACKER.md)** - Implementation progress
- **[specs/COLLABORATIVE_INTELLIGENCE_IMPLEMENTATION.md](../specs/COLLABORATIVE_INTELLIGENCE_IMPLEMENTATION.md)** - Technical specification
- **[QUICK_START_BIOMEOS.md](../QUICK_START_BIOMEOS.md)** - biomeOS integration guide

---

**Version**: 0.2.0  
**Status**: ✅ Production Ready  
**Last Updated**: January 10, 2026
