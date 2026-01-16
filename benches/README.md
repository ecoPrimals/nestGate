# NestGate Benchmarks

**Purpose**: Track performance improvements during DashMap migration and technical debt remediation.

---

## 📊 **Available Benchmarks**

### **1. DashMap Migration Benchmark**

**File**: `dashmap_migration_benchmark.rs`

**Purpose**: Measure performance improvements as we migrate from `Arc<RwLock<HashMap>>` to `DashMap`.

**Scenarios**:
- **Single-threaded**: Baseline performance (minimal difference expected)
- **Concurrent mixed workload**: 70% reads, 30% writes, 2-16 threads
- **High contention**: Multiple threads fighting for same keys (DashMap should excel!)

**Expected Results**:
- Single-threaded: Similar performance (both are fast)
- Concurrent: **10-25x improvement** with DashMap
- High contention: **25-50x improvement** with DashMap

**Run**:
```bash
cargo bench --bench dashmap_migration_benchmark
```

---

## 🎯 **Migration Progress Tracking**

### **Baseline** (43/406 files, 10.6%)

Files already using DashMap:
- Discovery system
- Authentication
- Metrics
- Configuration
- Encryption
- Resilience
- Health checks

**Expected throughput**: ~40K ops/sec (estimated)

### **Target** (53/406 files, 13%)

Next 10 files for migration:
- Service registry
- Connection pools
- Cache managers
- Session stores
- Rate limiters

**Expected throughput**: 50-100K ops/sec

---

## 📈 **How to Use**

### **1. Run Baseline**

Before making changes:
```bash
cargo bench --bench dashmap_migration_benchmark > baseline.txt
```

### **2. Migrate Files**

Apply DashMap migration to target files.

### **3. Run Comparison**

After migration:
```bash
cargo bench --bench dashmap_migration_benchmark > after_migration.txt
```

### **4. Analyze Results**

Compare results:
```bash
# Look for improvements in:
# - Throughput (ops/sec) - higher is better
# - Time per operation - lower is better
# - Scalability across thread counts

diff baseline.txt after_migration.txt
```

---

## 📝 **Interpreting Results**

### **Good Results** ✅

- **Concurrent mixed workload**: 10-25x improvement
- **High contention**: 25-50x improvement
- **Linear scalability**: Performance scales with threads

### **Expected Results** 📊

- **Single-threaded**: 0-10% improvement (overhead similar)
- **2 threads**: 2-5x improvement
- **4 threads**: 5-10x improvement  
- **8 threads**: 10-20x improvement
- **16 threads**: 15-30x improvement

### **Warning Signs** ⚠️

- Performance regression in any scenario
- Non-linear scaling
- High variance in measurements

---

## 🔬 **Advanced Usage**

### **Profile a Specific Scenario**

```bash
cargo bench --bench dashmap_migration_benchmark -- "concurrent_mixed_workload/modern_dashmap/8"
```

### **Generate Flame Graphs**

```bash
cargo bench --bench dashmap_migration_benchmark --profile-time=5
```

### **Export Results**

```bash
cargo bench --bench dashmap_migration_benchmark -- --save-baseline migration_v1
```

Compare later:
```bash
cargo bench --bench dashmap_migration_benchmark -- --baseline migration_v1
```

---

## 📁 **Benchmark Results Location**

Results stored in:
```
target/criterion/
├── single_thread_insert/
├── single_thread_read/
├── concurrent_mixed_workload/
└── high_contention/
```

HTML reports at:
```
target/criterion/report/index.html
```

---

## 🎯 **Success Criteria**

For DashMap migration to be successful:

1. ✅ **No regressions**: Single-threaded performance unchanged
2. ✅ **Concurrent gains**: 10-25x improvement in concurrent scenarios
3. ✅ **Contention handling**: 25-50x improvement under high contention
4. ✅ **Scalability**: Near-linear scaling with thread count
5. ✅ **Consistency**: Low variance across runs

---

## 💡 **Tips**

1. **Run multiple times**: Benchmarks can have variance
2. **Close other apps**: Reduce system noise
3. **Use release mode**: Always benchmark with `--release`
4. **Track trends**: Keep historical results
5. **Isolate changes**: Only migrate a few files at a time

---

**Created**: January 16, 2026  
**Updated**: January 16, 2026  
**Status**: Ready for DashMap migration tracking! 🚀
