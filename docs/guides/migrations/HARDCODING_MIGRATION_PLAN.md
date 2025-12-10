# 🔧 HARDCODING MIGRATION PLAN

**Date**: November 27, 2025  
**Priority**: HIGH  
**Timeline**: 10-14 days  
**Impact**: Enables multi-environment deployment

---

## 📊 EXECUTIVE SUMMARY

**Problem**: 1,105+ hardcoded ports, 575+ hardcoded IPs/hostnames, 200-300 other constants  
**Impact**: Cannot deploy to different environments without code changes  
**Solution**: Migrate to environment-driven configuration  
**Good News**: ✅ Configuration infrastructure already exists!

---

## 🎯 AUDIT FINDINGS

### Hardcoded Values Identified

| Category | Count | Severity | Examples |
|----------|-------|----------|----------|
| **Ports** | 1,105+ | HIGH | 8080, 3000, 5432, 6379, 9090, 27017 |
| **IP Addresses** | 575+ | HIGH | 127.0.0.1, localhost |
| **Paths** | 200+ | MEDIUM | /opt/nestgate, /etc/nestgate |
| **Constants** | 100+ | LOW | Timeouts, buffer sizes |

### Distribution by Type

**Ports**:
- `8080` - HTTP API server (most common)
- `3000` - Development server
- `5432` - PostgreSQL
- `6379` - Redis
- `9090` - Prometheus metrics
- `27017` - MongoDB

**Hostnames/IPs**:
- `127.0.0.1` - Local loopback
- `localhost` - Local hostname

---

## ✅ EXISTING INFRASTRUCTURE

### Configuration Files Present

```
config/
├── canonical-master.toml          ✅ Master config template
├── dynamic-config-template.toml   ✅ Dynamic config support
├── environment-variables.example  ✅ Env var examples
├── production_config.toml         ✅ Production defaults
├── production.env.example         ✅ Env file template
└── zero-cost-production.toml      ✅ Zero-cost optimizations
```

### Configuration System in Code

```rust
// Already implemented:
code/crates/nestgate-core/src/config/
├── mod.rs                    // Main config module
├── canonical_primary/        // Canonical config system
├── validation.rs             // Config validation
├── runtime.rs                // Runtime config loading
├── defaults.rs               // Default values
└── external/                 // External config sources
```

---

## 📋 MIGRATION STRATEGY

### Phase 1: Environment Variables (Week 1)

**Priority**: Critical paths first

**Step 1.1: Network Ports** [2-3 days]
```rust
// BEFORE:
let port = 8080;

// AFTER:
let port = env::var("NESTGATE_API_PORT")
    .unwrap_or_else(|_| "8080".to_string())
    .parse::<u16>()
    .unwrap_or(8080);

// BETTER (using existing config):
let port = config.network.api.port; // From canonical config
```

**Files to Update**:
```
code/crates/nestgate-core/src/config/runtime.rs
code/crates/nestgate-core/src/config/port_config.rs
code/crates/nestgate-core/src/constants/port_defaults.rs
code/crates/nestgate-api/src/bin/nestgate-api-server.rs
```

**Environment Variables to Add**:
```bash
NESTGATE_API_PORT=8080
NESTGATE_ADMIN_PORT=9090
NESTGATE_METRICS_PORT=9091
NESTGATE_POSTGRES_PORT=5432
NESTGATE_REDIS_PORT=6379
NESTGATE_MONGODB_PORT=27017
```

**Step 1.2: Hostnames/IPs** [1-2 days]
```rust
// BEFORE:
let host = "127.0.0.1";

// AFTER:
let host = env::var("NESTGATE_API_HOST")
    .unwrap_or_else(|_| "0.0.0.0".to_string());

// BETTER (using existing config):
let host = config.network.api.host; // From canonical config
```

**Environment Variables to Add**:
```bash
NESTGATE_API_HOST=0.0.0.0
NESTGATE_DATABASE_HOST=localhost
NESTGATE_REDIS_HOST=localhost
NESTGATE_PRIMAL_DISCOVERY_HOST=0.0.0.0
```

**Step 1.3: Update Config Files** [1 day]
```toml
# config/production.toml
[network.api]
host = "${NESTGATE_API_HOST:-0.0.0.0}"
port = "${NESTGATE_API_PORT:-8080}"

[database]
host = "${NESTGATE_DATABASE_HOST:-localhost}"
port = "${NESTGATE_DATABASE_PORT:-5432}"

[redis]
host = "${NESTGATE_REDIS_HOST:-localhost}"
port = "${NESTGATE_REDIS_PORT:-6379}"
```

### Phase 2: Config System Integration (Week 2)

**Step 2.1: Centralize Constants** [2-3 days]

Create `config/defaults.rs`:
```rust
pub struct NetworkDefaults {
    pub api_port: u16,
    pub api_host: String,
    pub metrics_port: u16,
    // etc.
}

impl NetworkDefaults {
    pub fn from_env() -> Self {
        Self {
            api_port: env::var("NESTGATE_API_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8080),
            api_host: env::var("NESTGATE_API_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            metrics_port: env::var("NESTGATE_METRICS_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(9090),
        }
    }
}
```

**Step 2.2: Update All References** [3-4 days]

Use grep to find all instances:
```bash
# Find all hardcoded ports
rg "\b(8080|3000|5432|6379|9090|27017)\b" --type rust code/ > hardcoded_ports.txt

# Find all hardcoded IPs
rg "(127\.0\.0\.1|localhost)" --type rust code/ > hardcoded_ips.txt

# Systematically replace each instance
```

**Step 2.3: Testing** [1-2 days]

Test with different configurations:
```bash
# Test with default config
cargo test --workspace

# Test with custom ports
export NESTGATE_API_PORT=9999
cargo test --workspace

# Test with production config
export NESTGATE_ENV=production
cargo test --workspace
```

### Phase 3: Documentation & Validation (Week 2)

**Step 3.1: Document Environment Variables**

Update `config/environment-variables.example`:
```bash
# Network Configuration
NESTGATE_API_HOST=0.0.0.0
NESTGATE_API_PORT=8080
NESTGATE_METRICS_PORT=9090

# Database Configuration  
NESTGATE_DATABASE_HOST=localhost
NESTGATE_DATABASE_PORT=5432

# Redis Configuration
NESTGATE_REDIS_HOST=localhost
NESTGATE_REDIS_PORT=6379

# Primal Discovery
NESTGATE_PRIMAL_DISCOVERY_HOST=0.0.0.0
NESTGATE_PRIMAL_DISCOVERY_PORT=5000
```

**Step 3.2: Update Deployment Guides**

- Update `PRODUCTION_DEPLOYMENT_GUIDE.md`
- Update `CONFIGURATION_GUIDE.md`
- Add examples for Docker, Kubernetes
- Add troubleshooting section

**Step 3.3: Validation**

Run automated checks:
```bash
# Verify no hardcoded values remain
./scripts/check-hardcoding.sh

# Verify config loading works
./scripts/validate-config.sh

# Verify all environments
./scripts/test-all-environments.sh
```

---

## 🎯 PRIORITIZED FILE LIST

### Tier 1: Critical (Do First) [3-4 days]

**Network Configuration**:
```
code/crates/nestgate-core/src/config/port_config.rs
code/crates/nestgate-core/src/config/runtime.rs
code/crates/nestgate-core/src/constants/port_defaults.rs
code/crates/nestgate-core/src/constants/network_defaults.rs
code/crates/nestgate-api/src/bin/nestgate-api-server.rs
```

**Service Discovery**:
```
code/crates/nestgate-core/src/service_discovery/
code/crates/nestgate-core/src/universal_adapter/
code/crates/nestgate-core/src/infant_discovery/
```

### Tier 2: Important (Do Second) [3-4 days]

**Database & Cache**:
```
code/crates/nestgate-core/src/config/external/services_config.rs
code/crates/nestgate-core/src/ecosystem_integration/ecosystem_config.rs
```

**Testing Infrastructure**:
```
tests/common/test_environment.rs
tests/integration/config_tests.rs
```

### Tier 3: Nice-to-Have (Do Last) [2-3 days]

**Documentation & Examples**:
```
examples/*.rs
docs/examples/
```

**Development Helpers**:
```
code/crates/nestgate-core/src/smart_abstractions/smart_default.rs
```

---

## 🛠️ IMPLEMENTATION HELPERS

### Script 1: Find & Replace Helper

```bash
#!/bin/bash
# scripts/migrate-hardcoding.sh

# Find all files with hardcoded port 8080
echo "Files with hardcoded port 8080:"
rg "\b8080\b" --type rust code/ -l

# Generate replacement commands
echo ""
echo "Suggested replacements:"
rg "\b8080\b" --type rust code/ -n | \
    awk -F: '{print "# " $1 ":" $2 "\n# Replace: " $3}'
```

### Script 2: Config Validator

```bash
#!/bin/bash
# scripts/validate-config.sh

# Check all required env vars are documented
required_vars=(
    "NESTGATE_API_HOST"
    "NESTGATE_API_PORT"
    "NESTGATE_DATABASE_HOST"
    "NESTGATE_DATABASE_PORT"
)

for var in "${required_vars[@]}"; do
    if ! grep -q "$var" config/environment-variables.example; then
        echo "❌ Missing documentation for: $var"
    else
        echo "✅ Documented: $var"
    fi
done
```

### Script 3: Automated Migration

```bash
#!/bin/bash
# scripts/auto-migrate-ports.sh

# Backup files
git add -A
git commit -m "Backup before hardcoding migration"

# Replace common patterns
find code/ -name "*.rs" -type f -exec sed -i \
    's/let port = 8080;/let port = config.network.api.port;/g' {} \;

# Verify changes
git diff --stat
```

---

## ✅ SUCCESS CRITERIA

### Must Have
- ✅ Zero hardcoded ports in production code
- ✅ Zero hardcoded IPs in production code
- ✅ All config loaded from environment/files
- ✅ Tests pass with custom ports
- ✅ Docker/K8s deployments work

### Should Have
- ✅ Comprehensive env var documentation
- ✅ Configuration validation
- ✅ Migration guide for operators
- ✅ Automated config checks

### Nice to Have
- ✅ Config hot-reloading
- ✅ Config versioning
- ✅ Config diff tools

---

## 📊 EFFORT ESTIMATION

| Phase | Tasks | Days | Developer Hours |
|-------|-------|------|----------------|
| **Phase 1** | Env vars & ports | 4-6 | 32-48 |
| **Phase 2** | Config integration | 5-7 | 40-56 |
| **Phase 3** | Docs & validation | 1-2 | 8-16 |
| **TOTAL** | **All phases** | **10-15** | **80-120** |

**Realistic Timeline**: 10-14 days with 1 developer (6-8 hours/day)

---

## 🚨 RISKS & MITIGATION

### Risk 1: Breaking Changes
**Mitigation**: 
- Keep defaults matching current hardcoded values
- Extensive testing before merging
- Gradual rollout (dev → staging → production)

### Risk 2: Config Complexity
**Mitigation**:
- Good documentation
- Validation tools
- Sensible defaults

### Risk 3: Test Pollution
**Mitigation**:
- Isolate test environments
- Reset env vars between tests
- Use test fixtures

---

## 📋 CHECKLIST

### Pre-Migration
- [ ] Review existing config system
- [ ] Identify all hardcoded values
- [ ] Create backup branch
- [ ] Set up test environments

### During Migration
- [ ] Phase 1: Environment variables (Week 1)
  - [ ] Network ports migrated
  - [ ] Hostnames/IPs migrated
  - [ ] Config files updated
- [ ] Phase 2: Config integration (Week 2)
  - [ ] Constants centralized
  - [ ] All references updated
  - [ ] Testing complete
- [ ] Phase 3: Documentation (Week 2)
  - [ ] Env vars documented
  - [ ] Deployment guides updated
  - [ ] Validation scripts created

### Post-Migration
- [ ] All tests passing
- [ ] Config validation passing
- [ ] Documentation complete
- [ ] Deployed to staging
- [ ] Deployed to production

---

## 🎯 QUICK START

**Day 1**: 
```bash
# 1. Create feature branch
git checkout -b feature/config-migration

# 2. Run audit
./scripts/audit-hardcoding.sh

# 3. Start with network ports
# Focus on: code/crates/nestgate-core/src/config/port_config.rs
```

**Days 2-5**: Systematically migrate Tier 1 files  
**Days 6-10**: Migrate Tier 2 files  
**Days 11-12**: Documentation & validation  
**Days 13-14**: Testing & deployment

---

**Status**: ✅ PLAN READY  
**Next Action**: Create feature branch and start Phase 1  
**Owner**: TBD  
**Timeline**: 10-14 days  
**Priority**: HIGH

---

*"Configuration shouldn't be code. Make it data!"* 🔧

