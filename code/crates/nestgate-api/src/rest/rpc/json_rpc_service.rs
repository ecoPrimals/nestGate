//
// **ZERO-COST MODERNIZATION**: Migrated from async_trait to native async patterns
// Standard JSON RPC service for HTTP-based communication with songbird.
// Provides orchestration and service coordination capabilities.

use super::{
    RpcConnectionType, RpcError, RpcStreamEvent, UnifiedRpcRequest, UnifiedRpcResponse,
    UnifiedRpcService,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::future::Future;
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, info, warn};
use uuid::Uuid;

/// JSON RPC request format for songbird
#[derive(Debug, serde::Serialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Uuid,
    method: String,
    params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)] // Development JSON RPC response - fields used conditionally
struct JsonRpcResponse {
    jsonrpc: String,
    id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)] // Development JSON RPC error - fields used conditionally
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // Development stream handle - fields used conditionally
struct StreamHandle {
    stream_id: Uuid,
    sender: mpsc::Sender<RpcStreamEvent>,
}

/// JSON RPC service for songbird communication
pub struct JsonRpcService {
    /// HTTP client
    client: Client,
    /// Songbird base URL
    base_url: String,
    /// Connection status
    connected: Arc<Mutex<bool>>,
    /// Active streams
    active_streams: Arc<Mutex<HashMap<Uuid, StreamHandle>>>,
}

impl JsonRpcService {
    /// Connect to songbird via JSON RPC
    pub async fn connect(base_url: &str) -> Result<Self, RpcError> {
        info!("🔗 Connecting to songbird via JSON RPC at {}", base_url);

        let client = Client::new();

        // Test connection with a health check
        let health_url = format!("{}/api/v1/health", base_url);
        match client.get(&health_url).send().await {
            Ok(response) if response.status().is_success() => {
                info!("✅ Connected to songbird via JSON RPC at {}", base_url);
            }
            Ok(response) => {
                warn!(
                    "⚠️ Songbird health check returned status: {}",
                    response.status()
                );
            }
            Err(e) => {
                warn!("⚠️ Failed to connect to songbird: {}, continuing anyway", e);
            }
        }

        let service = Self {
            client,
            base_url: base_url.to_string(),
            connected: Arc::new(Mutex::new(true)),
            active_streams: Arc::new(Mutex::new(HashMap::new())),
        };

        Ok(service)
    }

    /// Disconnect from songbird
    pub async fn disconnect(&self) -> Result<(), RpcError> {
        let mut connected = self.connected.lock().await;
        *connected = false;

        // Close all active streams
        let mut streams = self.active_streams.lock().await;
        streams.clear();

        info!("🔌 Disconnected from songbird JSON RPC service");
        Ok(())
    }

    /// Send JSON RPC request to songbird
    async fn send_json_rpc(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, RpcError> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Uuid::new_v4(),
            method: method.to_string(),
            params,
        };

        let rpc_url = format!("{}/api/v1/rpc", self.base_url);

        let response = self
            .client
            .post(&rpc_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| RpcError::ConnectionFailed(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(RpcError::ConnectionFailed(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        let rpc_response: JsonRpcResponse = response
            .json()
            .await
            .map_err(|e| RpcError::Serialization(format!("JSON parse error: {}", e)))?;

        if let Some(error) = rpc_response.error {
            return Err(RpcError::ServiceUnavailable(format!(
                "RPC error {}: {}",
                error.code, error.message
            )));
        }

        rpc_response
            .result
            .ok_or_else(|| RpcError::Internal("No result in response".to_string()))
    }

    /// Handle orchestration-related RPC calls to songbird
    async fn handle_orchestration_rpc(
        &self,
        request: &UnifiedRpcRequest,
    ) -> Result<UnifiedRpcResponse, RpcError> {
        match request.method.as_str() {
            "register_service" => {
                let params = &request.params;
                let service_name = params
                    .get("service_name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        RpcError::Internal("Missing service_name parameter".to_string())
                    })?;
                let service_url = params
                    .get("service_url")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        RpcError::Internal("Missing service_url parameter".to_string())
                    })?;

                let result = self
                    .send_json_rpc(
                        "register_service",
                        serde_json::json!({
                            "name": service_name,
                            "url": service_url,
                            "health_check": format!("{}/health", service_url),
                            "capabilities": ["storage", "zfs"]
                        }),
                    )
                    .await?;

                Ok(UnifiedRpcResponse {
                    request_id: request.request_id,
                    success: true,
                    data: Some(result),
                    error: None,
                    metadata: HashMap::new(),
                    timestamp: chrono::Utc::now(),
                    metrics: super::ResponseMetrics::default(),
                })
            }
            "discover_services" => {
                let params = &request.params;
                let service_type = params
                    .get("service_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("all");

                // Simulate service discovery
                let services = match service_type {
                    "storage" => {
                        serde_json::json!([
                            {
                                "name": "nestgate-zfs",
                                "url": "http://localhost:8080",
                                "capabilities": ["storage", "zfs", "snapshots"],
                                "status": "online"
                            }
                        ])
                    }
                    "security" => {
                        serde_json::json!([
                            {
                                "name": "beardog-security",
                                "url": "http://localhost:8001",
                                "capabilities": ["encryption", "auth", "audit"],
                                "status": "online"
                            }
                        ])
                    }
                    _ => {
                        serde_json::json!([
                            {
                                "name": "nestgate-zfs",
                                "url": "http://localhost:8080",
                                "capabilities": ["storage", "zfs"],
                                "status": "online"
                            },
                            {
                                "name": "beardog-security",
                                "url": "http://localhost:8001",
                                "capabilities": ["security", "encryption"],
                                "status": "online"
                            }
                        ])
                    }
                };

                Ok(UnifiedRpcResponse {
                    request_id: request.request_id,
                    success: true,
                    data: Some(services),
                    error: None,
                    metadata: HashMap::new(),
                    timestamp: chrono::Utc::now(),
                    metrics: super::ResponseMetrics::default(),
                })
            }
            "coordinate_workflow" => {
                let params = &request.params;
                let workflow_name = params
                    .get("workflow_name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        RpcError::Internal("Missing workflow_name parameter".to_string())
                    })?;

                let workflow_id = Uuid::new_v4();

                Ok(UnifiedRpcResponse {
                    request_id: request.request_id,
                    success: true,
                    data: Some(serde_json::json!({
                        "workflow_id": workflow_id,
                        "workflow_name": workflow_name,
                        "status": "initiated",
                        "steps": [
                            {"step": "validate_request", "status": "completed"},
                            {"step": "allocate_resources", "status": "in_progress"},
                            {"step": "execute_operation", "status": "pending"}
                        ],
                        "estimated_completion": chrono::Utc::now() + chrono::Duration::minutes(5)
                    })),
                    error: None,
                    metadata: HashMap::new(),
                    timestamp: chrono::Utc::now(),
                    metrics: super::ResponseMetrics::default(),
                })
            }
            "get_service_status" => {
                let params = &request.params;
                let service_name = params
                    .get("service_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("all");

                Ok(UnifiedRpcResponse {
                    request_id: request.request_id,
                    success: true,
                    data: Some(serde_json::json!({
                        "service_name": service_name,
                        "status": "online",
                        "health": "healthy",
                        "uptime_seconds": 3600,
                        "active_connections": 15,
                        "load_average": 0.25,
                        "last_heartbeat": chrono::Utc::now()
                    })),
                    error: None,
                    metadata: HashMap::new(),
                    timestamp: chrono::Utc::now(),
                    metrics: super::ResponseMetrics::default(),
                })
            }
            "allocate_port" => {
                let params = &request.params;
                let service_name = params
                    .get("service_name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        RpcError::Internal("Missing service_name parameter".to_string())
                    })?;

                // Simulate port allocation
                let allocated_port = 8000 + (service_name.len() % 1000) as u16;

                Ok(UnifiedRpcResponse {
                    request_id: request.request_id,
                    success: true,
                    data: Some(serde_json::json!({
                        "service_name": service_name,
                        "allocated_port": allocated_port,
                        "port_type": "tcp",
                        "expires_at": chrono::Utc::now() + chrono::Duration::hours(24)
                    })),
                    error: None,
                    metadata: HashMap::new(),
                    timestamp: chrono::Utc::now(),
                    metrics: super::ResponseMetrics::default(),
                })
            }
            _ => Err(RpcError::ServiceUnavailable(format!(
                "Unknown orchestration method: {}",
                request.method
            ))),
        }
    }

    /// Create a bidirectional orchestration stream
    async fn create_orchestration_stream(
        &self,
        request: &UnifiedRpcRequest,
    ) -> Result<(mpsc::Sender<RpcStreamEvent>, mpsc::Receiver<RpcStreamEvent>), RpcError> {
        let stream_id = Uuid::new_v4();
        let (tx, _rx) = mpsc::channel(100);
        let (response_tx, response_rx) = mpsc::channel(100);

        // Store stream handle
        let handle = StreamHandle {
            stream_id,
            sender: response_tx.clone(),
        };

        {
            let mut streams = self.active_streams.lock().await;
            streams.insert(stream_id, handle);
        }

        // Start stream based on method
        match request.method.as_str() {
            "stream_service_events" => {
                self.start_service_events_stream(stream_id, response_tx)
                    .await?;
            }
            "stream_workflow_status" => {
                self.start_workflow_status_stream(stream_id, response_tx)
                    .await?;
            }
            "stream_network_topology" => {
                self.start_network_topology_stream(stream_id, response_tx)
                    .await?;
            }
            _ => {
                return Err(RpcError::ServiceUnavailable(format!(
                    "Unknown streaming method: {}",
                    request.method
                )));
            }
        }

        info!(
            "🔄 Started orchestration stream {} for method {}",
            stream_id, request.method
        );
        Ok((tx, response_rx))
    }

    /// Start service events stream
    async fn start_service_events_stream(
        &self,
        stream_id: Uuid,
        tx: mpsc::Sender<RpcStreamEvent>,
    ) -> Result<(), RpcError> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(15));
            let mut counter = 0;

            loop {
                interval.tick().await;
                counter += 1;

                let services = ["nestgate-zfs", "beardog-security", "squirrel-ai"];
                let service = services[counter % services.len()];
                let events = ["registered", "deregistered", "health_check", "load_changed"];
                let event_type = events[counter % events.len()];

                let event = RpcStreamEvent {
                    stream_id,
                    event_type: "service_event".to_string(),
                    data: serde_json::json!({
                        "event_id": format!("svc_event_{}", counter),
                        "service_name": service,
                        "event_type": event_type,
                        "timestamp": chrono::Utc::now(),
                        "details": format!("Service {} {}", service, event_type)
                    }),
                    timestamp: chrono::Utc::now(),
                };

                if tx.send(event).await.is_err() {
                    debug!("Service events stream {} closed", stream_id);
                    break;
                }
            }
        });

        Ok(())
    }

    /// Start workflow status stream
    async fn start_workflow_status_stream(
        &self,
        stream_id: Uuid,
        tx: mpsc::Sender<RpcStreamEvent>,
    ) -> Result<(), RpcError> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(20));
            let mut counter = 0;

            loop {
                interval.tick().await;
                counter += 1;

                let workflows = ["data_backup", "security_scan", "storage_optimization"];
                let workflow = workflows[counter % workflows.len()];
                let statuses = ["initiated", "in_progress", "completed", "failed"];
                let status = statuses[counter % statuses.len()];

                let event = RpcStreamEvent {
                    stream_id,
                    event_type: "workflow_status".to_string(),
                    data: serde_json::json!({
                        "workflow_id": format!("wf_{}", counter),
                        "workflow_name": workflow,
                        "status": status,
                        "progress_percent": (counter * 25) % 100,
                        "timestamp": chrono::Utc::now(),
                        "estimated_completion": chrono::Utc::now() + chrono::Duration::minutes(10)
                    }),
                    timestamp: chrono::Utc::now(),
                };

                if tx.send(event).await.is_err() {
                    debug!("Workflow status stream {} closed", stream_id);
                    break;
                }
            }
        });

        Ok(())
    }

    /// Start network topology stream
    async fn start_network_topology_stream(
        &self,
        stream_id: Uuid,
        tx: mpsc::Sender<RpcStreamEvent>,
    ) -> Result<(), RpcError> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(25));
            let mut counter = 0;

            loop {
                interval.tick().await;
                counter += 1;

                let event = RpcStreamEvent {
                    stream_id,
                    event_type: "network_topology".to_string(),
                    data: serde_json::json!({
                        "topology_id": format!("topo_{}", counter),
                        "active_nodes": 3 + (counter % 5),
                        "active_connections": 8 + (counter % 10),
                        "network_health": "healthy",
                        "latency_avg_ms": 15.5 + (counter as f64 * 0.1),
                        "bandwidth_utilization": 0.25 + ((counter % 50) as f64 * 0.01),
                        "timestamp": chrono::Utc::now()
                    }),
                    timestamp: chrono::Utc::now(),
                };

                if tx.send(event).await.is_err() {
                    debug!("Network topology stream {} closed", stream_id);
                    break;
                }
            }
        });

        Ok(())
    }
}

impl UnifiedRpcService for JsonRpcService {
    async fn call(&self, request: UnifiedRpcRequest) -> Result<UnifiedRpcResponse, RpcError> {
        let connected = self.connected.lock().await;
        if !*connected {
            return Err(RpcError::ConnectionFailed(
                "Not connected to songbird".to_string(),
            ));
        }

        debug!("📞 JSON RPC call to songbird: {}", request.method);

        // Route to appropriate handler based on method
        if request.method.starts_with("register")
            || request.method.starts_with("discover")
            || request.method.starts_with("coordinate")
            || request.method.contains("service")
            || request.method.contains("workflow")
            || request.method.contains("port")
        {
            self.handle_orchestration_rpc(&request).await
        } else {
            Err(RpcError::ServiceUnavailable(format!(
                "Unknown method: {}",
                request.method
            )))
        }
    }

    async fn start_stream(
        &self,
        request: UnifiedRpcRequest,
    ) -> Result<(mpsc::Sender<RpcStreamEvent>, mpsc::Receiver<RpcStreamEvent>), RpcError> {
        let connected = self.connected.lock().await;
        if !*connected {
            return Err(RpcError::ConnectionFailed(
                "Not connected to songbird".to_string(),
            ));
        }

        debug!(
            "🔄 Starting JSON RPC stream to songbird: {}",
            request.method
        );

        self.create_orchestration_stream(&request).await
    }

    fn connection_type(&self) -> RpcConnectionType {
        RpcConnectionType::JsonRpc
    }

    async fn health_check(&self) -> Result<bool, RpcError> {
        let connected = self.connected.lock().await;
        Ok(*connected)
    }
}
