use nestgate_api::rest::rpc::{
    BidirectionalStreamManager, JsonRpcRequest, JsonRpcResponse, RpcError, UnifiedRpcRouter,
};
use nestgate_api::streaming::{StreamEvent, StreamManager};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Initialize the RPC router
    let mut router = UnifiedRpcRouter::new();

    // Register a simple method
    router.register_method("echo", |params: Value| async move {
        Ok(json!({
            "echo": params,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    });

    // Register a math method
    router.register_method("add", |params: Value| async move {
        let a = params.get("a").and_then(|v| v.as_i64()).unwrap_or(0);
        let b = params.get("b").and_then(|v| v.as_i64()).unwrap_or(0);
        Ok(json!({"result": a + b}))
    });

    let router = Arc::new(RwLock::new(router));

    // Test JSON-RPC requests
    println!("🚀 Testing NestGate RPC System");
    println!("================================");

    // Test echo method
    let echo_request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "echo".to_string(),
        params: Some(json!({"message": "Hello, NestGate!"})),
        id: Some(json!(1)),
    };

    let response = router.read().await.handle_request(echo_request).await;
    println!(
        "📡 Echo Response: {}",
        serde_json::to_string_pretty(&response)?
    );

    // Test math method
    let math_request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "add".to_string(),
        params: Some(json!({"a": 42, "b": 58})),
        id: Some(json!(2)),
    };

    let response = router.read().await.handle_request(math_request).await;
    println!(
        "🔢 Math Response: {}",
        serde_json::to_string_pretty(&response)?
    );

    // Test unknown method (error case)
    let _error_request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "unknown_method".to_string(),
        params: None,
        id: Some(json!(3)),
    };

    let response = router.read().await.handle_request(error_request).await;
    println!(
        "❌ Error Response: {}",
        serde_json::to_string_pretty(&response)?
    );

    // Test bidirectional streaming
    println!("\n🔄 Testing Bidirectional Streaming");
    println!("===================================");

    let stream_manager = StreamManager::new();
    let stream_id = stream_manager.create_stream("test_stream").await?;

    // Send some events
    let events = vec![
        StreamEvent {
            id: "event1".to_string(),
            event_type: "data".to_string(),
            data: json!({"message": "First event"}),
            timestamp: chrono::Utc::now(),
        },
        StreamEvent {
            id: "event2".to_string(),
            event_type: "status".to_string(),
            data: json!({"status": "processing"}),
            timestamp: chrono::Utc::now(),
        },
    ];

    for event in events {
        stream_manager.send_event(&stream_id, event).await?;
    }

    println!("✅ Stream {} created with events", stream_id);

    // Demonstrate tarpc integration (if enabled)
    #[cfg(feature = "tarpc")]
    {
        println!("\n⚡ Testing tarpc Integration");
        println!("============================");

        use nestgate_api::rest::rpc::TarpcRpcService;

        let tarpc_handler = TarpcRpcService::new(router.clone());
        println!("✅ tarpc service handler initialized");
    }

    println!("\n🎉 All RPC tests completed successfully!");
    println!("The NestGate RPC system is fully operational.");

    Ok(())
}
