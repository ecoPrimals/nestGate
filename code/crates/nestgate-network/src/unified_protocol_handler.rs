/// Unified Protocol Handler Implementation
/// This module demonstrates the migration from legacy ProtocolHandler to the new
/// UnifiedHandler system, showing how trait consolidation improves consistency
/// while maintaining backward compatibility.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

// Import the new unified traits
use nestgate_core::unified_traits::{
    UnifiedHandler, UnifiedHandlerType, UnifiedHandlerConfig,
    UnifiedRequest, UnifiedResponse, storage::UnifiedStorageHandler,
    StorageMountRequest, StorageMountResponse, StorageOperation, StorageOperationResult
};
use nestgate_core::unified_enums::{UnifiedOperationType, UnifiedHealthStatus};
use nestgate_core::Result;

// Import legacy types for compatibility
use super::protocol::{Protocol, MountRequest, MountResponse, MountStatus, Credentials};

/// Modern NFS Handler implementing the unified trait system
#[derive(Debug)]
pub struct UnifiedNfsHandler {
    handler_id: String,
    config: UnifiedHandlerConfig,
    active_mounts: HashMap<String, MountStatus>,
    }

impl UnifiedNfsHandler {
    pub fn new(handler_id: String) -> Self {
        let config = UnifiedHandlerConfig {
            handler_id: handler_id.clone(),
            handler_type: UnifiedHandlerType::Storage,
            timeout: Duration::from_secs(30),
            max_concurrent_requests: 100,
            custom_config: HashMap::new(),
        };

        Self {
            handler_id,
            config,
            active_mounts: HashMap::new(),
    }
    }
    }

#[async_trait]
impl UnifiedHandler for UnifiedNfsHandler {
    fn handler_id(&self) -> &str {
        &self.handler_id
    }
    
    fn handler_type(&self) -> UnifiedHandlerType {
        UnifiedHandlerType::Storage
    }
    
    fn supported_operations(&self) -> Vec<UnifiedOperationType> {
        vec![
            UnifiedOperationType::Mount,
            UnifiedOperationType::Unmount,
            UnifiedOperationType::Status,
            UnifiedOperationType::Test,
        ]
    }
    
    async fn handle_request(&self, request: UnifiedRequest) -> Result<UnifiedResponse> {
        let start_time = Instant::now();
        
        let result = match request.operation {
            UnifiedOperationType::Mount => {
                // Handle mount operation
                let mount_params = request.parameters;
                // Implementation would parse mount_params and execute mount
                Ok(serde_json::json!({"mount_id": "mount_123", "status": "mounted"}))
            },
            UnifiedOperationType::Unmount => {
                // Handle unmount operation
                Ok(serde_json::json!({"status": "unmounted"}))
            },
            UnifiedOperationType::Status => {
                // Handle status check
                Ok(serde_json::json!({"active_mounts": self.active_mounts.len()}))
            },
            UnifiedOperationType::Test => {
                // Handle connection test
                Ok(serde_json::json!({"connection": "successful"}))
            },
            _ => Err(nestgate_core::NestGateError::Internal {
                message: format!("Unsupported operation: {:?}", request.operation),
                location: Some(format!("{}:{}", file!(), line!())),
                debug_info: None,
                is_bug: false,
            })
        };

        let execution_time = start_time.elapsed();
        
        match result {
            Ok(data) => Ok(UnifiedResponse {
                success: true,
                data: Some(data),
                error: None,
                execution_time,
            }),
            Err(e) => Ok(UnifiedResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
                execution_time,
            })
    }
    }
    
    async fn health_check(&self) -> Result<UnifiedHealthStatus> {
        // Perform NFS-specific health checks
        if self.active_mounts.len() < self.config.max_concurrent_requests {
            Ok(UnifiedHealthStatus::Healthy)
        } else {
            Ok(UnifiedHealthStatus::Degraded)
    }
    }
    
    fn configuration(&self) -> UnifiedHandlerConfig {
        self.config.clone()
    }
    
    async fn initialize(&mut self, config: UnifiedHandlerConfig) -> Result<()> {
        self.config = config;
        // Perform NFS-specific initialization
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        // Gracefully unmount all active mounts
        self.active_mounts.clear();
    }
    }

#[async_trait]
impl UnifiedStorageHandler for UnifiedNfsHandler {
    async fn mount(&self, request: StorageMountRequest) -> Result<StorageMountResponse> {
        // NFS-specific mount implementation
        let mount_id = format!("nfs_{}", uuid::Uuid::new_v4());
        
        Ok(StorageMountResponse {
            mount_id,
            mount_point: request.mount_point,
            success: true,
        })
    }
    
    async fn unmount(&self, mount_id: &str) -> Result<bool> {
        // NFS-specific unmount implementation
        Ok(true)
    }
    
    async fn storage_operation_with_timeout(
        &self, 
        operation: StorageOperation,
        timeout: Duration
    ) -> Result<StorageOperationResult> {
        let start_time = Instant::now();
        
        // Simulate operation with timeout
        tokio::time::timeout(timeout, async {
            // Perform the actual operation
            tokio::time::sleep(Duration::from_millis(100)).await;
            
            StorageOperationResult {
                success: true,
                result: Some(serde_json::json!({"operation": operation.operation_type})),
                execution_time: start_time.elapsed(),
    }
        }).await.map_err(|_| nestgate_core::NestGateError::Internal {
            message: "Operation timed out".to_string(),
            location: Some(format!("{}:{}", file!(), line!())),
            debug_info: Some(format!("Timeout: {:?}", timeout)),
            is_bug: false,
        })
    }
    }

// **DEPRECATED LEGACY ADAPTER REMOVED**
// Use native UnifiedHandler implementations instead of legacy adapters

/// Example usage demonstrating the unified trait system
pub mod examples {
    use super::*;
    
    pub async fn demonstrate_unified_handler() -> Result<()> {
        // Create a modern unified handler
        let mut nfs_handler = UnifiedNfsHandler::new("nfs-handler-001".to_string());
        
        // Initialize with configuration
        let config = UnifiedHandlerConfig {
            handler_id: "nfs-handler-001".to_string(),
            handler_type: UnifiedHandlerType::Storage,
            timeout: Duration::from_secs(60),
            max_concurrent_requests: 200,
            custom_config: HashMap::new(),
        };
        nfs_handler.initialize(config).await?;
        
        // Use the unified interface
        let request = UnifiedRequest {
            operation: UnifiedOperationType::Mount,
            parameters: {
                let mut params = HashMap::new();
                params.insert("source".to_string(), serde_json::json!("192.168.1.100:/data"));
                params.insert("mount_point".to_string(), serde_json::json!("/mnt/nfs"));
                params
            },
            timeout: Some(Duration::from_secs(30)),
        };
        
        let response = nfs_handler.handle_request(request).await?;
        println!("Mount response: {:?}", response);
        
        // Check health
        let health = nfs_handler.health_check().await?;
        println!("Handler health: {:?}", health);
        
        // Use storage-specific interface
        let mount_request = StorageMountRequest {
            source: "192.168.1.100:/data".to_string(),
            mount_point: std::path::PathBuf::from("/mnt/nfs"),
            protocol: "nfs".to_string(),
            options: HashMap::new(),
        };
        
        let mount_response = nfs_handler.mount(mount_request).await?;
        println!("Storage mount response: {:?}", mount_response);
        
        // Graceful shutdown
        nfs_handler.shutdown().await?;
        
    }
    
    // **DEPRECATED LEGACY ADAPTATION EXAMPLE REMOVED**
    // Use native UnifiedHandler implementations for all protocol handling
} 