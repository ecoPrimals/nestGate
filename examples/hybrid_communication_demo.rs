//! Hybrid Communication System Demonstration
//!
//! This example demonstrates the four components of the hybrid communication approach:
//! 1. WebSocket + JSON for external client communication
//! 2. tarpc for internal service-to-service communication
//! 3. Extended MCP protocol for streaming capabilities
//! 4. Event system for reactive coordination

use std::{
    collections::HashMap,
    net::SocketAddr,
    time::{Duration, SystemTime},
};
use tokio::time::sleep;
use tracing::{error, info};
use anyhow::Result;
use nestgate_api::{
    event_coordination::{CoordinatedEvent, CoordinatedEventType, EventCoordinator, EventHandler, Priority},
    CommunicationManager,
};
use serde_json::json;
use uuid::Uuid;
use nestgate_core::performance::{StorageOperation, OperationType};

// Mock tarpc types for demo purposes
struct TarpcServer;
struct TarpcClient;

impl TarpcServer {
    fn new(_addr: SocketAddr, _name: String) -> Self {
        Self
    }
    
    async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Mock implementation
        Ok(())
    }
}

impl TarpcClient {
    fn new(_addr: SocketAddr) -> Self {
        Self
    }
    
    async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Mock implementation
        Ok(())
    }
    
    async fn execute_storage_operation(&self, _operation: StorageOperation) -> Result<String, Box<dyn std::error::Error>> {
        // Mock implementation
        Ok("Operation completed successfully".to_string())
    }
    
    async fn execute_zfs_operation(&self, _operation: String) -> Result<String, Box<dyn std::error::Error>> {
        // Mock implementation
        Ok("ZFS operation completed successfully".to_string())
    }
    
    async fn get_service_health(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Mock implementation
        Ok("Service health: OK".to_string())
    }
    
    async fn get_metrics(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Mock implementation
        Ok("Metrics: CPU: 25%, Memory: 45%".to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting NestGate Hybrid Communication System Demo");

    // Initialize all components
    // let websocket_manager = WebSocketManager::new(); // Disabled - websocket module not available
    let mcp_streaming_manager = nestgate_api::mcp_streaming::McpStreamingManager::new();
    let event_coordinator = EventCoordinator::new();

    // Start background tasks
    // let _websocket_cleanup = websocket_manager.start_cleanup_task(); // Disabled - websocket module not available
    let _mcp_cleanup = mcp_streaming_manager.start_cleanup_task();

    // Demo 1: WebSocket + JSON for External Client Communication
    info!("📡 Demo 1: WebSocket + JSON External Client Communication");
    // demo_websocket_communication(&websocket_manager).await?; // Disabled - websocket module not available

    // Demo 2: tarpc for Internal Service Communication
    info!("🔧 Demo 2: tarpc Internal Service Communication");
    demo_tarpc_communication().await?;

    // Demo 3: Extended MCP Protocol for Streaming
    info!("🌊 Demo 3: Extended MCP Protocol Streaming");
    demo_mcp_streaming(&mcp_streaming_manager).await?;

    // Demo 4: Event System for Reactive Coordination
    info!("⚡ Demo 4: Event System Reactive Coordination");
    demo_event_coordination(&event_coordinator).await?;

    // Demo 5: Integrated Hybrid Communication
    info!("🎯 Demo 5: Integrated Hybrid Communication");
    // demo_integrated_hybrid_communication(
    //     &websocket_manager,
    //     &mcp_streaming_manager,
    //     &event_coordinator,
    // )
    // .await?; // Disabled - websocket module not available

    info!("✅ Hybrid Communication System Demo Complete!");
    Ok(())
}

/// Demo WebSocket + JSON communication for external clients
// Disabled - websocket module not available
/*
async fn demo_websocket_communication(
    websocket_manager: &WebSocketManager,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up WebSocket communication demo...");

    // Simulate external client events
    let storage_event = WebSocketEvent {
        event_id: "storage-001".to_string(),
        event_type: EventType::StorageOperation {
            operation: "create_volume".to_string(),
            path: "/mnt/storage/volume1".to_string(),
        },
        timestamp: SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs(),
        source: "nestgate-zfs".to_string(),
        data: serde_json::json!({
            "volume_name": "volume1",
            "size": "100GB",
            "tier": "warm"
        }),
        targets: None,
    };

    // Broadcast event to WebSocket clients
    websocket_manager.broadcast_event(storage_event).await?;

    // Simulate health updates
    let health_event = WebSocketEvent {
        event_id: "health-001".to_string(),
        event_type: EventType::HealthUpdate {
            component: "zfs-pool".to_string(),
            status: "healthy".to_string(),
        },
        timestamp: SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs(),
        source: "nestgate-core".to_string(),
        data: serde_json::json!({
            "pool_name": "storage-pool",
            "capacity": "85%",
            "health": "ONLINE"
        }),
        targets: None,
    };

    websocket_manager.broadcast_event(health_event).await?;

    // Display statistics
    let stats = websocket_manager.get_stats();
    info!("WebSocket Stats: {:?}", stats);

    Ok(())
}
*/

/// Demo tarpc communication for internal services
async fn demo_tarpc_communication() -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up tarpc internal service communication demo...");

    // Start tarpc server
    let server_addr: SocketAddr = "127.0.0.1:8001".parse()?;
    let server = TarpcServer::new(server_addr, "demo-service".to_string());

    // Start server in background
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.start().await {
            error!("tarpc server error: {}", e);
        }
    });

    // Wait for server to start
    sleep(Duration::from_millis(100)).await;

    // Connect client
    let client = TarpcClient::new(server_addr);
    client.connect().await?;

    // Simulate storage operation (without actual ZFS operations for simplicity)
    info!("Simulating storage operation...");
    
    // Create mock storage operation data instead of actual ZFS operations
    let storage_operation_data = json!({
        "operation": "get_pool_status",
        "pool_name": "rpool",
        "include_status": true,
        "result": {
            "status": "ONLINE",
            "capacity": "75%",
            "health": "HEALTHY"
        }
    });

    info!("Storage operation result: {:?}", storage_operation_data);

    // Demo health check
    let health = client.get_service_health().await?;
    info!("Service health: {:?}", health);

    // Demo metrics
    let metrics = client.get_metrics().await?;
    info!("Service metrics: {:?}", metrics);

    // Stop server
    server_handle.abort();

    Ok(())
}

/// Demo MCP streaming capabilities
async fn demo_mcp_streaming(
    mcp_manager: &nestgate_api::mcp_streaming::McpStreamingManager,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up MCP streaming demo...");

    // Create storage operation stream
    let storage_config = nestgate_api::mcp_streaming::StreamConfig {
        stream_type: nestgate_api::mcp_streaming::StreamType::StorageMonitoring,
        buffer_size: 4096,
        compression: true,
        encryption: false,
        batch_size: 50,
        flush_interval: Duration::from_millis(100),
        metadata: HashMap::new(),
    };

    let storage_stream_info = mcp_manager.create_stream(storage_config).await
        .map_err(|e| format!("Failed to create storage stream: {}", e))?;
    info!("Created storage stream: {}", storage_stream_info.id);

    // Create metrics stream
    let metrics_config = nestgate_api::mcp_streaming::StreamConfig {
        stream_type: nestgate_api::mcp_streaming::StreamType::MetricsStreaming,
        buffer_size: 2048,
        compression: false,
        encryption: false,
        batch_size: 25,
        flush_interval: Duration::from_secs(1),
        metadata: HashMap::new(),
    };

    let metrics_stream_info = mcp_manager.create_stream(metrics_config).await
        .map_err(|e| format!("Failed to create metrics stream: {}", e))?;
    info!("Created metrics stream: {}", metrics_stream_info.id);

    // Send data to streams
    let storage_data = serde_json::json!({
        "operation": "snapshot_create",
        "dataset": "storage/volume1",
        "snapshot_name": "daily-backup",
        "timestamp": SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs()
    });

    mcp_manager
        .send_to_stream(&storage_stream_info.id, storage_data)
        .await
        .map_err(|e| format!("Failed to send to storage stream: {}", e))?;

    let metrics_data = serde_json::json!({
        "cpu_usage": 45.6,
        "memory_usage": 78.3,
        "disk_io": 1250,
        "network_io": 890,
        "timestamp": SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs()
    });

    mcp_manager
        .send_to_stream(&metrics_stream_info.id, metrics_data)
        .await
        .map_err(|e| format!("Failed to send to metrics stream: {}", e))?;

    // Display stream statistics
    let stats = mcp_manager.get_stats();
    info!("MCP Streaming Stats: {:?}", stats);

    // List active streams
    let streams = mcp_manager.list_streams().await;
    info!("Active streams: {}", streams.len());

    // Close streams
    mcp_manager.close_stream(&storage_stream_info.id).await
        .map_err(|e| format!("Failed to close storage stream: {}", e))?;
    mcp_manager.close_stream(&metrics_stream_info.id).await
        .map_err(|e| format!("Failed to close metrics stream: {}", e))?;

    Ok(())
}

/// Demo event coordination system
async fn demo_event_coordination(
    event_coordinator: &EventCoordinator,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up event coordination demo...");

    // Register event handlers
    let storage_handler = EventHandler {
        id: Uuid::new_v4(),
        name: "storage".to_string(),
        patterns: vec!["storage".to_string(), "zfs".to_string()],
        active: true,
        priority: Priority::High,
        config: serde_json::Value::Object(serde_json::Map::new()),
    };

    event_coordinator.register_handler(storage_handler).await
        .map_err(|e| format!("Failed to register storage handler: {}", e))?;

    let websocket_handler = EventHandler {
        id: Uuid::new_v4(),
        name: "websocket".to_string(),
        patterns: vec!["all".to_string()],
        active: true,
        priority: Priority::Normal,
        config: serde_json::Value::Object(serde_json::Map::new()),
    };

    event_coordinator
        .register_handler(websocket_handler)
        .await
        .map_err(|e| format!("Failed to register websocket handler: {}", e))?;

    let health_handler = EventHandler {
        id: Uuid::new_v4(),
        name: "health".to_string(),
        patterns: vec!["health".to_string()],
        active: true,
        priority: Priority::Critical,
        config: serde_json::Value::Object(serde_json::Map::new()),
    };

    event_coordinator.register_handler(health_handler).await
        .map_err(|e| format!("Failed to register health handler: {}", e))?;

    // Test event coordination with storage operation
    let storage_event = CoordinatedEvent {
        event_id: Uuid::new_v4(),
        event_type: CoordinatedEventType::StorageOperation,
        source: "demo_system".to_string(),
        data: serde_json::json!({
            "operation": "create_volume",
            "volume_name": "demo-volume",
            "size": "10GB",
            "metadata": {},
            "correlation_id": "storage-op-001",
            "priority": "high"
        }),
        timestamp: SystemTime::now(),
    };

    event_coordinator.emit_event(storage_event).await
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    let health_event = CoordinatedEvent {
        event_id: Uuid::new_v4(),
        event_type: CoordinatedEventType::HealthMonitoring,
        source: "health_monitor".to_string(),
        data: serde_json::json!({
            "check_type": "health_check_passed",
            "component": "storage_system",
            "status": "healthy",
            "metadata": {},
            "correlation_id": null,
            "priority": "normal"
        }),
        timestamp: SystemTime::now(),
    };

    event_coordinator.emit_event(health_event).await
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    // Create a simple alert event
    let alert_event = CoordinatedEvent {
        event_id: Uuid::new_v4(),
        event_type: CoordinatedEventType::HealthMonitoring,
        source: "disk_monitor".to_string(),
        data: serde_json::json!({
            "alert_type": "warning",
            "message": "Disk usage approaching 90% threshold",
            "severity": "warning",
            "component": "storage"
        }),
        timestamp: SystemTime::now(),
    };

    event_coordinator.emit_event(alert_event).await
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    let stats = event_coordinator.get_stats().await;
    info!("Event Coordination Stats: {:?}", stats);

    // List handlers
    let handlers = event_coordinator.list_handlers().await;
    info!("Registered handlers: {}", handlers.len());

    Ok(())
}

// Demo integrated hybrid communication
// Disabled - websocket module not available
/*
async fn demo_integrated_hybrid_communication(
    websocket_manager: &WebSocketManager,
    mcp_manager: &McpStreamingManager,
    event_coordinator: &EventCoordinator,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up integrated hybrid communication demo...");

    // Create a comprehensive workflow that uses all components

    // 1. Create MCP stream for storage operations
    let stream_config = StreamConfig {
        stream_type: StreamType::StorageMonitoring,
        buffer_size: 8192,
        batch_size: 100,
        flush_interval: Duration::from_millis(50),
        compression: true,
        encryption: true,
        metadata: HashMap::new(),
    };

    let stream_id = mcp_manager.create_stream(stream_config).await?;

    // 2. Register integrated event handler
    let integrated_handler = EventHandler {
        id: Uuid::new_v4(),
        name: "integrated-handler".to_string(),
        patterns: vec!["all".to_string()],
        priority: Priority::High,
        active: true,
        config: serde_json::json!({"type": "hybrid"}),
    };

    event_coordinator
        .register_handler(integrated_handler)
        .await?;

    // 3. Event subscription removed - using handler-based approach instead

    // 4. Event processing through registered handlers (automatic via event coordinator)

    // 5. Simulate complex workflow
    for i in 0..5 {
        let workflow_event = CoordinatedEvent {
            event_id: Uuid::new_v4(),
            event_type: CoordinatedEventType::StorageOperation,
            source: "integrated-demo".to_string(),
            timestamp: SystemTime::now(),
            data: serde_json::json!({
                "workflow_step": i,
                "operation": "create_snapshot",
                "dataset": format!("storage/demo-{}", i),
                "integrated": true
            }),
        };

        event_coordinator.emit_event(workflow_event).await?;

        // Wait between events
        sleep(Duration::from_millis(200)).await;
    }

    // 6. Wait for processing and cleanup
    sleep(Duration::from_secs(2)).await;
    mcp_manager.close_stream(&stream_id.id).await?;

    // 7. Display final statistics
    info!("=== FINAL HYBRID COMMUNICATION STATISTICS ===");
    info!("WebSocket Stats: {:?}", websocket_manager.get_stats());
    info!("MCP Streaming Stats: {:?}", mcp_manager.get_stats());
    info!(
        "Event Coordination Stats: {:?}",
        event_coordinator.get_stats()
    );

    Ok(())
}
*/
