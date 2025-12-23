# NestGate Showcase

**Purpose**: Interactive demonstrations of NestGate's capabilities  
**Status**: ✅ Complete and Tested  
**Last Updated**: December 21, 2025

---

## 🎯 QUICK START

### Run the Full Showcase (20 minutes)
```bash
./run_full_showcase.sh
```

This automated script runs all demos in sequence and generates a comprehensive report.

### Run Individual Demos

**Level 1: Isolated Capabilities**
```bash
cd 01_isolated/01_storage_basics && ./demo.sh
cd 01_isolated/03_capability_discovery && ./demo.sh
```

**Level 2: Ecosystem Integration**
```bash
cd 02_ecosystem_integration/live/01_beardog_discovery && ./demo.sh
cd 02_ecosystem_integration/live/02_beardog_btsp && ./demo.sh
```

---

## 📚 SHOWCASE STRUCTURE

```
showcase/
├── 00_SHOWCASE_INDEX.md                    # This file
├── PROGRESSIVE_SHOWCASE_GUIDE.md           # Complete walkthrough guide
├── run_full_showcase.sh                    # Automated showcase runner
│
├── 01_isolated/                            # Level 1: Standalone capabilities
│   ├── 01_storage_basics/                  # Filesystem operations
│   │   ├── demo.sh                         # Runnable demo
│   │   ├── README.md                       # Demo documentation
│   │   └── outputs/                        # Demo results (generated)
│   │
│   └── 03_capability_discovery/            # Runtime discovery
│       ├── demo.sh
│       ├── README.md
│       └── outputs/
│
└── 02_ecosystem_integration/               # Level 2: Multi-primal
    └── live/
        ├── README.md                       # Live integration index
        ├── 01_beardog_discovery/           # Graceful degradation
        │   ├── demo.sh
        │   └── README.md
        └── 02_beardog_btsp/                # Real BTSP communication
            ├── demo.sh
            └── README.md
```

---

## 🌟 SHOWCASE PHILOSOPHY

This showcase follows a **progressive learning path**:

### Level 1: Isolated Capabilities
**Question**: "What can NestGate do on its own?"

**Demos**:
- `01_storage_basics`: Real filesystem storage operations
- `03_capability_discovery`: Runtime self-knowledge and discovery

**Key Takeaway**: NestGate is fully functional without any dependencies.

### Level 2: Ecosystem Integration
**Question**: "How does NestGate work with other primals?"

**Demos**:
- `01_beardog_discovery`: Graceful degradation (works with or without BearDog)
- `02_beardog_btsp`: Real primal-to-primal BTSP communication

**Key Takeaway**: NestGate discovers and uses available primals, but doesn't require them.

---

## 🎬 DEMO DESCRIPTIONS

### Demo 1.1: Storage Basics
**Time**: 5 minutes  
**Prerequisites**: None  
**What it shows**:
- Create filesystem storage backend
- Create 5 datasets (data, logs, backups, cache, tmp)
- Write 10MB test file
- Create snapshots
- Verify data integrity

**Run it**:
```bash
cd 01_isolated/01_storage_basics
./demo.sh
```

**Expected Output**:
```
✅ Storage backend created (filesystem)
✅ 5 datasets created
✅ 10MB test file written
✅ 5 snapshots created
✅ Data integrity verified
```

---

### Demo 1.3: Capability Discovery
**Time**: 7 minutes  
**Prerequisites**: None  
**What it shows**:
- Self-knowledge discovery (storage, network, resources)
- Network primal detection via port scanning
- Capability negotiation
- Dynamic configuration generation
- Capability advertisement

**Run it**:
```bash
cd 01_isolated/03_capability_discovery
./demo.sh
```

**Expected Output**:
```
🔍 Self-Knowledge Discovery
✓ ZFS available
✓ Filesystem backend available
✓ CPU cores: 24, Memory: 64 GB

🌐 Network Discovery
✓ Discovered N primals on network

🤝 Capability Negotiation
✓ All required capabilities available
Status: operational
```

---

### Demo 2.1: BearDog Discovery (Graceful Degradation)
**Time**: 3 minutes  
**Prerequisites**: None (works with or without BearDog)  
**What it shows**:
- Runtime primal discovery
- Graceful degradation when BearDog is absent
- Fallback to built-in encryption

**Run it**:
```bash
cd 02_ecosystem_integration/live/01_beardog_discovery
./demo.sh
```

**Expected Output (BearDog not running)**:
```
⚠️ No security primal discovered
📝 Falling back to built-in encryption
✅ SUCCESS: Graceful degradation working!
```

**Expected Output (BearDog running)**:
```
✅ Found security primal: BearDog
   Endpoint: http://localhost:9000
🎉 SUCCESS: Multi-primal integration working!
```

---

### Demo 2.2: BearDog BTSP Communication
**Time**: 5 minutes  
**Prerequisites**: BearDog BTSP server running on port 9000  
**What it shows**:
- Real BTSP tunnel establishment
- Encrypted storage via BearDog
- Multi-primal coordination

**Start BearDog first**:
```bash
# Terminal 1: Start BearDog
cd /home/eastgate/Development/ecoPrimals/beardog
BTSP_PORT=9000 ./target/release/examples/btsp_server
```

**Run demo**:
```bash
# Terminal 2: Run demo
cd 02_ecosystem_integration/live/02_beardog_btsp
./demo.sh
```

**Expected Output**:
```
✅ BearDog BTSP server started on port 9000
✅ BTSP tunnel established successfully
✅ Data encrypted via BearDog BTSP
✅ Encrypted data stored in NestGate
🎉 SUCCESS: Multi-primal encrypted storage working!
```

---

## 🚀 AUTOMATED SHOWCASE

### Full Showcase Runner

```bash
./run_full_showcase.sh
```

**What it does**:
1. Runs all Level 1 demos (isolated)
2. Checks for BearDog availability
3. Tests graceful degradation
4. Runs Level 2 demos if BearDog is available
5. Generates comprehensive report

**Output**:
```
full_showcase_output-<timestamp>/
├── SHOWCASE_REPORT.md              # Comprehensive report
├── 01_storage_basics.log           # Demo logs
├── 03_capability_discovery_isolated.log
├── 01_beardog_discovery_degraded.log
├── 02_beardog_btsp_integration.log (if BearDog available)
└── 03_capability_discovery_integrated.log (if BearDog available)
```

---

## 📊 COMPARISON TABLE

| Feature | Demo 1.1 | Demo 1.3 | Demo 2.1 | Demo 2.2 |
|---------|----------|----------|----------|----------|
| **Storage** | ✅ | ✅ | ✅ | ✅ |
| **Discovery** | - | ✅ | ✅ | ✅ |
| **Encryption** | - | - | ✅ (built-in) | ✅ (BearDog) |
| **Multi-Primal** | - | ✅ | ✅ | ✅ |
| **BTSP** | - | - | - | ✅ |
| **Dependencies** | None | None | None | BearDog |

---

## 🎯 KEY DIFFERENTIATORS

### What Makes This Showcase Different

**Traditional Demos**:
- Show best-case scenarios
- Assume all dependencies available
- Break when dependencies are missing
- Hardcoded configurations

**NestGate Showcase**:
- Shows isolated AND integrated scenarios
- Tests graceful degradation
- Works with or without dependencies
- Runtime discovery, zero hardcoding

---

## 🔬 EXPERIMENTS TO TRY

### Experiment 1: Chaos Testing
```bash
# Start integrated demo
cd 02_ecosystem_integration/live/02_beardog_btsp
./demo.sh &

# Kill BearDog mid-execution
sleep 2
killall btsp_server

# Watch NestGate handle the failure
```

### Experiment 2: Multi-Node Discovery
```bash
# Machine 1
cd 01_isolated/03_capability_discovery
NESTGATE_NODE_NAME=node1 ./demo.sh

# Machine 2 (different machine/VM)
NESTGATE_NODE_NAME=node2 ./demo.sh

# They'll discover each other!
```

### Experiment 3: Performance Comparison
```bash
# Benchmark standalone storage
cd 01_isolated/01_storage_basics
time ./demo.sh

# Benchmark integrated storage (with encryption)
cd 02_ecosystem_integration/live/02_beardog_btsp
time ./demo.sh

# Compare durations
```

---

## 📋 DEMO OUTPUT

Each demo generates:

### 1. RECEIPT.md
Human-readable summary of operations performed:
```markdown
# Demo Receipt
- Operations performed
- Performance metrics
- Files generated
- Cleanup instructions
```

### 2. JSON Artifacts
Machine-readable results:
- `self-capabilities.json`: Discovered capabilities
- `network-discovery.json`: Found primals
- `runtime-config.toml`: Generated configuration

### 3. Logs
Detailed execution logs for debugging

---

## 🆘 TROUBLESHOOTING

### "Demo failed - BearDog not found"
**Cause**: Demo 2.2 requires BearDog  
**Fix**: Start BearDog first:
```bash
cd ../beardog
BTSP_PORT=9000 ./target/release/examples/btsp_server
```

### "Permission denied - ZFS"
**Cause**: ZFS operations require sudo  
**Fix**: Demos use filesystem backend by default (no sudo needed)

### "mDNS discovery not working"
**Cause**: Firewall blocking port 5353  
**Fix**: Allow mDNS traffic:
```bash
sudo ufw allow 5353/udp
```

### "Demo outputs directory growing"
**Cause**: Each demo creates timestamped outputs  
**Fix**: Clean up old outputs:
```bash
find showcase -type d -name "outputs" -exec rm -rf {} +
```

---

## 📚 LEARNING PATH

### For Beginners
1. Read `PROGRESSIVE_SHOWCASE_GUIDE.md`
2. Run `./run_full_showcase.sh`
3. Explore individual demo READMEs
4. Try experiments

### For Advanced Users
1. Review all demo scripts (`demo.sh`)
2. Study capability discovery implementation
3. Create custom multi-primal scenarios
4. Contribute new demos

---

## 🔗 RELATED DOCUMENTATION

**Architecture**:
- `../docs/ECOSYSTEM_INTEGRATION_PLAN.md`: Integration strategy
- `../specs/INFANT_DISCOVERY_ARCHITECTURE_SPEC.md`: Discovery spec
- `../docs/current/UNIVERSAL_ADAPTER_ARCHITECTURE.md`: Adapter pattern

**Implementation**:
- `../examples/live-integration-01-storage-security.rs`: Graceful degradation example
- `../examples/live-integration-02-real-beardog.rs`: BTSP communication example
- `../tests/ecosystem/`: Ecosystem test harness

**Reports**:
- `../LIVE_INTEGRATION_SUCCESS_DEC_21_2025.md`: Integration success report
- `../00_ECOSYSTEM_INTEGRATION_COMPLETE_DEC_21_2025.md`: Ecosystem readiness

---

## ✅ SUCCESS CRITERIA

You'll know the showcase worked when:

1. ✅ All Level 1 demos pass without any primals running
2. ✅ Demo 1.3 detects 0 primals when isolated
3. ✅ Demo 2.1 successfully degrades when BearDog is absent
4. ✅ Demo 1.3 detects 1+ primals when BearDog is running
5. ✅ Demo 2.2 successfully communicates via BTSP
6. ✅ All receipts show "✅ SUCCESS"

---

## 📊 SHOWCASE METRICS

**Total Demos**: 4 core demos + 1 automated runner  
**Time Investment**: 20 minutes (full showcase)  
**Prerequisites**: None (BearDog optional for Level 2)  
**Success Rate**: 100% (tested on Linux)  
**Coverage**: Isolated → Graceful Degradation → Integration

---

## 🎓 WHAT YOU'LL LEARN

### Technical Concepts
- Runtime capability discovery
- Graceful degradation patterns
- Zero-knowledge architecture
- Service discovery and negotiation
- Multi-primal coordination

### NestGate Principles
- Sovereignty (works standalone)
- Collaboration (better together)
- Adaptability (graceful degradation)
- Discovery (runtime, not hardcoded)

---

## 🚀 NEXT STEPS

After completing this showcase:

1. **Explore the Code**:
   - Read example implementations
   - Review capability discovery code
   - Study BTSP client integration

2. **Run the Tests**:
   ```bash
   cargo test --workspace
   ```

3. **Build Your Own Integration**:
   - Start with your own primal
   - Use NestGate's discovery patterns
   - Follow zero-knowledge principles

4. **Contribute**:
   - Add new demos
   - Improve existing demos
   - Share your experiments

---

## 📈 ROADMAP

### Planned Demos

**Level 3: Advanced Scenarios**
- Songbird orchestration integration
- 3-primal scenario (NestGate + BearDog + Songbird)
- Distributed storage demo
- Chaos testing demo

**Level 4: Performance**
- Benchmark suite
- Load testing
- Performance comparison (standalone vs integrated)

**Level 5: Production**
- Docker Compose multi-primal setup
- Kubernetes deployment
- Production monitoring

---

## 🎯 KEY TAKEAWAYS

1. **NestGate works standalone** - Full functionality without dependencies
2. **Runtime discovery** - Finds and uses available primals dynamically
3. **Graceful degradation** - Adapts to available services
4. **Zero hardcoding** - No vendor lock-in or assumptions
5. **Multi-primal coordination** - Better together, not required

---

**This showcase proves NestGate's philosophy: Sovereign by default, collaborative by design.** 🏰

---

## 📞 GET HELP

**Issues**: Open an issue in the repo  
**Questions**: Check `PROGRESSIVE_SHOWCASE_GUIDE.md`  
**Experiments**: Share in discussions  

---

*NestGate Showcase - v0.1.0*  
*Last Updated: December 21, 2025*  
*Maintained by: NestGate Team*

