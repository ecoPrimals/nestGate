# 🔄 ZFS Replication Demo

**Level**: 3 - Federation  
**Demo**: 3.2 - ZFS Replication  
**Duration**: 10 minutes  
**Prerequisites**: Multiple NestGate instances, ZFS support  

---

## 🎯 What This Demonstrates

This demo shows NestGate's distributed replication capabilities:

1. **Incremental Snapshots** - ZFS snapshot-based replication
2. **Cross-Instance Sync** - Replicate data between NestGate nodes
3. **Bandwidth Optimization** - Only send changed blocks
4. **Failover Ready** - Maintain replicas for disaster recovery

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│              ZFS REPLICATION FLOW                       │
│                                                         │
│  NestGate Primary (Instance A)                         │
│  ┌──────────────────┐                                  │
│  │  main-pool/data  │                                  │
│  │  ├─ snapshot-1   │  (baseline)                      │
│  │  ├─ snapshot-2   │  (increment)                     │
│  │  └─ snapshot-3   │  (increment)                     │
│  └────────┬─────────┘                                  │
│           │                                            │
│           │ Replication Stream                         │
│           │ (only deltas)                              │
│           ↓                                            │
│  NestGate Replica (Instance B)                         │
│  ┌──────────────────┐                                  │
│  │  backup-pool/data│                                  │
│  │  ├─ snapshot-1   │  (received)                      │
│  │  ├─ snapshot-2   │  (received)                      │
│  │  └─ snapshot-3   │  (received)                      │
│  └──────────────────┘                                  │
│                                                         │
└─────────────────────────────────────────────────────────┘

Benefits:
  • Incremental: Only send changes
  • Compressed: Bandwidth efficient
  • Consistent: Point-in-time snapshots
  • Automated: Scheduled replication
```

---

## 🚀 Running the Demo

### Quick Start

```bash
# From showcase root
./03_federation/02_replication/demo.sh
```

### Manual Steps

```bash
# 1. Create source dataset
curl -X POST http://127.0.0.1:8080/api/v1/storage/datasets \
  -H "Content-Type: application/json" \
  -d '{"name": "main-pool/replicated-data", "pool": "main-pool"}'

# 2. Create initial snapshot
curl -X POST http://127.0.0.1:8080/api/v1/zfs/datasets/main-pool/replicated-data/snapshots \
  -H "Content-Type: application/json" \
  -d '{"name": "snap-001"}'

# 3. Replicate to backup
curl -X POST http://127.0.0.1:8080/api/v1/replication/start \
  -H "Content-Type: application/json" \
  -d '{
    "source": "main-pool/replicated-data@snap-001",
    "destination": "backup-pool/replicated-data",
    "target_host": "nestgate-backup:8080"
  }'

# 4. Monitor replication status
curl http://127.0.0.1:8080/api/v1/replication/status | jq '.'
```

---

## 📊 Expected Output

### Replication Status

```json
{
  "status": "active",
  "jobs": [
    {
      "id": "repl-job-001",
      "source": "main-pool/replicated-data",
      "destination": "backup-pool/replicated-data@nestgate-backup",
      "state": "syncing",
      "progress": {
        "snapshots_sent": 3,
        "bytes_sent": 2147483648,
        "bytes_total": 5368709120,
        "percentage": 40,
        "estimated_time_remaining": "5m 30s"
      },
      "last_snapshot": "snap-003",
      "next_sync": "2025-12-18T01:00:00Z"
    }
  ]
}
```

### Replication Performance

```
Replication Performance:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Initial Sync:      2.5 GB → 2.5 GB  (100%)
Incremental #1:    2.5 GB → 150 MB  (6%)   94% savings!
Incremental #2:    2.5 GB → 89 MB   (3.5%) 96.5% savings!
Incremental #3:    2.5 GB → 45 MB   (1.8%) 98.2% savings!

Compression: enabled (1.5x)
Deduplication: enabled (1.2x)
Network: 1 Gbps
Bandwidth used: ~15% (incremental)
```

---

## 🔍 What's Happening

### 1. Snapshot-Based Replication

```
Time T0: Create snapshot-1 (baseline)
  ├─ Full dataset: 2.5 GB
  └─ Send to replica

Time T1: Create snapshot-2 (after changes)
  ├─ Identify blocks changed since snapshot-1
  ├─ Only send delta: 150 MB
  └─ Apply to replica

Time T2: Create snapshot-3 (after more changes)
  ├─ Identify blocks changed since snapshot-2
  ├─ Only send delta: 89 MB
  └─ Apply to replica
```

### 2. ZFS Send/Receive

```bash
# Behind the scenes, NestGate uses ZFS send/receive
zfs send main-pool/data@snap-001 | \
  ssh nestgate-backup \
  zfs receive backup-pool/data

# Incremental sends
zfs send -i @snap-001 main-pool/data@snap-002 | \
  ssh nestgate-backup \
  zfs receive backup-pool/data
```

### 3. Bandwidth Optimization

- **Compression**: Reduce data size before sending
- **Deduplication**: Don't send duplicate blocks
- **Incremental**: Only send changes
- **Throttling**: Limit bandwidth usage

---

## 🎓 Learning Points

### 1. Why Snapshot-Based Replication?

**Traditional Replication**:
- File-level copying
- Inconsistent (files change during copy)
- No incremental support
- Slow and bandwidth-heavy

**ZFS Snapshot Replication**:
- Block-level copying
- Consistent (point-in-time)
- Incremental by design
- Fast and efficient

### 2. Replication Modes

**Full Replication**:
```bash
# Send entire dataset
zfs send pool/dataset@snapshot
```

**Incremental Replication**:
```bash
# Send only changes since last snapshot
zfs send -i @old pool/dataset@new
```

**Resume Support**:
```bash
# Resume interrupted replication
zfs send -t token pool/dataset@snapshot
```

### 3. Use Cases

**Disaster Recovery**:
- Maintain offsite replica
- Quick failover capability
- Data protection

**Data Migration**:
- Move datasets between systems
- Minimal downtime
- Preserve all properties

**Backup Strategy**:
- Regular scheduled snapshots
- Incremental backups
- Space-efficient

---

## 🔗 Integration with Ecosystem

### Songbird Orchestration

```rust
// Songbird coordinates replication across NestGate fleet
let replication_job = ReplicationJob {
    source: "nestgate-a:main-pool/data@daily-001",
    destinations: vec![
        "nestgate-b:backup-pool/data",
        "nestgate-c:archive-pool/data",
    ],
    schedule: "0 2 * * *",  // 2 AM daily
    retention: Duration::days(30),
};

orchestrator.schedule_replication(replication_job).await?;
```

### ToadStool Integration

```rust
// ToadStool AI training with automatic replication
let training_dataset = nestgate.create_dataset("ml-pool/training-001")?;

// Songbird automatically sets up replication
orchestrator.enable_replication(
    training_dataset,
    vec!["nestgate-backup-1", "nestgate-backup-2"]
).await?;

// Train model
toadstool.train(training_dataset).await?;

// Checkpoints automatically replicated
```

---

## 📚 API Reference

### Create Replication Job

```bash
POST /api/v1/replication/jobs
{
  "source": "pool/dataset@snapshot",
  "destination": "backup-pool/dataset",
  "target_host": "nestgate-backup:8080",
  "schedule": "0 * * * *",  // hourly
  "incremental": true,
  "compression": true,
  "bandwidth_limit_mbps": 100
}
```

### List Replication Jobs

```bash
GET /api/v1/replication/jobs
```

### Get Job Status

```bash
GET /api/v1/replication/jobs/{job_id}/status
```

### Pause/Resume Job

```bash
POST /api/v1/replication/jobs/{job_id}/pause
POST /api/v1/replication/jobs/{job_id}/resume
```

---

## 🎯 Success Criteria

✅ **Replication Working**
- Initial sync completes
- Incremental syncs succeed
- Data verified on replica

✅ **Bandwidth Optimized**
- Incremental sync < 10% of full
- Compression enabled
- No duplicate data sent

✅ **Automated Scheduling**
- Jobs run on schedule
- Failures handled gracefully
- Monitoring alerts work

---

## 🐛 Troubleshooting

### Replication Stalled

```bash
# Check job status
curl http://127.0.0.1:8080/api/v1/replication/jobs/{id}/status

# Check logs
tail -f ~/.nestgate/logs/replication.log

# Verify network connectivity
ping nestgate-backup
```

### Snapshot Not Found

```bash
# List available snapshots
curl http://127.0.0.1:8080/api/v1/zfs/datasets/{pool}/{dataset}/snapshots

# Create missing snapshot
curl -X POST http://127.0.0.1:8080/api/v1/zfs/datasets/{pool}/{dataset}/snapshots \
  -d '{"name": "snap-new"}'
```

### Bandwidth Issues

```bash
# Check replication bandwidth
curl http://127.0.0.1:8080/api/v1/replication/metrics

# Adjust bandwidth limit
curl -X PATCH http://127.0.0.1:8080/api/v1/replication/jobs/{id} \
  -d '{"bandwidth_limit_mbps": 50}'
```

---

## 📖 Related Documentation

- `showcase/03_federation/01_mesh_formation` - Federation basics
- ZFS replication guide in `docs/zfs_replication.md`
- Disaster recovery playbook in `docs/disaster_recovery.md`

---

## 🎉 What You Learned

1. ✅ ZFS replication is snapshot-based and incremental
2. ✅ Only changed blocks are sent (massive bandwidth savings)
3. ✅ Replication is consistent (point-in-time snapshots)
4. ✅ NestGate automates complex ZFS send/receive
5. ✅ Songbird can orchestrate replication across fleet

---

**Next Demo**: `03_federation/03_load_balancing` - Distribute load across NestGate instances!

