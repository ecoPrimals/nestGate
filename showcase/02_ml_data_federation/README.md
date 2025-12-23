# Live ML Data Federation: Complete Pipeline

**Date**: December 21, 2025  
**Status**: 🎯 Building Now  
**Focus**: Real-world ML data workflows with NestGate + ToadStool + Songbird

## 🌐 The Vision

Build **live working demos** for production ML data management:

1. **NCBI Datasets** - Scientific data storage and versioning
2. **Model Checkpointing** - Automatic checkpoint saving during training
3. **Hugging Face Mirroring** - Local cache to reduce egress costs
4. **Frozen Culture** - Immutable dataset/model snapshots for reproducibility
5. **Federation** - Distribute data across Westgate (cold storage) + Stradgate (backup)

## 🏗️ Architecture

```
┌────────────────────────────────────────────────────────────────┐
│                    SONGBIRD ORCHESTRATOR                       │
│           (Dynamic Discovery + Compute Bridge)                 │
└───────────┬────────────────────────────────────────────────────┘
            │
            ├─────────────────────┬──────────────────────────────┐
            │                     │                              │
    ┌───────▼────────┐    ┌──────▼────────┐         ┌──────────▼─────────┐
    │  WESTGATE      │    │  STRADGATE    │         │    TOADSTOOL       │
    │  Cold Storage  │    │  Backup       │         │    ML Compute      │
    │                │◄───┤               │         │                    │
    │  • NCBI Data   │    │  • Replicas   │         │  • Training        │
    │  • HF Models   │    │  • Sharding   │         │  • Inference       │
    │  • Checkpoints │    │  • Failover   │         │  • Checkpointing   │
    │  • ZFS Snaps   │    │               │         │                    │
    └────────────────┘    └───────────────┘         └────────────────────┘
```

## 📊 Use Cases

### 1. NCBI Dataset Management

**Problem**: Bioinformatics data is large, slow to download, and expensive

**Solution**: Mirror NCBI datasets to Westgate cold storage

```
Download once → Store in Westgate → Share across team
                         ↓
                  ZFS compression (2-3x)
                  ZFS deduplication
                  Instant snapshots
```

**Benefits**:
- ✅ Download once, use forever
- ✅ No egress fees (local access)
- ✅ Fast access (local network)
- ✅ Versioned (ZFS snapshots)
- ✅ Team sharing (multi-user)

### 2. Model Checkpoint Management

**Problem**: Training takes days, crashes lose progress, checkpoints are large

**Solution**: Automatic checkpointing to NestGate

```
ToadStool Training
        ↓ (every N epochs)
Save Checkpoint to Westgate
        ↓ (ZFS dedup)
Storage efficient (only differences)
        ↓ (on crash)
Resume from last checkpoint
```

**Benefits**:
- ✅ Never lose training progress
- ✅ Resume from any epoch
- ✅ Efficient storage (dedup)
- ✅ Automatic workflow

### 3. Hugging Face Local Mirror

**Problem**: Downloading models repeatedly costs money and time

**Solution**: Local Hugging Face cache on Westgate

```
First Download: HF Hub → Westgate (cache)
                           ↓
Subsequent: Westgate cache → ToadStool (instant!)
                           ↓
Team Access: Everyone uses cached version
```

**Benefits**:
- ✅ Reduce egress costs (80-90% savings)
- ✅ Faster model loading
- ✅ Offline capability
- ✅ Version control (frozen models)

### 4. Frozen Culture (Reproducibility)

**Problem**: ML experiments must be reproducible, but data/models change

**Solution**: ZFS snapshots create immutable "frozen" versions

```
Dataset v1 → Snapshot → Frozen forever
  ↓
Train Model → Checkpoint → Snapshot → Frozen
  ↓
Publish Paper → Link to frozen data + model → Reproducible!
```

**Benefits**:
- ✅ Perfect reproducibility
- ✅ Audit trail
- ✅ Compliance (FDA, HIPAA)
- ✅ Scientific integrity

## 🎯 Demo Structure

```
showcase/02_ml_data_federation/
├── README.md                           # This file
├── 01-ncbi-datasets/
│   ├── demo-ncbi-mirror.sh            # Download and mirror NCBI data
│   ├── demo-ncbi-versioning.sh        # Version datasets with ZFS
│   └── README.md
├── 02-checkpoint-federation/
│   ├── demo-distributed-checkpointing.sh  # Multi-node checkpointing
│   ├── demo-resume-training.sh        # Resume from checkpoint
│   └── README.md
├── 03-huggingface-cache/
│   ├── demo-hf-mirror.sh              # Local HF model cache
│   ├── demo-model-library.sh          # Team model library
│   └── README.md
├── 04-frozen-culture/
│   ├── demo-dataset-snapshot.sh       # Immutable dataset versions
│   ├── demo-reproducible-experiment.sh # Complete reproducible workflow
│   └── README.md
├── 05-complete-pipeline/
│   ├── demo-ml-pipeline-live.sh       # End-to-end live demo
│   ├── deploy-compute-bridge.sh       # Deploy ToadStool via Songbird
│   └── README.md
└── workflows/
    ├── ml-data-federation.yaml        # Songbird workflow
    └── README.md
```

## 🚀 Implementation Plan

### Phase 1: NCBI Dataset Management (Now)

**Goal**: Store and version scientific datasets

**Deliverables**:
1. Demo script to download NCBI dataset
2. Store in Westgate with metadata
3. ZFS snapshot for versioning
4. Discovery via Songbird registry

**Time**: 2-3 hours

### Phase 2: Distributed Checkpointing

**Goal**: ToadStool saves checkpoints to NestGate federation

**Deliverables**:
1. ToadStool checkpoint client
2. Automatic saving every N epochs
3. Distributed across Westgate + Stradgate
4. Resume training demo

**Time**: 3-4 hours

### Phase 3: Hugging Face Mirror

**Goal**: Local HF model cache

**Deliverables**:
1. HF download → Westgate storage
2. Cache lookup logic
3. Model library management
4. Team sharing

**Time**: 2-3 hours

### Phase 4: Frozen Culture

**Goal**: Reproducible experiments

**Deliverables**:
1. Dataset snapshot workflow
2. Model + data linking
3. Reproducibility demo
4. Compliance features

**Time**: 2 hours

### Phase 5: Complete Pipeline

**Goal**: End-to-end live demo

**Deliverables**:
1. Songbird compute bridge deployment
2. ToadStool → NestGate live integration
3. Multi-node workflow
4. Production-ready patterns

**Time**: 3-4 hours

**Total**: 12-16 hours of focused work

## 💡 Key Features

### Dynamic Federation (No Hardcoding!)

Using Songbird's service registry:

```yaml
# Workflow discovers storage dynamically
discovery:
  primal_type: "nestgate"
  required_capabilities: ["storage", "cold-storage"]

# ToadStool discovers compute dynamically
discovery:
  primal_type: "toadstool"
  required_capabilities: ["ml-training", "gpu"]
```

### Songbird Compute Bridge

Deploy ToadStool to remote nodes:

```bash
# Songbird deploys ToadStool binary to Stradgate
songbird deploy \
  --binary toadstool-worker \
  --target stradgate \
  --capabilities ml-training,gpu

# ToadStool auto-registers with Songbird
# Workflow discovers it dynamically
```

### Automatic Replication

Westgate → Stradgate backup:

```yaml
stages:
  - name: store-checkpoint
    tower: westgate  # Primary cold storage
  
  - name: replicate-checkpoint
    tower: stradgate  # Automatic backup
```

## 🎓 Real-World Benefits

### Cost Savings

**Scenario**: Team of 10 researchers

**Before**:
- Each downloads 100GB HF models independently
- 10 × 100GB = 1TB egress
- At $0.09/GB = **$90/month**

**After**:
- Download once to Westgate
- Everyone uses local cache
- 100GB egress
- At $0.09/GB = **$9/month**

**Savings**: **90% reduction** ($81/month)

### Time Savings

**Scenario**: Training large model

**Before**:
- 7 days of training
- Crash at day 6
- Restart from scratch
- **14 days total**

**After**:
- 7 days of training
- Automatic checkpoints
- Crash at day 6
- Resume from last checkpoint
- **7 days total**

**Savings**: **50% faster**

### Reproducibility

**Scenario**: Published research

**Before**:
- "We used NCBI dataset downloaded in March"
- Dataset has changed since
- Results not reproducible

**After**:
- ZFS snapshot: `ncbi-dataset-2025-03-15`
- Permanent, immutable
- Anyone can reproduce exactly

**Result**: **Perfect reproducibility**

## 🔧 Technical Details

### ZFS Features We Use

1. **Compression** - Save 50-70% space on text datasets
2. **Deduplication** - ML checkpoints share weights
3. **Snapshots** - Instant, immutable versions
4. **Copy-on-Write** - Efficient incremental saves

### API Endpoints

**NestGate Storage API**:
```
POST   /api/v1/data/store          # Store data/model
GET    /api/v1/data/retrieve/{id}  # Retrieve data
POST   /api/v1/data/snapshot        # Create ZFS snapshot
GET    /api/v1/data/list            # List stored items
```

**Songbird Registry API**:
```
POST   /api/v1/services/register    # Register service
GET    /api/v1/services/discover    # Find services
POST   /api/v1/workflows/execute    # Run workflow
```

### Data Formats

**Checkpoint**:
```json
{
  "checkpoint_id": "mnist-epoch-100",
  "model_name": "mnist-cnn",
  "epoch": 100,
  "loss": 0.032,
  "accuracy": 0.987,
  "weights_hash": "sha256:abc123...",
  "optimizer_state": "...",
  "created_at": "2025-12-21T12:00:00Z"
}
```

**Dataset Metadata**:
```json
{
  "dataset_id": "ncbi-genomics-2025-12",
  "source": "ncbi.nlm.nih.gov",
  "version": "v1.0.0",
  "samples": 1000000,
  "size_bytes": 107374182400,
  "format": "fasta",
  "snapshot": "pool0/datasets/ncbi@2025-12-21",
  "frozen": true
}
```

## 🎯 Success Criteria

After building this showcase:

- [ ] ✅ NCBI dataset download and storage working
- [ ] ✅ ZFS snapshots for versioning
- [ ] ✅ ToadStool → NestGate checkpointing live
- [ ] ✅ Hugging Face mirror functional
- [ ] ✅ Dynamic discovery via Songbird
- [ ] ✅ Multi-node federation (Westgate + Stradgate)
- [ ] ✅ Compute bridge deployment working
- [ ] ✅ Complete end-to-end demo
- [ ] ✅ Documentation and tutorials

## 📚 References

- **ToadStool ML Checkpoints**: `../toadstool/showcase/nestgate-integration/02-ml-checkpoints/`
- **ToadStool Dataset Management**: `../toadstool/showcase/nestgate-integration/03-dataset-management/`
- **Songbird Federation**: `01_nestgate_songbird_live/`
- **Network Effect Architecture**: `01_nestgate_songbird_live/NETWORK_EFFECT.md`
- **BearDog Encryption**: `02_ecosystem_integration/live/`

---

**Status**: 📋 PLAN DEFINED - Ready to build  
**Next**: Implement Phase 1 (NCBI Dataset Management)  
**Timeline**: 12-16 hours focused work  
**Impact**: Production-ready ML data management for ecoPrimals

