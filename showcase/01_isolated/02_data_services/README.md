# Demo 1.2: Data Services

**Level**: 1 (Isolated)  
**Time**: 8 minutes  
**Complexity**: Beginner  
**Status**: 🆕 New demo

---

## 🎯 WHAT THIS DEMO SHOWS

This demo demonstrates NestGate's REST API for data services:
1. Advanced API operations (POST, GET, PUT, DELETE)
2. Querying and filtering data
3. Metadata management
4. Batch operations
5. Health and status endpoints

**Key Concept**: NestGate as a full-featured data service platform

---

## 🚀 QUICK RUN

```bash
# Make sure NestGate is running
../../scripts/start_data_service.sh

# Run the demo
./demo.sh

# Expected runtime: ~2 minutes
```

---

## 📋 WHAT YOU'LL SEE

### Part 1: CRUD Operations
Complete Create, Read, Update, Delete cycle:

```bash
# Create
POST /api/v1/datasets/users/objects/user1
{"name": "Alice", "role": "admin"}

# Read
GET /api/v1/datasets/users/objects/user1

# Update
PUT /api/v1/datasets/users/objects/user1
{"name": "Alice", "role": "superadmin"}

# Delete
DELETE /api/v1/datasets/users/objects/user1
```

### Part 2: Metadata Operations
Add and query metadata:

```bash
# Store with metadata
PUT /api/v1/datasets/photos/objects/sunset.jpg
Headers: X-Metadata-Tags: nature,sunset,2025
         X-Metadata-Author: demo-user

# Query by metadata
GET /api/v1/datasets/photos/objects?tags=nature
GET /api/v1/datasets/photos/objects?author=demo-user
```

### Part 3: Batch Operations
Efficient bulk operations:

```bash
# Batch upload
POST /api/v1/datasets/logs/batch
[
  {"key": "log1.txt", "data": "..."},
  {"key": "log2.txt", "data": "..."},
  {"key": "log3.txt", "data": "..."}
]

# Batch retrieve
GET /api/v1/datasets/logs/batch?keys=log1.txt,log2.txt,log3.txt
```

### Part 4: Health & Status
Monitor service health:

```bash
# Health check
GET /health
→ {"status": "healthy", "uptime": 3600, "version": "0.1.0"}

# Detailed status
GET /api/v1/status
→ {
    "storage": {"available": true, "backend": "zfs"},
    "api": {"requests": 1234, "errors": 5},
    "metrics": {"cpu": 12.5, "memory": 1024}
  }
```

---

## 🔍 API REFERENCE (Quick)

### Datasets
```bash
GET    /api/v1/datasets              # List all
POST   /api/v1/datasets              # Create new
GET    /api/v1/datasets/{name}       # Get info
DELETE /api/v1/datasets/{name}       # Delete
```

### Objects
```bash
GET    /api/v1/datasets/{ds}/objects           # List
POST   /api/v1/datasets/{ds}/objects           # Create
GET    /api/v1/datasets/{ds}/objects/{key}     # Retrieve
PUT    /api/v1/datasets/{ds}/objects/{key}     # Update
DELETE /api/v1/datasets/{ds}/objects/{key}     # Delete
```

### Batch
```bash
POST   /api/v1/datasets/{ds}/batch             # Batch upload
GET    /api/v1/datasets/{ds}/batch?keys=...    # Batch retrieve
```

### Query
```bash
GET    /api/v1/datasets/{ds}/objects?tags=...  # Filter by tags
GET    /api/v1/datasets/{ds}/objects?author=...# Filter by author
GET    /api/v1/datasets/{ds}/objects?limit=10  # Pagination
```

---

## 💡 REAL-WORLD USE CASES

### Use Case 1: User Management System
```bash
# Store user profiles
curl -X POST ${NESTGATE}/api/v1/datasets/users/objects/alice \
  -d '{"email": "alice@example.com", "role": "admin"}'

# Query by role
curl "${NESTGATE}/api/v1/datasets/users/objects?role=admin"

# Update user
curl -X PUT ${NESTGATE}/api/v1/datasets/users/objects/alice \
  -d '{"email": "alice@example.com", "role": "superadmin"}'
```

### Use Case 2: Log Aggregation
```bash
# Batch upload logs
curl -X POST ${NESTGATE}/api/v1/datasets/logs/batch \
  -d '[
    {"key": "app1.log", "data": "..."},
    {"key": "app2.log", "data": "..."}
  ]'

# Query recent logs
curl "${NESTGATE}/api/v1/datasets/logs/objects?since=2025-12-17"
```

### Use Case 3: Document Storage
```bash
# Store with metadata
curl -X PUT ${NESTGATE}/api/v1/datasets/docs/objects/report.pdf \
  -H "X-Metadata-Author: Jane Doe" \
  -H "X-Metadata-Tags: quarterly,finance,2025" \
  --data-binary @report.pdf

# Find all quarterly reports
curl "${NESTGATE}/api/v1/datasets/docs/objects?tags=quarterly"
```

---

## 🧪 EXPERIMENTS TO TRY

### Experiment 1: Stress Test
```bash
# Upload 100 small files
for i in {1..100}; do
  curl -X PUT "${NESTGATE}/api/v1/datasets/stress/objects/file${i}" \
    -d "Test data ${i}" &
done
wait

# Check how long it took
curl "${NESTGATE}/api/v1/metrics/performance"
```

### Experiment 2: Large File Handling
```bash
# Create 1GB file
dd if=/dev/zero of=/tmp/1gb.bin bs=1M count=1024

# Upload and time it
time curl -X PUT "${NESTGATE}/api/v1/datasets/large/objects/1gb.bin" \
  --data-binary @/tmp/1gb.bin

# Download and verify
time curl "${NESTGATE}/api/v1/datasets/large/objects/1gb.bin" \
  -o /tmp/1gb-downloaded.bin
```

### Experiment 3: Metadata Search
```bash
# Create searchable documents
for i in {1..20}; do
  curl -X PUT "${NESTGATE}/api/v1/datasets/search/objects/doc${i}" \
    -H "X-Metadata-Tags: demo,test,doc${i}" \
    -H "X-Metadata-Date: 2025-12-$(($i % 31 + 1))" \
    -d "Document ${i} content"
done

# Search by date
curl "${NESTGATE}/api/v1/datasets/search/objects?date=2025-12-17"
```

---

## 📊 EXPECTED BEHAVIOR

### HTTP Status Codes
- `200 OK` - Successful operation
- `201 Created` - Resource created
- `204 No Content` - Successful deletion
- `400 Bad Request` - Invalid input
- `404 Not Found` - Resource doesn't exist
- `409 Conflict` - Resource already exists
- `500 Internal Server Error` - Server error

### Response Times (Typical)
- Small object (<1MB): <10ms
- Medium object (1-10MB): <100ms
- Large object (100MB+): ~1s per 100MB
- Batch operations: ~50ms overhead + per-object time
- Metadata queries: <50ms

---

## 🆘 TROUBLESHOOTING

### Error: "Dataset not found"
**Cause**: Trying to access non-existent dataset  
**Fix**: Create dataset first
```bash
curl -X POST ${NESTGATE}/api/v1/datasets \
  -d '{"name": "your-dataset"}'
```

### Error: "Object already exists"
**Cause**: POST to existing object (use PUT instead)  
**Fix**: Use PUT for updates
```bash
curl -X PUT ${NESTGATE}/api/v1/datasets/ds/objects/key \
  -d "updated data"
```

### Error: "Request Entity Too Large"
**Cause**: File exceeds configured limit  
**Fix**: Adjust limit or use chunked upload
```bash
# Configure larger limit
export NESTGATE_MAX_UPLOAD_SIZE=10GB
```

---

## 🔄 CLEANUP

```bash
# Clean up demo data
./cleanup.sh

# Or manually
curl -X DELETE ${NESTGATE}/api/v1/datasets/demo-data-services
```

---

## ⏭️ NEXT DEMO

**Demo 1.3: Capability Discovery** (`../03_capability_discovery/`)
- See how NestGate discovers its own capabilities
- Understand zero-knowledge architecture
- No hardcoded configuration!

---

## 📚 LEARN MORE

- **Full API Docs**: `../../../docs/API_REFERENCE.md`
- **REST Design**: `../../../docs/architecture/API_ARCHITECTURE.md`
- **Performance**: `../../../docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`

---

**Status**: 🆕 New demo  
**Estimated time**: 8 minutes  
**Prerequisites**: Completed Demo 1.1

---

*Demo 1.2 - Data Services*  
*Part of Level 1: Isolated Instance*

