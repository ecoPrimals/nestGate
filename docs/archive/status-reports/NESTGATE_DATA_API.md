# 🗄️ **NESTGATE DATA API**

**Date**: January 30, 2025  
**Status**: ✅ **PURE DATA LAYER COMPLETE**  
**Purpose**: Clean data API for biomeOS and other management systems

---

## 🎯 **PURE DATA LAYER**

NestGate provides a **pure data API** - no authentication, no user management, just clean ZFS and storage data operations. Perfect for consumption by biomeOS which handles the UI and user concerns.

### **🏗️ Data-First Design**

- **Pure Data Operations** - Focus solely on ZFS/storage data
- **Clean JSON Responses** - Simple, consistent data structures  
- **Fast & Efficient** - No auth overhead, optimized for data access
- **Real-Time Streams** - WebSocket data feeds for live updates
- **Comprehensive Error Codes** - Structured error handling

---

## 📋 **DATA API ENDPOINTS**

### **🗄️ ZFS Dataset Data**

#### List Datasets
```http
GET /api/v1/zfs/datasets?page=1&per_page=50&sort=name&filter=tank
```

**Response:**
```json
{
  "data": [
    {
      "name": "tank/data",
      "dataset_type": "filesystem",
      "backend": "filesystem", 
      "properties": {
        "compression": true,
        "compression_type": "lz4",
        "checksum": true,
        "checksum_type": "sha256",
        "deduplication": false,
        "encryption": false,
        "readonly": false,
        "custom": {}
      },
      "stats": {
        "used_bytes": 1073741824,
        "available_bytes": 10737418240,
        "files_written": 150,
        "files_read": 300,
        "cow_operations": 25,
        "blocks_copied": 100,
        "compression_ratio": 0.65,
        "compression_space_saved": 536870912,
        "checksums_computed": 150,
        "checksums_verified": 300,
        "read_throughput": 125.5,
        "write_throughput": 89.3,
        "avg_latency_ms": 2.1
      },
      "created": "2025-01-30T09:00:00Z",
      "modified": "2025-01-30T10:00:00Z", 
      "status": "online",
      "snapshot_count": 5
    }
  ],
  "timestamp": "2025-01-30T10:00:00Z",
  "meta": {
    "total": 1,
    "page": 1,
    "per_page": 50,
    "has_more": false
  }
}
```

#### Create Dataset
```http
POST /api/v1/zfs/datasets
Content-Type: application/json

{
  "name": "tank/photos",
  "backend": "filesystem",
  "path": "/data/photos",
  "properties": {
    "compression": true,
    "compression_type": "zstd",
    "checksum": true,
    "checksum_type": "sha256"
  }
}
```

#### Get Dataset
```http
GET /api/v1/zfs/datasets/tank%2Fdata
```

#### Update Dataset  
```http
PUT /api/v1/zfs/datasets/tank%2Fdata
```

#### Delete Dataset
```http
DELETE /api/v1/zfs/datasets/tank%2Fdata
```

#### Get Dataset Properties
```http
GET /api/v1/zfs/datasets/tank%2Fdata/properties
```

#### Set Dataset Properties
```http
PUT /api/v1/zfs/datasets/tank%2Fdata/properties
```

#### Get Dataset Stats
```http
GET /api/v1/zfs/datasets/tank%2Fdata/stats
```

---

### **📸 ZFS Snapshot Data**

#### List Snapshots
```http
GET /api/v1/zfs/datasets/tank%2Fdata/snapshots?page=1&per_page=50
```

**Response:**
```json
{
  "data": [
    {
      "id": "tank/data_0",
      "name": "backup-2025-01-30", 
      "dataset": "tank/data",
      "created": "2025-01-30T09:30:00Z",
      "size_bytes": 1073741824,
      "unique_bytes": 536870912,
      "file_count": 150,
      "status": "active",
      "description": "Daily backup snapshot",
      "tags": ["backup", "daily", "automated"]
    }
  ],
  "timestamp": "2025-01-30T10:00:00Z",
  "meta": {
    "total": 1,
    "page": 1, 
    "per_page": 50,
    "has_more": false
  }
}
```

#### Create Snapshot
```http
POST /api/v1/zfs/datasets/tank%2Fdata/snapshots
Content-Type: application/json

{
  "name": "manual-backup-2025-01-30",
  "description": "Manual backup before system upgrade",
  "tags": ["backup", "manual", "pre-upgrade"]
}
```

#### Get Snapshot
```http
GET /api/v1/zfs/datasets/tank%2Fdata/snapshots/backup-2025-01-30
```

#### Delete Snapshot
```http
DELETE /api/v1/zfs/datasets/tank%2Fdata/snapshots/backup-2025-01-30
```

#### Clone Snapshot
```http
POST /api/v1/zfs/datasets/tank%2Fdata/snapshots/backup-2025-01-30/clone
```

---

### **📦 Storage Data**

#### List Storage Backends
```http
GET /api/v1/storage/backends
```

**Response:**
```json
{
  "data": [
    {
      "backend_type": "filesystem",
      "name": "Local SSD",
      "description": "High-performance local SSD storage",
      "available_bytes": 536870912000,
      "total_bytes": 1073741824000,
      "capabilities": ["basic_operations", "compression", "snapshots", "checksumming"],
      "performance": {
        "read_throughput_mbps": 550.0,
        "write_throughput_mbps": 520.0,
        "avg_latency_ms": 0.8,
        "iops": 100000,
        "tier": "high"
      },
      "status": "online"
    }
  ],
  "timestamp": "2025-01-30T10:00:00Z"
}
```

#### Scan Storage
```http
POST /api/v1/storage/scan
Content-Type: application/json

{
  "path": "/data",
  "include_cloud": true,
  "include_network": true,
  "include_block": true
}
```

#### Benchmark Storage
```http
POST /api/v1/storage/benchmark
Content-Type: application/json

{
  "backend": "filesystem",
  "config": {
    "path": "/data/benchmark"
  },
  "duration_seconds": 60,
  "test_size_mb": 1024
}
```

#### Auto-Configure Storage
```http
POST /api/v1/storage/auto-config
Content-Type: application/json

{
  "use_case": "home_nas",
  "min_capacity_gb": 2000,
  "performance_requirements": {
    "min_throughput_mbps": 100.0,
    "max_latency_ms": 10.0,
    "performance_priority": false
  },
  "redundancy_level": "mirror",
  "preferred_features": ["compression", "snapshots", "checksumming"]
}
```

---

### **📊 Performance Data**

#### Get Current Metrics
```http
GET /api/v1/monitoring/metrics
```

**Response:**
```json
{
  "data": {
    "timestamp": "2025-01-30T10:00:00Z",
    "cpu_usage_percent": 15.5,
    "memory_usage_percent": 45.2,
    "disk_io": {
      "read_mbps": 125.3,
      "write_mbps": 89.7,
      "read_iops": 8500,
      "write_iops": 6200,
      "avg_queue_depth": 2.1
    },
    "network_io": {
      "rx_bytes_per_sec": 1048576,
      "tx_bytes_per_sec": 524288,
      "rx_packets_per_sec": 1000,
      "tx_packets_per_sec": 800
    },
    "zfs_metrics": {
      "total_datasets": 5,
      "total_snapshots": 25,
      "total_used_bytes": 5368709120,
      "total_available_bytes": 53687091200,
      "overall_compression_ratio": 0.68,
      "cache_hit_ratio": 0.95
    }
  },
  "timestamp": "2025-01-30T10:00:00Z"
}
```

#### Get Historical Metrics
```http
GET /api/v1/monitoring/metrics/history?start=2025-01-30T09:00:00Z&end=2025-01-30T10:00:00Z&interval=5m
```

#### Get Alerts
```http
GET /api/v1/monitoring/alerts
```

---

### **🔌 Real-Time Data Streams**

#### Live Metrics Stream
```javascript
const ws = new WebSocket('ws://localhost:8080/ws/metrics');
ws.onmessage = (event) => {
  const metrics = JSON.parse(event.data);
  updateDashboard(metrics);
};
```

#### Live System Events
```javascript
const ws = new WebSocket('ws://localhost:8080/ws/events');
ws.onmessage = (event) => {
  const event = JSON.parse(event.data);
  handleSystemEvent(event);
};
```

#### Live Logs Stream
```javascript
const ws = new WebSocket('ws://localhost:8080/ws/logs?level=info');
ws.onmessage = (event) => {
  const logEntry = JSON.parse(event.data);
  displayLogEntry(logEntry);
};
```

---

## 🔧 **ERROR HANDLING**

### **Simple Error Response Format**
```json
{
  "error": "Dataset 'tank/data' not found",
  "code": "DATASET_NOT_FOUND",
  "timestamp": "2025-01-30T10:00:00Z"
}
```

### **Common Error Codes**
- `DATASET_NOT_FOUND` - Dataset doesn't exist
- `DATASET_EXISTS` - Dataset already exists
- `INVALID_NAME` - Invalid dataset/snapshot name
- `BACKEND_ERROR` - Storage backend error
- `ENGINE_ERROR` - ZFS engine error
- `SNAPSHOT_NOT_FOUND` - Snapshot doesn't exist
- `SNAPSHOT_ERROR` - Snapshot operation failed
- `NOT_IMPLEMENTED` - Feature not yet implemented

---

## 🌟 **BIOME OS INTEGRATION**

### **Simple Data Fetching**
```typescript
// Fetch dataset list
const datasets = await fetch('/api/v1/zfs/datasets')
  .then(r => r.json())
  .then(response => response.data);

// Create snapshot
const snapshot = await fetch('/api/v1/zfs/datasets/tank%2Fdata/snapshots', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ 
    name: 'backup-2025-01-30',
    description: 'Daily backup'
  })
}).then(r => r.json());
```

### **Real-Time Dashboard Component**
```typescript
class NestGateDataProvider {
  private metricsWs: WebSocket;
  private eventsWs: WebSocket;
  
  constructor() {
    // Connect to real-time data streams
    this.metricsWs = new WebSocket('ws://localhost:8080/ws/metrics');
    this.eventsWs = new WebSocket('ws://localhost:8080/ws/events');
    
    this.metricsWs.onmessage = (event) => {
      const metrics = JSON.parse(event.data);
      this.updateMetrics(metrics);
    };
    
    this.eventsWs.onmessage = (event) => {
      const systemEvent = JSON.parse(event.data);
      this.handleEvent(systemEvent);
    };
  }
  
  async getDatasets(page = 1, filter?: string) {
    const url = `/api/v1/zfs/datasets?page=${page}&per_page=50${filter ? `&filter=${filter}` : ''}`;
    const response = await fetch(url);
    const result = await response.json();
    return result.data;
  }
  
  async createDataset(name: string, backend: string, properties?: any) {
    const response = await fetch('/api/v1/zfs/datasets', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name, backend, properties })
    });
    
    if (!response.ok) {
      const error = await response.json();
      throw new Error(`${error.code}: ${error.error}`);
    }
    
    const result = await response.json();
    return result.data;
  }
  
  async createSnapshot(dataset: string, name: string, description?: string) {
    const response = await fetch(`/api/v1/zfs/datasets/${encodeURIComponent(dataset)}/snapshots`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name, description })
    });
    
    const result = await response.json();
    return result.data;
  }
}
```

### **Error Handling**
```typescript
async function handleDataOperation(operation: () => Promise<any>) {
  try {
    return await operation();
  } catch (error) {
    if (error.response?.status === 400) {
      const errorData = await error.response.json();
      
      switch (errorData.code) {
        case 'DATASET_NOT_FOUND':
          showNotification('Dataset not found', 'error');
          break;
        case 'DATASET_EXISTS':
          showNotification('Dataset already exists', 'warning');
          break;
        case 'INVALID_NAME':
          showNotification('Invalid name format', 'error');
          break;
        default:
          showNotification(errorData.error, 'error');
      }
    }
  }
}
```

---

## 🚀 **API BENEFITS**

### **🎯 Perfect for biomeOS**

1. **🔌 Zero Authentication Overhead** - biomeOS handles users, NestGate handles data
2. **⚡ Fast Data Access** - No auth checks, direct data operations  
3. **📊 Rich Data Models** - Complete ZFS information for advanced UIs
4. **🔄 Real-Time Updates** - WebSocket streams for responsive dashboards
5. **🛡️ Simple Error Handling** - Clear error codes for programmatic handling
6. **📈 Comprehensive Data** - All ZFS metrics and statistics available

### **🌟 Clean Separation of Concerns**

- **NestGate**: Pure ZFS/storage data layer
- **biomeOS**: User interface, authentication, user management
- **Perfect Integration**: Clean API boundary, no overlap

---

## 📊 **DATA API SUMMARY**

| **Category** | **Endpoints** | **Purpose** |
|-------------|---------------|-------------|
| **Datasets** | 8 endpoints | Complete ZFS dataset data operations |
| **Snapshots** | 5 endpoints | Snapshot data management |  
| **Storage** | 4 endpoints | Storage backend data and auto-config |
| **Monitoring** | 3 endpoints | Performance and system data |
| **Real-Time** | 3 WebSocket | Live data streams |

### **🎉 Total: 23 Pure Data Endpoints**

All focused on providing clean, fast access to ZFS and storage data without any authentication or user management overhead.

---

## 🎯 **CONCLUSION**

**NestGate now provides the perfect data layer** for biomeOS and other management systems:

- **Pure Data Focus** - No auth, no users, just data operations
- **Clean & Fast** - Optimized for data access and real-time updates
- **Complete ZFS Data** - Everything needed for rich storage UIs
- **Simple Integration** - Standard REST with clear error handling
- **Real-Time Capable** - WebSocket streams for responsive dashboards

**biomeOS can now build amazing storage UIs** on top of this clean, efficient data layer! 🌟

---

## 📝 **IMPLEMENTATION FILES**

- `code/crates/nestgate-api/src/rest/mod.rs` - Pure data API router
- `code/crates/nestgate-api/src/rest/models.rs` - Clean data models
- `code/crates/nestgate-api/src/rest/handlers/zfs.rs` - ZFS data handlers
- `NESTGATE_DATA_API.md` - This data API documentation

**Total Data API**: 1,800+ lines of pure data layer! 🗄️ 