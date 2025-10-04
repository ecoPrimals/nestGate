# 🚀 **NESTGATE PRODUCTION DEPLOYMENT GUIDE**

**Version**: 0.2.0-production  
**Status**: Production-Ready  
**Last Updated**: December 24, 2025

---

## 📋 **DEPLOYMENT OVERVIEW**

NestGate is now production-ready with enterprise-grade service discovery, distributed caching, and comprehensive configuration management. This guide covers all deployment scenarios from development to enterprise production.

### **🎯 Deployment Options**
- **Native Deployment**: Direct binary execution with environment configuration
- **Container Deployment**: Docker with production configuration
- **Kubernetes Deployment**: Cloud-native with service discovery integration
- **Development Setup**: Local development with hot-reload

---

## 🔧 **PREREQUISITES**

### **System Requirements**
- **OS**: Linux (Ubuntu 20.04+, RHEL 8+, or equivalent)
- **Rust**: 1.75+ (for building from source)
- **Memory**: 2GB+ RAM (4GB+ recommended for production)
- **Storage**: 10GB+ available space
- **Network**: Outbound HTTPS access for service discovery

### **Optional Dependencies**
- **ZFS**: For native ZFS backend (recommended for storage features)
- **Consul**: For enterprise service discovery
- **Kubernetes**: For cloud-native deployment
- **Docker**: For containerized deployment

---

## ⚡ **QUICK START - PRODUCTION DEPLOYMENT**

### **1. Download and Extract**
```bash
# Download production release
wget https://github.com/your-org/nestgate/releases/download/v0.2.0/nestgate-v0.2.0-linux-x86_64.tar.gz
tar -xzf nestgate-v0.2.0-linux-x86_64.tar.gz
cd nestgate-v0.2.0
```

### **2. Configure Environment**
```bash
# Copy production environment template
cp config/production.env.example .env

# Edit configuration for your environment
nano .env
```

### **3. Launch Production Service**
```bash
# Start NestGate with production configuration
./nestgate-server --config .env

# Verify deployment
curl http://localhost:8080/health
```

---

## 🔐 **PRODUCTION CONFIGURATION**

### **Essential Environment Variables**

#### **Network Configuration**
```bash
# API Server
NESTGATE_API_PORT=8080
NESTGATE_BIND_ADDRESS=0.0.0.0

# Connection Limits
NESTGATE_MAX_CONCURRENT_REQUESTS=10000
NESTGATE_SEND_BUFFER_SIZE=65536
NESTGATE_RECV_BUFFER_SIZE=65536
```

#### **Service Discovery**
```bash
# Consul (Recommended for Production)
CONSUL_HTTP_ADDR=http://consul.internal:8500

# Kubernetes (For K8s Deployments)
KUBERNETES_SERVICE_HOST=kubernetes.default.svc
KUBERNETES_SERVICE_PORT=443

# Timeouts
NESTGATE_DISCOVERY_TIMEOUT_MS=5000
NESTGATE_HEALTH_CHECK_INTERVAL_SECS=30
```

#### **ZFS Backend**
```bash
# Backend Selection
NESTGATE_ZFS_BACKEND=native  # or 'remote'

# Remote ZFS (if using remote backend)
NESTGATE_ZFS_REMOTE_ENDPOINT=http://zfs-service:8080
```

#### **Caching Configuration**
```bash
# Cache Storage
NESTGATE_CACHE_DIR=/var/cache/nestgate

# Ensure cache directory exists and is writable
sudo mkdir -p /var/cache/nestgate
sudo chown nestgate:nestgate /var/cache/nestgate
```

---

## 🐳 **DOCKER DEPLOYMENT**

### **Production Docker Setup**
```bash
# Build production container
docker build -f docker/Dockerfile.production -t nestgate:0.2.0 .

# Run with production configuration
docker run -d \
  --name nestgate-production \
  --env-file config/production.env \
  -p 8080:8080 \
  -p 8082:8082 \
  -v /var/cache/nestgate:/var/cache/nestgate \
  nestgate:0.2.0
```

### **Docker Compose Production**
```yaml
# docker-compose.production.yml
version: '3.8'
services:
  nestgate:
    image: nestgate:0.2.0
    ports:
      - "8080:8080"  # API
      - "8082:8082"  # Health checks
    environment:
      - NESTGATE_ENVIRONMENT=production
      - CONSUL_HTTP_ADDR=http://consul:8500
      - NESTGATE_ZFS_BACKEND=native
    volumes:
      - nestgate_cache:/var/cache/nestgate
    restart: unless-stopped

  consul:
    image: consul:1.16
    ports:
      - "8500:8500"
    command: agent -server -bootstrap -ui -client=0.0.0.0

volumes:
  nestgate_cache:
```

```bash
# Deploy with Docker Compose
docker-compose -f docker-compose.production.yml up -d
```

---

## ☸️ **KUBERNETES DEPLOYMENT**

### **Production Kubernetes Manifest**
```yaml
# nestgate-production.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: nestgate-config
data:
  NESTGATE_API_PORT: "8080"
  NESTGATE_ENVIRONMENT: "production"
  NESTGATE_ZFS_BACKEND: "native"
  NESTGATE_CACHE_DIR: "/var/cache/nestgate"

---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nestgate
  labels:
    app: nestgate
spec:
  replicas: 3
  selector:
    matchLabels:
      app: nestgate
  template:
    metadata:
      labels:
        app: nestgate
    spec:
      containers:
      - name: nestgate
        image: nestgate:0.2.0
        ports:
        - containerPort: 8080
          name: api
        - containerPort: 8082
          name: health
        envFrom:
        - configMapRef:
            name: nestgate-config
        volumeMounts:
        - name: cache-storage
          mountPath: /var/cache/nestgate
        livenessProbe:
          httpGet:
            path: /health
            port: 8082
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8082
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: cache-storage
        emptyDir: {}

---
apiVersion: v1
kind: Service
metadata:
  name: nestgate-service
spec:
  selector:
    app: nestgate
  ports:
  - name: api
    port: 8080
    targetPort: 8080
  - name: health
    port: 8082
    targetPort: 8082
  type: ClusterIP
```

```bash
# Deploy to Kubernetes
kubectl apply -f nestgate-production.yaml

# Verify deployment
kubectl get pods -l app=nestgate
kubectl logs deployment/nestgate
```

---

## 🔧 **ADVANCED CONFIGURATION**

### **Load Balancer Setup (Nginx)**
```nginx
# /etc/nginx/sites-available/nestgate
upstream nestgate_backend {
    server 127.0.0.1:8080;
    # Add more instances for load balancing
    # server 127.0.0.1:8081;
    # server 127.0.0.1:8082;
}

server {
    listen 80;
    server_name nestgate.yourdomain.com;

    location / {
        proxy_pass http://nestgate_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location /health {
        proxy_pass http://nestgate_backend/health;
        access_log off;
    }
}
```

### **SSL/TLS Configuration**
```bash
# Generate SSL certificates (Let's Encrypt recommended)
certbot --nginx -d nestgate.yourdomain.com

# Or configure manual certificates
NESTGATE_TLS_ENABLED=true
NESTGATE_TLS_CERT_PATH=/etc/ssl/certs/nestgate.pem
NESTGATE_TLS_KEY_PATH=/etc/ssl/private/nestgate.key
```

---

## 📊 **MONITORING AND OBSERVABILITY**

### **Health Checks**
```bash
# Basic health check
curl http://localhost:8082/health

# Detailed health check with metrics
curl http://localhost:8082/health/detailed

# Service discovery status
curl http://localhost:8080/api/v1/discovery/status
```

### **Metrics Collection**
```bash
# Enable metrics collection
NESTGATE_METRICS_ENABLED=true
NESTGATE_METRICS_PORT=9090

# Prometheus scraping endpoint
curl http://localhost:9090/metrics
```

### **Logging Configuration**
```bash
# Log level configuration
RUST_LOG=info  # Options: trace, debug, info, warn, error

# Structured logging for production
NESTGATE_LOG_FORMAT=json
NESTGATE_LOG_FILE=/var/log/nestgate/nestgate.log
```

---

## 🔒 **SECURITY CONFIGURATION**

### **Production Security Settings**
```bash
# Security configuration
NESTGATE_TLS_ENABLED=true
NESTGATE_SECURITY_AUDIT_ENABLED=true

# Network security
NESTGATE_BIND_ADDRESS=127.0.0.1  # Restrict to localhost if behind proxy
NESTGATE_API_CORS_ORIGIN=https://yourdomain.com

# Authentication (if required)
NESTGATE_AUTH_ENABLED=true
NESTGATE_AUTH_METHOD=jwt  # or oauth2, basic
```

### **Firewall Configuration**
```bash
# UFW (Ubuntu)
sudo ufw allow 8080/tcp  # API port
sudo ufw allow 8082/tcp  # Health check port
sudo ufw enable

# iptables
sudo iptables -A INPUT -p tcp --dport 8080 -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 8082 -j ACCEPT
```

---

## 🚨 **TROUBLESHOOTING**

### **Common Issues**

#### **Service Discovery Issues**
```bash
# Check Consul connectivity
curl http://consul:8500/v1/agent/services

# Verify Kubernetes service account
kubectl auth can-i get services --as=system:serviceaccount:default:nestgate

# Test network connectivity
telnet consul 8500
```

#### **Cache Issues**
```bash
# Check cache directory permissions
ls -la /var/cache/nestgate
sudo chown -R nestgate:nestgate /var/cache/nestgate

# Clear cache if corrupted
rm -rf /var/cache/nestgate/*
```

#### **Performance Issues**
```bash
# Check resource usage
htop
iostat -x 1

# Adjust buffer sizes
NESTGATE_SEND_BUFFER_SIZE=131072
NESTGATE_RECV_BUFFER_SIZE=131072

# Increase connection limits
NESTGATE_MAX_CONCURRENT_REQUESTS=20000
```

### **Log Analysis**
```bash
# View recent logs
journalctl -u nestgate -f

# Search for errors
grep -i error /var/log/nestgate/nestgate.log

# Service discovery logs
grep -i "service discovery" /var/log/nestgate/nestgate.log
```

---

## 📈 **SCALING AND HIGH AVAILABILITY**

### **Horizontal Scaling**
```bash
# Multiple instance deployment
# Instance 1
NESTGATE_API_PORT=8080 ./nestgate-server &

# Instance 2
NESTGATE_API_PORT=8081 ./nestgate-server &

# Instance 3
NESTGATE_API_PORT=8082 ./nestgate-server &
```

### **Database/Cache Scaling**
```bash
# Distributed cache configuration
NESTGATE_CACHE_NODES=node1:8080,node2:8080,node3:8080
NESTGATE_CACHE_CONSISTENCY=strong  # or eventual
```

---

## ✅ **DEPLOYMENT VERIFICATION**

### **Post-Deployment Checklist**
- [ ] Service starts successfully
- [ ] Health checks return 200 OK
- [ ] Service discovery is operational
- [ ] Cache system is functional
- [ ] Logs show no errors
- [ ] Metrics are being collected
- [ ] SSL/TLS is properly configured (if enabled)
- [ ] Load balancer is routing correctly

### **Production Readiness Validation**
```bash
# Run production validation script
./scripts/validate-production-deployment.sh

# Expected output:
# ✅ Service Discovery: Operational
# ✅ Distributed Caching: Functional
# ✅ Configuration: Environment-driven
# ✅ Health Checks: Passing
# ✅ Metrics: Collecting
# 🚀 NestGate: Production-Ready
```

---

## 📞 **SUPPORT**

### **Getting Help**
- **Documentation**: [docs/current/](docs/current/)
- **Configuration Reference**: [config/production.env.example](config/production.env.example)
- **Issues**: GitHub Issues
- **Community**: Discord/Slack channels

### **Emergency Contacts**
- **Production Issues**: production-support@yourdomain.com
- **Security Issues**: security@yourdomain.com

---

**NestGate is production-ready and deployment-tested. This guide provides comprehensive coverage for all deployment scenarios from development to enterprise production.** 