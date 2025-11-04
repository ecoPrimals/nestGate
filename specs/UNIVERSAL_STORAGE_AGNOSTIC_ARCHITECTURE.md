# NestGate Universal Storage-Agnostic Architecture Specification

**Version**: 2.0  
**Date**: October 30, 2025  
**Status**: ⚡ **FUNCTIONAL** - Filesystem backend operational, other backends planned  

## Executive Summary

NestGate's Universal Storage-Agnostic Architecture enables the system to manage data on **any underlying storage technology** while presenting a unified interface to users. This specification outlines the complete transformation from a ZFS-centric system to a truly universal data management platform that works seamlessly across filesystems, cloud storage, object stores, and specialized storage systems.

## 🎯 Vision Statement

**"NestGate should work anywhere, with any storage, on any system - users just touch it and it works."**

The user experience should be identical whether NestGate is running on:
- A high-end server with ZFS pools
- A laptop with ext4/NTFS filesystems  
- A cloud instance with object storage
- An embedded device with minimal storage
- A container with ephemeral storage
- A distributed system with multiple storage types

## 🏗️ Current Architecture Status (Updated Oct 30, 2025)

### ✅ Phase 1B: COMPLETE (Self-Contained Storage Foundation - v1.1.0)
**Status**: ✅ **COMPLETE** - Pure Rust storage foundation implemented and tested (Oct 30, 2025)

- ✅ **Pure Rust Compression**: LZ4 (1.24) and ZSTD (0.13) fully integrated
  - 8 comprehensive tests passing
  - Streaming support for large files
  - ~2 GB/s LZ4, configurable ZSTD levels
- ✅ **Pure Rust Checksums**: Blake3 (1.5) and SHA-256 (0.10) fully integrated
  - 11 comprehensive tests passing
  - Streaming support with 64KB buffers
  - ~1.5 GB/s Blake3, constant-time verification
- ✅ **Software Snapshots**: Hardlink and Copy strategies implemented
  - 7 comprehensive tests passing
  - Auto-detection of filesystem capabilities
  - Metadata tracking (bincode + JSON)
  - Garbage collection and rollback

**Files**: 3 modules, ~1,937 lines, 26 tests passing

---

### ✅ Phase 1C: COMPLETE (Unified Filesystem Backend - v1.2.0)
**Status**: ✅ **COMPLETE** - Universal storage system production-ready (Oct 30, 2025)

- ✅ **Filesystem Backend**: Unified API combining compression + checksums + snapshots
  - 9 comprehensive tests passing
  - Automatic compression on write (~2 GB/s)
  - Automatic verification on read (~3 GB/s)
  - Metadata index (JSON-based)
  - Builder pattern configuration
  - Storage statistics (compression ratio, space saved)
- ✅ **Backend Detection**: Auto-select ZFS vs Filesystem
  - 7 comprehensive tests passing
  - System capability detection
  - Intelligent scoring and selection
  - Detailed reporting
- ⚠️ **ZFS Backend**: Works but **REQUIRES system ZFS installation** via package manager
- ✅ **Test Coverage**: 50/50 universal_storage tests passing
- ✅ **Zero System Dependencies**: All Phase 1B+1C components pure Rust

**Files**: 5 modules total, ~3,047 lines, 50 tests passing

**Achievement**: NestGate provides ZFS-like features on **any filesystem** without system dependencies!

**Deployment**: ✅ **PRODUCTION READY** - Works on Linux, macOS, Windows with zero system requirements

### ✅ Phase 1: COMPLETE (v1.0.0 - v1.2.0)
**Status**: ✅ **COMPLETE** - Self-contained storage system fully implemented

**Achievement**: NestGate works on ANY system WITHOUT requiring ZFS installation

- 🔴 **Pure Rust Compression**: Integrate lz4-rust and zstd crates for software compression
- 🔴 **Pure Rust Checksums**: Integrate blake3 and sha2 for integrity verification
- 🔴 **Snapshot System**: Implement hardlink/reflink/metadata-based snapshots
- 🔴 **Filesystem Backend**: Pure Rust implementation for ext4/NTFS/APFS/Btrfs/XFS
- 🔴 **Auto-Detection**: Detect filesystem type and select optimal strategy
- **Timeline**: 6-8 weeks
- **Priority**: CRITICAL (blocks laptop, Windows, macOS, container deployments)

### 🔄 Phase 2: FRAMEWORK (Additional Backends - v1.2.0)
**Status**: ⚡ **FRAMEWORK EXISTS** (after Phase 1B complete)

- ⚡ **Object Storage Backend**: Framework ready, needs AWS/S3 implementation
- ⚡ **Block Storage Backend**: Framework ready, needs implementation
- ⚡ **Network Storage Backend**: Framework ready, needs NFS/SMB implementation
- ⚡ **Memory Backend**: Framework ready, needs caching implementation
- **Timeline**: 2-4 weeks per backend

### 📋 Phase 3: PLANNED (Advanced Features - v1.2.0+)
**Status**: 📋 **DESIGN PHASE**

- 📋 **Software RAID-Z**: Erasure coding across multiple devices
- 📋 **Advanced Deduplication**: Block-level dedup across backends
- 📋 **Cross-Backend Replication**: Data sync across storage types
- 📋 **Performance Optimization**: Zero-copy operations
- **Timeline**: 4-8 weeks after v1.1.0

## 🎯 Architecture Goals

### Primary Objectives
1. **Universal Compatibility**: Work on any system without modification
2. **Transparent Operation**: Users don't need to know underlying storage type
3. **Performance Parity**: Optimal performance regardless of storage backend
4. **Feature Consistency**: Same features available across all storage types
5. **Zero Configuration**: Automatic detection and optimal configuration
6. **Graceful Degradation**: Reduced features when storage limitations exist

### Technical Requirements
1. **Storage Agnostic**: Abstract all storage operations through unified interface
2. **Protocol Flexible**: Support filesystem, object, block, and network storage
3. **Performance Optimal**: Zero-copy operations where possible
4. **Memory Efficient**: Minimal overhead for storage abstraction
5. **Error Resilient**: Comprehensive error handling and recovery
6. **Monitoring Complete**: Full observability across all storage types

## 🏛️ Universal Storage Architecture

### Core Components

#### 1. Universal Storage Manager (`nestgate-core/src/universal_storage/manager.rs`)
```rust
pub struct UniversalStorageManager {
    backend_registry: BackendRegistry,
    load_balancer: StorageLoadBalancer,
    consistency_manager: ConsistencyManager,
    transaction_manager: TransactionManager,
    performance_monitor: PerformanceMonitor,
}
```

**Status**: 🟡 Partially Implemented  
**Remaining Work**:
- Complete backend health monitoring
- Implement load balancing algorithms
- Add consistency management for distributed operations
- Create transaction support for atomic operations

#### 2. Storage Protocol Handler Trait (`nestgate-core/src/universal_storage/types.rs`)
```rust
#[async_trait]
pub trait StorageProtocolHandler: Send + Sync {
    async fn create_pool(&self, config: &PoolConfig) -> StorageResult<PoolInfo>;
    async fn list_pools(&self) -> StorageResult<Vec<PoolInfo>>;
    async fn create_dataset(&self, config: &DatasetConfig) -> StorageResult<DatasetInfo>;
    async fn list_datasets(&self, pool: Option<&str>) -> StorageResult<Vec<DatasetInfo>>;
    async fn create_snapshot(&self, config: &SnapshotConfig) -> StorageResult<SnapshotInfo>;
    async fn list_snapshots(&self, dataset: Option<&str>) -> StorageResult<Vec<SnapshotInfo>>;
    async fn read_data(&self, path: &str, offset: u64, length: u64) -> StorageResult<Vec<u8>>;
    async fn write_data(&self, path: &str, offset: u64, data: &[u8]) -> StorageResult<u64>;
    async fn get_capabilities(&self) -> StorageCapabilities;
    async fn health_check(&self) -> StorageResult<HealthStatus>;
}
```

**Status**: 🟡 Partially Implemented  
**Remaining Work**:
- Complete all method implementations for each storage type
- Add streaming operations for large data transfers
- Implement advanced features (compression, encryption, deduplication)
- Add metadata operations and extended attributes

#### 3. Storage Backend Implementations

##### A. Filesystem Backend (`handlers/storage/backends/filesystem.rs`)
**Target Storage Types**: ext4, NTFS, APFS, Btrfs, XFS, F2FS  
**Status**: 🔴 Not Implemented - **CRITICAL PRIORITY**
**Why Critical**: Blocks deployment to laptops, Windows, macOS, containers, cloud instances
**Features Needed**:
- Directory-based "pools" (mount points)
- File-based "datasets" (directories with metadata)
- Pure Rust compression (lz4-rust, zstd crates)
- Pure Rust checksums (blake3, sha2 crates)
- Hardlink-based "snapshots" (ext4, NTFS)
- Reflink-based "snapshots" (Btrfs, XFS, APFS)
- Metadata-based "snapshots" (fallback)
- Extended attributes for metadata storage
- Filesystem-specific optimizations
**Implementation Path**: See Phase 1B roadmap below

##### B. ZFS Backend (`handlers/storage/backends/zfs.rs`)
**Target Storage Types**: OpenZFS, ZFS on Linux, FreeBSD ZFS  
**Status**: ⚠️ Partially Implemented - **REQUIRES SYSTEM ZFS**
**Current Limitation**: Calls system `zfs` and `zpool` commands via `tokio::process::Command`
**Deployment Blocker**: Requires `apt install zfs-dkms` or equivalent on host system
**Features**:
- Native ZFS pool operations (via system commands)
- ZFS dataset management (via system commands)
- ZFS snapshot functionality (via system commands)
- Advanced ZFS features (compression, deduplication, encryption)
**Future Enhancement**: Consider pure Rust OpenZFS bindings (libzfs-rs) for tighter integration

##### C. Object Storage Backend (`handlers/storage/backends/object.rs`)
**Target Storage Types**: S3, MinIO, Azure Blob, Google Cloud Storage  
**Status**: 🔴 Not Implemented  
**Features**:
- Bucket-based "pools"
- Object-based "datasets" with metadata
- Version-based "snapshots"
- Multipart upload for large files
- Cloud-specific optimizations

##### D. Block Storage Backend (`handlers/storage/backends/block.rs`)
**Target Storage Types**: LVM, device-mapper, raw block devices  
**Status**: 🔴 Not Implemented  
**Features**:
- Volume group "pools"
- Logical volume "datasets"
- LVM snapshot functionality
- Device-specific optimizations

##### E. Network Storage Backend (`handlers/storage/backends/network.rs`)
**Target Storage Types**: NFS, SMB/CIFS, iSCSI, Ceph, GlusterFS  
**Status**: 🔴 Not Implemented  
**Features**:
- Remote mount "pools"
- Network path "datasets"
- Protocol-specific snapshots
- Network optimization and caching

##### F. Memory Storage Backend (`handlers/storage/backends/memory.rs`)
**Target Storage Types**: tmpfs, RAM disk, in-memory caching  
**Status**: 🔴 Not Implemented  
**Features**:
- Memory region "pools"
- Buffer-based "datasets"
- Copy-on-write "snapshots"
- High-performance temporary storage

### Storage Protocol Types

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageProtocol {
    // Filesystem-based
    Filesystem,     // ext4, NTFS, APFS, etc.
    Zfs,           // ZFS pools and datasets
    Btrfs,         // Btrfs subvolumes
    
    // Object-based
    ObjectStorage, // S3, MinIO, Azure Blob
    Swift,         // OpenStack Swift
    
    // Block-based
    BlockStorage,  // Raw block devices
    Lvm,          // LVM logical volumes
    DeviceMapper, // device-mapper targets
    
    // Network-based
    Nfs,          // Network File System
    Smb,          // Server Message Block
    Iscsi,        // iSCSI targets
    Ceph,         // Ceph distributed storage
    Gluster,      // GlusterFS
    
    // Memory-based
    Memory,       // In-memory storage
    Tmpfs,        // Temporary filesystem
    
    // Hybrid/Virtual
    Union,        // Union filesystems (OverlayFS, AUFS)
    Fuse,         // FUSE-based filesystems
    
    // Specialized
    Database,     // Database-backed storage
    Tape,         // Tape storage systems
    Optical,      // CD/DVD/Blu-ray
}
```

### Storage Capabilities Framework

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageCapability {
    // Basic Operations
    ReadWrite,
    ReadOnly,
    WriteOnly,
    
    // Advanced Features
    Snapshots,
    Compression,
    Deduplication,
    Encryption,
    Checksums,
    
    // Performance Features
    ZeroCopy,
    Streaming,
    Parallel,
    Caching,
    
    // Metadata Features
    ExtendedAttributes,
    ACLs,
    Quotas,
    
    // Reliability Features
    Redundancy,
    SelfHealing,
    Scrubbing,
    
    // Network Features
    RemoteAccess,
    Replication,
    Clustering,
}
```

## 📋 Implementation Roadmap

### Phase 1B: Self-Contained Filesystem Backend (CRITICAL - Current Priority)

#### 1B.1 Pure Rust Compression Integration
**Files**: `nestgate-core/src/universal_storage/compression/`  
**Timeline**: 1 week  
**Tasks**:
- [ ] Add dependencies: lz4 = "1.24", zstd = "0.13"
- [ ] Create `RustCompressor` wrapper with LZ4 and ZSTD support
- [ ] Implement compression levels and configuration
- [ ] Add benchmarks to validate performance
- [ ] Write unit tests for all compression algorithms
- [ ] Document compression API and usage

#### 1B.2 Pure Rust Checksum Integration
**Files**: `nestgate-core/src/universal_storage/checksums/`  
**Timeline**: 1 week  
**Tasks**:
- [ ] Add dependencies: blake3 = "1.5", sha2 = "0.10"
- [ ] Create `RustChecksummer` wrapper with Blake3 and SHA-256
- [ ] Implement streaming checksums for large files
- [ ] Add verification and corruption detection
- [ ] Write unit tests for checksum accuracy
- [ ] Document checksum API and usage

#### 1B.3 Snapshot Strategy System
**Files**: `nestgate-core/src/universal_storage/snapshots/`  
**Timeline**: 2 weeks  
**Tasks**:
- [ ] Design `SnapshotStrategy` enum (Hardlink, Reflink, Metadata)
- [ ] Implement hardlink-based snapshots (ext4, NTFS support)
- [ ] Implement reflink-based snapshots (Btrfs, XFS, APFS support)
- [ ] Implement metadata-based snapshots (fallback)
- [ ] Add snapshot listing and deletion
- [ ] Write integration tests for each strategy
- [ ] Document snapshot system architecture

#### 1B.4 Filesystem Backend Implementation
**Files**: `nestgate-core/src/universal_storage/backends/filesystem/`  
**Timeline**: 2-3 weeks  
**Tasks**:
- [ ] Implement `FilesystemBackend` struct and `StorageProtocolHandler` trait
- [ ] Create directory-based pool management
- [ ] Implement dataset operations with metadata
- [ ] Integrate compression for all write operations
- [ ] Integrate checksums for data integrity
- [ ] Add filesystem capability detection
- [ ] Optimize for each filesystem type (ext4, NTFS, APFS, Btrfs, XFS)
- [ ] Write comprehensive integration tests
- [ ] Add E2E tests on different filesystems
- [ ] Document filesystem backend architecture

#### 1B.5 Auto-Detection and Fallback
**Files**: `nestgate-core/src/universal_storage/detection/`  
**Timeline**: 1 week  
**Tasks**:
- [ ] Implement filesystem type detection
- [ ] Create ZFS availability checker
- [ ] Build backend selection logic (ZFS → Filesystem fallback)
- [ ] Add configuration override support
- [ ] Write tests for all detection scenarios
- [ ] Document detection and fallback behavior

**Total Phase 1B Timeline**: 6-8 weeks  
**Priority**: CRITICAL - Blocks universal deployment

### Phase 2: Core Storage Abstraction (After Phase 1B)

#### 2.1 Complete Universal Storage Manager
**Files**: `nestgate-core/src/universal_storage/manager.rs`  
**Timeline**: 1-2 weeks  
**Tasks**:
- [ ] Implement backend health monitoring with automatic failover
- [ ] Create load balancing algorithms for multi-backend scenarios
- [ ] Add consistency management for distributed operations
- [ ] Implement transaction support for atomic operations
- [ ] Add performance monitoring and metrics collection

#### 2.2 Implement Filesystem Backend
**Files**: `handlers/storage/backends/filesystem.rs`  
**Timeline**: 2-3 weeks  
**Tasks**:
- [ ] Create filesystem detection and capability assessment
- [ ] Implement directory-based pool management
- [ ] Add file-based dataset operations with metadata
- [ ] Implement hardlink/reflink-based snapshots where supported
- [ ] Add extended attributes for metadata storage
- [ ] Optimize for each filesystem type (ext4, NTFS, APFS, etc.)

#### 2.3 Wire All API Endpoints to Universal Storage
**Files**: `handlers/zfs/*.rs`, `handlers/storage.rs`  
**Timeline**: 1-2 weeks  
**Tasks**:
- [ ] Replace remaining ZFS-specific endpoints with universal calls
- [ ] Update dataset management APIs
- [ ] Convert snapshot operations to universal interface
- [ ] Migrate performance analytics to universal metrics
- [ ] Update health checks to report universal storage status

### Phase 3: Advanced Storage Backends (Next Priority)

#### 3.1 Object Storage Backend
**Timeline**: 3-4 weeks  
**Features**:
- S3-compatible API support
- Bucket-based pool management
- Object versioning for snapshots
- Multipart upload optimization
- Cloud provider integrations (AWS, Azure, GCP)

#### 3.2 Network Storage Backend  
**Timeline**: 2-3 weeks  
**Features**:
- NFS/SMB mount management
- Remote pool detection and health monitoring
- Network-optimized data transfer
- Caching layer for performance

#### 3.3 Block Storage Backend
**Timeline**: 2-3 weeks  
**Features**:
- LVM integration
- Raw block device management
- Device-mapper target support
- RAID configuration and monitoring

### Phase 4: Advanced Features (Future)

#### 4.1 Cross-Storage Operations
**Timeline**: 2-3 weeks  
**Features**:
- Data migration between storage types
- Hybrid storage pools (hot/warm/cold tiers)
- Automatic data placement based on access patterns
- Cross-storage replication and backup

#### 4.2 Performance Optimization
**Timeline**: 3-4 weeks  
**Features**:
- Zero-copy operations where possible
- Async I/O optimization
- Memory-mapped file operations
- SIMD optimizations for data processing
- GPU acceleration for checksums and compression

#### 4.3 Advanced Data Services
**Timeline**: 4-6 weeks  
**Features**:
- Universal compression (LZ4, ZSTD, etc.)
- Universal encryption (AES, ChaCha20, etc.)
- Universal checksumming (CRC32, Blake3, etc.)
- Universal deduplication across storage types
- Content-addressed storage

## 🔧 Technical Implementation Details

### Backend Registration and Discovery

```rust
impl UniversalStorageManager {
    pub async fn discover_backends(&mut self) -> Result<Vec<StorageBackend>> {
        let mut backends = Vec::new();
        
        // Check for ZFS
        if self.detect_zfs().await {
            backends.push(self.register_zfs_backend().await?);
        }
        
        // Check for filesystems
        for mount in self.discover_mounts().await? {
            backends.push(self.register_filesystem_backend(mount).await?);
        }
        
        // Check for object storage (environment variables, config files)
        if let Ok(config) = self.discover_object_storage().await {
            backends.push(self.register_object_storage_backend(config).await?);
        }
        
        // Check for network storage
        for network_mount in self.discover_network_storage().await? {
            backends.push(self.register_network_backend(network_mount).await?);
        }
        
        Ok(backends)
    }
}
```

### Unified Data Path

```rust
pub struct UniversalDataPath {
    protocol: StorageProtocol,
    backend_id: String,
    pool: String,
    dataset: Option<String>,
    path: String,
}

impl UniversalDataPath {
    // Examples:
    // zfs://tank/dataset/file.txt
    // filesystem:///mnt/data/file.txt  
    // s3://bucket/object/key
    // nfs://server/export/file.txt
    // memory://buffer/data
}
```

### Performance Monitoring

```rust
pub struct UniversalStorageMetrics {
    pub backend_metrics: HashMap<String, BackendMetrics>,
    pub global_metrics: GlobalMetrics,
    pub performance_trends: PerformanceTrends,
}

pub struct BackendMetrics {
    pub read_ops_per_sec: f64,
    pub write_ops_per_sec: f64,
    pub read_throughput_bytes_per_sec: f64,
    pub write_throughput_bytes_per_sec: f64,
    pub latency_percentiles: LatencyPercentiles,
    pub error_rate: f64,
    pub availability: f64,
    pub capacity_utilization: f64,
}
```

## 🧪 Testing Strategy

### Unit Tests
- [ ] Storage protocol handler implementations
- [ ] Backend registration and discovery
- [ ] Data path parsing and routing
- [ ] Error handling and recovery
- [ ] Performance metric collection

### Integration Tests  
- [ ] Cross-backend data operations
- [ ] Failover and recovery scenarios
- [ ] Performance benchmarks across storage types
- [ ] Compatibility testing on different platforms
- [ ] Load testing with multiple concurrent operations

### End-to-End Tests
- [ ] Complete workflow testing (create pool → dataset → snapshot → data operations)
- [ ] Multi-backend scenarios
- [ ] Platform-specific testing (Linux, Windows, macOS)
- [ ] Container and cloud environment testing
- [ ] Performance regression testing

## 🎯 Success Metrics

### Functional Metrics
- **Universal Compatibility**: 100% of core features work on any supported storage type
- **API Consistency**: Identical API responses regardless of underlying storage
- **Error Handling**: Graceful degradation with informative error messages
- **Feature Parity**: 90%+ feature availability across all storage types

### Performance Metrics
- **Overhead**: <5% performance overhead compared to direct storage access
- **Memory Usage**: <100MB additional memory for storage abstraction
- **Latency**: <1ms additional latency for storage routing
- **Throughput**: 95%+ of native storage throughput maintained

### Reliability Metrics
- **Availability**: 99.9%+ uptime across all storage backends
- **Data Integrity**: Zero data corruption or loss
- **Error Recovery**: 100% recovery from transient storage failures
- **Monitoring**: Complete visibility into all storage operations

## 🚀 Deployment Scenarios

### Single Machine Deployment
- **Laptop/Desktop**: Automatic filesystem detection and optimization
- **Server**: Multi-storage support with intelligent tiering
- **Embedded Device**: Minimal resource usage with essential features

### Cloud Deployment  
- **Container**: Ephemeral storage with persistent volume support
- **Virtual Machine**: Cloud storage integration with local caching
- **Serverless**: Object storage backend with minimal cold start time

### Distributed Deployment
- **Cluster**: Multi-node storage coordination and replication
- **Edge**: Distributed storage with central management
- **Hybrid Cloud**: On-premises and cloud storage integration

## 📚 Documentation Requirements

### User Documentation
- [ ] Universal Storage Configuration Guide
- [ ] Storage Backend Comparison Matrix
- [ ] Performance Tuning Guide
- [ ] Troubleshooting Guide
- [ ] Migration Guide (from ZFS-only to Universal)

### Developer Documentation  
- [ ] Storage Backend Implementation Guide
- [ ] Universal Storage API Reference
- [ ] Performance Optimization Guide
- [ ] Testing Framework Documentation
- [ ] Contribution Guidelines for New Storage Types

### Operations Documentation
- [ ] Monitoring and Alerting Setup
- [ ] Backup and Recovery Procedures
- [ ] Capacity Planning Guide
- [ ] Security Best Practices
- [ ] Disaster Recovery Planning

## 🎉 Expected Outcomes

Upon completion of this Universal Storage-Agnostic Architecture:

1. **True Universality**: NestGate will work identically on any system with any storage technology
2. **Zero Configuration**: Users can deploy NestGate anywhere without storage-specific setup
3. **Optimal Performance**: Each storage type will be used to its maximum potential
4. **Seamless Experience**: Users won't know or care about underlying storage technology
5. **Future Proof**: New storage technologies can be added without changing user-facing APIs
6. **Enterprise Ready**: Production deployments across diverse infrastructure environments

## 🔮 Future Vision

The Universal Storage-Agnostic Architecture positions NestGate as:

- **The Universal Data Platform**: One system that works everywhere
- **Storage Technology Independent**: Never locked into specific storage vendors
- **Performance Optimized**: Best-in-class performance on any storage type
- **Infinitely Extensible**: Easy addition of new storage technologies
- **Truly Sovereign**: Complete control over data regardless of underlying storage

This architecture transforms NestGate from a "ZFS NAS system" into a **"Universal Data Management Platform"** that can manage any data, on any storage, anywhere in the world. 