# Troubleshooting Guide

**Quick solutions to common NestGate issues**

---

## Common Issues

### **Issue: Port Already in Use**

**Symptom**:
```
Error: Address already in use (os error 98)
Failed to bind to 127.0.0.1:8080
```

**Solutions**:

```bash
# Option 1: Find and kill process using port
sudo lsof -ti:8080 | xargs kill -9

# Option 2: Use different port
export NESTGATE_PORT=8090
./nestgate

# Option 3: Check what's using the port
sudo netstat -tlnp | grep :8080
```

---

### **Issue: Permission Denied (Socket)**

**Symptom**:
```
Error: Permission denied (os error 13)
Failed to create Unix socket at /run/user/1000/nestgate/
```

**Solutions**:

```bash
# Option 1: Use user-writable directory
export NESTGATE_SOCKET_DIR=$HOME/.nestgate/sockets
mkdir -p $NESTGATE_SOCKET_DIR
./nestgate

# Option 2: Fix permissions
sudo chmod 755 /run/user/$(id -u)
mkdir -p /run/user/$(id -u)/nestgate

# Option 3: Check XDG_RUNTIME_DIR
echo $XDG_RUNTIME_DIR
# Should be /run/user/1000 or similar
```

---

### **Issue: ZFS Not Found**

**Symptom**:
```
Error: ZFS binary not found at /usr/sbin/zfs
ZFS features disabled
```

**Solutions**:

```bash
# Option 1: Install ZFS
sudo apt install zfsutils-linux  # Ubuntu/Debian
sudo dnf install zfs              # Fedora
brew install openzfs              # macOS

# Option 2: Specify custom ZFS path
export NESTGATE_ZFS_BINARY_PATH=/usr/local/sbin/zfs
export NESTGATE_ZPOOL_BINARY_PATH=/usr/local/sbin/zpool
./nestgate

# Option 3: Disable ZFS features
export NESTGATE_ZFS_ENABLED=false
./nestgate
```

---

### **Issue: Dataset Not Found**

**Symptom**:
```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "Dataset 'my-data' not found"
  }
}
```

**Solutions**:

```bash
# Check existing datasets
curl http://localhost:8080/api/datasets

# Create dataset if missing
curl -X POST http://localhost:8080/api/datasets \
  -H "Content-Type: application/json" \
  -d '{"name":"my-data"}'

# Check storage path
ls -la ~/.local/share/nestgate/datasets/
```

---

### **Issue: Disk Full / Storage Quota Exceeded**

**Symptom**:
```json
{
  "error": {
    "code": "INSUFFICIENT_STORAGE",
    "message": "Disk space exhausted"
  }
}
```

**Solutions**:

```bash
# Check disk usage
df -h ~/.local/share/nestgate

# Clean old objects
curl -X DELETE http://localhost:8080/api/datasets/old-dataset

# Increase quota (if using ZFS)
sudo zfs set quota=100G tank/nestgate

# Change storage location
export NESTGATE_DATA_DIR=/mnt/large-disk/nestgate
./nestgate
```

---

## Connection Issues

### **Issue: Cannot Connect to NestGate**

**Diagnosis**:

```bash
# Check if NestGate is running
ps aux | grep nestgate

# Check port binding
sudo netstat -tlnp | grep :8080

# Test connection
curl -v http://localhost:8080/health

# Check firewall
sudo ufw status
sudo iptables -L -n | grep 8080
```

**Solutions**:

```bash
# Ensure NestGate is running
./target/release/nestgate &

# Check bind address (might be 127.0.0.1 only)
export NESTGATE_HOST=0.0.0.0  # Listen on all interfaces
./nestgate

# Open firewall
sudo ufw allow 8080/tcp
```

---

### **Issue: Discovery Not Finding Services**

**Symptom**:
```json
{
  "error": {
    "code": "SERVICE_UNAVAILABLE",
    "message": "No services found with capability 'security'"
  }
}
```

**Solutions**:

```bash
# Check discovery is enabled
curl http://localhost:8080/api/config | jq '.discovery'

# Enable discovery
export NESTGATE_DISCOVERY_ENABLED=true
./nestgate

# Check service registry
curl http://localhost:8080/api/services

# Manual service announcement (if needed)
curl -X POST http://localhost:8080/api/services/announce \
  -H "Content-Type: application/json" \
  -d '{
    "name":"my-service",
    "capabilities":["custom"],
    "endpoint":"http://localhost:9000"
  }'
```

---

## Build/Compilation Issues

### **Issue: Compilation Fails**

**Symptom**:
```
error: could not compile `nestgate-core`
```

**Solutions**:

```bash
# Update Rust
rustup update

# Clean build artifacts
cargo clean
cargo build

# Check Rust version (need 1.75+)
rustc --version

# Update dependencies
cargo update

# Check for incompatible features
cargo build --no-default-features
```

---

### **Issue: Tests Failing**

**Symptom**:
```
test services::storage::test_zfs_integration ... FAILED
```

**Solutions**:

```bash
# Run single test with output
cargo test test_zfs_integration -- --nocapture

# Skip ZFS tests if ZFS not available
cargo test --lib -- --skip zfs

# Check test environment
env | grep NESTGATE

# Reset test environment
unset $(env | grep NESTGATE | cut -d= -f1)
cargo test
```

---

## Performance Issues

### **Issue: Slow Response Times**

**Diagnosis**:

```bash
# Measure response time
time curl http://localhost:8080/api/datasets

# Check resource usage
top -p $(pgrep nestgate)

# Check I/O wait
iostat -x 1 10
```

**Solutions**:

```bash
# Increase connection pool
export NESTGATE_MAX_CONNECTIONS=2000
./nestgate

# Enable caching
export NESTGATE_CACHE_ENABLED=true
export NESTGATE_CACHE_SIZE_MB=1024
./nestgate

# Use release build (not debug)
cargo build --release
./target/release/nestgate

# Increase ZFS ARC cache
sudo zfs set primarycache=all tank
```

---

### **Issue: High Memory Usage**

**Symptom**:
```
nestgate process using 2GB+ RAM
```

**Solutions**:

```bash
# Reduce cache size
export NESTGATE_CACHE_SIZE_MB=256
./nestgate

# Limit concurrent operations
export NESTGATE_MAX_CONCURRENT_OPERATIONS=100
./nestgate

# Disable detailed metrics
export NESTGATE_DETAILED_METRICS=false
./nestgate

# Check for memory leaks
valgrind --leak-check=full ./nestgate
```

---

## Discovery Issues

### **Issue: mDNS Not Working**

**Solutions**:

```bash
# Install Avahi (Linux)
sudo apt install avahi-daemon
sudo systemctl start avahi-daemon

# Check mDNS
avahi-browse -a

# Use Consul instead
export CONSUL_HTTP_ADDR=http://localhost:8500
./nestgate

# Or disable discovery
export NESTGATE_DISCOVERY_ENABLED=false
./nestgate
```

---

## Storage Issues

### **Issue: Objects Not Persisting**

**Diagnosis**:

```bash
# Check storage path exists
ls -la $(echo $NESTGATE_DATA_DIR)

# Check permissions
ls -ld ~/.local/share/nestgate

# Check disk space
df -h ~/.local/share/nestgate
```

**Solutions**:

```bash
# Create storage directory
mkdir -p ~/.local/share/nestgate/datasets

# Fix permissions
chmod 755 ~/.local/share/nestgate

# Change storage location
export NESTGATE_DATA_DIR=/tmp/nestgate-storage
mkdir -p $NESTGATE_DATA_DIR
./nestgate
```

---

## Debugging Tips

### **Enable Debug Logging**:

```bash
# Maximum verbosity
export RUST_LOG=nestgate=trace

# Specific modules
export RUST_LOG=nestgate::storage=debug,nestgate::discovery=trace

# With timestamps
export RUST_LOG_FORMAT=full
./nestgate 2>&1 | tee nestgate.log
```

### **Inspect Traffic**:

```bash
# HTTP traffic
tcpdump -i lo port 8080 -A

# Unix socket traffic (strace)
strace -e trace=connect,sendto,recvfrom -p $(pgrep nestgate)
```

### **Check Configuration**:

```bash
# Dump current config
curl http://localhost:8080/api/config | jq

# Verify environment
env | grep NESTGATE | sort

# Check defaults
./nestgate --show-config  # If implemented
```

---

## Getting Help

### **Before Asking for Help**:

1. Check this troubleshooting guide
2. Review relevant documentation
3. Check logs for error messages
4. Try with default configuration
5. Search existing GitHub issues

### **When Creating an Issue**:

Include:
- NestGate version (`./nestgate --version`)
- OS and version (`uname -a`)
- Configuration (sanitized, no secrets!)
- Full error message
- Steps to reproduce
- Relevant logs

### **Resources**:

- **GitHub Issues**: https://github.com/ecoPrimals/nestGate/issues
- **Documentation**: `docs/` directory
- **Examples**: `examples/` directory
- **Tests**: `tests/` directory (show expected behavior)

---

## Quick Diagnostics Checklist

```bash
# Run this diagnostic script
#!/bin/bash

echo "=== NestGate Diagnostics ==="

# 1. Check if running
echo "Process status:"
pgrep -f nestgate && echo "OK: Running" || echo "FAIL: Not running"

# 2. Check ports
echo -e "\nPort binding:"
sudo netstat -tlnp | grep :8080 || echo "FAIL: Port 8080 not bound"

# 3. Check socket
echo -e "\nUnix socket:"
ls /run/user/$(id -u)/nestgate/*.sock 2>/dev/null && echo "OK: Socket exists" || echo "FAIL: No socket"

# 4. Health check
echo -e "\nHealth status:"
curl -s http://localhost:8080/health | jq -r '.status' || echo "FAIL: Cannot connect"

# 5. Storage path
echo -e "\nStorage path:"
ls -ld ~/.local/share/nestgate 2>/dev/null && echo "OK: Storage exists" || echo "FAIL: No storage"

# 6. ZFS status
echo -e "\nZFS availability:"
which zfs && echo "OK: ZFS installed" || echo "Warning: ZFS not found"

# 7. Environment
echo -e "\nEnvironment variables:"
env | grep NESTGATE | wc -l
```

---

**NestGate Troubleshooting** · Quick Fixes · Production Support
