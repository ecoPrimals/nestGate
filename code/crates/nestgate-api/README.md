# NestGate API - Enhanced Streaming & Bidirectional Communication

**Advanced API server with comprehensive real-time communication capabilities**

## 🌟 **Features Overview**

NestGate API provides a complete communication infrastructure supporting:

- **🌐 HTTP REST API** - Traditional request/response patterns
- **⚡ Server-Sent Events (SSE)** - Real-time server-to-client streaming  
- **🔄 WebSocket Communication** - Bidirectional real-time messaging
- **🚀 Streaming RPC** - Type-safe bidirectional RPC with tarpc
- **🤖 MCP Protocol Extensions** - AI system integration streaming
- **📡 Event Coordination** - Unified event system across all layers

## 🏗️ **Architecture Overview**

```
┌─────────────────────────────────────────────────────────────────┐
│                     NestGate API Gateway                        │
├─────────────────────────────────────────────────────────────────┤
│  HTTP REST API  │  SSE Streaming  │  WebSocket  │  Streaming RPC │
├─────────────────────────────────────────────────────────────────┤
│                    Event Coordination Layer                     │
├─────────────────────────────────────────────────────────────────┤
│          MCP Streaming          │       Communication           │
│          Extensions             │       Manager                 │
├─────────────────────────────────────────────────────────────────┤
│                      NestGate Core Services                     │
└─────────────────────────────────────────────────────────────────┘
```

## 🚀 **Quick Start**

### Basic Server Setup

```rust
use nestgate_api::{start_server, CommunicationManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start with all communication layers
    let comm_manager = CommunicationManager::new();
    comm_manager.start_all("127.0.0.1:8080", "127.0.0.1:8081").await?;
    Ok(())
}
```

### Client Integration

```rust
use nestgate_api_client::NestGateStreamingClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NestGateStreamingClient::new("http://127.0.0.1:8080".to_string()).await?;
    client.connect_all().await?;
    
    // Subscribe to real-time events
    let mut events = client.get_event_stream();
    while let Ok(event) = events.recv().await {
        println!("Received: {:?}", event);
    }
    
    Ok(())
}
```

## 📡 **Communication Layers**

### 1. Server-Sent Events (SSE)

**Real-time server-to-client streaming for live updates**

#### Endpoints
```
GET /api/v1/sse/events    - All event types
GET /api/v1/sse/storage   - Storage operations  
GET /api/v1/sse/health    - System health
GET /api/v1/sse/metrics   - Performance metrics
```

#### Example Usage
```javascript
// JavaScript client
const eventSource = new EventSource('/api/v1/sse/storage');
eventSource.onmessage = function(event) {
    const data = JSON.parse(event.data);
    console.log('Storage event:', data);
};
```

```rust
// Rust client
let mut event_stream = client.create_storage_stream().await;
while let Some(event) = event_stream.next().await {
    println!("Storage event: {:?}", event);
}
```

#### Event Types
- **Storage Operations**: Dataset creation, snapshots, transfers
- **System Health**: Component status, health metrics
- **Performance Metrics**: CPU, memory, disk, network stats
- **Hardware Tuning**: Optimization events and results

### 2. WebSocket Communication

**Bidirectional real-time messaging**

#### Connection
```
ws://localhost:8080/api/v1/communication/websocket
```

#### Message Format
```json
{
    "event_id": "uuid",
    "event_type": "storage_operation",
    "data": {
        "operation": "create_dataset",
        "dataset": "/storage/my-data",
        "status": "success"
    },
    "timestamp": 1704067200
}
```

#### Example Usage
```rust
// Send WebSocket message
client.send_websocket_message(json!({
    "type": "storage_request",
    "operation": "list_datasets",
    "filters": {"pool": "main"}
})).await?;
```

### 3. Streaming RPC (tarpc)

**Type-safe bidirectional RPC with streaming support**

#### Connection
```rust
use nestgate_api::streaming_rpc::StreamingRpcClient;

let client = StreamingRpcClient::connect("127.0.0.1:8081".to_string()).await?;
```

#### Operations
```rust
// Storage operations
let result = client.execute_storage_operation(
    StorageOperation::CreateDataset {
        name: "my/dataset".to_string(),
        properties: HashMap::new(),
    }
).await?;

// ZFS operations  
let pools = client.execute_zfs_operation(
    ZfsOperation::ListPools { include_status: true }
).await?;

// Streaming events
let mut event_stream = client.stream_storage_events(EventFilter {
    event_types: vec!["storage".to_string()],
    source_filter: Some("nestgate".to_string()),
    priority_filter: None,
    since: Some(SystemTime::now()),
}).await?;

while let Some(event) = event_stream.next().await {
    println!("RPC Event: {:?}", event);
}
```

#### Bidirectional Streaming
```rust
// Create bidirectional stream
let (client_tx, client_rx) = tokio::sync::mpsc::unbounded_channel();
let client_stream = tokio_stream::wrappers::UnboundedReceiverStream::new(client_rx);

let mut server_stream = client.bidirectional_stream(Box::pin(client_stream)).await?;

// Send client messages
client_tx.send(ClientMessage::Command {
    id: "cmd-1".to_string(),
    command: Command::StorageCommand(StorageOperation::ListDatasets {
        pool: None,
        recursive: true,
    }),
})?;

// Process server responses
while let Some(response) = server_stream.next().await {
    match response {
        ServerMessage::CommandResult { id, result } => {
            println!("Command {} result: {:?}", id, result);
        }
        ServerMessage::Event { subscription_id, event } => {
            println!("Event for {}: {:?}", subscription_id, event);
        }
        _ => {}
    }
}
```

### 4. MCP Protocol Extensions

**AI system integration with streaming capabilities**

```rust
// Create MCP stream for AI training
let ai_config = StreamConfig {
    stream_type: StreamType::StorageMonitoring,
    buffer_size: 1000,
    compression: true,
    encryption: true,
    batch_size: 50,
    flush_interval: Duration::from_millis(100),
    metadata: HashMap::from([
        ("purpose".to_string(), "ai_training".to_string()),
        ("model_type".to_string(), "storage_predictor".to_string()),
    ]),
};

let stream = mcp_manager.create_stream(ai_config).await?;

// Send training data
let training_data = json!({
    "storage_operations": [...],
    "performance_metrics": {...},
    "prediction_features": {...}
});

mcp_manager.send_to_stream(&stream.id, training_data).await?;
```

## 🎯 **Event Coordination**

The event coordination system unifies events across all communication layers:

```rust
// Register event handler
let handler = EventHandler {
    id: uuid::Uuid::new_v4(),
    name: "storage_coordinator".to_string(),
    patterns: vec!["storage".to_string(), "zfs".to_string()],
    priority: Priority::High,
    active: true,
    config: json!({"auto_respond": true}),
};

event_coordinator.register_handler(handler).await?;

// Emit coordinated event
let event = CoordinatedEvent {
    event_id: uuid::Uuid::new_v4(),
    event_type: CoordinatedEventType::StorageOperation,
    source: "api_server".to_string(),
    data: json!({
        "operation": "backup_started",
        "dataset": "/storage/critical-data"
    }),
    timestamp: SystemTime::now(),
};

event_coordinator.emit_event(event).await?;
```

## 📊 **Monitoring & Statistics**

### Communication Statistics
```
GET /api/v1/communication/stats
```

```json
{
    "websocket": {
        "active_connections": 15,
        "total_connections": 42,
        "messages_sent": 1250,
        "messages_received": 800,
        "bytes_transferred": 2048576
    },
    "sse": {
        "active_connections": 8,
        "events_sent": 5000,
        "bytes_transferred": 1048576
    },
    "mcp_streaming": {
        "active_streams": 5,
        "total_streams": 12,
        "messages_sent": 15000
    },
    "event_coordination": {
        "total_events": 2500,
        "events_processed": 2500,
        "active_handlers": 10
    }
}
```

### Health Check
```
GET /health
```

```json
{
    "status": "ok",
    "service": "nestgate-api",
    "version": "0.1.0",
    "communication_layers": {
        "websocket": true,
        "sse": true,
        "streaming_rpc": true,
        "mcp_streaming": true,
        "event_coordination": true
    }
}
```

## 🔧 **Configuration**

### Feature Flags
```toml
[features]
default = ["sse", "streaming-rpc", "full-communication"]

# SSE streaming support
sse = ["axum/sse"]

# Enhanced RPC streaming
streaming-rpc = ["tarpc", "tokio-stream"]

# Full communication suite
full-communication = ["sse", "streaming-rpc"]
```

### Environment Variables
```bash
# Server configuration
NESTGATE_HTTP_ADDR=127.0.0.1:8080
NESTGATE_RPC_ADDR=127.0.0.1:8081

# Communication settings
NESTGATE_MAX_CONNECTIONS=1000
NESTGATE_EVENT_BUFFER_SIZE=10000
NESTGATE_COMPRESSION_ENABLED=true

# Security settings  
NESTGATE_TLS_ENABLED=false
NESTGATE_AUTH_REQUIRED=false
```

## 🔒 **Security Features**

- **TLS/SSL Support**: Secure all communication layers
- **Authentication**: Bearer token and API key support
- **Authorization**: Role-based access control
- **Rate Limiting**: Per-client connection and message limits
- **Input Validation**: Comprehensive message validation
- **Event Encryption**: Optional encryption for sensitive events

## 🚀 **Performance Optimizations**

- **Connection Pooling**: Efficient connection management
- **Message Batching**: Reduce overhead for high-frequency events
- **Compression**: Optional compression for large payloads
- **Backpressure Handling**: Graceful handling of slow clients
- **Memory Management**: Efficient buffer management
- **Async Processing**: Non-blocking event processing

## 📚 **Examples**

### Full Integration Demo
```bash
cargo run --example enhanced_streaming_demo
```

### Client Implementation
```bash
cargo run --example streaming_client_demo
```

### Simple Usage
```bash
cargo run --example hybrid_communication_simple_demo
```

## 🔗 **API Reference**

### Storage Operations
- `POST /api/v1/storage/deployments` - Create storage deployment
- `GET /api/v1/storage/deployments/{id}` - Get deployment status
- `DELETE /api/v1/storage/deployments/{id}` - Delete deployment

### Workspace Management
- `POST /api/v1/workspaces` - Create workspace
- `GET /api/v1/workspaces/{id}` - Get workspace details
- `POST /api/v1/workspaces/{id}/deploy` - Deploy workspace

### Hardware Tuning
- `POST /api/v1/hardware/tune` - Start auto-tuning
- `GET /api/v1/hardware/config` - Get current configuration

## 🤝 **Contributing**

1. Fork the repository
2. Create a feature branch
3. Add tests for new communication features
4. Ensure all tests pass
5. Submit a pull request

## 📄 **License**

AGPL-3.0-or-later - See LICENSE file for details

## 🆘 **Support**

- **Documentation**: [NestGate Docs](https://docs.nestgate.io)
- **Issues**: [GitHub Issues](https://github.com/ecoPrimals/NestGateV2/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ecoPrimals/NestGateV2/discussions)

---

**Built with ❤️ for high-performance, real-time distributed systems** 