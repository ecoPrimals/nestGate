# NestGate Deployment Guide - Universal Architecture

This guide covers deploying NestGate's new **sovereignty-compliant universal architecture** in production environments.

## 🌟 **Universal Architecture Overview**

NestGate now implements a **Universal Capability Discovery** system that:
- Respects service sovereignty through dynamic discovery
- Eliminates hardcoded primal dependencies  
- Enables cross-ecosystem interoperability
- Maintains production-grade performance

## 🚀 **Production Deployment**

### **Prerequisites**

- **Rust**: 1.70+ (latest stable recommended)
- **System**: Linux with ZFS support (optional)
- **Memory**: 4GB minimum, 8GB recommended
- **Storage**: 50GB for logs and cache
- **Network**: Outbound HTTPS access for service discovery

### **Quick Production Setup**

```bash
# 1. Clone and build optimized release
git clone <your-repository>
cd nestgate
cargo build --release --all-features

# 2. Configure universal architecture
export NESTGATE_DISCOVERY_MODE=universal
export NESTGATE_ENABLE_CAPABILITY_DISCOVERY=true
export NESTGATE_RESPECT_SERVICE_SOVEREIGNTY=true

# 3. Start production instance
./target/release/nestgate-bin
```

## ⚙️ **Configuration**

### **Universal Configuration File**

Create `config/production.toml`:

```toml
[universal_config]
# Core universal architecture settings
discovery_mode = "universal"
enable_capability_discovery = true
respect_service_sovereignty = true

[service_discovery]
# Dynamic service discovery vs hardcoded endpoints
timeout_seconds = 30
retry_attempts = 3
discovery_interval_seconds = 60

# Capability-based service requirements
orchestration_capabilities = ["workflow", "scheduling", "resource_management"]
security_capabilities = ["authentication", "authorization", "encryption"]
compute_capabilities = ["processing", "analytics", "optimization"]
storage_capabilities = ["persistence", "caching", "backup"]

[network]
# Universal network configuration
bind_address = "0.0.0.0"
port = 8080
enable_tls = true
max_connections = 1000

[storage]
# Universal storage configuration  
data_directory = "/var/lib/nestgate"
cache_size_mb = 512
enable_zfs_integration = true
backup_enabled = true

[logging]
level = "info"
format = "json"
output = "/var/log/nestgate/nestgate.log"
```

### **Environment Variables**

#### **Universal Architecture**
```bash
# Core universal settings
export NESTGATE_DISCOVERY_MODE=universal
export NESTGATE_ENABLE_CAPABILITY_DISCOVERY=true
export NESTGATE_RESPECT_SERVICE_SOVEREIGNTY=true

# Service discovery endpoints (discovered dynamically)
export NESTGATE_DISCOVERY_REGISTRY_URL=http://consul:8500
export NESTGATE_DISCOVERY_TIMEOUT=30s
export NESTGATE_DISCOVERY_RETRY_ATTEMPTS=3
```

#### **Production Settings**
```bash
# Performance optimization
export NESTGATE_WORKER_THREADS=4
export NESTGATE_MAX_MEMORY_MB=2048
export NESTGATE_CACHE_SIZE_MB=512

# Security
export NESTGATE_ENABLE_TLS=true
export NESTGATE_TLS_CERT_PATH=/etc/ssl/certs/nestgate.crt
export NESTGATE_TLS_KEY_PATH=/etc/ssl/private/nestgate.key

# Logging
export NESTGATE_LOG_LEVEL=info
export NESTGATE_LOG_FORMAT=json
export NESTGATE_LOG_PATH=/var/log/nestgate/nestgate.log
```

## 🐳 **Docker Deployment**

### **Universal Architecture Docker Image**

```dockerfile
FROM rust:1.70-alpine AS builder

WORKDIR /app
COPY . .
RUN cargo build --release --all-features

FROM alpine:3.18
RUN apk add --no-cache ca-certificates openssl

COPY --from=builder /app/target/release/nestgate-bin /usr/local/bin/
COPY --from=builder /app/config/production.toml /etc/nestgate/

# Universal architecture configuration
ENV NESTGATE_DISCOVERY_MODE=universal
ENV NESTGATE_ENABLE_CAPABILITY_DISCOVERY=true
ENV NESTGATE_RESPECT_SERVICE_SOVEREIGNTY=true

EXPOSE 8080
CMD ["nestgate-bin", "--config", "/etc/nestgate/production.toml"]
```

### **Docker Compose with Service Discovery**

```yaml
version: '3.8'

services:
  nestgate:
    build: .
    ports:
      - "8080:8080"
    environment:
      # Universal architecture
      - NESTGATE_DISCOVERY_MODE=universal
      - NESTGATE_ENABLE_CAPABILITY_DISCOVERY=true
      - NESTGATE_RESPECT_SERVICE_SOVEREIGNTY=true
      
      # Service discovery
      - NESTGATE_DISCOVERY_REGISTRY_URL=http://consul:8500
      
    volumes:
      - nestgate_data:/var/lib/nestgate
      - nestgate_logs:/var/log/nestgate
    depends_on:
      - consul
      
  # Service discovery registry
  consul:
    image: consul:1.16
    ports:
      - "8500:8500"
    command: consul agent -dev -client=0.0.0.0

volumes:
  nestgate_data:
  nestgate_logs:
```

## ☸️ **Kubernetes Deployment**

### **Universal Architecture Deployment**

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nestgate-universal
  labels:
    app: nestgate
    architecture: universal
spec:
  replicas: 3
  selector:
    matchLabels:
      app: nestgate
  template:
    metadata:
      labels:
        app: nestgate
        architecture: universal
    spec:
      containers:
      - name: nestgate
        image: nestgate:universal
        ports:
        - containerPort: 8080
        env:
        # Universal architecture configuration
        - name: NESTGATE_DISCOVERY_MODE
          value: "universal"
        - name: NESTGATE_ENABLE_CAPABILITY_DISCOVERY
          value: "true"
        - name: NESTGATE_RESPECT_SERVICE_SOVEREIGNTY
          value: "true"
          
        # Kubernetes service discovery
        - name: NESTGATE_DISCOVERY_REGISTRY_URL
          value: "http://kubernetes.default.svc.cluster.local"
          
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "1000m"
            
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
          
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5

---
apiVersion: v1
kind: Service
metadata:
  name: nestgate-service
spec:
  selector:
    app: nestgate
  ports:
  - port: 80
    targetPort: 8080
  type: LoadBalancer
```

## 🔍 **Monitoring & Observability**

### **Health Checks**

NestGate provides sovereignty-compliant health endpoints:

```bash
# Universal architecture health check
curl http://localhost:8080/health
# Response: {"status": "healthy", "architecture": "universal", "sovereignty_compliant": true}

# Service discovery status
curl http://localhost:8080/discovery/status
# Response: {"discovered_services": [...], "capabilities": [...]}

# Capability verification
curl http://localhost:8080/capabilities
# Response: {"available": [...], "required": [...], "satisfied": true}
```

### **Metrics & Logging**

```bash
# View universal architecture metrics
curl http://localhost:8080/metrics

# Check sovereignty compliance
curl http://localhost:8080/sovereignty/status

# Service discovery metrics
curl http://localhost:8080/discovery/metrics
```

## 🛡️ **Security**

### **Sovereignty-Compliant Security**

```toml
[security]
# Capability-based authentication
enable_capability_auth = true
require_service_verification = true
respect_service_boundaries = true

# TLS configuration
enable_tls = true
tls_cert_path = "/etc/ssl/certs/nestgate.crt"
tls_key_path = "/etc/ssl/private/nestgate.key"
min_tls_version = "1.3"

# Service discovery security
verify_discovered_services = true
require_service_signatures = true
```

## 🔧 **Troubleshooting**

### **Common Universal Architecture Issues**

1. **Service Discovery Failures**
   ```bash
   # Check discovery configuration
   curl http://localhost:8080/discovery/debug
   
   # Verify registry connectivity
   curl $NESTGATE_DISCOVERY_REGISTRY_URL/v1/health
   ```

2. **Sovereignty Compliance Issues**
   ```bash
   # Check sovereignty status
   curl http://localhost:8080/sovereignty/violations
   
   # Verify capability compliance
   curl http://localhost:8080/capabilities/check
   ```

3. **Performance Issues**
   ```bash
   # Check universal architecture performance
   curl http://localhost:8080/performance/universal
   
   # Monitor capability discovery latency
   curl http://localhost:8080/metrics/discovery_latency
   ```

## 🚀 **Production Checklist**

### **Pre-Deployment**
- [ ] Universal architecture configuration verified
- [ ] Service discovery registry accessible
- [ ] Capability requirements defined
- [ ] Security certificates configured
- [ ] Resource limits set appropriately

### **Post-Deployment**
- [ ] Health checks passing
- [ ] Service discovery working
- [ ] Sovereignty compliance verified
- [ ] Metrics collection active
- [ ] Log aggregation configured

## 📊 **Performance Tuning**

### **Universal Architecture Optimizations**

```toml
[performance]
# Capability discovery optimization
discovery_cache_ttl_seconds = 300
capability_resolution_timeout_seconds = 10
max_concurrent_discoveries = 10

# Universal adapter performance
adapter_pool_size = 50
adapter_timeout_seconds = 30
enable_adapter_caching = true

# Service sovereignty respect
respect_service_rate_limits = true
honor_service_timeouts = true
use_service_preferred_protocols = true
```

---

## 🌟 **Summary**

NestGate's **Universal Architecture** provides:

✅ **Sovereignty-Compliant**: Respects service autonomy  
✅ **Universal Compatibility**: Works across ecosystems  
✅ **Production-Ready**: Enterprise deployment capabilities  
✅ **Performance-Optimized**: Maintains high throughput  
✅ **Secure**: Capability-based authentication  

The system is now ready for production deployment with full sovereignty compliance and universal cross-ecosystem interoperability! 🎊 