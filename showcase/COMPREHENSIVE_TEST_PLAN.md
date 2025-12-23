# 🧪 Comprehensive Testing Plan - NestGate Showcase

**Date**: December 17, 2025  
**Purpose**: Validate all showcase demos with live NestGate services  
**Target**: 10 demos (5 Level 1 + 4 Level 2 + 1 Level 3)

---

## 📋 Testing Strategy

### Phase 1: Service Preparation ✅
1. Build NestGate release binary
2. Start local NestGate service
3. Verify health endpoints
4. Check prerequisites

### Phase 2: Level 1 Testing (Isolated)
Test each demo with live service:
1. Storage Basics
2. Data Services
3. Capability Discovery
4. Health Monitoring
5. ZFS Advanced

### Phase 3: Level 2 Testing (Ecosystem)
Test integration demos:
1. BearDog Crypto
2. Songbird Orchestration
3. ToadStool Storage
4. Data Flow Patterns

### Phase 4: Level 3 Testing (Federation)
Test distributed demos:
1. Mesh Formation

### Phase 5: Documentation & Fixes
- Document all issues
- Create fixes
- Update demos as needed
- Create validation report

---

## 🎯 Test Criteria

### Success Criteria
- ✅ Demo runs without errors
- ✅ All API calls succeed
- ✅ Output is meaningful
- ✅ Documentation matches behavior
- ✅ Cleanup works properly

### Failure Indicators
- ❌ Script exits with error
- ❌ API calls return 404/500
- ❌ Undefined behavior
- ❌ Documentation mismatch
- ❌ Resource leaks

---

## 📊 Test Matrix

| Demo | Service | API Used | Expected Result | Status |
|------|---------|----------|----------------|--------|
| 1.1 Storage | NestGate | /datasets, /objects | CRUD operations | 📋 |
| 1.2 Data | NestGate | /api/v1/* | Full CRUD | 📋 |
| 1.3 Discovery | NestGate | /capabilities | Service list | 📋 |
| 1.4 Health | NestGate | /health, /metrics | Status info | 📋 |
| 1.5 ZFS | NestGate | /storage/zfs | Advanced ops | 📋 |
| 2.1 BearDog | Nested+BD | Crypto APIs | Encryption | 📋 |
| 2.2 Songbird | Nested+SB | Orchestration | Workflows | 📋 |
| 2.3 ToadStool | Nested+TS | Compute | Storage ops | 📋 |
| 2.4 Flows | All | Various | Patterns | 📋 |
| 3.1 Mesh | Multi-NG | Federation | Mesh form | 📋 |

---

## 🔧 Prerequisites

### Required
- [x] Rust/Cargo installed
- [x] NestGate built (release)
- [ ] curl available
- [ ] jq available
- [ ] bash 4.0+

### Optional
- [ ] BearDog service (for Level 2.1)
- [ ] Songbird service (for Level 2.2)
- [ ] ToadStool service (for Level 2.3)
- [ ] Docker (for multi-instance)

---

## 🚀 Execution Plan

### Step 1: Build & Start Service
```bash
cd /home/eastgate/Development/ecoPrimals/nestgate
cargo build --release
./start_local_dev.sh
```

### Step 2: Run Prerequisites Check
```bash
cd showcase/utils/setup
./check_prerequisites.sh
```

### Step 3: Test Level 1 (Sequential)
```bash
cd showcase/01_isolated
./run_all_isolated.sh --test-mode
```

### Step 4: Test Level 2 (Individual)
```bash
cd showcase/02_ecosystem_integration
for demo in */; do
    echo "Testing: $demo"
    cd "$demo" && ./demo.sh --test-mode
    cd ..
done
```

### Step 5: Test Level 3
```bash
cd showcase/03_federation/01_mesh_formation
./demo.sh --test-mode
```

### Step 6: Generate Report
```bash
cd showcase
./generate_test_report.sh
```

---

## 📝 Test Log Format

```json
{
  "demo": "01_storage_basics",
  "level": 1,
  "timestamp": "2025-12-17T20:00:00Z",
  "duration_seconds": 12,
  "status": "pass|fail|skip",
  "errors": [],
  "warnings": [],
  "api_calls": 15,
  "api_failures": 0,
  "notes": "All operations successful"
}
```

---

## 🐛 Known Issues

### Expected Issues
1. **Level 2 Demos** - Require external services (BearDog, Songbird, ToadStool)
   - **Solution**: Mock or skip if services unavailable
   
2. **Level 3 Demo** - Requires multiple NestGate instances
   - **Solution**: Use Docker or skip multi-node tests
   
3. **ZFS Operations** - May require root/sudo
   - **Solution**: Mock ZFS or document limitations

### Mitigations
- All demos have `--test-mode` flag for CI/automated testing
- Mock services available for Level 2
- Single-node mode for Level 3

---

## 📊 Expected Results

### Level 1: Isolated (High Confidence)
- **Expected Pass Rate**: 100%
- **Reason**: Only requires NestGate service
- **Risk**: Low

### Level 2: Ecosystem (Medium Confidence)
- **Expected Pass Rate**: 25-100% (depends on services)
- **Reason**: Requires external primal services
- **Risk**: Medium (service availability)

### Level 3: Federation (Medium Confidence)
- **Expected Pass Rate**: 50-100% (depends on setup)
- **Reason**: Requires multi-instance setup
- **Risk**: Medium (complexity)

---

## 🎯 Success Metrics

### Minimum Viable
- ✅ All Level 1 demos pass (5/5)
- ✅ At least 1 Level 2 demo passes (1/4)
- ✅ Level 3 runs in single-node mode (1/1)
- **Total**: 7/10 (70%) minimum

### Target
- ✅ All Level 1 demos pass (5/5)
- ✅ All Level 2 demos pass or gracefully degrade (4/4)
- ✅ Level 3 demonstrates concept (1/1)
- **Total**: 10/10 (100%) target

### Stretch
- ✅ All demos pass with real services
- ✅ Performance benchmarks included
- ✅ Chaos testing included
- **Total**: 10/10 + extras

---

## 📦 Deliverables

1. **Test Report** (`TEST_REPORT_DEC_17_2025.md`)
   - Summary of all tests
   - Pass/fail status
   - Issues found
   - Recommendations

2. **Updated Demos**
   - Fixes for any issues
   - Enhanced error handling
   - Better documentation

3. **Validation Checklist**
   - Pre-flight checks
   - Test commands
   - Expected outputs

4. **CI/CD Integration**
   - Automated test script
   - GitHub Actions config
   - Test matrix

---

## 🔄 Iteration Plan

### Round 1: Discovery (Current)
- Run all tests
- Document issues
- Identify patterns

### Round 2: Fixes
- Fix critical issues
- Update documentation
- Add error handling

### Round 3: Polish
- Improve UX
- Add more examples
- Performance tuning

### Round 4: Validation
- Rerun all tests
- Verify fixes
- Final report

---

## 📚 References

- [00_START_HERE.md](00_START_HERE.md) - Showcase entry point
- [NESTGATE_SHOWCASE_PLAN_DEC_17_2025.md](NESTGATE_SHOWCASE_PLAN_DEC_17_2025.md) - Full plan
- [utils/setup/check_prerequisites.sh](utils/setup/check_prerequisites.sh) - Prerequisites
- [../STATUS.md](../STATUS.md) - Overall status

---

**Status**: Ready to Execute  
**Next**: Build service and begin testing  
**Owner**: Automated testing + manual validation

