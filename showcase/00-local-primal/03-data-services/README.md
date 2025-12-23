# 🔌 Data Services - REST API & Metrics

**Time**: 10 minutes  
**Complexity**: Beginner-Intermediate  
**Prerequisites**: Completed 02-zfs-magic

---

## 🎯 WHAT YOU'LL LEARN

- ✅ Full REST API for storage operations
- ✅ Real-time metrics collection
- ✅ Health monitoring endpoints
- ✅ Production-ready data services

**After this demo**: You'll understand NestGate's API

---

## 🚀 QUICK START

```bash
./demo-rest-api.sh    # Full CRUD operations
./demo-metrics.sh     # Real-time metrics
./demo-health.sh      # Health monitoring
```

---

## 📋 DEMOS

### Demo 1: REST API (4 min)
**What**: Full CRUD operations via HTTP

```bash
./demo-rest-api.sh
```

**You'll see**:
- POST /api/v1/storage - Create dataset
- GET /api/v1/storage - List all datasets
- PUT /api/v1/storage - Update dataset
- DELETE /api/v1/storage - Delete dataset
- Response times: <5ms

**Real-world usage**:
```bash
# Store data
curl -X POST http://localhost:8080/api/v1/storage/store \
  -H "Content-Type: application/octet-stream" \
  -d @myfile.txt

# Retrieve data
curl http://localhost:8080/api/v1/storage/retrieve/dataset/myfile.txt
```

---

### Demo 2: Metrics (3 min)
**What**: Real-time system metrics

```bash
./demo-metrics.sh
```

**You'll see**:
```json
{
  "storage": {
    "total_gb": 500,
    "used_gb": 120,
    "available_gb": 380,
    "usage_percent": 24
  },
  "performance": {
    "read_ops_sec": 1234,
    "write_ops_sec": 567,
    "read_mb_sec": 156,
    "write_mb_sec": 89
  },
  "health": {
    "status": "healthy",
    "pools": 2,
    "datasets": 15
  }
}
```

---

### Demo 3: Health Monitoring (3 min)
**What**: System health checks

```bash
./demo-health.sh
```

**You'll see**:
- Pool health status
- Dataset health
- Disk health (SMART data)
- Replication health
- Overall system status

---

## 💡 API DESIGN PRINCIPLES

### RESTful & Standard
```
GET    /api/v1/resource     # List
POST   /api/v1/resource     # Create
GET    /api/v1/resource/:id # Read
PUT    /api/v1/resource/:id # Update
DELETE /api/v1/resource/:id # Delete
```

### JSON Everywhere
- Request: JSON
- Response: JSON
- Errors: JSON with codes
- Metrics: JSON time-series

### Idempotent & Safe
- GET: Safe (no side effects)
- PUT/DELETE: Idempotent (same result)
- POST: Not idempotent (creates new)

---

## 🔧 INTEGRATION EXAMPLES

### Python
```python
import requests

# Store data
response = requests.post(
    'http://localhost:8080/api/v1/storage/store',
    data=open('file.txt', 'rb')
)

# Get metrics
metrics = requests.get(
    'http://localhost:8080/api/v1/metrics'
).json()

print(f"Storage used: {metrics['storage']['usage_percent']}%")
```

### JavaScript
```javascript
// Store data
const response = await fetch(
    'http://localhost:8080/api/v1/storage/store',
    {
        method: 'POST',
        body: fileData
    }
);

// Get health
const health = await fetch(
    'http://localhost:8080/api/v1/health'
).then(r => r.json());

console.log(`Status: ${health.status}`);
```

### curl (CLI)
```bash
# Health check
curl http://localhost:8080/api/v1/health

# Store file
curl -X POST http://localhost:8080/api/v1/storage/store \
  -d @file.txt

# Get metrics
curl http://localhost:8080/api/v1/metrics | jq .
```

---

## 📊 PERFORMANCE

**API Response Times** (local network):
```
Health check:   1-2ms
Storage list:   2-3ms
Store file:     3-5ms (small files)
Retrieve file:  2-4ms (cached)
Metrics:        1-2ms
```

**Throughput** (concurrent requests):
```
100 concurrent: 500 req/s
1000 concurrent: 800 req/s
Network limited: ~1Gbps
```

---

## 🏆 SUCCESS CRITERIA

After this demo, you should:
- [ ] Understand the REST API
- [ ] Know how to store/retrieve data
- [ ] Monitor system metrics
- [ ] Check system health
- [ ] Integrate with your applications

**All done?** → Proceed to Level 4: `../04-self-awareness/`

---

## 📚 NEXT STEPS

**Level 4**: `../04-self-awareness/` - Zero-knowledge discovery (10 min)  
**Level 5**: `../05-performance/` - Performance benchmarks (15 min)

---

🔌 **Production-ready API, ready to use!** 🔌

