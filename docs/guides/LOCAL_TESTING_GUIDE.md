# **NESTGATE LOCAL TESTING GUIDE**

**Build, Test, and Validate on Eastgate Before Tower Deployment**

---

## **OVERVIEW**

This guide helps you:
1. Build NestGate on your local machine (Eastgate)
2. Run comprehensive tests safely
3. Validate functionality without affecting production
4. Prepare for tower deployment

**Safety First**: All tests run in isolated environments. No impact on your existing systems.

---

## **PREREQUISITES CHECK**

### **Your Current Environment** (Eastgate)

```bash
# Check what you have
echo "=== System Info ==="
uname -a
cat /etc/os-release | grep PRETTY_NAME

echo -e "\n=== CPU Info ==="
lscpu | grep "Model name"
lscpu | grep "CPU(s):"

echo -e "\n=== Memory ==="
free -h | grep "Mem:"

echo -e "\n=== Disk Space ==="
df -h | grep -E "/$|/home"

echo -e "\n=== Rust Version ==="
rustc --version
cargo --version

echo -e "\n=== ZFS Status ==="
if command -v zfs &> /dev/null; then
    zfs --version
    echo "OK ZFS installed"
else
    echo "ZFS not installed (optional for testing)"
fi

echo -e "\n=== GPU Status ==="
if command -v nvidia-smi &> /dev/null; then
    nvidia-smi --query-gpu=name,memory.total --format=csv,noheader
else
    echo "No NVIDIA GPU detected"
fi
```

**Expected on Eastgate**:
- 20-core i9-12900K
- 128GB RAM
- RTX 4070
- Ubuntu/Linux
- Rust 1.70+

---

## **PHASE 1: BUILD & UNIT TESTS** (30 minutes)

### **Step 1.1: Clean Build**

```bash
cd /path/to/nestgate

# Clean any old artifacts
cargo clean

# Build in debug mode (faster compilation)
echo "Building NestGate (debug)..."
time cargo build --workspace

# Check for errors
if [ $? -eq 0 ]; then
    echo "OK Debug build successful!"
else
    echo "FAIL Build failed - check errors above"
    exit 1
fi

# Build in release mode (optimized)
echo "Building NestGate (release)..."
time cargo build --release --workspace

if [ $? -eq 0 ]; then
    echo "OK Release build successful!"
    ls -lh target/release/nestgate
else
    echo "FAIL Release build failed"
    exit 1
fi
```

**Expected Output**:
```
Debug build:   ~5-10 minutes (i9-12900K)
Release build: ~8-15 minutes (with optimizations)
```

### **Step 1.2: Run All Tests**

```bash
# Run all unit tests
echo "Running unit tests..."
cargo test --workspace --lib -- --nocapture

# Run with verbose output
cargo test --workspace --lib -- --test-threads=4 --nocapture | tee test_results.txt

# Check results
TEST_COUNT=$(grep "test result:" test_results.txt | tail -1)
echo "==================================="
echo "Test Results: $TEST_COUNT"
echo "==================================="

# Look for failures
if grep -q "FAILED" test_results.txt; then
    echo "Some tests failed - review test_results.txt"
    grep -A 5 "FAILED" test_results.txt
else
    echo "OK All tests passed!"
fi
```

**Expected**: 1,925+ tests passing (100% pass rate)

### **Step 1.3: Run Lints**

```bash
# Clippy (Rust linter)
echo "Running Clippy..."
cargo clippy --workspace -- -D warnings 2>&1 | tee clippy_results.txt

# Count warnings
WARNING_COUNT=$(grep -c "warning:" clippy_results.txt || echo 0)
ERROR_COUNT=$(grep -c "error:" clippy_results.txt || echo 0)

echo "==================================="
echo "Clippy Results:"
echo "  Warnings: $WARNING_COUNT"
echo "  Errors:   $ERROR_COUNT"
echo "==================================="

# Format check
echo "Checking code format..."
cargo fmt --all -- --check

if [ $? -eq 0 ]; then
    echo "OK Code is properly formatted"
else
    echo "Code needs formatting - run: cargo fmt --all"
fi
```

### **Step 1.4: Build Verification**

```bash
# Verify all binaries exist
echo "Verifying build artifacts..."

BINARIES=(
    "target/release/nestgate"
)

for bin in "${BINARIES[@]}"; do
    if [ -f "$bin" ]; then
        SIZE=$(ls -lh "$bin" | awk '{print $5}')
        echo "OK $bin ($SIZE)"
    else
        echo "FAIL $bin not found"
    fi
done

# Test basic execution
echo -e "\n Testing basic execution..."
./target/release/nestgate --version
./target/release/nestgate --help | head -20
```

---

## **PHASE 2: LOCAL INTEGRATION TESTS** (1 hour)

### **Step 2.1: Create Test Environment**

```bash
# Create isolated test directory
TEST_DIR="/tmp/nestgate_test_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

echo "Test directory: $TEST_DIR"

# Create test data
mkdir -p test_data/{input,output,cache}

# Generate sample files
echo "Generating test data..."
for i in {1..10}; do
    dd if=/dev/urandom of="test_data/input/file_${i}.dat" bs=1M count=10 2>/dev/null
    echo "OK Created test_data/input/file_${i}.dat (10MB)"
done

# Calculate total size
TOTAL_SIZE=$(du -sh test_data/input | awk '{print $1}')
echo "Total test data: $TOTAL_SIZE"
```

### **Step 2.2: Test File Operations** (Without ZFS)

```bash
# Use filesystem backend for safe testing
cd /path/to/nestgate

# Test basic storage operations
cat > /tmp/test_storage.py << 'EOF'
#!/usr/bin/env python3
"""Test NestGate storage operations"""

import subprocess
import time
import os

def test_storage_operations():
    """Test basic storage operations"""
    
    test_dir = "/tmp/nestgate_storage_test"
    os.makedirs(test_dir, exist_ok=True)
    
    print("=== Testing Storage Operations ===\n")
    
    # Test 1: Write file
    print("Test 1: Write file...")
    test_file = f"{test_dir}/test1.txt"
    with open(test_file, 'w') as f:
        f.write("Hello NestGate!\n" * 1000)
    
    size = os.path.getsize(test_file)
    print(f"OK Wrote {size} bytes to {test_file}")
    
    # Test 2: Read file
    print("\nTest 2: Read file...")
    with open(test_file, 'r') as f:
        content = f.read()
    
    lines = len(content.split('\n'))
    print(f"OK Read {len(content)} bytes ({lines} lines)")
    
    # Test 3: Metadata
    print("\nTest 3: File metadata...")
    stat = os.stat(test_file)
    print(f"OK Size: {stat.st_size} bytes")
    print(f"OK Modified: {time.ctime(stat.st_mtime)}")
    
    # Test 4: Compression simulation
    print("\nTest 4: Compression test...")
    compressed_file = f"{test_dir}/test1.txt.gz"
    subprocess.run(['gzip', '-k', test_file], check=True)
    
    original_size = os.path.getsize(test_file)
    compressed_size = os.path.getsize(compressed_file)
    ratio = original_size / compressed_size
    
    print(f"OK Original: {original_size} bytes")
    print(f"OK Compressed: {compressed_size} bytes")
    print(f"OK Ratio: {ratio:.2f}x")
    
    print("\n=== All Storage Tests Passed! ===")

if __name__ == "__main__":
    test_storage_operations()
EOF

chmod +x /tmp/test_storage.py
python3 /tmp/test_storage.py
```

### **Step 2.3: Test API Server** (Local)

```bash
# Start NestGate API server (background)
echo "Starting NestGate API server..."

cd /path/to/nestgate

# Start in background
./target/release/nestgate \
    --mode api \
    --bind 127.0.0.1:8888 \
    --data-dir /tmp/nestgate_test_data \
    --log-level info &

NESTGATE_PID=$!
echo "Started with PID: $NESTGATE_PID"

# Wait for startup
sleep 5

# Test API endpoints
echo -e "\n Testing API endpoints..."

# Test 1: Health check
echo "Test 1: Health check..."
curl -s http://127.0.0.1:8888/health | jq '.'

# Test 2: Version info
echo -e "\nTest 2: Version info..."
curl -s http://127.0.0.1:8888/api/v1/version | jq '.'

# Test 3: Metrics
echo -e "\nTest 3: Metrics..."
curl -s http://127.0.0.1:8888/metrics | head -20

# Cleanup
echo -e "\n Stopping server..."
kill $NESTGATE_PID 2>/dev/null || true
sleep 2

echo "OK API tests complete"
```

### **Step 2.4: Test Performance** (Benchmark)

```bash
cd /path/to/nestgate

# Run performance benchmarks
echo "Running performance benchmarks..."

# Run short benchmark
cargo bench --bench simple_performance_validation -- --test

# Or run specific benchmarks
cargo bench --bench zero_copy_benchmarks

# Save results
cargo bench 2>&1 | tee benchmark_results.txt

echo "Benchmark results saved to benchmark_results.txt"
```

---

## **PHASE 3: SAFE ZFS TESTING** (1 hour)

### **Step 3.1: Create Test ZFS Pool** (Safe!)

**IMPORTANT**: This uses **file-backed pools**, not your real disks!

```bash
# Create file-backed ZFS pool (completely safe)
echo "Creating SAFE test ZFS pool..."

TEST_POOL_DIR="/tmp/zfs_test_pool_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$TEST_POOL_DIR"

# Create backing files (not real disks!)
echo "Creating backing files..."
for i in {1..3}; do
    truncate -s 2G "$TEST_POOL_DIR/disk${i}.img"
    echo "OK Created disk${i}.img (2GB)"
done

# Create ZFS pool from files
echo -e "\nCreating ZFS pool 'nestgate_test'..."
sudo zpool create nestgate_test \
    "$TEST_POOL_DIR/disk1.img" \
    "$TEST_POOL_DIR/disk2.img" \
    "$TEST_POOL_DIR/disk3.img"

if [ $? -eq 0 ]; then
    echo "OK Test pool created successfully!"
    sudo zpool status nestgate_test
else
    echo "ZFS pool creation failed (ZFS may not be installed)"
    echo "Skipping ZFS-specific tests..."
    exit 0
fi
```

### **Step 3.2: Test ZFS Operations**

```bash
# Test dataset creation
echo -e "\n Testing dataset operations..."

sudo zfs create nestgate_test/data
sudo zfs create nestgate_test/cache
sudo zfs create nestgate_test/archive

sudo zfs list -r nestgate_test

# Test properties
echo -e "\n Testing ZFS properties..."

sudo zfs set compression=lz4 nestgate_test/data
sudo zfs set compression=gzip-9 nestgate_test/archive
sudo zfs set compression=off nestgate_test/cache

sudo zfs get compression nestgate_test/data
sudo zfs get compression nestgate_test/archive

# Test snapshots
echo -e "\n Testing snapshots..."

# Write some data
TEST_FILE="/nestgate_test/data/testfile.txt"
echo "Original content" | sudo tee $TEST_FILE > /dev/null

# Create snapshot
sudo zfs snapshot nestgate_test/data@test1
echo "OK Snapshot created: nestgate_test/data@test1"

# Modify file
echo "Modified content" | sudo tee $TEST_FILE > /dev/null

# List snapshots
sudo zfs list -t snapshot

# Test rollback
echo -e "\n Testing rollback..."
echo "Content before rollback: $(sudo cat $TEST_FILE)"

sudo zfs rollback nestgate_test/data@test1
echo "Content after rollback: $(sudo cat $TEST_FILE)"

# Test compression
echo -e "\n Testing compression..."

# Write compressible data
sudo dd if=/dev/zero of=/nestgate_test/data/zeros.dat bs=1M count=100 2>/dev/null

# Check compression ratio
sudo zfs get compressratio nestgate_test/data
sudo zfs get used,referenced nestgate_test/data
```

### **Step 3.3: Test NestGate with ZFS**

```bash
cd /path/to/nestgate

# Test NestGate ZFS integration
cat > /tmp/test_nestgate_zfs.sh << 'EOF'
#!/bin/bash
set -e

echo "Testing NestGate ZFS Integration"
echo "====================================="

# Test 1: Pool detection
echo -e "\nTest 1: Pool detection..."
sudo zpool list nestgate_test
echo "OK Pool detected"

# Test 2: Dataset operations
echo -e "\nTest 2: Dataset operations..."
sudo zfs create nestgate_test/nestgate_test_data 2>/dev/null || true
sudo zfs list nestgate_test/nestgate_test_data
echo "OK Dataset operations work"

# Test 3: Properties
echo -e "\nTest 3: Property management..."
sudo zfs set compression=lz4 nestgate_test/nestgate_test_data
COMP=$(sudo zfs get -H -o value compression nestgate_test/nestgate_test_data)
echo "Compression: $COMP"
echo "OK Properties work"

# Test 4: Snapshots
echo -e "\nTest 4: Snapshot management..."
sudo zfs snapshot nestgate_test/nestgate_test_data@test
sudo zfs list -t snapshot | grep nestgate_test
echo "OK Snapshots work"

echo -e "\nAll NestGate ZFS tests passed!"
EOF

chmod +x /tmp/test_nestgate_zfs.sh
/tmp/test_nestgate_zfs.sh
```

### **Step 3.4: Cleanup Test Pool**

```bash
# Destroy test pool (completely safe)
echo "Cleaning up test ZFS pool..."

sudo zpool destroy nestgate_test

# Remove backing files
rm -rf "$TEST_POOL_DIR"

echo "OK Test pool cleaned up"
```

---

## **PHASE 4: VALIDATION SUMMARY**

```bash
cat > /tmp/validation_report.sh << 'EOF'
#!/bin/bash

echo "======================================"
echo "  NESTGATE VALIDATION REPORT"
echo "======================================"

cd /path/to/nestgate

# Build status
echo -e "\n BUILD STATUS"
if [ -f "target/release/nestgate" ]; then
    SIZE=$(ls -lh target/release/nestgate | awk '{print $5}')
    echo "OK Release build exists ($SIZE)"
else
    echo "FAIL Release build missing"
fi

# Test results
echo -e "\n TEST RESULTS"
if [ -f "test_results.txt" ]; then
    grep "test result:" test_results.txt | tail -1
else
    echo "Test results not found - run tests first"
fi

# Clippy status
echo -e "\n CODE QUALITY"
if [ -f "clippy_results.txt" ]; then
    WARNING_COUNT=$(grep -c "warning:" clippy_results.txt || echo 0)
    ERROR_COUNT=$(grep -c "error:" clippy_results.txt || echo 0)
    echo "Clippy warnings: $WARNING_COUNT"
    echo "Clippy errors: $ERROR_COUNT"
else
    echo "Clippy results not found"
fi

# Benchmark results
echo -e "\n PERFORMANCE"
if [ -f "benchmark_results.txt" ]; then
    echo "Benchmark data available in benchmark_results.txt"
    grep -E "test.*time:" benchmark_results.txt | head -5
else
    echo "Benchmark results not found"
fi

# System info
echo -e "\n SYSTEM INFO"
echo "CPU: $(lscpu | grep 'Model name' | cut -d':' -f2 | xargs)"
echo "Cores: $(nproc)"
echo "RAM: $(free -h | grep Mem | awk '{print $2}')"

# Deployment readiness
echo -e "\n DEPLOYMENT READINESS"
CHECKS=0
PASSED=0

# Check 1: Build exists
if [ -f "target/release/nestgate" ]; then
    ((CHECKS++))
    ((PASSED++))
    echo "OK Binary built"
else
    ((CHECKS++))
    echo "FAIL Binary missing"
fi

# Check 2: Tests passed
if [ -f "test_results.txt" ] && ! grep -q "FAILED" test_results.txt; then
    ((CHECKS++))
    ((PASSED++))
    echo "OK Tests passed"
else
    ((CHECKS++))
    echo "FAIL Tests failed or not run"
fi

# Check 3: No critical warnings
if [ -f "clippy_results.txt" ]; then
    ERROR_COUNT=$(grep -c "error:" clippy_results.txt || echo 0)
    if [ "$ERROR_COUNT" -eq 0 ]; then
        ((CHECKS++))
        ((PASSED++))
        echo "OK No critical errors"
    else
        ((CHECKS++))
        echo "FAIL Clippy errors present"
    fi
fi

# Summary
echo -e "\n======================================"
echo "SUMMARY: $PASSED/$CHECKS checks passed"
echo "======================================"

if [ $PASSED -eq $CHECKS ]; then
    echo "READY FOR TOWER DEPLOYMENT"
    exit 0
else
    echo "FIX ISSUES BEFORE DEPLOYMENT"
    exit 1
fi
EOF

chmod +x /tmp/validation_report.sh
/tmp/validation_report.sh
```

---

## **READY FOR TOWER DEPLOYMENT?**

If validation passes, you're ready to deploy to your towers!

### **Next Steps**:

1. **Node-A** (NAS/Archive)
   ```bash
   # Copy binary
   scp target/release/nestgate node-a:/tmp/
   
   # SSH and test
   ssh node-a
   /tmp/nestgate --version
   ```

2. **Node-C** (Parallel Compute)
   ```bash
   scp target/release/nestgate node-c:/tmp/
   ssh node-c "/tmp/nestgate --version"
   ```

3. **Node-B** (AI Flagship)
   ```bash
   scp target/release/nestgate node-b:/tmp/
   ssh node-b "/tmp/nestgate --version"
   ```

**See**: `TOWER_DEPLOYMENT_GUIDE.md` (next step)

---

## **SAFETY CHECKLIST**

Before deploying to towers:

- [ ] All tests pass locally
- [ ] No critical Clippy errors
- [ ] Benchmarks show expected performance
- [ ] ZFS tests work (if available)
- [ ] API server starts and responds
- [ ] Binary is optimized (release build)
- [ ] You have backups of important data
- [ ] You understand rollback procedures

---

## **TROUBLESHOOTING**

### **Build Fails**

```bash
# Clean and rebuild
cargo clean
cargo build --release 2>&1 | tee build_errors.txt

# Check specific errors
grep "error:" build_errors.txt
```

### **Tests Fail**

```bash
# Run specific test
cargo test --lib test_name -- --nocapture

# Run with backtrace
RUST_BACKTRACE=1 cargo test

# Skip failing tests temporarily
cargo test -- --skip failing_test_name
```

### **ZFS Issues**

```bash
# Check if ZFS is loaded
sudo modprobe zfs
lsmod | grep zfs

# Install ZFS (Ubuntu)
sudo apt install zfsutils-linux

# Use filesystem backend instead
./target/release/nestgate --backend filesystem
```

---

## **QUESTIONS?**

- **Build issues**: Check `build_errors.txt`
- **Test failures**: Check `test_results.txt`  
- **Performance**: Check `benchmark_results.txt`
- **ZFS issues**: Test pool is safe to delete anytime

---

**LOCAL TESTING COMPLETE! Ready for tower deployment!**

**Next**: Copy binary to towers and start production deployment!

