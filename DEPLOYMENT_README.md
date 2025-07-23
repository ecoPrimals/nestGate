# 🚀 NestGate Production Deployment Guide

Complete guide for deploying NestGate storage system in production environments with monitoring and observability.

## 📋 Prerequisites

### System Requirements
- **OS**: Linux (Ubuntu 20.04+, Debian 11+, RHEL 8+)
- **CPU**: 4+ cores (8+ recommended)
- **Memory**: 8GB+ RAM (16GB+ recommended)
- **Storage**: 100GB+ available disk space
- **Network**: Stable internet connection

### Software Dependencies
- **Docker**: 20.10+ 
- **Docker Compose**: 2.0+
- **ZFS** (optional): For advanced storage features
- **curl**: For health checks

### Port Requirements
- `8000`: NestGate API
- `8080`: WebSocket connections
- `9090`: Metrics endpoint
- `9091`: Prometheus monitoring
- `3000`: Grafana dashboard
- `3100`: Loki log aggregation

## 🔧 Quick Start Deployment

### 1. Clone and Setup
```bash
# Clone repository (or extract release)
git clone <repository-url> nestgate
cd nestgate

# Run automated deployment
./deploy.sh
```

### 2. Verify Deployment
```bash
# Run comprehensive health check
./scripts/health-check.sh --verbose

# Check individual endpoints
curl http://localhost:8000/health
curl http://localhost:9090/metrics
```

### 3. Access Services
- **NestGate API**: http://localhost:8000
- **Grafana Dashboard**: http://localhost:3000 (admin/nestgate_admin_2024)
- **Prometheus Metrics**: http://localhost:9091

## ⚙️ Configuration

### Environment Variables
Create or modify `.env` file:

```bash
# Core Configuration
NESTGATE_ENVIRONMENT=production
NESTGATE_LOG_LEVEL=info
NESTGATE_API_PORT=8000

# Performance Tuning
NESTGATE_DEFAULT_CACHE_SIZE=4294967296    # 4GB
NESTGATE_MAX_FILE_SIZE=214748364800       # 200GB
NESTGATE_CONNECTION_TIMEOUT_MS=30000      # 30s
NESTGATE_REQUEST_TIMEOUT_MS=60000         # 60s

# Security
GRAFANA_ADMIN_PASSWORD=your_secure_password
```

### Storage Configuration
Edit `docker/production.toml` for storage tiers:

```toml
[[storage.tiers]]
name = "hot"
tier_type = "hot"
path = "/opt/nestgate/data/hot"
capacity = 2199023255552  # 2TB

[[storage.tiers]]
name = "warm"
tier_type = "warm"
path = "/opt/nestgate/data/warm"  
capacity = 10995116277760  # 10TB
```

### Monitoring Configuration
Customize monitoring in `docker/prometheus.yml`:

```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'nestgate'
    static_configs:
      - targets: ['nestgate:9090']
    scrape_interval: 10s
```

## 🐳 Docker Deployment

### Manual Docker Commands
```bash
# Build image
docker build -t nestgate:latest .

# Run with basic configuration
docker run -d \
  --name nestgate \
  -p 8000:8000 \
  -p 8080:8080 \
  -p 9090:9090 \
  -v /opt/nestgate/data:/opt/nestgate/data \
  -v /opt/nestgate/logs:/opt/nestgate/logs \
  --env-file .env \
  nestgate:latest
```

### Docker Compose (Recommended)
```bash
# Start full stack
docker-compose up -d

# View logs
docker-compose logs -f nestgate

# Scale services
docker-compose up -d --scale nestgate=3

# Stop services
docker-compose down
```

## 🔍 Monitoring & Observability

### Grafana Dashboards
1. Open http://localhost:3000
2. Login with `admin / nestgate_admin_2024`
3. Import NestGate dashboards from `/docker/grafana/dashboards/`

### Prometheus Metrics
Key metrics available at http://localhost:9091:
- `nestgate_api_requests_total`
- `nestgate_storage_capacity_bytes`
- `nestgate_zfs_pool_health`
- `nestgate_websocket_connections`

### Log Aggregation
- **Loki**: http://localhost:3100
- **Container logs**: `docker-compose logs -f`
- **System logs**: `/opt/nestgate/logs/`

## 🔒 Security Configuration

### Network Security
```toml
[security.network]
default_bind_interface = "0.0.0.0"
localhost_only = false
max_bind_interfaces = 3
```

### Authentication
```toml
[security]
auth_method = "jwt"
key_rotation_days = 30
max_failed_attempts = 5

[security.rbac]
enabled = true
default_role = "user"
```

### TLS/SSL (Optional)
```bash
# Generate certificates
openssl req -x509 -newkey rsa:4096 \
  -keyout /opt/nestgate/config/server.key \
  -out /opt/nestgate/config/server.crt \
  -days 365 -nodes

# Update configuration
echo 'NESTGATE_TLS_ENABLED=true' >> .env
```

## 📊 Performance Tuning

### Resource Allocation
```yaml
# In docker-compose.yml
deploy:
  resources:
    limits:
      cpus: '8.0'
      memory: 16G
    reservations:
      cpus: '4.0'
      memory: 8G
```

### Storage Optimization
```toml
[storage_constants.memory_limits]
default_cache_size = 4294967296      # 4GB
max_cache_size = 17179869184        # 16GB

[storage_constants.performance_thresholds]
io_operation_timeout_ms = 45000     # 45s
```

### ZFS Optimization (if available)
```bash
# Set ZFS properties for performance
zfs set primarycache=all rpool/nestgate
zfs set secondarycache=all rpool/nestgate
zfs set compression=lz4 rpool/nestgate
```

## 🏥 Health Checks & Maintenance

### Automated Health Monitoring
```bash
# Run health checks every 5 minutes
*/5 * * * * /opt/nestgate/scripts/health-check.sh > /dev/null 2>&1

# Verbose health check
./scripts/health-check.sh --verbose
```

### Log Rotation
```bash
# Automated log rotation (add to crontab)
0 2 * * * docker-compose exec nestgate logrotate /etc/logrotate.conf
```

### Backup Strategy
```bash
# Backup data volumes
docker run --rm -v nestgate_data:/data -v $(pwd):/backup \
  ubuntu tar czf /backup/nestgate-data-$(date +%Y%m%d).tar.gz /data

# Backup configuration
cp -r docker/ /backup/nestgate-config-$(date +%Y%m%d)/
```

## 🚨 Troubleshooting

### Common Issues

#### Service Won't Start
```bash
# Check container logs
docker-compose logs nestgate

# Verify configuration
docker-compose config

# Check port conflicts
netstat -tlnp | grep -E ':(8000|8080|9090)'
```

#### High Memory Usage
```bash
# Check resource usage
docker stats

# Reduce cache size
export NESTGATE_DEFAULT_CACHE_SIZE=1073741824  # 1GB
docker-compose restart nestgate
```

#### ZFS Issues
```bash
# Check ZFS status
zpool status

# Import pools if needed
zpool import -a

# Check device permissions
ls -la /dev/zfs
```

### Performance Issues
```bash
# Monitor real-time metrics
watch -n 1 'curl -s http://localhost:9090/metrics | grep nestgate'

# Check system resources
htop
iotop
```

## 🔄 Updates & Maintenance

### Rolling Updates
```bash
# Pull latest image
docker-compose pull nestgate

# Rolling restart
docker-compose up -d --no-deps nestgate
```

### Configuration Changes
```bash
# Reload configuration without restart
docker-compose exec nestgate killall -SIGHUP nestgate

# Full restart if needed
docker-compose restart nestgate
```

### Database Maintenance
```bash
# Compact database
docker-compose exec nestgate nestgate --compact-db

# Check storage integrity
docker-compose exec nestgate nestgate --verify-storage
```

## 📞 Support & Documentation

### Log Analysis
```bash
# View structured logs
docker-compose logs nestgate | jq '.'

# Filter error logs
docker-compose logs nestgate | grep -i error

# Export logs for analysis
docker-compose logs --no-color nestgate > nestgate.log
```

### Metrics Export
```bash
# Export Prometheus metrics
curl -s http://localhost:9091/api/v1/query?query=up > metrics.json

# Generate performance report
./scripts/health-check.sh > health-report.txt
```

### Getting Help
- **Documentation**: [Internal docs](./docs/)
- **Health Check**: `./scripts/health-check.sh --verbose`
- **Configuration**: Check `docker/production.toml`
- **Monitoring**: Grafana dashboard at http://localhost:3000

---

## 🎯 Production Checklist

- [ ] System requirements met
- [ ] Docker & Docker Compose installed
- [ ] Ports 8000, 8080, 9090 available
- [ ] `/opt/nestgate` directory created with proper permissions
- [ ] Environment variables configured
- [ ] Security settings reviewed
- [ ] Monitoring stack deployed
- [ ] Health checks passing
- [ ] Backup strategy implemented
- [ ] Log rotation configured
- [ ] Performance tuning applied
- [ ] Team trained on operations

**✅ Ready for Production Deployment!** 