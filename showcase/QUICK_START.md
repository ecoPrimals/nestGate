# 🚀 NestGate Showcase - Quick Start

**Get up and running in 5 minutes!**

---

## ⚡ Ultra Quick Start

```bash
cd /home/eastgate/Development/ecoPrimals/nestgate/showcase
./run_showcase.sh
```

That's it! The showcase will:
1. Check your system
2. Run all demonstrations
3. Show live metrics
4. Automatically cleanup

---

## 📋 Prerequisites Check

```bash
# Check if you have everything
rustc --version   # Should show 1.70+
df -h .           # Should show 10GB+ available
zfs version       # Optional (will use filesystem if not available)
```

---

## 🎯 Choose Your Demo

### Full Showcase (Recommended)
```bash
./run_showcase.sh
```

Duration: ~15-20 minutes  
Disk space: ~5-10GB  
Shows: All features

### Quick Demo (Fast)
```bash
./run_showcase.sh --demo zfs --max-space 2GB
```

Duration: ~5 minutes  
Disk space: ~2GB  
Shows: ZFS basics only

### Performance Only
```bash
./run_showcase.sh --demo performance
```

Duration: ~10 minutes  
Disk space: Minimal  
Shows: Performance benchmarks

### Dry Run (Preview)
```bash
./run_showcase.sh --dry-run
```

Duration: ~1 minute  
Disk space: None  
Shows: What would be executed

---

## 🎬 What You'll See

### 1. System Check ✓
```
╔════════════════════════════════════════════════════════════╗
║      🎬 NESTGATE LIVE SHOWCASE                            ║
╚════════════════════════════════════════════════════════════╝

Environment Check
────────────────────────────────────────────────────
▶ Checking Rust installation...
  ✓ Rust installed: 1.70.0
▶ Checking disk space...
  ✓ Available: 50GB (need: 20GB)
▶ Checking ZFS availability...
  ✓ ZFS available: 2.1.0
```

### 2. ZFS Demo 💾
```
Demo 1: ZFS Fundamentals
═══════════════════════════════════════════════════

▶ Creating test pool 'nestgate_demo'...
  ✓ Pool created: 10.0GB total
  ✓ Pool status: ONLINE

▶ Writing test data (100MB)...
  [████████████████████████] 100MB
  ✓ Write completed in 2.3s (43.5 MB/s)

▶ Creating snapshot 'nestgate_demo/data@demo1'...
  ✓ Snapshot created
  ✓ Space used: 0B (copy-on-write)

▶ Compression analysis...
  Original size:  100.0 MB
  Compressed:     45.2 MB
  Ratio:         2.21x
  ✓ Space saved: 54.8 MB
```

### 3. Performance Demo ⚡
```
Demo 2: Performance Showcase
═══════════════════════════════════════════════════

▶ Running zero-cost vs traditional comparison...

  Zero-Cost Implementation:
    Throughput:  125,450 ops/sec
    Latency p50: 0.42 ms
    Latency p95: 0.89 ms
    Memory:      45.2 MB

  Traditional Implementation:
    Throughput:  78,320 ops/sec
    Latency p50: 0.78 ms
    Latency p95: 1.45 ms
    Memory:      89.5 MB

  ✓ Performance gain: 60.2% faster
  ✓ Memory savings:  49.5% less
```

### 4. Live Dashboard 📊

The dashboard automatically opens at: **http://localhost:3000**

Shows:
- Real-time operations/sec
- Live latency metrics
- Memory usage graphs
- Activity log
- System resources

---

## 🔧 Common Options

### Limit Disk Usage
```bash
./run_showcase.sh --max-space 5GB
```

### Keep Results (Don't Cleanup)
```bash
./run_showcase.sh --no-cleanup
```

Then inspect:
```bash
ls outputs/logs/        # View logs
cat outputs/metrics/*   # See metrics
zpool status nestgate_demo  # Check ZFS pool
```

### Use Filesystem Backend (No ZFS)
```bash
./run_showcase.sh --backend filesystem
```

### Verbose Output
```bash
./run_showcase.sh --verbose
```

---

## 📁 Where Are My Results?

After running, find results in:

```
showcase/
├── outputs/
│   ├── logs/
│   │   └── showcase.log         # Execution log
│   ├── metrics/
│   │   └── performance.json     # Performance data
│   └── screenshots/
│       └── dashboard.png        # Dashboard capture
```

View logs:
```bash
cat outputs/logs/showcase.log
```

View metrics:
```bash
cat outputs/metrics/*.json | jq .
```

---

## 🧹 Cleanup

### Automatic Cleanup
By default, cleanup happens automatically when showcase ends.

### Manual Cleanup
```bash
./scripts/cleanup.sh
```

This removes:
- ZFS pools
- Mount points  
- Temporary files
- Test data

### Check What's Left
```bash
# Check ZFS resources
sudo zpool list | grep nestgate

# Check mount points
ls /tmp/nestgate_*

# Check processes
ps aux | grep nestgate
```

---

## ❓ Troubleshooting

### "ZFS module not loaded"
```bash
# Option 1: Load ZFS
sudo modprobe zfs

# Option 2: Use filesystem backend
./run_showcase.sh --backend filesystem
```

### "Insufficient disk space"
```bash
# Reduce demo size
./run_showcase.sh --max-space 5GB

# Or clean up first
./scripts/cleanup.sh --all
df -h .
```

### "Permission denied"
```bash
# Option 1: Run with sudo
sudo ./run_showcase.sh

# Option 2: Use filesystem backend
./run_showcase.sh --backend filesystem
```

### "Port already in use"
```bash
# Use different ports
./run_showcase.sh --api-port 8081 --metrics-port 3001
```

---

## 🎓 Next Steps

### Explore Individual Demos
```bash
cd demos/01_zfs_basics
./demo.sh

cd ../02_performance
./demo.sh
```

### Read Full Documentation
```bash
cat README.md          # Comprehensive guide
cat config.toml        # Configuration options
```

### Try the Playground
```bash
./scripts/playground.sh

# Interactive commands:
nestgate> create pool mypool 5G
nestgate> create dataset mypool/data
nestgate> write-test-data mypool/data 100M
```

---

## 💡 Pro Tips

### 1. **Record Your Demo**
```bash
# Install asciinema
sudo apt install asciinema

# Record
asciinema rec demo.cast
./run_showcase.sh
# Press Ctrl+D to stop

# Play back
asciinema play demo.cast
```

### 2. **Generate Reports**
```bash
./run_showcase.sh > demo_output.txt
./scripts/generate_report.sh demo_output.txt
```

### 3. **Customize Configuration**
```bash
# Edit config
nano config.toml

# Run with custom config
./run_showcase.sh --config my_config.toml
```

### 4. **Run Specific Demos**
```bash
# Just ZFS
./run_showcase.sh --demo zfs

# Just performance
./run_showcase.sh --demo performance

# Multiple demos
./run_showcase.sh --demo "zfs performance"
```

---

## 📊 Expected Performance

On typical hardware:

| Metric | Expected Value |
|--------|----------------|
| **Pool Creation** | ~100ms |
| **Dataset Creation** | ~50ms |
| **Snapshot Creation** | ~5ms |
| **Write Throughput** | 200-500 MB/s |
| **Operations/sec** | 100K-150K |
| **Latency (p95)** | <1ms |
| **Memory Usage** | ~50MB |

---

## 🆘 Need Help?

### Check Logs
```bash
cat outputs/logs/showcase.log | tail -100
```

### Verify Setup
```bash
./run_showcase.sh --dry-run --verbose
```

### Run Diagnostics
```bash
./scripts/diagnose.sh
```

### Get Support
- **Documentation**: `../docs/`
- **Examples**: `../examples/`
- **Issues**: Create GitHub issue

---

## ✅ Success Checklist

After running showcase, you should have:

- [ ] Seen ZFS pool creation and operations
- [ ] Observed performance comparisons
- [ ] Viewed live metrics dashboard
- [ ] Reviewed activity logs
- [ ] Understood compression benefits
- [ ] Seen zero-cost architecture in action

---

**🎉 Enjoy the showcase!**

Questions? Read the [full README](README.md) or check the [documentation](../docs/).

---

*Quick Start Guide | Version 1.0.0 | November 8, 2025*

