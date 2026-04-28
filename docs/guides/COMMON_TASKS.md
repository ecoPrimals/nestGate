# Common Tasks Cookbook

**Quick recipes for everyday NestGate operations**

---

## Storage Operations

### **Create and Use a Dataset**

```bash
# Create dataset
curl -X POST http://localhost:8080/api/datasets \
  -H "Content-Type: application/json" \
  -d '{"name":"my-data","description":"My dataset"}'

# Store files
for file in *.txt; do
  curl -X PUT "http://localhost:8080/api/datasets/my-data/objects/$file" \
    --data-binary "@$file"
done

# List objects
curl http://localhost:8080/api/datasets/my-data/objects
```

### **Backup and Restore**

```bash
# Backup dataset (download all objects)
mkdir -p backup/my-data
for obj in $(curl -s http://localhost:8080/api/datasets/my-data/objects | jq -r '.objects[].key'); do
  curl "http://localhost:8080/api/datasets/my-data/objects/$obj" > "backup/my-data/$obj"
done

# Restore dataset (upload all objects)
curl -X POST http://localhost:8080/api/datasets \
  -H "Content-Type: application/json" \
  -d '{"name":"my-data"}'

for file in backup/my-data/*; do
  key=$(basename "$file")
  curl -X PUT "http://localhost:8080/api/datasets/my-data/objects/$key" \
    --data-binary "@$file"
done
```

### **Verify Data Integrity**

```bash
# Get checksum from NestGate
REMOTE_CHECKSUM=$(curl -sI http://localhost:8080/api/datasets/data/objects/file.bin | \
  grep -i x-checksum-sha256 | cut -d' ' -f2 | tr -d '\r')

# Calculate local checksum
LOCAL_CHECKSUM=$(sha256sum file.bin | cut -d' ' -f1)

# Compare
if [ "$REMOTE_CHECKSUM" == "$LOCAL_CHECKSUM" ]; then
  echo "OK: Checksums match - data is intact"
else
  echo "FAIL: Checksums differ - data corrupted!"
fi
```

---

## Discovery Operations

### **Find a Service by Capability**

```bash
# Find security provider
curl http://localhost:8080/api/services/discover/security | jq

# Find orchestrator
curl http://localhost:8080/api/services/discover/orchestration | jq

# Find all services
curl http://localhost:8080/api/services | jq '.services[]'
```

### **Register Custom Service**

```rust
use nestgate_core::primal_discovery::RuntimeDiscovery;
use nestgate_core::capabilities::Capability;

#[tokio::main]
async fn main() -> Result<()> {
    let discovery = RuntimeDiscovery::new().await?;
    
    // Announce yourself
    let self_knowledge = SelfKnowledge::builder()
        .with_name("my-service")
        .with_capability(Capability::Custom("processing".into()))
        .build()?;
    
    discovery.announce(&self_knowledge).await?;
    
    Ok(())
}
```

---

## Configuration Tasks

### **Environment-Based Configuration**

```bash
# Create production config
cat > production.env << 'EOF'
# Network
NESTGATE_PORT=8080
NESTGATE_HOST=0.0.0.0  # Listen on all interfaces

# Storage
NESTGATE_DATA_DIR=/var/lib/nestgate
NESTGATE_ZFS_POOL=tank

# Discovery
NESTGATE_DISCOVERY_ENABLED=true
NESTGATE_DISCOVERY_INTERVAL=30

# Security
NESTGATE_TLS_ENABLED=true
NESTGATE_TLS_CERT=/etc/nestgate/cert.pem
NESTGATE_TLS_KEY=/etc/nestgate/key.pem

# Monitoring
NESTGATE_METRICS_PORT=9090
RUST_LOG=info,nestgate=debug
EOF

# Load and run
source production.env
./nestgate
```

### **Multi-Instance Setup**

```bash
# Instance 1 (Leader)
export NESTGATE_PORT=8081
export NESTGATE_SOCKET_PATH=/tmp/nestgate-1.sock
./nestgate &

# Instance 2 (Follower)
export NESTGATE_PORT=8082
export NESTGATE_SOCKET_PATH=/tmp/nestgate-2.sock
./nestgate &

# Instance 3 (Follower)
export NESTGATE_PORT=8083
export NESTGATE_SOCKET_PATH=/tmp/nestgate-3.sock
./nestgate &

# Load balancer config (nginx)
upstream nestgate_cluster {
    server localhost:8081;
    server localhost:8082;
    server localhost:8083;
}
```

---

## Docker Tasks

### **Run in Docker**

```bash
# Build image
docker build -t nestgate:latest .

# Run container
docker run -d \
  --name nestgate \
  -p 8080:8080 \
  -v nestgate-data:/var/lib/nestgate \
  -e NESTGATE_ZFS_ENABLED=false \
  nestgate:latest

# Check logs
docker logs -f nestgate

# Access API
curl http://localhost:8080/health
```

### **Docker Compose**

```yaml
version: '3.8'
services:
  nestgate:
    image: nestgate:latest
    ports:
      - "8080:8080"
      - "9090:9090"  # metrics
    environment:
      - NESTGATE_PORT=8080
      - NESTGATE_ZFS_ENABLED=false
      - RUST_LOG=info
    volumes:
      - nestgate-data:/var/lib/nestgate
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

volumes:
  nestgate-data:
```

---

## Security Tasks

### **Enable TLS**

```bash
# Generate self-signed certificate
openssl req -x509 -newkey rsa:4096 -nodes \
  -keyout key.pem -out cert.pem -days 365 \
  -subj "/CN=nestgate.local"

# Configure NestGate
export NESTGATE_TLS_ENABLED=true
export NESTGATE_TLS_CERT=cert.pem
export NESTGATE_TLS_KEY=key.pem

# Run with TLS
./nestgate

# Access via HTTPS
curl --cacert cert.pem https://localhost:8080/health
```

### **Enable API Key Authentication**

```bash
# Generate API key
export NESTGATE_API_KEY=$(openssl rand -hex 32)

# Run NestGate
./nestgate

# Use API key in requests
curl -H "X-API-Key: $NESTGATE_API_KEY" \
  http://localhost:8080/api/datasets
```

---

## Monitoring Tasks

### **Prometheus Integration**

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'nestgate'
    static_configs:
      - targets: ['localhost:9090']
```

```bash
# Start Prometheus
prometheus --config.file=prometheus.yml

# View metrics
curl http://localhost:9090/metrics
```

### **Health Monitoring Script**

```bash
#!/bin/bash
# health-monitor.sh

while true; do
  STATUS=$(curl -s http://localhost:8080/health | jq -r '.status')
  
  if [ "$STATUS" == "healthy" ]; then
    echo "OK: NestGate is healthy"
  else
    echo "FAIL: NestGate is unhealthy: $STATUS"
    # Send alert
    notify-send "NestGate Alert" "Service is $STATUS"
  fi
  
  sleep 60
done
```

---

## Development Tasks

### **Local Development Setup**

```bash
# Install development dependencies
cargo install cargo-watch cargo-nextest

# Run with auto-reload
cargo watch -x 'run --package nestgate-bin'

# Run tests on change
cargo watch -x 'nextest run'

# Check code quality
cargo clippy --all-targets
cargo fmt --check
```

### **Add a New Dataset Type**

```rust
// In your code
use nestgate_core::rpc::tarpc_types::{DatasetParams, DatasetInfo};

async fn create_custom_dataset(name: &str) -> Result<DatasetInfo> {
    let params = DatasetParams {
        description: Some("Custom dataset".into()),
        compression_enabled: true,
        encryption_enabled: true,
        retention_days: Some(365),
        ..Default::default()
    };
    
    let dataset = storage_service
        .create_dataset(name, params)
        .await?;
    
    Ok(dataset)
}
```

---

## Troubleshooting Tasks

### **Debug Connection Issues**

```bash
# Check if NestGate is running
curl http://localhost:8080/health

# Check Unix socket
ls -l /run/user/$(id -u)/nestgate/

# Test socket connection
echo '{"jsonrpc":"2.0","method":"health","id":1}' | \
  socat - UNIX-CONNECT:/run/user/$(id -u)/nestgate/nestgate.sock

# Check logs
journalctl -u nestgate -f

# Or file logs
tail -f ~/.local/share/nestgate/logs/nestgate.log
```

### **Clear Cache and Reset**

```bash
# Stop NestGate
pkill nestgate

# Clear cache (preserves data)
rm -rf ~/.cache/nestgate/*

# Or full reset (WARNING: deletes data!)
rm -rf ~/.local/share/nestgate/*
rm -rf ~/.cache/nestgate/*

# Restart
./nestgate
```

---

## More Resources

- **API Reference**: `docs/api/REST_API.md`
- **Architecture**: `docs/architecture/COMPONENT_INTERACTIONS.md`
- **Environment Variables**: `docs/guides/ENVIRONMENT_VARIABLES.md`
- **Troubleshooting**: `docs/guides/TROUBLESHOOTING.md`

---

**NestGate Cookbook** · Practical Examples · Production-Ready
