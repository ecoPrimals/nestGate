# 🛡️ NestGate Error Handling Patterns
**Version**: 1.0  
**Date**: November 23, 2025  
**Status**: ✅ Production Standard

---

## 🎯 Philosophy

**Never Panic in Production Code**

Our error handling follows these principles:
1. **Explicit Over Implicit**: All errors should be represented in types
2. **Recoverable By Default**: Use `Result<T>` for fallible operations
3. **Context-Rich**: Errors should provide actionable information
4. **Type-Safe**: Leverage Rust's type system for correctness

---

## 📚 Pattern Library

### **Pattern 1: Basic Result<T> Usage**

✅ **DO**:
```rust
use crate::{Result, NestGateError};

pub fn parse_config(input: &str) -> Result<Config> {
    let config = serde_json::from_str(input)
        .map_err(|e| NestGateError::validation_error(&format!("Invalid config: {}", e)))?;
    Ok(config)
}
```

❌ **DON'T**:
```rust
pub fn parse_config(input: &str) -> Config {
    serde_json::from_str(input).unwrap() // NEVER in production code
}
```

---

### **Pattern 2: Lock Poisoning Handling**

✅ **DO** (Use Safe Abstractions):
```rust
use crate::safe_operations::mutexes::{safe_mutex_lock, safe_mutex_read, safe_mutex_write};

pub fn get_data(&self) -> Result<Data> {
    let guard = safe_mutex_read(&self.data)?;
    Ok(guard.clone())
}
```

❌ **DON'T**:
```rust
pub fn get_data(&self) -> Data {
    self.data.read().unwrap().clone() // Panics on poison
}
```

---

### **Pattern 3: Network Operations**

✅ **DO**:
```rust
use crate::utils::network;

pub fn validate_endpoint(endpoint: &str) -> Result<IpAddr> {
    network::parse_ip(endpoint) // Returns Result<IpAddr>
}
```

❌ **DON'T**:
```rust
pub fn validate_endpoint(endpoint: &str) -> IpAddr {
    endpoint.parse::<IpAddr>().unwrap()
}
```

---

### **Pattern 4: Configuration Loading**

✅ **DO**:
```rust
pub fn load_config() -> Result<Config> {
    let config = Config::from_env()
        .or_else(|_| Config::from_file("config.toml"))
        .or_else(|_| Ok(Config::default()))?;
    Ok(config)
}
```

❌ **DON'T**:
```rust
pub fn load_config() -> Config {
    Config::from_env().unwrap_or_else(|_| {
        Config::from_file("config.toml").unwrap_or_else(|_| {
            Config::default()
        })
    })
}
```

---

### **Pattern 5: Builder Pattern (Fallible)**

✅ **DO**:
```rust
pub struct ServiceBuilder {
    name: Option<String>,
    endpoint: Option<String>,
}

impl ServiceBuilder {
    pub fn build(self) -> Result<Service> {
        Ok(Service {
            name: self.name.ok_or_else(|| 
                NestGateError::validation_error("Service name required"))?,
            endpoint: self.endpoint.ok_or_else(|| 
                NestGateError::validation_error("Service endpoint required"))?,
        })
    }
}
```

❌ **DON'T**:
```rust
impl ServiceBuilder {
    pub fn build(self) -> Service {
        Service {
            name: self.name.expect("Service name required"),
            endpoint: self.endpoint.expect("Service endpoint required"),
        }
    }
}
```

---

### **Pattern 6: File Operations**

✅ **DO**:
```rust
use std::fs;

pub fn read_config_file(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .map_err(|e| NestGateError::io_error(&format!("Failed to read {}: {}", path.display(), e)))
}
```

❌ **DON'T**:
```rust
pub fn read_config_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}
```

---

### **Pattern 7: Option Handling**

✅ **DO**:
```rust
pub fn get_required_value(&self, key: &str) -> Result<String> {
    self.values.get(key)
        .cloned()
        .ok_or_else(|| NestGateError::validation_error(&format!("Missing required key: {}", key)))
}
```

❌ **DON'T**:
```rust
pub fn get_required_value(&self, key: &str) -> String {
    self.values.get(key).unwrap().clone()
}
```

---

### **Pattern 8: Async Operations**

✅ **DO**:
```rust
pub async fn fetch_data(&self, url: &str) -> Result<Vec<u8>> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| NestGateError::network_error(&format!("Failed to fetch {}: {}", url, e)))?;
    
    let data = response.bytes()
        .await
        .map_err(|e| NestGateError::network_error(&format!("Failed to read response: {}", e)))?;
    
    Ok(data.to_vec())
}
```

❌ **DON'T**:
```rust
pub async fn fetch_data(&self, url: &str) -> Vec<u8> {
    let response = reqwest::get(url).await.unwrap();
    response.bytes().await.unwrap().to_vec()
}
```

---

### **Pattern 9: Error Context Chaining**

✅ **DO**:
```rust
pub fn process_workflow(&self) -> Result<Output> {
    let step1 = self.validate()
        .map_err(|e| NestGateError::internal_error(&format!("Validation failed: {}", e), "workflow"))?;
    
    let step2 = self.transform(step1)
        .map_err(|e| NestGateError::internal_error(&format!("Transform failed: {}", e), "workflow"))?;
    
    self.finalize(step2)
        .map_err(|e| NestGateError::internal_error(&format!("Finalize failed: {}", e), "workflow"))
}
```

❌ **DON'T**:
```rust
pub fn process_workflow(&self) -> Output {
    let step1 = self.validate().unwrap();
    let step2 = self.transform(step1).unwrap();
    self.finalize(step2).unwrap()
}
```

---

### **Pattern 10: Initialization Code**

✅ **DO** (When failure is truly unrecoverable):
```rust
pub fn initialize_logger() {
    env_logger::builder()
        .format_timestamp_millis()
        .init();
    // Logging failure means we can't log errors anyway - acceptable panic point
}
```

✅ **BETTER** (Fallible with context):
```rust
pub fn initialize_logger() -> Result<()> {
    env_logger::builder()
        .format_timestamp_millis()
        .try_init()
        .map_err(|e| NestGateError::internal_error(&format!("Logger init failed: {}", e), "init"))?;
    Ok(())
}
```

---

## 🚫 When `.unwrap()` Is Acceptable

### **Test Code Only**:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_parsing() {
        let result = parse_value("42").unwrap(); // OK in tests
        assert_eq!(result, 42);
    }
}
```

### **Mathematical Invariants**:
```rust
fn is_even(n: u32) -> bool {
    (n % 2) == 0
}

fn process_even_count(items: &[Item]) -> usize {
    let count = items.len();
    if is_even(count.try_into().unwrap()) {  // OK: usize → u32 invariant
        count / 2
    } else {
        count / 2 + 1
    }
}
```

### **Static/Const Initialization**:
```rust
static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\d{3}-\d{4}$").unwrap() // OK: static pattern, compile-time checked
});
```

---

## 📋 Migration Checklist

When removing `.unwrap()` from production code:

- [ ] **Identify Context**: What can fail? Why?
- [ ] **Choose Error Type**: Validation? Network? Internal?
- [ ] **Add Context**: Include actionable error message
- [ ] **Update Signature**: Change return type to `Result<T>`
- [ ] **Handle at Call Site**: Don't just push unwrap up the stack
- [ ] **Test Error Path**: Add tests for the error case
- [ ] **Document**: Add `# Errors` section to function docs

---

## 🔧 Helper Functions

### **Safe Lock Operations**:
```rust
// Available in: crate::safe_operations::mutexes

pub fn safe_mutex_lock<T>(mutex: &Mutex<T>) -> Result<MutexGuard<'_, T>>
pub fn safe_mutex_read<T>(mutex: &RwLock<T>) -> Result<RwLockReadGuard<'_, T>>
pub fn safe_mutex_write<T>(mutex: &RwLock<T>) -> Result<RwLockWriteGuard<'_, T>>
```

### **Safe Network Operations**:
```rust
// Available in: crate::utils::network

pub fn parse_ip(ip: &str) -> Result<IpAddr>
pub fn parse_ipv4(ip: &str) -> Result<Ipv4Addr>
pub fn parse_ipv6(ip: &str) -> Result<Ipv6Addr>
pub fn parse_cidr(cidr: &str) -> Result<(IpAddr, u8)>
```

### **Configuration Loading**:
```rust
// Available in: crate::config::external

pub fn from_env() -> Result<ExternalConfig>
pub fn from_env_production() -> Result<ExternalConfig>
pub fn default_dev() -> ExternalConfig  // Never fails, safe for dev
```

---

## 🎓 Examples from NestGate

### **Example 1: Router Endpoint Discovery**
```rust
pub async fn route_to_storage(&self, _request_type: &str) -> Result<String> {
    // Check cache first
    if let Some(endpoint) = self.get_cached_endpoint("storage").await {
        return Ok(endpoint);
    }

    // Discover dynamically
    let endpoint = format!("storage://zfs-{}", "universal");
    self.cache_endpoint("storage", &endpoint).await;
    Ok(endpoint)
}
```

### **Example 2: Safe Mutex Access**
```rust
pub fn get_provider_registry(&self) -> Result<HashMap<String, String>> {
    let guard = safe_mutex_read(&self.provider_registry)?;
    Ok(guard.clone())
}
```

### **Example 3: IP Validation**
```rust
pub fn validate_and_parse_ip(input: &str) -> Result<IpAddr> {
    if !is_valid_ip(input) {
        return Err(NestGateError::validation_error(
            &format!("Invalid IP format: '{}', expected IPv4 or IPv6", input)
        ));
    }
    parse_ip(input)
}
```

---

## 🚀 Migration Tools

### **Find Unwraps in Production Code**:
```bash
# Find all unwraps in non-test files
find code/crates -name "*.rs" \
    -not -path "*/tests/*" \
    -not -name "*_tests.rs" \
    -not -name "*_test.rs" \
    -exec grep -Hn "\.unwrap()" {} \;
```

### **Find Unsafe Blocks**:
```bash
# Find all unsafe blocks
find code/crates -name "*.rs" -exec grep -Hn "unsafe" {} \;
```

### **Check for Panic Macros**:
```bash
# Find panic!/expect!/unreachable! in production
find code/crates -name "*.rs" \
    -not -path "*/tests/*" \
    -not -name "*_tests.rs" \
    -exec grep -Hn -E "panic!|expect\(|unreachable!" {} \;
```

---

## 📚 Further Reading

- **Rust Book**: [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- **NestGate**: `safe_operations/mutexes.rs` - Safe lock abstractions
- **NestGate**: `error/mod.rs` - NestGateError type definition
- **NestGate**: `utils/network.rs` - Network utility examples

---

**Standard**: All production code MUST follow these patterns  
**Enforcement**: Clippy with `#![deny(unwrap_used)]` in production modules  
**Review**: Error handling is a key review criterion  

---

*Error handling is not just about preventing crashes—it's about providing clear, actionable feedback and maintaining system reliability.*

