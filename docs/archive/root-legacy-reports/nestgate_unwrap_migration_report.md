# NestGate Unwrap Migration Report

**Files Processed**: 252
**Migrations Applied**: 20
**Execution Time**: 173ms

## Migrations by Error Type

### NestGateError::Io (16 migrations)

**File**: `code/crates/nestgate-core/src/universal_storage/backends/memory.rs`
**Line**: 68
**Before**: `let current = *self.current_memory.read().unwrap();`
**After**: `let current = *self.current_memory.read().map_err(|e| NestGateError::Io {
        operation: "io".to_string(),
        error_message: format!("I/O operation failed: io: {}", e),
        operation: "{operation}", retryable: true
    })?`

**File**: `code/crates/nestgate-core/src/universal_storage/backends/memory.rs`
**Line**: 83
**Before**: `let mut current = self.current_memory.write().unwrap();`
**After**: `let mut current = self.current_memory.write().map_err(|e| NestGateError::Io {
        operation: "io".to_string(),
        error_message: format!("I/O operation failed: io: {}", e),
        operation: "{operation}", retryable: true
    })?`

**File**: `code/crates/nestgate-core/src/universal_storage/backends/memory.rs`
**Line**: 123
**Before**: `let storage = self.storage.read().unwrap();`
**After**: `let storage = self.storage.read().map_err(|e| NestGateError::Io {
        operation: "io".to_string(),
        error_message: format!("I/O operation failed: io: {}", e),
        operation: "{operation}", retryable: true
    })?`

**File**: `code/crates/nestgate-core/src/universal_storage/backends/memory.rs`
**Line**: 137
**Before**: `let mut storage = self.storage.write().unwrap();`
**After**: `let mut storage = self.storage.write().map_err(|e| NestGateError::Io {
        operation: "io".to_string(),
        error_message: format!("I/O operation failed: io: {}", e),
        operation: "{operation}", retryable: true
    })?`

**File**: `code/crates/nestgate-core/src/universal_storage/backends/memory.rs`
**Line**: 191
**Before**: `let mut storage = self.storage.write().unwrap();`
**After**: `let mut storage = self.storage.write().map_err(|e| NestGateError::Io {
        operation: "io".to_string(),
        error_message: format!("I/O operation failed: io: {}", e),
        operation: "{operation}", retryable: true
    })?`

... and 11 more

### NestGateError::Validation (3 migrations)

**File**: `code/crates/nestgate-core/src/config/canonical.rs`
**Line**: 344
**Before**: `host: "127.0.0.1".parse().unwrap(),`
**After**: `host: "127.0.0.1".parse().map_err(|e| NestGateError::Validation {
        operation: "parse".to_string(),
        error_message: format!("Parsing failed: parse: {}", e),
        field: "parsed_value", user_error: true
    })?`

**File**: `code/crates/nestgate-core/src/constants/mod.rs`
**Line**: 30
**Before**: `CONFIG.set(config).expect("Configuration already initialized");`
**After**: `CONFIG.set(config).map_err(|e| NestGateError::Validation {
        operation: "config".to_string(),
        error_message: format!("Configuration error: config: {}", e),
        field: "config_field", user_error: true
    })?`

**File**: `code/crates/nestgate-core/src/constants/mod.rs`
**Line**: 35
**Before**: `CONFIG.get().expect("Configuration not initialized. Call init_config() first.")`
**After**: `CONFIG.get().map_err(|e| NestGateError::Validation {
        operation: "config".to_string(),
        error_message: format!("Configuration error: config: {}", e),
        field: "config_field", user_error: true
    })?`

### NestGateError::ResourceExhausted (1 migrations)

**File**: `code/crates/nestgate-core/src/safe_operations/mutexes.rs`
**Line**: 2
**Before**: `/// Provides safe alternatives to lock().unwrap() patterns with proper error handling`
**After**: `/// Provides safe alternatives to lock().map_err(|e| NestGateError::ResourceExhausted {
        operation: "resource".to_string(),
        error_message: format!("Resource exhausted: resource: {}", e),
        resource: "{resource}", current: 0, limit: 100
    })?`

