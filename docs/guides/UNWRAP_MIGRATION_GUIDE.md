# Unwrap Migration Guide - NestGate
**Date**: November 19, 2025  
**Goal**: Eliminate all .unwrap() calls with intentional error handling

---

## 🎯 Migration Strategy

### Production Code Rules
- ✅ **NEVER** use `.unwrap()` in production code
- ✅ **USE** `.expect("descriptive message")` ONLY for infallible operations
- ✅ **PREFER** proper Result<T, E> propagation with `?`
- ✅ **USE** pattern matching for complex error handling

### Test Code Rules
- ⚠️ **AVOID** `.unwrap()` even in tests
- ✅ **USE** `.expect("Test setup: description")` with context
- ✅ **USE** `.unwrap_or_else(|e| panic!("..."))`  for better error messages
- ✅ **CONSIDER** `assert!()` with `.is_ok()` for better test output

---

## 📋 Migration Patterns

### Pattern 1: Result Propagation (Best)
```rust
// ❌ BEFORE
fn process_data(input: &str) -> String {
    let parsed = input.parse::<u32>().unwrap();
    format!("Value: {}", parsed)
}

// ✅ AFTER
fn process_data(input: &str) -> Result<String, ParseIntError> {
    let parsed = input.parse::<u32>()?;
    Ok(format!("Value: {}", parsed))
}
```

### Pattern 2: Option with Context
```rust
// ❌ BEFORE
let value = map.get("key").unwrap();

// ✅ AFTER (Production)
let value = map.get("key")
    .ok_or_else(|| Error::MissingKey("key".to_string()))?;

// ✅ AFTER (Test)
let value = map.get("key")
    .expect("Test setup: 'key' should exist in map");
```

### Pattern 3: Infallible Operations
```rust
// ❌ BEFORE
let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

// ✅ AFTER
let time = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("System time should be after UNIX epoch");
```

### Pattern 4: Default Values
```rust
// ❌ BEFORE
let config = env::var("CONFIG").unwrap();

// ✅ AFTER
let config = env::var("CONFIG")
    .unwrap_or_else(|_| DEFAULT_CONFIG.to_string());
```

### Pattern 5: Test Assertions
```rust
// ❌ BEFORE (in tests)
assert_eq!(result.compression.unwrap(), "lz4");

// ✅ AFTER
assert_eq!(
    result.compression.as_ref().expect("compression should be set"),
    "lz4"
);

// ✅ BETTER
assert!(result.compression.is_some(), "compression should be set");
assert_eq!(result.compression.as_ref().unwrap(), "lz4");
```

---

## 🔧 Migration Priority

### Priority 1: Production Code (555 instances)
1. API handlers
2. Core business logic
3. Database operations
4. Network operations
5. File I/O operations

### Priority 2: Test Code (2000 instances)
1. Test setup code
2. Test assertions
3. Test fixtures
4. Mock data creation

---

## 📊 Tracking Progress

### By Module
- [ ] nestgate-api/handlers: ~200 unwraps
- [ ] nestgate-zfs/native: ~150 unwraps
- [ ] nestgate-core/services: ~100 unwraps
- [ ] nestgate-network: ~50 unwraps
- [ ] nestgate-mcp: ~30 unwraps
- [ ] Others: ~25 unwraps

### By Type
- [ ] String/parsing unwraps: ~150
- [ ] Option unwraps: ~200
- [ ] Time/system unwraps: ~50
- [ ] Config unwraps: ~80
- [ ] Serialization unwraps: ~75

---

## 🚀 Execution Plan

### Week 1: Critical Production Code
- [ ] Fix all API handler unwraps
- [ ] Fix all database operation unwraps
- [ ] Fix all network operation unwraps
- **Target**: 555 → 300 production unwraps

### Week 2: Remaining Production Code
- [ ] Fix core business logic unwraps
- [ ] Fix file I/O unwraps
- [ ] Fix configuration unwraps
- **Target**: 300 → 50 production unwraps

### Week 3-4: Test Code Migration
- [ ] Migrate test setup unwraps → expect
- [ ] Migrate test assertions → expect
- [ ] Improve test error messages
- **Target**: 2000 → 0 test unwraps

---

## 📝 Examples from Codebase

### Example 1: dataset_manager.rs
```rust
// ❌ CURRENT (test code)
assert_eq!(options.compression.as_ref().unwrap(), "lz4");

// ✅ IMPROVED
assert_eq!(
    options.compression.as_ref()
        .expect("Test: compression option should be set"),
    "lz4"
);
```

### Example 2: orchestrator_integration.rs
```rust
// ✅ ALREADY GOOD (has descriptive message)
.duration_since(std::time::UNIX_EPOCH)
.expect("System time should be after UNIX epoch")
```

---

## 🎓 Error Types to Use

### Create Custom Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum DatasetError {
    #[error("Missing required option: {0}")]
    MissingOption(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
}
```

### Use in Code
```rust
fn parse_option(value: Option<String>) -> Result<u32, DatasetError> {
    value
        .ok_or_else(|| DatasetError::MissingOption("value".to_string()))?
        .parse()
        .map_err(DatasetError::ParseError)
}
```

---

## ✅ Verification

### Commands
```bash
# Count production unwraps
find code/crates -name "*.rs" -not -path "*/tests/*" -not -name "*_test*.rs" \
  -exec grep -l "\.unwrap()" {} \; | wc -l

# Count test unwraps
find code/crates -name "*.rs" -path "*/tests/*" -o -name "*_test*.rs" \
  -exec grep -c "\.unwrap()" {} \; | awk '{s+=$1} END {print s}'

# Find files with most unwraps
find code/crates -name "*.rs" -exec grep -c "\.unwrap()\|\.expect(" {} + \
  | sort -t: -k2 -rn | head -20
```

---

## 🎯 Success Criteria

- [ ] Zero `.unwrap()` in production code
- [ ] All `.expect()` have descriptive messages
- [ ] All errors properly propagated
- [ ] Tests use `.expect()` with context
- [ ] Error messages are actionable

---

**Last Updated**: November 19, 2025  
**Status**: Migration in progress


