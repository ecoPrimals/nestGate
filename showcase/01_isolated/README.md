# Level 1: Isolated Instance

**Goal**: Understand single NestGate capabilities on one machine  
**Time**: 30 minutes  
**Complexity**: Beginner  
**Prerequisites**: NestGate built, basic command-line skills

---

## 🎯 WHAT YOU'LL LEARN

By completing these 5 demos, you'll understand:
- ✅ Basic storage operations (store, retrieve, list)
- ✅ REST API usage for data services
- ✅ Capability-based self-discovery (no hardcoded config!)
- ✅ Health monitoring and metrics
- ✅ Advanced ZFS features

**After this level**: You'll be ready for Level 2 (Ecosystem Integration)

---

## 📋 DEMOS IN THIS LEVEL

### 1. Storage Basics (5 minutes) ⭐ START HERE
**Path**: `01_storage_basics/`  
**What it shows**: Store and retrieve data using NestGate

```bash
cd 01_storage_basics
./demo.sh
```

**You'll see**:
- Create a dataset
- Store data
- Retrieve data
- List all datasets

---

### 2. Data Services (8 minutes)
**Path**: `02_data_services/`  
**What it shows**: Use REST API for data operations

```bash
cd 02_data_services
./demo.sh
```

**You'll see**:
- POST data via API
- GET data via API
- Query datasets
- Health check endpoint

---

### 3. Capability Discovery (7 minutes)
**Path**: `03_capability_discovery/`  
**What it shows**: NestGate discovers its own capabilities at runtime

```bash
cd 03_capability_discovery
./demo.sh
```

**You'll see**:
- Auto-detect storage backends
- Discover available features
- No hardcoded configuration
- Runtime capability reporting

**This is important**: Shows zero-knowledge architecture!

---

### 4. Health Monitoring (5 minutes)
**Path**: `04_health_monitoring/`  
**What it shows**: Monitor system health and metrics

```bash
cd 04_health_monitoring
./demo.sh
```

**You'll see**:
- System health checks
- Storage metrics
- Performance data
- Resource usage

---

### 5. ZFS Advanced (5 minutes)
**Path**: `05_zfs_advanced/`  
**What it shows**: Advanced ZFS features

```bash
cd 05_zfs_advanced
./demo.sh
```

**You'll see**:
- Snapshots
- Compression
- Deduplication
- Copy-on-write benefits

---

## 🚀 QUICK START

**Option 1: Run all demos sequentially**
```bash
./run_all_isolated.sh
```

**Option 2: Run demos individually**
```bash
cd 01_storage_basics && ./demo.sh
cd ../02_data_services && ./demo.sh
# ... etc
```

---

## 📊 PROGRESS TRACKER

Track your progress:
- [ ] 01_storage_basics - Completed
- [ ] 02_data_services - Completed
- [ ] 03_capability_discovery - Completed
- [ ] 04_health_monitoring - Completed
- [ ] 05_zfs_advanced - Completed

**All done?** → Proceed to Level 2: `../02_ecosystem_integration/README.md`

---

## 🆘 TROUBLESHOOTING

### "NestGate not running"
```bash
# Start NestGate first
../../scripts/start_data_service.sh

# Check if running
curl http://localhost:8080/health
```

### "Demo script fails"
```bash
# Make executable
chmod +x */demo.sh

# Check prerequisites
../../utils/setup/check_prerequisites.sh
```

### "Port already in use"
```bash
# Check what's using port 8080
lsof -i :8080

# Or run on different port
NESTGATE_API_PORT=8081 ./demo.sh
```

---

## 💡 TIPS

1. **Watch the terminal output** - Each demo explains what it's doing
2. **Read the code** - All scripts are heavily commented
3. **Experiment** - Try changing parameters and see what happens
4. **Take notes** - Jot down questions for Level 2

---

## 📚 ADDITIONAL RESOURCES

- **API Reference**: `../../docs/API_REFERENCE.md`
- **Configuration Guide**: `../../CONFIGURATION_SYSTEM_GUIDE.md`
- **Architecture**: `../../ARCHITECTURE_OVERVIEW.md`

---

## ⏭️ WHAT'S NEXT?

**Level 2: Ecosystem Integration** (`../02_ecosystem_integration/`)
- See NestGate work with BearDog (crypto)
- See NestGate work with Songbird (orchestration)
- See NestGate work with ToadStool (compute)

**Time**: 45 minutes  
**Complexity**: Intermediate

---

**Questions?** See individual demo READMEs or `../00_START_HERE.md`

---

*Level 1 - Isolated Instance*  
*Last updated: December 17, 2025*

