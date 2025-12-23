# Demo 2.2: Songbird Data Service Orchestration

**Level**: 2 (Ecosystem Integration)  
**Time**: 25 minutes  
**Complexity**: Medium  
**Status**: 🚧 Building (Week 2)

---

## 🎯 **WHAT THIS DEMO SHOWS**

This demo demonstrates NestGate integration with Songbird for workflow orchestration:

1. **Service Discovery** - Songbird finds NestGate automatically
2. **Workflow Orchestration** - Multi-step automated operations
3. **Data Pipelines** - Snapshot → Backup → Verify workflows
4. **Status Monitoring** - Real-time workflow tracking
5. **Failure Handling** - Automatic retry and recovery

**Key Concept**: Orchestrated data operations with zero manual intervention

---

## 🚀 **QUICK RUN**

```bash
# Make sure both services are running
../../scripts/start_ecosystem.sh --services nestgate,songbird

# Run the demo
./demo.sh

# Expected runtime: ~5 minutes
```

---

## 📋 **WHAT YOU'LL SEE**

### Part 1: Service Discovery
```bash
# Songbird discovers NestGate
songbird discover --capability storage

→ Found: NestGate Data Service
  Endpoint: discovered://nestgate.local:8080
  Capabilities: [storage, snapshots, replication, backup]
  Health: Healthy
  Discovery Time: 52ms (O(1))
```

### Part 2: Simple Workflow
```bash
# Define a backup workflow
cat > backup-workflow.yaml <<EOF
apiVersion: songbird/v1
kind: Workflow
metadata:
  name: daily-backup

steps:
  - name: create-snapshot
    service: nestgate
    operation: snapshot
    params:
      dataset: /production/data
      
  - name: backup-to-remote
    service: nestgate
    operation: backup
    params:
      source: "\${steps.create-snapshot.snapshot_id}"
      destination: s3://backup-bucket/daily/
      
  - name: verify-backup
    service: nestgate
    operation: verify
    params:
      backup_id: "\${steps.backup-to-remote.backup_id}"
EOF

# Run workflow
songbird workflow run backup-workflow.yaml

→ Workflow started: wf_k8x3n9p2
  Step 1/3: Creating snapshot... ✓ (2.3s)
  Step 2/3: Backing up to remote... ✓ (45.7s)
  Step 3/3: Verifying backup... ✓ (8.2s)
  
  Status: SUCCESS
  Duration: 56.2s
  Data backed up: 2.4 GB
```

### Part 3: Data Pipeline
```bash
# Complex data processing pipeline
songbird workflow run data-pipeline.yaml

→ Pipeline: ML Training Data Preparation
  
  Stage 1: Data Ingestion
    ✓ Fetch from NestGate: /raw/dataset-2024-12 (3.2 GB)
    ✓ Validate checksums
    ✓ Create snapshot: snap_abc123
    
  Stage 2: Preprocessing
    ✓ Clean data (removed 234 invalid records)
    ✓ Normalize values
    ✓ Store intermediate: /processed/dataset-2024-12
    
  Stage 3: Feature Engineering
    ✓ Extract features (87 dimensions)
    ✓ Split train/test (80/20)
    ✓ Store final: /ml-ready/dataset-2024-12
    
  Stage 4: Backup & Archive
    ✓ Create final snapshot
    ✓ Backup to S3
    ✓ Update metadata
    
  Pipeline complete: 8m 23s
  Data processed: 3.2 GB → 2.8 GB
  Ready for training: Yes
```

### Part 4: Scheduled Operations
```bash
# Schedule recurring workflow
songbird schedule create \
  --workflow backup-workflow.yaml \
  --cron "0 2 * * *" \
  --name nightly-backup

→ Schedule created: sch_7m4k2x9p
  Workflow: daily-backup
  Schedule: Daily at 2:00 AM
  Next run: 2025-12-18 02:00:00
  Status: Active
```

### Part 5: Failure Recovery
```bash
# Workflow with automatic retry
songbird workflow run resilient-backup.yaml

→ Workflow: Resilient Backup
  
  Step 1: Create snapshot... ✓
  Step 2: Backup to primary... ✗ FAILED
    Error: Primary backup service unavailable
    Retry 1/3... ✗ FAILED
    Retry 2/3... ✗ FAILED
    Fallback to secondary... ✓ SUCCESS
    
  Step 3: Verify backup... ✓
  
  Status: SUCCESS (with fallback)
  Duration: 2m 34s
  Note: Used secondary backup location
```

---

## 💡 **WHY SONGBIRD + NESTGATE**

### The Problem: Manual Data Operations

**Traditional Approach**:
```bash
# Manual, error-prone process
ssh server1
sudo zfs snapshot tank/data@backup-$(date +%Y%m%d)
sudo zfs send tank/data@backup-20251217 | ssh backup-server zfs receive backup/data@20251217
# Wait... did it work?
# Check manually...
ssh backup-server zfs list
# Hope nothing failed...
```

**Problems**:
- Manual intervention required
- Error-prone (typos, missed steps)
- No automatic recovery
- Hard to schedule
- No monitoring
- Can't scale

---

### The Solution: Orchestrated Workflows

**Songbird + NestGate**:
```yaml
# Declarative, automated, reliable
apiVersion: songbird/v1
kind: Workflow
metadata:
  name: backup

steps:
  - snapshot
  - backup
  - verify

# Songbird handles:
# - Service discovery (finds NestGate)
# - Execution (runs all steps)
# - Monitoring (tracks progress)
# - Retry logic (handles failures)
# - Notifications (alerts on issues)
```

**Benefits**:
- ✅ Fully automated
- ✅ Error handling built-in
- ✅ Automatic retries
- ✅ Easy to schedule
- ✅ Real-time monitoring
- ✅ Scales effortlessly

---

## 🏗️ **ARCHITECTURE**

### Orchestration Flow

```
┌─────────────────────────────────────────────────────┐
│         Songbird Orchestration Architecture         │
├─────────────────────────────────────────────────────┤
│                                                     │
│  1. User defines workflow (YAML)                   │
│     ↓                                               │
│                                                     │
│  2. Songbird parses workflow                       │
│     ↓ Discovers required services                  │
│                                                     │
│  3. NestGate discovered                            │
│     ↓ Capabilities validated                        │
│                                                     │
│  4. Workflow execution starts                       │
│     ↓ Step 1: Songbird → NestGate (snapshot)      │
│     ✓ Step 1 complete                              │
│     ↓ Step 2: Songbird → NestGate (backup)        │
│     ✓ Step 2 complete                              │
│     ↓ Step 3: Songbird → NestGate (verify)        │
│     ✓ Step 3 complete                              │
│                                                     │
│  5. Workflow complete                               │
│     ↓ Status reported                              │
│     ↓ Metrics collected                            │
│     ↓ Logs stored                                  │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Workflow State Machine

```
PENDING → DISCOVERING → RUNNING → [SUCCESS | FAILED]
                           ↓
                        RETRYING
                           ↓
                    [SUCCESS | FAILED]
```

---

## 🧪 **EXPERIMENTS TO TRY**

### Experiment 1: Multi-Step Workflow
```yaml
# Create complex-workflow.yaml
apiVersion: songbird/v1
kind: Workflow
metadata:
  name: data-lifecycle

steps:
  - name: ingest
    service: nestgate
    operation: store
    params:
      path: /data/raw/input.csv
      source: http://data-source.com/input.csv
      
  - name: snapshot-raw
    service: nestgate
    operation: snapshot
    params:
      path: /data/raw
      
  - name: process
    service: toadstool  # Compute step!
    operation: execute
    params:
      script: process-data.py
      input: /data/raw/input.csv
      output: /data/processed/output.csv
      
  - name: snapshot-processed
    service: nestgate
    operation: snapshot
    params:
      path: /data/processed
      
  - name: backup-all
    service: nestgate
    operation: backup
    params:
      snapshots:
        - "${steps.snapshot-raw.snapshot_id}"
        - "${steps.snapshot-processed.snapshot_id}"
      destination: s3://backup/

# Run it
songbird workflow run complex-workflow.yaml
```

### Experiment 2: Scheduled Maintenance
```bash
# Create maintenance schedule
songbird schedule create \
  --workflow maintenance.yaml \
  --cron "0 3 * * 0" \
  --name weekly-maintenance

# maintenance.yaml includes:
# - Create snapshots of all datasets
# - Delete old snapshots (>30 days)
# - Run scrub operations
# - Backup to remote
# - Generate health report
```

### Experiment 3: Failure Injection
```yaml
# Test failure handling
apiVersion: songbird/v1
kind: Workflow
metadata:
  name: resilient-backup

steps:
  - name: backup
    service: nestgate
    operation: backup
    retry:
      max_attempts: 3
      backoff: exponential
      fallback:
        - service: nestgate
          operation: backup
          params:
            destination: s3://backup-secondary/

# Run and simulate failures
songbird workflow run resilient-backup.yaml

# Watch it automatically retry and fallback
songbird workflow logs wf_xxx
```

### Experiment 4: Parallel Operations
```yaml
# Parallel data processing
apiVersion: songbird/v1
kind: Workflow
metadata:
  name: parallel-backup

steps:
  - name: create-snapshots
    parallel: true
    tasks:
      - service: nestgate
        operation: snapshot
        params: {path: /data/dataset-1}
      - service: nestgate
        operation: snapshot
        params: {path: /data/dataset-2}
      - service: nestgate
        operation: snapshot
        params: {path: /data/dataset-3}
        
  - name: backup-all
    service: nestgate
    operation: backup_batch
    params:
      snapshots: "${steps.create-snapshots.*.snapshot_id}"

# Snapshots created in parallel (3x faster)
```

---

## 📊 **WORKFLOW PATTERNS**

### Pattern 1: Backup Workflow
```yaml
# Simple, reliable backups
steps:
  - snapshot    # Point-in-time copy
  - backup      # Store remotely
  - verify      # Confirm integrity
  - cleanup     # Remove old snapshots
```

**Use Cases**: Daily backups, disaster recovery, compliance

### Pattern 2: Data Pipeline
```yaml
# ETL (Extract, Transform, Load)
steps:
  - extract     # Pull from sources
  - validate    # Check data quality
  - transform   # Process data
  - load        # Store results
  - index       # Make searchable
```

**Use Cases**: Data warehousing, ML pipelines, analytics

### Pattern 3: Testing Workflow
```yaml
# Automated testing with snapshots
steps:
  - snapshot_production    # Capture current state
  - clone_to_test         # Create test environment
  - run_tests             # Execute test suite
  - rollback              # Restore if needed
  - report                # Generate results
```

**Use Cases**: Pre-deployment testing, validation, QA

### Pattern 4: Disaster Recovery
```yaml
# Automated DR procedures
steps:
  - detect_failure        # Health check failed
  - snapshot_current      # Preserve state
  - restore_from_backup   # Load last good state
  - verify_integrity      # Check restoration
  - notify_team           # Alert operators
```

**Use Cases**: High availability, business continuity

---

## 🆘 **TROUBLESHOOTING**

### "Songbird not found"
**Cause**: Service discovery timeout  
**Fix**:
```bash
# Check Songbird is running
curl http://localhost:6666/health

# If not, start it
cd ../../songbird && ./start.sh

# Manually configure if needed
export SONGBIRD_ENDPOINT=http://localhost:6666
songbird workflow run backup.yaml --endpoint $SONGBIRD_ENDPOINT
```

### "Workflow stuck in RUNNING"
**Cause**: Step may have hung  
**Fix**:
```bash
# Check workflow status
songbird workflow status wf_xxx

# See which step is stuck
songbird workflow logs wf_xxx

# Cancel if needed
songbird workflow cancel wf_xxx

# Increase timeout
songbird workflow run backup.yaml --step-timeout 300s
```

### "Step failed: NestGate unavailable"
**Cause**: NestGate service down  
**Fix**:
```bash
# Check NestGate health
curl http://localhost:8080/health

# Restart if needed
docker-compose restart nestgate

# Retry workflow
songbird workflow retry wf_xxx
```

---

## 📚 **LEARN MORE**

**Songbird Documentation**:
- Songbird Architecture: `../../../songbird/docs/ARCHITECTURE.md`
- Workflow Syntax: `../../../songbird/docs/WORKFLOWS.md`
- Best Practices: `../../../songbird/docs/BEST_PRACTICES.md`

**Integration Guides**:
- NestGate Integration: `../../../docs/guides/SONGBIRD_INTEGRATION.md`
- Workflow Examples: `../../../songbird/examples/`
- Error Handling: `../../../songbird/docs/ERROR_HANDLING.md`

**Workflow Patterns**:
- Backup Patterns: `../../../docs/guides/BACKUP_PATTERNS.md`
- Data Pipelines: `../../../docs/guides/DATA_PIPELINES.md`
- Disaster Recovery: `../../../docs/guides/DISASTER_RECOVERY.md`

---

## ⏭️ **WHAT'S NEXT**

**Completed Demo 2.2?** Great! You now understand:
- ✅ Workflow orchestration
- ✅ Multi-step operations
- ✅ Automatic retry logic
- ✅ Service coordination

**Ready for Demo 2.3?** (`../03_toadstool_storage/`)
- Compute workloads using NestGate
- Volume mounting and management
- Performance characteristics

**Or explore more orchestration**:
- Create custom workflows
- Test failure scenarios
- Schedule operations
- Build data pipelines

---

**Status**: 🚧 Building (Week 2)  
**Estimated time**: 25 minutes  
**Prerequisites**: NestGate + Songbird running

**Key Takeaway**: Automation makes data operations reliable and scalable! 🎼

---

*Demo 2.2 - Songbird Data Service Orchestration*  
*Part of Level 2: Ecosystem Integration*  
*Building Week 2 - December 2025*

