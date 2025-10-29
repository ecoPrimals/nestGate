# 📋 NestGate Strategic Plans

**Location**: `/docs/plans/`  
**Purpose**: Strategic migration and improvement plans with timelines and patterns  
**Last Updated**: October 28, 2025

---

## 🎯 Active Plans

### **Testing & Coverage**

#### **[E2E_TEST_RESTORATION_PLAN.md](./E2E_TEST_RESTORATION_PLAN.md)**
- **Status**: Ready to execute
- **Scope**: Restore 11 disabled E2E test files + 3 examples
- **Timeline**: 2-3 weeks
- **Phases**:
  1. Analysis & infrastructure setup (3-4 days)
  2. Priority tests restoration (4-5 days)
  3. Comprehensive coverage (5-7 days)
  4. Chaos & fault testing (3-4 days)
- **Impact**: Critical for production readiness

#### **[TEST_MODERNIZATION_PLAN.md](./TEST_MODERNIZATION_PLAN.md)**
- **Status**: Ready to execute
- **Scope**: Fix commented tests, standardize organization
- **Timeline**: 1-2 weeks
- **Key Tasks**:
  - Fix 47 commented tests
  - Standardize test organization
  - Organize test helpers
  - Add missing test modules
- **Impact**: Improves test maintainability

---

### **Code Quality & Idiomatic Rust**

#### **[UNWRAP_MIGRATION_PLAN_STRATEGIC.md](./UNWRAP_MIGRATION_PLAN_STRATEGIC.md)**
- **Status**: Ready to execute (HIGH PRIORITY)
- **Scope**: Reduce 1,296 unwrap/expect to <100 in production
- **Timeline**: 3-4 weeks (75-100 hours)
- **Target**: 20 unwraps/day average
- **Phases**:
  1. API handlers & REST endpoints (Week 1)
  2. Service layer & business logic (Week 2)
  3. Core libraries & utilities (Week 3)
  4. Infrastructure & tooling (Week 4)
- **Patterns**: 8 proven migration patterns provided
- **Impact**: Production-grade error handling

#### **[HARDCODED_PORT_MIGRATION_PLAN_STRATEGIC.md](./HARDCODED_PORT_MIGRATION_PLAN_STRATEGIC.md)**
- **Status**: Ready to execute
- **Scope**: Migrate 388 hardcoded port instances to env vars
- **Timeline**: 6-8 weeks
- **Breakdown**:
  - Test files: 354 instances
  - Source files: 34 instances
- **Patterns**: Environment variable patterns, config structure
- **Impact**: Essential for deployment flexibility

#### **[FILE_SIZE_REDUCTION_PLAN.md](./FILE_SIZE_REDUCTION_PLAN.md)**
- **Status**: DEFERRED (complex, low priority)
- **Scope**: Refactor 4 files exceeding 1000 lines
- **Timeline**: 6-8 hours per file (24-32 hours total)
- **Files**:
  1. `nestgate-api/src/rest/handlers/zfs.rs` (1261 lines)
  2. `nestgate-engine/src/engine/mod.rs` (1108 lines)
  3. `nestgate-core/src/services/workspace_service.rs` (1056 lines)
  4. `nestgate-api/src/handlers/snapshot_management/lifecycle.rs` (1026 lines)
- **Reason for Deferral**: Complex dependencies, better ROI elsewhere
- **Impact**: Code maintainability (not critical)

---

### **Session Planning**

#### **[NEXT_SESSION_ACTION_PLAN.md](./NEXT_SESSION_ACTION_PLAN.md)**
- **Status**: Current session plan
- **Prioritized Actions**:
  1. ✅ Fix clippy error (completed)
  2. ✅ Documentation sprint (completed)
  3. 🔄 Add 171 tests for 25% coverage (in progress)
  4. 🔜 Migrate 20 unwraps in API handlers
  5. 🔜 Analyze 3 disabled E2E test files
- **Decision Points**: Test coverage vs. unwrap migration vs. E2E restoration

---

## 📊 Plan Status Overview

| Plan | Priority | Status | Timeline | Effort |
|------|----------|--------|----------|--------|
| **E2E Test Restoration** | 🔴 Critical | Ready | 2-3 weeks | 80-100h |
| **Unwrap Migration** | 🔴 High | Ready | 3-4 weeks | 75-100h |
| **Test Modernization** | 🟡 Medium | Ready | 1-2 weeks | 40-60h |
| **Port Migration** | 🟡 Medium | Ready | 6-8 weeks | 120-160h |
| **File Size Reduction** | 🟢 Low | Deferred | 4-5 days | 24-32h |

---

## 🎯 Recommended Execution Order

### **Phase 1: Quick Wins (Current)**
1. ✅ Fix immediate issues (clippy, docs)
2. 🔄 Add tests to reach 25% coverage (171 tests)
3. 🔜 Begin unwrap migration (20 unwraps in API handlers)

### **Phase 2: Foundation (Weeks 1-2)**
1. Complete unwrap migration in API layer
2. Restore priority E2E tests (first 3 files)
3. Modernize test organization

### **Phase 3: Comprehensive (Weeks 3-6)**
1. Complete E2E test restoration (all 11 files)
2. Continue unwrap migration (service layer)
3. Begin port migration (test files first)

### **Phase 4: Excellence (Weeks 7-10)**
1. Complete unwrap migration (core libraries)
2. Complete port migration (all files)
3. Add chaos/fault testing
4. Reach 90% test coverage

### **Phase 5: Polish (Optional)**
1. Refactor large files if needed
2. Performance optimizations
3. Final documentation pass

---

## 🔗 Related Documentation

- **[Audit Reports](../audit-reports/)** - Comprehensive audit findings
- **[Project Status](../../PROJECT_STATUS.md)** - Current metrics
- **[Architecture Overview](../../ARCHITECTURE_OVERVIEW.md)** - System design
- **[Contributing Guide](../../CONTRIBUTING.md)** - Development workflow

---

## 📝 Plan Maintenance

### **How to Use These Plans**
1. Read the relevant plan document for detailed steps
2. Follow the patterns and examples provided
3. Track progress using the plan's metrics
4. Update plan status as work completes

### **Updating Plans**
- Update status when phases complete
- Adjust timelines based on actual velocity
- Add lessons learned and new patterns
- Keep metrics current

### **Creating New Plans**
- Use existing plans as templates
- Include: scope, timeline, phases, patterns, metrics
- Link to related documentation
- Provide clear success criteria

---

**Last Review**: October 28, 2025  
**Next Review**: As plans complete or priorities shift

