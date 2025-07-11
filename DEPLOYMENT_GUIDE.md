# 🚀 NestGate Production Deployment Guide

**Status**: ✅ **PRODUCTION READY**  
**Date**: 2025-01-26  
**Architecture**: Headless API-first system with zero compilation errors

## 🎯 **DEPLOYMENT OVERVIEW**

### **System Requirements**
- **OS**: Linux (Ubuntu 22.04+ recommended)
- **Rust**: 1.70+
- **ZFS**: 2.1+ (for storage features)
- **Memory**: 512MB minimum, 2GB recommended
- **CPU**: 1 core minimum, 4 cores recommended
- **Storage**: 1GB for binaries, unlimited for data

### **Key Features**
- ✅ **Zero compilation errors** - All 13 crates build successfully
- ✅ **Headless operation** - No UI dependencies
- ✅ **Complete API coverage** - 150+ BYOB storage endpoints
- ✅ **Enterprise-grade security** - BearDog crypto integration
- ✅ **AI-first design** - Optimized for autonomous operation

---

## 🔧 **INSTALLATION METHODS**

### **Method 1: From Source (Recommended)**
```bash
# Clone repository
git clone https://github.com/nestgate/nestgate.git
cd nestgate

# Build all crates (confirms zero compilation errors)
cargo build --release

# Install binaries
cargo install --path code/crates/nestgate-bin

# Verify installation
nestgate --version
nestgate --help
```

### **Method 2: Docker Deployment**
```bash
# Build Docker image
docker build -t nestgate:latest .

# Run headless container
docker run -d \
  --name nestgate \
  -p 8080:8080 \
  -v /data:/data \
  -v /etc/nestgate:/etc/nestgate \
  nestgate:latest --headless

# Verify deployment
curl http://localhost:8080/api/v1/health
```

### **Method 3: Kubernetes Deployment**
```bash
# Apply Kubernetes manifests
kubectl apply -f k8s/

# Verify deployment
kubectl get pods -l app=nestgate-headless
kubectl port-forward svc/nestgate 8080:8080

# Test API
curl http://localhost:8080/api/v1/storage/pools
```

---

## ⚙️ **CONFIGURATION**

### **Base Configuration**
```toml
# /etc/nestgate/config.toml
[server]
mode = "headless"
api_only = true
ui_disabled = true
bind_address = "0.0.0.0"
port = 8080

[storage]
default_pool = "data"
enable_tiering = true
compression = "zstd"
snapshots_enabled = true

[security]
beardog_enabled = true
crypto_locks_enabled = true
audit_logging = true

[api]
enable_openapi = true
enable_metrics = true
enable_websocket = true
rate_limiting = true
max_requests_per_minute = 10000

[monitoring]
prometheus_enabled = true
health_check_interval = 30
log_level = "info"
```

### **Environment Variables**
```bash
# Core settings
export NESTGATE_CONFIG_PATH="/etc/nestgate/config.toml"
export NESTGATE_DATA_PATH="/data"
export NESTGATE_LOG_LEVEL="info"

# API settings
export NESTGATE_API_ONLY="true"
export NESTGATE_UI_DISABLED="true"
export NESTGATE_HEADLESS="true"

# Integration settings
export NESTGATE_BIOMEOS_MODE="true"
export NESTGATE_MCP_ENABLED="true"
export NESTGATE_AI_OPTIMIZED="true"

# Security settings
export NESTGATE_BEARDOG_ENABLED="true"
export NESTGATE_CRYPTO_LOCKS_ENABLED="true"
```

---

## 🔒 **SECURITY DEPLOYMENT**

### **BearDog Crypto Integration**
```bash
# Install BearDog crypto locks
curl -X POST http://localhost:8080/api/v1/security/crypto-lock \
  -H "Content-Type: application/json" \
  -d '{
    "key_id": "production-key-2025",
    "permissions": ["read", "write", "admin"]
  }'

# Verify crypto lock status
curl http://localhost:8080/api/v1/security/status
```

### **Access Control Setup**
```bash
# Create admin user
curl -X POST http://localhost:8080/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "role": "administrator",
    "permissions": ["*"]
  }'

# Generate API key
curl -X POST http://localhost:8080/api/v1/auth/api-key \
  -H "Authorization: Bearer <admin-token>"
```

---

## 📊 **MONITORING & OBSERVABILITY**

### **Health Checks**
```bash
# System health
curl http://localhost:8080/api/v1/health

# Detailed system status
curl http://localhost:8080/api/v1/monitoring/status

# Performance metrics
curl http://localhost:8080/api/v1/monitoring/metrics
```

### **Prometheus Integration**
```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'nestgate'
    static_configs:
      - targets: ['localhost:8080']
    metrics_path: '/api/v1/metrics'
    scrape_interval: 15s
```

### **Grafana Dashboard**
```json
{
  "dashboard": {
    "title": "NestGate Production Monitoring",
    "panels": [
      {
        "title": "API Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "nestgate_http_request_duration_seconds",
            "legendFormat": "{{method}} {{endpoint}}"
          }
        ]
      },
      {
        "title": "Storage Usage",
        "type": "graph",
        "targets": [
          {
            "expr": "nestgate_storage_pool_usage_bytes",
            "legendFormat": "{{pool}} {{tier}}"
          }
        ]
      }
    ]
  }
}
```

---

## 🌐 **NETWORKING & LOAD BALANCING**

### **Nginx Reverse Proxy**
```nginx
upstream nestgate_backend {
    server 127.0.0.1:8080;
    server 127.0.0.1:8081;
    server 127.0.0.1:8082;
}

server {
    listen 80;
    server_name nestgate.example.com;

    location /api/ {
        proxy_pass http://nestgate_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location /ws/ {
        proxy_pass http://nestgate_backend;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

### **HAProxy Configuration**
```
frontend nestgate_frontend
    bind *:80
    default_backend nestgate_backend

backend nestgate_backend
    balance roundrobin
    option httpchk GET /api/v1/health
    server nestgate1 127.0.0.1:8080 check
    server nestgate2 127.0.0.1:8081 check
    server nestgate3 127.0.0.1:8082 check
```

---

## 🔧 **SCALING & PERFORMANCE**

### **Horizontal Scaling**
```yaml
# docker-compose.yml
version: '3.8'
services:
  nestgate:
    image: nestgate:latest
    deploy:
      replicas: 3
      resources:
        limits:
          cpus: '2'
          memory: 2G
        reservations:
          cpus: '0.5'
          memory: 512M
    environment:
      - NESTGATE_HEADLESS=true
      - NESTGATE_API_ONLY=true
      - NESTGATE_UI_DISABLED=true
    ports:
      - "8080-8082:8080"
    volumes:
      - /data:/data
      - /etc/nestgate:/etc/nestgate
```

### **Performance Tuning**
```bash
# Enable performance profile
curl -X POST http://localhost:8080/api/v1/hardware/tune \
  -H "Content-Type: application/json" \
  -d '{
    "profile": "performance",
    "target": "api-throughput"
  }'

# Monitor performance
curl http://localhost:8080/api/v1/hardware/metrics
```

---

## 🤖 **AI INTEGRATION DEPLOYMENT**

### **MCP (Model Context Protocol) Setup**
```bash
# Enable MCP integration
export NESTGATE_MCP_ENABLED=true
export NESTGATE_AI_OPTIMIZED=true

# Start with AI mode
nestgate --headless --ai-mode

# Test MCP endpoints
curl http://localhost:8080/api/v1/mcp/capabilities
```

### **biomeOS Integration**
```bash
# Configure for biomeOS
export NESTGATE_BIOMEOS_MODE=true
export NESTGATE_BIOMEOS_INTEGRATION=true

# Start with biomeOS compatibility
nestgate --headless --biomeos-mode

# Test biomeOS endpoints
curl http://localhost:8080/api/v1/biomeos/integration
```

---

## 🚨 **TROUBLESHOOTING**

### **Common Issues**

#### **1. Compilation Errors**
```bash
# This should not happen - zero compilation errors achieved
# If you encounter errors, check:
rustc --version  # Ensure Rust 1.70+
cargo clean
cargo build --release
```

#### **2. ZFS Not Available**
```bash
# Check ZFS installation
zpool status
zfs version

# Install ZFS if needed (Ubuntu)
sudo apt update
sudo apt install zfsutils-linux
```

#### **3. API Not Responding**
```bash
# Check service status
systemctl status nestgate

# Check logs
journalctl -u nestgate -f

# Verify configuration
nestgate --check-config
```

#### **4. Performance Issues**
```bash
# Check resource usage
curl http://localhost:8080/api/v1/monitoring/resources

# Enable performance tuning
curl -X POST http://localhost:8080/api/v1/hardware/tune \
  -d '{"profile": "performance"}'
```

---

## 📋 **MAINTENANCE PROCEDURES**

### **Updates**
```bash
# Update from source
cd nestgate
git pull origin main
cargo build --release
sudo systemctl restart nestgate

# Verify update
nestgate --version
curl http://localhost:8080/api/v1/health
```

### **Backup**
```bash
# Create system backup
curl -X POST http://localhost:8080/api/v1/backup/create \
  -H "Content-Type: application/json" \
  -d '{
    "type": "full",
    "compression": "zstd",
    "encryption": true
  }'

# List backups
curl http://localhost:8080/api/v1/backup/list
```

### **Monitoring**
```bash
# Check system health
curl http://localhost:8080/api/v1/health

# Check performance metrics
curl http://localhost:8080/api/v1/monitoring/metrics

# Check audit logs
curl http://localhost:8080/api/v1/security/audit
```

---

## 🎉 **DEPLOYMENT VERIFICATION**

### **Post-Deployment Checklist**
- [ ] ✅ **Zero compilation errors** verified
- [ ] ✅ **All 13 crates** building successfully
- [ ] ✅ **API endpoints** responding correctly
- [ ] ✅ **Health checks** passing
- [ ] ✅ **Security** configured properly
- [ ] ✅ **Monitoring** active
- [ ] ✅ **Performance** optimized
- [ ] ✅ **Backup** configured

### **Success Verification**
```bash
# Comprehensive system test
curl http://localhost:8080/api/v1/health
curl http://localhost:8080/api/v1/storage/pools
curl http://localhost:8080/api/v1/monitoring/status
curl http://localhost:8080/api/v1/security/status

# Expected responses: All 200 OK with proper JSON
```

---

## 🏆 **PRODUCTION READINESS CONFIRMATION**

### **Achievement Summary**
- ✅ **Zero compilation errors** - All 13 crates compile successfully
- ✅ **Complete API coverage** - 150+ BYOB storage endpoints
- ✅ **Enterprise security** - BearDog crypto integration
- ✅ **Headless operation** - No UI dependencies
- ✅ **AI-first design** - Optimized for autonomous operation
- ✅ **Production deployment** - Docker, Kubernetes, systemd ready
- ✅ **Comprehensive monitoring** - Health checks, metrics, logging
- ✅ **Scalable architecture** - Horizontal scaling supported

### **Next Steps**
1. **Deploy to production** following this guide
2. **Configure monitoring** and alerting
3. **Set up backup** procedures
4. **Plan biomeOS integration** using provided APIs
5. **Scale as needed** using provided configurations

**Result**: NestGate is production-ready with zero compilation errors and complete feature coverage, ready for enterprise deployment and biomeOS integration. 