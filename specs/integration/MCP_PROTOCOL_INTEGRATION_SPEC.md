---
title: MCP Protocol Integration Specification
description: Detailed specification for integrating GitClone MCP protocol into v2 orchestrator-centric architecture
version: 2.0.0
date: 2025-01-26
status: Integration Specification
---

# MCP Protocol Integration Specification

## Overview

This specification details the integration of the mature MCP (Machine Context Protocol) implementation from the `nestgate-gitclone` repository into our v2 orchestrator-centric architecture. The integration will enhance our federation capabilities while maintaining the orchestrator as the central connectivity hub.

## Integration Objectives

### Primary Goals
1. **Enhance Federation**: Add sophisticated MCP federation capabilities to v2 orchestrator
2. **Maintain Architecture**: Preserve orchestrator-centric communication patterns
3. **Improve Reliability**: Leverage proven MCP protocol implementation
4. **Enable Capabilities**: Support advanced MCP cluster participation

### Success Criteria
- ✅ Orchestrator can connect to MCP clusters through integrated protocol
- ✅ Storage capabilities properly registered with MCP
- ✅ All MCP messages routed through orchestrator
- ✅ Graceful degradation when MCP unavailable
- ✅ No performance degradation in standalone mode

## Component Analysis

### GitClone MCP Components to Integrate

#### 1. Protocol Definitions (`nestgate-protocol/src/proto/`)
```protobuf
// MCP core message structure (from GitClone)
syntax = "proto3";

message Message {
  Header header = 1;
  oneof body {
    Command command = 2;
    Response response = 3;
    Event event = 4;
    State state = 5;
  }
}

message Header {
  string message_id = 1;
  string correlation_id = 2;
  int64 timestamp = 3;
  string source = 4;
  string destination = 5;
}

message Capability {
  string name = 1;
  string version = 2;
  repeated Parameter inputs = 3;
  repeated Parameter outputs = 4;
  string description = 5;
}

service MCP {
  rpc Stream(stream Message) returns (stream Message) {}
  rpc Execute(Command) returns (Response) {}
  rpc Subscribe(Command) returns (stream Event) {}
  rpc UpdateState(stream State) returns (Response) {}
}
```

#### 2. Message Handling (`nestgate-protocol/src/handlers/`)
```rust
// GitClone message handler pattern
pub trait MessageHandler {
    async fn handle(&self, message: Message) -> Result<Response, McpError>;
    fn capabilities(&self) -> Vec<Capability>;
}

pub struct MessageRouter {
    handlers: HashMap<String, Box<dyn MessageHandler>>,
}

impl MessageRouter {
    pub async fn route_message(&self, message: Message) -> Result<Response, McpError> {
        // Route messages to appropriate handlers
    }
}
```

#### 3. Capability System (`nestgate-protocol/src/capabilities/`)
```rust
// GitClone capability management
pub struct CapabilityManager {
    registered_capabilities: HashMap<String, Capability>,
    capability_handlers: HashMap<String, Box<dyn CapabilityHandler>>,
}

impl CapabilityManager {
    pub async fn register_capability(&mut self, capability: Capability, handler: Box<dyn CapabilityHandler>) {
        // Register capability with handler
    }
    
    pub async fn execute_capability(&self, name: &str, inputs: Vec<Parameter>) -> Result<Vec<Parameter>, McpError> {
        // Execute capability and return results
    }
}
```

## v2 Integration Architecture

### Orchestrator MCP Integration Layer
```rust
// code/crates/nestgate-orchestrator/src/mcp_integration.rs
use nestgate_protocol::{Message, Capability, McpService, MessageHandler};

pub struct OrchestratorMcpIntegration {
    orchestrator: Arc<Orchestrator>,
    mcp_service: McpService,
    capability_manager: CapabilityManager,
    message_router: MessageRouter,
}

impl OrchestratorMcpIntegration {
    pub async fn new(orchestrator: Arc<Orchestrator>) -> Result<Self, McpError> {
        let mut integration = Self {
            orchestrator,
            mcp_service: McpService::new(),
            capability_manager: CapabilityManager::new(),
            message_router: MessageRouter::new(),
        };
        
        // Register orchestrator capabilities
        integration.register_orchestrator_capabilities().await?;
        
        Ok(integration)
    }
    
    async fn register_orchestrator_capabilities(&mut self) -> Result<(), McpError> {
        // Register v2 orchestrator capabilities with MCP
        let storage_capability = Capability {
            name: "nestgate_storage_management".to_string(),
            version: "2.0.0".to_string(),
            inputs: vec![
                Parameter {
                    name: "operation".to_string(),
                    param_type: ParameterType::String,
                    required: true,
                    description: "Storage operation type".to_string(),
                },
                Parameter {
                    name: "tier".to_string(),
                    param_type: ParameterType::String,
                    required: false,
                    description: "Storage tier (hot/warm/cold)".to_string(),
                },
            ],
            outputs: vec![
                Parameter {
                    name: "result".to_string(),
                    param_type: ParameterType::Map,
                    required: true,
                    description: "Operation result".to_string(),
                },
            ],
            description: "NestGate v2 storage management capabilities".to_string(),
        };
        
        let handler = Box::new(OrchestratorStorageHandler::new(self.orchestrator.clone()));
        self.capability_manager.register_capability(storage_capability, handler).await;
        
        Ok(())
    }
}
```

### Orchestrator Storage Handler
```rust
// code/crates/nestgate-orchestrator/src/mcp_handlers/storage.rs
pub struct OrchestratorStorageHandler {
    orchestrator: Arc<Orchestrator>,
}

impl OrchestratorStorageHandler {
    pub fn new(orchestrator: Arc<Orchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait]
impl CapabilityHandler for OrchestratorStorageHandler {
    async fn execute(&self, inputs: Vec<Parameter>) -> Result<Vec<Parameter>, McpError> {
        // Parse inputs
        let operation = inputs.iter()
            .find(|p| p.name == "operation")
            .ok_or(McpError::MissingParameter("operation".to_string()))?
            .value.as_str()
            .ok_or(McpError::InvalidParameter("operation must be string".to_string()))?;
        
        // Route through orchestrator to appropriate service
        let request = match operation {
            "create_pool" => self.handle_create_pool(inputs).await?,
            "list_pools" => self.handle_list_pools().await?,
            "get_pool_status" => self.handle_get_pool_status(inputs).await?,
            "create_export" => self.handle_create_export(inputs).await?,
            _ => return Err(McpError::UnsupportedOperation(operation.to_string())),
        };
        
        // Convert orchestrator response to MCP parameters
        Ok(vec![Parameter {
            name: "result".to_string(),
            param_type: ParameterType::Map,
            value: request.into(),
            required: true,
            description: "Operation result".to_string(),
        }])
    }
    
    async fn handle_create_pool(&self, inputs: Vec<Parameter>) -> Result<serde_json::Value, McpError> {
        // Route to nestgate-core through orchestrator
        let request = CreatePoolRequest::from_mcp_parameters(inputs)?;
        let response = self.orchestrator
            .route_request("/api/storage/pools", request.into())
            .await
            .map_err(|e| McpError::OrchestrationError(e.to_string()))?;
        
        Ok(response.into())
    }
}
```

### MCP Federation Handler Enhancement
```rust
// code/crates/nestgate-orchestrator/src/mcp_federation.rs (Enhanced)
use nestgate_protocol::{McpClient, Message, Capability};

pub struct EnhancedMcpFederation {
    orchestrator: Arc<Orchestrator>,
    mcp_integration: Arc<OrchestratorMcpIntegration>,
    mcp_client: Option<McpClient>,
    federation_mode: FederationMode,
}

impl EnhancedMcpFederation {
    pub async fn new(orchestrator: Arc<Orchestrator>) -> Result<Self, FederationError> {
        let mcp_integration = Arc::new(OrchestratorMcpIntegration::new(orchestrator.clone()).await?);
        
        Ok(Self {
            orchestrator,
            mcp_integration,
            mcp_client: None,
            federation_mode: FederationMode::AutoDetect,
        })
    }
    
    pub async fn connect_to_mcp_cluster(&mut self, endpoint: &str) -> Result<(), FederationError> {
        // Create MCP client using integrated protocol
        let mut client = McpClient::new(endpoint).await?;
        
        // Register capabilities with MCP cluster
        let capabilities = self.mcp_integration.capability_manager.get_capabilities();
        for capability in capabilities {
            client.register_capability(capability).await?;
        }
        
        // Start message handling loop
        let integration = self.mcp_integration.clone();
        let message_handler = tokio::spawn(async move {
            while let Some(message) = client.receive_message().await {
                if let Err(e) = integration.handle_mcp_message(message).await {
                    error!("Failed to handle MCP message: {}", e);
                }
            }
        });
        
        self.mcp_client = Some(client);
        
        Ok(())
    }
    
    pub async fn handle_mcp_message(&self, message: Message) -> Result<(), McpError> {
        // Route MCP messages through orchestrator
        match message.body {
            Some(MessageBody::Command(command)) => {
                let response = self.mcp_integration
                    .capability_manager
                    .execute_capability(&command.capability, command.inputs)
                    .await?;
                
                // Send response back through MCP client
                if let Some(client) = &self.mcp_client {
                    client.send_response(Response {
                        correlation_id: message.header.message_id,
                        outputs: response,
                        status: ResponseStatus::Success,
                    }).await?;
                }
            }
            Some(MessageBody::Event(event)) => {
                // Handle MCP events (cluster updates, etc.)
                self.handle_mcp_event(event).await?;
            }
            _ => {
                warn!("Received unsupported MCP message type");
            }
        }
        
        Ok(())
    }
}
```

## Integration Implementation Plan

### Phase 1: Protocol Foundation (Week 1)
```yaml
tasks:
  day_1_2:
    - Copy nestgate-protocol crate to code/crates/
    - Update Cargo.toml with protocol dependencies
    - Resolve any dependency conflicts
    - Basic compilation and testing
  
  day_3_4:
    - Create orchestrator MCP integration module
    - Implement basic message routing
    - Add capability registration framework
    - Unit tests for core functionality
  
  day_5:
    - Integration testing with mock MCP server
    - Performance baseline measurements
    - Documentation updates
```

### Phase 2: Capability Integration (Week 2)
```yaml
tasks:
  day_1_2:
    - Implement storage management capabilities
    - Create orchestrator capability handlers
    - Add service routing through orchestrator
    - Test capability execution
  
  day_3_4:
    - Implement network management capabilities
    - Add ZFS integration capabilities
    - Create metadata management capabilities
    - Integration testing
  
  day_5:
    - End-to-end testing with real MCP cluster
    - Performance optimization
    - Error handling improvements
```

## Configuration Integration

### Enhanced Orchestrator Configuration
```yaml
# config/orchestrator.yaml (Enhanced for MCP)
orchestrator:
  bind_address: "0.0.0.0:8080"
  log_level: "info"
  
  mcp_integration:
    enabled: true
    protocol_version: "1.0"
    capabilities:
      storage_management:
        enabled: true
        version: "2.0.0"
      network_management:
        enabled: true
        version: "2.0.0"
      zfs_integration:
        enabled: true
        version: "2.0.0"
  
  federation:
    mode: "auto_detect"  # standalone | auto_detect | federated
    mcp_endpoints:
      - "https://mcp-cluster-1.local:8443"
      - "https://mcp-cluster-2.local:8443"
    
    connection:
      timeout: 30s
      retry_attempts: 3
      heartbeat_interval: 30s
    
    capabilities:
      advertise_storage: true
      advertise_network: true
      advertise_zfs: true
```

## Testing Strategy

### Unit Tests
```rust
// tests/unit/mcp_integration_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_protocol::testing::MockMcpClient;
    
    #[tokio::test]
    async fn test_capability_registration() {
        let orchestrator = Arc::new(MockOrchestrator::new());
        let integration = OrchestratorMcpIntegration::new(orchestrator).await.unwrap();
        
        let capabilities = integration.capability_manager.get_capabilities();
        assert!(!capabilities.is_empty());
        assert!(capabilities.iter().any(|c| c.name == "nestgate_storage_management"));
    }
    
    #[tokio::test]
    async fn test_storage_capability_execution() {
        let orchestrator = Arc::new(MockOrchestrator::new());
        let integration = OrchestratorMcpIntegration::new(orchestrator).await.unwrap();
        
        let inputs = vec![
            Parameter {
                name: "operation".to_string(),
                param_type: ParameterType::String,
                value: "list_pools".into(),
                required: true,
                description: "".to_string(),
            }
        ];
        
        let result = integration.capability_manager
            .execute_capability("nestgate_storage_management", inputs)
            .await
            .unwrap();
        
        assert!(!result.is_empty());
    }
    
    #[tokio::test]
    async fn test_message_routing() {
        let orchestrator = Arc::new(MockOrchestrator::new());
        let integration = OrchestratorMcpIntegration::new(orchestrator).await.unwrap();
        
        let message = Message {
            header: Header {
                message_id: "test-123".to_string(),
                correlation_id: "corr-456".to_string(),
                timestamp: chrono::Utc::now().timestamp(),
                source: "test-client".to_string(),
                destination: "nestgate".to_string(),
            },
            body: Some(MessageBody::Command(Command {
                capability: "nestgate_storage_management".to_string(),
                inputs: vec![],
            })),
        };
        
        let result = integration.handle_mcp_message(message).await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests
```rust
// tests/integration/mcp_federation_test.rs
#[cfg(test)]
mod integration_tests {
    use super::*;
    use nestgate_protocol::testing::TestMcpCluster;
    
    #[tokio::test]
    async fn test_mcp_cluster_connection() {
        let test_cluster = TestMcpCluster::new().await;
        let orchestrator = Arc::new(TestOrchestrator::new());
        let mut federation = EnhancedMcpFederation::new(orchestrator).await.unwrap();
        
        // Test connection to MCP cluster
        let result = federation.connect_to_mcp_cluster(&test_cluster.endpoint()).await;
        assert!(result.is_ok());
        
        // Verify capabilities were registered
        let registered_capabilities = test_cluster.get_registered_capabilities().await;
        assert!(registered_capabilities.contains(&"nestgate_storage_management".to_string()));
    }
    
    #[tokio::test]
    async fn test_graceful_degradation() {
        let orchestrator = Arc::new(TestOrchestrator::new());
        let mut federation = EnhancedMcpFederation::new(orchestrator).await.unwrap();
        
        // Start in auto-detect mode
        federation.start_auto_detect().await.unwrap();
        
        // Verify standalone operation when no MCP cluster available
        assert_eq!(federation.get_status().await, FederationStatus::Standalone);
        
        // Verify orchestrator still functions normally
        let health = federation.orchestrator.get_health().await.unwrap();
        assert_eq!(health.status, HealthStatus::Healthy);
    }
}
```

## Performance Considerations

### Benchmarks
```rust
// benches/mcp_integration_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_capability_execution(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("storage_capability_execution", |b| {
        b.to_async(&rt).iter(|| async {
            let orchestrator = Arc::new(MockOrchestrator::new());
            let integration = OrchestratorMcpIntegration::new(orchestrator).await.unwrap();
            
            let inputs = vec![Parameter {
                name: "operation".to_string(),
                param_type: ParameterType::String,
                value: "list_pools".into(),
                required: true,
                description: "".to_string(),
            }];
            
            black_box(
                integration.capability_manager
                    .execute_capability("nestgate_storage_management", inputs)
                    .await
                    .unwrap()
            );
        });
    });
}

fn benchmark_message_routing(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("mcp_message_routing", |b| {
        b.to_async(&rt).iter(|| async {
            let orchestrator = Arc::new(MockOrchestrator::new());
            let integration = OrchestratorMcpIntegration::new(orchestrator).await.unwrap();
            
            let message = create_test_message();
            
            black_box(
                integration.handle_mcp_message(message).await.unwrap()
            );
        });
    });
}

criterion_group!(benches, benchmark_capability_execution, benchmark_message_routing);
criterion_main!(benches);
```

## Security Considerations

### MCP Security Integration
```rust
// code/crates/nestgate-orchestrator/src/mcp_security.rs
pub struct McpSecurityManager {
    orchestrator_auth: Arc<SecurityManager>,
    mcp_credentials: McpCredentials,
    capability_permissions: HashMap<String, Vec<Permission>>,
}

impl McpSecurityManager {
    pub async fn authenticate_mcp_request(&self, message: &Message) -> Result<Principal, SecurityError> {
        // Authenticate MCP request
        // Verify message signature
        // Extract principal information
    }
    
    pub async fn authorize_capability_execution(&self, principal: &Principal, capability: &str) -> Result<(), SecurityError> {
        // Check if principal has permission to execute capability
        // Verify capability-specific permissions
        // Log security events
    }
}
```

## Migration Strategy

### Gradual Integration Approach
1. **Week 1**: Integrate protocol foundation without affecting existing federation
2. **Week 2**: Add capability system alongside existing MCP federation
3. **Week 3**: Switch to integrated MCP protocol for new connections
4. **Week 4**: Migrate existing connections to integrated protocol
5. **Week 5**: Remove old MCP federation code
6. **Week 6**: Performance optimization and documentation

### Rollback Plan
- Keep existing MCP federation code until integration proven stable
- Feature flag for enabling/disabling integrated MCP protocol
- Automated testing to verify no regression in standalone mode
- Performance monitoring to detect any degradation

## Success Metrics

### Technical Metrics
- **Protocol Compatibility**: 100% compatibility with existing MCP clusters
- **Performance**: <5ms additional latency for MCP message routing
- **Reliability**: >99.9% message delivery success rate
- **Capability Coverage**: All v2 orchestrator capabilities exposed via MCP

### Operational Metrics
- **Connection Success**: >95% MCP cluster connection success rate
- **Graceful Degradation**: <1s fallback time to standalone mode
- **Error Recovery**: <30s recovery time from MCP connection loss
- **Resource Usage**: <10% additional memory/CPU usage

## Documentation Requirements

### Developer Documentation
- MCP integration architecture guide
- Capability development tutorial
- Testing framework documentation
- Performance optimization guide

### Operational Documentation
- MCP cluster configuration guide
- Troubleshooting guide
- Security configuration guide
- Monitoring and alerting setup

## Summary

The MCP protocol integration will significantly enhance NestGate v2's federation capabilities while maintaining the orchestrator-centric architecture. By leveraging the proven MCP implementation from GitClone, we can:

1. **Accelerate Development**: Reuse mature protocol implementation
2. **Enhance Capabilities**: Add sophisticated MCP cluster participation
3. **Maintain Architecture**: Preserve orchestrator as central hub
4. **Improve Reliability**: Benefit from proven protocol handling

The integration follows a systematic, low-risk approach that ensures compatibility with existing v2 functionality while adding powerful new federation capabilities. 