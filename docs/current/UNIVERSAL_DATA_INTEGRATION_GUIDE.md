# Universal Data Integration Guide

**NestGate's Universal Data Capabilities System**

## 🌟 Overview

NestGate implements a truly agnostic data integration system where **NestGate only knows itself**. External systems can provide genome data, model data, research data, or any other capability without NestGate being coupled to specific providers like NCBI, HuggingFace, or Ensembl.

## 🏗️ Architecture Principles

### 1. **Capability-Based Integration**
- NestGate defines **what** it needs (capabilities)
- External providers implement **how** they provide it
- No hardcoded provider names or endpoints

### 2. **Universal Interfaces**
- `DataCapability` - Base trait for any data provider
- `GenomeDataCapability` - For genome databases
- `ModelDataCapability` - For AI model repositories  
- `ResearchDataCapability` - For research databases

### 3. **Provider Agnosticism**
- Any system can implement these capabilities
- Automatic failover between providers
- Dynamic provider discovery

## 🚀 Quick Start

### Basic Usage

```rust
use nestgate_core::data_sources::{
    UniversalDataAdapter, DataRequest,
    providers::live_providers::NCBILiveProvider
};

// Create adapter
let mut adapter = UniversalDataAdapter::new();

// Register any genome provider (NCBI, Ensembl, custom database, etc.)
let ncbi_provider = NCBILiveProvider::new(api_key, email)?;
adapter.register_provider(Arc::new(ncbi_provider));

// Request genome data without knowing which provider responds
let request = DataRequest {
    capability_type: "genome_data".to_string(),
    parameters: json!({
        "query": "human insulin",
        "organism": "homo sapiens"
    }),
    metadata: HashMap::new(),
};

let response = adapter.execute_request(&request).await?;
// NestGate gets genome data without being coupled to NCBI
```

### With Fallback Providers

```rust
// Primary provider
let ncbi_provider = NCBILiveProvider::new(Some(api_key), Some(email))?;
adapter.register_provider(Arc::new(ncbi_provider));

// Fallback provider (different database, same capability)
let ensembl_provider = EnsemblLiveProvider::new()?;
adapter.register_fallback_provider(
    "genome_data".to_string(), 
    Arc::new(ensembl_provider)
);

// Request automatically fails over if primary provider is unavailable
let response = adapter.execute_request(&request).await?;
```

## 📊 Data Capabilities

### Genome Data Capability

For any genome database (NCBI, Ensembl, custom):

```rust
use nestgate_core::data_sources::data_capabilities::*;

#[async_trait]
impl GenomeDataCapability for YourGenomeProvider {
    async fn search_genomes(&self, query: &str) -> Result<Vec<GenomeResult>> {
        // Your implementation here
    }
    
    async fn get_genome_sequence(&self, genome_id: &str) -> Result<GenomeSequence> {
        // Your implementation here
    }
}
```

### Model Data Capability

For any AI model repository (HuggingFace, ModelHub, custom):

```rust
#[async_trait]
impl ModelDataCapability for YourModelProvider {
    async fn search_models(&self, query: &str) -> Result<Vec<ModelResult>> {
        // Your implementation here
    }
    
    async fn get_model_info(&self, model_id: &str) -> Result<ModelInfo> {
        // Your implementation here
    }
}
```

### Research Data Capability

For any research database (PubMed, arXiv, custom):

```rust
#[async_trait]
impl ResearchDataCapability for YourResearchProvider {
    async fn search_research(&self, query: &str) -> Result<Vec<ResearchResult>> {
        // Your implementation here
    }
    
    async fn get_research_data(&self, research_id: &str) -> Result<ResearchData> {
        // Your implementation here
    }
}
```

## 🌐 HTTP Provider Integration

### Universal HTTP Provider

Adapt any REST API to NestGate capabilities:

```rust
use nestgate_core::data_sources::providers::universal_http_provider::*;

// Adapt any API
let config = HttpProviderConfigBuilder::new(
    "https://api.your-database.com".to_string(),
    "genome_data".to_string()
)
.with_api_key("your-api-key".to_string())
.with_timeout(60)
.with_metadata("provider_name".to_string(), "Your Database".to_string())
.build();

let provider = UniversalHttpProvider::new(config)?;
adapter.register_provider(Arc::new(provider));
```

### Live Provider Examples

#### NCBI Integration

```rust
use nestgate_core::data_sources::providers::live_providers::*;

// From environment variables
let ncbi_provider = NCBIProviderFactory::create_from_env()?;

// With explicit credentials
let ncbi_provider = NCBIProviderFactory::create_with_api_key(
    "your-ncbi-api-key".to_string(),
    "your-email@domain.com".to_string()
)?;

adapter.register_provider(ncbi_provider);
```

#### Custom Database Integration

```rust
// Any genome database can integrate
let custom_provider = UniversalGenomeProvider::for_custom_database(
    "https://your-genome-db.com/api".to_string(),
    "Your Genome Database".to_string(),
    Some("api-key".to_string())
)?;

adapter.register_provider(Arc::new(custom_provider));
```

## 🏪 Storage Backends

### Universal Storage Interface

```rust
use nestgate_core::universal_storage::*;

// Filesystem storage
let fs_backend = FilesystemBackend::new("/data".to_string())?;

// S3-compatible object storage
let s3_backend = ObjectStorageBackend::for_aws_s3(
    "us-east-1".to_string(),
    access_key,
    secret_key,
    "my-bucket".to_string()
)?;

// MinIO object storage
let minio_backend = ObjectStorageBackend::for_minio(
    "http://localhost:9000".to_string(),
    access_key,
    secret_key,
    "my-bucket".to_string()
)?;
```

### Streaming Large Files

```rust
// Stream large genome files efficiently
let stream = backend.stream_data(StreamRequest {
    path: "large_genome.fasta".to_string(),
    chunk_size: Some(8192), // 8KB chunks
}).await?;

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    // Process chunk
    println!("Chunk {}: {} bytes", chunk.sequence, chunk.data.len());
}
```

## ⚡ Performance Optimization

### Connection Pooling

```rust
use nestgate_core::performance::*;

// Create HTTP connection pool
let pool_config = ConnectionPoolConfig {
    max_connections: 20,
    min_connections: 5,
    max_idle_time: Duration::from_secs(300),
    ..Default::default()
};

let http_pool = HttpConnectionPool::new_http_pool(pool_config);

// Use pooled connections
let connection = http_pool.get_connection().await?;
let response = connection.connection().get("https://api.example.com").send().await?;
```

### Request Batching

```rust
// Batch multiple requests for efficiency
let requests = vec![
    DataRequest { capability_type: "genome_data".to_string(), /* ... */ },
    DataRequest { capability_type: "genome_data".to_string(), /* ... */ },
    DataRequest { capability_type: "genome_data".to_string(), /* ... */ },
];

let responses = futures::future::join_all(
    requests.iter().map(|req| adapter.execute_request(req))
).await;
```

## 🔧 Configuration

### Environment Variables

```bash
# NCBI Configuration
export NCBI_API_KEY="your-ncbi-api-key"
export NCBI_EMAIL="your-email@domain.com"

# S3 Configuration  
export S3_ACCESS_KEY="your-access-key"
export S3_SECRET_KEY="your-secret-key"
export S3_BUCKET="your-bucket"
export S3_REGION="us-east-1"

# MinIO Configuration
export S3_ENDPOINT="http://localhost:9000"
export S3_PATH_STYLE="true"
```

### Programmatic Configuration

```rust
use nestgate_core::config::canonical::*;

let config = CanonicalConfigBuilder::new()
    .with_network_timeout(60)
    .with_max_connections(100)
    .with_cache_size(1024 * 1024 * 1024) // 1GB
    .build();

// Initialize global configuration
nestgate_core::constants::limits::init_config(config);
```

## 🧪 Testing

### Integration Tests

```rust
#[tokio::test]
async fn test_provider_agnosticism() {
    let mut adapter = UniversalDataAdapter::new();
    
    // Register multiple providers with same capability
    adapter.register_provider(create_provider_a());
    adapter.register_fallback_provider("genome_data".to_string(), create_provider_b());
    
    // NestGate doesn't know or care which provider responds
    let response = adapter.execute_request(&genome_request).await?;
    assert!(response.source_info.is_some());
}
```

### Mock Providers

```rust
// Create mock providers for testing
let mock_provider = MockGenomeProvider::new(mock_config);
adapter.register_provider(Arc::new(mock_provider));

// Test without hitting real APIs
let response = adapter.execute_request(&test_request).await?;
```

## 📈 Monitoring & Observability

### Provider Health Checks

```rust
// Check provider health
let health = provider.health_check().await?;
match health {
    HealthStatus::Healthy => println!("✅ Provider is healthy"),
    HealthStatus::Unhealthy { reason } => println!("❌ Provider unhealthy: {}", reason),
}
```

### Connection Pool Statistics

```rust
let stats = connection_pool.get_stats().await;
println!("Pool stats: {} active, {} idle, {} total", 
    stats.active_connections, 
    stats.idle_connections, 
    stats.total_connections
);
```

### Request Metrics

```rust
let metadata = adapter.get_provider_metadata("genome_data");
if let Some(metadata) = metadata {
    println!("Provider: {}", metadata.get("provider_name").unwrap_or(&"Unknown".to_string()));
    println!("Rate limit: {}", metadata.get("rate_limit").unwrap_or(&"None".to_string()));
}
```

## 🔐 Security & Attribution

### API Key Management

```rust
// Secure API key handling
let api_key = std::env::var("NCBI_API_KEY")
    .map_err(|_| "NCBI_API_KEY environment variable not set")?;

let provider = NCBILiveProvider::new(Some(api_key), Some(email))?;
```

### Data Attribution

```rust
let response = adapter.execute_request(&request).await?;

if let Some(source_info) = response.source_info {
    println!("Data provided by: {}", 
        source_info.provider_name.unwrap_or("Unknown".to_string()));
    println!("License: {}", 
        source_info.license.unwrap_or("Check provider terms".to_string()));
}
```

## 🌍 Ecosystem Integration

### Adding New Providers

1. **Implement the capability trait**:
```rust
struct YourDatabaseProvider;

#[async_trait]
impl DataCapability for YourDatabaseProvider {
    fn capability_type(&self) -> &str { "genome_data" }
    // ... implement other methods
}
```

2. **Register with the adapter**:
```rust
let provider = YourDatabaseProvider::new(config)?;
adapter.register_provider(Arc::new(provider));
```

3. **Use immediately**:
```rust
// NestGate can now use your database without knowing its identity
let response = adapter.execute_request(&request).await?;
```

### Provider Discovery

```rust
// Discover available capabilities
let capabilities = adapter.get_available_capabilities();
println!("Available capabilities: {:?}", capabilities);

// Get provider metadata
for capability in capabilities {
    if let Some(metadata) = adapter.get_provider_metadata(&capability) {
        println!("{}: {}", capability, metadata.get("provider_name").unwrap_or(&"Unknown".to_string()));
    }
}
```

## 🎯 Best Practices

### 1. **Always Use Fallback Providers**
```rust
// Primary provider
adapter.register_provider(primary_provider);
// Always have a backup
adapter.register_fallback_provider(capability_type, fallback_provider);
```

### 2. **Handle Rate Limits**
```rust
// Check provider metadata for rate limits
let metadata = provider.get_metadata();
if let Some(rate_limit) = metadata.get("rate_limit") {
    // Implement appropriate throttling
}
```

### 3. **Use Connection Pooling**
```rust
// Pool connections for better performance
let pool = HttpConnectionPool::new_http_pool(pool_config);
// Reuse connections across requests
```

### 4. **Implement Proper Error Handling**
```rust
match adapter.execute_request(&request).await {
    Ok(response) => {
        // Process successful response
    },
    Err(NestGateError::NotFound { .. }) => {
        // Handle data not found
    },
    Err(NestGateError::RateLimit { .. }) => {
        // Implement backoff strategy
    },
    Err(e) => {
        // Handle other errors
    }
}
```

## 🔮 Future Extensions

The universal architecture enables easy integration of:

- **New Data Types**: Add `ImageDataCapability`, `TimeSeriesDataCapability`, etc.
- **New Providers**: Any system can implement existing capabilities
- **New Protocols**: GraphQL, gRPC, WebSocket providers
- **Advanced Features**: Caching, compression, encryption

## 📚 API Reference

### Core Types

- `DataCapability` - Base trait for data providers
- `DataRequest` - Universal request format
- `DataResponse` - Universal response format
- `UniversalDataAdapter` - Main routing system

### Provider Types

- `GenomeDataCapability` - For genome databases
- `ModelDataCapability` - For AI model repositories
- `ResearchDataCapability` - For research databases
- `UniversalHttpProvider` - For REST API adaptation

### Storage Types

- `UniversalStorageBackend` - Base storage interface
- `FilesystemBackend` - Local filesystem storage
- `ObjectStorageBackend` - S3-compatible storage

### Performance Types

- `UniversalConnectionPool` - Generic connection pooling
- `ConnectionPoolManager` - Multi-provider pool management
- `PoolStats` - Connection pool statistics

---

**The Universal Data Integration System enables true ecosystem agnosticism - NestGate knows only capabilities, never provider identities. Any external system can integrate seamlessly.** 🌟 