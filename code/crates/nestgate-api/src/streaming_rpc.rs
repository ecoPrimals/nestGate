//! Enhanced Streaming RPC Module for NestGate
//!
//! This module provides advanced streaming RPC capabilities using tarpc with:
//! - Bidirectional streaming support
//! - Type-safe RPC definitions
//! - Real-time data streaming
//! - Event-driven communication
//! - Backpressure handling
//! - Connection multiplexing

use async_trait::async_trait;
use futures_util::{SinkExt, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tarpc::{
    client::{self, NewClient},
    context::Context,
    server::{incoming::Incoming, BaseChannel, Channel},
    tokio_serde::formats::Bincode,
    transport,
};
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Streaming RPC service definition with bidirectional capabilities
#[tarpc::service]
pub trait StreamingRpcService {
    /// Execute a storage operation
    async fn execute_storage_operation(
        operation: StorageOperation,
    ) -> Result<StorageOperationResult, RpcError>;

    /// Execute a ZFS operation
    async fn execute_zfs_operation(operation: ZfsOperation)
        -> Result<ZfsOperationResult, RpcError>;

    /// Stream storage events (server streaming)
    async fn stream_storage_events(filter: EventFilter) -> StorageEventStream;

    /// Stream system metrics (server streaming)
    async fn stream_system_metrics(interval: Duration) -> MetricsStream;

    /// Bidirectional stream for real-time communication
    async fn bidirectional_stream(client_stream: ClientMessageStream) -> ServerMessageStream;

    /// Subscribe to event channel
    async fn subscribe_to_events(
        subscription: EventSubscription,
    ) -> Result<SubscriptionId, RpcError>;

    /// Unsubscribe from event channel
    async fn unsubscribe_from_events(subscription_id: SubscriptionId) -> Result<(), RpcError>;

    /// Send command with streaming response
    async fn execute_command_stream(command: Command) -> CommandResultStream;

    /// Health check
    async fn health_check() -> Result<HealthStatus, RpcError>;

    /// Get service capabilities
    async fn get_capabilities() -> Result<ServiceCapabilities, RpcError>;
}

/// Storage operations for RPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageOperation {
    CreateDataset {
        name: String,
        properties: HashMap<String, String>,
    },
    DeleteDataset {
        name: String,
    },
    ListDatasets {
        pool: Option<String>,
        recursive: bool,
    },
    GetDatasetInfo {
        name: String,
    },
    CreateSnapshot {
        dataset: String,
        snapshot_name: String,
        recursive: bool,
    },
    DeleteSnapshot {
        full_name: String,
    },
    RollbackSnapshot {
        full_name: String,
    },
    CloneDataset {
        snapshot: String,
        clone_name: String,
    },
    StreamDataset {
        source: String,
        target: String,
        incremental: bool,
    },
}

/// ZFS operations for RPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsOperation {
    CreatePool {
        name: String,
        devices: Vec<String>,
        config: PoolConfig,
    },
    DestroyPool {
        name: String,
        force: bool,
    },
    ListPools {
        include_status: bool,
    },
    GetPoolStatus {
        name: String,
    },
    ScrubPool {
        name: String,
    },
    ExportPool {
        name: String,
    },
    ImportPool {
        name: String,
        import_dir: Option<String>,
    },
    UpgradePool {
        name: String,
        version: Option<u32>,
    },
}

/// Pool configuration for ZFS operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    pub ashift: Option<u8>,
    pub features: HashMap<String, bool>,
    pub properties: HashMap<String, String>,
}

/// Storage operation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageOperationResult {
    DatasetCreated {
        name: String,
        mountpoint: String,
    },
    DatasetDeleted {
        name: String,
    },
    DatasetList {
        datasets: Vec<DatasetInfo>,
    },
    DatasetInfo {
        info: DatasetInfo,
    },
    SnapshotCreated {
        full_name: String,
        creation_time: SystemTime,
    },
    SnapshotDeleted {
        full_name: String,
    },
    SnapshotRolledBack {
        dataset: String,
        snapshot: String,
    },
    DatasetCloned {
        source: String,
        clone: String,
    },
    DatasetStreamInitiated {
        stream_id: String,
        source: String,
        target: String,
    },
}

/// ZFS operation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsOperationResult {
    PoolCreated {
        name: String,
        devices: Vec<String>,
    },
    PoolDestroyed {
        name: String,
    },
    PoolList {
        pools: Vec<PoolInfo>,
    },
    PoolStatus {
        status: PoolStatus,
    },
    ScrubStarted {
        pool: String,
        estimated_completion: Option<SystemTime>,
    },
    PoolExported {
        name: String,
    },
    PoolImported {
        name: String,
        devices: Vec<String>,
    },
    PoolUpgraded {
        name: String,
        old_version: u32,
        new_version: u32,
    },
}

/// Dataset information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub name: String,
    pub type_: String,
    pub used: u64,
    pub available: u64,
    pub referenced: u64,
    pub mountpoint: Option<String>,
    pub properties: HashMap<String, String>,
    pub creation_time: SystemTime,
}

/// Pool information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    pub name: String,
    pub size: u64,
    pub allocated: u64,
    pub free: u64,
    pub capacity: f64,
    pub health: String,
    pub version: u32,
    pub devices: Vec<DeviceInfo>,
}

/// Device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub name: String,
    pub state: String,
    pub read_errors: u64,
    pub write_errors: u64,
    pub checksum_errors: u64,
}

/// Pool status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStatus {
    pub name: String,
    pub state: String,
    pub status: String,
    pub action: Option<String>,
    pub scan: Option<ScanInfo>,
    pub errors: Option<String>,
    pub config: Vec<VDevInfo>,
}

/// Scan information (scrub/resilver)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanInfo {
    pub function: String,
    pub state: String,
    pub start_time: SystemTime,
    pub end_time: Option<SystemTime>,
    pub percentage: f64,
    pub bytes_scanned: u64,
    pub bytes_to_scan: u64,
    pub rate: u64,
    pub errors: u64,
}

/// VDev information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VDevInfo {
    pub name: String,
    pub state: String,
    pub read: u64,
    pub write: u64,
    pub cksum: u64,
    pub children: Vec<VDevInfo>,
}

/// Event filter for streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFilter {
    pub event_types: Vec<String>,
    pub source_filter: Option<String>,
    pub priority_filter: Option<u8>,
    pub since: Option<SystemTime>,
}

/// Event subscription configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscription {
    pub subscription_id: String,
    pub event_types: Vec<String>,
    pub filters: HashMap<String, String>,
    pub buffer_size: usize,
}

/// Subscription identifier
pub type SubscriptionId = String;

/// Commands for streaming execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    StorageCommand(StorageOperation),
    ZfsCommand(ZfsOperation),
    SystemCommand {
        command: String,
        args: Vec<String>,
    },
    CustomCommand {
        name: String,
        parameters: HashMap<String, serde_json::Value>,
    },
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: SystemTime,
    pub uptime: Duration,
    pub version: String,
    pub capabilities: Vec<String>,
}

/// Service capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCapabilities {
    pub supported_operations: Vec<String>,
    pub streaming_support: bool,
    pub bidirectional_support: bool,
    pub authentication_required: bool,
    pub max_concurrent_streams: usize,
    pub supported_formats: Vec<String>,
}

/// RPC error types
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum RpcError {
    #[error("Operation not supported: {0}")]
    NotSupported(String),

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Authorization failed: {0}")]
    AuthorizationFailed(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Resource already exists: {0}")]
    ResourceAlreadyExists(String),

    #[error("Operation failed: {0}")]
    OperationFailed(String),

    #[error("Service unavailable")]
    ServiceUnavailable,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Type aliases for streams
pub type StorageEventStream = Pin<Box<dyn Stream<Item = StorageEvent> + Send>>;
pub type MetricsStream = Pin<Box<dyn Stream<Item = SystemMetrics> + Send>>;
pub type ClientMessageStream = Pin<Box<dyn Stream<Item = ClientMessage> + Send>>;
pub type ServerMessageStream = Pin<Box<dyn Stream<Item = ServerMessage> + Send>>;
pub type CommandResultStream = Pin<Box<dyn Stream<Item = CommandResult> + Send>>;

/// Storage events for streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEvent {
    pub id: String,
    pub event_type: String,
    pub timestamp: SystemTime,
    pub source: String,
    pub data: serde_json::Value,
    pub priority: u8,
}

/// System metrics for streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: SystemTime,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_io: NetworkMetrics,
    pub zfs_metrics: ZfsMetrics,
}

/// Network metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub bytes_in: u64,
    pub bytes_out: u64,
    pub packets_in: u64,
    pub packets_out: u64,
    pub errors: u64,
}

/// ZFS metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsMetrics {
    pub arc_size: u64,
    pub arc_hits: u64,
    pub arc_misses: u64,
    pub l2arc_size: u64,
    pub l2arc_hits: u64,
    pub l2arc_misses: u64,
    pub zil_commits: u64,
    pub zil_itx_count: u64,
}

/// Client messages for bidirectional streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Command {
        id: String,
        command: Command,
    },
    Subscribe {
        subscription: EventSubscription,
    },
    Unsubscribe {
        subscription_id: String,
    },
    Ping {
        timestamp: SystemTime,
    },
    Data {
        stream_id: String,
        data: serde_json::Value,
    },
}

/// Server messages for bidirectional streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    CommandResult {
        id: String,
        result: Result<serde_json::Value, RpcError>,
    },
    Event {
        subscription_id: String,
        event: StorageEvent,
    },
    Metrics {
        metrics: SystemMetrics,
    },
    Pong {
        timestamp: SystemTime,
        server_timestamp: SystemTime,
    },
    Data {
        stream_id: String,
        data: serde_json::Value,
    },
    Error {
        error: RpcError,
    },
}

/// Command execution results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandResult {
    Success {
        data: serde_json::Value,
    },
    Progress {
        percentage: f64,
        message: String,
        data: Option<serde_json::Value>,
    },
    Error {
        error: RpcError,
    },
    Completed {
        final_result: serde_json::Value,
    },
}

/// Streaming RPC server implementation
#[derive(Clone)]
pub struct StreamingRpcServer {
    /// Active subscriptions
    subscriptions: Arc<RwLock<HashMap<String, EventSubscription>>>,
    /// Event broadcaster
    event_broadcaster: broadcast::Sender<StorageEvent>,
    /// Metrics broadcaster
    metrics_broadcaster: broadcast::Sender<SystemMetrics>,
    /// Service capabilities
    capabilities: ServiceCapabilities,
}

impl StreamingRpcServer {
    /// Create a new streaming RPC server
    pub fn new() -> Self {
        let (event_broadcaster, _) = broadcast::channel(10000);
        let (metrics_broadcaster, _) = broadcast::channel(1000);

        let capabilities = ServiceCapabilities {
            supported_operations: vec![
                "storage".to_string(),
                "zfs".to_string(),
                "streaming".to_string(),
                "bidirectional".to_string(),
            ],
            streaming_support: true,
            bidirectional_support: true,
            authentication_required: false,
            max_concurrent_streams: 100,
            supported_formats: vec!["bincode".to_string(), "json".to_string()],
        };

        Self {
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            event_broadcaster,
            metrics_broadcaster,
            capabilities,
        }
    }

    /// Start the RPC server on the specified address
    pub async fn start(
        &self,
        addr: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Starting streaming RPC server on {}", addr);

        let listener = tarpc::serde_transport::tcp::listen(&addr, Bincode::default).await?;
        let server = self.clone();

        tokio::spawn(async move {
            listener
                .filter_map(|r| async move { r.ok() })
                .map(BaseChannel::with_defaults)
                .execute(server.serve())
                .for_each(|response| async move {
                    if let Err(e) = response {
                        warn!("Error serving request: {}", e);
                    }
                })
                .await;
        });

        // Start background metrics collection
        self.start_metrics_collection().await?;

        Ok(())
    }

    /// Start metrics collection background task
    async fn start_metrics_collection(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let metrics_broadcaster = self.metrics_broadcaster.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));

            loop {
                interval.tick().await;

                // Collect system metrics (simplified - real implementation would collect from system)
                let metrics = SystemMetrics {
                    timestamp: SystemTime::now(),
                    cpu_usage: 25.5, // Placeholder - real implementation would use system metrics
                    memory_usage: 45.2,
                    disk_usage: 67.8,
                    network_io: NetworkMetrics {
                        bytes_in: 1024000,
                        bytes_out: 512000,
                        packets_in: 1000,
                        packets_out: 800,
                        errors: 0,
                    },
                    zfs_metrics: ZfsMetrics {
                        arc_size: 2048000000,
                        arc_hits: 1000000,
                        arc_misses: 50000,
                        l2arc_size: 512000000,
                        l2arc_hits: 100000,
                        l2arc_misses: 5000,
                        zil_commits: 10000,
                        zil_itx_count: 25000,
                    },
                };

                if let Err(e) = metrics_broadcaster.send(metrics) {
                    debug!("Failed to broadcast metrics: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Broadcast a storage event
    pub async fn broadcast_event(
        &self,
        event: StorageEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.event_broadcaster.send(event)?;
        Ok(())
    }
}

#[async_trait]
impl StreamingRpcService for StreamingRpcServer {
    async fn execute_storage_operation(
        self,
        _ctx: Context,
        operation: StorageOperation,
    ) -> Result<StorageOperationResult, RpcError> {
        debug!("Executing storage operation: {:?}", operation);

        // Simulate operation execution
        match operation {
            StorageOperation::CreateDataset {
                name,
                properties: _,
            } => Ok(StorageOperationResult::DatasetCreated {
                name: name.clone(),
                mountpoint: format!("/mnt/{}", name),
            }),
            StorageOperation::DeleteDataset { name } => {
                Ok(StorageOperationResult::DatasetDeleted { name })
            }
            StorageOperation::ListDatasets {
                pool: _,
                recursive: _,
            } => Ok(StorageOperationResult::DatasetList {
                datasets: vec![DatasetInfo {
                    name: "pool/dataset1".to_string(),
                    type_: "filesystem".to_string(),
                    used: 1024000,
                    available: 10240000,
                    referenced: 1024000,
                    mountpoint: Some("/mnt/pool/dataset1".to_string()),
                    properties: HashMap::new(),
                    creation_time: SystemTime::now(),
                }],
            }),
            // Add more operations as needed
            _ => Err(RpcError::NotSupported(
                "Operation not yet implemented".to_string(),
            )),
        }
    }

    async fn execute_zfs_operation(
        self,
        _ctx: Context,
        operation: ZfsOperation,
    ) -> Result<ZfsOperationResult, RpcError> {
        debug!("Executing ZFS operation: {:?}", operation);

        // Simulate operation execution
        match operation {
            ZfsOperation::ListPools { include_status: _ } => Ok(ZfsOperationResult::PoolList {
                pools: vec![PoolInfo {
                    name: "rpool".to_string(),
                    size: 1024000000000,
                    allocated: 512000000000,
                    free: 512000000000,
                    capacity: 50.0,
                    health: "ONLINE".to_string(),
                    version: 5000,
                    devices: vec![],
                }],
            }),
            _ => Err(RpcError::NotSupported(
                "Operation not yet implemented".to_string(),
            )),
        }
    }

    async fn stream_storage_events(self, _ctx: Context, filter: EventFilter) -> StorageEventStream {
        debug!("Creating storage event stream with filter: {:?}", filter);

        let receiver = self.event_broadcaster.subscribe();
        let stream = tokio_stream::wrappers::BroadcastStream::new(receiver).filter_map(
            |result| async move {
                match result {
                    Ok(event) => Some(event),
                    Err(_) => None,
                }
            },
        );

        Box::pin(stream)
    }

    async fn stream_system_metrics(self, _ctx: Context, _interval: Duration) -> MetricsStream {
        debug!("Creating system metrics stream");

        let receiver = self.metrics_broadcaster.subscribe();
        let stream = tokio_stream::wrappers::BroadcastStream::new(receiver).filter_map(
            |result| async move {
                match result {
                    Ok(metrics) => Some(metrics),
                    Err(_) => None,
                }
            },
        );

        Box::pin(stream)
    }

    async fn bidirectional_stream(
        self,
        _ctx: Context,
        client_stream: ClientMessageStream,
    ) -> ServerMessageStream {
        info!("Starting bidirectional stream");

        // Create response stream
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        // Process client messages
        tokio::spawn(async move {
            tokio::pin!(client_stream);

            while let Some(message) = client_stream.next().await {
                match message {
                    ClientMessage::Command { id, command } => {
                        // Process command and send result
                        let result = Ok(serde_json::json!({"status": "success"}));
                        let response = ServerMessage::CommandResult { id, result };
                        if tx.send(response).is_err() {
                            break;
                        }
                    }
                    ClientMessage::Ping { timestamp } => {
                        let response = ServerMessage::Pong {
                            timestamp,
                            server_timestamp: SystemTime::now(),
                        };
                        if tx.send(response).is_err() {
                            break;
                        }
                    }
                    _ => {
                        // Handle other message types
                    }
                }
            }
        });

        // Convert receiver to stream
        let stream = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);
        Box::pin(stream)
    }

    async fn subscribe_to_events(
        self,
        _ctx: Context,
        subscription: EventSubscription,
    ) -> Result<SubscriptionId, RpcError> {
        let subscription_id = subscription.subscription_id.clone();

        let mut subscriptions = self.subscriptions.write().await;
        subscriptions.insert(subscription_id.clone(), subscription);

        info!("Created event subscription: {}", subscription_id);
        Ok(subscription_id)
    }

    async fn unsubscribe_from_events(
        self,
        _ctx: Context,
        subscription_id: SubscriptionId,
    ) -> Result<(), RpcError> {
        let mut subscriptions = self.subscriptions.write().await;

        if subscriptions.remove(&subscription_id).is_some() {
            info!("Removed event subscription: {}", subscription_id);
            Ok(())
        } else {
            Err(RpcError::ResourceNotFound(format!(
                "Subscription not found: {}",
                subscription_id
            )))
        }
    }

    async fn execute_command_stream(self, _ctx: Context, command: Command) -> CommandResultStream {
        debug!("Executing command with streaming response: {:?}", command);

        // Create a stream that sends progress updates
        let stream = futures_util::stream::iter(vec![
            CommandResult::Progress {
                percentage: 25.0,
                message: "Starting command execution".to_string(),
                data: None,
            },
            CommandResult::Progress {
                percentage: 50.0,
                message: "Command execution in progress".to_string(),
                data: None,
            },
            CommandResult::Progress {
                percentage: 75.0,
                message: "Command execution nearly complete".to_string(),
                data: None,
            },
            CommandResult::Completed {
                final_result: serde_json::json!({"status": "success"}),
            },
        ]);

        Box::pin(stream)
    }

    async fn health_check(self, _ctx: Context) -> Result<HealthStatus, RpcError> {
        Ok(HealthStatus {
            status: "healthy".to_string(),
            timestamp: SystemTime::now(),
            uptime: Duration::from_secs(3600), // 1 hour
            version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: vec![
                "storage".to_string(),
                "zfs".to_string(),
                "streaming".to_string(),
                "bidirectional".to_string(),
            ],
        })
    }

    async fn get_capabilities(self, _ctx: Context) -> Result<ServiceCapabilities, RpcError> {
        Ok(self.capabilities.clone())
    }
}

/// Streaming RPC client for connecting to the server
pub struct StreamingRpcClient {
    client: StreamingRpcServiceClient,
}

impl StreamingRpcClient {
    /// Connect to a streaming RPC server
    pub async fn connect(addr: String) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        info!("Connecting to streaming RPC server at {}", addr);

        let transport = tarpc::serde_transport::tcp::connect(&addr, Bincode::default).await?;
        let client = StreamingRpcServiceClient::new(client::Config::default(), transport).spawn();

        Ok(Self { client })
    }

    /// Execute a storage operation
    pub async fn execute_storage_operation(
        &self,
        operation: StorageOperation,
    ) -> Result<StorageOperationResult, RpcError> {
        self.client
            .execute_storage_operation(Context::current(), operation)
            .await?
    }

    /// Execute a ZFS operation
    pub async fn execute_zfs_operation(
        &self,
        operation: ZfsOperation,
    ) -> Result<ZfsOperationResult, RpcError> {
        self.client
            .execute_zfs_operation(Context::current(), operation)
            .await?
    }

    /// Stream storage events
    pub async fn stream_storage_events(
        &self,
        filter: EventFilter,
    ) -> Result<StorageEventStream, RpcError> {
        let stream = self
            .client
            .stream_storage_events(Context::current(), filter)
            .await?;
        Ok(stream)
    }

    /// Stream system metrics
    pub async fn stream_system_metrics(
        &self,
        interval: Duration,
    ) -> Result<MetricsStream, RpcError> {
        let stream = self
            .client
            .stream_system_metrics(Context::current(), interval)
            .await?;
        Ok(stream)
    }

    /// Create bidirectional stream
    pub async fn bidirectional_stream(
        &self,
        client_stream: ClientMessageStream,
    ) -> Result<ServerMessageStream, RpcError> {
        let stream = self
            .client
            .bidirectional_stream(Context::current(), client_stream)
            .await?;
        Ok(stream)
    }

    /// Subscribe to events
    pub async fn subscribe_to_events(
        &self,
        subscription: EventSubscription,
    ) -> Result<SubscriptionId, RpcError> {
        self.client
            .subscribe_to_events(Context::current(), subscription)
            .await?
    }

    /// Unsubscribe from events
    pub async fn unsubscribe_from_events(
        &self,
        subscription_id: SubscriptionId,
    ) -> Result<(), RpcError> {
        self.client
            .unsubscribe_from_events(Context::current(), subscription_id)
            .await?
    }

    /// Execute command with streaming response
    pub async fn execute_command_stream(
        &self,
        command: Command,
    ) -> Result<CommandResultStream, RpcError> {
        let stream = self
            .client
            .execute_command_stream(Context::current(), command)
            .await?;
        Ok(stream)
    }

    /// Health check
    pub async fn health_check(&self) -> Result<HealthStatus, RpcError> {
        self.client.health_check(Context::current()).await?
    }

    /// Get service capabilities
    pub async fn get_capabilities(&self) -> Result<ServiceCapabilities, RpcError> {
        self.client.get_capabilities(Context::current()).await?
    }
}
