# Environment Variables Reference

**Last Updated**: June 28, 2026  
**Status**: Comprehensive reference for all NestGate environment variables  
**Default mode**: HTTP (`nestgate server`); use `--socket-only` for UDS-only operation

---

## Quick Reference

### **Network Configuration**

| Variable | Default | Description |
|----------|---------|-------------|
| `NESTGATE_API_PORT` | `8080` | Main API server port |
| `NESTGATE_BIND_ADDRESS` | `127.0.0.1` | Server bind address |
| `NESTGATE_HOST` | `127.0.0.1` | Server hostname |
| `NESTGATE_ADMIN_PORT` | `8081` | Admin/WebSocket port |
| `NESTGATE_HEALTH_PORT` | `8082` | Health check endpoint port |
| `NESTGATE_DEV_PORT` | `3000` | Development server port |
| `NESTGATE_METRICS_PORT` | `9090` | Prometheus metrics port |

### **Socket Configuration**

| Variable | Default | Description |
|----------|---------|-------------|
| `NESTGATE_SOCKET` | - | Explicit Unix socket path (highest priority) |
| `BIOMEOS_SOCKET_DIR` | `/run/user/{uid}/biomeos` | biomeOS shared socket directory |
| `NESTGATE_FAMILY_ID` | `default` | Family identifier |
| `NESTGATE_NODE_ID` | `default` | Node identifier for multi-instance |

### **Storage Configuration**

| Variable | Default | Description |
|----------|---------|-------------|
| `NESTGATE_STORAGE_PATH` | (XDG) | Storage root directory |
| `XDG_DATA_HOME` | `~/.local/share` | XDG data directory (if set) |
| `NESTGATE_STORAGE_BACKEND` | `filesystem` | Storage backend type |

### **Database Configuration** (Optional external — not required for core operation)

| Variable | Default | Description |
|----------|---------|-------------|
| `NESTGATE_DB_HOST` | `localhost` | PostgreSQL host (optional) |
| `NESTGATE_DB_PORT` | `5432` | PostgreSQL port (optional) |
| `NESTGATE_REDIS_HOST` | `localhost` | Redis host (optional) |
| `NESTGATE_REDIS_PORT` | `6379` | Redis port (optional) |

### **Security Configuration**

| Variable | Default | Description |
|----------|---------|-------------|
| `NESTGATE_JWT_SECRET` | - | JWT signing secret (REQUIRED in production) |
| `NESTGATE_TLS_CERT` | - | TLS certificate path |
| `NESTGATE_TLS_KEY` | - | TLS private key path |

### **Service Discovery**

Discovery uses capability-based IPC (JSON-RPC `primal.announce` / `capability.call`).
External backends (mDNS, Consul) were removed in the April 2026 sovereignty cleanup.

---

## Usage Examples

### **Basic Configuration**

```bash
# Development environment (HTTP default)
export NESTGATE_API_PORT=3000
export NESTGATE_BIND_ADDRESS=0.0.0.0

nestgate server
```

### **Socket-Only Mode** (NUCLEUS Integration)

```bash
# biomeOS standard — UDS only, no HTTP listener
export BIOMEOS_SOCKET_DIR=/run/user/$(id -u)/biomeos
export NESTGATE_FAMILY_ID=nat0

nestgate server --socket-only
```

### **Production Configuration**

```bash
# Production with HTTP + UDS
export NESTGATE_API_PORT=8080
export NESTGATE_BIND_ADDRESS=0.0.0.0
export NESTGATE_STORAGE_PATH=/var/lib/nestgate

nestgate server
```

### **Container Configuration**

```dockerfile
# Dockerfile
ENV NESTGATE_API_PORT=8080
ENV NESTGATE_BIND_ADDRESS=0.0.0.0
ENV NESTGATE_STORAGE_PATH=/data/nestgate

# Mount storage
VOLUME ["/data/nestgate"]
```

### **Multi-Instance Setup**

```bash
# Instance 1
export NESTGATE_NODE_ID=instance1
export NESTGATE_API_PORT=8080
nestgate daemon &

# Instance 2
export NESTGATE_NODE_ID=instance2
export NESTGATE_API_PORT=8081
nestgate daemon &
```

---

## Advanced Configuration

### **XDG Base Directory Support**

NestGate follows XDG Base Directory Specification:

```bash
# Data directory (storage, databases)
export XDG_DATA_HOME=~/.local/share
# NestGate uses: $XDG_DATA_HOME/nestgate/

# Runtime directory (sockets, PID files)
export XDG_RUNTIME_DIR=/run/user/$(id -u)
# NestGate uses: $XDG_RUNTIME_DIR/biomeos/nestgate.sock

# Config directory (configuration files)
export XDG_CONFIG_HOME=~/.config
# NestGate uses: $XDG_CONFIG_HOME/nestgate/
```

### **Fallback Hierarchy**

NestGate uses a multi-tier fallback system:

**Socket Path Example**:
```
1. NESTGATE_SOCKET (explicit path)
2. BIOMEOS_SOCKET_DIR/nestgate.sock (biomeOS standard)
3. /run/user/{uid}/biomeos/nestgate.sock (XDG + biomeOS)
4. /tmp/nestgate-{family}-{node}.sock (fallback)
```

**Storage Path Example**:
```
1. NESTGATE_STORAGE_PATH (explicit path)
2. XDG_DATA_HOME/nestgate/ (XDG standard)
3. HOME/.local/share/nestgate/ (user default)
4. /var/lib/nestgate/ (system default - requires permissions)
```

---

## Platform-Specific Configuration

### **Linux** (Recommended)

```bash
# Use XDG standards
export XDG_RUNTIME_DIR=/run/user/$(id -u)
export XDG_DATA_HOME=~/.local/share
export XDG_CONFIG_HOME=~/.config

# Or explicit paths
export NESTGATE_STORAGE_PATH=/var/lib/nestgate
export NESTGATE_SOCKET=/run/user/$(id -u)/biomeos/nestgate.sock
```

### **macOS**

```bash
# macOS doesn't have /run/user, use HOME
export NESTGATE_STORAGE_PATH=~/Library/Application\ Support/nestgate
export NESTGATE_SOCKET=~/Library/Application\ Support/nestgate/nestgate.sock
```

### **Docker/Kubernetes**

```yaml
# ConfigMap
apiVersion: v1
kind: ConfigMap
metadata:
  name: nestgate-config
data:
  NESTGATE_API_PORT: "8080"
  NESTGATE_BIND_ADDRESS: "0.0.0.0"
  NESTGATE_STORAGE_BACKEND: "filesystem"

# Secret
apiVersion: v1
kind: Secret
metadata:
  name: nestgate-secrets
type: Opaque
stringData:
  NESTGATE_JWT_SECRET: "your-secret-here"
```

---

## Configuration Priority

NestGate uses this priority order for all configuration:

1. **Environment Variables** (highest priority)
2. **Configuration Files** (`~/.config/nestgate/config.toml`)
3. **CLI Arguments** (for specific commands)
4. **Compiled Defaults** (lowest priority)

---

## Security Best Practices

### **JWT Secret**

```bash
# NEVER use default or weak secrets
export NESTGATE_JWT_SECRET="secret123"  # BAD!

# Generate secure random secret
export NESTGATE_JWT_SECRET="$(openssl rand -base64 48)"

# Use secret management
export NESTGATE_JWT_SECRET="$(kubectl get secret nestgate-jwt -o jsonpath='{.data.secret}' | base64 -d)"
```

### **TLS Configuration**

```bash
# Production TLS
export NESTGATE_TLS_CERT=/etc/nestgate/tls/cert.pem
export NESTGATE_TLS_KEY=/etc/nestgate/tls/key.pem

# Let's Encrypt
export NESTGATE_TLS_CERT=/etc/letsencrypt/live/nestgate.example.com/fullchain.pem
export NESTGATE_TLS_KEY=/etc/letsencrypt/live/nestgate.example.com/privkey.pem
```

---

## Testing Configuration

### **Development**

```bash
# Local development with test database
export NESTGATE_DEV=true
export NESTGATE_API_PORT=3000
export NESTGATE_DB_HOST=localhost
export NESTGATE_LOG_LEVEL=debug
```

### **Testing**

```bash
# Use test configuration
export NESTGATE_ENV=test
export NESTGATE_DB_NAME=nestgate_test
export NESTGATE_STORAGE_PATH=/tmp/nestgate-test
```

### **CI/CD**

```bash
# CI environment
export NESTGATE_ENV=ci
export NESTGATE_DB_HOST=postgres-ci
export NESTGATE_REDIS_HOST=redis-ci
export NESTGATE_STORAGE_PATH=$GITHUB_WORKSPACE/data
```

---

## Related Documentation

- [STATUS.md](../../STATUS.md) — current build and feature status
- [QUICK_START.md](../../QUICK_START.md) — getting started guide

Historical socket-mode and deployment docs have been fossilized to `ecoPrimals/infra/fossilRecord/nestgate/`.

---

**Environment-Driven · Configurable · Production Ready**
