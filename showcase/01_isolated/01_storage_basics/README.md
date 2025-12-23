# Demo 1.1: Storage Basics

**Level**: 1 (Isolated)  
**Time**: 5 minutes  
**Complexity**: Beginner  
**Status**: ✅ Ready (Migrated from 01_zfs_basics)

---

## 🎯 WHAT THIS DEMO SHOWS

This demo demonstrates NestGate's fundamental storage capabilities:
1. Creating datasets
2. Storing data
3. Retrieving data
4. Listing datasets
5. Basic data operations

**Key Concept**: NestGate as a sovereign data storage platform

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

### Step 1: Create Dataset
```bash
curl -X POST http://localhost:8080/api/v1/datasets \
  -H "Content-Type: application/json" \
  -d '{"name": "demo-data", "description": "Demo dataset"}'
```

**Output**:
```json
{
  "status": "success",
  "dataset_id": "demo-data",
  "created_at": "2025-12-17T20:30:00Z"
}
```

### Step 2: Store Data
```bash
curl -X PUT http://localhost:8080/api/v1/datasets/demo-data/objects/hello.txt \
  -d "Hello from NestGate!"
```

**Output**:
```json
{
  "status": "success",
  "stored": true,
  "size": 19
}
```

### Step 3: Retrieve Data
```bash
curl http://localhost:8080/api/v1/datasets/demo-data/objects/hello.txt
```

**Output**:
```
Hello from NestGate!
```

### Step 4: List Datasets
```bash
curl http://localhost:8080/api/v1/datasets
```

**Output**:
```json
{
  "datasets": [
    {
      "name": "demo-data",
      "size": "19 bytes",
      "objects": 1,
      "created": "2025-12-17T20:30:00Z"
    }
  ]
}
```

---

## 🔍 WHAT'S HAPPENING BEHIND THE SCENES

1. **NestGate receives API request** via REST endpoint
2. **Validates request** using capability-based system
3. **Routes to storage backend** (ZFS in this case)
4. **Stores data** with metadata tracking
5. **Returns success** with operation details

**Architecture**:
```
curl → NestGate API → Router → Storage Backend (ZFS) → Disk
                ↓
            Response ← Metadata ← Operation Result
```

---

## 💡 KEY CONCEPTS

### 1. Datasets
**What**: Logical groupings of data  
**Why**: Organization and access control  
**How**: Create via API, managed by NestGate

### 2. Objects
**What**: Individual data items (files, blobs)  
**Why**: Granular data management  
**How**: Store/retrieve with unique keys

### 3. REST API
**What**: HTTP-based interface  
**Why**: Universal, language-agnostic access  
**How**: Standard GET/POST/PUT/DELETE operations

---

## 🧪 TRY IT YOURSELF

### Experiment 1: Store Different Data Types
```bash
# Text
curl -X PUT http://localhost:8080/api/v1/datasets/demo-data/objects/text.txt \
  -d "Some text"

# JSON
curl -X PUT http://localhost:8080/api/v1/datasets/demo-data/objects/data.json \
  -H "Content-Type: application/json" \
  -d '{"key": "value"}'

# Binary (image, etc)
curl -X PUT http://localhost:8080/api/v1/datasets/demo-data/objects/image.png \
  --data-binary @/path/to/image.png
```

### Experiment 2: Query by Metadata
```bash
# Add metadata
curl -X PUT http://localhost:8080/api/v1/datasets/demo-data/objects/tagged.txt \
  -d "Data with tags" \
  -H "X-Metadata-Tags: demo,test,example"

# Query by tag
curl http://localhost:8080/api/v1/datasets/demo-data/objects?tags=demo
```

### Experiment 3: Large File
```bash
# Generate 100MB test file
dd if=/dev/zero of=/tmp/largefile bs=1M count=100

# Upload it
curl -X PUT http://localhost:8080/api/v1/datasets/demo-data/objects/largefile \
  --data-binary @/tmp/largefile

# Check performance metrics
curl http://localhost:8080/api/v1/metrics/storage
```

---

## 🆘 TROUBLESHOOTING

### Error: "Connection refused"
**Cause**: NestGate not running  
**Fix**:
```bash
../../scripts/start_data_service.sh
# Wait 5 seconds, then retry
```

### Error: "Dataset already exists"
**Cause**: Ran demo before, data still present  
**Fix**:
```bash
# Clean up previous run
curl -X DELETE http://localhost:8080/api/v1/datasets/demo-data
# Or use cleanup script
./cleanup.sh
```

### Error: "Permission denied"
**Cause**: Insufficient filesystem permissions  
**Fix**:
```bash
# Check NestGate has write permissions to data directory
ls -la ~/.nestgate/data
# Or configure alternative location
export NESTGATE_DATA_DIR=/tmp/nestgate-data
```

---

## 📊 EXPECTED PERFORMANCE

**On typical hardware** (modern SSD):
- Dataset creation: <10ms
- Store 1KB: <5ms
- Store 1MB: <50ms
- Store 100MB: <500ms
- Retrieve 1KB: <2ms
- List datasets: <10ms

**Your system may vary** based on:
- Storage backend (SSD vs HDD)
- Network latency (local vs remote)
- System load
- ZFS configuration

---

## 🔄 CLEANUP

```bash
# Remove demo data
./cleanup.sh

# Or manually:
curl -X DELETE http://localhost:8080/api/v1/datasets/demo-data
```

---

## ⏭️ NEXT DEMO

**Demo 1.2: Data Services** (`../02_data_services/`)
- Learn advanced API operations
- See querying and filtering
- Understand batch operations

---

## 📚 LEARN MORE

- **API Documentation**: `../../../docs/API_REFERENCE.md`
- **Storage Architecture**: `../../../docs/architecture/UNIVERSAL_STORAGE_DESIGN.md`
- **ZFS Details**: `../../../docs/current/STORAGE_TRAITS_INVENTORY.md`

---

**Status**: ✅ Ready to run  
**Estimated time**: 5 minutes  
**Prerequisites**: NestGate running

---

*Demo 1.1 - Storage Basics*  
*Part of Level 1: Isolated Instance*

