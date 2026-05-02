# 🏠 NESTGATE CORE DOMAIN SPECIFICATION

**Version**: 2.0.0  
**Date**: November 6, 2025  
**Status**: 🔄 **REFOCUSED** - Clear Domain Boundaries

---

## 🎯 EXECUTIVE SUMMARY

### NestGate's True Purpose:
**World-class storage and data management system** that integrates with the ecoPrimals ecosystem.

### What Changed:
**Removed**: Networking and security features (delegate to Songbird & BearDog)  
**Focus**: Storage excellence, ZFS operations, data management

---

## 🏗️ CORE DOMAIN: STORAGE & DATA MANAGEMENT

### ✅ **NestGate SHOULD Implement**:

#### 1. **ZFS Operations** (Core Competency)
```yaml
Domain: ZFS dataset and pool management
Scope:
  - Dataset creation, deletion, modification
  - Pool management and configuration
  - ZFS properties and attributes
  - Quota and reservation management
  - Compression settings
  - Deduplication management
Status: 80% complete
Priority: P0
```

#### 2. **Snapshot Management** (Core Competency)
```yaml
Domain: Data snapshots and versioning
Scope:
  - Snapshot creation and deletion
  - Snapshot scheduling and automation
  - Snapshot browsing and restoration
  - Clone operations
  - Incremental snapshot transfers
  - Snapshot retention policies
Status: 70% complete
Priority: P0
```

#### 3. **Storage Monitoring** (Core Competency)
```yaml
Domain: Storage health and performance metrics
Scope:
  - Disk usage tracking
  - I/O performance metrics
  - Pool health monitoring
  - SMART data integration
  - Storage capacity planning
  - Alert thresholds
Status: 60% complete
Priority: P1
```

#### 4. **Data Encryption at Rest** (Core Competency)
```yaml
Domain: Storage-level encryption
Scope:
  - ZFS native encryption
  - Encryption key management (via BearDog)
  - Encrypted dataset creation
  - Key rotation
  - Encryption performance monitoring
Status: 50% complete
Priority: P1
Note: Key management delegated to BearDog
```

#### 5. **Storage Access Control** (Core Competency)
```yaml
Domain: Data-level permissions
Scope:
  - Dataset permissions (who can access what data)
  - File/directory ACLs
  - User/group management for storage
  - Quota enforcement per user
  - Audit logging for data access
Status: 40% complete
Priority: P1
Note: User authentication delegated to BearDog
```

#### 6. **Universal Storage Adapters** (Core Competency)
```yaml
Domain: Storage backend abstraction
Scope:
  - Filesystem adapter (local FS)
  - ZFS adapter (native)
  - Object storage adapter (S3-compatible)
  - Block storage adapter (iSCSI, etc.)
  - Network filesystem adapter (NFS, SMB)
Status: 60% complete (filesystem working)
Priority: P1
```

#### 7. **Infant Discovery for Storage** (Core Competency)
```yaml
Domain: Zero-knowledge storage capability discovery
Scope:
  - Discover available storage backends
  - Detect ZFS availability
  - Find storage capabilities
  - Auto-configure based on detected features
  - No hardcoded storage assumptions
Status: 85% complete
Priority: P0
```

#### 8. **Data Migration & Replication** (Core Competency)
```yaml
Domain: Moving data between storage systems
Scope:
  - Dataset replication
  - Cross-pool migration
  - Backup/restore operations
  - Incremental transfers
  - Multi-tower data sync
Status: 30% complete
Priority: P2
```

---

## ❌ **NestGate SHOULD NOT Implement** (Delegate to Other Primals)

### 🎼 **Delegate to Songbird** (Networking & Orchestration):

#### 1. **Load Balancing** ❌
```yaml
Primal: Songbird
Reason: Service mesh is Songbird's domain
Current Status: Implemented in NestGate (WRONG)
Action: Remove LoadBalancingConfig, delegate to Songbird
Lines to Remove: ~200-300
Effort: 8-12 hours
```

#### 2. **Circuit Breaker** ❌
```yaml
Primal: Songbird
Reason: Failure handling is service mesh feature
Current Status: Implemented in NestGate (WRONG)
Action: Remove CircuitBreakerConfig, delegate to Songbird
Lines to Remove: ~150-250
Effort: 6-10 hours
```

#### 3. **Service Discovery** ❌
```yaml
Primal: Songbird
Reason: Universal capability discovery is Songbird's core
Current Status: ServiceDiscoveryClient in NestGate (WRONG)
Action: Remove, use Songbird's discovery
Lines to Remove: ~300-400
Effort: 12-16 hours
```

#### 4. **Orchestration** ❌
```yaml
Primal: Songbird
Reason: Multi-service workflows are Songbird's purpose
Current Status: OrchestrationAdapter in NestGate (WRONG)
Action: Remove completely, delegate to Songbird
Lines to Remove: ~400-500
Effort: 16-20 hours
```

#### 5. **Network Protocols** ❌
```yaml
Primal: Songbird
Reason: Network-level protocols are Songbird's domain
Current Status: Protocol handling in nestgate-network (WRONG) [RESOLVED: crate removed]
Action: Keep only storage-specific network access (NFS, SMB client)
Lines to Remove: ~200-300
Effort: 8-12 hours
```

**Total Songbird Overlap**: 1,250-1,750 lines, 50-70 hours

---

### 🐻 **Delegate to BearDog** (Security & Cryptography):

#### 1. **Rate Limiting** ❌
```yaml
Primal: BearDog
Reason: Network security is BearDog's domain
Current Status: RateLimiter in NestGate (WRONG)
Action: Remove, delegate to BearDog
Lines to Remove: ~250-350
Effort: 10-14 hours
```

#### 2. **Intrusion Detection** ❌
```yaml
Primal: BearDog
Reason: Threat detection is BearDog's expertise
Current Status: IntrusionDetectionSystem in NestGate (WRONG)
Action: Remove, delegate to BearDog
Lines to Remove: ~300-400
Effort: 12-16 hours
```

#### 3. **Security Audit Logging** ❌
```yaml
Primal: BearDog
Reason: Security events are BearDog's responsibility
Current Status: SecurityAuditLogger in NestGate (WRONG)
Action: Remove, delegate to BearDog
Lines to Remove: ~200-300
Effort: 8-12 hours
```

#### 4. **Security Input Validation** ❌
```yaml
Primal: BearDog
Reason: SQL injection, XSS detection is security domain
Current Status: RequestValidator with security checks (WRONG)
Action: Keep data format validation, remove security validation
Lines to Remove: ~200-300
Effort: 8-12 hours
Note: Keep basic format validation (file types, sizes)
```

#### 5. **Authentication System** ❌
```yaml
Primal: BearDog
Reason: User authentication is BearDog's core domain
Current Status: AuthContext, auth module in NestGate (WRONG)
Action: Remove, receive pre-authenticated requests from BearDog
Lines to Remove: ~400-500
Effort: 16-20 hours
Note: Keep storage access control (which user can access which data)
```

#### 6. **Security Hardening Manager** ❌
```yaml
Primal: BearDog
Reason: Comprehensive security infrastructure
Current Status: 974 lines in security_hardening.rs (WRONG)
Action: Remove entire production_hardening module
Lines to Remove: ~1,000-1,200
Effort: 40-50 hours
Note: One of the largest overlaps
```

**Total BearDog Overlap**: 2,350-3,050 lines, 94-124 hours

---

## 🔗 ECOSYSTEM INTEGRATION POINTS

### How NestGate Integrates with Other Primals:

#### With 🎼 **Songbird** (Networking):
```rust
// NestGate discovers Songbird for networking capabilities
let songbird = universal_adapter
    .discover_capability("networking")
    .await?;

// Songbird handles load balancing to NestGate instances
// Songbird orchestrates multi-storage workflows
// Songbird provides circuit breaking for NestGate calls
```

**NestGate provides to Songbird**:
- Storage capability registration
- Health status for load balancing
- Storage metrics for monitoring
- Storage API endpoints

**NestGate receives from Songbird**:
- Service discovery
- Load balanced requests
- Network orchestration
- Failure handling

---

#### With 🐻 **BearDog** (Security):
```rust
// NestGate discovers BearDog for security capabilities
let beardog = universal_adapter
    .discover_capability("security")
    .await?;

// BearDog handles authentication
let user_context = beardog.validate_token(auth_token).await?;

// BearDog manages encryption keys
let encryption_key = beardog.get_encryption_key(dataset_id).await?;

// BearDog performs security auditing
beardog.log_security_event(event).await?;
```

**NestGate provides to BearDog**:
- Storage access events for auditing
- Encryption key usage patterns
- Data access requests for authorization

**NestGate receives from BearDog**:
- Pre-authenticated user contexts
- Encryption key management
- Security event logging
- Rate limiting enforcement

---

## 📊 REVISED IMPLEMENTATION STATUS

### Before Refocus (Old Scope):
```yaml
Overall: 55% complete (45% mocks)
Problems:
  - Implementing networking (Songbird's domain)
  - Implementing security (BearDog's domain)
  - Duplicating tested implementations
  - Violating ecosystem sovereignty
```

### After Refocus (True Scope):
```yaml
Core Storage Features:
  ZFS Operations:           80% complete ✅
  Snapshot Management:      70% complete ✅
  Storage Monitoring:       60% complete ⚠️
  Universal Adapters:       60% complete ⚠️
  Infant Discovery:         85% complete ✅
  Data Encryption:          50% complete ⚠️
  Access Control:           40% complete ⚠️
  Migration/Replication:    30% complete ⚠️

Average: 59% complete (up from 55%)

Ecosystem Integration:
  Songbird Delegation:      0% complete ❌ (needs implementation)
  BearDog Delegation:       0% complete ❌ (needs implementation)
```

---

## 🎯 PRIORITIES & ROADMAP

### Phase 1: Remove Overlaps (6-9 weeks)
**Priority**: P0 - CRITICAL

#### Week 1-2: Remove Songbird Overlaps
- [ ] Remove LoadBalancingConfig (8-12h)
- [ ] Remove CircuitBreakerConfig (6-10h)
- [ ] Remove ServiceDiscoveryClient (12-16h)
- [ ] Remove OrchestrationAdapter (16-20h)
- [ ] Add Songbird discovery adapter (10-15h)
**Subtotal**: 52-73 hours

#### Week 3-5: Remove BearDog Overlaps
- [ ] Remove RateLimiter (10-14h)
- [ ] Remove IntrusionDetectionSystem (12-16h)
- [ ] Remove SecurityAuditLogger (8-12h)
- [ ] Remove security validation (8-12h)
- [ ] Remove auth system (16-20h)
- [ ] Remove security_hardening module (40-50h)
- [ ] Add BearDog discovery adapter (10-15h)
**Subtotal**: 104-139 hours

#### Week 6-9: Integration & Testing
- [ ] Integration tests with Songbird (15-20h)
- [ ] Integration tests with BearDog (15-20h)
- [ ] E2E ecosystem tests (20-30h)
- [ ] Update all documentation (10-15h)
**Subtotal**: 60-85 hours

**Phase 1 Total**: 216-297 hours (6-9 weeks)

---

### Phase 2: Complete Storage Features (4-7 weeks)
**Priority**: P1 - HIGH

#### Storage Feature Completion
- [ ] Complete snapshot automation (15-20h)
- [ ] Complete storage monitoring (20-30h)
- [ ] Complete universal adapters (25-35h)
- [ ] Complete data encryption (20-30h)
- [ ] Complete access control (25-35h)
- [ ] Implement migration/replication (40-60h)
**Subtotal**: 145-210 hours

**Phase 2 Total**: 145-210 hours (4-7 weeks)

---

### Phase 3: Polish & Production (2-4 weeks)
**Priority**: P2 - MEDIUM

- [ ] Achieve 90% test coverage (40-60h)
- [ ] Performance optimization (20-30h)
- [ ] Documentation polish (15-20h)
- [ ] Production hardening (15-20h)
**Subtotal**: 90-130 hours

**Phase 3 Total**: 90-130 hours (2-4 weeks)

---

## 📈 REVISED TIMELINE

**Total Effort**: 451-637 hours (12-20 weeks)

**Breakdown**:
- Remove overlaps: 216-297 hours (48%)
- Complete storage: 145-210 hours (32%)
- Polish & production: 90-130 hours (20%)

**Previous Timeline**: 655-975 hours (14-18 weeks)  
**Savings**: 204-338 hours (2-8 weeks faster!)

**Why Faster?**:
- Not implementing networking (Songbird already has it)
- Not implementing security (BearDog already has it)
- Focused scope = faster completion

---

## ✅ SUCCESS CRITERIA

### Phase 1 Complete When:
- [ ] Zero networking code remains (all delegated to Songbird)
- [ ] Zero security hardening code remains (all delegated to BearDog)
- [ ] Universal adapters work with Songbird & BearDog
- [ ] All tests pass with mocked Songbird & BearDog
- [ ] Integration tests work with real Songbird & BearDog
- [ ] Documentation reflects true domain

### Phase 2 Complete When:
- [ ] All storage features 80%+ complete
- [ ] Snapshot automation working
- [ ] Storage monitoring comprehensive
- [ ] Universal adapters support 4+ backends
- [ ] Data encryption fully functional
- [ ] Access control complete

### Phase 3 Complete When:
- [ ] 90% test coverage
- [ ] Performance meets benchmarks
- [ ] Production deployment ready
- [ ] Full ecosystem integration validated
- [ ] Documentation complete

---

## 📚 ARCHITECTURAL PRINCIPLES

### 1. **Single Responsibility**
NestGate does **ONE THING WELL**: Storage & Data Management

### 2. **Ecosystem Sovereignty**
Each primal owns its domain. No overlap. Clear boundaries.

### 3. **Capability Discovery**
Discover what others can do. Never hardcode primal names.

### 4. **Graceful Degradation**
Work standalone if other primals unavailable. Degrade gracefully.

### 5. **Zero-Cost Abstractions**
Storage operations must be fast. No unnecessary overhead.

---

## 🎓 LESSONS LEARNED

### What Went Wrong:
1. NestGate tried to be "self-sufficient" ❌
2. Reimplemented networking (Songbird's domain) ❌
3. Reimplemented security (BearDog's domain) ❌
4. Created 3,600+ lines of duplicate code ❌
5. Violated ecosystem sovereignty ❌

### What We're Fixing:
1. Clear domain boundaries ✅
2. Delegate to specialized primals ✅
3. Focus on storage excellence ✅
4. Remove 3,600+ lines of overlap ✅
5. Achieve ecosystem compliance ✅

### Key Insight:
> **"A world-class storage system that delegates to world-class security and networking is better than a mediocre all-in-one system."**

---

## 🔗 RELATED SPECIFICATIONS

- [Universal Storage Architecture](UNIVERSAL_STORAGE_AGNOSTIC_ARCHITECTURE.md)
- [Infant Discovery](INFANT_DISCOVERY_ARCHITECTURE_SPEC.md)
- [Zero-Cost Architecture](ZERO_COST_ARCHITECTURE_FINAL_SPECIFICATION.md)
- [Primal Ecosystem Integration](PRIMAL_ECOSYSTEM_INTEGRATION_SPEC.md)

---

## 📞 INTEGRATION EXAMPLES

### Example 1: Storing User Data
```rust
// 1. BearDog authenticates user
let user = beardog.authenticate(credentials).await?;

// 2. NestGate checks storage access
let has_access = storage.check_access(user.id, dataset_id)?;

// 3. NestGate stores data (encrypted via BearDog keys)
let encryption_key = beardog.get_key(dataset_id).await?;
storage.write_encrypted(data, encryption_key)?;

// 4. BearDog logs security event
beardog.log_event("data_written", user.id, dataset_id).await?;
```

### Example 2: Load Balanced Storage Access
```rust
// 1. Songbird discovers available NestGate instances
let storage_nodes = songbird.discover_capability("storage").await?;

// 2. Songbird load balances request
let chosen_node = songbird.select_node(storage_nodes, algorithm)?;

// 3. Songbird routes request with circuit breaker
let result = songbird.call_with_protection(
    chosen_node,
    |node| node.get_dataset(id)
).await?;
```

---

## ✅ APPROVAL & SIGN-OFF

**Specification Version**: 2.0.0  
**Approved By**: Pending  
**Date**: November 6, 2025  
**Status**: ✅ READY FOR IMPLEMENTATION

**This specification**:
- ✅ Clearly defines NestGate's domain
- ✅ Identifies what to delegate
- ✅ Provides integration patterns
- ✅ Includes realistic timeline
- ✅ Based on ecosystem architecture

**Next Steps**: Implement Phase 1 (Remove Overlaps)

---

**NestGate's Mission**: Be the **world's best storage system** that seamlessly integrates with the ecoPrimals ecosystem.

**Not**: Be an all-in-one system that does everything poorly.

