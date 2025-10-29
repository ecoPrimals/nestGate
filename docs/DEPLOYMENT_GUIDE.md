# 🚀 NestGate Production Deployment Guide

> **Status**: Production Ready | **Version**: 2.0.0 | **Date**: September 10, 2025

This guide provides comprehensive instructions for deploying NestGate v2.0.0 in production environments.

## 📋 Table of Contents

- [🎯 Overview](#-overview)
- [⚙️ Prerequisites](#️-prerequisites)
- [🚀 Quick Deployment](#-quick-deployment)
- [🔧 Configuration](#-configuration)
- [🐳 Docker Deployment](#-docker-deployment)
- [🏗️ Manual Deployment](#️-manual-deployment)
- [📊 Monitoring Setup](#-monitoring-setup)
- [🔒 Security Configuration](#-security-configuration)
- [🌍 Ecosystem Integration](#-ecosystem-integration)
- [🧪 Validation](#-validation)
- [🚨 Troubleshooting](#-troubleshooting)

## 🎯 Overview

NestGate v2.0.0 features a completely modernized architecture designed for production deployment:

### ✨ Deployment Features

- **🏆 Zero-Downtime Deployment**: Rolling updates with health checks
- **📊 Built-in Monitoring**: Prometheus metrics and Grafana dashboards
- **🔒 Production Security**: TLS, RBAC, and sovereignty compliance
- **⚡ High Performance**: Zero-cost abstractions and native async
- **🌍 Ecosystem Ready**: Seamless integration with ecoPrimals services
- **🛡️ Fault Tolerance**: Automatic recovery and circuit breakers

## ⚙️ Prerequisites

### System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **CPU** | 2 cores | 4+ cores |
| **Memory** | 2GB RAM | 4GB+ RAM |
| **Storage** | 20GB | 50GB+ SSD |
| **Network** | 100 Mbps | 1 Gbps |

### Software Dependencies

```bash
# Required tools
docker --version          # 20.10+
docker-compose --version  # 1.29+
curl --version           # 7.68+
jq --version             # 1.6+

# Optional (for development)
cargo --version          # 1.70+
rust --version          # 1.70+
```

### ZFS Support (Optional)

For advanced storage features:

```bash
# Ubuntu/Debian
sudo apt install zfsutils-linux

# CentOS/RHEL
sudo dnf install zfs

# Verify ZFS
sudo zpool status
```

## 🚀 Quick Deployment

### 1. Clone Repository

```bash
git clone https://github.com/ecoprimals/nestgate.git
cd nestgate
```

### 2. Configure Environment

```bash
# Copy production configuration
cp deploy/production.env .env

# Edit configuration (see Configuration section)
nano .env
```

### 3. Deploy Services

```bash
# Run deployment script
./deploy/deploy.sh

# Verify deployment
curl http://localhost:8080/health
```

### 4. Access Services

- **NestGate API**: http://localhost:8080
- **Health Check**: http://localhost:8080/health
- **Metrics**: http://localhost:9090/metrics
- **Grafana Dashboard**: http://localhost:3000

## 🔧 Configuration

### Core Configuration

Edit `.env` or `deploy/production.env`:

```bash
# Core Service Configuration
NESTGATE_API_PORT=8080
NESTGATE_BIND_ADDRESS=0.0.0.0
NESTGATE_WORKERS=auto
NESTGATE_REQUEST_TIMEOUT_SECS=30

# Storage Configuration
NESTGATE_STORAGE_BACKEND=zfs
NESTGATE_STORAGE_ZFS_POOL=tank
NESTGATE_STORAGE_DATA_DIR=/var/lib/nestgate

# Performance Configuration
NESTGATE_ZERO_COPY=true
NESTGATE_BUFFER_SIZE=65536
NESTGATE_THREAD_POOL_SIZE=auto
NESTGATE_ASYNC_RUNTIME=tokio

# Security Configuration
NESTGATE_TLS_ENABLED=true
NESTGATE_AUTH_MODE=primal
NESTGATE_RBAC_ENABLED=true
```

### Environment-Specific Configurations

#### Development
```bash
NESTGATE_LOG_LEVEL=debug
NESTGATE_DEV_MODE=true
NESTGATE_MOCK_SERVICES=true
```

#### Staging
```bash
NESTGATE_LOG_LEVEL=info
NESTGATE_METRICS_ENABLED=true
NESTGATE_TRACING_ENABLED=true
```

#### Production
```bash
NESTGATE_LOG_LEVEL=warn
NESTGATE_METRICS_ENABLED=true
NESTGATE_BACKUP_ENABLED=true
NESTGATE_TLS_ENABLED=true
```

## 🐳 Docker Deployment

### Using Docker Compose (Recommended)

```bash
# Deploy full stack
docker-compose -f deploy/production.yml up -d

# View logs
docker-compose -f deploy/production.yml logs -f

# Scale services
docker-compose -f deploy/production.yml up -d --scale nestgate=3

# Stop services
docker-compose -f deploy/production.yml down
```

### Manual Docker Deployment

```bash
# Build image
docker build -t nestgate:2.0.0 .

# Run container
docker run -d \
  --name nestgate-production \
  --restart unless-stopped \
  -p 8080:8080 \
  -p 8090:8090 \
  -v nestgate-data:/var/lib/nestgate \
  -v nestgate-config:/etc/nestgate \
  --env-file deploy/production.env \
  nestgate:2.0.0

# Check status
docker logs nestgate-production
```

### Container Health Checks

```bash
# Check container health
docker inspect --format='{{.State.Health.Status}}' nestgate-production

# View health check logs
docker inspect --format='{{range .State.Health.Log}}{{.Output}}{{end}}' nestgate-production
```

## 🏗️ Manual Deployment

### 1. Build from Source

```bash
# Clone repository
git clone https://github.com/ecoprimals/nestgate.git
cd nestgate

# Build release
cargo build --release --workspace

# Install binary
sudo cp target/release/nestgate-bin /usr/local/bin/nestgate
sudo chmod +x /usr/local/bin/nestgate
```

### 2. System Service Setup

Create systemd service file:

```bash
sudo tee /etc/systemd/system/nestgate.service << 'EOF'
[Unit]
Description=NestGate Storage Service
After=network.target
Wants=network.target

[Service]
Type=exec
User=nestgate
Group=nestgate
ExecStart=/usr/local/bin/nestgate
Restart=always
RestartSec=10
EnvironmentFile=/etc/nestgate/production.env
WorkingDirectory=/var/lib/nestgate

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/nestgate /var/log/nestgate

[Install]
WantedBy=multi-user.target
EOF

# Enable and start service
sudo systemctl enable nestgate
sudo systemctl start nestgate
sudo systemctl status nestgate
```

### 3. User and Directory Setup

```bash
# Create nestgate user
sudo useradd -r -s /bin/false nestgate

# Create directories
sudo mkdir -p /var/lib/nestgate /var/log/nestgate /etc/nestgate
sudo chown -R nestgate:nestgate /var/lib/nestgate /var/log/nestgate
sudo chmod 755 /var/lib/nestgate /var/log/nestgate

# Copy configuration
sudo cp deploy/production.env /etc/nestgate/
sudo chown nestgate:nestgate /etc/nestgate/production.env
sudo chmod 600 /etc/nestgate/production.env
```

## 📊 Monitoring Setup

### Prometheus Configuration

Create `monitoring/prometheus.yml`:

```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'nestgate'
    static_configs:
      - targets: ['nestgate:9090']
    scrape_interval: 10s
    metrics_path: /metrics

  - job_name: 'node-exporter'
    static_configs:
      - targets: ['node-exporter:9100']
```

### Grafana Dashboards

Import pre-configured dashboards:

```bash
# Copy dashboard configurations
cp -r monitoring/grafana/dashboards /var/lib/grafana/dashboards/
cp -r monitoring/grafana/datasources /var/lib/grafana/datasources/

# Restart Grafana
docker-compose restart grafana
```

### Key Metrics to Monitor

| Metric | Description | Alert Threshold |
|--------|-------------|-----------------|
| `nestgate_requests_total` | Total requests | N/A |
| `nestgate_request_duration_seconds` | Request latency | >1s |
| `nestgate_memory_usage_bytes` | Memory usage | >80% of limit |
| `nestgate_active_connections` | Active connections | >1000 |
| `nestgate_storage_usage_bytes` | Storage usage | >90% of capacity |

## 🔒 Security Configuration

### TLS Certificate Setup

```bash
# Generate self-signed certificates (development only)
openssl req -x509 -newkey rsa:4096 -keyout server.key -out server.crt -days 365 -nodes

# Or use Let's Encrypt (production)
certbot certonly --standalone -d your-domain.com

# Copy certificates
sudo cp server.crt /etc/nestgate/certs/
sudo cp server.key /etc/nestgate/certs/
sudo chown -R nestgate:nestgate /etc/nestgate/certs/
sudo chmod 600 /etc/nestgate/certs/*
```

### RBAC Configuration

Create `rbac.yml`:

```yaml
roles:
  - name: admin
    permissions:
      - "*"
  
  - name: operator
    permissions:
      - "storage:read"
      - "storage:write"
      - "metrics:read"
  
  - name: readonly
    permissions:
      - "storage:read"
      - "metrics:read"

users:
  - name: admin
    roles: ["admin"]
  
  - name: operator
    roles: ["operator"]
```

### Firewall Configuration

```bash
# UFW (Ubuntu)
sudo ufw allow 8080/tcp  # NestGate API
sudo ufw allow 8090/tcp  # Health checks
sudo ufw allow 9090/tcp  # Metrics
sudo ufw enable

# iptables
sudo iptables -A INPUT -p tcp --dport 8080 -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 8090 -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 9090 -j ACCEPT
```

## 🌍 Ecosystem Integration

### Service Discovery Configuration

```bash
# Enable ecosystem discovery
NESTGATE_ECOSYSTEM_DISCOVERY=true
NESTGATE_DISCOVERY_INTERVAL=60

# Configure service endpoints
ORCHESTRATION_DISCOVERY_ENDPOINT=http://songbird:8081
SECURITY_DISCOVERY_ENDPOINT=http://beardog:8082
MANAGEMENT_DISCOVERY_ENDPOINT=http://biomeos:8083
```

### Integration Validation

```bash
# Test ecosystem connectivity
curl http://localhost:8080/ecosystem/status

# Check service discovery
curl http://localhost:8080/discovery/services
```

## 🧪 Validation

### Deployment Validation Script

```bash
#!/bin/bash
# validate-deployment.sh

echo "🔍 Validating NestGate deployment..."

# Health check
if curl -f -s http://localhost:8080/health > /dev/null; then
    echo "✅ Health check passed"
else
    echo "❌ Health check failed"
    exit 1
fi

# Metrics check
if curl -f -s http://localhost:9090/metrics > /dev/null; then
    echo "✅ Metrics endpoint accessible"
else
    echo "❌ Metrics endpoint failed"
fi

# Performance test
echo "⚡ Running performance test..."
ab -n 1000 -c 10 http://localhost:8080/health

echo "🎉 Validation completed"
```

### Load Testing

```bash
# Install Apache Bench
sudo apt install apache2-utils

# Basic load test
ab -n 10000 -c 100 http://localhost:8080/health

# Extended load test with wrk
wrk -t12 -c400 -d30s --latency http://localhost:8080/health
```

## 🚨 Troubleshooting

### Common Issues

#### 1. Service Won't Start

```bash
# Check logs
docker logs nestgate-production
journalctl -u nestgate -f

# Check configuration
./deploy/deploy.sh validate

# Check port conflicts
netstat -tulpn | grep :8080
```

#### 2. High Memory Usage

```bash
# Check memory metrics
curl http://localhost:9090/metrics | grep memory

# Adjust configuration
NESTGATE_MAX_MEMORY_MB=2048
NESTGATE_CACHE_MAX_MEMORY_MB=512
```

#### 3. Performance Issues

```bash
# Check CPU usage
top -p $(pgrep nestgate)

# Adjust worker configuration
NESTGATE_WORKERS=8
NESTGATE_THREAD_POOL_SIZE=16
```

#### 4. Storage Issues

```bash
# Check ZFS status
sudo zpool status

# Check disk space
df -h /var/lib/nestgate

# Check storage metrics
curl http://localhost:9090/metrics | grep storage
```

### Debug Mode

Enable debug logging:

```bash
# Temporary debug
NESTGATE_LOG_LEVEL=debug docker-compose restart nestgate

# Persistent debug
echo "NESTGATE_LOG_LEVEL=debug" >> .env
```

### Health Check Commands

```bash
# Comprehensive health check
curl -s http://localhost:8080/health | jq .

# Service-specific checks
curl -s http://localhost:8080/health/storage
curl -s http://localhost:8080/health/network
curl -s http://localhost:8080/health/ecosystem
```

---

## 📞 Support

- **Documentation**: [docs.ecoprimals.com/nestgate](https://docs.ecoprimals.com/nestgate)
- **Issues**: [GitHub Issues](https://github.com/ecoprimals/nestgate/issues)
- **Email**: admin@ecoprimals.com

---

**🏆 NestGate v2.0.0 - Production Ready** | **Built with ❤️ by the ecoPrimals team** 