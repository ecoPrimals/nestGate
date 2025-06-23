# NestGate Testing Guide

This guide covers testing approaches for NestGate, including unit tests, integration tests, and live ZFS testing.

## 🎯 Quick Start

### Run All Tests with Real ZFS
```bash
# Complete test suite with real ZFS (recommended)
./scripts/test-with-zfs.sh

# Or run individual test suites
./scripts/test-with-zfs.sh unit        # Unit tests only
./scripts/test-with-zfs.sh integration # Integration tests only
./scripts/test-with-zfs.sh api         # API tests only
```

### Run Tests with Mock ZFS
```bash
# Use mock ZFS for CI/CD or environments without ZFS
USE_MOCK_ZFS=true ./scripts/test-with-zfs.sh

# Or run tests directly with cargo
USE_MOCK_ZFS=true cargo test --workspace
```

## 🏗️ Test Architecture

### Real ZFS vs Mock ZFS

**Real ZFS (Default - Recommended)**
- ✅ Tests actual ZFS functionality
- ✅ Catches real-world integration issues
- ✅ Validates performance characteristics
- ✅ Tests error handling with real ZFS errors
- ⚠️ Requires ZFS tools and kernel module
- ⚠️ Uses loop devices for safe testing

**Mock ZFS (Fallback)**
- ✅ Fast execution
- ✅ No system dependencies
- ✅ Suitable for CI/CD environments
- ❌ May miss real-world issues
- ❌ Limited error scenario testing

## 🔧 Test Environment Setup

### Automatic Setup (Recommended)
```bash
# Sets up complete ZFS test environment
./scripts/setup-test-zfs.sh setup

# Check status
./scripts/setup-test-zfs.sh status

# Clean up when done
./scripts/setup-test-zfs.sh cleanup
```

### Manual Setup
```bash
# Check ZFS availability
which zpool && which zfs

# Load ZFS module if needed
sudo modprobe zfs

# Create test pool manually
sudo zpool create nestpool mirror /dev/loop0 /dev/loop1
```

## 📊 Test Categories

### 1. Unit Tests
Tests individual components in isolation.

```bash
# Run all unit tests
cargo test --lib --workspace

# Run specific crate unit tests
cargo test --lib --package nestgate-zfs
cargo test --lib --package nestgate-api
```

**Coverage:**
- ✅ ZFS configuration validation
- ✅ Data structure serialization
- ✅ Error handling logic
- ✅ Storage tier management
- ✅ API request/response validation

### 2. Integration Tests
Tests component interactions with real or mock ZFS.

```bash
# With real ZFS (default)
cargo test --test integration_tests --workspace

# With mock ZFS
USE_MOCK_ZFS=true cargo test --test integration_tests --workspace
```

**Coverage:**
- ✅ Pool management operations
- ✅ Dataset CRUD operations
- ✅ Snapshot management
- ✅ Tier migration logic
- ✅ AI integration features
- ✅ Error recovery scenarios

### 3. API Tests
Tests HTTP API endpoints end-to-end.

```bash
# With real ZFS (default)
cargo test --test zfs_api_tests --package nestgate-api

# With mock ZFS
USE_MOCK_ZFS=true cargo test --test zfs_api_tests --package nestgate-api
```

**Coverage:**
- ✅ All 18 API endpoints
- ✅ Request/response validation
- ✅ Error status codes
- ✅ CORS functionality
- ✅ Authentication (when implemented)

### 4. Performance Tests
Benchmarks and performance validation.

```bash
# Run performance tests
cargo test --release --test performance_tests --workspace
```

**Coverage:**
- ✅ Tier performance characteristics
- ✅ API response times
- ✅ Concurrent operation handling
- ✅ Memory usage patterns

## 🎯 Test Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `USE_MOCK_ZFS` | `false` | Use mock ZFS instead of real ZFS |
| `TEST_POOL_NAME` | `nestpool` | Name of test pool to use |
| `TEST_TIMEOUT_SECONDS` | `30` | Test timeout duration |

### Configuration Files

Tests use the same configuration system as the main application:

```rust
// Custom test configuration
let mut config = ZfsConfig::default();
config.use_real_zfs = true;  // Use real ZFS
config.default_pool = "testpool".to_string();
```

## 🚀 Continuous Integration

### GitHub Actions Example
```yaml
name: Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      # Install ZFS for real testing
      - name: Install ZFS
        run: |
          sudo apt update
          sudo apt install -y zfsutils-linux
          sudo modprobe zfs
      
      # Run tests with real ZFS
      - name: Run Tests
        run: ./scripts/test-with-zfs.sh
      
      # Fallback to mock tests if ZFS fails
      - name: Run Mock Tests
        if: failure()
        run: USE_MOCK_ZFS=true cargo test --workspace
```

### Docker Testing
```dockerfile
FROM ubuntu:22.04

# Install ZFS
RUN apt update && apt install -y zfsutils-linux

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Copy code and run tests
COPY . /app
WORKDIR /app
RUN ./scripts/test-with-zfs.sh
```

## 🔍 Test Data Management

### Test Pool Structure
```
nestpool/                 # Root pool (2GB mirrored)
├── hot/                  # Hot tier (lz4 compression, 128K records)
│   ├── test/
│   │   ├── random_hot.dat (10MB)
│   │   └── info_hot.txt
│   └── @initial          # Initial snapshot
├── warm/                 # Warm tier (zstd compression, 1M records)
│   ├── test/
│   │   ├── random_warm.dat (10MB)
│   │   └── info_warm.txt
│   └── @initial          # Initial snapshot
└── cold/                 # Cold tier (gzip-9 compression, 1M records)
    ├── test/
    │   ├── random_cold.dat (10MB)
    │   └── info_cold.txt
    └── @initial           # Initial snapshot
```

### Test Data Cleanup
```bash
# Automatic cleanup after tests
./scripts/test-with-zfs.sh cleanup

# Manual cleanup
sudo zpool destroy nestpool
sudo losetup -D  # Detach all loop devices
rm -rf /tmp/nestgate-zfs-loops
```

## 🐛 Debugging Tests

### Verbose Test Output
```bash
# Run tests with detailed output
cargo test --workspace --verbose -- --nocapture

# Run specific test with tracing
RUST_LOG=debug cargo test test_pool_creation --workspace -- --nocapture
```

### ZFS Debugging
```bash
# Check ZFS status during tests
zpool status nestpool
zfs list -t all nestpool

# Monitor ZFS events
sudo tail -f /var/log/syslog | grep zfs
```

### Common Issues

**ZFS Module Not Loaded**
```bash
sudo modprobe zfs
```

**Permission Issues**
```bash
# Ensure user can use sudo for ZFS commands
sudo usermod -a -G sudo $USER
```

**Loop Device Issues**
```bash
# Clean up stuck loop devices
sudo losetup -D
sudo losetup -l
```

## 📈 Test Metrics

### Success Criteria

| Test Suite | Target Pass Rate | Current Status |
|------------|------------------|----------------|
| Unit Tests | 100% | ✅ 16/16 (100%) |
| Integration Tests | 90% | 🔄 12/23 (52%) → Target: 90% |
| API Tests | 95% | ✅ Ready for improvement |
| Performance Tests | 80% | 🔄 In development |

### Performance Benchmarks

| Operation | Target | Hot Tier | Warm Tier | Cold Tier |
|-----------|--------|----------|-----------|-----------|
| Read Latency | < 10ms | < 2ms | < 5ms | < 10ms |
| Write Latency | < 20ms | < 5ms | < 15ms | < 20ms |
| Throughput | > 100MB/s | > 500MB/s | > 200MB/s | > 100MB/s |

## 🎉 Next Steps

1. **Run the complete test suite**: `./scripts/test-with-zfs.sh`
2. **Address integration test failures**: Focus on the 11 failing integration tests
3. **Add performance benchmarks**: Implement comprehensive performance testing
4. **Set up CI/CD**: Configure automated testing with real ZFS
5. **Monitor test coverage**: Aim for 90%+ code coverage

---

**Ready to test?** Start with: `./scripts/test-with-zfs.sh setup && ./scripts/test-with-zfs.sh` 