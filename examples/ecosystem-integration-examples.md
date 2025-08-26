# 🌐 **NestGate v2.0 Ecosystem Integration Examples**

**Date**: January 30, 2025  
**Version**: NestGate v2.0 - Ecosystem Integration  
**Status**: ✅ **PRODUCTION READY**

---

## 📋 **OVERVIEW**

NestGate v2.0 is designed to seamlessly integrate with the broader ecosystem through the **Universal Adapter** pattern. This document provides comprehensive examples of how NestGate interacts with other primals to create a cohesive, capability-driven ecosystem.

### **Integration Architecture**

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│    BearDog      │    │    Songbird     │    │    Squirrel     │
│   (Security)    │◄──►│ (Orchestration) │◄──►│      (AI)       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         ▲                        ▲                        ▲
         │                        │                        │
         ▼                        ▼                        ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Universal Adapter                            │
│                  (Capability Discovery)                         │
└─────────────────────────────────────────────────────────────────┘
                                 ▲
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                      NestGate v2.0                             │
│                   (Storage Primal)                             │
└─────────────────────────────────────────────────────────────────┘
                                 ▲
                                 │
                                 ▼
┌─────────────────┐                        ┌─────────────────┐
│    Toadstool    │                        │   External      │
│   (Compute)     │◄──────────────────────►│   Services      │
└─────────────────┘                        └─────────────────┘
```

---

## 🔐 **INTEGRATION WITH BEARDOG (SECURITY)**

### **Authentication Delegation**

NestGate v2.0 delegates all authentication and authorization to BearDog through capability-based requests.

#### **Example: User Authentication Flow**

```rust
// NestGate authentication handler
use crate::ecosystem_integration::universal_adapter::{CapabilityRequest, UniversalAdapter};

pub async fn authenticate_user(
    adapter: &UniversalAdapter,
    token: &str,
) -> Result<UserContext, NestGateError> {
    // Delegate authentication to BearDog
    let auth_request = CapabilityRequest {
        capability: "security.authentication".to_string(),
        operation: "validate_token".to_string(),
        payload: serde_json::json!({
            "token": token,
            "service": "nestgate",
            "required_permissions": ["storage.read", "storage.write"]
        }).to_string().into_bytes(),
        metadata: HashMap::from([
            ("source".to_string(), "nestgate".to_string()),
            ("request_id".to_string(), uuid::Uuid::new_v4().to_string()),
        ]),
        timeout_ms: Some(5000),
        priority: crate::ecosystem_integration::universal_adapter::types::Priority::High,
        performance_requirements: Some(PerformanceRequirements {
            max_response_time_ms: Some(3000.0),
            min_throughput_ops_per_sec: Some(100.0),
            min_success_rate_percent: Some(99.0),
            max_error_rate_percent: Some(1.0),
            min_availability_percent: Some(99.9),
        }),
    };

    match adapter.execute_capability(auth_request).await {
        Ok(response) => {
            let auth_result: AuthenticationResult = 
                serde_json::from_slice(&response.payload)?;
            
            if auth_result.valid {
                Ok(UserContext {
                    user_id: auth_result.user_id,
                    permissions: auth_result.permissions,
                    session_id: auth_result.session_id,
                })
            } else {
                Err(NestGateError::Unauthorized {
                    message: "Invalid authentication token".to_string(),
                })
            }
        }
        Err(e) => {
            // Graceful fallback to local auth if BearDog unavailable
            warn!("BearDog authentication failed, falling back to local: {}", e);
            authenticate_locally(token).await
        }
    }
}

#[derive(Deserialize)]
struct AuthenticationResult {
    valid: bool,
    user_id: String,
    permissions: Vec<String>,
    session_id: String,
}
```

#### **Example: Authorization Check**

```rust
pub async fn check_storage_permission(
    adapter: &UniversalAdapter,
    user_context: &UserContext,
    resource_path: &str,
    operation: &str,
) -> Result<bool, NestGateError> {
    let authz_request = CapabilityRequest {
        capability: "security.authorization".to_string(),
        operation: "check_permission".to_string(),
        payload: serde_json::json!({
            "user_id": user_context.user_id,
            "resource": format!("storage://{}", resource_path),
            "operation": operation,
            "context": {
                "service": "nestgate",
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }
        }).to_string().into_bytes(),
        metadata: HashMap::from([
            ("user_id".to_string(), user_context.user_id.clone()),
            ("session_id".to_string(), user_context.session_id.clone()),
        ]),
        timeout_ms: Some(3000),
        priority: Priority::High,
        performance_requirements: None,
    };

    match adapter.execute_capability(authz_request).await {
        Ok(response) => {
            let authz_result: AuthorizationResult = 
                serde_json::from_slice(&response.payload)?;
            Ok(authz_result.permitted)
        }
        Err(_) => {
            // Fail secure - deny access if BearDog unavailable
            Ok(false)
        }
    }
}

#[derive(Deserialize)]
struct AuthorizationResult {
    permitted: bool,
    reason: Option<String>,
}
```

#### **Configuration Example**

```toml
# production.toml - BearDog integration
[ecosystem.security]
preferred_primal = "beardog"
fallback_to_local = true
auth_timeout_ms = 5000
authz_timeout_ms = 3000

# Security capabilities required
required_capabilities = [
    "security.authentication",
    "security.authorization",
    "security.audit_logging"
]

# Optional security capabilities
optional_capabilities = [
    "security.encryption",
    "security.key_management",
    "security.threat_detection"
]
```

---

## 🎵 **INTEGRATION WITH SONGBIRD (ORCHESTRATION)**

### **Workflow Coordination**

NestGate delegates complex orchestration tasks to Songbird, focusing on storage operations while Songbird handles workflow coordination.

#### **Example: Backup Orchestration**

```rust
pub async fn initiate_backup_workflow(
    adapter: &UniversalAdapter,
    backup_config: &BackupConfiguration,
) -> Result<WorkflowId, NestGateError> {
    // Delegate backup orchestration to Songbird
    let workflow_request = CapabilityRequest {
        capability: "orchestration.workflow".to_string(),
        operation: "start_workflow".to_string(),
        payload: serde_json::json!({
            "workflow_type": "storage_backup",
            "parameters": {
                "storage_service": "nestgate",
                "backup_type": backup_config.backup_type,
                "datasets": backup_config.datasets,
                "destination": backup_config.destination,
                "compression": backup_config.compression,
                "encryption": backup_config.encryption,
            },
            "steps": [
                {
                    "name": "pre_backup_validation",
                    "service": "nestgate",
                    "operation": "validate_backup_prerequisites"
                },
                {
                    "name": "create_snapshots",
                    "service": "nestgate", 
                    "operation": "create_backup_snapshots"
                },
                {
                    "name": "transfer_data",
                    "service": "nestgate",
                    "operation": "transfer_backup_data"
                },
                {
                    "name": "verify_backup",
                    "service": "nestgate",
                    "operation": "verify_backup_integrity"
                },
                {
                    "name": "cleanup_snapshots",
                    "service": "nestgate",
                    "operation": "cleanup_backup_snapshots"
                }
            ]
        }).to_string().into_bytes(),
        metadata: HashMap::from([
            ("initiator".to_string(), "nestgate".to_string()),
            ("backup_id".to_string(), backup_config.id.clone()),
        ]),
        timeout_ms: Some(300000), // 5 minutes for workflow setup
        priority: Priority::Normal,
        performance_requirements: None,
    };

    match adapter.execute_capability(workflow_request).await {
        Ok(response) => {
            let workflow_result: WorkflowStartResult = 
                serde_json::from_slice(&response.payload)?;
            
            info!("Backup workflow started: {}", workflow_result.workflow_id);
            Ok(WorkflowId(workflow_result.workflow_id))
        }
        Err(e) => {
            warn!("Songbird workflow delegation failed, executing locally: {}", e);
            execute_backup_locally(backup_config).await
        }
    }
}

#[derive(Deserialize)]
struct WorkflowStartResult {
    workflow_id: String,
    status: String,
    estimated_duration_seconds: u64,
}
```

#### **Example: Multi-Service Data Pipeline**

```rust
pub async fn create_data_processing_pipeline(
    adapter: &UniversalAdapter,
    pipeline_config: &PipelineConfiguration,
) -> Result<PipelineId, NestGateError> {
    let pipeline_request = CapabilityRequest {
        capability: "orchestration.pipeline".to_string(),
        operation: "create_pipeline".to_string(),
        payload: serde_json::json!({
            "pipeline_name": pipeline_config.name,
            "services": {
                "storage": {
                    "service": "nestgate",
                    "role": "data_source_and_sink",
                    "capabilities": ["storage.read", "storage.write", "storage.snapshots"]
                },
                "compute": {
                    "service": "toadstool",
                    "role": "data_processor",
                    "capabilities": ["compute.batch_processing", "compute.parallel_execution"]
                },
                "ai": {
                    "service": "squirrel",
                    "role": "data_analyzer",
                    "capabilities": ["ai.pattern_recognition", "ai.optimization"]
                }
            },
            "data_flow": [
                {
                    "from": "nestgate",
                    "to": "toadstool",
                    "operation": "extract_and_process",
                    "data_format": "parquet"
                },
                {
                    "from": "toadstool", 
                    "to": "squirrel",
                    "operation": "analyze_patterns",
                    "data_format": "json"
                },
                {
                    "from": "squirrel",
                    "to": "nestgate",
                    "operation": "store_insights",
                    "data_format": "json"
                }
            ]
        }).to_string().into_bytes(),
        metadata: HashMap::from([
            ("creator".to_string(), "nestgate".to_string()),
            ("pipeline_type".to_string(), "data_processing".to_string()),
        ]),
        timeout_ms: Some(60000),
        priority: Priority::Normal,
        performance_requirements: None,
    };

    // Execute pipeline creation request
    match adapter.execute_capability(pipeline_request).await {
        Ok(response) => {
            let pipeline_result: PipelineCreateResult = 
                serde_json::from_slice(&response.payload)?;
            Ok(PipelineId(pipeline_result.pipeline_id))
        }
        Err(e) => Err(NestGateError::EcosystemIntegration {
            message: format!("Failed to create pipeline: {}", e),
            service: "songbird".to_string(),
        })
    }
}
```

---

## 🐿️ **INTEGRATION WITH SQUIRREL (AI)**

### **AI-Powered Storage Optimization**

NestGate leverages Squirrel's AI capabilities for intelligent storage management, predictive analytics, and optimization recommendations.

#### **Example: Storage Optimization Analysis**

```rust
pub async fn get_storage_optimization_recommendations(
    adapter: &UniversalAdapter,
    storage_metrics: &StorageMetrics,
) -> Result<OptimizationRecommendations, NestGateError> {
    let ai_request = CapabilityRequest {
        capability: "ai.optimization".to_string(),
        operation: "analyze_storage_patterns".to_string(),
        payload: serde_json::json!({
            "analysis_type": "storage_optimization",
            "data": {
                "current_usage": {
                    "total_capacity_gb": storage_metrics.total_capacity_gb,
                    "used_capacity_gb": storage_metrics.used_capacity_gb,
                    "tier_distribution": storage_metrics.tier_distribution,
                    "access_patterns": storage_metrics.access_patterns,
                    "compression_ratios": storage_metrics.compression_ratios,
                },
                "historical_data": storage_metrics.historical_data,
                "performance_metrics": {
                    "iops": storage_metrics.iops,
                    "throughput_mbps": storage_metrics.throughput_mbps,
                    "latency_ms": storage_metrics.latency_ms,
                },
                "cost_metrics": storage_metrics.cost_metrics,
            },
            "optimization_goals": [
                "minimize_cost",
                "maximize_performance", 
                "optimize_capacity_utilization",
                "improve_data_lifecycle_management"
            ]
        }).to_string().into_bytes(),
        metadata: HashMap::from([
            ("requester".to_string(), "nestgate".to_string()),
            ("analysis_timestamp".to_string(), chrono::Utc::now().to_rfc3339()),
        ]),
        timeout_ms: Some(30000), // AI analysis can take time
        priority: Priority::Normal,
        performance_requirements: Some(PerformanceRequirements {
            max_response_time_ms: Some(30000.0),
            min_throughput_ops_per_sec: Some(1.0),
            min_success_rate_percent: Some(95.0),
            max_error_rate_percent: Some(5.0),
            min_availability_percent: Some(95.0),
        }),
    };

    match adapter.execute_capability(ai_request).await {
        Ok(response) => {
            let recommendations: OptimizationRecommendations = 
                serde_json::from_slice(&response.payload)?;
            
            info!("Received {} optimization recommendations from Squirrel", 
                  recommendations.recommendations.len());
            Ok(recommendations)
        }
        Err(e) => {
            warn!("AI optimization analysis failed: {}", e);
            // Fallback to basic rule-based optimization
            Ok(generate_basic_recommendations(storage_metrics))
        }
    }
}

#[derive(Deserialize)]
pub struct OptimizationRecommendations {
    pub recommendations: Vec<OptimizationRecommendation>,
    pub confidence_score: f64,
    pub estimated_savings: EstimatedSavings,
    pub implementation_priority: Vec<String>,
}

#[derive(Deserialize)]
pub struct OptimizationRecommendation {
    pub category: String,
    pub action: String,
    pub description: String,
    pub impact: String,
    pub effort: String,
    pub estimated_savings_percent: f64,
}
```

#### **Example: Predictive Storage Planning**

```rust
pub async fn get_capacity_predictions(
    adapter: &UniversalAdapter,
    historical_data: &CapacityHistoricalData,
    prediction_horizon_days: u32,
) -> Result<CapacityPredictions, NestGateError> {
    let prediction_request = CapabilityRequest {
        capability: "ai.prediction".to_string(),
        operation: "predict_storage_capacity".to_string(),
        payload: serde_json::json!({
            "prediction_type": "storage_capacity_planning",
            "historical_data": {
                "daily_usage_gb": historical_data.daily_usage_gb,
                "growth_patterns": historical_data.growth_patterns,
                "seasonal_variations": historical_data.seasonal_variations,
                "business_events": historical_data.business_events,
            },
            "prediction_parameters": {
                "horizon_days": prediction_horizon_days,
                "confidence_intervals": [50, 80, 95],
                "include_scenarios": ["conservative", "expected", "aggressive"],
            },
            "context": {
                "current_capacity_gb": historical_data.current_capacity_gb,
                "growth_constraints": historical_data.growth_constraints,
                "business_factors": historical_data.business_factors,
            }
        }).to_string().into_bytes(),
        metadata: HashMap::from([
            ("prediction_id".to_string(), uuid::Uuid::new_v4().to_string()),
            ("requester".to_string(), "nestgate".to_string()),
        ]),
        timeout_ms: Some(45000),
        priority: Priority::Normal,
        performance_requirements: None,
    };

    match adapter.execute_capability(prediction_request).await {
        Ok(response) => {
            let predictions: CapacityPredictions = 
                serde_json::from_slice(&response.payload)?;
            Ok(predictions)
        }
        Err(e) => {
            warn!("AI capacity prediction failed: {}", e);
            // Fallback to linear extrapolation
            Ok(generate_linear_predictions(historical_data, prediction_horizon_days))
        }
    }
}
```

---

## 🍄 **INTEGRATION WITH TOADSTOOL (COMPUTE)**

### **Compute-Intensive Storage Operations**

NestGate delegates CPU-intensive operations to Toadstool while maintaining control over storage-specific logic.

#### **Example: Parallel Data Processing**

```rust
pub async fn process_large_dataset(
    adapter: &UniversalAdapter,
    dataset_path: &str,
    processing_config: &ProcessingConfiguration,
) -> Result<ProcessingResult, NestGateError> {
    // First, prepare data access for Toadstool
    let data_access_token = generate_temporary_access_token(dataset_path).await?;
    
    let compute_request = CapabilityRequest {
        capability: "compute.batch_processing".to_string(),
        operation: "process_dataset".to_string(),
        payload: serde_json::json!({
            "job_type": "data_processing",
            "data_source": {
                "service": "nestgate",
                "path": dataset_path,
                "access_token": data_access_token,
                "format": processing_config.input_format,
            },
            "processing_parameters": {
                "operation": processing_config.operation,
                "batch_size": processing_config.batch_size,
                "parallel_workers": processing_config.parallel_workers,
                "memory_limit_gb": processing_config.memory_limit_gb,
            },
            "output_configuration": {
                "service": "nestgate",
                "path": processing_config.output_path,
                "format": processing_config.output_format,
                "compression": processing_config.compression,
            },
            "resource_requirements": {
                "cpu_cores": processing_config.cpu_cores,
                "memory_gb": processing_config.memory_gb,
                "estimated_duration_minutes": processing_config.estimated_duration_minutes,
            }
        }).to_string().into_bytes(),
        metadata: HashMap::from([
            ("job_id".to_string(), uuid::Uuid::new_v4().to_string()),
            ("priority".to_string(), processing_config.priority.to_string()),
        ]),
        timeout_ms: Some(processing_config.timeout_ms),
        priority: Priority::Normal,
        performance_requirements: Some(PerformanceRequirements {
            max_response_time_ms: Some(processing_config.timeout_ms as f64),
            min_throughput_ops_per_sec: Some(processing_config.min_throughput),
            min_success_rate_percent: Some(95.0),
            max_error_rate_percent: Some(5.0),
            min_availability_percent: Some(99.0),
        }),
    };

    match adapter.execute_capability(compute_request).await {
        Ok(response) => {
            let processing_result: ProcessingResult = 
                serde_json::from_slice(&response.payload)?;
            
            info!("Toadstool processing completed: {} records processed", 
                  processing_result.records_processed);
            Ok(processing_result)
        }
        Err(e) => {
            warn!("Toadstool processing failed, attempting local processing: {}", e);
            process_dataset_locally(dataset_path, processing_config).await
        }
    }
}

#[derive(Deserialize)]
pub struct ProcessingResult {
    pub job_id: String,
    pub status: String,
    pub records_processed: u64,
    pub output_location: String,
    pub processing_time_seconds: u64,
    pub resource_utilization: ResourceUtilization,
}
```

#### **Example: Distributed Compression**

```rust
pub async fn compress_large_files(
    adapter: &UniversalAdapter,
    file_list: &[String],
    compression_config: &CompressionConfiguration,
) -> Result<CompressionResults, NestGateError> {
    let compression_request = CapabilityRequest {
        capability: "compute.parallel_execution".to_string(),
        operation: "compress_files".to_string(),
        payload: serde_json::json!({
            "operation_type": "file_compression",
            "files": file_list.iter().map(|path| {
                serde_json::json!({
                    "source_path": path,
                    "access_method": "nestgate_direct",
                    "size_bytes": get_file_size(path).unwrap_or(0),
                })
            }).collect::<Vec<_>>(),
            "compression_settings": {
                "algorithm": compression_config.algorithm,
                "compression_level": compression_config.level,
                "parallel_streams": compression_config.parallel_streams,
                "chunk_size_mb": compression_config.chunk_size_mb,
            },
            "resource_allocation": {
                "max_cpu_cores": compression_config.max_cpu_cores,
                "max_memory_gb": compression_config.max_memory_gb,
                "io_priority": compression_config.io_priority,
            }
        }).to_string().into_bytes(),
        metadata: HashMap::from([
            ("compression_job_id".to_string(), uuid::Uuid::new_v4().to_string()),
            ("total_files".to_string(), file_list.len().to_string()),
        ]),
        timeout_ms: Some(compression_config.timeout_ms),
        priority: Priority::Normal,
        performance_requirements: None,
    };

    match adapter.execute_capability(compression_request).await {
        Ok(response) => {
            let compression_results: CompressionResults = 
                serde_json::from_slice(&response.payload)?;
            Ok(compression_results)
        }
        Err(e) => {
            warn!("Toadstool compression failed: {}", e);
            // Fallback to local compression
            compress_files_locally(file_list, compression_config).await
        }
    }
}
```

---

## 🔄 **ECOSYSTEM DISCOVERY AND FAILOVER**

### **Dynamic Service Discovery**

NestGate automatically discovers available ecosystem services and adapts its behavior based on available capabilities.

#### **Example: Service Discovery Implementation**

```rust
pub struct EcosystemManager {
    adapter: Arc<UniversalAdapter>,
    discovered_services: Arc<RwLock<HashMap<String, ServiceInfo>>>,
    capability_map: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl EcosystemManager {
    pub async fn discover_ecosystem_services(&self) -> Result<(), NestGateError> {
        let discovery_request = CapabilityRequest {
            capability: "system.discovery".to_string(),
            operation: "list_services".to_string(),
            payload: serde_json::json!({
                "discovery_type": "ecosystem_services",
                "filters": {
                    "status": "active",
                    "capabilities": [
                        "security.*",
                        "orchestration.*", 
                        "ai.*",
                        "compute.*"
                    ]
                }
            }).to_string().into_bytes(),
            metadata: HashMap::from([
                ("requester".to_string(), "nestgate".to_string()),
                ("discovery_timestamp".to_string(), chrono::Utc::now().to_rfc3339()),
            ]),
            timeout_ms: Some(10000),
            priority: Priority::High,
            performance_requirements: None,
        };

        match self.adapter.execute_capability(discovery_request).await {
            Ok(response) => {
                let discovery_result: ServiceDiscoveryResult = 
                    serde_json::from_slice(&response.payload)?;
                
                self.update_service_registry(discovery_result.services).await;
                info!("Discovered {} ecosystem services", discovery_result.services.len());
                Ok(())
            }
            Err(e) => {
                warn!("Service discovery failed: {}", e);
                Err(NestGateError::EcosystemIntegration {
                    message: format!("Service discovery failed: {}", e),
                    service: "universal_adapter".to_string(),
                })
            }
        }
    }

    async fn update_service_registry(&self, services: Vec<ServiceInfo>) {
        let mut discovered_services = self.discovered_services.write().await;
        let mut capability_map = self.capability_map.write().await;
        
        discovered_services.clear();
        capability_map.clear();
        
        for service in services {
            // Update service registry
            discovered_services.insert(service.name.clone(), service.clone());
            
            // Update capability mapping
            for capability in &service.capabilities {
                capability_map
                    .entry(capability.clone())
                    .or_insert_with(Vec::new)
                    .push(service.name.clone());
            }
        }
        
        info!("Updated ecosystem registry with {} services", discovered_services.len());
    }

    pub async fn get_preferred_service_for_capability(
        &self,
        capability: &str,
    ) -> Option<String> {
        let capability_map = self.capability_map.read().await;
        let discovered_services = self.discovered_services.read().await;
        
        if let Some(services) = capability_map.get(capability) {
            // Prioritize services based on performance and availability
            let mut best_service = None;
            let mut best_score = 0.0;
            
            for service_name in services {
                if let Some(service_info) = discovered_services.get(service_name) {
                    let score = calculate_service_score(service_info);
                    if score > best_score {
                        best_score = score;
                        best_service = Some(service_name.clone());
                    }
                }
            }
            
            best_service
        } else {
            None
        }
    }
}

fn calculate_service_score(service_info: &ServiceInfo) -> f64 {
    let availability_score = service_info.availability_percent / 100.0;
    let performance_score = 1.0 - (service_info.avg_response_time_ms / 1000.0).min(1.0);
    let load_score = 1.0 - (service_info.current_load_percent / 100.0);
    
    (availability_score * 0.4) + (performance_score * 0.3) + (load_score * 0.3)
}

#[derive(Deserialize, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub endpoint: String,
    pub status: String,
    pub availability_percent: f64,
    pub avg_response_time_ms: f64,
    pub current_load_percent: f64,
}
```

### **Graceful Fallback Implementation**

```rust
pub async fn execute_with_fallback<T, F, Fut>(
    primary_operation: F,
    fallback_operation: impl FnOnce() -> Fut,
    operation_name: &str,
) -> Result<T, NestGateError>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, NestGateError>>,
{
    match primary_operation().await {
        Ok(result) => {
            debug!("Primary operation '{}' succeeded", operation_name);
            Ok(result)
        }
        Err(e) => {
            warn!("Primary operation '{}' failed: {}, attempting fallback", operation_name, e);
            
            match fallback_operation().await {
                Ok(result) => {
                    info!("Fallback operation '{}' succeeded", operation_name);
                    Ok(result)
                }
                Err(fallback_error) => {
                    error!("Both primary and fallback operations failed for '{}': primary={}, fallback={}", 
                           operation_name, e, fallback_error);
                    Err(NestGateError::EcosystemIntegration {
                        message: format!("All operations failed for {}: {}", operation_name, e),
                        service: "ecosystem".to_string(),
                    })
                }
            }
        }
    }
}

// Usage example
pub async fn authenticate_with_fallback(
    adapter: &UniversalAdapter,
    token: &str,
) -> Result<UserContext, NestGateError> {
    execute_with_fallback(
        || authenticate_user(adapter, token),
        || authenticate_locally(token),
        "user_authentication",
    ).await
}
```

---

## 📊 **ECOSYSTEM MONITORING AND HEALTH**

### **Ecosystem Health Dashboard**

```rust
pub async fn get_ecosystem_health_status(
    adapter: &UniversalAdapter,
) -> Result<EcosystemHealthStatus, NestGateError> {
    let health_request = CapabilityRequest {
        capability: "system.health".to_string(),
        operation: "get_ecosystem_status".to_string(),
        payload: serde_json::json!({
            "include_services": ["beardog", "songbird", "squirrel", "toadstool"],
            "include_metrics": true,
            "include_connectivity": true,
        }).to_string().into_bytes(),
        metadata: HashMap::new(),
        timeout_ms: Some(5000),
        priority: Priority::High,
        performance_requirements: None,
    };

    match adapter.execute_capability(health_request).await {
        Ok(response) => {
            let health_status: EcosystemHealthStatus = 
                serde_json::from_slice(&response.payload)?;
            Ok(health_status)
        }
        Err(e) => {
            warn!("Ecosystem health check failed: {}", e);
            // Return degraded status
            Ok(EcosystemHealthStatus {
                overall_status: "degraded".to_string(),
                services: HashMap::new(),
                connectivity_status: "unknown".to_string(),
                last_updated: chrono::Utc::now(),
            })
        }
    }
}

#[derive(Deserialize)]
pub struct EcosystemHealthStatus {
    pub overall_status: String,
    pub services: HashMap<String, ServiceHealthInfo>,
    pub connectivity_status: String,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct ServiceHealthInfo {
    pub status: String,
    pub response_time_ms: f64,
    pub availability_percent: f64,
    pub active_connections: u32,
    pub last_seen: chrono::DateTime<chrono::Utc>,
}
```

---

## 🔧 **CONFIGURATION EXAMPLES**

### **Complete Ecosystem Integration Configuration**

```toml
# production.toml - Complete ecosystem integration

[ecosystem]
# Discovery configuration
discovery_enabled = true
discovery_timeout_ms = 30000
discovery_interval_seconds = 60
max_discovery_retries = 3

# Service preferences
preferred_security_primal = "beardog"
preferred_orchestration_primal = "songbird"
preferred_ai_primal = "squirrel"
preferred_compute_primal = "toadstool"

# Fallback behavior
enable_graceful_fallbacks = true
local_operation_on_failure = true
max_retry_attempts = 3
retry_backoff_ms = 1000
circuit_breaker_enabled = true

# Performance requirements
[ecosystem.performance]
default_timeout_ms = 30000
high_priority_timeout_ms = 10000
low_priority_timeout_ms = 60000

# Capability requirements
required_capabilities = [
    "storage.zfs",
    "storage.snapshots",
    "network.api",
    "monitoring.metrics"
]

optional_capabilities = [
    "security.authentication",
    "security.authorization", 
    "orchestration.workflow",
    "orchestration.pipeline",
    "ai.optimization",
    "ai.prediction",
    "compute.batch_processing",
    "compute.parallel_execution"
]

# Service-specific configurations
[ecosystem.beardog]
auth_timeout_ms = 5000
authz_timeout_ms = 3000
session_timeout_minutes = 60
max_auth_retries = 3

[ecosystem.songbird]
workflow_timeout_ms = 300000
pipeline_timeout_ms = 600000
max_concurrent_workflows = 10
enable_workflow_persistence = true

[ecosystem.squirrel]
analysis_timeout_ms = 30000
prediction_timeout_ms = 45000
optimization_timeout_ms = 60000
enable_model_caching = true

[ecosystem.toadstool]
job_timeout_ms = 3600000  # 1 hour
max_concurrent_jobs = 5
resource_reservation_enabled = true
enable_job_queuing = true

# Health monitoring
[ecosystem.health]
health_check_interval_seconds = 30
health_check_timeout_ms = 5000
unhealthy_threshold_count = 3
recovery_check_interval_seconds = 60

# Circuit breaker configuration
[ecosystem.circuit_breaker]
failure_threshold = 5
timeout_duration_seconds = 60
half_open_max_calls = 3
```

---

## 🎯 **BEST PRACTICES**

### **Integration Patterns**

1. **Capability-First Design**
   - Always request capabilities, not specific services
   - Design for service unavailability
   - Implement graceful degradation

2. **Timeout and Retry Strategy**
   - Set appropriate timeouts for each capability
   - Implement exponential backoff
   - Use circuit breakers for failing services

3. **Error Handling**
   - Log all ecosystem interactions
   - Provide meaningful error messages
   - Implement fallback mechanisms

4. **Performance Optimization**
   - Cache capability discovery results
   - Use connection pooling
   - Monitor ecosystem latency

### **Security Considerations**

1. **Token Management**
   - Use short-lived tokens for service communication
   - Implement token refresh mechanisms
   - Secure token storage

2. **Authorization**
   - Validate permissions for all operations
   - Implement least-privilege access
   - Audit all security decisions

3. **Communication Security**
   - Use TLS for all ecosystem communication
   - Validate service certificates
   - Implement message signing

---

## 📞 **TROUBLESHOOTING**

### **Common Integration Issues**

#### **Service Discovery Failures**
```bash
# Check universal adapter status
curl http://localhost:8080/api/v1/adapter/status

# Test individual service connectivity
curl http://beardog-service:8080/health
curl http://songbird-service:8080/health

# Review discovery logs
journalctl -u nestgate | grep -i discovery
```

#### **Authentication Failures**
```bash
# Test BearDog connectivity
curl http://localhost:8080/api/v1/ecosystem/services | grep beardog

# Check authentication configuration
grep -i auth /opt/nestgate/config/production.toml

# Review authentication logs
journalctl -u nestgate | grep -i auth
```

#### **Performance Issues**
```bash
# Monitor ecosystem response times
curl http://localhost:9090/metrics | grep ecosystem

# Check timeout configurations
grep -i timeout /opt/nestgate/config/production.toml

# Review performance logs
journalctl -u nestgate | grep -i "timeout\|slow"
```

---

**🌐 Ecosystem Integration Complete**  
**🔄 Automatic Service Discovery**  
**🛡️ Graceful Fallback Mechanisms**  
**📊 Comprehensive Monitoring**

**Your NestGate v2.0 ecosystem integration is production-ready!** 