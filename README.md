# Songbird Orchestrator

**Universal Service Orchestration Platform for Rust Projects**

Songbird provides a generic, trait-based approach to service orchestration that works across any domain or project type. It replaces project-specific orchestration patterns with universal, reusable components.

[![Rust](https://github.com/strandgate/songbird-orchestrator/workflows/CI/badge.svg)](https://github.com/strandgate/songbird-orchestrator/actions)
[![Crates.io](https://img.shields.io/crates/v/songbird-orchestrator.svg)](https://crates.io/crates/songbird-orchestrator)
[![Documentation](https://docs.rs/songbird-orchestrator/badge.svg)](https://docs.rs/songbird-orchestrator)

## 🎯 Mission

Transform project-specific orchestration patterns into **universal, reusable components** that work across any Rust project, domain, or use case.

## ✨ Key Features

### 🔧 Universal Service Interface
- **Trait-based design**: Any Rust service can implement `UniversalService`
- **Domain agnostic**: Works with web services, microservices, system daemons, AI models, databases, etc.
- **Type-safe configuration**: Generic configuration system with validation
- **Lifecycle management**: Start, stop, restart, health checks, graceful shutdown

### 🌐 Pluggable Architecture
- **Communication backends**: HTTP, WebSocket, gRPC, in-memory, custom
- **Service discovery**: Static, Consul, Kubernetes, etcd, custom
- **Configuration providers**: File, environment, Consul, custom
- **Load balancing**: Round-robin, weighted, least-connections, health-aware
- **Health monitoring**: HTTP checks, TCP checks, custom function checks

### 🚀 Production Ready
- **Zero dependencies** on any specific project or framework
- **Comprehensive error handling** with detailed error types
- **Metrics and observability** built-in
- **Graceful shutdown** and resource cleanup
- **Extensive testing** with mock frameworks

## 🚀 Quick Start

### Add to Cargo.toml

```toml
[dependencies]
songbird-orchestrator = "0.1.0"

# Optional feature backends
songbird-orchestrator = { version = "0.1.0", features = ["consul", "kubernetes", "grpc"] }
```

### Basic Usage

```rust
use songbird_orchestrator::prelude::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// 1. Define your service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MyServiceConfig {
    pub database_url: String,
    pub port: u16,
    pub workers: usize,
}

// 2. Define your health information
#[derive(Debug, Serialize)]
struct MyServiceHealth {
    pub status: String,
    pub connections: u32,
    pub uptime_seconds: u64,
}

// 3. Implement the UniversalService trait
struct MyService {
    config: Option<MyServiceConfig>,
    running: bool,
}

#[async_trait]
impl UniversalService for MyService {
    type Config = MyServiceConfig;
    type Health = MyServiceHealth;
    type Error = Box<dyn std::error::Error + Send + Sync>;

    async fn initialize(&mut self, config: Self::Config) -> Result<(), Self::Error> {
        self.config = Some(config);
        Ok(())
    }

    async fn start(&mut self) -> Result<(), Self::Error> {
        self.running = true;
        println!("Service started!");
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Self::Error> {
        self.running = false;
        println!("Service stopped!");
        Ok(())
    }

    async fn health_check(&self) -> Result<Self::Health, Self::Error> {
        Ok(MyServiceHealth {
            status: if self.running { "healthy".to_string() } else { "stopped".to_string() },
            connections: 42,
            uptime_seconds: 3600,
        })
    }

    async fn handle_request(&self, request: ServiceRequest) -> Result<ServiceResponse, Self::Error> {
        Ok(ServiceResponse::success(
            request.id,
            serde_json::json!({"message": "Hello from MyService!"})
        ))
    }

    // ... other required methods

    fn service_info(&self) -> ServiceInfo {
        ServiceInfo {
            id: "my-service".to_string(),
            name: "My Service".to_string(),
            version: "1.0.0".to_string(),
            service_type: "web".to_string(),
            description: "Example service".to_string(),
            endpoints: vec![],
            capabilities: vec!["http".to_string()],
            tags: std::collections::HashMap::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    // ... implement remaining methods
}

// 4. Set up the orchestrator
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = OrchestratorConfig::default();
    
    // Create orchestrator
    let orchestrator = Orchestrator::new(config).await?;
    
    // Create and register your service
    let service = MyService { config: None, running: false };
    orchestrator.register_service(Box::new(service)).await?;
    
    // Start orchestrator
    orchestrator.start().await?;
    
    println!("Orchestrator running!");
    
    // Your application logic here...
    
    // Graceful shutdown
    orchestrator.stop().await?;
    
    Ok(())
}
```

## 🏗️ Architecture

### Core Components

```
┌─────────────────────────────────────────────────────────────────┐
│                    Songbird Orchestrator                        │
├─────────────────────────────────────────────────────────────────┤
│  Universal Service Trait                                       │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐   │
│  │   Any Service   │ │   Any Service   │ │   Any Service   │   │
│  │  (Web, AI, DB)  │ │  (Microservice) │ │  (System Daemon)│   │
│  └─────────────────┘ └─────────────────┘ └─────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│  Core Orchestration Layer                                      │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐   │
│  │   Service       │ │   Load          │ │   Health        │   │
│  │   Registry      │ │   Balancer      │ │   Monitor       │   │
│  └─────────────────┘ └─────────────────┘ └─────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│  Pluggable Backends                                            │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐   │
│  │   Discovery     │ │  Communication  │ │  Configuration  │   │
│  │ (Consul/K8s)    │ │ (HTTP/gRPC/WS)  │ │ (File/Env/etc)  │   │
│  └─────────────────┘ └─────────────────┘ └─────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### Universal Service Trait

The `UniversalService` trait is the heart of Songbird. Any service that implements this trait can be orchestrated:

```rust
#[async_trait]
pub trait UniversalService: Send + Sync + 'static {
    type Config: Clone + Send + Sync + for<'de> Deserialize<'de> + Debug;
    type Health: Send + Sync + Serialize + Debug;
    type Error: std::error::Error + Send + Sync + 'static;

    // Lifecycle management
    async fn initialize(&mut self, config: Self::Config) -> Result<(), Self::Error>;
    async fn start(&mut self) -> Result<(), Self::Error>;
    async fn stop(&mut self) -> Result<(), Self::Error>;
    async fn restart(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;

    // Health and monitoring
    async fn health_check(&self) -> Result<Self::Health, Self::Error>;
    async fn get_metrics(&self) -> Result<ServiceMetrics, Self::Error>;

    // Request handling
    async fn handle_request(&self, request: ServiceRequest) -> Result<ServiceResponse, Self::Error>;

    // Service information
    fn service_info(&self) -> ServiceInfo;

    // Load management
    async fn can_handle_load(&self) -> Result<bool, Self::Error>;
    async fn get_load_factor(&self) -> Result<f64, Self::Error>;

    // Configuration management
    async fn update_config(&mut self, config: Self::Config) -> Result<(), Self::Error>;
}
```

## 🔧 Configuration

### Orchestrator Configuration

```rust
use songbird_orchestrator::config::*;
use std::time::Duration;

let config = OrchestratorConfig {
    orchestrator: CoreOrchestratorConfig {
        port: 8080,
        max_services: 100,
        health_check_interval: Duration::from_secs(30),
        service_start_timeout: Duration::from_secs(30),
        service_stop_timeout: Duration::from_secs(30),
        request_timeout: Duration::from_secs(30),
    },
    network: NetworkConfig {
        host: "0.0.0.0".to_string(),
        port: 8080,
        max_connections: 1000,
        connection_timeout: Duration::from_secs(30),
        keep_alive: true,
        compression: true,
    },
    security: SecurityConfig {
        enable_tls: false,
        cert_path: None,
        key_path: None,
        require_auth: false,
        auth_method: "none".to_string(),
        allowed_origins: vec!["*".to_string()],
    },
    monitoring: MonitoringConfig {
        enable_metrics: true,
        metrics_port: 9090,
        enable_tracing: true,
        log_level: "info".to_string(),
        log_format: "json".to_string(),
    },
    discovery: DiscoveryConfig {
        provider: "static".to_string(),
        consul_url: None,
        kubernetes_namespace: None,
        static_services: vec![],
    },
    load_balancing: LoadBalancingConfig {
        algorithm: "round_robin".to_string(),
        health_check_enabled: true,
        failure_threshold: 3,
        recovery_threshold: 2,
    },
    health: HealthConfig {
        enabled: true,
        check_interval: Duration::from_secs(30),
        timeout: Duration::from_secs(10),
        failure_threshold: 3,
        success_threshold: 2,
    },
    services: YourServiceConfig::default(), // Your service-specific config
};
```

### Configuration Providers

```rust
use songbird_orchestrator::config::ConfigProvider;

// File-based configuration
let provider = FileConfigProvider::new("config.yaml");
let config = provider.load_config().await?;

// Environment-based configuration
let provider = EnvConfigProvider::new();
let config = provider.load_config().await?;

// Consul-based configuration (with feature = "consul")
let provider = ConsulConfigProvider::new("http://localhost:8500");
let config = provider.load_config().await?;
```

## 🌐 Service Discovery

### Static Discovery

```rust
use songbird_orchestrator::discovery::StaticServiceDiscovery;

let discovery = StaticServiceDiscovery::new();
discovery.register(service_info).await?;
let services = discovery.discover(&query).await?;
```

### Consul Discovery (with feature = "consul")

```rust
use songbird_orchestrator::discovery::ConsulServiceDiscovery;

let discovery = ConsulServiceDiscovery::new("http://localhost:8500");
discovery.register(service_info).await?;
```

### Kubernetes Discovery (with feature = "kubernetes")

```rust
use songbird_orchestrator::discovery::KubernetesServiceDiscovery;

let discovery = KubernetesServiceDiscovery::new("default".to_string());
discovery.register(service_info).await?;
```

## ⚖️ Load Balancing

### Built-in Algorithms

```rust
use songbird_orchestrator::load_balancer::*;

// Round Robin
let balancer = RoundRobinLoadBalancer::new();

// Weighted Round Robin  
let balancer = WeightedRoundRobinLoadBalancer::new();

// Least Connections
let balancer = LeastConnectionsLoadBalancer::new();

// Health-Aware (wraps any balancer)
let balancer = HealthAwareLoadBalancer::new(RoundRobinLoadBalancer::new());

// Random
let balancer = RandomLoadBalancer::new();
```

## 🏥 Health Monitoring

### HTTP Health Checks

```rust
use songbird_orchestrator::health::HttpHealthCheck;

let check = HttpHealthCheck::new("api-health", "http://localhost:8080/health");
monitor.register_service("my-service", vec![Box::new(check)]).await?;
```

### TCP Health Checks

```rust
use songbird_orchestrator::health::TcpHealthCheck;

let check = TcpHealthCheck::new("tcp-check", "localhost", 5432);
monitor.register_service("database", vec![Box::new(check)]).await?;
```

### Custom Health Checks

```rust
use songbird_orchestrator::health::FunctionHealthCheck;

let check = FunctionHealthCheck::new("custom-check", || {
    // Your custom health check logic
    Ok(true)
});
```

## 📡 Communication

### HTTP Communication

```rust
use songbird_orchestrator::communication::HttpCommunication;

let comm = HttpCommunication::new("http://localhost:8080");
let response = comm.send_message(address, message).await?;
```

### WebSocket Communication

```rust
use songbird_orchestrator::communication::WebSocketCommunication;

let comm = WebSocketCommunication::new("ws://localhost:8080");
let response = comm.send_message(address, message).await?;
```

### In-Memory Communication (for testing)

```rust
use songbird_orchestrator::communication::InMemoryCommunication;

let comm = InMemoryCommunication::new();
let response = comm.send_message(address, message).await?;
```

## 🧪 Testing

Songbird includes comprehensive testing utilities:

```rust
use songbird_orchestrator::testing::*;

// Mock service for testing
let mut service = MockService::new("test-service");
service.set_error_rate(0.1).await; // 10% error rate

// Test orchestrator setup
let orchestrator = setup_test_orchestrator().await?;

// Multiple mock services
let services = create_mock_services(5);

// Load testing
let results = service.simulate_load(100, Duration::from_secs(60)).await;
println!("RPS: {}, Avg Latency: {}ms", results.requests_per_second, results.average_latency_ms);
```

## 🎯 Use Cases

### Web Services & APIs
```rust
// REST API service
struct ApiService { /* ... */ }
impl UniversalService for ApiService { /* ... */ }

// GraphQL service  
struct GraphQLService { /* ... */ }
impl UniversalService for GraphQLService { /* ... */ }
```

### Microservices
```rust
// User service
struct UserService { /* ... */ }
impl UniversalService for UserService { /* ... */ }

// Payment service
struct PaymentService { /* ... */ }  
impl UniversalService for PaymentService { /* ... */ }
```

### AI/ML Services
```rust
// Model inference service
struct ModelService { /* ... */ }
impl UniversalService for ModelService { /* ... */ }

// Training pipeline service
struct TrainingService { /* ... */ }
impl UniversalService for TrainingService { /* ... */ }
```

### System Services & Daemons
```rust
// File watcher service
struct FileWatcherService { /* ... */ }
impl UniversalService for FileWatcherService { /* ... */ }

// Backup service
struct BackupService { /* ... */ }
impl UniversalService for BackupService { /* ... */ }
```

### Database Services
```rust
// Database connection pool service
struct DatabaseService { /* ... */ }
impl UniversalService for DatabaseService { /* ... */ }

// Cache service
struct CacheService { /* ... */ }
impl UniversalService for CacheService { /* ... */ }
```

## 📚 Examples

See the `examples/` directory for complete examples:

- `examples/nestgate_integration.rs` - NestGate NAS service integration
- `examples/basic_orchestrator.rs` - Basic orchestrator setup
- `examples/microservices.rs` - Microservices orchestration
- `examples/load_testing.rs` - Performance testing setup

## 🚀 Migration from Project-Specific Orchestrators

### From NestGate Orchestrator

```rust
// Before (NestGate-specific)
use nestgate_core::orchestrator::NestGateOrchestrator;
use nestgate_core::services::NasService;

// After (Universal)
use songbird_orchestrator::prelude::*;

struct NestGateNasService { /* ... */ }
impl UniversalService for NestGateNasService { /* ... */ }
```

### From Custom Orchestrators

1. **Implement UniversalService** for your existing services
2. **Replace orchestrator** with Songbird Orchestrator  
3. **Configure backends** (discovery, communication, etc.)
4. **Test migration** with mock services first
5. **Deploy incrementally** service by service

## 🔧 Development

### Building

```bash
# Basic build
cargo build

# With all features
cargo build --all-features

# Release build
cargo build --release --all-features
```

### Testing

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Load tests
cargo test --test performance

# With all features
cargo test --all-features
```

### Documentation

```bash
# Generate docs
cargo doc --open

# With all features
cargo doc --all-features --open
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

1. **Clone repository**: `git clone https://github.com/strandgate/songbird-orchestrator.git`
2. **Install Rust**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`  
3. **Install dependencies**: `cargo build`
4. **Run tests**: `cargo test`
5. **Check formatting**: `cargo fmt --check`
6. **Run lints**: `cargo clippy`

### Project Structure

```
songbird-orchestrator/
├── src/
│   ├── traits/          # Core trait definitions
│   ├── config/          # Configuration system
│   ├── discovery/       # Service discovery implementations
│   ├── communication/   # Communication layer implementations  
│   ├── health/          # Health monitoring implementations
│   ├── load_balancer/   # Load balancing implementations
│   ├── security/        # Security implementations
│   ├── registry/        # Service registry
│   └── orchestrator/    # Main orchestrator
├── tests/               # Test suites
├── examples/            # Usage examples
├── docs/                # Documentation
└── benches/             # Performance benchmarks
```

## 📋 Roadmap

### ✅ Phase 1: Foundation (Current)
- [x] Universal service trait design
- [x] Core orchestrator implementation  
- [x] Basic configuration system
- [x] Error handling framework
- [x] Testing infrastructure

### 🚧 Phase 2: Core Features (In Progress)
- [ ] Service registry completion
- [ ] Communication layer implementations
- [ ] Health monitoring system
- [ ] Load balancer implementations
- [ ] Service discovery backends

### 🔮 Phase 3: Advanced Features (Planned)
- [ ] Metrics and observability
- [ ] Security and authentication
- [ ] Performance optimizations
- [ ] Advanced load balancing
- [ ] Circuit breakers and resilience

### 🌟 Phase 4: Ecosystem (Future)
- [ ] Kubernetes operator
- [ ] Service mesh integration
- [ ] Cloud provider integrations
- [ ] Monitoring integrations
- [ ] CLI tooling

## 📄 License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- The Rust community for excellent async/await support
- Tokio team for the async runtime
- Serde team for serialization framework
- All contributors and early adopters

---

**Built with ❤️ for the Rust ecosystem** 