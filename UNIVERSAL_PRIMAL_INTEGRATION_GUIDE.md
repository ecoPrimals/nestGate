# 🌐 NestGate Universal Primal Integration Guide

## 🎯 **Overview**

NestGate implements a **universal primal architecture** that enables seamless integration with any primal ecosystem. This agnostic approach allows NestGate to work with:

- **🐕 BearDog** (Security)
- **🐿️ Squirrel** (AI Integration)
- **🎵 Songbird** (Distribution)
- **🍄 Toadstool** (Compute)
- **🔮 Future Primals** (Zero Code Changes)

## 🏗️ **Architecture Philosophy**

### **Universal Primal Principles**

1. **Agnostic Design**: Works with any primal without system-specific code
2. **Auto-Discovery**: Automatically finds and integrates with available primals
3. **Capability-Based**: Negotiates capabilities rather than fixed interfaces
4. **Future-Proof**: New primals work without NestGate changes
5. **Composable**: Multiple primals can be combined for complex workflows

### **Core Components**

```
┌─────────────────────────────────────────────────────────────────┐
│                    🏠 NestGate Storage Primal                    │
├─────────────────────────────────────────────────────────────────┤
│  Universal Primal Interface │ Discovery Engine │ Config Manager │
│     Capability Negotiation  │ Health Monitor   │ Metrics System │
└─────────────────────────────────────────────────────────────────┘
                              │
                    Universal Communication
                              │
┌─────────────────────────────────────────────────────────────────┐
│                  🌐 Universal Primal Ecosystem                   │
├─────────────────────────────────────────────────────────────────┤
│ 🐕 BearDog │ 🐿️ Squirrel │ 🎵 Songbird │ 🍄 Toadstool │ 🔮 Future │
│ Security   │ AI & MCP    │ Distribution │ Compute     │ Primals  │
└─────────────────────────────────────────────────────────────────┘
```

## 🚀 **Quick Start**

### **1. Basic Setup**

```bash
# Clone NestGate
git clone https://github.com/your-org/nestgate.git
cd nestgate

# Build with universal primal support
cargo build --release --features universal-primal

# Run with universal primal configuration
cargo run --bin nestgate -- --config examples/universal-primal-config.toml
```

### **2. Configuration**

Create a `universal-primal-config.toml`:

```toml
[primal_ecosystem]
enabled = true
primal_id = "nestgate"
advertised_capabilities = [
    "zfs_storage",
    "high_performance", 
    "data_protection",
    "ai_data_services"
]

[discovery]
enabled = true
methods = ["environment", "network_scan", "config"]

[primal_integrations.beardog]
enabled = true
requested_capabilities = ["encryption", "access_control"]

[primal_integrations.squirrel]
enabled = true
requested_capabilities = ["model_inference", "agent_framework"]

[primal_integrations.songbird]
enabled = true
requested_capabilities = ["service_discovery", "load_balancing"]
```

### **3. Basic Usage**

```rust
use nestgate_api::universal_primal::*;
use nestgate_api::universal_primal_config::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = UniversalNestGateConfig::load_from_file("config.toml").await?;
    
    // Initialize NestGate primal
    let nestgate = NestGateStoragePrimal::new(config.into()).await?;
    
    // Start primal services
    nestgate.start_primal_services().await?;
    
    // Register with ecosystem
    nestgate.register_with_ecosystem().await?;
    
    // Auto-discover other primals
    let discovered = nestgate.discover_peer_primals().await?;
    println!("Found {} primals", discovered.len());
    
    Ok(())
}
```

## 🔧 **Integration Examples**

### **🐕 BearDog Security Integration**

```rust
// Request secure storage from BearDog
let security_request = StoragePrimalRequest {
    request_id: Uuid::new_v4(),
    from_primal: "nestgate".to_string(),
    to_primal: "beardog".to_string(),
    request_type: StorageRequestType::SecureStorageRequest {
        encryption_requirements: EncryptionRequirements {
            algorithm: "AES-256-GCM".to_string(),
            key_length: 256,
            at_rest: true,
            in_transit: true,
        },
        access_control: AccessControlRequirements {
            rbac: true,
            multi_tenant: true,
            audit_logging: true,
        },
    },
    payload: serde_json::json!({
        "storage_path": "/secure/data",
        "compliance_level": "enterprise"
    }),
    timestamp: SystemTime::now(),
};

let response = nestgate.handle_primal_request(security_request).await?;
```

### **🐿️ Squirrel AI Integration**

```rust
// Request AI data processing from Squirrel
let ai_request = StoragePrimalRequest {
    request_id: Uuid::new_v4(),
    from_primal: "nestgate".to_string(),
    to_primal: "squirrel".to_string(),
    request_type: StorageRequestType::AiDataRequest {
        data_type: AiDataType::VectorStore,
        performance_requirements: PerformanceRequirements {
            min_iops: 10_000,
            min_throughput_mbps: 1_000,
            max_latency_ms: 100,
        },
    },
    payload: serde_json::json!({
        "vector_dimensions": 1536,
        "expected_vectors": 1_000_000,
        "ai_models": ["gpt-4", "claude-3"]
    }),
    timestamp: SystemTime::now(),
};

let response = nestgate.handle_primal_request(ai_request).await?;
```

### **🎵 Songbird Distribution Integration**

```rust
// Request distributed storage from Songbird
let network_request = StoragePrimalRequest {
    request_id: Uuid::new_v4(),
    from_primal: "nestgate".to_string(),
    to_primal: "songbird".to_string(),
    request_type: StorageRequestType::NetworkStorageRequest {
        distribution_requirements: DistributionRequirements {
            geo_distribution: true,
            edge_caching: true,
            cdn_integration: false,
        },
        replication_config: ReplicationConfig {
            replicas: 3,
            consistency_level: ConsistencyLevel::Strong,
            cross_region: true,
        },
    },
    payload: serde_json::json!({
        "regions": ["us-east-1", "us-west-2", "eu-west-1"],
        "byob_manifest": "nestgate-storage-service.yaml"
    }),
    timestamp: SystemTime::now(),
};

let response = nestgate.handle_primal_request(network_request).await?;
```

## 🌟 **Advanced Features**

### **Multi-Primal Coordination**

```rust
// Coordinate multiple primals for complex workflows
async fn secure_ai_workflow(nestgate: &NestGateStoragePrimal) -> Result<()> {
    // 1. Request security from BearDog
    let security_response = request_beardog_security(nestgate).await?;
    
    // 2. Request AI processing from Squirrel
    let ai_response = request_squirrel_ai(nestgate).await?;
    
    // 3. Request distribution from Songbird
    let distribution_response = request_songbird_distribution(nestgate).await?;
    
    // 4. Coordinate all responses
    coordinate_responses(security_response, ai_response, distribution_response).await?;
    
    Ok(())
}
```

### **Auto-Discovery and Health Monitoring**

```rust
// Automatic primal discovery and health monitoring
async fn monitor_primal_ecosystem(nestgate: &NestGateStoragePrimal) -> Result<()> {
    loop {
        // Discover available primals
        let discovered = nestgate.discover_peer_primals().await?;
        
        // Check health of each primal
        for primal in discovered {
            let health_request = create_health_check_request(&primal);
            let health_response = nestgate.handle_primal_request(health_request).await?;
            
            match health_response.status {
                StorageResponseStatus::Success => {
                    info!("✅ {} is healthy", primal.primal_id);
                }
                _ => {
                    warn!("⚠️  {} has issues", primal.primal_id);
                }
            }
        }
        
        // Wait before next check
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
```

## 📊 **Configuration Reference**

### **Core Configuration**

```toml
[nestgate]
# Core NestGate settings
[nestgate.server]
host = "0.0.0.0"
port = 8080
workers = 4

[nestgate.storage]
zfs_enabled = true
compression_enabled = true
encryption_enabled = true

[nestgate.security]
enable_tls = true
require_authentication = true
audit_logging = true
```

### **Primal Ecosystem Configuration**

```toml
[primal_ecosystem]
enabled = true
primal_id = "nestgate"
advertised_capabilities = [
    "zfs_storage",
    "high_performance",
    "data_protection",
    "ai_data_services",
    "security_integration",
    "network_storage"
]

[primal_ecosystem.health_check]
enabled = true
interval_seconds = 30
timeout_ms = 5000
failure_threshold = 3
```

### **Discovery Configuration**

```toml
[discovery]
enabled = true
methods = ["environment", "network_scan", "config"]
discovery_interval_seconds = 60

[discovery.network_discovery]
interfaces = ["eth0", "wlan0"]
protocols = ["mdns", "upnp"]
scan_timeout_ms = 5000

[[discovery.network_discovery.port_ranges]]
start = 8080
end = 8090
```

### **Primal Integration Configuration**

```toml
[primal_integrations.beardog]
enabled = true
requested_capabilities = ["encryption", "access_control"]

[primal_integrations.beardog.settings]
connection_timeout_ms = 5000
tls_enabled = true
mutual_tls = true

[primal_integrations.squirrel]
enabled = true
requested_capabilities = ["model_inference", "agent_framework"]

[primal_integrations.squirrel.settings]
request_timeout_ms = 30000  # AI operations take longer
compression_enabled = true

[primal_integrations.songbird]
enabled = true
requested_capabilities = ["service_discovery", "load_balancing"]

[primal_integrations.songbird.settings]
connection_pool_size = 20
retry_attempts = 5
```

## 🛠️ **Development Guide**

### **Adding New Primal Support**

1. **Define Capabilities**: Add new capability types to `StorageCapability` enum
2. **Add Request Types**: Extend `StorageRequestType` for new primal
3. **Update Configuration**: Add integration config for new primal
4. **Implement Handlers**: Create request handlers in `NestGateStoragePrimal`

```rust
// Example: Adding new primal support
pub enum StorageCapability {
    // ... existing capabilities
    
    // New primal capabilities
    NewPrimalCapability {
        feature_x: bool,
        feature_y: Vec<String>,
    },
}

pub enum StorageRequestType {
    // ... existing types
    
    // New primal request type
    NewPrimalRequest {
        requirements: NewPrimalRequirements,
    },
}
```

### **Testing Integration**

```bash
# Run integration tests
cargo test --package nestgate-api --test universal_primal_integration

# Run specific primal tests
cargo test --package nestgate-api beardog_integration
cargo test --package nestgate-api squirrel_integration

# Run demo
cargo run --example universal_primal_integration_demo
```

## 🔍 **Troubleshooting**

### **Common Issues**

1. **Discovery Failures**
   ```bash
   # Check network connectivity
   ping beardog.local
   ping squirrel.local
   ping songbird.local
   
   # Check DNS resolution
   nslookup beardog.local
   ```

2. **Authentication Issues**
   ```bash
   # Check certificates
   openssl x509 -in /etc/nestgate/ssl/beardog-client.crt -text -noout
   
   # Test TLS connection
   openssl s_client -connect beardog.local:8443
   ```

3. **Configuration Validation**
   ```bash
   # Validate configuration
   cargo run --bin nestgate -- --config config.toml --validate
   
   # Check configuration syntax
   toml-validator config.toml
   ```

### **Debug Logging**

```bash
# Enable debug logging
RUST_LOG=debug cargo run --bin nestgate

# Enable primal-specific logging
RUST_LOG=nestgate_api::universal_primal=debug cargo run
```

## 📈 **Performance Optimization**

### **Connection Pooling**

```toml
[primal_integrations.beardog.settings]
connection_pool_size = 10
keep_alive_enabled = true

[primal_integrations.squirrel.settings]
connection_pool_size = 5
request_timeout_ms = 30000
```

### **Caching**

```toml
[nestgate.performance]
enable_caching = true
cache_size_mb = 512
```

### **Monitoring**

```toml
[primal_ecosystem.metrics]
enabled = true
collection_interval_seconds = 15
export_endpoint = "/metrics"
```

## 🔒 **Security Considerations**

### **TLS Configuration**

```toml
[networking.tls]
enabled = true
cert_file = "/etc/nestgate/ssl/server.crt"
key_file = "/etc/nestgate/ssl/server.key"
verify_client = true
```

### **Authentication**

```toml
[primal_integrations.beardog.auth]
auth_type = "mutual_tls"
cert_file = "/etc/nestgate/ssl/beardog-client.crt"
key_file = "/etc/nestgate/ssl/beardog-client.key"
```

### **Access Control**

```toml
[nestgate.security]
require_authentication = true
audit_logging = true
rate_limiting = true
```

## 🎯 **Best Practices**

1. **Configuration Management**
   - Use environment variables for sensitive data
   - Validate configuration before deployment
   - Version control configuration files

2. **Monitoring and Alerting**
   - Enable comprehensive health checks
   - Set up alerts for primal failures
   - Monitor performance metrics

3. **Security**
   - Use mutual TLS for primal communication
   - Regularly rotate certificates
   - Enable audit logging

4. **Performance**
   - Tune connection pools based on workload
   - Enable compression for network efficiency
   - Monitor and optimize cache usage

5. **Reliability**
   - Configure appropriate retry policies
   - Implement circuit breakers
   - Plan for graceful degradation

## 📚 **Additional Resources**

- [NestGate API Reference](docs/api-reference.md)
- [Universal Primal Specification](specs/universal-primal-spec.md)
- [BearDog Integration Guide](docs/beardog-integration.md)
- [Squirrel Integration Guide](docs/squirrel-integration.md)
- [Songbird Integration Guide](docs/songbird-integration.md)
- [Performance Tuning Guide](docs/performance-tuning.md)

## 🤝 **Contributing**

We welcome contributions to improve the universal primal integration:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Update documentation
5. Submit a pull request

## 📄 **License**

This project is licensed under AGPL-3.0-or-later. See [LICENSE](LICENSE) for details.

---

**🌐 NestGate: Universal storage for the universal primal ecosystem** 