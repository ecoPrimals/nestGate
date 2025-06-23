use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore};
use tokio::net::TcpStream;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error, debug};
use uuid::Uuid;
use dashmap::DashMap;
use tokio_tungstenite::{WebSocketStream, tungstenite::Message as TungsteniteMessage};
use futures_util::{SinkExt, StreamExt};

use nestgate_core::Result;

/// High-performance inter-tower communication manager
#[derive(Debug)]
pub struct InterTowerCommunicationManager {
    pub local_tower_id: String,
    pub connection_pools: Arc<DashMap<String, Arc<TowerConnectionPool>>>,
    pub message_router: Arc<MessageRouter>,
    pub discovery_service: Arc<TowerDiscoveryService>,
    pub security_manager: Arc<CommunicationSecurityManager>,
    pub metrics: Arc<CommunicationMetrics>,
}

impl InterTowerCommunicationManager {
    pub fn new(tower_id: String, bind_port: u16) -> Self {
        Self {
            local_tower_id: tower_id.clone(),
            connection_pools: Arc::new(DashMap::new()),
            message_router: Arc::new(MessageRouter::new(tower_id.clone())),
            discovery_service: Arc::new(TowerDiscoveryService::new(tower_id.clone(), bind_port)),
            security_manager: Arc::new(CommunicationSecurityManager::new()),
            metrics: Arc::new(CommunicationMetrics::default()),
        }
    }
    
    /// Start inter-tower communication services
    pub async fn start(&self) -> Result<Vec<tokio::task::JoinHandle<()>>> {
        info!("Starting inter-tower communication for tower: {}", self.local_tower_id);
        
        let mut handles = Vec::new();
        
        // Start discovery service
        let discovery_handle = self.discovery_service.start().await?;
        handles.push(discovery_handle);
        
        // Start message router
        let router_handle = self.message_router.start().await?;
        handles.push(router_handle);
        
        // Start connection pool manager
        let pool_manager_handle = self.start_connection_pool_manager().await?;
        handles.push(pool_manager_handle);
        
        // Start metrics collector
        let metrics_handle = self.start_metrics_collector().await?;
        handles.push(metrics_handle);
        
        info!("Inter-tower communication services started");
        Ok(handles)
    }
    
    /// Send message to another tower with automatic connection management
    pub async fn send_to_tower(&self, target_tower_id: &str, message: TowerMessage) -> Result<TowerMessageResponse> {
        let start_time = Instant::now();
        self.metrics.messages_sent.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        // Get or create connection pool for target tower
        let pool = self.get_or_create_connection_pool(target_tower_id).await?;
        
        // Send message through pool
        let response = pool.send_message(message).await?;
        
        // Update metrics
        let duration = start_time.elapsed();
        self.metrics.update_response_time(duration);
        
        Ok(response)
    }
    
    /// Broadcast message to all connected towers
    pub async fn broadcast(&self, message: TowerMessage) -> Result<Vec<(String, Result<TowerMessageResponse>)>> {
        let mut results = Vec::new();
        let towers: Vec<String> = self.connection_pools.iter()
            .map(|entry| entry.key().clone())
            .collect();
        
        // Send to all towers concurrently
        let tasks: Vec<_> = towers.into_iter()
            .map(|tower_id| {
                let message = message.clone();
                let comm_manager = self.clone();
                tokio::spawn(async move {
                    let result = comm_manager.send_to_tower(&tower_id, message).await;
                    (tower_id, result)
                })
            })
            .collect();
        
        for task in tasks {
            if let Ok(result) = task.await {
                results.push(result);
            }
        }
        
        Ok(results)
    }
    
    /// Get or create connection pool for a tower
    async fn get_or_create_connection_pool(&self, tower_id: &str) -> Result<Arc<TowerConnectionPool>> {
        if let Some(pool) = self.connection_pools.get(tower_id) {
            return Ok(pool.clone());
        }
        
        // Discover tower endpoint
        let tower_info = self.discovery_service.get_tower_info(tower_id).await
            .ok_or_else(|| nestgate_core::NestGateError::NotFound(format!("Tower {} not found", tower_id)))?;
        
        // Create new connection pool
        let pool = Arc::new(TowerConnectionPool::new(
            tower_id.to_string(),
            tower_info.endpoint,
            self.security_manager.clone(),
        ));
        
        // Initialize pool
        pool.initialize().await?;
        
        self.connection_pools.insert(tower_id.to_string(), pool.clone());
        info!("Created connection pool for tower: {}", tower_id);
        
        Ok(pool)
    }
    
    /// Start connection pool manager background task
    async fn start_connection_pool_manager(&self) -> Result<tokio::task::JoinHandle<()>> {
        let pools = self.connection_pools.clone();
        let metrics = self.metrics.clone();
        
        Ok(tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                // Health check and cleanup for all pools
                for pool_entry in pools.iter() {
                    let pool = pool_entry.value();
                    
                    // Health check
                    if let Err(e) = pool.health_check().await {
                        warn!("Health check failed for tower {}: {}", pool_entry.key(), e);
                        
                        // Attempt to recover
                        if let Err(e) = pool.recover().await {
                            error!("Failed to recover connection pool for tower {}: {}", pool_entry.key(), e);
                        }
                    }
                    
                    // Update metrics
                    metrics.active_connections.store(
                        pool.active_connections() as u64,
                        std::sync::atomic::Ordering::Relaxed
                    );
                }
            }
        }))
    }
    
    /// Start metrics collection
    async fn start_metrics_collector(&self) -> Result<tokio::task::JoinHandle<()>> {
        let metrics = self.metrics.clone();
        
        Ok(tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                metrics.collect_and_log().await;
            }
        }))
    }
}

impl Clone for InterTowerCommunicationManager {
    fn clone(&self) -> Self {
        Self {
            local_tower_id: self.local_tower_id.clone(),
            connection_pools: self.connection_pools.clone(),
            message_router: self.message_router.clone(),
            discovery_service: self.discovery_service.clone(),
            security_manager: self.security_manager.clone(),
            metrics: self.metrics.clone(),
        }
    }
}

/// High-performance connection pool for a specific tower
#[derive(Debug)]
pub struct TowerConnectionPool {
    pub tower_id: String,
    pub endpoint: String,
    pub connections: Arc<RwLock<Vec<Arc<TowerConnection>>>>,
    pub available_connections: Arc<Semaphore>,
    pub security_manager: Arc<CommunicationSecurityManager>,
    pub pool_config: ConnectionPoolConfig,
    pub metrics: Arc<PoolMetrics>,
}

#[derive(Debug, Clone)]
pub struct ConnectionPoolConfig {
    pub min_connections: usize,
    pub max_connections: usize,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_retries: u32,
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 2,
            max_connections: 20,
            connection_timeout: Duration::from_secs(10),
            idle_timeout: Duration::from_secs(300),
            max_retries: 3,
        }
    }
}

impl TowerConnectionPool {
    pub fn new(tower_id: String, endpoint: String, security_manager: Arc<CommunicationSecurityManager>) -> Self {
        let config = ConnectionPoolConfig::default();
        
        Self {
            tower_id,
            endpoint,
            connections: Arc::new(RwLock::new(Vec::new())),
            available_connections: Arc::new(Semaphore::new(config.max_connections)),
            security_manager,
            pool_config: config,
            metrics: Arc::new(PoolMetrics::default()),
        }
    }
    
    /// Initialize the connection pool
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing connection pool for tower: {}", self.tower_id);
        
        // Create minimum number of connections
        for i in 0..self.pool_config.min_connections {
            match self.create_connection().await {
                Ok(connection) => {
                    self.connections.write().await.push(Arc::new(connection));
                    debug!("Created initial connection {} for tower: {}", i, self.tower_id);
                }
                Err(e) => {
                    warn!("Failed to create initial connection {} for tower {}: {}", i, self.tower_id, e);
                }
            }
        }
        
        info!("Connection pool initialized for tower: {}", self.tower_id);
        Ok(())
    }
    
    /// Send message through the pool
    pub async fn send_message(&self, message: TowerMessage) -> Result<TowerMessageResponse> {
        // Acquire connection permit
        let _permit = self.available_connections.acquire().await
            .map_err(|e| nestgate_core::NestGateError::Internal(format!("Failed to acquire connection: {}", e)))?;
        
        // Get or create connection
        let connection = self.get_or_create_connection().await?;
        
        // Send message with retries
        let mut last_error = None;
        for attempt in 0..self.pool_config.max_retries {
            match connection.send_message(message.clone()).await {
                Ok(response) => {
                    self.metrics.successful_messages.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    return Ok(response);
                }
                Err(e) => {
                    warn!("Message send attempt {} failed for tower {}: {}", attempt + 1, self.tower_id, e);
                    last_error = Some(e);
                    
                    // Mark connection as potentially bad
                    connection.mark_potentially_bad().await;
                    
                    // Short delay before retry
                    tokio::time::sleep(Duration::from_millis(100 * (attempt + 1) as u64)).await;
                }
            }
        }
        
        self.metrics.failed_messages.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Err(last_error.unwrap_or_else(|| 
            nestgate_core::NestGateError::Internal("Max retries exceeded".to_string())
        ))
    }
    
    /// Get or create a connection
    async fn get_or_create_connection(&self) -> Result<Arc<TowerConnection>> {
        // First, try to get a healthy existing connection
        {
            let connections = self.connections.read().await;
            for connection in connections.iter() {
                if connection.is_healthy().await {
                    return Ok(connection.clone());
                }
            }
        }
        
        // No healthy connections, create a new one
        let new_connection = Arc::new(self.create_connection().await?);
        
        // Add to pool if we have space
        {
            let mut connections = self.connections.write().await;
            if connections.len() < self.pool_config.max_connections {
                connections.push(new_connection.clone());
            }
        }
        
        Ok(new_connection)
    }
    
    /// Create a new connection
    async fn create_connection(&self) -> Result<TowerConnection> {
        debug!("Creating new connection to tower: {} at {}", self.tower_id, self.endpoint);
        
        let connection = TowerConnection::new(
            self.tower_id.clone(),
            self.endpoint.clone(),
            self.security_manager.clone(),
        );
        
        connection.connect().await?;
        Ok(connection)
    }
    
    /// Health check for the pool
    pub async fn health_check(&self) -> Result<()> {
        let connections = self.connections.read().await;
        let healthy_count = connections.iter()
            .map(|conn| async { conn.is_healthy().await })
            .collect::<futures_util::stream::FuturesUnordered<_>>()
            .fold(0usize, |acc, is_healthy| async move {
                if is_healthy { acc + 1 } else { acc }
            })
            .await;
        
        if healthy_count == 0 && !connections.is_empty() {
            return Err(nestgate_core::NestGateError::Internal("No healthy connections".to_string()));
        }
        
        Ok(())
    }
    
    /// Recover the connection pool
    pub async fn recover(&self) -> Result<()> {
        info!("Recovering connection pool for tower: {}", self.tower_id);
        
        // Clear unhealthy connections
        {
            let mut connections = self.connections.write().await;
            let mut healthy_connections = Vec::new();
            
            for conn in connections.iter() {
                if conn.is_healthy().await {
                    healthy_connections.push(conn.clone());
                }
            }
            
            *connections = healthy_connections;
        }
        
        // Recreate minimum connections
        self.initialize().await?;
        
        Ok(())
    }
    
    /// Get number of active connections
    pub fn active_connections(&self) -> usize {
        // This is an approximation since we can't easily await in a sync context
        self.pool_config.max_connections - self.available_connections.available_permits()
    }
}

/// Individual connection to a tower
#[derive(Debug)]
pub struct TowerConnection {
    pub tower_id: String,
    pub endpoint: String,
    pub websocket: Arc<RwLock<Option<WebSocketStream<TcpStream>>>>,
    pub security_manager: Arc<CommunicationSecurityManager>,
    pub last_used: Arc<RwLock<Instant>>,
    pub is_healthy: Arc<std::sync::atomic::AtomicBool>,
    pub connection_id: String,
}

impl TowerConnection {
    pub fn new(tower_id: String, endpoint: String, security_manager: Arc<CommunicationSecurityManager>) -> Self {
        Self {
            tower_id,
            endpoint,
            websocket: Arc::new(RwLock::new(None)),
            security_manager,
            last_used: Arc::new(RwLock::new(Instant::now())),
            is_healthy: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            connection_id: Uuid::new_v4().to_string(),
        }
    }
    
    /// Connect to the tower
    pub async fn connect(&self) -> Result<()> {
        debug!("Connecting to tower: {} at {}", self.tower_id, self.endpoint);
        
        // Parse endpoint and create WebSocket URL
        let ws_url = format!("ws://{}/ws/tower", self.endpoint);
        
        // Connect with timeout
        let stream = tokio::time::timeout(
            Duration::from_secs(10),
            TcpStream::connect(&self.endpoint)
        ).await
            .map_err(|_| nestgate_core::NestGateError::Timeout("Connection timeout".to_string()))?
            .map_err(|e| nestgate_core::NestGateError::Network(format!("Connection failed: {}", e)))?;
        
        // Upgrade to WebSocket
        let (ws_stream, _) = tokio_tungstenite::client_async(&ws_url, stream).await
            .map_err(|e| nestgate_core::NestGateError::Network(format!("WebSocket upgrade failed: {}", e)))?;
        
        // Store connection
        *self.websocket.write().await = Some(ws_stream);
        self.is_healthy.store(true, std::sync::atomic::Ordering::Relaxed);
        *self.last_used.write().await = Instant::now();
        
        info!("Connected to tower: {} (connection: {})", self.tower_id, self.connection_id);
        Ok(())
    }
    
    /// Send message through this connection
    pub async fn send_message(&self, message: TowerMessage) -> Result<TowerMessageResponse> {
        // Update last used time
        *self.last_used.write().await = Instant::now();
        
        // Serialize message
        let message_json = serde_json::to_string(&message)
            .map_err(|e| nestgate_core::NestGateError::Serialization(format!("Failed to serialize message: {}", e)))?;
        
        // Encrypt if needed
        let encrypted_message = self.security_manager.encrypt_message(&message_json).await?;
        
        // Send through WebSocket
        {
            let mut ws_guard = self.websocket.write().await;
            if let Some(ws) = ws_guard.as_mut() {
                ws.send(TungsteniteMessage::Text(encrypted_message)).await
                    .map_err(|e| nestgate_core::NestGateError::Network(format!("Failed to send message: {}", e)))?;
                
                // Wait for response with timeout
                let response_msg = tokio::time::timeout(
                    Duration::from_secs(30),
                    ws.next()
                ).await
                    .map_err(|_| nestgate_core::NestGateError::Timeout("Response timeout".to_string()))?
                    .ok_or_else(|| nestgate_core::NestGateError::Network("Connection closed".to_string()))?
                    .map_err(|e| nestgate_core::NestGateError::Network(format!("Failed to receive response: {}", e)))?;
                
                // Process response
                if let TungsteniteMessage::Text(response_text) = response_msg {
                    let decrypted_response = self.security_manager.decrypt_message(&response_text).await?;
                    let response: TowerMessageResponse = serde_json::from_str(&decrypted_response)
                        .map_err(|e| nestgate_core::NestGateError::Serialization(format!("Failed to deserialize response: {}", e)))?;
                    
                    return Ok(response);
                }
            }
        }
        
        Err(nestgate_core::NestGateError::Internal("No WebSocket connection".to_string()))
    }
    
    /// Check if connection is healthy
    pub async fn is_healthy(&self) -> bool {
        if !self.is_healthy.load(std::sync::atomic::Ordering::Relaxed) {
            return false;
        }
        
        // Check if connection is too old
        let last_used = *self.last_used.read().await;
        if last_used.elapsed() > Duration::from_secs(300) { // 5 minutes
            return false;
        }
        
        // Could add ping/pong check here
        true
    }
    
    /// Mark connection as potentially bad
    pub async fn mark_potentially_bad(&self) {
        self.is_healthy.store(false, std::sync::atomic::Ordering::Relaxed);
    }
}

/// Message types for inter-tower communication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TowerMessage {
    pub message_id: String,
    pub source_tower: String,
    pub target_tower: String,
    pub message_type: TowerMessageType,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TowerMessageType {
    CapabilitySync,
    TaskDelegation,
    TaskResult,
    HealthCheck,
    ResourceQuery,
    DataTransfer,
    ServiceDiscovery,
    ConsensusVote,
    LeaderElection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerMessageResponse {
    pub message_id: String,
    pub response_to: String,
    pub success: bool,
    pub payload: Option<serde_json::Value>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Message routing for efficient message handling
pub struct MessageRouter {
    pub local_tower_id: String,
    pub route_handlers: Arc<DashMap<TowerMessageType, Box<dyn MessageHandler + Send + Sync>>>,
    pub message_queue: Arc<RwLock<Vec<TowerMessage>>>,
    pub processing_semaphore: Arc<Semaphore>,
}

impl std::fmt::Debug for MessageRouter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageRouter")
            .field("local_tower_id", &self.local_tower_id)
            .field("message_queue", &self.message_queue)
            .field("processing_semaphore", &self.processing_semaphore)
            .finish()
    }
}

impl MessageRouter {
    pub fn new(tower_id: String) -> Self {
        Self {
            local_tower_id: tower_id,
            route_handlers: Arc::new(DashMap::new()),
            message_queue: Arc::new(RwLock::new(Vec::new())),
            processing_semaphore: Arc::new(Semaphore::new(100)), // Max 100 concurrent message processing
        }
    }
    
    /// Start message router
    pub async fn start(&self) -> Result<tokio::task::JoinHandle<()>> {
        let router = self.clone();
        
        Ok(tokio::spawn(async move {
            router.message_processing_loop().await;
        }))
    }
    
    /// Main message processing loop
    async fn message_processing_loop(&self) {
        let mut interval = tokio::time::interval(Duration::from_millis(10));
        
        loop {
            interval.tick().await;
            
            // Process queued messages
            let messages_to_process = {
                let mut queue = self.message_queue.write().await;
                let batch_size = std::cmp::min(queue.len(), 50); // Process up to 50 messages at once
                queue.drain(0..batch_size).collect::<Vec<_>>()
            };
            
            if !messages_to_process.is_empty() {
                let processing_tasks: Vec<_> = messages_to_process.into_iter()
                    .map(|message| {
                        let router = self.clone();
                        tokio::spawn(async move {
                            router.process_message(message).await;
                        })
                    })
                    .collect();
                
                // Wait for all processing tasks to complete
                for task in processing_tasks {
                    let _ = task.await;
                }
            }
        }
    }
    
    /// Process individual message
    async fn process_message(&self, message: TowerMessage) {
        let _permit = match self.processing_semaphore.acquire().await {
            Ok(permit) => permit,
            Err(e) => {
                error!("Failed to acquire processing permit: {}", e);
                return;
            }
        };
        
        debug!("Processing message: {} from tower: {}", message.message_id, message.source_tower);
        
        if let Some(handler) = self.route_handlers.get(&message.message_type) {
            if let Err(e) = handler.handle_message(message).await {
                error!("Failed to handle message: {}", e);
            }
        } else {
            warn!("No handler found for message type: {:?}", message.message_type);
        }
    }
    
    /// Add message to processing queue
    pub async fn queue_message(&self, message: TowerMessage) {
        let mut queue = self.message_queue.write().await;
        queue.push(message);
    }
    
    /// Register message handler
    pub fn register_handler(&self, message_type: TowerMessageType, handler: Box<dyn MessageHandler + Send + Sync>) {
        self.route_handlers.insert(message_type, handler);
    }
}

impl Clone for MessageRouter {
    fn clone(&self) -> Self {
        Self {
            local_tower_id: self.local_tower_id.clone(),
            route_handlers: self.route_handlers.clone(),
            message_queue: self.message_queue.clone(),
            processing_semaphore: self.processing_semaphore.clone(),
        }
    }
}

/// Trait for message handlers
#[async_trait::async_trait]
pub trait MessageHandler: std::fmt::Debug {
    async fn handle_message(&self, message: TowerMessage) -> Result<TowerMessageResponse>;
}

/// Security manager for inter-tower communication
#[derive(Debug)]
pub struct CommunicationSecurityManager {
    pub encryption_key: Option<[u8; 32]>,
    pub trusted_towers: Arc<DashMap<String, TowerSecurityInfo>>,
}

#[derive(Debug, Clone)]
pub struct TowerSecurityInfo {
    pub tower_id: String,
    pub public_key: Vec<u8>,
    pub last_verified: chrono::DateTime<chrono::Utc>,
    pub trust_level: TrustLevel,
}

#[derive(Debug, Clone)]
pub enum TrustLevel {
    Trusted,
    Verified,
    Unknown,
    Blocked,
}

impl Default for CommunicationSecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CommunicationSecurityManager {
    pub fn new() -> Self {
        Self {
            encryption_key: None, // Would be loaded from config
            trusted_towers: Arc::new(DashMap::new()),
        }
    }
    
    /// Encrypt message
    pub async fn encrypt_message(&self, message: &str) -> Result<String> {
        // For now, return as-is. In production, implement AES-GCM encryption
        Ok(message.to_string())
    }
    
    /// Decrypt message
    pub async fn decrypt_message(&self, encrypted_message: &str) -> Result<String> {
        // For now, return as-is. In production, implement AES-GCM decryption
        Ok(encrypted_message.to_string())
    }
}

impl Clone for CommunicationSecurityManager {
    fn clone(&self) -> Self {
        Self {
            encryption_key: self.encryption_key,
            trusted_towers: self.trusted_towers.clone(),
        }
    }
}

/// Tower discovery service for finding other towers on LAN
#[derive(Debug)]
pub struct TowerDiscoveryService {
    pub local_tower_id: String,
    pub bind_port: u16,
    pub discovered_towers: Arc<DashMap<String, TowerInfo>>,
    pub discovery_config: DiscoveryConfig,
}

#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    pub broadcast_interval: Duration,
    pub discovery_timeout: Duration,
    pub max_towers: u32,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            broadcast_interval: Duration::from_secs(30),
            discovery_timeout: Duration::from_secs(10),
            max_towers: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerInfo {
    pub tower_id: String,
    pub tower_name: String,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub load_factor: f64,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

impl TowerDiscoveryService {
    pub fn new(tower_id: String, bind_port: u16) -> Self {
        Self {
            local_tower_id: tower_id,
            bind_port,
            discovered_towers: Arc::new(DashMap::new()),
            discovery_config: DiscoveryConfig::default(),
        }
    }
    
    /// Start discovery service
    pub async fn start(&self) -> Result<tokio::task::JoinHandle<()>> {
        let service = self.clone();
        
        Ok(tokio::spawn(async move {
            service.discovery_loop().await;
        }))
    }
    
    /// Main discovery loop
    async fn discovery_loop(&self) {
        let mut interval = tokio::time::interval(self.discovery_config.broadcast_interval);
        
        loop {
            interval.tick().await;
            
            // Broadcast our presence
            if let Err(e) = self.broadcast_presence().await {
                warn!("Failed to broadcast presence: {}", e);
            }
            
            // Clean up stale towers
            self.cleanup_stale_towers().await;
        }
    }
    
    /// Broadcast our presence to the network
    async fn broadcast_presence(&self) -> Result<()> {
        // Implementation would use UDP broadcast or mDNS
        debug!("Broadcasting presence for tower: {}", self.local_tower_id);
        Ok(())
    }
    
    /// Clean up towers that haven't been seen recently
    async fn cleanup_stale_towers(&self) {
        let cutoff_time = chrono::Utc::now() - chrono::Duration::minutes(5);
        
        self.discovered_towers.retain(|_, tower_info| {
            tower_info.last_seen > cutoff_time
        });
    }
    
    /// Get tower information
    pub async fn get_tower_info(&self, tower_id: &str) -> Option<TowerInfo> {
        self.discovered_towers.get(tower_id).map(|entry| entry.value().clone())
    }
    
    /// List all discovered towers
    pub async fn list_towers(&self) -> Vec<TowerInfo> {
        self.discovered_towers.iter()
            .map(|entry| entry.value().clone())
            .collect()
    }
}

impl Clone for TowerDiscoveryService {
    fn clone(&self) -> Self {
        Self {
            local_tower_id: self.local_tower_id.clone(),
            bind_port: self.bind_port,
            discovered_towers: self.discovered_towers.clone(),
            discovery_config: self.discovery_config.clone(),
        }
    }
}

/// Communication metrics
#[derive(Debug, Default)]
pub struct CommunicationMetrics {
    pub messages_sent: std::sync::atomic::AtomicU64,
    pub messages_received: std::sync::atomic::AtomicU64,
    pub messages_failed: std::sync::atomic::AtomicU64,
    pub active_connections: std::sync::atomic::AtomicU64,
    pub average_response_time: std::sync::atomic::AtomicU64,
    pub bytes_sent: std::sync::atomic::AtomicU64,
    pub bytes_received: std::sync::atomic::AtomicU64,
}

impl CommunicationMetrics {
    pub fn update_response_time(&self, duration: Duration) {
        let current_avg = self.average_response_time.load(std::sync::atomic::Ordering::Relaxed);
        let new_time = duration.as_millis() as u64;
        
        // Simple moving average (in production, use a more sophisticated approach)
        let new_avg = (current_avg + new_time) / 2;
        self.average_response_time.store(new_avg, std::sync::atomic::Ordering::Relaxed);
    }
    
    pub async fn collect_and_log(&self) {
        info!("Communication Metrics - Sent: {}, Received: {}, Failed: {}, Active Connections: {}, Avg Response Time: {}ms",
            self.messages_sent.load(std::sync::atomic::Ordering::Relaxed),
            self.messages_received.load(std::sync::atomic::Ordering::Relaxed),
            self.messages_failed.load(std::sync::atomic::Ordering::Relaxed),
            self.active_connections.load(std::sync::atomic::Ordering::Relaxed),
            self.average_response_time.load(std::sync::atomic::Ordering::Relaxed)
        );
    }
}

/// Pool-specific metrics
#[derive(Debug, Default)]
pub struct PoolMetrics {
    pub successful_messages: std::sync::atomic::AtomicU64,
    pub failed_messages: std::sync::atomic::AtomicU64,
    pub connection_creations: std::sync::atomic::AtomicU64,
    pub connection_failures: std::sync::atomic::AtomicU64,
} 