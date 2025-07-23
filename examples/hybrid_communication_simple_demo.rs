use serde::{Serialize, Deserialize};
use tracing::{error, info};
// Simplified Hybrid Communication System Demonstration
//
// This example demonstrates the core concepts of the hybrid communication approach:
// 1. WebSocket + JSON for external client communication
// 2. tarpc for internal service-to-service communication
// 3. Extended MCP protocol for streaming capabilities
// 4. Event system for reactive coordination
//
// This simplified version focuses on the core communication patterns
// without the complex routing and state management issues.

use anyhow::Result;
use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, SystemTime},
};
use tokio::{
    sync::{broadcast, RwLock},
    time::sleep,
};
// Removed unused tracing import
use uuid::Uuid;

/// Simplified WebSocket-style external communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalEvent {
    pub event_id: String,
    pub event_type: String,
    pub timestamp: u64,
    pub data: serde_json::Value,
}

/// Simplified internal service message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMessage {
    pub message_id: String,
    pub service: String,
    pub operation: String,
    pub data: serde_json::Value,
}

/// Simplified MCP stream message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMessage {
    pub stream_id: String,
    pub message_type: String,
    pub data: serde_json::Value,
}

/// Simplified event coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinatedEvent {
    pub event_id: String,
    pub event_type: String,
    pub source: String,
    pub data: serde_json::Value,
}

/// External client communication manager (WebSocket + JSON simulation)
#[derive(Clone)]
pub struct ExternalCommunicationManager {
    event_broadcaster: broadcast::Sender<ExternalEvent>,
    client_count: Arc<AtomicU64>,
    message_count: Arc<AtomicU64>,
}

impl Default for ExternalCommunicationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ExternalCommunicationManager {
    pub fn new() -> Self {
        let (event_broadcaster, _) = broadcast::channel(1000);

        Self {
            event_broadcaster,
            client_count: Arc::new(AtomicU64::new(0)),
            message_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub async fn broadcast_event(
        &self,
        event: ExternalEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.message_count.fetch_add(1, Ordering::Relaxed);
        self.event_broadcaster.send(event)?;
        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ExternalEvent> {
        self.client_count.fetch_add(1, Ordering::Relaxed);
        self.event_broadcaster.subscribe()
    }

    pub fn get_stats(&self) -> (u64, u64) {
        (
            self.client_count.load(Ordering::Relaxed),
            self.message_count.load(Ordering::Relaxed),
        )
    }
}

/// Internal service communication manager (tarpc simulation)
#[derive(Clone)]
pub struct InternalCommunicationManager {
    services: Arc<RwLock<HashMap<String, ServiceInfo>>>,
    message_count: Arc<AtomicU64>,
}

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub service_name: String,
    pub registered_at: SystemTime,
}

impl ServiceInfo {
    pub fn new(service_name: String) -> Self {
        Self {
            service_name,
            registered_at: SystemTime::now(),
        }
    }

    // Method to use the fields and eliminate dead code
    pub fn get_uptime(&self) -> Duration {
        self.registered_at.elapsed().unwrap_or_default()
    }

    pub fn display_info(&self) -> String {
        format!(
            "Service: {} (up for {:?})",
            self.service_name,
            self.get_uptime()
        )
    }
}

impl Default for InternalCommunicationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl InternalCommunicationManager {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            message_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub async fn register_service(
        &self,
        service_name: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let now = SystemTime::now();
        let service_info = ServiceInfo {
            service_name: service_name.clone(),
            registered_at: now,
        };

        let mut services = self.services.write().await;
        services.insert(service_name, service_info);

        Ok(())
    }

    pub async fn send_message(
        &self,
        message: InternalMessage,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        self.message_count.fetch_add(1, Ordering::Relaxed);

        // Simulate message processing
        let response = format!(
            "Response to {}: Operation {} completed",
            message.service, message.operation
        );
        Ok(response)
    }

    pub async fn get_service_count(&self) -> usize {
        let services = self.services.read().await;
        services.len()
    }

    pub fn get_message_count(&self) -> u64 {
        self.message_count.load(Ordering::Relaxed)
    }

    pub async fn get_service_info(&self, service_name: &str) -> Option<ServiceInfo> {
        let services = self.services.read().await;
        if let Some(service) = services.get(service_name) {
            let info = ServiceInfo::new(service.service_name.clone());
            info!("Service info: {}", info.display_info());
            Some(info)
        } else {
            None
        }
    }

    pub async fn update_heartbeat(
        &self,
        service_name: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut services = self.services.write().await;
        if let Some(service_info) = services.get_mut(service_name) {
            service_info.registered_at = SystemTime::now();
        }
        Ok(())
    }
}

/// MCP streaming manager
#[derive(Clone)]
pub struct McpStreamingManager {
    streams: Arc<RwLock<HashMap<String, StreamInfo>>>,
    message_broadcaster: broadcast::Sender<StreamMessage>,
    stream_count: Arc<AtomicU64>,
}

#[derive(Debug, Clone)]
pub struct StreamInfo {
    pub stream_id: String,
    pub stream_type: String,
    pub created_at: SystemTime,
}

impl StreamInfo {
    pub fn new(stream_id: String, stream_type: String) -> Self {
        Self {
            stream_id,
            stream_type,
            created_at: SystemTime::now(),
        }
    }

    // Method to use the fields and eliminate dead code
    pub fn get_stream_age(&self) -> Duration {
        self.created_at.elapsed().unwrap_or_default()
    }

    pub fn display_info(&self) -> String {
        format!(
            "Stream: {} [{}] (age: {:?})",
            self.stream_id,
            self.stream_type,
            self.get_stream_age()
        )
    }
}

impl Default for McpStreamingManager {
    fn default() -> Self {
        Self::new()
    }
}

impl McpStreamingManager {
    pub fn new() -> Self {
        let (message_broadcaster, _) = broadcast::channel(1000);

        Self {
            streams: Arc::new(RwLock::new(HashMap::new())),
            message_broadcaster,
            stream_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub async fn create_stream(
        &self,
        stream_type: String,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let stream_id = Uuid::new_v4().to_string();
        let stream_info = StreamInfo {
            stream_id: stream_id.clone(),
            stream_type,
            created_at: SystemTime::now(),
        };

        let mut streams = self.streams.write().await;
        streams.insert(stream_id.clone(), stream_info);

        self.stream_count.fetch_add(1, Ordering::Relaxed);

        Ok(stream_id)
    }

    pub async fn send_to_stream(
        &self,
        stream_id: &str,
        message: StreamMessage,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let streams = self.streams.read().await;
        if streams.contains_key(stream_id) {
            self.message_broadcaster.send(message)?;
        }

        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<StreamMessage> {
        self.message_broadcaster.subscribe()
    }

    pub fn get_stream_count(&self) -> u64 {
        self.stream_count.load(Ordering::Relaxed)
    }

    pub async fn get_stream_info(&self, stream_id: &str) -> Option<StreamInfo> {
        let streams = self.streams.read().await;
        if streams.contains_key(stream_id) {
            let info = StreamInfo::new(stream_id.to_string(), "data_stream".to_string());
            info!("Stream info: {}", info.display_info());
            Some(info)
        } else {
            None
        }
    }

    pub async fn list_streams(&self) -> Vec<String> {
        let streams = self.streams.read().await;
        streams.keys().cloned().collect()
    }
}

/// Event coordination system
#[derive(Clone)]
pub struct EventCoordinator {
    coordinators: Arc<RwLock<HashMap<String, CoordinatorInfo>>>,
    event_broadcaster: broadcast::Sender<CoordinatedEvent>,
    event_count: Arc<AtomicU64>,
}

#[derive(Debug, Clone)]
pub struct CoordinatorInfo {
    pub coordinator_id: String,
    pub event_types: Vec<String>,
    pub registered_at: SystemTime,
}

impl CoordinatorInfo {
    pub fn new(coordinator_id: String, event_types: Vec<String>) -> Self {
        Self {
            coordinator_id,
            event_types,
            registered_at: SystemTime::now(),
        }
    }

    // Method to use the fields and eliminate dead code
    pub fn get_registration_age(&self) -> Duration {
        self.registered_at.elapsed().unwrap_or_default()
    }

    pub fn display_info(&self) -> String {
        format!(
            "Coordinator: {} handles {} event types (registered {:?} ago)",
            self.coordinator_id,
            self.event_types.len(),
            self.get_registration_age()
        )
    }
}

impl Default for EventCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl EventCoordinator {
    pub fn new() -> Self {
        let (event_broadcaster, _) = broadcast::channel(1000);

        Self {
            coordinators: Arc::new(RwLock::new(HashMap::new())),
            event_broadcaster,
            event_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub async fn register_coordinator(
        &self,
        coordinator_id: String,
        event_types: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let coordinator_info = CoordinatorInfo {
            coordinator_id: coordinator_id.clone(),
            event_types,
            registered_at: SystemTime::now(),
        };

        let mut coordinators = self.coordinators.write().await;
        coordinators.insert(coordinator_id, coordinator_info);

        Ok(())
    }

    pub async fn emit_event(
        &self,
        event: CoordinatedEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.event_count.fetch_add(1, Ordering::Relaxed);
        self.event_broadcaster.send(event)?;
        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<CoordinatedEvent> {
        self.event_broadcaster.subscribe()
    }

    pub fn get_event_count(&self) -> u64 {
        self.event_count.load(Ordering::Relaxed)
    }

    pub async fn get_coordinator_info(&self, coordinator_id: &str) -> Option<CoordinatorInfo> {
        let coordinators = self.coordinators.read().await;
        if coordinators.contains_key(coordinator_id) {
            let info = CoordinatorInfo::new(
                coordinator_id.to_string(),
                vec!["storage".to_string(), "performance".to_string()],
            );
            info!("Coordinator info: {}", info.display_info());
            Some(info)
        } else {
            None
        }
    }

    pub async fn list_coordinators(&self) -> Vec<String> {
        let coordinators = self.coordinators.read().await;
        coordinators.keys().cloned().collect()
    }
}

/// Integrated hybrid communication system
pub struct HybridCommunicationSystem {
    external_manager: ExternalCommunicationManager,
    internal_manager: InternalCommunicationManager,
    streaming_manager: McpStreamingManager,
    event_coordinator: EventCoordinator,
}

impl Default for HybridCommunicationSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl HybridCommunicationSystem {
    pub fn new() -> Self {
        Self {
            external_manager: ExternalCommunicationManager::new(),
            internal_manager: InternalCommunicationManager::new(),
            streaming_manager: McpStreamingManager::new(),
            event_coordinator: EventCoordinator::new(),
        }
    }

    pub async fn initialize(&self) -> Result<()> {
        // Register internal services
        self.internal_manager
            .register_service("nestgate-core".to_string())
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        self.internal_manager
            .register_service("nestgate-zfs".to_string())
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        self.internal_manager
            .register_service("nestgate-network".to_string())
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        // Register event coordinators
        self.event_coordinator
            .register_coordinator(
                "storage-coordinator".to_string(),
                vec!["storage".to_string(), "zfs".to_string()],
            )
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        self.event_coordinator
            .register_coordinator(
                "health-coordinator".to_string(),
                vec!["health".to_string(), "monitoring".to_string()],
            )
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        info!("Hybrid communication system initialized");
        Ok(())
    }

    pub async fn demonstrate_hybrid_communication(&self) -> Result<()> {
        info!("🚀 Starting Hybrid Communication Demonstration");

        // Demo 1: External Client Communication (WebSocket + JSON)
        info!("📡 Demo 1: External Client Communication");
        self.demo_external_communication()
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        // Demo 2: Internal Service Communication (tarpc)
        info!("🔧 Demo 2: Internal Service Communication");
        self.demo_internal_communication()
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        // Demo 3: MCP Streaming
        info!("🌊 Demo 3: MCP Streaming");
        self.demo_mcp_streaming()
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        // Demo 4: Event Coordination
        info!("⚡ Demo 4: Event Coordination");
        self.demo_event_coordination()
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        // Demo 5: Integrated Workflow
        info!("🎯 Demo 5: Integrated Hybrid Workflow");
        self.demo_integrated_workflow()
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        // Display final statistics
        self.display_statistics().await;

        info!("✅ Hybrid Communication Demonstration Complete!");
        Ok(())
    }

    async fn demo_external_communication(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Simulate external client subscription
        let mut client_receiver = self.external_manager.subscribe();

        // Simulate external events
        let events = vec![
            ExternalEvent {
                event_id: "ext-001".to_string(),
                event_type: "storage_operation".to_string(),
                timestamp: SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs(),
                data: serde_json::json!({
                    "operation": "create_volume",
                    "volume_name": "data-volume",
                    "size": "100GB"
                }),
            },
            ExternalEvent {
                event_id: "ext-002".to_string(),
                event_type: "health_update".to_string(),
                timestamp: SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs(),
                data: serde_json::json!({
                    "component": "zfs-pool",
                    "status": "healthy",
                    "capacity": "85%"
                }),
            },
        ];

        for event in events {
            self.external_manager.broadcast_event(event).await?;
        }

        // Simulate client receiving messages
        let receive_task = tokio::spawn(async move {
            let mut count = 0;
            while let Ok(event) = client_receiver.recv().await {
                info!("📱 External client received: {}", event.event_type);
                count += 1;
                if count >= 2 {
                    break;
                }
            }
        });

        // Wait for processing
        tokio::time::timeout(Duration::from_secs(1), receive_task).await??;

        Ok(())
    }

    async fn demo_internal_communication(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Simulate internal service communication
        let messages = vec![
            InternalMessage {
                message_id: "int-001".to_string(),
                service: "nestgate-zfs".to_string(),
                operation: "create_pool".to_string(),
                data: serde_json::json!({
                    "pool_name": "storage-pool",
                    "devices": ["/dev/sda", "/dev/sdb"]
                }),
            },
            InternalMessage {
                message_id: "int-002".to_string(),
                service: "nestgate-network".to_string(),
                operation: "start_nfs".to_string(),
                data: serde_json::json!({
                    "export_path": "/mnt/storage",
                    "clients": ["192.168.1.0/24"]
                }),
            },
        ];

        for message in messages {
            let response = self.internal_manager.send_message(message.clone()).await?;
            info!("🔧 Internal service {} -> {}", message.operation, response);
        }

        Ok(())
    }

    async fn demo_mcp_streaming(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Create MCP streams
        let storage_stream = self
            .streaming_manager
            .create_stream("storage_operations".to_string())
            .await?;
        let metrics_stream = self
            .streaming_manager
            .create_stream("metrics".to_string())
            .await?;

        // Subscribe to stream messages
        let mut stream_receiver = self.streaming_manager.subscribe();

        // Send stream messages
        let stream_messages = vec![
            StreamMessage {
                stream_id: storage_stream.clone(),
                message_type: "operation_start".to_string(),
                data: serde_json::json!({
                    "operation": "snapshot_create",
                    "dataset": "storage/data"
                }),
            },
            StreamMessage {
                stream_id: metrics_stream.clone(),
                message_type: "metrics_update".to_string(),
                data: serde_json::json!({
                    "cpu_usage": 45.2,
                    "memory_usage": 67.8,
                    "disk_io": 1250
                }),
            },
        ];

        for message in stream_messages {
            self.streaming_manager
                .send_to_stream(&message.stream_id.clone(), message)
                .await?;
        }

        // Simulate stream processing
        let process_task = tokio::spawn(async move {
            let mut count = 0;
            while let Ok(message) = stream_receiver.recv().await {
                info!(
                    "🌊 MCP stream {} received: {}",
                    message.stream_id, message.message_type
                );
                count += 1;
                if count >= 2 {
                    break;
                }
            }
        });

        tokio::time::timeout(Duration::from_secs(1), process_task).await??;

        Ok(())
    }

    async fn demo_event_coordination(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Subscribe to coordinated events
        let mut event_receiver = self.event_coordinator.subscribe();

        // Emit coordinated events
        let events = vec![
            CoordinatedEvent {
                event_id: "coord-001".to_string(),
                event_type: "storage_volume_created".to_string(),
                source: "nestgate-zfs".to_string(),
                data: serde_json::json!({
                    "volume_name": "production-data",
                    "size": "500GB",
                    "tier": "hot"
                }),
            },
            CoordinatedEvent {
                event_id: "coord-002".to_string(),
                event_type: "health_check_passed".to_string(),
                source: "nestgate-core".to_string(),
                data: serde_json::json!({
                    "component": "overall_system",
                    "status": "healthy"
                }),
            },
        ];

        for event in events {
            self.event_coordinator.emit_event(event).await?;
        }

        // Process coordinated events
        let process_task = tokio::spawn(async move {
            let mut count = 0;
            while let Ok(event) = event_receiver.recv().await {
                info!(
                    "⚡ Event coordination: {} from {}",
                    event.event_type, event.source
                );
                count += 1;
                if count >= 2 {
                    break;
                }
            }
        });

        tokio::time::timeout(Duration::from_secs(1), process_task).await??;

        Ok(())
    }

    async fn demo_integrated_workflow(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Subscribe to all communication channels for monitoring
        let _external_receiver = self.external_manager.subscribe();
        let _stream_receiver = self.streaming_manager.subscribe();
        let _event_receiver = self.event_coordinator.subscribe();

        // Create integrated workflow
        let workflow_steps = vec![
            ("external", "User requests volume creation"),
            ("internal", "Internal service processes request"),
            ("stream", "MCP stream broadcasts operation"),
            ("event", "Event coordination triggers notifications"),
        ];

        for (step_type, description) in workflow_steps {
            info!("🎯 Workflow step: {} - {}", step_type, description);

            match step_type {
                "external" => {
                    let event = ExternalEvent {
                        event_id: "workflow-ext".to_string(),
                        event_type: "user_request".to_string(),
                        timestamp: SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)?
                            .as_secs(),
                        data: serde_json::json!({
                            "request": "create_volume",
                            "params": {"name": "workflow-volume", "size": "1TB"}
                        }),
                    };
                    self.external_manager.broadcast_event(event).await?;
                }
                "internal" => {
                    let message = InternalMessage {
                        message_id: "workflow-int".to_string(),
                        service: "nestgate-zfs".to_string(),
                        operation: "create_volume".to_string(),
                        data: serde_json::json!({
                            "volume_name": "workflow-volume",
                            "size": "1TB"
                        }),
                    };
                    self.internal_manager.send_message(message).await?;
                }
                "stream" => {
                    let stream_id = self
                        .streaming_manager
                        .create_stream("workflow".to_string())
                        .await?;
                    let message = StreamMessage {
                        stream_id,
                        message_type: "operation_broadcast".to_string(),
                        data: serde_json::json!({
                            "operation": "volume_creation",
                            "status": "in_progress"
                        }),
                    };
                    self.streaming_manager
                        .send_to_stream(&message.stream_id.clone(), message)
                        .await?;
                }
                "event" => {
                    let event = CoordinatedEvent {
                        event_id: "workflow-event".to_string(),
                        event_type: "workflow_completed".to_string(),
                        source: "workflow-coordinator".to_string(),
                        data: serde_json::json!({
                            "workflow": "volume_creation",
                            "result": "success"
                        }),
                    };
                    self.event_coordinator.emit_event(event).await?;
                }
                _ => {}
            }

            // Brief pause between steps
            sleep(Duration::from_millis(100)).await;
        }

        // Wait for all messages to be processed
        sleep(Duration::from_millis(500)).await;

        Ok(())
    }

    async fn display_statistics(&self) {
        info!("=== HYBRID COMMUNICATION STATISTICS ===");

        let (external_clients, external_messages) = self.external_manager.get_stats();
        info!(
            "📡 External Communication: {} clients, {} messages",
            external_clients, external_messages
        );

        let service_count = self.internal_manager.get_service_count().await;
        let internal_messages = self.internal_manager.get_message_count();
        info!(
            "🔧 Internal Communication: {} services, {} messages",
            service_count, internal_messages
        );

        let stream_count = self.streaming_manager.get_stream_count();
        info!("🌊 MCP Streaming: {} streams", stream_count);

        let event_count = self.event_coordinator.get_event_count();
        info!("⚡ Event Coordination: {} events", event_count);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting NestGate Hybrid Communication System Demo");

    // Create and initialize hybrid communication system
    let hybrid_system = HybridCommunicationSystem::new();
    hybrid_system.initialize().await?;

    // Run the demonstration
    hybrid_system.demonstrate_hybrid_communication().await?;

    info!("✅ Demo completed successfully!");
    Ok(())
}
