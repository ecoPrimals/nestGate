# 🌍 Ecosystem Coordination Plan

**Date**: December 17, 2025  
**Purpose**: Coordinate NestGate showcase with Songbird and ToadStool services  
**Goal**: Enable full ecosystem integration testing (10/10 demos)

---

## 🎯 Current Status

### NestGate ✅ **READY**
```
Status:         ✅ Production validated
Binary:         ✅ target/release/nestgate
Service:        ✅ Tested and working
Port:           8080
Demos Tested:   7/10 (100% pass rate)
```

### Songbird 🔍 **NEEDS CHECK**
```
Status:         🔍 Checking readiness
Binary:         ? target/release/songbird-orchestrator
Service:        ? Not yet tested
Port:           6666 (or 9090)
Demos Needed:   1 (Demo 2.2)
```

### ToadStool 🔍 **NEEDS CHECK**
```
Status:         🔍 Checking readiness  
Binary:         ? target/release/toadstool-server
Service:        ? Not yet tested
Port:           7000
Demos Needed:   1 (Demo 2.3)
```

### BearDog 📋 **OPTIONAL**
```
Status:         📋 Not in scope yet
Binary:         N/A
Service:        N/A
Port:           8000
Demos Needed:   1 (Demo 2.1)
```

---

## 📊 Demo Dependencies

### Already Validated (7/10) ✅
- **1.1**: Storage Basics - NestGate only ✅
- **1.2**: Data Services - NestGate only ✅
- **1.3**: Capability Discovery - NestGate only ✅
- **1.4**: Health Monitoring - NestGate only ✅
- **1.5**: ZFS Advanced - NestGate only ✅
- **2.4**: Data Flow Patterns - NestGate only ✅
- **3.1**: Mesh Formation - NestGate only ✅

### Requires Songbird (1/10) 🔍
- **2.2**: Songbird Orchestration - Needs Songbird service
  - **Port**: 6666 or 9090
  - **Endpoints**: `/health`, `/api/v1/discover`, `/api/v1/workflow`
  - **Priority**: High (ecosystem integration)

### Requires ToadStool (1/10) 🔍
- **2.3**: ToadStool Storage - Needs ToadStool service
  - **Port**: 7000
  - **Endpoints**: `/health`, `/api/v1/jobs`, `/api/v1/runtimes`
  - **Priority**: High (ecosystem integration)

### Requires BearDog (1/10) 📋
- **2.1**: BearDog Crypto - Needs BearDog service
  - **Port**: 8000
  - **Endpoints**: `/health`, `/api/v1/keys`, `/api/v1/encrypt`
  - **Priority**: Medium (crypto integration)

---

## 🚀 Coordination Strategy

### Phase 1: Service Discovery ✅ **COMPLETE**
- [x] Check NestGate binary
- [x] Check Songbird binary status
- [x] Check ToadStool binary status
- [x] Identify required ports
- [x] Document service dependencies

### Phase 2: Service Verification 🔍 **CURRENT**
- [ ] Verify Songbird binary exists and is recent
- [ ] Verify ToadStool binary exists and is recent
- [ ] Check if binaries need rebuild
- [ ] Document any missing dependencies

### Phase 3: Service Startup 📋 **NEXT**
- [ ] Create unified startup script
- [ ] Start Songbird service
- [ ] Start ToadStool service
- [ ] Verify all health endpoints
- [ ] Test basic inter-service communication

### Phase 4: Integration Testing 📋 **FUTURE**
- [ ] Run Demo 2.2 (Songbird)
- [ ] Run Demo 2.3 (ToadStool)
- [ ] Run full test suite (10/10)
- [ ] Generate comprehensive report
- [ ] Validate ecosystem readiness

### Phase 5: Documentation & Refinement 📋 **FUTURE**
- [ ] Update demo READMEs with real output
- [ ] Document service startup procedures
- [ ] Create troubleshooting guides
- [ ] Update ecosystem integration plan

---

## 🔧 Technical Requirements

### Port Allocation
```
Service      Port   Status
─────────────────────────────
NestGate     8080   ✅ Running
Songbird     6666   🔍 TBD (or 9090)
ToadStool    7000   🔍 TBD
BearDog      8000   📋 Future
```

### Service Dependencies
```
Demo 2.2 (Songbird) requires:
  ✅ NestGate running (8080)
  🔍 Songbird running (6666)
  
Demo 2.3 (ToadStool) requires:
  ✅ NestGate running (8080)
  🔍 ToadStool running (7000)
  
Demo 2.1 (BearDog) requires:
  ✅ NestGate running (8080)
  📋 BearDog running (8000)
```

### API Requirements

#### Songbird Must Provide:
- `GET /health` - Health check
- `GET /api/v1/discover?capability=storage` - Service discovery
- `POST /api/v1/workflow` - Workflow submission
- `GET /api/v1/workflow/{id}` - Workflow status
- `GET /api/v1/workflow/{id}/logs` - Workflow logs

#### ToadStool Must Provide:
- `GET /health` - Health check
- `GET /api/v1/runtimes` - Available runtimes
- `POST /api/v1/jobs` - Job submission
- `GET /api/v1/jobs/{id}` - Job status
- `GET /api/v1/jobs/{id}/logs` - Job logs

---

## 📝 Next Steps

### Immediate (Next 10 minutes)
1. ✅ Check if Songbird binary exists
2. ✅ Check if ToadStool binary exists
3. 📋 Verify binary build dates
4. 📋 Test basic service startup

### Short Term (Next 1 hour)
5. Build missing binaries if needed
6. Create unified ecosystem startup script
7. Start all services
8. Verify health endpoints

### Medium Term (Next 2-3 hours)
9. Run Demo 2.2 with live Songbird
10. Run Demo 2.3 with live ToadStool
11. Update test reports
12. Achieve 9/10 or 10/10 demo validation

---

## 🎯 Success Criteria

### Minimum Viable
- [ ] Songbird service starts successfully
- [ ] ToadStool service starts successfully
- [ ] All health endpoints respond
- [ ] Demo 2.2 OR Demo 2.3 passes
- **Result**: 8/10 demos validated (80%)

### Target
- [ ] Both Songbird and ToadStool running
- [ ] Demo 2.2 passes (Songbird)
- [ ] Demo 2.3 passes (ToadStool)
- [ ] All services stable
- **Result**: 9/10 demos validated (90%)

### Stretch
- [ ] BearDog service integrated
- [ ] Demo 2.1 passes (BearDog)
- [ ] Full ecosystem validated
- [ ] Performance benchmarks complete
- **Result**: 10/10 demos validated (100%)

---

## 📚 Reference Files

### Songbird
- Binary: `../songbird/target/release/songbird-orchestrator`
- Start Script: `../songbird/start_songbird_all_towers_http.sh`
- Status: `../songbird/STATUS.md`
- Showcase: `../songbird/showcase/`

### ToadStool
- Binary: `../toadstool/target/release/toadstool-*`
- Examples: `../toadstool/examples/`
- Status: `../toadstool/STATUS.md`
- Showcase: `../toadstool/showcase/`

### NestGate
- Binary: `target/release/nestgate` ✅
- Start Script: `./start_local_dev.sh` ✅
- Demo Scripts: `showcase/02_ecosystem_integration/` ✅
- Test Reports: `showcase/LIVE_SERVICE_TEST_REPORT_DEC_17_2025.md` ✅

---

## 🔍 Discovery Questions

### For Songbird:
1. Is `songbird-orchestrator` binary built?
2. What port does it use by default?
3. Are there startup scripts available?
4. What APIs are currently implemented?
5. Is it ready for integration testing?

### For ToadStool:
1. Is `toadstool-server` or similar binary built?
2. What port does it use by default?
3. How do we start the service?
4. What APIs are currently implemented?
5. Is it ready for integration testing?

### For Integration:
1. Do services need specific environment variables?
2. Are there authentication requirements?
3. Do services discover each other automatically?
4. Are there existing integration tests we can leverage?
5. What's the startup order (if any)?

---

## 🎨 Ecosystem Startup Script (Draft)

```bash
#!/usr/bin/env bash
# Start full ecoPrimals ecosystem for showcase testing

set -euo pipefail

echo "🌍 Starting ecoPrimals Ecosystem..."

# 1. Start NestGate (storage & data)
echo "📦 Starting NestGate..."
cd /home/eastgate/Development/ecoPrimals/nestgate
./start_local_dev.sh
sleep 2

# 2. Start Songbird (orchestration)
echo "🐦 Starting Songbird..."
cd /home/eastgate/Development/ecoPrimals/songbird
# TBD: actual start command
sleep 2

# 3. Start ToadStool (compute)
echo "🍄 Starting ToadStool..."
cd /home/eastgate/Development/ecoPrimals/toadstool
# TBD: actual start command
sleep 2

# 4. Verify all services
echo "✅ Verifying services..."
curl -sf http://localhost:8080/health && echo "  ✓ NestGate" || echo "  ✗ NestGate"
curl -sf http://localhost:6666/health && echo "  ✓ Songbird" || echo "  ✗ Songbird"
curl -sf http://localhost:7000/health && echo "  ✓ ToadStool" || echo "  ✗ ToadStool"

echo "🎉 Ecosystem ready!"
```

---

## 📊 Expected Timeline

### If Binaries Exist & Are Ready
- **Phase 2**: 10 minutes (verify)
- **Phase 3**: 20 minutes (startup)
- **Phase 4**: 30 minutes (testing)
- **Phase 5**: 20 minutes (docs)
- **Total**: ~1.5 hours

### If Binaries Need Building
- **Build Time**: +30-60 minutes (Songbird + ToadStool)
- **Total**: ~2-3 hours

### If APIs Need Implementation
- **Development**: Variable (hours to days)
- **Fallback**: Use simulated/mock responses in demos

---

## 🎯 Decision Point

**Question for User**: Are Songbird and ToadStool binaries:
1. ✅ **Built and ready** - We can start immediately
2. 📋 **Need building** - We should build them first
3. 🔍 **API incomplete** - We should document what's missing

**Next Action**: Based on binary check results, determine path forward.

---

**Status**: Ready for Phase 2 (Service Verification)  
**Confidence**: High (NestGate validated, infrastructure exists)  
**Blocker**: None yet (checking service readiness)

🚀 **Ready to coordinate full ecosystem!**

