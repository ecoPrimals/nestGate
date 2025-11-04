# 📚 API Documentation Guide - NestGate

**Purpose**: Standard template for documenting public APIs  
**Scope**: 45-60 functions need documentation additions  
**Estimated Effort**: 15-20 hours (systematic completion)  
**Priority**: MEDIUM

---

## 🎯 **WHAT'S NEEDED**

### **Missing Documentation Sections**
1. **`# Errors`** - Document all error conditions
2. **`# Panics`** - Document any panic conditions
3. **`# Examples`** - Provide usage examples

### **Current Status**
- ✅ Many functions have basic documentation
- ⚠️ ~45-60 functions missing `# Errors` sections
- ⚠️ ~45-60 functions missing `# Panics` sections
- ⚠️ ~30-40 functions missing `# Examples` sections

---

## 📋 **DOCUMENTATION TEMPLATE**

### **Standard Function Documentation**

```rust
/// Brief one-line description of what the function does
///
/// Longer description with more context about:
/// - What the function accomplishes
/// - When to use it
/// - Any important behavior notes
///
/// # Arguments
///
/// * `param1` - Description of first parameter
/// * `param2` - Description of second parameter
///
/// # Returns
///
/// Returns `Result<T, E>` where:
/// - `Ok(T)` - Description of success case
/// - `Err(E)` - Brief error summary (details in Errors section)
///
/// # Errors
///
/// This function will return an error if:
/// - Specific condition 1 causes `ErrorType::Variant1`
/// - Specific condition 2 causes `ErrorType::Variant2`
/// - Network/IO errors occur
///
/// # Panics
///
/// This function will panic if:
/// - Specific unsafe condition is violated
/// - Internal invariant is broken
///
/// Note: If function doesn't panic, write "This function does not panic."
///
/// # Examples
///
/// Basic usage:
/// ```rust
/// # use nestgate_core::*;
/// # async fn example() -> Result<()> {
/// let result = function_name(param1, param2).await?;
/// assert_eq!(result, expected_value);
/// # Ok(())
/// # }
/// ```
///
/// Advanced usage:
/// ```rust
/// # use nestgate_core::*;
/// # async fn example() -> Result<()> {
/// // More complex example showing edge cases
/// let config = Config::builder()
///     .with_option(value)
///     .build()?;
/// let result = function_name_with_config(config).await?;
/// # Ok(())
/// # }
/// ```
pub async fn function_name(param1: Type1, param2: Type2) -> Result<ReturnType> {
    // Implementation
}
```

---

## 🎯 **PRIORITY AREAS**

### **HIGH PRIORITY** (Complete First)
1. **Public API Handlers** (`nestgate-api/src/handlers/`)
   - REST endpoints
   - WebSocket handlers
   - RPC methods

2. **Core Capabilities** (`nestgate-core/src/`)
   - Universal adapter methods
   - Infant discovery functions
   - Storage operations

3. **Security Functions** (`nestgate-core/src/security/`)
   - Authentication methods
   - Authorization checks
   - Validation functions

### **MEDIUM PRIORITY**
4. **Network Operations** (`nestgate-network/`)
   - Connection methods
   - Protocol handlers

5. **ZFS Operations** (`nestgate-zfs/`)
   - Pool management
   - Dataset operations

### **LOW PRIORITY**
6. **Internal Utilities**
   - Helper functions
   - Internal types

---

## 📝 **REAL EXAMPLES**

### **Example 1: Good Documentation** ✅

```rust
/// Discover capabilities through infant discovery pattern
///
/// Performs zero-knowledge capability discovery without vendor assumptions.
/// Each primal discovers others dynamically through multiple methods:
/// environment variables, network scan, service announcements, and queries.
///
/// # Returns
///
/// Returns `Ok(Vec<CapabilityInfo>)` containing all discovered capabilities,
/// or `Err` if the discovery process encounters a fatal error.
///
/// # Errors
///
/// This function will return an error if:
/// - All discovery methods fail (no capabilities found)
/// - Critical system resources are unavailable
/// - Invalid configuration prevents discovery
///
/// Note: Individual discovery method failures are logged but don't cause
/// the entire operation to fail.
///
/// # Panics
///
/// This function does not panic under normal operation.
///
/// # Examples
///
/// Basic capability discovery:
/// ```rust
/// # use nestgate_core::discovery::InfantDiscovery;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut discovery = InfantDiscovery::new();
/// let capabilities = discovery.discover_capabilities().await?;
/// 
/// for cap in capabilities {
///     println!("Found: {} at {}", cap.capability_type, cap.endpoint);
/// }
/// # Ok(())
/// # }
/// ```
///
/// Filtering discovered capabilities:
/// ```rust
/// # use nestgate_core::discovery::InfantDiscovery;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut discovery = InfantDiscovery::new();
/// let capabilities = discovery.discover_capabilities().await?;
/// 
/// // Find orchestration capabilities
/// let orchestration = capabilities.iter()
///     .filter(|c| c.capability_type == "orchestration")
///     .collect::<Vec<_>>();
/// # Ok(())
/// # }
/// ```
pub async fn discover_capabilities(&mut self) -> Result<Vec<CapabilityInfo>> {
    // Implementation
}
```

### **Example 2: Error Documentation** ✅

```rust
/// Validates request input against security patterns
///
/// Checks for SQL injection, XSS, and other security threats.
///
/// # Arguments
///
/// * `method` - HTTP method (GET, POST, etc.)
/// * `path` - Request path
/// * `headers` - Request headers
/// * `body` - Request body (optional)
///
/// # Returns
///
/// Returns `Ok(ValidationResult)` if validation completes, with
/// `is_valid` indicating whether the request passed validation.
///
/// # Errors
///
/// This function will return an error if:
/// - Regex compilation fails (should not occur with valid patterns)
/// - Invalid UTF-8 in request body
/// - Header parsing fails
///
/// # Panics
///
/// This function does not panic. Regex patterns are validated at
/// initialization time with `.expect()`.
///
/// # Examples
///
/// ```rust
/// # use nestgate_core::security::RequestValidator;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let validator = RequestValidator::new(config);
/// let headers = HashMap::new();
/// 
/// let result = validator
///     .validate_request("POST", "/api/data", &headers, Some(body))
///     .await?;
///     
/// if !result.is_valid {
///     return Err("Request failed validation");
/// }
/// # Ok(())
/// # }
/// ```
pub async fn validate_request(
    &self,
    method: &str,
    path: &str,
    headers: &HashMap<String, String>,
    body: Option<&[u8]>,
) -> Result<ValidationResult> {
    // Implementation
}
```

---

## 🔍 **HOW TO FIND UNDOCUMENTED FUNCTIONS**

### **Method 1: Cargo Doc Warnings**
```bash
cargo doc --no-deps --document-private-items 2>&1 | grep warning
```

### **Method 2: Manual Search**
```bash
# Find public functions without Examples section
rg "pub (async )?fn" -A 20 code/crates/ | grep -v "# Examples"

# Find public functions without Errors section  
rg "pub (async )?fn.*Result" -A 20 code/crates/ | grep -v "# Errors"
```

### **Method 3: Systematic Review**
Review each public API file:
1. `nestgate-api/src/handlers/` - All handlers
2. `nestgate-core/src/universal_adapter/mod.rs` - Adapter methods
3. `nestgate-core/src/discovery/` - Discovery functions
4. `nestgate-core/src/security/` - Security functions

---

## ✅ **COMPLETION CHECKLIST**

### **Per Function**
- [ ] Has one-line summary
- [ ] Has detailed description
- [ ] Lists all parameters (if any)
- [ ] Documents return value
- [ ] Has `# Errors` section (if returns Result)
- [ ] Has `# Panics` section (or states "does not panic")
- [ ] Has at least one `# Examples` section
- [ ] Example compiles (use `# ` for hidden lines)
- [ ] Example is realistic and useful

### **Per Module**
- [ ] All public functions documented
- [ ] Module-level documentation exists
- [ ] Examples cover common use cases
- [ ] Integration with other modules explained

---

## 📊 **PROGRESS TRACKING**

### **Template**
```markdown
### Documentation Progress - [Date]

**Files Completed**: X/Y
**Functions Documented**: X/~45-60

**Completed**:
- [x] `path/to/file.rs` - All public functions
- [x] `path/to/another.rs` - All public functions

**In Progress**:
- [ ] `path/to/file.rs` - 5/10 functions
  - [x] function1
  - [x] function2
  - [ ] function3 (needs Examples)

**Remaining**:
- [ ] `path/to/file.rs` - Not started
```

---

## 🎯 **QUICK WINS** (Start Here)

### **1. Universal Adapter** (nestgate-core/src/universal_adapter/mod.rs)
**Functions needing docs**: ~8-10
**Impact**: HIGH - Core public API
**Effort**: 2-3 hours

### **2. Infant Discovery** (nestgate-core/src/discovery/infant_discovery.rs)  
**Functions needing docs**: ~6-8
**Impact**: HIGH - Revolutionary feature
**Effort**: 1-2 hours

### **3. Security Validation** (nestgate-core/src/security/production_hardening/validation.rs)
**Functions needing docs**: ~5-7
**Impact**: HIGH - Security critical
**Effort**: 1-2 hours

### **4. API Handlers** (nestgate-api/src/handlers/)
**Functions needing docs**: ~20-25
**Impact**: MEDIUM - User-facing
**Effort**: 4-6 hours

---

## 💡 **TIPS & BEST PRACTICES**

### **Writing Good Error Documentation**
✅ **Good**: "Returns `NetworkError::Timeout` if connection takes >30s"  
❌ **Bad**: "Returns error if something goes wrong"

### **Writing Good Examples**
✅ **Good**: Complete, runnable example with setup and teardown  
❌ **Bad**: Code snippet that won't compile

### **Panics Documentation**
✅ **Good**: "Panics if index > len() or buffer is null"  
✅ **Also Good**: "This function does not panic"  
❌ **Bad**: Omitting the section entirely

---

## 📚 **RESOURCES**

### **Rust Documentation Guidelines**
- [The Rust Book - Documentation](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments)
- [RFC 1574 - API Documentation Conventions](https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html)

### **NestGate Specific**
- Review existing well-documented functions for style
- Check `discover_capabilities()` for good example
- Security functions show error documentation style

---

## 🚀 **GETTING STARTED**

### **Step 1: Choose Priority Area**
Pick from Quick Wins list above

### **Step 2: Review Existing Code**
Read the function implementation to understand:
- What it does
- What errors it can return
- Whether it can panic
- How it's meant to be used

### **Step 3: Write Documentation**
Use the template above and real examples as reference

### **Step 4: Test Examples**
Ensure example code compiles:
```bash
cargo test --doc
```

### **Step 5: Review**
Check that documentation answers:
- What does this do?
- How do I use it?
- What can go wrong?
- What's a realistic example?

---

## 📝 **ESTIMATED TIME**

**Per Function**: 15-25 minutes  
**Total (45-60 functions)**: 15-20 hours

**Breakdown**:
- Read & understand: 5 min
- Write Errors section: 3-5 min
- Write Panics section: 2-3 min
- Write Examples: 5-10 min
- Test & review: 3-5 min

**Recommendations**:
- Work in 2-3 hour blocks
- Complete one module at a time
- Have someone review before merging

---

## ✅ **COMPLETION CRITERIA**

Documentation is complete when:
1. All public API functions have complete docs
2. All examples compile with `cargo test --doc`
3. No `missing_docs` warnings in `cargo doc`
4. Examples cover common use cases
5. Error conditions are clearly documented
6. Panic conditions are documented (or marked as panic-free)

---

**Status**: ✅ Template and guide complete  
**Next**: Systematic documentation of 45-60 functions  
**Estimate**: 15-20 hours (can be parallelized across team)

---

*Use this guide to maintain consistent, high-quality API documentation across NestGate.*

