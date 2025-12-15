# 🎯 PRACTICAL MIGRATION GUIDE
## December 14, 2025 - Step-by-Step Execution

**Purpose**: Concrete examples and steps for systematic improvement  
**Baseline Established**: ✅ Analysis complete, targets identified

---

## 📊 BASELINE METRICS (Established Today)

### Migration Targets Identified
```
Hardcoded IPs (production):      593 instances
Hardcoded Ports (production):    367 instances  
Production unwrap():             1,599 instances
Production expect():             1,951 instances
─────────────────────────────────────────────
TOTAL MIGRATION TARGETS:         4,510 instances
```

### Files Generated
- ✅ `hardcoded_ips.txt` - 593 IP references to migrate
- ✅ `hardcoded_ports.txt` - 367 port references to migrate  
- ✅ `production_unwraps.txt` - 1,599 unwraps to replace
- ✅ `production_expects.txt` - 1,951 expects to replace

---

## 🚀 MIGRATION STRATEGY

### Phase 1: Hardcoded Values (593 IPs + 367 Ports = 960 total)

**Target**: Migrate 50% (480 instances) over 4 weeks

#### Week 1: Migrate 100 instances
**Priority**: Most frequently used constants

**Example 1: API Bind Address**
```rust
// File: code/crates/nestgate-api/src/server.rs (example)

// ❌ BEFORE: Hardcoded
let bind_addr = "0.0.0.0:8080";

// ✅ AFTER: Environment-driven
use nestgate_core::config::capability_based::*;
let config = CapabilityConfigBuilder::new().build()?;
let service = config.discover(PrimalCapability::HttpApi).await?;
let bind_addr = service.endpoint;
```

**Example 2: Database Connection**
```rust
// ❌ BEFORE: Hardcoded
let db_url = "localhost:5432";

// ✅ AFTER: Capability-based
let config = CapabilityConfigBuilder::new().build()?;
let db_service = config.discover(PrimalCapability::Database).await?;
let db_url = db_service.endpoint;
```

#### Systematic Approach
1. **Review** `hardcoded_ips.txt` and `hardcoded_ports.txt`
2. **Group** by module/functionality
3. **Prioritize** most frequently used
4. **Migrate** using pattern above
5. **Test** each migration
6. **Track** progress in `EXECUTION_PROGRESS_DEC_14_2025.md`

### Phase 2: Unwraps (1,599 unwrap + 1,951 expect = 3,550 total)

**Target**: Replace 300 production instances over 4 weeks

#### Week 1: Replace 75 instances
**Priority**: Critical paths (initialization, network, file I/O)

**Example 1: Environment Variable**
```rust
// File: Any configuration module

// ❌ BEFORE: Panics if missing
use std::env;
let port = env::var("PORT").unwrap().parse::<u16>().unwrap();

// ✅ AFTER: Proper error handling
use nestgate_core::utils::safe_operations::parse_env_var;
let port: u16 = parse_env_var("PORT")?;
// Returns Result with helpful error: "Environment variable PORT not found or invalid"
```

**Example 2: Collection Access**
```rust
// ❌ BEFORE: Panics if empty
let first_item = items[0];
let config = configs.get("api").unwrap();

// ✅ AFTER: Safe access
use nestgate_core::utils::safe_operations::SafeCollectionExt;
let first_item = items.safe_first()
    .context("Items list is empty")?;
let config = configs.safe_get("api")
    .context("API configuration not found")?;
```

**Example 3: File Operations**
```rust
// ❌ BEFORE: Panics on failure
let contents = std::fs::read_to_string("config.toml").unwrap();

// ✅ AFTER: Contextual errors
let contents = std::fs::read_to_string("config.toml")
    .context("Failed to read config.toml")?;
```

#### Systematic Approach
1. **Review** `production_unwraps.txt` and `production_expects.txt`
2. **Categorize** by error type (I/O, parsing, collection, etc.)
3. **Prioritize** critical paths first
4. **Replace** using patterns above
5. **Add tests** for error paths
6. **Verify** error messages are helpful

---

## 📅 WEEKLY EXECUTION PLAN

### Week 1: Foundation (Dec 14-20, 2025)

**Day 1 (Today)**: ✅ Complete
- [x] Run analysis (`./improve.sh all`)
- [x] Establish baseline (960 hardcoded, 3,550 unwraps)
- [x] Review generated target files

**Day 2-3**: Hardcoding Migration (25 instances)
```bash
# Target files to review
head -50 hardcoded_ips.txt
head -50 hardcoded_ports.txt

# Migrate most common patterns first
# Focus on: API endpoints, bind addresses, service URLs
```

**Day 4-5**: Unwrap Replacement (20 instances)
```bash
# Target files to review
head -50 production_unwraps.txt
head -50 production_expects.txt

# Replace critical paths first
# Focus on: initialization, configuration, network setup
```

**Day 6-7**: Testing & Documentation (30 tests)
```bash
# Add error path tests for:
# - Invalid environment variables
# - Missing configuration
# - Network failures
# - File not found errors

# Run tests
cargo test --lib --workspace

# Update progress
./improve.sh report
```

**Week 1 Target**: 
- Migrate 100 hardcoded values (10% of 960)
- Replace 75 unwraps (2% of 3,550)
- Add 50 error path tests
- Document patterns

### Week 2: Acceleration (Dec 21-27, 2025)

**Target**:
- Migrate 150 more hardcoded values (total: 250)
- Replace 100 more unwraps (total: 175)
- Add 75 more tests (total: 125)
- Reach 73-75% coverage

### Week 3: Deep Migration (Dec 28 - Jan 3, 2026)

**Target**:
- Migrate 200 more values (total: 450, ~47%)
- Replace 125 more unwraps (total: 300, ~8%)
- Add 100 more tests (total: 225)
- Reach 76-78% coverage

### Week 4: Milestone Completion (Jan 4-10, 2026)

**Target**:
- Complete 50% hardcoding milestone (480 total)
- Complete unwrap milestone (300 total)
- Add 150 final tests (total: 375)
- Reach 85-90% coverage
- **Achieve A+ grade (95/100)**

---

## 🛠️ DAILY WORKFLOW

### Morning (30 min)
```bash
# Review yesterday's progress
cat EXECUTION_PROGRESS_DEC_14_2025.md

# Pick today's targets
head -10 hardcoded_ips.txt  # or production_unwraps.txt
```

### Afternoon (2-3 hours)
```bash
# Make migrations
# - Edit source files
# - Apply patterns
# - Run tests locally

# Verify
cargo test --lib
cargo clippy
```

### Evening (15 min)
```bash
# Update progress
./improve.sh report

# Commit changes
git add -p
git commit -m "refactor: migrate hardcoded values to capability-based discovery"
# or
git commit -m "refactor: replace unwraps with proper error handling"
```

---

## 📈 PROGRESS TRACKING

### Metrics to Track Weekly

```bash
# Run analysis
./improve.sh hardcoding
./improve.sh unwraps

# Compare counts
echo "Week 1: 593 IPs → $(wc -l < hardcoded_ips.txt) IPs"
echo "Week 1: 1599 unwraps → $(wc -l < production_unwraps.txt) unwraps"
```

### Success Criteria

**Week 1**: ✅ if 100+ values migrated, 75+ unwraps replaced  
**Week 2**: ✅ if 250+ total migrated, 175+ total replaced  
**Week 3**: ✅ if 450+ total migrated, 300+ total replaced  
**Week 4**: ✅ if 480+ total migrated, 300+ total replaced, 85%+ coverage

---

## 💡 MIGRATION PATTERNS LIBRARY

### Pattern 1: Service Discovery
```rust
use nestgate_core::config::capability_based::*;

// Replace ANY hardcoded service endpoint
let config = CapabilityConfigBuilder::new().build()?;
let service = config.discover(PrimalCapability::YourCapability).await?;
let endpoint = service.endpoint;
```

### Pattern 2: Environment Variables
```rust
use nestgate_core::utils::safe_operations::parse_env_var;

// Replace ANY env var unwrap
let value: Type = parse_env_var("VAR_NAME")?;
```

### Pattern 3: Collection Access
```rust
use nestgate_core::utils::safe_operations::SafeCollectionExt;

// Replace ANY collection unwrap
let item = collection.safe_get(key)?;
let first = vec.safe_first()?;
```

### Pattern 4: Option to Result
```rust
// Replace ANY option unwrap
let value = optional
    .ok_or_else(|| anyhow::anyhow!("Descriptive error message"))?;
```

---

## 🎯 QUICK WINS (Do These First)

### Top 10 Hardcoded Values to Migrate
1. API bind addresses (0.0.0.0, 127.0.0.1)
2. Default ports (8080, 3000, 9090)
3. Database URLs (localhost:5432)
4. Service discovery endpoints
5. Health check endpoints
6. Metrics endpoints
7. WebSocket URLs
8. Admin interface addresses
9. Storage service URLs
10. Cache service URLs

### Top 10 Unwraps to Replace
1. Environment variable parsing
2. Configuration file reading
3. JSON/TOML deserialization
4. Network connection establishment
5. File I/O operations
6. Collection access (first, get, etc.)
7. String parsing (parse::<T>())
8. HashMap/BTreeMap lookups
9. Option chaining
10. Service initialization

---

## 📊 AUTOMATION COMMANDS

### Daily Analysis
```bash
./improve.sh hardcoding  # Find remaining hardcoded values
./improve.sh unwraps     # Find remaining unwraps
./improve.sh report      # Generate progress report
```

### Weekly Full Check
```bash
./improve.sh all         # Run all phases
./improve.sh coverage    # Measure test coverage (after tests pass)
```

### Quality Gates
```bash
cargo fmt               # Format code
cargo clippy            # Lint code
cargo test --lib        # Run tests
```

---

## 🎊 CELEBRATION MILESTONES

### Week 1: 🥉 Bronze
- [x] Baseline established
- [ ] 100 values migrated
- [ ] 75 unwraps replaced
- [ ] 50 tests added

### Week 2: 🥈 Silver
- [ ] 250 values migrated (26%)
- [ ] 175 unwraps replaced (5%)
- [ ] 125 tests added

### Week 3: 🥇 Gold
- [ ] 450 values migrated (47%)
- [ ] 300 unwraps replaced (8%)
- [ ] 225 tests added

### Week 4: 🏆 Platinum
- [ ] 480 values migrated (50%)
- [ ] 300+ unwraps replaced
- [ ] 85-90% coverage
- [ ] **A+ GRADE (95/100)**

---

## 🚀 START NOW

### Your First Migration (10 minutes)

```bash
# 1. Pick a file
head -1 hardcoded_ips.txt

# 2. Open the file
# 3. Find the hardcoded value
# 4. Apply migration pattern
# 5. Test locally
cargo test --lib

# 6. Commit
git commit -m "refactor: migrate hardcoded IP to capability discovery"

# 7. Repeat!
```

**You've got 4,510 targets. Start with 1. Then do another. Momentum builds.**

---

**Guide Version**: 1.0  
**Created**: December 14, 2025  
**Baseline**: 960 hardcoded, 3,550 unwraps  
**Target**: 50% migration in 4 weeks

✅ **Analysis complete. Targets identified. Patterns documented. Execute!**

