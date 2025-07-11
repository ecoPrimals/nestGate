---
description: Universal primal storage architecture with real-time synchronization and distributed data management
globs: ["nestgate/src/**/*.rs", "nestgate/code/**/*.rs"]
---

# Universal Primal Storage Specification

## Context
- When implementing universal primal storage coordination
- When providing multiple primal integration support
- When managing distributed primal synchronization
- When integrating with ecosystem-wide primal coordination

## Requirements

### Universal Primal Integration
- Implement universal primal interface for any storage system
- Support capability-based primal negotiation
- Enable dynamic primal discovery and registration
- Provide unified primal abstraction layer

### Real-Time Primal Events
- Implement real-time primal event streaming
- Support bidirectional primal communication
- Enable primal operation progress tracking
- Provide distributed primal coordination

### Distributed Primal Management
- Implement distributed primal coordination
- Support cross-primal data sharing
- Enable automatic primal discovery and health monitoring
- Provide consistent primal access patterns

## Architecture

### Universal Primal Storage Manager
```rust
pub struct UniversalPrimalStorageManager {
    primal_registry: Arc<PrimalRegistry>,
    storage_coordinator: Arc<StorageCoordinator>,
    event_broadcaster: Arc<PrimalEventBroadcaster>,
    capability_negotiator: Arc<CapabilityNegotiator>,
    health_monitor: Arc<PrimalHealthMonitor>,
    discovery_engine: Arc<PrimalDiscoveryEngine>,
}

impl UniversalPrimalStorageManager {
    pub async fn new(config: UniversalPrimalConfig) -> Result<Self>;
    pub async fn start(&self) -> Result<()>;
    pub async fn register_primal(&self, primal: StoragePrimal) -> Result<()>;
    pub async fn discover_primals(&self) -> Result<Vec<AvailablePrimal>>;
    pub async fn coordinate_primal_request(&self, request: PrimalRequest) -> Result<PrimalResponse>;
    pub async fn stream_primal_events(&self) -> Result<PrimalEventStream>;
}
```

### Storage Primal Provider Interface
```rust
#[async_trait]
pub trait StoragePrimalProvider {
    async fn handle_request(&self, request: StoragePrimalRequest) -> Result<StoragePrimalResponse>;
    async fn stream_data(&self, request: StreamRequest) -> Result<DataStream>;
    async fn get_capabilities(&self) -> Result<Vec<StorageCapability>>;
    async fn get_health(&self) -> Result<PrimalHealth>;
    fn primal_info(&self) -> PrimalInfo;
}

#[derive(Debug, Clone)]
pub enum StorageCapability {
    ZfsPoolManagement,
    DatasetLifecycle,
    TieredStorage,
    SnapshotManagement,
    BackupReplication,
    PerformanceMonitoring,
    SecurityIntegration,
    AiDataOptimization,
    NetworkDistribution,
    ComputeIntegration,
}

#[derive(Debug, Clone)]
pub struct PrimalInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub primal_type: PrimalType,
    pub endpoints: Vec<String>,
    pub capabilities: Vec<StorageCapability>,
}
```

### Primal Discovery Engine
```rust
pub struct PrimalDiscoveryEngine {
    network_scanner: Arc<NetworkScanner>,
    environment_reader: Arc<EnvironmentReader>,
    config_loader: Arc<ConfigLoader>,
    service_registry: Arc<ServiceRegistry>,
    discovery_cache: Arc<RwLock<HashMap<String, DiscoveredPrimal>>>,
}

impl PrimalDiscoveryEngine {
    pub async fn discover_all_primals(&self) -> Result<Vec<AvailablePrimal>>;
    pub async fn discover_by_capability(&self, capability: StorageCapability) -> Result<Vec<AvailablePrimal>>;
    pub async fn register_primal(&self, primal: PrimalRegistration) -> Result<()>;
    pub async fn health_check_all(&self) -> Result<HashMap<String, PrimalHealth>>;
}

#[derive(Debug, Clone)]
pub struct AvailablePrimal {
    pub info: PrimalInfo,
    pub endpoint: String,
    pub health: PrimalHealth,
    pub capabilities: Vec<StorageCapability>,
    pub authentication: AuthenticationMethod,
}
```

### Capability Negotiation System
```rust
pub struct CapabilityNegotiator {
    capability_cache: Arc<RwLock<HashMap<String, Vec<StorageCapability>>>>,
    compatibility_matrix: Arc<CompatibilityMatrix>,
    version_resolver: Arc<VersionResolver>,
}

impl CapabilityNegotiator {
    pub async fn negotiate_capabilities(&self, primal_id: &str, required: Vec<StorageCapability>) -> Result<Vec<StorageCapability>>;
    pub async fn check_compatibility(&self, primal_a: &str, primal_b: &str) -> Result<bool>;
    pub async fn resolve_feature_conflicts(&self, primals: Vec<&str>) -> Result<ConflictResolution>;
    pub async fn get_optimal_primal(&self, requirement: StorageRequirement) -> Result<String>;
}

#[derive(Debug, Clone)]
pub enum ConflictResolution {
    UseFirst,
    UseBest,
    Merge,
    Fallback(String),
}
```

### Real-Time Primal Event System
```rust
pub struct PrimalEventBroadcaster {
    event_channels: HashMap<String, broadcast::Sender<PrimalEvent>>,
    subscription_manager: Arc<SubscriptionManager>,
    event_history: Arc<EventHistory>,
    cross_primal_events: Arc<CrossPrimalEventManager>,
}

#[derive(Debug, Clone)]
pub enum PrimalEvent {
    StorageCreated { primal_id: String, storage_id: String, details: StorageDetails },
    StorageModified { primal_id: String, storage_id: String, changes: Vec<Change> },
    StorageDeleted { primal_id: String, storage_id: String },
    PrimalRegistered { primal_info: PrimalInfo },
    PrimalUnregistered { primal_id: String },
    CapabilityChanged { primal_id: String, capabilities: Vec<StorageCapability> },
    HealthChanged { primal_id: String, health: PrimalHealth },
    CrossPrimalOperation { operation: CrossPrimalOperation, status: OperationStatus },
}
```

### Distributed Primal Coordination
```rust
pub struct StorageCoordinator {
    primal_registry: Arc<PrimalRegistry>,
    request_router: Arc<RequestRouter>,
    load_balancer: Arc<PrimalLoadBalancer>,
    consistency_manager: Arc<ConsistencyManager>,
    transaction_manager: Arc<TransactionManager>,
}

impl StorageCoordinator {
    pub async fn route_request(&self, request: StoragePrimalRequest) -> Result<StoragePrimalResponse>;
    pub async fn coordinate_multi_primal(&self, operation: MultiPrimalOperation) -> Result<OperationResult>;
    pub async fn ensure_consistency(&self, operation_id: &str) -> Result<ConsistencyStatus>;
    pub async fn manage_transaction(&self, transaction: PrimalTransaction) -> Result<TransactionResult>;
}
```

## Implementation Tasks

### Phase 1: Universal Primal Foundation
1. **Universal Primal Interface**
   - Implement StoragePrimalProvider trait
   - Create unified primal request/response types
   - Build primal capability discovery
   - Enable dynamic primal registration

2. **Primal Discovery System**
   - Implement network-based primal discovery
   - Create environment variable configuration
   - Build service registry integration
   - Add primal health monitoring

### Phase 2: Capability Negotiation
1. **Capability Management**
   - Implement capability-based negotiation
   - Create compatibility checking system
   - Build version resolution mechanisms
   - Enable feature conflict resolution

2. **Real-Time Integration**
   - Implement real-time primal event streaming
   - Create bidirectional primal communication
   - Build event subscription management
   - Enable cross-primal operation tracking

### Phase 3: Distributed Coordination
1. **Multi-Primal Operations**
   - Implement cross-primal data operations
   - Create distributed transaction management
   - Build consistency guarantee mechanisms
   - Enable atomic multi-primal operations

2. **Production Features**
   - Implement primal load balancing
   - Create fault tolerance mechanisms
   - Build monitoring and alerting
   - Enable performance optimization

## Universal Primal Types

### Storage Primal Request/Response
```rust
#[derive(Debug, Clone)]
pub enum StoragePrimalRequest {
    // ZFS Storage Operations
    CreatePool { name: String, devices: Vec<String>, config: PoolConfig },
    CreateDataset { pool: String, name: String, config: DatasetConfig },
    ManageSnapshots { dataset: String, policy: SnapshotPolicy },
    
    // Security Integration
    SecureStorage { request: SecureStorageRequest },
    
    // AI Data Management
    AiDataRequest { request: AiDataRequest },
    
    // Network Distribution
    NetworkStorage { request: NetworkStorageRequest },
    
    // Compute Integration
    ComputeStorage { request: ComputeStorageRequest },
    
    // Universal Operations
    GetCapabilities,
    GetHealth,
    GetMetrics,
}

#[derive(Debug, Clone)]
pub enum StoragePrimalResponse {
    // Operation Results
    Success { operation_id: String, result: OperationResult },
    Error { error: PrimalError },
    
    // Data Responses
    Capabilities { capabilities: Vec<StorageCapability> },
    Health { health: PrimalHealth },
    Metrics { metrics: StorageMetrics },
    
    // Streaming Responses
    Stream { stream_id: String },
    Progress { operation_id: String, progress: f64 },
}
```

### Primal Configuration
```rust
#[derive(Debug, Clone)]
pub struct UniversalPrimalConfig {
    pub core: CorePrimalConfig,
    pub storage: StoragePrimalConfig,
    pub primal_ecosystem: PrimalEcosystemConfig,
    pub integrations: HashMap<String, PrimalIntegrationConfig>,
}

#[derive(Debug, Clone)]
pub struct PrimalEcosystemConfig {
    pub auto_discovery: bool,
    pub discovery_timeout: u64,
    pub capability_cache_ttl: u64,
    pub health_check_interval: u64,
    pub discovery_methods: Vec<DiscoveryMethod>,
}

#[derive(Debug, Clone)]
pub enum DiscoveryMethod {
    NetworkScanning { ports: Vec<u16> },
    EnvironmentVariables { prefix: String },
    ConfigFiles { paths: Vec<String> },
    ServiceRegistry { endpoint: String },
}
```

## Security Integration

### Mutual TLS Authentication
```rust
pub struct PrimalAuthentication {
    pub mutual_tls: bool,
    pub cert_path: String,
    pub key_path: String,
    pub ca_cert_path: String,
    pub verify_peer: bool,
}

impl PrimalAuthentication {
    pub async fn authenticate_primal(&self, primal_id: &str) -> Result<AuthToken>;
    pub async fn verify_request(&self, request: &PrimalRequest) -> Result<bool>;
    pub async fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>>;
    pub async fn decrypt_data(&self, encrypted: &[u8]) -> Result<Vec<u8>>;
}
```

### Audit Logging
```rust
pub struct PrimalAuditLog {
    pub operation_id: String,
    pub primal_id: String,
    pub operation: PrimalOperation,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub metadata: HashMap<String, String>,
}

impl PrimalAuditLog {
    pub async fn log_operation(&self, operation: PrimalOperation) -> Result<()>;
    pub async fn log_security_event(&self, event: SecurityEvent) -> Result<()>;
    pub async fn log_access_attempt(&self, attempt: AccessAttempt) -> Result<()>;
}
```

## Performance Optimization

### Primal Load Balancing
```rust
pub struct PrimalLoadBalancer {
    pub strategies: HashMap<String, LoadBalancingStrategy>,
    pub health_weights: HashMap<String, f64>,
    pub capacity_limits: HashMap<String, u64>,
}

#[derive(Debug, Clone)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    CapabilityBased,
    Performance,
}
```

### Caching Strategy
```rust
pub struct PrimalCacheManager {
    pub capability_cache: Arc<RwLock<HashMap<String, Vec<StorageCapability>>>>,
    pub health_cache: Arc<RwLock<HashMap<String, PrimalHealth>>>,
    pub response_cache: Arc<RwLock<HashMap<String, CachedResponse>>>,
    pub cache_ttl: HashMap<CacheType, u64>,
}

impl PrimalCacheManager {
    pub async fn cache_capabilities(&self, primal_id: &str, capabilities: Vec<StorageCapability>) -> Result<()>;
    pub async fn get_cached_health(&self, primal_id: &str) -> Result<Option<PrimalHealth>>;
    pub async fn invalidate_cache(&self, primal_id: &str) -> Result<()>;
}
```

## Integration Examples

### ZFS Storage Primal
```rust
pub struct ZfsStoragePrimal {
    pub zfs_manager: Arc<ZfsManager>,
    pub pool_monitor: Arc<PoolMonitor>,
    pub dataset_manager: Arc<DatasetManager>,
    pub snapshot_manager: Arc<SnapshotManager>,
}

#[async_trait]
impl StoragePrimalProvider for ZfsStoragePrimal {
    async fn handle_request(&self, request: StoragePrimalRequest) -> Result<StoragePrimalResponse> {
        match request {
            StoragePrimalRequest::CreatePool { name, devices, config } => {
                let pool = self.zfs_manager.create_pool(&name, &devices, &config).await?;
                Ok(StoragePrimalResponse::Success { 
                    operation_id: pool.id.clone(), 
                    result: OperationResult::PoolCreated(pool) 
                })
            },
            StoragePrimalRequest::GetCapabilities => {
                Ok(StoragePrimalResponse::Capabilities { 
                    capabilities: vec![
                        StorageCapability::ZfsPoolManagement,
                        StorageCapability::DatasetLifecycle,
                        StorageCapability::TieredStorage,
                        StorageCapability::SnapshotManagement,
                    ]
                })
            },
            _ => Err(PrimalError::UnsupportedOperation),
        }
    }
    
    async fn get_capabilities(&self) -> Result<Vec<StorageCapability>> {
        Ok(vec![
            StorageCapability::ZfsPoolManagement,
            StorageCapability::DatasetLifecycle,
            StorageCapability::TieredStorage,
            StorageCapability::SnapshotManagement,
            StorageCapability::BackupReplication,
            StorageCapability::PerformanceMonitoring,
        ])
    }
    
    async fn get_health(&self) -> Result<PrimalHealth> {
        let pools = self.zfs_manager.get_all_pools().await?;
        let total_capacity = pools.iter().map(|p| p.capacity).sum();
        let used_capacity = pools.iter().map(|p| p.used).sum();
        
        Ok(PrimalHealth {
            status: HealthStatus::Healthy,
            uptime: std::time::SystemTime::now(),
            storage_capacity: total_capacity,
            storage_used: used_capacity,
            performance_metrics: self.get_performance_metrics().await?,
        })
    }
    
    fn primal_info(&self) -> PrimalInfo {
        PrimalInfo {
            id: "nestgate-zfs".to_string(),
            name: "NestGate ZFS Storage".to_string(),
            version: "1.0.0".to_string(),
            primal_type: PrimalType::Storage,
            endpoints: vec!["https://localhost:8080".to_string()],
            capabilities: vec![
                StorageCapability::ZfsPoolManagement,
                StorageCapability::DatasetLifecycle,
                StorageCapability::TieredStorage,
                StorageCapability::SnapshotManagement,
                StorageCapability::BackupReplication,
                StorageCapability::PerformanceMonitoring,
            ],
        }
    }
}
```

## Testing Strategy

### Primal Integration Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_primal_discovery() {
        let config = UniversalPrimalConfig::test_config();
        let manager = UniversalPrimalStorageManager::new(config).await.unwrap();
        
        let discovered = manager.discover_primals().await.unwrap();
        assert!(!discovered.is_empty());
        
        // Test capability negotiation
        let capabilities = manager.negotiate_capabilities("test-primal", vec![
            StorageCapability::ZfsPoolManagement,
            StorageCapability::DatasetLifecycle,
        ]).await.unwrap();
        
        assert!(capabilities.contains(&StorageCapability::ZfsPoolManagement));
    }
    
    #[tokio::test]
    async fn test_multi_primal_coordination() {
        let config = UniversalPrimalConfig::test_config();
        let manager = UniversalPrimalStorageManager::new(config).await.unwrap();
        
        // Register test primals
        let zfs_primal = MockZfsStoragePrimal::new();
        let security_primal = MockSecurityPrimal::new();
        
        manager.register_primal(zfs_primal).await.unwrap();
        manager.register_primal(security_primal).await.unwrap();
        
        // Test cross-primal operation
        let request = StoragePrimalRequest::SecureStorage { 
            request: SecureStorageRequest::CreateEncryptedDataset { 
                name: "test-dataset".to_string(),
                encryption: EncryptionConfig::default(),
            }
        };
        
        let response = manager.coordinate_primal_request(request).await.unwrap();
        assert!(matches!(response, StoragePrimalResponse::Success { .. }));
    }
}
```

---

This universal primal storage specification provides a complete framework for integrating any storage system into the NestGate ecosystem using the universal primal architecture patterns. The system is designed to be future-proof, allowing new primals to integrate seamlessly without requiring changes to existing code. 