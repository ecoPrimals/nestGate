# 🔧 NestGate Environment Variables Reference
**Last Updated**: November 2, 2025  
**Status**: Comprehensive configuration reference  
**Purpose**: Complete elimination of hardcoded values

---

## 📊 OVERVIEW

NestGate uses environment variables for **100% of configuration**, ensuring:
- ✅ **Zero hardcoded values** in production code
- ✅ **Complete sovereignty** - user controls all settings
- ✅ **Environment-specific** configuration (dev/staging/prod)
- ✅ **Docker/K8s friendly** - containerization ready
- ✅ **Security-first** - secrets never in code

---

## 🌐 NETWORK CONFIGURATION

### API Server
```bash
# API server host (default: "127.0.0.1")
NESTGATE_API_HOST=0.0.0.0

# API server port (default: 8080)
NESTGATE_API_PORT=8080

# Full API bind address (overrides host+port)
NESTGATE_API_BIND=0.0.0.0:8080

# API URL (for client configuration)
NESTGATE_API_URL=http://api.example.com

# Alternative PORT variable (fallback)
PORT=8080
```

### WebSocket
```bash
# WebSocket port (default: 8082)
NESTGATE_WS_PORT=8082

# WebSocket bind address
NESTGATE_WS_BIND=0.0.0.0:8082

# WebSocket URL
NESTGATE_WS_URL=ws://ws.example.com
```

### Health Checks
```bash
# Health check port (default: 8081)
NESTGATE_HEALTH_PORT=8081

# Health check bind address
NESTGATE_HEALTH_BIND=0.0.0.0:8081

# Health check URL
NESTGATE_HEALTH_URL=http://health.example.com/health
```

### Metrics
```bash
# Metrics/monitoring port (default: 9090)
NESTGATE_METRICS_PORT=9090

# Metrics bind address
NESTGATE_METRICS_BIND=0.0.0.0:9090
```

### Storage
```bash
# Storage service port (default: 5000)
NESTGATE_STORAGE_PORT=5000

# Storage bind address
NESTGATE_STORAGE_BIND=0.0.0.0:5000
```

---

## ⏱️ TIMEOUTS

```bash
# Connection timeout in milliseconds (default: 5000)
NESTGATE_CONNECT_TIMEOUT_MS=5000

# Request timeout in milliseconds (default: 30000)
NESTGATE_REQUEST_TIMEOUT_MS=30000

# Long operation timeout in milliseconds (default: 300000)
NESTGATE_LONG_OP_TIMEOUT_MS=300000
```

---

## 🔐 SECURITY

```bash
# Bind only to localhost (default: true for security)
NESTGATE_LOCALHOST_ONLY=true

# Allow external connections (disables localhost_only)
NESTGATE_ALLOW_EXTERNAL=true

# API key for authentication
NESTGATE_API_KEY=your-secret-api-key-here

# Enable TLS/SSL (default: false)
NESTGATE_TLS_ENABLED=true
```

---

## 🏢 SERVICE CONFIGURATION

```bash
# Service ID (default: generated UUID)
NESTGATE_SERVICE_ID=nestgate-prod-01

# Service name (default: "nestgate")
NESTGATE_SERVICE_NAME=nestgate

# Environment (default: "development")
NESTGATE_ENV=production
# Alternatives:
NODE_ENV=production
RUST_ENV=production

# Log level (default: "info", "warn" in production)
NESTGATE_LOG_LEVEL=info
# Alternative:
RUST_LOG=info

# Bind address (default: "127.0.0.1" for security)
NESTGATE_BIND_ADDRESS=0.0.0.0

# External hostname (default: "localhost")
NESTGATE_HOSTNAME=api.example.com
```

---

## 💾 STORAGE CONFIGURATION

```bash
# ZFS backend selection (default: "auto")
# Options: "auto", "native", "docker", "kubernetes"
NESTGATE_ZFS_BACKEND=native

# Storage root path (default: "/var/lib/nestgate")
NESTGATE_STORAGE_PATH=/mnt/data/nestgate

# Temporary directory (default: "/tmp/nestgate")
NESTGATE_TEMP_DIR=/tmp/nestgate
# Alternative:
TMPDIR=/tmp/nestgate
```

---

## 🔍 SERVICE DISCOVERY

```bash
# Service discovery timeout in milliseconds (default: 5000)
NESTGATE_DISCOVERY_TIMEOUT_MS=5000

# Port range for capability scanning
NESTGATE_DISCOVERY_PORT_START=3000
NESTGATE_DISCOVERY_PORT_END=3999

# Custom service endpoints (dynamic)
NESTGATE_SERVICE_ZFS_ENDPOINT=http://zfs.example.com:5000
NESTGATE_SERVICE_SECURITY_ENDPOINT=http://security.example.com:5001
NESTGATE_SERVICE_AI_ENDPOINT=http://ai.example.com:5002
NESTGATE_SERVICE_ORCHESTRATION_ENDPOINT=http://orchestration.example.com:5003
```

---

## 📝 LOGGING

```bash
# Log level options: "trace", "debug", "info", "warn", "error"
NESTGATE_LOG_LEVEL=info
RUST_LOG=nestgate=debug,hyper=warn

# Log format: "json" or "pretty"
NESTGATE_LOG_FORMAT=json

# Log output: "stdout", "stderr", or file path
NESTGATE_LOG_OUTPUT=stdout
```

---

## 🌍 ECOSYSTEM INTEGRATION

```bash
# Universal adapter endpoint
UNIVERSAL_ADAPTER_ENDPOINT=http://adapter.example.com:3000

# Primal service endpoints
ORCHESTRATION_DISCOVERY_ENDPOINT=http://songbird.example.com:3001
SECURITY_DISCOVERY_ENDPOINT=http://beardog.example.com:3002
AI_DISCOVERY_ENDPOINT=http://squirrel.example.com:3003
STORAGE_DISCOVERY_ENDPOINT=http://toadstool.example.com:3004
```

---

## 🐳 CONTAINER EXAMPLES

### Docker Compose
```yaml
version: '3.8'
services:
  nestgate:
    image: nestgate:latest
    environment:
      - NESTGATE_ENV=production
      - NESTGATE_API_HOST=0.0.0.0
      - NESTGATE_API_PORT=8080
      - NESTGATE_BIND_ADDRESS=0.0.0.0
      - NESTGATE_STORAGE_PATH=/data/nestgate
      - NESTGATE_LOG_LEVEL=info
      - NESTGATE_ALLOW_EXTERNAL=true
    ports:
      - "8080:8080"  # API
      - "8081:8081"  # Health
      - "8082:8082"  # WebSocket
      - "9090:9090"  # Metrics
    volumes:
      - nestgate-data:/data/nestgate
```

### Kubernetes ConfigMap
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: nestgate-config
data:
  NESTGATE_ENV: "production"
  NESTGATE_API_HOST: "0.0.0.0"
  NESTGATE_API_PORT: "8080"
  NESTGATE_BIND_ADDRESS: "0.0.0.0"
  NESTGATE_STORAGE_PATH: "/data/nestgate"
  NESTGATE_LOG_LEVEL: "info"
  NESTGATE_ALLOW_EXTERNAL: "true"
```

---

## 🎯 EXAMPLE CONFIGURATIONS

### Development (Local)
```bash
export NESTGATE_ENV=development
export NESTGATE_API_HOST=127.0.0.1
export NESTGATE_API_PORT=8080
export NESTGATE_LOCALHOST_ONLY=true
export NESTGATE_LOG_LEVEL=debug
export NESTGATE_STORAGE_PATH=./data
```

### Staging
```bash
export NESTGATE_ENV=staging
export NESTGATE_API_HOST=0.0.0.0
export NESTGATE_API_PORT=8080
export NESTGATE_ALLOW_EXTERNAL=true
export NESTGATE_LOG_LEVEL=info
export NESTGATE_STORAGE_PATH=/var/lib/nestgate
export NESTGATE_HOSTNAME=staging-api.example.com
```

### Production
```bash
export NESTGATE_ENV=production
export NESTGATE_API_HOST=0.0.0.0
export NESTGATE_API_PORT=8080
export NESTGATE_ALLOW_EXTERNAL=true
export NESTGATE_TLS_ENABLED=true
export NESTGATE_API_KEY=${SECRET_API_KEY}
export NESTGATE_LOG_LEVEL=warn
export NESTGATE_LOG_FORMAT=json
export NESTGATE_STORAGE_PATH=/mnt/data/nestgate
export NESTGATE_HOSTNAME=api.example.com
```

---

## 🔒 SECURITY BEST PRACTICES

### 1. Never Hardcode Secrets
```bash
# ❌ WRONG - Hardcoded in .env file
NESTGATE_API_KEY=secret123

# ✅ CORRECT - Use secret management
NESTGATE_API_KEY=$(vault kv get -field=api_key secret/nestgate)
NESTGATE_API_KEY=$(kubectl get secret nestgate-secrets -o jsonpath='{.data.api-key}' | base64 -d)
```

### 2. Localhost by Default
```bash
# Default behavior: binds to localhost only
# No environment variables = secure

# To expose externally, explicitly enable:
export NESTGATE_ALLOW_EXTERNAL=true
export NESTGATE_API_HOST=0.0.0.0
```

### 3. TLS in Production
```bash
export NESTGATE_TLS_ENABLED=true
export NESTGATE_TLS_CERT_PATH=/etc/nestgate/cert.pem
export NESTGATE_TLS_KEY_PATH=/etc/nestgate/key.pem
```

---

## 📊 VALIDATION

### Check Current Configuration
```bash
# Show all NESTGATE_* variables
env | grep NESTGATE_ | sort

# Test API connectivity
curl http://localhost:8080/health

# Validate configuration
nestgate validate-config
```

### Default Values
```bash
# Without any environment variables, NestGate uses:
API Host:          127.0.0.1  (secure)
API Port:          8080
WebSocket Port:    8082
Health Port:       8081
Metrics Port:      9090
Storage Port:      5000
Environment:       development
Log Level:         info
Localhost Only:    true  (secure)
```

---

## 🚨 TROUBLESHOOTING

### Port Already in Use
```bash
# Change the port:
export NESTGATE_API_PORT=8090
```

### Cannot Connect Externally
```bash
# Enable external access:
export NESTGATE_ALLOW_EXTERNAL=true
export NESTGATE_API_HOST=0.0.0.0
```

### Service Discovery Failing
```bash
# Increase timeout:
export NESTGATE_DISCOVERY_TIMEOUT_MS=10000

# Adjust port range:
export NESTGATE_DISCOVERY_PORT_START=3000
export NESTGATE_DISCOVERY_PORT_END=4000
```

---

## ✅ VERIFICATION CHECKLIST

Before deploying to production:
- [ ] All secrets loaded from secure vault
- [ ] `NESTGATE_ENV=production` set
- [ ] TLS enabled if exposing externally
- [ ] Log level set to `warn` or `error`
- [ ] Storage path has proper permissions
- [ ] Firewall rules configured for exposed ports
- [ ] Health check endpoint accessible
- [ ] Metrics endpoint accessible for monitoring

---

## 📚 RELATED DOCUMENTATION

- `HARDCODING_ELIMINATION_PLAN.md` - Hardcoding elimination strategy
- `DEPLOYMENT_GUIDE.md` - Production deployment guide
- `SECURITY.md` - Security best practices
- `config/` - Example configuration files

---

## 🎯 SOVEREIGNTY COMPLIANCE

This configuration system ensures **100% sovereignty compliance**:
- ✅ No vendor lock-in (all values configurable)
- ✅ No hardcoded endpoints (environment-driven)
- ✅ No forced defaults (everything overridable)
- ✅ Full user control (transparent configuration)
- ✅ Privacy-first (localhost default)

**Status**: ✅ **SOVEREIGNTY COMPLIANT**

---

*Last Updated*: November 2, 2025  
*Hardcoding Elimination*: In Progress  
*Configuration Status*: Production Ready

