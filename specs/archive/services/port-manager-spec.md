# NestGate Port Manager Specification

## Overview

The NestGate Port Manager is a service coordination layer that handles dynamic port allocation, service discovery, and lifecycle management across the NestGate ecosystem. It solves problems with hardcoded ports, connection failures, and service coordination by providing a central registry and management system.

## Goals

- Eliminate hardcoded port configurations across services
- Improve resilience through dynamic port allocation
- Simplify service startup and shutdown processes
- Provide central monitoring of service health
- Support cross-platform operation (Linux, Windows, macOS)

## Architecture

The Port Manager will be implemented in Rust with TypeScript bindings for frontend integration:

1. **Rust Core Service**
   - Central port management daemon
   - Port discovery and allocation
   - Process management
   - Health monitoring

2. **TypeScript Client SDK**
   - Service discovery
   - Connection management
   - Configuration generation

## Functional Requirements

### Port Management

- Scan and identify available ports in configurable ranges
- Allocate ports to services based on priority and requirements
- Store port assignments persistently
- Handle port conflicts gracefully
- Release ports when services terminate

### Service Registration

- Allow services to register with the port manager
- Define service dependencies
- Specify port requirements (preferred port, port range)
- Track service metadata (name, type, version)

### Service Lifecycle Management

- Start services in the correct dependency order
- Restart failed services with exponential backoff
- Gracefully shut down services
- Handle cleanup operations
- Monitor service health

### Configuration Management

- Generate dynamic configuration for services
- Provide connection information to clients
- Support environment variable substitution
- Manage secrets and credentials

### API Interface

- REST API for service registration and management
- WebSocket interface for real-time updates
- CLI tools for local development

## Technical Implementation

### Rust Core Architecture

```rust
// Core structs and interfaces
pub struct PortManager {
    service_registry: ServiceRegistry,
    port_allocator: PortAllocator,
    process_manager: ProcessManager,
    health_monitor: HealthMonitor,
}

pub struct ServiceDefinition {
    id: String,
    name: String,
    service_type: ServiceType,
    dependencies: Vec<String>,
    preferred_port: Option<u16>,
    port_range: Option<(u16, u16)>,
    startup_command: String,
    shutdown_command: Option<String>,
}

pub struct AllocatedService {
    definition: ServiceDefinition,
    allocated_port: u16,
    process_id: Option<u32>,
    status: ServiceStatus,
    health: ServiceHealth,
    urls: HashMap<String, String>,
    metrics: ServiceMetrics,
}
```

### Port Allocation Strategy

1. **Sequential Allocation**
   - Start with preferred port if specified
   - Check if port is available
   - Increment until an available port is found within range
   - Fall back to dynamic assignment if range is exhausted

2. **Range Management**
   - Define service-specific ranges (e.g., UI: 3000-3999, API: 4000-4999)
   - Reserve ports for critical services
   - Allow port exclusions for system services

3. **Availability Detection**
   - TCP connection attempt
   - Permission checks
   - System service checks

### Service Startup Flow

1. Build dependency graph from registered services
2. Perform topological sort to determine startup order
3. Allocate ports to each service
4. Start services in order, injecting port configuration
5. Verify service health before starting dependent services
6. Report completion status

### Health Monitoring

- Regular health checks via HTTP/TCP probes
- Process status monitoring
- Resource usage tracking (CPU, memory)
- Log monitoring
- Automatic recovery actions

## TypeScript Client Integration

```typescript
// Client SDK
class PortManagerClient {
  constructor(managerUrl: string) {}
  
  // Connect to port manager
  async connect(): Promise<boolean> {}
  
  // Register the current service
  async registerService(definition: ServiceDefinition): Promise<ServiceInfo> {}
  
  // Get connection info for a dependent service
  async getServiceConnection(serviceId: string): Promise<ConnectionInfo> {}
  
  // Get all available services
  async discoverServices(): Promise<ServiceInfo[]> {}
  
  // Report health status
  async reportHealth(status: HealthStatus): Promise<void> {}
}

// React Hook for Port Manager integration
function usePortManager() {
  // Service discovery and connection management
  const [services, setServices] = useState<ServiceInfo[]>([]);
  const [connections, setConnections] = useState<Record<string, ConnectionInfo>>({});
  
  // Service connection helper
  const connectToService = useCallback((serviceId: string) => {
    // Handle connection with current port info
  }, [connections]);
  
  return {
    services,
    connections,
    connectToService
  };
}
```

## Configuration and Deployment

### Default Port Ranges

| Service Type | Port Range | Priority | Notes |
|--------------|------------|----------|-------|
| UI           | 3000-3999  | Medium   | Web interfaces |
| API          | 4000-4999  | High     | Server APIs |
| WebSocket    | 5000-5999  | High     | Real-time data |
| Database     | 6000-6999  | Critical | Data storage |
| Metrics      | 7000-7999  | Low      | Monitoring |
| Admin        | 8000-8999  | Medium   | Admin tools |

### Environment Variables

- `NESTGATE_PORT_MANAGER_HOST`: Hostname for port manager (default: localhost)
- `NESTGATE_PORT_MANAGER_PORT`: Port for port manager (default: 9000)
- `NESTGATE_PORT_RANGES`: JSON configuration of port ranges
- `NESTGATE_SERVICE_CONFIG`: Path to service configuration file
- `NESTGATE_AUTO_RECOVERY`: Enable automatic service recovery (default: true)

### Configuration File

```yaml
# port-manager.yaml
manager:
  host: localhost
  port: 9000
  log_level: info
  persistence:
    enabled: true
    path: ./port-manager-state.json

port_ranges:
  ui: [3000, 3999]
  api: [4000, 4999]
  websocket: [5000, 5999]
  
services:
  backend_api:
    type: api
    startup: "./server/start.sh"
    dependencies: []
    preferred_port: 4000
    
  frontend:
    type: ui
    startup: "npm run start"
    dependencies: ["backend_api"]
    preferred_port: 3000
```

## Development Plan

### Phase 1: Core Infrastructure

1. Implement Rust port allocation and management
2. Build basic service registry
3. Create simple REST API for service registration
4. Develop CLI for local testing

### Phase 2: Process Management

1. Implement process startup and shutdown
2. Build dependency resolution system
3. Add basic health monitoring
4. Develop state persistence

### Phase 3: Client Integration

1. Create TypeScript SDK
2. Develop React hooks for UI integration
3. Implement service discovery components
4. Add connection management utilities

### Phase 4: Testing and Deployment

1. Develop integration tests
2. Create automated deployment pipeline
3. Implement monitoring and metrics
4. Documentation and user guides

## Implementation Status (Updated May 2025)

### Completed Components

1. **Port Manager Core Service**
   - ✅ Basic port allocation and management
   - ✅ Service registry implementation
   - ✅ REST API endpoints
   - ✅ Health check system

2. **TypeScript Integration**
   - ✅ TypeScript client SDK (`src/services/port-manager.ts`)
   - ✅ React hooks for port manager integration (`src/hooks/usePortManager.ts`)
   - ✅ FileSystem Monitor integration (`src/hooks/useFileSystemMonitor.ts`)

3. **Service Orchestration**
   - ✅ Start/stop scripts for services
   - ✅ Service toggling functionality
   - ✅ Process management and cleanup
   - ✅ Cross-platform support (Linux/Mac and Windows)

4. **Scripts and Utilities**
   - ✅ Toggle scripts for port manager (`scripts/toggle-port-manager.sh`/`.ps1`)
   - ✅ Toggle scripts for live service mode (`scripts/toggle-live-service.sh`/`.ps1`)
   - ✅ Streamlined npm commands for service management
   - ✅ Removal of redundant scripts and consolidation into toggle functionality

### In Progress

1. **Health Monitoring**
   - 🟡 Advanced health check implementations
   - 🟡 Failure recovery mechanisms
   - 🟡 Service metrics collection

2. **Testing Framework**
   - 🟡 Integration tests for port manager
   - 🟡 Unit tests for client SDK
   - 🟡 End-to-end system tests

3. **UI Components**
   - 🟡 Service management dashboard
   - 🟡 Port visualization
   - 🟡 Health status indicators

### Not Started

1. **Advanced Features**
   - ⚪ Automatic service discovery
   - ⚪ Load balancing
   - ⚪ High availability clustering
   - ⚪ Configuration generation

2. **Documentation**
   - ⚪ API documentation
   - ⚪ Developer guides
   - ⚪ Deployment guides
   - ⚪ Troubleshooting documentation

### Next Steps

1. Complete the testing framework with comprehensive test coverage
2. Implement advanced health monitoring and recovery
3. Create a service management dashboard UI
4. Improve documentation and developer guides
5. Address performance optimizations and resource usage

## Benefits

1. **Developer Experience**
   - No more port conflicts during local development
   - Simplified service startup
   - Automatic configuration

2. **Operational Reliability**
   - Dynamic adaptation to environment constraints
   - Automatic recovery from failures
   - Centralized monitoring

3. **Deployment Flexibility**
   - Support for multiple deployment scenarios
   - Environment-specific configuration
   - Containerization support

## Compatibility

The Port Manager will be compatible with all existing NestGate services and will provide a migration path from static to dynamic port configuration. 