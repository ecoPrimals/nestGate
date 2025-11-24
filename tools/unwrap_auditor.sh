#!/bin/bash
# Unwrap Auditor - Categorize unwrap/expect calls by context
# Created: November 19, 2025

set -euo pipefail

WORKSPACE_ROOT="${1:-$(pwd)}"
OUTPUT_FILE="${2:-UNWRAP_AUDIT_DETAILED_NOV_19_2025.md}"

echo "🔍 Unwrap Auditor - Comprehensive Analysis"
echo "=========================================="
echo ""
echo "Workspace: $WORKSPACE_ROOT"
echo "Output: $OUTPUT_FILE"
echo ""

cd "$WORKSPACE_ROOT"

# Find all unwrap/expect calls
echo "📊 Scanning codebase..."

# Count by category
TOTAL=$(rg "\.unwrap\(\)|\.expect\(" --type rust code/crates/ | wc -l)
TEST_FILES=$(rg "\.unwrap\(\)|\.expect\(" --type rust code/crates/ -g "*test*.rs" -g "*tests/*.rs" | wc -l)
PROD_FILES=$((TOTAL - TEST_FILES))

echo "Total unwraps found: $TOTAL"
echo "In test files: $TEST_FILES"
echo "In production code: $PROD_FILES"
echo ""

# Generate detailed report
cat > "$OUTPUT_FILE" << 'EOF'
# 🔍 UNWRAP AUDIT - Detailed Analysis
**Generated**: $(date)  
**Total Found**: TOTAL_PLACEHOLDER unwraps/expects

---

## 📊 SUMMARY

| Category | Count | Percentage | Risk Level |
|----------|-------|------------|------------|
| **Test Files** | TEST_PLACEHOLDER | TEST_PCT% | ✅ LOW |
| **Production Code** | PROD_PLACEHOLDER | PROD_PCT% | ⚠️ HIGH |

---

## 🎯 PRODUCTION CODE UNWRAPS (HIGH PRIORITY)

### By Crate

EOF

# Replace placeholders
sed -i "s/TOTAL_PLACEHOLDER/$TOTAL/g" "$OUTPUT_FILE"
sed -i "s/TEST_PLACEHOLDER/$TEST_FILES/g" "$OUTPUT_FILE"
sed -i "s/PROD_PLACEHOLDER/$PROD_FILES/g" "$OUTPUT_FILE"

TEST_PCT=$((TEST_FILES * 100 / TOTAL))
PROD_PCT=$((PROD_FILES * 100 / TOTAL))

sed -i "s/TEST_PCT/$TEST_PCT/g" "$OUTPUT_FILE"
sed -i "s/PROD_PCT/$PROD_PCT/g" "$OUTPUT_FILE"

# Analyze by crate
echo "#### Production Unwraps by Crate" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "\`\`\`" >> "$OUTPUT_FILE"

for crate_dir in code/crates/*/; do
    crate_name=$(basename "$crate_dir")
    
    # Count production unwraps (exclude test files)
    prod_count=$(rg "\.unwrap\(\)|\.expect\(" --type rust "$crate_dir" \
        -g "!*test*.rs" -g "!tests/" -g "!benches/" -g "!examples/" 2>/dev/null | wc -l || echo "0")
    
    if [ "$prod_count" -gt 0 ]; then
        echo "$crate_name: $prod_count unwraps" >> "$OUTPUT_FILE"
    fi
done

echo "\`\`\`" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Find critical patterns
echo "### 🚨 CRITICAL PATTERNS (Production Code)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Look for unwraps in error handling
echo "#### Unwraps in Error Handling" >> "$OUTPUT_FILE"
echo "\`\`\`" >> "$OUTPUT_FILE"
rg "Result.*\.unwrap\(\)" --type rust code/crates/ -g "!*test*.rs" -g "!tests/" -n | head -20 >> "$OUTPUT_FILE" || echo "None found" >> "$OUTPUT_FILE"
echo "\`\`\`" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Look for unwraps in async code
echo "#### Unwraps in Async Code" >> "$OUTPUT_FILE"
echo "\`\`\`" >> "$OUTPUT_FILE"
rg "await.*\.unwrap\(\)" --type rust code/crates/ -g "!*test*.rs" -g "!tests/" -n | head -20 >> "$OUTPUT_FILE" || echo "None found" >> "$OUTPUT_FILE"
echo "\`\`\`" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Categorize expect messages
echo "### 📝 EXPECT MESSAGES (Context Analysis)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "#### Production Expects with Messages" >> "$OUTPUT_FILE"
echo "\`\`\`" >> "$OUTPUT_FILE"
rg "\.expect\(" --type rust code/crates/ -g "!*test*.rs" -g "!tests/" -A 1 | head -50 >> "$OUTPUT_FILE" || echo "None found" >> "$OUTPUT_FILE"
echo "\`\`\`" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Generate recommendations
cat >> "$OUTPUT_FILE" << 'EOFEND'

---

## 🎯 MIGRATION PRIORITIES

### P0 - Critical (Fix Immediately)
- [ ] Unwraps in error handling paths
- [ ] Unwraps in async code without timeout
- [ ] Unwraps in network/IO operations
- [ ] Unwraps in configuration loading

### P1 - High Priority (Fix This Week)
- [ ] Unwraps in API handlers
- [ ] Unwraps in data processing
- [ ] Unwraps in storage operations
- [ ] Unwraps with no context

### P2 - Medium Priority (Fix This Sprint)
- [ ] Expects with vague messages
- [ ] Unwraps in initialization code
- [ ] Unwraps in utility functions

### P3 - Low Priority (Document & Review)
- [ ] Unwraps in test code (acceptable)
- [ ] Expects with clear context
- [ ] Justified unwraps (document why)

---

## 🔧 MIGRATION PATTERNS

### Pattern 1: Option → Result with Context
```rust
// Before
let value = map.get(&key).unwrap();

// After
let value = map.get(&key)
    .ok_or_else(|| anyhow!("Missing required key: {}", key))?;
```

### Pattern 2: Result → Propagate with Context
```rust
// Before
let data = serde_json::from_str(&json).unwrap();

// After
let data = serde_json::from_str(&json)
    .context("Failed to parse JSON configuration")?;
```

### Pattern 3: Async Operations
```rust
// Before
let response = client.request().await.unwrap();

// After
let response = tokio::time::timeout(
    Duration::from_secs(5),
    client.request()
).await
    .context("Request timed out")?
    .context("Request failed")?;
```

### Pattern 4: Initialization
```rust
// Before
static CONFIG: Lazy<Config> = Lazy::new(|| {
    load_config().unwrap()
});

// After
static CONFIG: Lazy<Result<Config>> = Lazy::new(|| {
    load_config().context("Failed to load config")
});

// Usage
let config = CONFIG.as_ref()
    .map_err(|e| anyhow!("Config error: {}", e))?;
```

---

## 📋 ACTION PLAN

### Week 1: Critical Path
1. Identify all P0 unwraps
2. Add proper error types
3. Migrate critical paths
4. Add integration tests

### Week 2: API Layer
1. Review all API handlers
2. Add error context
3. Improve error messages
4. Update documentation

### Week 3: Storage & Network
1. Audit storage operations
2. Add timeout patterns
3. Implement retry logic
4. Test error scenarios

### Week 4: Completion
1. Review remaining unwraps
2. Document justified cases
3. Add linting rules
4. Update team guidelines

---

## 🎯 SUCCESS METRICS

### Targets
- **P0 Unwraps**: 0 (zero tolerance)
- **P1 Unwraps**: <10 (with justification)
- **Production Unwraps**: <50 total (documented)
- **Test Unwraps**: OK (no change needed)

### Current Baseline
- Total: TOTAL_BASELINE
- Production: PROD_BASELINE
- Tests: TEST_BASELINE

---

**Generated by**: `tools/unwrap_auditor.sh`  
**Next Review**: Weekly until completion  
**Owner**: Engineering team
EOFEND

# Final replacements
sed -i "s/TOTAL_BASELINE/$TOTAL/g" "$OUTPUT_FILE"
sed -i "s/PROD_BASELINE/$PROD_FILES/g" "$OUTPUT_FILE"
sed -i "s/TEST_BASELINE/$TEST_FILES/g" "$OUTPUT_FILE"

echo "✅ Audit complete!"
echo "📄 Report saved to: $OUTPUT_FILE"
echo ""
echo "📊 Summary:"
echo "  Total: $TOTAL"
echo "  Production: $PROD_FILES ($PROD_PCT%)"
echo "  Tests: $TEST_FILES ($TEST_PCT%)"
echo ""
echo "🎯 Next: Review $OUTPUT_FILE and prioritize P0 unwraps"

