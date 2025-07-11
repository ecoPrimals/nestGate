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
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::time::{sleep, timeout};
use tracing::{error, info, warn};

// Import hybrid communication components
use nestgate_api::{
    event_coordination::{
        CoordinatedEvent, CoordinatedEventType, EventCoordinator, EventHandler, EventHandlerType,
        EventSource, Priority,
    },
    mcp_streaming::{McpStreamType, McpStreamingManager, StreamConfig},
    tarpc_service::{StorageOperation, TarpcClient, TarpcServer, ZfsOperation},
    websocket::{ClientType, EventType, WebSocketEvent, WebSocketManager},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting NestGate Hybrid Communication System Demo");

    // Initialize all components
    let websocket_manager = WebSocketManager::new();
    let mcp_streaming_manager = McpStreamingManager::new();
    let event_coordinator = EventCoordinator::new()
        .with_websocket_manager(websocket_manager.clone())
        .with_mcp_streaming_manager(mcp_streaming_manager.clone());

    // Start background tasks
    let _websocket_cleanup = websocket_manager.start_cleanup_task();
    let _mcp_cleanup = mcp_streaming_manager.start_cleanup_task();

    // Demo 1: WebSocket + JSON for External Client Communication
    info!("📡 Demo 1: WebSocket + JSON External Client Communication");
    demo_websocket_communication(&websocket_manager).await?;

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
    demo_integrated_hybrid_communication(
        &websocket_manager,
        &mcp_streaming_manager,
        &event_coordinator,
    )
    .await?;

    info!("✅ Hybrid Communication System Demo Complete!");
    Ok(())
}

/// Demo WebSocket + JSON communication for external clients
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
    let client = TarpcClient::connect(server_addr).await?;

    // Demo storage operations
    let storage_op = StorageOperation::CreateVolume {
        name: "demo-volume".to_string(),
        size: 1073741824, // 1GB
        tier: "warm".to_string(),
        options: HashMap::new(),
    };

    let storage_result = client.execute_storage_operation(storage_op).await?;
    info!("Storage operation result: {:?}", storage_result);

    // Demo ZFS operations
    let zfs_op = ZfsOperation::GetPoolStatus {
        pool: "storage-pool".to_string(),
    };

    let zfs_result = client.execute_zfs_operation(zfs_op).await?;
    info!("ZFS operation result: {:?}", zfs_result);

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
    mcp_manager: &McpStreamingManager,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up MCP streaming demo...");

    // Create storage operation stream
    let storage_config = StreamConfig {
        stream_type: McpStreamType::StorageOperations,
        source: "nestgate-zfs".to_string(),
        target: "mcp-cluster".to_string(),
        buffer_size: 4096,
        batch_size: 50,
        flush_interval: Duration::from_millis(100),
        compression: true,
        encryption: true,
        metadata: HashMap::new(),
    };

    let storage_stream_id = mcp_manager.create_stream(storage_config).await?;
    info!("Created storage stream: {}", storage_stream_id);

    // Create metrics stream
    let metrics_config = StreamConfig {
        stream_type: McpStreamType::MetricsStream,
        source: "nestgate-core".to_string(),
        target: "monitoring-service".to_string(),
        buffer_size: 2048,
        batch_size: 25,
        flush_interval: Duration::from_secs(1),
        compression: false,
        encryption: false,
        metadata: HashMap::new(),
    };

    let metrics_stream_id = mcp_manager.create_stream(metrics_config).await?;
    info!("Created metrics stream: {}", metrics_stream_id);

    // Send data to streams
    let storage_data = serde_json::json!({
        "operation": "snapshot_create",
        "dataset": "storage/volume1",
        "snapshot_name": "daily-backup",
        "timestamp": SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs()
    });

    mcp_manager
        .send_to_stream(&storage_stream_id, storage_data)
        .await?;

    let metrics_data = serde_json::json!({
        "cpu_usage": 45.6,
        "memory_usage": 78.3,
        "disk_io": 1250,
        "network_io": 890,
        "timestamp": SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs()
    });

    mcp_manager
        .send_to_stream(&metrics_stream_id, metrics_data)
        .await?;

    // Display stream statistics
    let stats = mcp_manager.get_stats();
    info!("MCP Streaming Stats: {:?}", stats);

    // List active streams
    let streams = mcp_manager.list_streams().await;
    info!("Active streams: {}", streams.len());

    // Close streams
    mcp_manager.close_stream(&storage_stream_id).await?;
    mcp_manager.close_stream(&metrics_stream_id).await?;

    Ok(())
}

/// Demo event coordination system
async fn demo_event_coordination(
    event_coordinator: &EventCoordinator,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up event coordination demo...");

    // Register event handlers
    let storage_handler = EventHandler {
        handler_id: "storage-handler".to_string(),
        handler_type: EventHandlerType::StorageOperation,
        event_patterns: vec!["storage".to_string(), "zfs".to_string()],
        priority: Priority::High,
        metadata: HashMap::new(),
    };

    event_coordinator.register_handler(storage_handler).await?;

    let websocket_handler = EventHandler {
        handler_id: "websocket-handler".to_string(),
        handler_type: EventHandlerType::WebSocket,
        event_patterns: vec!["all".to_string()],
        priority: Priority::Normal,
        metadata: HashMap::new(),
    };

    event_coordinator
        .register_handler(websocket_handler)
        .await?;

    let health_handler = EventHandler {
        handler_id: "health-handler".to_string(),
        handler_type: EventHandlerType::HealthMonitoring,
        event_patterns: vec!["health".to_string()],
        priority: Priority::Critical,
        metadata: HashMap::new(),
    };

    event_coordinator.register_handler(health_handler).await?;

    // Emit coordinated events
    let storage_event = CoordinatedEvent {
        event_id: "coord-storage-001".to_string(),
        event_type: CoordinatedEventType::VolumeCreated,
        source: EventSource::System {
            component: "nestgate-zfs".to_string(),
        },
        timestamp: SystemTime::now(),
        data: serde_json::json!({
            "volume_name": "production-data",
            "size": "500GB",
            "tier": "hot"
        }),
        metadata: HashMap::new(),
        correlation_id: Some("storage-op-001".to_string()),
        priority: Priority::High,
    };

    event_coordinator.emit_event(storage_event).await?;

    let health_event = CoordinatedEvent {
        event_id: "coord-health-001".to_string(),
        event_type: CoordinatedEventType::HealthCheckPassed,
        source: EventSource::System {
            component: "nestgate-core".to_string(),
        },
        timestamp: SystemTime::now(),
        data: serde_json::json!({
            "component": "zfs-pool",
            "status": "healthy",
            "details": "All disks online"
        }),
        metadata: HashMap::new(),
        correlation_id: None,
        priority: Priority::Normal,
    };

    event_coordinator.emit_event(health_event).await?;

    let alert_event = CoordinatedEvent::alert("warning", "Disk usage approaching 90% threshold");

    event_coordinator.emit_event(alert_event).await?;

    // Display event statistics
    let stats = event_coordinator.get_stats();
    info!("Event Coordination Stats: {:?}", stats);

    // List handlers
    let handlers = event_coordinator.list_handlers().await;
    info!("Registered handlers: {}", handlers.len());

    Ok(())
}

/// Demo integrated hybrid communication
async fn demo_integrated_hybrid_communication(
    websocket_manager: &WebSocketManager,
    mcp_manager: &McpStreamingManager,
    event_coordinator: &EventCoordinator,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up integrated hybrid communication demo...");

    // Create a comprehensive workflow that uses all components

    // 1. Create MCP stream for storage operations
    let stream_config = StreamConfig {
        stream_type: McpStreamType::StorageOperations,
        source: "nestgate-api".to_string(),
        target: "external-service".to_string(),
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
        handler_id: "integrated-handler".to_string(),
        handler_type: EventHandlerType::Custom("hybrid".to_string()),
        event_patterns: vec!["all".to_string()],
        priority: Priority::High,
        metadata: HashMap::new(),
    };

    event_coordinator
        .register_handler(integrated_handler)
        .await?;

    // 3. Subscribe to events
    let mut event_receiver = event_coordinator.subscribe_to_events();

    // 4. Start event processing task
    let stream_id_clone = stream_id.clone();
    let mcp_manager_clone = mcp_manager.clone();
    let websocket_manager_clone = websocket_manager.clone();

    let event_processor = tokio::spawn(async move {
        while let Ok(event) = event_receiver.recv().await {
            info!("Processing integrated event: {:?}", event.event_type);

            // Stream to MCP
            if let Err(e) = mcp_manager_clone
                .send_to_stream(&stream_id_clone, event.data.clone())
                .await
            {
                warn!("Failed to stream to MCP: {}", e);
            }

            // Broadcast to WebSocket clients
            let ws_event = WebSocketEvent {
                event_id: event.event_id.clone(),
                event_type: EventType::StorageOperation {
                    operation: "integrated".to_string(),
                    path: "/integrated".to_string(),
                },
                timestamp: event
                    .timestamp
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                source: "integrated-handler".to_string(),
                data: event.data,
                targets: None,
            };

            if let Err(e) = websocket_manager_clone.broadcast_event(ws_event).await {
                warn!("Failed to broadcast to WebSocket: {}", e);
            }
        }
    });

    // 5. Simulate complex workflow
    for i in 0..5 {
        let workflow_event = CoordinatedEvent {
            event_id: format!("workflow-{}", i),
            event_type: CoordinatedEventType::StorageOperationStarted,
            source: EventSource::System {
                component: "integrated-demo".to_string(),
            },
            timestamp: SystemTime::now(),
            data: serde_json::json!({
                "workflow_step": i,
                "operation": "create_snapshot",
                "dataset": format!("storage/demo-{}", i),
                "integrated": true
            }),
            metadata: HashMap::new(),
            correlation_id: Some(format!("workflow-{}", i)),
            priority: Priority::Normal,
        };

        event_coordinator.emit_event(workflow_event).await?;

        // Wait between events
        sleep(Duration::from_millis(200)).await;
    }

    // 6. Wait for processing and cleanup
    sleep(Duration::from_secs(2)).await;
    event_processor.abort();
    mcp_manager.close_stream(&stream_id).await?;

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
