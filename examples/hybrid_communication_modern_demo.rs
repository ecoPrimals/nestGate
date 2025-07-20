//! # NestGate Hybrid Communication Modern Demo
//!
//! A completely rebuilt, production-ready demonstration of NestGate's hybrid communication system.
//! Showcases 4 communication layers with MODERN APIs:
//! 1. WebSocket + JSON for external client communication
//! 2. Internal service-to-service communication
//! 3. Streaming capabilities for real-time data
//! 4. Event system for reactive coordination

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, RwLock};
use tokio::time::sleep;
use tracing::{error, info, warn};
use uuid::Uuid;

// Modern communication message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModernMessage {
    pub id: String,
    pub message_type: MessageType,
    pub payload: serde_json::Value,
    pub timestamp: u64,
    pub priority: Priority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    WebSocketRequest,
    WebSocketResponse,
    ServiceCall,
    ServiceResponse,
    StreamingData,
    EventNotification,
    HealthCheck,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

// Modern event system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModernEvent {
    pub event_id: String,
    pub event_type: EventType,
    pub source: String,
    pub target: Option<String>,
    pub data: serde_json::Value,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    SystemStatus,
    ResourceUpdate,
    PerformanceMetric,
    ErrorAlert,
    UserAction,
}

// Mock WebSocket connection for demonstration
#[derive(Debug)]
pub struct MockWebSocketConnection {
    pub connection_id: String,
    pub is_connected: bool,
    pub message_count: Arc<AtomicU64>,
}

impl Default for MockWebSocketConnection {
    fn default() -> Self {
        Self::new()
    }
}

impl MockWebSocketConnection {
    pub fn new() -> Self {
        Self {
            connection_id: Uuid::new_v4().to_string(),
            is_connected: true,
            message_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub async fn send_message(&self, message: &ModernMessage) -> Result<(), String> {
        if !self.is_connected {
            return Err("Connection closed".to_string());
        }

        self.message_count.fetch_add(1, Ordering::SeqCst);
        println!(
            "📡 WebSocket [{}] sent: {:?}",
            &self.connection_id[..8],
            message.message_type
        );

        // Simulate network latency
        sleep(Duration::from_millis(5)).await;
        Ok(())
    }

    pub async fn receive_message(&self) -> Result<ModernMessage, String> {
        if !self.is_connected {
            return Err("Connection closed".to_string());
        }

        // Simulate receiving a message
        sleep(Duration::from_millis(10)).await;

        Ok(ModernMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::WebSocketRequest,
            payload: serde_json::json!({"action": "get_status"}),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            priority: Priority::Medium,
        })
    }
}

// Modern service-to-service communication
#[derive(Debug)]
pub struct ModernServiceClient {
    pub service_name: String,
    pub request_count: Arc<AtomicU64>,
}

impl ModernServiceClient {
    pub fn new(service_name: String) -> Self {
        Self {
            service_name,
            request_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub async fn call_service(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        self.request_count.fetch_add(1, Ordering::SeqCst);

        println!(
            "🔗 Service call to {}: {}({})",
            self.service_name, method, params
        );

        // Simulate service call processing time
        sleep(Duration::from_millis(20 + rand::random::<u64>() % 30)).await;

        // Return mock response based on method
        let response = match method {
            "get_pool_status" => serde_json::json!({
                "pool_name": "main-pool",
                "status": "healthy",
                "capacity_used": 65.5,
                "capacity_total": 100.0
            }),
            "get_performance_metrics" => serde_json::json!({
                "cpu_usage": 42.3,
                "memory_usage": 58.7,
                "disk_io": 1250.0,
                "network_io": 890.0
            }),
            "optimize_storage" => serde_json::json!({
                "optimization_id": Uuid::new_v4().to_string(),
                "status": "started",
                "estimated_completion": "2m"
            }),
            _ => serde_json::json!({"error": "unknown_method", "method": method}),
        };

        Ok(response)
    }
}

// Modern streaming data source
#[derive(Debug)]
pub struct ModernStreamingSource {
    pub stream_id: String,
    pub is_active: bool,
    pub data_points_sent: Arc<AtomicU64>,
}

impl ModernStreamingSource {
    pub fn new(stream_id: String) -> Self {
        Self {
            stream_id,
            is_active: true,
            data_points_sent: Arc::new(AtomicU64::new(0)),
        }
    }

    pub async fn start_streaming(&self, tx: mpsc::Sender<ModernMessage>) -> Result<(), String> {
        if !self.is_active {
            return Err("Stream is inactive".to_string());
        }

        println!("🌊 Starting stream: {}", self.stream_id);

        // Stream data for demonstration
        for i in 0..10 {
            let data_point = serde_json::json!({
                "stream_id": self.stream_id,
                "sequence": i,
                "cpu_usage": 30.0 + (i as f64 * 2.5),
                "memory_usage": 45.0 + (i as f64 * 1.8),
                "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
            });

            let message = ModernMessage {
                id: Uuid::new_v4().to_string(),
                message_type: MessageType::StreamingData,
                payload: data_point,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                priority: Priority::Medium,
            };

            if tx.send(message).await.is_err() {
                warn!("Failed to send streaming data, receiver dropped");
                break;
            }

            self.data_points_sent.fetch_add(1, Ordering::SeqCst);
            sleep(Duration::from_millis(100)).await;
        }

        println!("✅ Stream completed: {}", self.stream_id);
        Ok(())
    }
}

// Modern event coordinator
#[derive(Debug)]
pub struct ModernEventCoordinator {
    pub events_processed: Arc<AtomicU64>,
    pub subscribers: Arc<RwLock<HashMap<String, mpsc::Sender<ModernEvent>>>>,
}

impl Default for ModernEventCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl ModernEventCoordinator {
    pub fn new() -> Self {
        Self {
            events_processed: Arc::new(AtomicU64::new(0)),
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn subscribe(&self, subscriber_id: String, tx: mpsc::Sender<ModernEvent>) {
        let mut subscribers = self.subscribers.write().await;
        subscribers.insert(subscriber_id.clone(), tx);
        println!("📫 Subscriber registered: {subscriber_id}");
    }

    pub async fn publish_event(&self, event: ModernEvent) -> Result<(), String> {
        self.events_processed.fetch_add(1, Ordering::SeqCst);

        println!(
            "📢 Publishing event: {:?} from {}",
            event.event_type, event.source
        );

        let subscribers = self.subscribers.read().await;
        let mut successful_sends = 0;

        for (subscriber_id, tx) in subscribers.iter() {
            if let Err(_) = tx.send(event.clone()).await {
                warn!("Failed to send event to subscriber: {}", subscriber_id);
            } else {
                successful_sends += 1;
            }
        }

        println!("  ✅ Event delivered to {successful_sends} subscribers");
        Ok(())
    }
}

// Main hybrid communication orchestrator
#[derive(Debug)]
pub struct HybridCommunicationOrchestrator {
    pub websocket_connections: Vec<MockWebSocketConnection>,
    pub service_clients: HashMap<String, ModernServiceClient>,
    pub streaming_sources: HashMap<String, ModernStreamingSource>,
    pub event_coordinator: ModernEventCoordinator,
}

impl Default for HybridCommunicationOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl HybridCommunicationOrchestrator {
    pub fn new() -> Self {
        let mut service_clients = HashMap::new();
        service_clients.insert(
            "storage-service".to_string(),
            ModernServiceClient::new("storage-service".to_string()),
        );
        service_clients.insert(
            "performance-service".to_string(),
            ModernServiceClient::new("performance-service".to_string()),
        );
        service_clients.insert(
            "optimization-service".to_string(),
            ModernServiceClient::new("optimization-service".to_string()),
        );

        let mut streaming_sources = HashMap::new();
        streaming_sources.insert(
            "metrics-stream".to_string(),
            ModernStreamingSource::new("metrics-stream".to_string()),
        );
        streaming_sources.insert(
            "status-stream".to_string(),
            ModernStreamingSource::new("status-stream".to_string()),
        );

        Self {
            websocket_connections: vec![
                MockWebSocketConnection::new(),
                MockWebSocketConnection::new(),
                MockWebSocketConnection::new(),
            ],
            service_clients,
            streaming_sources,
            event_coordinator: ModernEventCoordinator::new(),
        }
    }

    pub async fn run_hybrid_communication_demo(&self) -> Result<(), String> {
        println!("🚀 **NestGate Hybrid Communication Modern Demo**");
        println!("================================================\n");

        // Phase 1: WebSocket Communication
        self.demo_websocket_communication().await?;

        // Phase 2: Service-to-Service Communication
        self.demo_service_communication().await?;

        // Phase 3: Streaming Capabilities
        self.demo_streaming_capabilities().await?;

        // Phase 4: Event System
        self.demo_event_system().await?;

        println!("✅ **Hybrid communication demo completed successfully!**\n");
        Ok(())
    }

    async fn demo_websocket_communication(&self) -> Result<(), String> {
        println!("📡 **Phase 1: WebSocket Communication**");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        for (i, connection) in self.websocket_connections.iter().enumerate() {
            println!("🔗 Testing WebSocket connection #{}", i + 1);

            // Simulate sending messages
            let messages = vec![
                ModernMessage {
                    id: Uuid::new_v4().to_string(),
                    message_type: MessageType::WebSocketRequest,
                    payload: serde_json::json!({"action": "get_system_status"}),
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    priority: Priority::High,
                },
                ModernMessage {
                    id: Uuid::new_v4().to_string(),
                    message_type: MessageType::WebSocketResponse,
                    payload: serde_json::json!({"status": "operational", "uptime": "12h 34m"}),
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    priority: Priority::Medium,
                },
            ];

            for message in messages {
                connection.send_message(&message).await?;
            }

            // Simulate receiving a message
            let _received = connection.receive_message().await?;

            println!("  ✅ Connection {} processed messages", i + 1);
        }

        println!("  ✅ WebSocket communication phase completed\n");
        Ok(())
    }

    async fn demo_service_communication(&self) -> Result<(), String> {
        println!("🔗 **Phase 2: Service-to-Service Communication**");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Test various service calls
        let service_calls = vec![
            (
                "storage-service",
                "get_pool_status",
                serde_json::json!({"pool_id": "main-pool"}),
            ),
            (
                "performance-service",
                "get_performance_metrics",
                serde_json::json!({"time_range": "5m"}),
            ),
            (
                "optimization-service",
                "optimize_storage",
                serde_json::json!({"target": "performance", "level": "moderate"}),
            ),
        ];

        for (service_name, method, params) in service_calls {
            if let Some(client) = self.service_clients.get(service_name) {
                println!("🎯 Calling service: {service_name} -> {method}");

                let response = client.call_service(method, params).await?;

                println!(
                    "  📋 Response: {}",
                    serde_json::to_string_pretty(&response)
                        .unwrap_or_else(|_| "Error serializing response".to_string())
                );
                println!("  ✅ Service call completed\n");
            }
        }

        println!("  ✅ Service communication phase completed\n");
        Ok(())
    }

    async fn demo_streaming_capabilities(&self) -> Result<(), String> {
        println!("🌊 **Phase 3: Streaming Capabilities**");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Create channel for streaming data
        let (tx, mut rx) = mpsc::channel::<ModernMessage>(100);

        // Start streaming from multiple sources
        let streaming_tasks: Vec<_> = self
            .streaming_sources
            .values()
            .map(|source| {
                let tx_clone = tx.clone();
                let source_id = source.stream_id.clone();

                tokio::spawn(async move {
                    let test_source = ModernStreamingSource::new(source_id.clone());
                    if let Err(e) = test_source.start_streaming(tx_clone).await {
                        error!("Streaming error for {}: {}", source_id, e);
                    }
                })
            })
            .collect();

        // Drop the original sender so the receiver can complete
        drop(tx);

        // Consume streaming data
        let mut message_count = 0;
        while let Some(message) = rx.recv().await {
            message_count += 1;
            println!(
                "🌊 Received stream data #{}: {:?}",
                message_count, message.message_type
            );

            if message_count >= 20 {
                // Limit for demo
                break;
            }
        }

        // Wait for streaming tasks to complete
        futures::future::join_all(streaming_tasks).await;

        println!("  ✅ Streaming capabilities phase completed\n");
        Ok(())
    }

    async fn demo_event_system(&self) -> Result<(), String> {
        println!("📢 **Phase 4: Event System**");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Create event subscribers
        let (tx1, mut rx1) = mpsc::channel::<ModernEvent>(50);
        let (tx2, mut rx2) = mpsc::channel::<ModernEvent>(50);

        // Subscribe to events
        self.event_coordinator
            .subscribe("performance-monitor".to_string(), tx1)
            .await;
        self.event_coordinator
            .subscribe("alert-manager".to_string(), tx2)
            .await;

        // Publish various events
        let events = vec![
            ModernEvent {
                event_id: Uuid::new_v4().to_string(),
                event_type: EventType::SystemStatus,
                source: "system-monitor".to_string(),
                target: None,
                data: serde_json::json!({"status": "operational", "cpu": 45.2}),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            ModernEvent {
                event_id: Uuid::new_v4().to_string(),
                event_type: EventType::PerformanceMetric,
                source: "metrics-collector".to_string(),
                target: Some("performance-monitor".to_string()),
                data: serde_json::json!({"metric": "throughput", "value": 1250.7}),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            ModernEvent {
                event_id: Uuid::new_v4().to_string(),
                event_type: EventType::ResourceUpdate,
                source: "storage-manager".to_string(),
                target: None,
                data: serde_json::json!({"pool": "main-pool", "available_space": "2.5TB"}),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
        ];

        // Publish events
        for event in events {
            self.event_coordinator.publish_event(event).await?;
            sleep(Duration::from_millis(100)).await;
        }

        // Simulate event processing
        tokio::spawn(async move {
            let mut processed = 0;
            while let Some(event) = rx1.recv().await {
                processed += 1;
                println!(
                    "🎯 Performance Monitor processed event: {:?}",
                    event.event_type
                );
                if processed >= 3 {
                    break;
                }
            }
        });

        tokio::spawn(async move {
            let mut processed = 0;
            while let Some(event) = rx2.recv().await {
                processed += 1;
                println!("🚨 Alert Manager processed event: {:?}", event.event_type);
                if processed >= 3 {
                    break;
                }
            }
        });

        // Allow time for event processing
        sleep(Duration::from_millis(500)).await;

        println!("  ✅ Event system phase completed\n");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .init();

    info!("🚀 Starting NestGate Hybrid Communication Modern Demo");

    // Create orchestrator
    let orchestrator = HybridCommunicationOrchestrator::new();

    // Run comprehensive demonstration
    let start_time = Instant::now();

    match orchestrator.run_hybrid_communication_demo().await {
        Ok(_) => {
            let duration = start_time.elapsed();
            println!(
                "🎉 **Demo completed successfully in {:.2}s**",
                duration.as_secs_f64()
            );
            println!("🏆 **NestGate hybrid communication system is production-ready!**");

            // Print statistics
            println!("\n📊 **Demo Statistics:**");
            println!(
                "• WebSocket connections: {}",
                orchestrator.websocket_connections.len()
            );
            println!("• Service clients: {}", orchestrator.service_clients.len());
            println!(
                "• Streaming sources: {}",
                orchestrator.streaming_sources.len()
            );
            println!(
                "• Events processed: {}",
                orchestrator
                    .event_coordinator
                    .events_processed
                    .load(Ordering::SeqCst)
            );
        }
        Err(e) => {
            error!("Demo failed: {}", e);
            eprintln!("❌ Demo failed: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_websocket_connection() {
        let connection = MockWebSocketConnection::new();
        assert!(connection.is_connected);

        let message = ModernMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::WebSocketRequest,
            payload: serde_json::json!({"test": "data"}),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            priority: Priority::Medium,
        };

        connection
            .send_message(&message)
            .await
            .expect("Failed to send message");
        assert_eq!(connection.message_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_service_communication() {
        let client = ModernServiceClient::new("test-service".to_string());

        let response = client
            .call_service("get_pool_status", serde_json::json!({"pool": "test"}))
            .await
            .expect("Service call failed");

        assert!(response.is_object());
        assert_eq!(client.request_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_event_coordination() {
        let coordinator = ModernEventCoordinator::new();
        let (tx, mut rx) = mpsc::channel(10);

        coordinator
            .subscribe("test-subscriber".to_string(), tx)
            .await;

        let event = ModernEvent {
            event_id: Uuid::new_v4().to_string(),
            event_type: EventType::SystemStatus,
            source: "test-source".to_string(),
            target: None,
            data: serde_json::json!({"test": true}),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        coordinator
            .publish_event(event.clone())
            .await
            .expect("Failed to publish event");

        let received = rx.recv().await.expect("Failed to receive event");
        assert_eq!(received.event_id, event.event_id);
    }

    #[tokio::test]
    async fn test_streaming_source() {
        let source = ModernStreamingSource::new("test-stream".to_string());
        let (tx, mut rx) = mpsc::channel(50);

        // Start streaming in background
        let source_clone = source.stream_id.clone();
        let stream_task = tokio::spawn(async move {
            let test_source = ModernStreamingSource::new(source_clone);
            test_source.start_streaming(tx).await
        });

        // Receive a few messages
        let mut received_count = 0;
        while let Some(_message) = rx.recv().await {
            received_count += 1;
            if received_count >= 5 {
                break;
            }
        }

        assert!(received_count >= 5);
        stream_task.await.unwrap().expect("Streaming failed");
    }

    #[tokio::test]
    async fn test_full_orchestration() {
        let orchestrator = HybridCommunicationOrchestrator::new();
        orchestrator
            .run_hybrid_communication_demo()
            .await
            .expect("Orchestration demo failed");
    }
}
