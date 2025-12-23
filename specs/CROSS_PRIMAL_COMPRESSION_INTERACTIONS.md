# Cross-Primal Compression: Network Effect Architecture

**Date**: December 22, 2025  
**Status**: 🎯 Architecture Design  
**Philosophy**: Complexity solved by primal interactions, not monolithic design

---

## Core Insight: Focused Primals, Emergent Intelligence

```
❌ WRONG: Monolithic "smart" compression engine

✅ RIGHT: Multiple focused primals interacting to solve compression
```

### The Network Effect

```
┌─────────────────────────────────────────────────────────────────┐
│          COMPLEXITY EMERGES FROM INTERACTIONS                    │
└─────────────────────────────────────────────────────────────────┘

Simple Primal A  +  Simple Primal B  =  Complex Capability

RhizoCrypt       +  NestGate          =  Git-like versioning
(tracks deltas)     (stores blobs)

RhizoCrypt       +  RhizoCrypt        =  Deduplication across sessions
(session 1)         (session 2)           (shared payloads)

RhizoCrypt       +  LoamSpine         =  Provable compression
(delta chains)      (Merkle roots)        (verify without full data)

RhizoCrypt       +  SweetGrass        =  Attribution of space savings
(compression)       (provenance)          (who saved what)

NestGate         +  NestGate          =  Cross-tower deduplication
(tower A)           (tower B)             (federated storage)
```

---

## Interaction Pattern 1: NAS Optimization

**Use Case**: Optimize storage for 100TB photo library across multiple users

### Primal Roles

```rust
// RhizoCrypt Instance: "NAS Optimizer"
pub struct NasOptimizerSession {
    session_type: "nas_scan",
    purpose: "Find duplicate and similar files across storage",
}

impl NasOptimizer {
    async fn scan_directory(&mut self, path: &Path) -> Result<()> {
        // Walk directory tree
        for entry in walk_dir(path) {
            let hash = blake3::hash_file(&entry.path)?;
            
            // Append to DAG
            self.append_event(EventType::FileDiscovered {
                path: entry.path,
                hash,
                size: entry.size,
                mtime: entry.mtime,
            }).await?;
            
            // Check for similar files (perceptual hashing for images)
            let similar = self.find_similar_files(&hash).await?;
            
            if !similar.is_empty() {
                self.append_event(EventType::SimilarityDetected {
                    file: hash,
                    similar_to: similar,
                }).await?;
            }
        }
        
        Ok(())
    }
    
    async fn dehydrate_to_nestgate(&mut self) -> Result<DehydrationSummary> {
        // Create deduplication plan
        let plan = self.create_dedup_plan().await?;
        
        // Summary:
        // - Original size: 100TB
        // - Unique data: 60TB
        // - Duplicates: 40TB
        // - Similar (delta-able): 20TB
        // - Final storage: 45TB (55% savings!)
        
        DehydrationSummary {
            original_size: 100 * TB,
            unique_data: 60 * TB,
            dedup_savings: 40 * TB,
            delta_savings: 15 * TB,
            final_size: 45 * TB,
            compression_ratio: 2.22,
            dedup_plan: plan,
        }
    }
}

// NestGate Instance: "Cold Storage"
pub struct ColdStorageNestGate {
    config: NestGateConfig {
        compression: "zstd-19",  // Maximum compression
        deduplication: true,
        access_pattern: "cold",   // Optimize for write, rare reads
    }
}

impl ColdStorageNestGate {
    async fn apply_dedup_plan(&self, plan: &DedupPlan) -> Result<()> {
        for group in &plan.duplicate_groups {
            // Store one copy, create refs for others
            let primary = &group[0];
            self.store(primary.hash, primary.data).await?;
            
            for duplicate in &group[1..] {
                // Just create a reference, don't store bytes
                self.create_ref(duplicate.hash, primary.hash).await?;
            }
        }
        
        for delta_chain in &plan.delta_chains {
            // Store base + deltas
            let base = &delta_chain.base;
            self.store(base.hash, base.data).await?;
            
            for variant in &delta_chain.variants {
                // Store only delta
                self.store_delta(variant.hash, base.hash, &variant.delta).await?;
            }
        }
        
        Ok(())
    }
}
```

**Interaction Result**: 55% storage savings through RhizoCrypt finding patterns + NestGate executing the plan

---

## Interaction Pattern 2: Change Tracking

**Use Case**: Version control for scientific datasets (like Git LFS but better)

### Primal Roles

```rust
// RhizoCrypt Instance: "Change Tracker"
pub struct ChangeTrackerSession {
    session_type: "version_control",
    purpose: "Track changes to large datasets over time",
}

impl ChangeTracker {
    async fn commit(&mut self, changes: &[FileChange]) -> Result<CommitId> {
        // For each changed file
        for change in changes {
            match change.change_type {
                ChangeType::Added => {
                    // New file - store full content
                    let hash = self.store_payload(&change.content).await?;
                    
                    self.append_event(EventType::FileAdded {
                        path: change.path.clone(),
                        hash,
                        size: change.content.len(),
                    }).await?;
                }
                
                ChangeType::Modified => {
                    // Modified file - compute delta from previous version
                    let prev_hash = self.get_previous_version(&change.path)?;
                    let prev_content = self.get_payload(&prev_hash).await?;
                    
                    // Compute delta (like Git)
                    let delta = compute_delta(&prev_content, &change.content)?;
                    
                    // Store delta if it's smaller
                    let delta_size = delta.encoded_size();
                    let full_size = change.content.len();
                    
                    let hash = if delta_size < full_size * 0.5 {
                        // Store delta
                        self.store_delta(prev_hash, delta).await?
                    } else {
                        // Delta not worth it, store full
                        self.store_payload(&change.content).await?
                    };
                    
                    self.append_event(EventType::FileModified {
                        path: change.path.clone(),
                        prev_hash,
                        new_hash: hash,
                        delta_size: Some(delta_size),
                    }).await?;
                }
                
                ChangeType::Deleted => {
                    self.append_event(EventType::FileDeleted {
                        path: change.path.clone(),
                    }).await?;
                }
            }
        }
        
        // Create commit vertex
        let commit_id = self.create_commit(
            changes.len(),
            "User commit message",
        ).await?;
        
        Ok(commit_id)
    }
    
    async fn checkout(&self, commit_id: CommitId) -> Result<Vec<File>> {
        // Walk DAG from commit to reconstruct state
        let files = self.reconstruct_state_at(commit_id).await?;
        
        // Reconstruct deltas
        for file in &mut files {
            if file.is_delta {
                // Walk back to base version
                let base = self.find_base_version(&file.hash).await?;
                let deltas = self.collect_delta_chain(&file.hash, &base).await?;
                
                // Apply deltas in order
                let mut content = base.content;
                for delta in deltas {
                    content = delta.apply(&content)?;
                }
                
                file.content = content;
            }
        }
        
        Ok(files)
    }
}

// NestGate Instance: "Hot Storage"
pub struct HotStorageNestGate {
    config: NestGateConfig {
        compression: "lz4",      // Fast decompression
        deduplication: true,
        access_pattern: "hot",    // Optimize for reads
        cache_deltas: true,       // Cache reconstructed versions
    }
}
```

**Interaction Result**: Git-like versioning for datasets, with intelligent delta storage

---

## Interaction Pattern 3: Session Event Logging

**Use Case**: Track game session, scientific experiment, or ML training run

### Primal Roles

```rust
// RhizoCrypt Instance: "Session Logger"
pub struct SessionLoggerSession {
    session_type: "ephemeral_events",
    purpose: "High-frequency event capture with minimal overhead",
}

impl SessionLogger {
    async fn log_event(&mut self, event: GameEvent) -> Result<VertexId> {
        // High-frequency events (1000s per second)
        // Need minimal overhead
        
        match event {
            GameEvent::PlayerMove { position, velocity } => {
                // Inline small data
                self.append_event(EventType::AgentAction {
                    agent: self.player_did,
                    action: Action::Move {
                        position,
                        velocity,
                    },
                }).await?
            }
            
            GameEvent::ItemLoot { item_data } => {
                // Large data - store as payload
                let hash = self.store_payload(&item_data).await?;
                
                self.append_event(EventType::DataCreate {
                    data_ref: hash,
                    schema: SchemaRef::from_name("game_item"),
                }).await?
            }
        }
    }
    
    async fn dehydrate_for_commit(&mut self) -> Result<DehydrationSummary> {
        // Session ended - what do we keep?
        
        // Discard: All PlayerMove events (ephemeral)
        // Keep: ItemLoot events (for provenance)
        // Keep: Combat events (for anti-cheat)
        // Keep: Extraction event (for certificate)
        
        let important_events = self.filter_events(|event| {
            matches!(event.event_type,
                EventType::ItemLoot { .. } |
                EventType::Combat { .. } |
                EventType::Extraction { .. }
            )
        });
        
        // Compute Merkle root over ALL events (including ephemeral)
        let merkle_root = self.compute_merkle_root();
        
        // But only commit important payloads to NestGate
        let mut committed_payloads = Vec::new();
        for event in important_events {
            if let Some(payload_ref) = event.payload {
                committed_payloads.push(payload_ref);
            }
        }
        
        DehydrationSummary {
            merkle_root,
            total_events: self.vertices.len(),
            committed_events: important_events.len(),
            committed_payloads,
            discarded_events: self.vertices.len() - important_events.len(),
        }
    }
}

// NestGate Instance: "Ephemeral Buffer"
pub struct EphemeralBufferNestGate {
    config: NestGateConfig {
        compression: "none",      // No compression for speed
        deduplication: false,
        access_pattern: "ephemeral",
        ttl: Duration::hours(24), // Auto-delete after 24h
    }
}
```

**Interaction Result**: High-speed event capture with selective persistence

---

## Interaction Pattern 4: Cross-Tower Deduplication

**Use Case**: Westgate (cold storage) + Stradgate (backup) share common data

### Primal Interactions

```rust
// RhizoCrypt Instance: "Federation Analyzer"
pub struct FederationAnalyzer {
    session_type: "cross_tower_dedup",
    purpose: "Find redundancy across federated NestGate towers",
}

impl FederationAnalyzer {
    async fn analyze_towers(&mut self, towers: &[NestGateTower]) -> Result<DedupReport> {
        // Query Songbird for available NestGate nodes
        let nodes = self.songbird.discover_services("nestgate").await?;
        
        // For each node, get bloom filter of stored hashes
        let mut bloom_filters = Vec::new();
        for node in &nodes {
            let bloom = node.get_bloom_filter().await?;
            bloom_filters.push((node.id, bloom));
        }
        
        // Find common hashes
        let mut common = HashSet::new();
        for i in 0..bloom_filters.len() {
            for j in (i+1)..bloom_filters.len() {
                let (id_a, bloom_a) = &bloom_filters[i];
                let (id_b, bloom_b) = &bloom_filters[j];
                
                // Estimate intersection
                let intersection = bloom_a.intersect_estimate(bloom_b);
                
                if intersection.count > 1000 {
                    // Significant overlap - worth investigating
                    self.append_event(EventType::RedundancyDetected {
                        tower_a: id_a.clone(),
                        tower_b: id_b.clone(),
                        estimated_overlap: intersection,
                    }).await?;
                    
                    common.extend(intersection.hashes);
                }
            }
        }
        
        // Create deduplication plan
        let plan = self.create_federation_dedup_plan(&common, &nodes).await?;
        
        Ok(DedupReport {
            total_towers: nodes.len(),
            redundant_data: plan.savings,
            plan,
        })
    }
}

// NestGate Towers coordinate
pub struct NestGateFederation {
    towers: Vec<NestGateTower>,
}

impl NestGateFederation {
    async fn apply_dedup_plan(&self, plan: &FederationDedupPlan) -> Result<()> {
        // Designate primary storage for each common hash
        for (hash, primary_tower) in &plan.primary_assignments {
            // Other towers create refs instead of storing
            for tower in &self.towers {
                if tower.id != *primary_tower {
                    tower.create_federation_ref(hash, primary_tower).await?;
                }
            }
        }
        
        Ok(())
    }
}
```

**Interaction Result**: Federated deduplication across multiple NestGate towers

---

## Interaction Pattern 5: Compression Attribution

**Use Case**: Track who saved space, reward contributors via sunCloud

### Primal Interactions

```rust
// SweetGrass creates Braids for compression events
pub struct CompressionBraid {
    data_hash: ContentHash,
    was_generated_by: Activity {
        activity_type: "Compression",
        used: [original_data_ref],
        was_associated_with: [
            Agent { did: rhizocrypt_instance, role: "ComputeProvider" },
            Agent { did: user_did, role: "Creator" },
        ],
        compute_units: 0.01,  // Compression cost
    },
    
    metadata: {
        original_size: 100 * MB,
        compressed_size: 20 * MB,
        savings: 80 * MB,
        algorithm: "zstd + delta",
        compression_ratio: 5.0,
    },
}

// sunCloud calculates space-savings attribution
impl SunCloudEconomics {
    async fn calculate_storage_savings_reward(
        &self,
        entity: EntityReference,
    ) -> Result<Vec<RewardShare>> {
        // Query SweetGrass for compression Braids
        let braids = self.sweetgrass
            .query_by_activity_type("Compression")
            .for_entity(&entity)
            .await?;
        
        let mut total_savings = 0u64;
        let mut contributors = HashMap::new();
        
        for braid in braids {
            let savings = braid.metadata.savings;
            total_savings += savings;
            
            // Attribute savings to all contributors
            for agent in &braid.was_generated_by.was_associated_with {
                *contributors.entry(&agent.did).or_insert(0) += savings;
            }
        }
        
        // Storage has economic value
        let storage_cost_per_gb = Decimal::new(1, 2);  // $0.01 per GB per month
        let total_value = Decimal::from(total_savings / GB) * storage_cost_per_gb;
        
        // Distribute based on contribution
        let shares = contributors.into_iter()
            .map(|(did, savings)| {
                let share = savings as f64 / total_savings as f64;
                RewardShare {
                    agent: did.clone(),
                    share,
                    amount: total_value * Decimal::from_f64(share).unwrap(),
                    reason: "Storage space savings",
                }
            })
            .collect();
        
        Ok(shares)
    }
}
```

**Interaction Result**: Economic attribution for compression contributions

---

## The Emergence Pattern

```
┌─────────────────────────────────────────────────────────────────┐
│              COMPLEXITY FROM SIMPLE INTERACTIONS                 │
└─────────────────────────────────────────────────────────────────┘

Level 1: Individual Primals (Simple)
  ├─ RhizoCrypt: Track events in a DAG
  ├─ NestGate: Store blobs by hash
  ├─ LoamSpine: Append-only ledger
  └─ SweetGrass: Provenance tracking

Level 2: Pairwise Interactions (Focused Capabilities)
  ├─ RhizoCrypt + NestGate → Git-like versioning
  ├─ RhizoCrypt + LoamSpine → Provable compression
  ├─ RhizoCrypt + SweetGrass → Compression attribution
  └─ NestGate + NestGate → Federated deduplication

Level 3: Multi-Primal Interactions (Complex Capabilities)
  ├─ RhizoCrypt + NestGate + LoamSpine → Version control with provenance
  ├─ RhizoCrypt + NestGate + SweetGrass + sunCloud → Economic storage optimization
  └─ Multiple RhizoCrypt + Multiple NestGate → Distributed git across federation

Level 4: Ecosystem (Emergent Intelligence)
  └─ The whole system optimizes itself through primal interactions
```

---

## Implementation: Multiple RhizoCrypt Instances

```rust
// Each RhizoCrypt has a focused purpose
pub enum RhizoCryptInstanceType {
    /// High-frequency event capture (gaming, sensors)
    SessionLogger {
        target_throughput: u64,  // events/sec
        retention: Duration,
    },
    
    /// Change tracking (version control)
    ChangeTracker {
        delta_threshold: f64,    // When to use deltas
        history_depth: usize,
    },
    
    /// Storage optimization (NAS, backup)
    StorageOptimizer {
        scan_depth: usize,
        similarity_threshold: f64,
    },
    
    /// Federation coordinator
    FederationAnalyzer {
        tower_count: usize,
        sync_interval: Duration,
    },
}

// Spawn instances as needed
pub struct RhizoCryptOrchestrator {
    instances: HashMap<SessionId, RhizoCryptInstance>,
}

impl RhizoCryptOrchestrator {
    async fn spawn_for_purpose(
        &mut self,
        purpose: RhizoCryptInstanceType,
    ) -> Result<SessionHandle> {
        let instance = match purpose {
            RhizoCryptInstanceType::SessionLogger { .. } => {
                RhizoCryptInstance::new_ephemeral(purpose)
            }
            RhizoCryptInstanceType::ChangeTracker { .. } => {
                RhizoCryptInstance::new_persistent(purpose)
            }
            RhizoCryptInstanceType::StorageOptimizer { .. } => {
                RhizoCryptInstance::new_long_running(purpose)
            }
            RhizoCryptInstanceType::FederationAnalyzer { .. } => {
                RhizoCryptInstance::new_coordinating(purpose)
            }
        };
        
        let session_id = instance.session_id;
        self.instances.insert(session_id, instance);
        
        Ok(SessionHandle { session_id, orchestrator: self })
    }
}
```

---

## Idiomatic Rust: Trait-Based Composition

```rust
/// Core RhizoCrypt trait - all instances implement this
pub trait RhizoCryptCore {
    async fn append_event(&mut self, event: EventType) -> Result<VertexId>;
    async fn compute_merkle_root(&self) -> Result<MerkleRoot>;
    fn session_id(&self) -> SessionId;
}

/// Extension: Delta compression capability
pub trait DeltaCompression: RhizoCryptCore {
    async fn compute_delta(&self, base: PayloadRef, target: &[u8]) -> Result<DeltaOps>;
    async fn apply_delta(&self, base: PayloadRef, delta: &DeltaOps) -> Result<Bytes>;
    async fn find_delta_candidates(&self, target: &[u8]) -> Result<Vec<PayloadRef>>;
}

/// Extension: Similarity detection
pub trait SimilarityDetection: RhizoCryptCore {
    async fn compute_perceptual_hash(&self, data: &[u8]) -> Result<PerceptualHash>;
    async fn find_similar(&self, hash: &PerceptualHash, threshold: f64) -> Result<Vec<SimilarItem>>;
}

/// Extension: Federation coordination
pub trait FederationCoordination: RhizoCryptCore {
    async fn query_peer_bloom(&self, peer: PeerId) -> Result<BloomFilter>;
    async fn estimate_redundancy(&self, peers: &[PeerId]) -> Result<RedundancyEstimate>;
}

// Compose capabilities as needed
pub struct StorageOptimizerSession {
    core: Box<dyn RhizoCryptCore>,
    delta: Box<dyn DeltaCompression>,
    similarity: Box<dyn SimilarityDetection>,
}

pub struct FederationAnalyzerSession {
    core: Box<dyn RhizoCryptCore>,
    federation: Box<dyn FederationCoordination>,
}
```

---

## Zero Technical Debt

```rust
// Each primal stays focused
// No monolithic "CompressionEngine" with 50 strategies
// Instead: Multiple instances, each doing ONE thing well

// WRONG: Monolithic
struct CompressionEngine {
    nas_optimizer: NasOptimizer,
    change_tracker: ChangeTracker,
    session_logger: SessionLogger,
    federation_analyzer: FederationAnalyzer,
    // ... 20 more strategies
    // Technical debt: everything coupled
}

// RIGHT: Focused primals
// Each RhizoCrypt instance is independent
// Complexity emerges from interactions
// Each can evolve independently
// No coupling = no debt
```

---

## Summary: Network Effect in Action

**What we built:**

1. ✅ **RhizoCrypt as intelligent compressor** (Git-like DAG)
2. ✅ **NestGate as storage backend** (content-addressed blobs)
3. ✅ **Multiple instances for different purposes** (NAS, version control, sessions, federation)
4. ✅ **Cross-primal interactions** create complex capabilities
5. ✅ **Trait-based composition** for Rust idiomacy
6. ✅ **Zero technical debt** through focused design

**The key insight:**
> Complexity solved by network effect of primal interactions, not monolithic design

**When to use what:**
- **NAS optimization?** Spawn StorageOptimizer RhizoCrypt
- **Version control?** Spawn ChangeTracker RhizoCrypt  
- **Game session?** Spawn SessionLogger RhizoCrypt
- **Federation sync?** Spawn FederationAnalyzer RhizoCrypt

Each instance is simple. The interactions create intelligence.

---

**Next Step**: Build out RhizoCrypt's delta compression and similarity detection modules, making it the Git of the ecoPrimals ecosystem.

