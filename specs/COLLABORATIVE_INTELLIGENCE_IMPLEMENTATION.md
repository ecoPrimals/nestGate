# 🤝 Collaborative Intelligence - NestGate Implementation Specification

**Version**: 1.0  
**Date**: January 10, 2026  
**Status**: 📋 Approved - Implementation Pending  
**Owner**: NestGate Team  
**Timeline**: 3 weeks (Jan 13 - Feb 2, 2026)

---

## 📋 **OVERVIEW**

### **Purpose**

Implement template storage and audit trail capabilities to support biomeOS's Collaborative Intelligence initiative, enabling human-AI collaboration for 10x faster system deployments.

### **Context**

biomeOS is building a collaborative intelligence system where:
- Users can view and modify graphs in real-time
- AI learns from user modifications
- Templates accelerate bootstrap of new systems
- Audit trails enable learning from execution history

NestGate provides the **persistent memory** for this system.

---

## 🎯 **REQUIREMENTS**

### **Functional Requirements**

**FR1: Template Storage**
- Store graph templates with metadata
- Version control for templates
- User-specific and community templates
- Template search and filtering

**FR2: Template Retrieval**
- Retrieve templates by ID
- List templates with filters
- Query top community templates
- Version history access

**FR3: Audit Trails**
- Store execution audit data
- Track graph modifications during execution
- Record outcomes and metrics
- Query audit history

### **Non-Functional Requirements**

**NFR1: Performance**
- Template storage: < 100ms
- Template retrieval: < 50ms
- Audit storage: < 200ms (larger payload)
- Support 1000+ concurrent users

**NFR2: Scalability**
- Handle 100K+ templates
- Store 1M+ audit records
- Efficient community ranking

**NFR3: Quality**
- A-grade implementation (90+/100)
- 100% test coverage for new methods
- Full documentation

---

## 🏗️ **ARCHITECTURE**

### **System Context**

```
┌─────────────┐
│  biomeOS    │
│  GraphOS    │
└──────┬──────┘
       │ JSON-RPC
       │ Unix Socket
       ▼
┌─────────────────────────────────────┐
│  NestGate Unix Socket Server        │
│  ┌───────────────────────────────┐  │
│  │  Existing Methods (7)         │  │
│  │  - storage.store              │  │
│  │  - storage.retrieve           │  │
│  │  - storage.delete             │  │
│  │  - storage.list               │  │
│  │  - storage.stats              │  │
│  │  - storage.store_blob         │  │
│  │  - storage.retrieve_blob      │  │
│  └───────────────────────────────┘  │
│  ┌───────────────────────────────┐  │
│  │  NEW: Template Methods (4)    │  │
│  │  - templates.store            │  │
│  │  - templates.retrieve         │  │
│  │  - templates.list             │  │
│  │  - templates.community_top    │  │
│  └───────────────────────────────┘  │
│  ┌───────────────────────────────┐  │
│  │  NEW: Audit Methods (1)       │  │
│  │  - audit.store_execution      │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────┐
│  Storage Backend                    │
│  (In-memory → Future: ZFS/RocksDB)  │
└─────────────────────────────────────┘
```

### **Data Model**

**GraphTemplate**:
```rust
pub struct GraphTemplate {
    /// Unique template identifier
    pub id: String,
    
    /// Human-readable template name
    pub name: String,
    
    /// Template description
    pub description: String,
    
    /// Full graph structure (JSON)
    pub graph_data: serde_json::Value,
    
    /// Owner user ID
    pub user_id: String,
    
    /// Family/app identifier
    pub family_id: String,
    
    /// Current version number
    pub version: u32,
    
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
    
    /// Template metadata
    pub metadata: TemplateMetadata,
}

pub struct TemplateMetadata {
    /// Search tags
    pub tags: Vec<String>,
    
    /// Niche type (e.g., "web_service", "ml_pipeline")
    pub niche_type: String,
    
    /// Number of times used
    pub usage_count: u64,
    
    /// Success rate (0.0 - 1.0)
    pub success_rate: f64,
    
    /// Available to community
    pub is_community: bool,
    
    /// Community rating (0.0 - 5.0)
    pub community_rating: Option<f64>,
    
    /// Number of ratings
    pub rating_count: u64,
}
```

**ExecutionAudit**:
```rust
pub struct ExecutionAudit {
    /// Unique audit identifier
    pub id: String,
    
    /// Execution identifier
    pub execution_id: String,
    
    /// Graph identifier
    pub graph_id: String,
    
    /// Optional template used
    pub template_id: Option<String>,
    
    /// User who executed
    pub user_id: String,
    
    /// Family/app identifier
    pub family_id: String,
    
    /// Execution start time
    pub started_at: chrono::DateTime<chrono::Utc>,
    
    /// Execution completion time
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Status (running, completed, failed)
    pub status: ExecutionStatus,
    
    /// Graph modifications during execution
    pub modifications: Vec<GraphModification>,
    
    /// Node execution outcomes
    pub outcomes: Vec<NodeOutcome>,
    
    /// Additional metadata
    pub metadata: AuditMetadata,
}

pub struct GraphModification {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub modification_type: ModificationType,
    pub node_id: Option<String>,
    pub data: serde_json::Value,
}

pub enum ModificationType {
    AddNode,
    RemoveNode,
    ModifyNode,
    AddEdge,
    RemoveEdge,
}

pub struct NodeOutcome {
    pub node_id: String,
    pub status: NodeStatus,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duration_ms: u64,
    pub error: Option<String>,
    pub metrics: serde_json::Value,
}

pub enum ExecutionStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
}

pub enum NodeStatus {
    Success,
    Failed,
    Skipped,
}
```

---

## 📡 **API SPECIFICATION**

### **1. templates.store**

**Purpose**: Store a graph template with metadata and versioning.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "templates.store",
  "params": {
    "name": "FastAPI CRUD Service",
    "description": "REST API with PostgreSQL database and authentication",
    "graph_data": {
      "nodes": [...],
      "edges": [...],
      "metadata": {...}
    },
    "user_id": "user_abc123",
    "family_id": "myapp",
    "metadata": {
      "tags": ["api", "rest", "database", "auth"],
      "niche_type": "web_service",
      "is_community": false
    }
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "template_id": "tmpl_abc123def456",
    "version": 1,
    "created_at": "2026-01-13T10:00:00Z",
    "success": true
  },
  "id": 1
}
```

**Error Cases**:
- Invalid graph_data structure → -32602 (Invalid params)
- Missing required fields → -32602 (Invalid params)
- Storage failure → -32603 (Internal error)

---

### **2. templates.retrieve**

**Purpose**: Retrieve a specific template by ID.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "templates.retrieve",
  "params": {
    "template_id": "tmpl_abc123def456",
    "family_id": "myapp",
    "version": 1  // Optional, defaults to latest
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "id": "tmpl_abc123def456",
    "name": "FastAPI CRUD Service",
    "description": "REST API with PostgreSQL database and authentication",
    "graph_data": {
      "nodes": [...],
      "edges": [...],
      "metadata": {...}
    },
    "user_id": "user_abc123",
    "version": 1,
    "created_at": "2026-01-13T10:00:00Z",
    "updated_at": "2026-01-13T10:00:00Z",
    "metadata": {
      "tags": ["api", "rest", "database", "auth"],
      "niche_type": "web_service",
      "usage_count": 42,
      "success_rate": 0.95,
      "is_community": false
    }
  },
  "id": 1
}
```

**Error Cases**:
- Template not found → -32603 with "Template not found" message
- Permission denied → -32603 with "Access denied" message

---

### **3. templates.list**

**Purpose**: List templates with filtering.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "templates.list",
  "params": {
    "user_id": "user_abc123",  // Optional: filter by user
    "family_id": "myapp",
    "filters": {
      "tags": ["api"],  // Optional: filter by tags
      "niche_type": "web_service",  // Optional: filter by niche
      "is_community": false  // Optional: filter community/private
    },
    "sort": {
      "field": "usage_count",  // created_at, updated_at, usage_count, success_rate
      "order": "desc"  // asc, desc
    },
    "pagination": {
      "offset": 0,
      "limit": 20
    }
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "templates": [
      {
        "id": "tmpl_abc123def456",
        "name": "FastAPI CRUD Service",
        "description": "REST API with PostgreSQL...",
        "version": 1,
        "created_at": "2026-01-13T10:00:00Z",
        "updated_at": "2026-01-13T10:00:00Z",
        "metadata": {
          "tags": ["api", "rest", "database", "auth"],
          "niche_type": "web_service",
          "usage_count": 42,
          "success_rate": 0.95,
          "is_community": false
        }
      }
    ],
    "total": 156,
    "offset": 0,
    "limit": 20
  },
  "id": 1
}
```

---

### **4. templates.community_top**

**Purpose**: Get top-ranked community templates.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "templates.community_top",
  "params": {
    "niche_type": "web_service",  // Optional: filter by niche
    "limit": 10,
    "min_usage": 10  // Optional: minimum usage count
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "templates": [
      {
        "id": "tmpl_xyz789abc123",
        "name": "Production-Ready Web API",
        "description": "Battle-tested API template",
        "score": 0.98,  // Composite score
        "usage_count": 256,
        "success_rate": 0.97,
        "community_rating": 4.8,
        "rating_count": 89,
        "metadata": {
          "tags": ["api", "production", "monitoring"],
          "niche_type": "web_service"
        }
      }
    ]
  },
  "id": 1
}
```

**Ranking Algorithm**:
```
score = (
    0.4 * normalized_usage_count +
    0.3 * success_rate +
    0.3 * (community_rating / 5.0)
)
```

---

### **5. audit.store_execution**

**Purpose**: Store execution audit trail for learning.

**Request**:
```json
{
  "jsonrpc": "2.0",
  "method": "audit.store_execution",
  "params": {
    "execution_id": "exec_abc123xyz789",
    "graph_id": "graph_def456",
    "template_id": "tmpl_abc123def456",  // Optional
    "user_id": "user_abc123",
    "family_id": "myapp",
    "started_at": "2026-01-13T10:00:00Z",
    "completed_at": "2026-01-13T10:05:30Z",
    "status": "completed",  // running, completed, failed, cancelled
    "modifications": [
      {
        "timestamp": "2026-01-13T10:02:15Z",
        "modification_type": "add_node",
        "node_id": "node_new_cache",
        "data": {
          "type": "redis_cache",
          "config": {...}
        }
      },
      {
        "timestamp": "2026-01-13T10:03:30Z",
        "modification_type": "modify_node",
        "node_id": "node_database",
        "data": {
          "connection_pool": 20
        }
      }
    ],
    "outcomes": [
      {
        "node_id": "node_api",
        "status": "success",
        "started_at": "2026-01-13T10:00:05Z",
        "completed_at": "2026-01-13T10:01:23Z",
        "duration_ms": 78000,
        "metrics": {
          "requests_served": 1000,
          "avg_latency_ms": 45
        }
      },
      {
        "node_id": "node_database",
        "status": "success",
        "started_at": "2026-01-13T10:00:05Z",
        "completed_at": "2026-01-13T10:05:30Z",
        "duration_ms": 325000,
        "metrics": {
          "rows_processed": 50000
        }
      }
    ],
    "metadata": {
      "environment": "production",
      "region": "us-west-2",
      "total_cost": 0.45
    }
  },
  "id": 1
}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "audit_id": "audit_abc123xyz789",
    "stored_at": "2026-01-13T10:05:31Z",
    "success": true
  },
  "id": 1
}
```

---

## 🔄 **IMPLEMENTATION PHASES**

### **Phase 1: Core Template CRUD** (Week 1: Jan 13-19)

**Goals**:
- Implement basic template storage
- Get, list, and search working
- Unit tests for core operations

**Tasks**:
1. Define data structures (4 hours)
2. Implement templates.store (8 hours)
3. Implement templates.retrieve (6 hours)
4. Implement templates.list (10 hours)
5. Unit tests (8 hours)
6. Integration test with biomeOS (4 hours)

**Total**: 40 hours

**Deliverable**: Basic template CRUD operations working

---

### **Phase 2: Community & Versioning** (Week 2: Jan 20-26)

**Goals**:
- Community template ranking
- Version control system
- Template metadata tracking

**Tasks**:
1. Design ranking algorithm (4 hours)
2. Implement templates.community_top (10 hours)
3. Add version control (8 hours)
4. Usage tracking system (6 hours)
5. Success rate calculations (6 hours)
6. Unit tests (4 hours)
7. Integration tests (2 hours)

**Total**: 40 hours

**Deliverable**: Community features and versioning complete

---

### **Phase 3: Audit Trails** (Week 3: Jan 27-Feb 2)

**Goals**:
- Execution audit storage
- Query capabilities
- Learning data format

**Tasks**:
1. Define audit data structures (4 hours)
2. Implement audit.store_execution (10 hours)
3. Add audit query methods (internal) (8 hours)
4. Modification tracking (6 hours)
5. Outcome aggregation (4 hours)
6. Unit tests (4 hours)
7. Integration tests with biomeOS (4 hours)

**Total**: 40 hours

**Deliverable**: Full audit trail system

---

### **Phase 4: Polish & Deploy** (Week 4: Feb 3-9)

**Goals**:
- Performance optimization
- Complete documentation
- Production deployment

**Tasks**:
1. Performance testing (6 hours)
2. Optimization (8 hours)
3. API documentation (4 hours)
4. Integration guide (4 hours)
5. Example code (2 hours)
6. Production deployment (4 hours)
7. Monitoring setup (2 hours)

**Total**: 30 hours

**Deliverable**: Production-ready system

---

## 🧪 **TESTING STRATEGY**

### **Unit Tests**

**Template Storage**:
- ✅ Store template successfully
- ✅ Retrieve template by ID
- ✅ List templates with filters
- ✅ Filter by tags
- ✅ Filter by niche type
- ✅ Sort by different fields
- ✅ Pagination works correctly
- ✅ Community ranking algorithm
- ✅ Version control

**Audit Storage**:
- ✅ Store audit successfully
- ✅ Handle large modification lists
- ✅ Store complex outcomes
- ✅ Query audit by execution_id

### **Integration Tests**

**With biomeOS**:
- ✅ Full workflow: store → retrieve → deploy
- ✅ Template modification and re-save
- ✅ Community template discovery
- ✅ Audit trail capture during execution

### **Performance Tests**:
- ✅ Template storage < 100ms
- ✅ Template retrieval < 50ms
- ✅ List 1000 templates < 200ms
- ✅ Audit storage < 200ms
- ✅ Concurrent access (100 users)

---

## 📊 **SUCCESS CRITERIA**

### **Functional**:
- ✅ All 5 JSON-RPC methods implemented
- ✅ Template CRUD operations working
- ✅ Community ranking functional
- ✅ Audit trails captured correctly
- ✅ Version control working

### **Quality**:
- ✅ All unit tests passing
- ✅ Integration tests passing
- ✅ Performance targets met
- ✅ A-grade quality (90+/100)
- ✅ Zero critical bugs

### **Integration**:
- ✅ biomeOS client compatibility 100%
- ✅ Squirrel can query audit data
- ✅ petalTongue can load templates
- ✅ Full workflow tested end-to-end

---

## 🔒 **SECURITY CONSIDERATIONS**

### **Access Control**:
- Users can only access their own templates (unless community)
- Family isolation enforced
- Community templates read-only (except owner)

### **Validation**:
- Validate graph_data structure
- Sanitize user input
- Limit template size (max 10MB)
- Rate limiting on storage operations

### **Audit**:
- All template access logged
- Modification history preserved
- Immutable audit records

---

## 📚 **DOCUMENTATION**

### **API Reference**:
- JSON-RPC method specifications
- Request/response examples
- Error codes and handling

### **Integration Guide**:
- How to store templates from biomeOS
- How to retrieve and use templates
- Community template guidelines

### **Developer Guide**:
- Data model documentation
- Extension points
- Testing guidelines

---

## 🎯 **RISKS & MITIGATION**

### **Risk 1: Community Ranking Algorithm** (Medium)
**Mitigation**: Start with simple formula, iterate based on usage

### **Risk 2: Performance at Scale** (Medium)
**Mitigation**: Performance testing in week 3, optimization in week 4

### **Risk 3: Version Control Complexity** (Low)
**Mitigation**: MVP versioning first (linear history), optimize later

### **Risk 4: Integration Issues** (Low)
**Mitigation**: Weekly sync with biomeOS, integration tests early

---

## 📅 **TIMELINE**

```
Week 1 (Jan 13-19):  Core CRUD          ████████████ 100%
Week 2 (Jan 20-26):  Community/Version  ████████████ 100%
Week 3 (Jan 27-Feb 2): Audit Trails     ████████████ 100%
Week 4 (Feb 3-9):    Polish/Deploy      ████████████ 100%
```

**Start**: January 13, 2026  
**Complete**: February 2, 2026  
**Deploy**: February 9, 2026

---

## ✅ **APPROVAL**

**Specification**: Approved  
**Timeline**: Approved  
**Resources**: Allocated  
**Status**: Ready to implement

---

**Document Version**: 1.0  
**Last Updated**: January 10, 2026  
**Next Review**: January 20, 2026 (after Phase 1)
