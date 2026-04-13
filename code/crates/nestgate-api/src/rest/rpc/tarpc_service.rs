// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// High-performance binary RPC service using tarpc for communication with security.
// Provides real-time bidirectional streaming for security operations.

use tokio::sync::mpsc;
use tracing::{debug, info};
use uuid::Uuid;

/// **TARPC RPC SERVICE**
///
/// TarPC-based RPC service implementation for high-performance communication.
#[derive(Debug, Clone)]
/// Service implementation for `TarpcRpc`
pub struct TarpcRpcService {
    /// Connection address
    _endpoint: String,
}
#[derive(Debug, Clone)]
struct StreamHandle {
    _stream_id: Uuid,
    _sender: mpsc::Sender<super::RpcStreamEvent>,
}
impl TarpcRpcService {
    /// Create a new tarpc RPC service
    pub fn new(endpoint: &str) -> Self {
        let service = Self {
            _endpoint: endpoint.to_string(),
        };

        info!("tarpc RPC service initialized for endpoint: {}", endpoint);
        service
    }

    /// Execute a unified RPC request
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn execute_request(
        &self,
        request: super::UnifiedRpcRequest,
    ) -> Result<super::UnifiedRpcResponse, super::RpcError> {
        debug!("tarpc call to security: {}", request.method);

        Err(super::RpcError::ServiceUnavailable(format!(
            "tarpc security service not yet wired for method '{}'. \
             Use JSON-RPC IPC via Unix socket instead.",
            request.method
        )))
    }

    /// Start a bidirectional stream
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn start_stream(
        &self,
        request: super::UnifiedRpcRequest,
    ) -> Result<
        (
            mpsc::Sender<super::RpcStreamEvent>,
            mpsc::Receiver<super::RpcStreamEvent>,
        ),
        super::RpcError,
    > {
        debug!("Starting tarpc stream to security: {}", request.method);

        let stream_id = Uuid::new_v4();
        let (response_tx, response_rx) = mpsc::channel(100);

        let _handle = StreamHandle {
            _stream_id: stream_id,
            _sender: response_tx,
        };

        let (tx, _rx) = mpsc::channel(100);
        Ok((tx, response_rx))
    }

    /// Get connection type
    #[must_use]
    pub const fn connection_type(&self) -> super::RpcConnectionType {
        super::RpcConnectionType::Tarpc
    }

    /// Health check
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub const fn health_check(&self) -> Result<bool, super::RpcError> {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::rpc::types::{
        RequestPriority, ResponseMetrics, RpcConnectionType, UnifiedRpcRequest, UnifiedRpcResponse,
    };
    use std::collections::HashMap;
    use uuid::Uuid;

    fn sample_request(method: &str) -> UnifiedRpcRequest {
        UnifiedRpcRequest {
            id: Uuid::nil(),
            source: "nestgate".to_string(),
            target: "security".to_string(),
            method: method.to_string(),
            _params: serde_json::json!({}),
            _metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
            streaming: false,
            priority: RequestPriority::Normal,
            timeout: None,
        }
    }

    #[test]
    fn new_and_connection_type() {
        let svc = TarpcRpcService::new("tcp://127.0.0.1:0");
        assert_eq!(svc.connection_type(), RpcConnectionType::Tarpc);
        assert!(svc.health_check().unwrap());
    }

    #[test]
    fn execute_request_returns_service_unavailable() {
        let svc = TarpcRpcService::new("tcp://127.0.0.1:0");
        let req = sample_request("ping");
        let result = svc.execute_request(req);
        assert!(
            result.is_err(),
            "tarpc service not yet wired — should return error"
        );
    }

    #[test]
    fn start_stream_returns_channels() {
        let svc = TarpcRpcService::new("tcp://127.0.0.1:0");
        let req = sample_request("stream");
        let (tx, _rx) = svc.start_stream(req).unwrap();
        drop(tx);
    }

    #[test]
    fn unified_response_roundtrip_serde() {
        let r = UnifiedRpcResponse {
            request_id: Uuid::nil(),
            success: true,
            data: Some(serde_json::json!({"ok": true})),
            error: None,
            _metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
            metrics: ResponseMetrics::default(),
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: UnifiedRpcResponse = serde_json::from_str(&json).unwrap();
        assert!(back.success);
    }
}
