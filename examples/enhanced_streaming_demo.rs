//! Enhanced Streaming Communication Demo
//!
//! This demo showcases the enhanced streaming capabilities of NestGate, including:
//! - Server-Sent Events (SSE) for real-time updates
//! - WebSocket communication for bidirectional streaming
//! - MCP (Message Channel Protocol) streaming for efficient data transfer
//! - Event coordination and monitoring
//!
//! Note: This demo requires the streaming-rpc feature to be enabled.

#![allow(dead_code)]

#[cfg(feature = "streaming-rpc")]
mod streaming_demo {
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime},
};

use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::time::{sleep, timeout};
use tracing::{error, info, warn};

// Import all communication components
use nestgate_api::{
    event_coordination::{
        CoordinatedEvent, CoordinatedEventType, EventCoordinator, EventHandler, Priority,
    },
    mcp_streaming::{McpStreamingManager, StreamConfig, StreamType},
    sse::{
        create_health_event, create_metrics_event, create_storage_event, EventPriority, SseEvent,
        SseEventType, SseManager,
    },
    streaming_rpc::{
        ClientMessage, Command, EventFilter, EventSubscription, ServerMessage, StorageOperation,
        StreamingRpcClient, StreamingRpcServer, SystemMetrics, ZfsOperation,
    },
    websocket::{ClientType, WebSocketEvent, WebSocketEventType, WebSocketManager},
    CommunicationManager,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize comprehensive logging
    tracing_subscriber::fmt().with_env_filter("debug").init();

    info!("🚀 Starting Enhanced Streaming and Bidirectional Communication Demo");

    // Initialize the unified communication manager
    let comm_manager = Arc::new(CommunicationManager::new());

    // Start all communication protocols in the background
    let comm_manager_clone = comm_manager.clone();
    let server_task = tokio::spawn(async move {
        if let Err(e) = comm_manager_clone
            .start_all("127.0.0.1:8080", "127.0.0.1:8081")
            .await
        {
            error!("Failed to start communication protocols: {}", e);
        }
    });

    // Give servers time to start
    sleep(Duration::from_secs(2)).await;

    // Demo 1: Server-Sent Events (SSE) Streaming
    info!("📡 Demo 1: Server-Sent Events (SSE) Streaming");
    demo_sse_streaming(&comm_manager).await?;

    // Demo 2: Bidirectional RPC with tarpc
    info!("🔧 Demo 2: Bidirectional RPC Communication");
    demo_bidirectional_rpc().await?;

    // Demo 3: WebSocket Real-time Communication
    info!("🌐 Demo 3: WebSocket Real-time Communication");
    demo_websocket_communication(&comm_manager).await?;

    // Demo 4: MCP Streaming for AI Integration
    info!("🤖 Demo 4: MCP Streaming for AI Integration");
    demo_mcp_streaming(&comm_manager).await?;

    // Demo 5: Unified Event Coordination
    info!("⚡ Demo 5: Unified Event Coordination");
    demo_unified_event_coordination(&comm_manager).await?;

    // Demo 6: Performance Monitoring and Metrics
    info!("📊 Demo 6: Performance Monitoring and Metrics");
    demo_performance_monitoring(&comm_manager).await?;

    // Demo 7: Full Integration Workflow
    info!("🎯 Demo 7: Full Integration Workflow");
    demo_full_integration_workflow(&comm_manager).await?;

    info!("✅ Enhanced Streaming and Bidirectional Communication Demo Complete!");

    // Cleanup
    server_task.abort();
    Ok(())
}

/// Demo Server-Sent Events (SSE) streaming capabilities
async fn demo_sse_streaming(
    comm_manager: &CommunicationManager,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up SSE streaming demo...");

    // Create various types of SSE events
    let storage_event = create_storage_event(
        "dataset_created",
        "/storage/new-dataset",
        json!({
            "size": "10GB",
            "type": "filesystem",
            "properties": {
                "compression": "lz4",
                "recordsize": "128K"
            }
        }),
    );

    let health_event = create_health_event(
        "zfs_pool",
        "healthy",
        json!({
            "capacity": 75.5,
            "fragmentation": 12.3,
            "dedup_ratio": 1.85
        }),
    );

    let mut metrics = HashMap::new();
    metrics.insert("cpu_usage".to_string(), 25.6);
    metrics.insert("memory_usage".to_string(), 45.2);
    metrics.insert("disk_io".to_string(), 1250.0);

    let metrics_event = create_metrics_event(metrics);

    // Broadcast events through SSE
    comm_manager
        .sse_manager
        .broadcast_event(storage_event)
        .await?;
    comm_manager
        .sse_manager
        .broadcast_event(health_event)
        .await?;
    comm_manager
        .sse_manager
        .broadcast_event(metrics_event)
        .await?;

    // Get SSE statistics
    let sse_stats = comm_manager.sse_manager.get_stats().await;
    info!("SSE Statistics: {:?}", sse_stats);

    Ok(())
}

/// Demo bidirectional RPC communication with type safety
async fn demo_bidirectional_rpc() -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up bidirectional RPC demo...");

    // Start RPC server
    let server = StreamingRpcServer::new();
    let server_addr = "127.0.0.1:8082".to_string();
    let server_clone = server.clone();

    tokio::spawn(async move {
        if let Err(e) = server_clone.start(server_addr).await {
            error!("RPC server error: {}", e);
        }
    });

    // Wait for server to start
    sleep(Duration::from_millis(500)).await;

    // Connect RPC client
    let client = StreamingRpcClient::connect("127.0.0.1:8082".to_string()).await?;

    // Test storage operations
    let storage_op = StorageOperation::CreateDataset {
        name: "demo/streaming-dataset".to_string(),
        properties: {
            let mut props = HashMap::new();
            props.insert("compression".to_string(), "lz4".to_string());
            props.insert("recordsize".to_string(), "128K".to_string());
            props
        },
    };

    let storage_result = client.execute_storage_operation(storage_op).await?;
    info!("Storage operation result: {:?}", storage_result);

    // Test ZFS operations
    let zfs_op = ZfsOperation::ListPools {
        include_status: true,
    };
    let zfs_result = client.execute_zfs_operation(zfs_op).await?;
    info!("ZFS operation result: {:?}", zfs_result);

    // Test streaming capabilities
    let event_filter = EventFilter {
        event_types: vec!["storage".to_string(), "zfs".to_string()],
        source_filter: Some("nestgate".to_string()),
        priority_filter: Some(2),
        since: Some(SystemTime::now()),
    };

    let mut event_stream = client.stream_storage_events(event_filter).await?;

    // Simulate some events and consume them
    let event_consumer = tokio::spawn(async move {
        let mut count = 0;
        while let Some(event) = event_stream.next().await {
            info!("Received streamed event: {:?}", event.event_type);
            count += 1;
            if count >= 3 {
                break;
            }
        }
    });

    // Broadcast some events to the stream
    server
        .broadcast_event(nestgate_api::streaming_rpc::StorageEvent {
            id: "event-1".to_string(),
            event_type: "storage_operation".to_string(),
            timestamp: SystemTime::now(),
            source: "demo".to_string(),
            data: json!({"operation": "create", "dataset": "demo/test"}),
            priority: 2,
        })
        .await?;

    server
        .broadcast_event(nestgate_api::streaming_rpc::StorageEvent {
            id: "event-2".to_string(),
            event_type: "zfs_operation".to_string(),
            timestamp: SystemTime::now(),
            source: "demo".to_string(),
            data: json!({"operation": "scrub", "pool": "rpool"}),
            priority: 3,
        })
        .await?;

    server
        .broadcast_event(nestgate_api::streaming_rpc::StorageEvent {
            id: "event-3".to_string(),
            event_type: "system_health".to_string(),
            timestamp: SystemTime::now(),
            source: "demo".to_string(),
            data: json!({"status": "healthy", "component": "overall"}),
            priority: 1,
        })
        .await?;

    // Wait for event processing
    timeout(Duration::from_secs(2), event_consumer).await??;

    // Test health check
    let health = client.health_check().await?;
    info!("RPC server health: {:?}", health);

    // Test service capabilities
    let capabilities = client.get_capabilities().await?;
    info!("RPC server capabilities: {:?}", capabilities);

    Ok(())
}

/// Demo WebSocket real-time communication
async fn demo_websocket_communication(
    comm_manager: &CommunicationManager,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up WebSocket communication demo...");

    // Create WebSocket events
    let ws_event1 = WebSocketEvent {
        event_id: uuid::Uuid::new_v4(),
        client_id: uuid::Uuid::new_v4(),
        event_type: WebSocketEventType::Message,
        data: json!({
            "type": "storage_update",
            "message": "New dataset created successfully",
            "dataset": "/storage/demo-dataset"
        }),
        timestamp: SystemTime::now(),
    };

    let ws_event2 = WebSocketEvent {
        event_id: uuid::Uuid::new_v4(),
        client_id: uuid::Uuid::new_v4(),
        event_type: WebSocketEventType::Message,
        data: json!({
            "type": "performance_alert",
            "message": "High CPU usage detected",
            "cpu_usage": 85.3,
            "threshold": 80.0
        }),
        timestamp: SystemTime::now(),
    };

    // Broadcast WebSocket events
    comm_manager
        .websocket_manager
        .broadcast_event(ws_event1)
        .await?;
    comm_manager
        .websocket_manager
        .broadcast_event(ws_event2)
        .await?;

    // Get WebSocket statistics
    let ws_stats = comm_manager.websocket_manager.get_stats();
    info!("WebSocket Statistics: {:?}", ws_stats);

    Ok(())
}

/// Demo MCP streaming for AI system integration
async fn demo_mcp_streaming(
    comm_manager: &CommunicationManager,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up MCP streaming demo...");

    // Create MCP streams for different purposes
    let ai_config = nestgate_api::mcp_streaming::StreamConfig {
        stream_type: StreamType::StorageMonitoring,
        buffer_size: 1000,
        compression: true,
        encryption: true,
        batch_size: 50,
        flush_interval: Duration::from_millis(100),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("purpose".to_string(), "ai_training".to_string());
            meta.insert("model_type".to_string(), "storage_predictor".to_string());
            meta
        },
    };

    let ai_stream = comm_manager
        .mcp_streaming_manager
        .create_stream(ai_config)
        .await?;
    info!("Created AI training stream: {:?}", ai_stream.id);

    let metrics_config = nestgate_api::mcp_streaming::StreamConfig {
        stream_type: StreamType::MetricsStreaming,
        buffer_size: 500,
        compression: false,
        encryption: false,
        batch_size: 25,
        flush_interval: Duration::from_secs(1),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("purpose".to_string(), "real_time_monitoring".to_string());
            meta
        },
    };

    let metrics_stream = comm_manager
        .mcp_streaming_manager
        .create_stream(metrics_config)
        .await?;
    info!("Created metrics stream: {:?}", metrics_stream.id);

    // Send data to MCP streams
    let ai_data = json!({
        "timestamp": SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        "storage_operations": [
            {"type": "read", "size": 1024, "latency": 2.5},
            {"type": "write", "size": 2048, "latency": 5.2},
            {"type": "delete", "size": 512, "latency": 1.1}
        ],
        "prediction_features": {
            "access_pattern": "sequential",
            "file_size_avg": 1500,
            "io_intensity": "medium"
        }
    });

    comm_manager
        .mcp_streaming_manager
        .send_to_stream(&ai_stream.id, ai_data)
        .await?;

    let metrics_data = json!({
        "timestamp": SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        "system_metrics": {
            "cpu_usage": 32.1,
            "memory_usage": 58.7,
            "disk_io": 2500,
            "network_io": 1200
        },
        "zfs_metrics": {
            "arc_hit_ratio": 0.92,
            "compression_ratio": 2.1,
            "dedup_ratio": 1.75
        }
    });

    comm_manager
        .mcp_streaming_manager
        .send_to_stream(&metrics_stream.id, metrics_data)
        .await?;

    // Get MCP streaming statistics
    let mcp_stats = comm_manager.mcp_streaming_manager.get_stats();
    info!("MCP Streaming Statistics: {:?}", mcp_stats);

    Ok(())
}

/// Demo unified event coordination across all communication layers
async fn demo_unified_event_coordination(
    comm_manager: &CommunicationManager,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up unified event coordination demo...");

    // Register event handlers for cross-layer coordination
    let storage_handler = EventHandler {
        id: uuid::Uuid::new_v4(),
        name: "storage_coordinator".to_string(),
        patterns: vec!["storage".to_string(), "zfs".to_string()],
        priority: Priority::High,
        active: true,
        config: json!({
            "auto_respond": true,
            "broadcast_scope": "all_layers"
        }),
    };

    let performance_handler = EventHandler {
        id: uuid::Uuid::new_v4(),
        name: "performance_monitor".to_string(),
        patterns: vec!["performance".to_string(), "metrics".to_string()],
        priority: Priority::Normal,
        active: true,
        config: json!({
            "threshold_alerts": true,
            "metric_aggregation": true
        }),
    };

    comm_manager
        .event_coordinator
        .register_handler(storage_handler)
        .await?;
    comm_manager
        .event_coordinator
        .register_handler(performance_handler)
        .await?;

    // Create coordinated events that will be distributed across all layers
    let storage_event = CoordinatedEvent {
        event_id: uuid::Uuid::new_v4(),
        event_type: CoordinatedEventType::StorageOperation,
        source: "demo_coordinator".to_string(),
        data: json!({
            "operation": "large_dataset_import",
            "dataset": "/storage/ml-training-data",
            "size": "500GB",
            "estimated_time": "2h",
            "priority": "high"
        }),
        timestamp: SystemTime::now(),
    };

    let health_event = CoordinatedEvent {
        event_id: uuid::Uuid::new_v4(),
        event_type: CoordinatedEventType::HealthMonitoring,
        source: "system_monitor".to_string(),
        data: json!({
            "component": "storage_subsystem",
            "status": "optimal",
            "metrics": {
                "iops": 15000,
                "throughput": "2.5GB/s",
                "latency": "0.8ms"
            }
        }),
        timestamp: SystemTime::now(),
    };

    // Emit coordinated events
    comm_manager
        .event_coordinator
        .emit_event(storage_event)
        .await?;
    comm_manager
        .event_coordinator
        .emit_event(health_event)
        .await?;

    // Get event coordination statistics
    let event_stats = comm_manager.event_coordinator.get_stats().await;
    info!("Event Coordination Statistics: {:?}", event_stats);

    Ok(())
}

/// Demo performance monitoring and metrics collection
async fn demo_performance_monitoring(
    comm_manager: &CommunicationManager,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up performance monitoring demo...");

    // Collect performance metrics from all communication layers
    let ws_stats = comm_manager.websocket_manager.get_stats();
    let sse_stats = comm_manager.sse_manager.get_stats().await;
    let mcp_stats = comm_manager.mcp_streaming_manager.get_stats();
    let event_stats = comm_manager.event_coordinator.get_stats().await;

    // Create comprehensive performance report
    let performance_report = json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "communication_performance": {
            "websocket": {
                "active_connections": ws_stats.active_connections,
                "total_connections": ws_stats.total_connections,
                "messages_sent": ws_stats.messages_sent,
                "messages_received": ws_stats.messages_received,
                "bytes_transferred": ws_stats.bytes_transferred,
                "error_rate": if ws_stats.messages_sent > 0 {
                    (ws_stats.errors as f64 / ws_stats.messages_sent as f64) * 100.0
                } else { 0.0 }
            },
            "sse": {
                "active_connections": sse_stats.active_connections,
                "total_connections": sse_stats.total_connections,
                "events_sent": sse_stats.events_sent,
                "bytes_transferred": sse_stats.bytes_transferred,
                "error_rate": if sse_stats.events_sent > 0 {
                    (sse_stats.errors as f64 / sse_stats.events_sent as f64) * 100.0
                } else { 0.0 }
            },
            "mcp_streaming": {
                "total_streams": mcp_stats.total_streams,
                "active_streams": mcp_stats.active_streams,
                "messages_sent": mcp_stats.messages_sent,
                "messages_received": mcp_stats.messages_received,
                "bytes_transferred": mcp_stats.bytes_transferred
            },
            "event_coordination": {
                "total_events": event_stats.total_events,
                "events_processed": event_stats.events_processed,
                "active_handlers": event_stats.active_handlers,
                "error_rate": if event_stats.total_events > 0 {
                    (event_stats.errors as f64 / event_stats.total_events as f64) * 100.0
                } else { 0.0 }
            }
        },
        "overall_metrics": {
            "total_active_connections": ws_stats.active_connections + sse_stats.active_connections,
            "total_messages_processed": ws_stats.messages_sent + sse_stats.events_sent + mcp_stats.messages_sent,
            "total_bytes_transferred": ws_stats.bytes_transferred + sse_stats.bytes_transferred + mcp_stats.bytes_transferred,
            "communication_layers_active": 4,
            "system_health": "optimal"
        }
    });

    info!(
        "Performance Report: {}",
        serde_json::to_string_pretty(&performance_report)?
    );

    // Broadcast performance report to all communication layers
    comm_manager.broadcast_event(performance_report).await?;

    Ok(())
}

/// Demo full integration workflow showcasing all capabilities
async fn demo_full_integration_workflow(
    comm_manager: &CommunicationManager,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting full integration workflow demo...");

    // Simulate a complex workflow that uses all communication layers

    // 1. Initiate storage operation via coordinated event
    let workflow_event = CoordinatedEvent {
        event_id: uuid::Uuid::new_v4(),
        event_type: CoordinatedEventType::StorageOperation,
        source: "workflow_orchestrator".to_string(),
        data: json!({
            "workflow_id": "wf-2024-001",
            "operation": "distributed_backup",
            "datasets": ["/storage/critical-data", "/storage/user-data"],
            "target": "remote_cluster",
            "priority": "critical",
            "estimated_duration": "45m"
        }),
        timestamp: SystemTime::now(),
    };

    comm_manager
        .event_coordinator
        .emit_event(workflow_event)
        .await?;

    // 2. Stream real-time progress via SSE
    let progress_events = vec![
        ("Initializing backup workflow", 10.0),
        ("Preparing dataset snapshots", 25.0),
        ("Starting data transfer", 40.0),
        ("Verifying data integrity", 70.0),
        ("Finalizing backup", 90.0),
        ("Backup completed successfully", 100.0),
    ];

    for (message, progress) in progress_events {
        let progress_event = SseEvent {
            id: uuid::Uuid::new_v4(),
            event_type: SseEventType::StorageOperation,
            data: json!({
                "workflow_id": "wf-2024-001",
                "message": message,
                "progress_percentage": progress,
                "timestamp": SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
            }),
            timestamp: SystemTime::now(),
            source: "backup_service".to_string(),
            priority: EventPriority::High,
        };

        comm_manager
            .sse_manager
            .broadcast_event(progress_event)
            .await?;
        sleep(Duration::from_millis(200)).await;
    }

    // 3. Stream detailed metrics to MCP for AI analysis
    let ai_analytics_data = json!({
        "workflow_id": "wf-2024-001",
        "performance_metrics": {
            "total_data_transferred": "2.5TB",
            "transfer_rate_avg": "850MB/s",
            "compression_ratio": 2.3,
            "dedup_savings": "15%",
            "network_utilization": "78%"
        },
        "optimization_opportunities": {
            "suggested_compression": "zstd",
            "optimal_block_size": "1MB",
            "parallel_streams": 8
        }
    });

    // Create MCP stream for AI analytics
    let analytics_config = nestgate_api::mcp_streaming::StreamConfig {
        stream_type: StreamType::StateSynchronization,
        buffer_size: 100,
        compression: true,
        encryption: true,
        batch_size: 10,
        flush_interval: Duration::from_millis(50),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert(
                "analytics_type".to_string(),
                "workflow_optimization".to_string(),
            );
            meta
        },
    };

    let analytics_stream = comm_manager
        .mcp_streaming_manager
        .create_stream(analytics_config)
        .await?;
    comm_manager
        .mcp_streaming_manager
        .send_to_stream(&analytics_stream.id, ai_analytics_data)
        .await?;

    // 4. Send real-time alerts via WebSocket
    let alert_event = WebSocketEvent {
        event_id: uuid::Uuid::new_v4(),
        client_id: uuid::Uuid::new_v4(),
        event_type: WebSocketEventType::Message,
        data: json!({
            "alert_type": "workflow_completion",
            "workflow_id": "wf-2024-001",
            "status": "success",
            "summary": "Distributed backup completed successfully",
            "next_actions": ["verify_backup_integrity", "update_recovery_plan", "schedule_next_backup"]
        }),
        timestamp: SystemTime::now(),
    };

    comm_manager
        .websocket_manager
        .broadcast_event(alert_event)
        .await?;

    // 5. Generate final workflow report and broadcast to all layers
    let workflow_summary = json!({
        "workflow_id": "wf-2024-001",
        "status": "completed",
        "completion_time": chrono::Utc::now().to_rfc3339(),
        "performance_summary": {
            "total_duration": "42m 15s",
            "data_processed": "2.5TB",
            "average_throughput": "1.02GB/s",
            "efficiency_rating": "excellent"
        },
        "communication_layer_usage": {
            "sse_events_sent": 6,
            "websocket_alerts": 1,
            "mcp_analytics_streams": 1,
            "coordinated_events": 2
        },
        "success_metrics": {
            "data_integrity": "100%",
            "transfer_success": "100%",
            "compression_efficiency": "230%",
            "deduplication_savings": "15%"
        }
    });

    // Broadcast final summary to all communication layers
    comm_manager.broadcast_event(workflow_summary).await?;

    info!("Full integration workflow completed successfully!");

    // Final statistics report
    info!("=== FINAL COMMUNICATION STATISTICS ===");
    info!(
        "WebSocket: {:?}",
        comm_manager.websocket_manager.get_stats()
    );
    info!("SSE: {:?}", comm_manager.sse_manager.get_stats().await);
    info!(
        "MCP Streaming: {:?}",
        comm_manager.mcp_streaming_manager.get_stats()
    );
    info!(
        "Event Coordination: {:?}",
        comm_manager.event_coordinator.get_stats().await
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_communication_layers() {
        let comm_manager = CommunicationManager::new();

        // Test event broadcasting
        let test_event = json!({
            "test": "integration_test",
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        assert!(comm_manager.broadcast_event(test_event).await.is_ok());
    }

    #[tokio::test]
    async fn test_sse_functionality() {
        let sse_manager = SseManager::new();

        let test_event = SseEvent {
            id: uuid::Uuid::new_v4(),
            event_type: SseEventType::SystemEvent,
            data: json!({"test": "sse_test"}),
            timestamp: SystemTime::now(),
            source: "test".to_string(),
            priority: EventPriority::Normal,
        };

        assert!(sse_manager.broadcast_event(test_event).await.is_ok());
    }

    #[tokio::test]
    async fn test_streaming_rpc() {
        let server = StreamingRpcServer::new();

        let health = server
            .health_check(tarpc::context::Context::current())
            .await;
        assert!(health.is_ok());

        let capabilities = server
            .get_capabilities(tarpc::context::Context::current())
            .await;
        assert!(capabilities.is_ok());
        assert!(capabilities.unwrap().bidirectional_support);
    }
    }
}

#[cfg(not(feature = "streaming-rpc"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Enhanced streaming demo is only available with streaming-rpc feature");
    println!("Please enable the streaming-rpc feature to run this demo");
    Ok(())
}
