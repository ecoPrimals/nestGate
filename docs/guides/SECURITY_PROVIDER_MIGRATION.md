# Security Provider Migration Guide
**Version**: 0.11.3  
**Created**: November 10, 2025  
**Target**: All implementations of deprecated security provider traits

---

## Overview

This guide provides a comprehensive migration path from deprecated security provider traits to the canonical `SecurityProvider` trait. This consolidation:

- **Reduces** 9+ security trait variants → 1 canonical trait
- **Enhances** functionality: 5 methods → 14 comprehensive methods
- **Maintains** zero-cost abstractions through native async (RPITIT)
- **Provides** clear deprecation timeline with 6-month migration period

---

## Timeline

| Version | Date | Action |
|---------|------|--------|
| **v0.11.3** | November 2025 | Deprecation warnings added |
| **v0.11.4-12.0** | Nov 2025 - May 2026 | Migration period (6 months) |
| **v0.12.0** | May 2026 | **Deprecated traits removed** |

---

## Deprecated Traits

The following traits are deprecated and will be removed in v0.12.0:

1. `universal_traits::SecurityPrimalProvider`
2. `zero_cost_security_provider::traits::ZeroCostSecurityProvider`
3. `universal_providers_zero_cost::ZeroCostSecurityProvider`
4. `zero_cost::traits::ZeroCostSecurityProvider`

**Replace all with**: `traits::canonical_provider_unification::SecurityProvider`

---

## Migration Paths

### Path 1: From `SecurityPrimalProvider`

**Before (Deprecated)**:

```rust
use nestgate_core::universal_traits::SecurityPrimalProvider;

impl SecurityPrimalProvider for MySecurityProvider {
    async fn authenticate(&self, credentials: &Credentials) -> Result<AuthToken> {
        // implementation
    }
    
    async fn encrypt(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>> {
        // implementation
    }
    
    async fn decrypt(&self, encrypted: &[u8], algorithm: &str) -> Result<Vec<u8>> {
        // implementation
    }
    
    async fn sign_data(&self, data: &[u8]) -> Result<Signature> {
        // implementation
    }
    
    async fn verify_signature(&self, data: &[u8], signature: &Signature) -> Result<bool> {
        // implementation
    }
    
    async fn get_key_id(&self) -> Result<String> {
        // implementation
    }
    
    async fn hash_data(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>> {
        // implementation
    }
    
    async fn generate_random(&self, length: usize) -> Result<Vec<u8>> {
        // implementation
    }
}
```

**After (Canonical)**:

```rust
use nestgate_core::traits::canonical_provider_unification::{
    SecurityProvider, CanonicalUniversalProvider, AuthToken, SecurityService,
};

impl SecurityProvider for MySecurityProvider {
    // ===== AUTHENTICATION =====
    fn authenticate(&self, credentials: &[u8]) -> impl Future<Output = Result<AuthToken>> + Send {
        async move {
            // Convert &[u8] to your credential format
            let creds = parse_credentials(credentials)?;
            // Your existing authentication logic
            Ok(auth_token)
        }
    }
    
    fn authorize(&self, token: &AuthToken, data: &[u8]) 
        -> impl Future<Output = Result<Vec<u8>>> + Send 
    {
        async move {
            // Implement authorization logic
            Ok(data.to_vec())
        }
    }
    
    // ===== TOKEN MANAGEMENT (NEW) =====
    fn validate_token(&self, token: &AuthToken) 
        -> impl Future<Output = Result<bool>> + Send 
    {
        async move {
            // Implement token validation
            Ok(true)
        }
    }
    
    fn refresh_token(&self, token: &AuthToken) 
        -> impl Future<Output = Result<AuthToken>> + Send 
    {
        async move {
            // Implement token refresh logic
            Ok(new_token)
        }
    }
    
    fn revoke_token(&self, token: &AuthToken) 
        -> impl Future<Output = Result<()>> + Send 
    {
        async move {
            // Implement token revocation
            Ok(())
        }
    }
    
    // ===== ENCRYPTION =====
    fn encrypt(&self, data: &[u8], algorithm: &str) 
        -> impl Future<Output = Result<Vec<u8>>> + Send 
    {
        async move {
            // Your existing encryption logic
            Ok(encrypted)
        }
    }
    
    fn decrypt(&self, data: &[u8]) 
        -> impl Future<Output = Result<Option<Vec<u8>>>> + Send 
    {
        async move {
            // Your existing decryption logic (note: returns Option now)
            Ok(Some(decrypted))
        }
    }
    
    // ===== SIGNING =====
    fn sign(&self, data: &[u8]) 
        -> impl Future<Output = Result<()>> + Send 
    {
        async move {
            // Sign data (signature stored internally)
            Ok(())
        }
    }
    
    fn verify(&self, data: &[u8], signature: &[u8]) 
        -> impl Future<Output = Result<Option<(String, Vec<u8>)>>> + Send 
    {
        async move {
            // Verify signature, return algorithm and key_id if valid
            if is_valid {
                Ok(Some(("RS256".to_string(), key_id)))
            } else {
                Ok(None)
            }
        }
    }
    
    // ===== KEY MANAGEMENT (ENHANCED) =====
    fn get_key_id(&self) -> impl Future<Output = Result<String>> + Send {
        async move {
            // Your existing key ID logic
            Ok(key_id)
        }
    }
    
    fn supported_algorithms(&self) 
        -> impl Future<Output = Result<Vec<String>>> + Send 
    {
        async move {
            Ok(vec![
                "AES-256-GCM".to_string(),
                "ChaCha20-Poly1305".to_string(),
            ])
        }
    }
    
    // ===== UTILITIES =====
    fn hash_data(&self, data: &[u8], algorithm: &str) 
        -> impl Future<Output = Result<Vec<u8>>> + Send 
    {
        async move {
            // Your existing hash logic
            Ok(hash)
        }
    }
    
    fn generate_random(&self, length: usize) 
        -> impl Future<Output = Result<Vec<u8>>> + Send 
    {
        async move {
            // Your existing random generation logic
            Ok(random_bytes)
        }
    }
}

// Also implement CanonicalUniversalProvider
impl CanonicalUniversalProvider<Box<dyn SecurityService>> for MySecurityProvider {
    type Config = MyConfig;
    type Error = MyError;
    type Metadata = MyMetadata;
    
    // ... implement required methods
}
```

### Path 2: From `ZeroCostSecurityProvider`

**Before (Deprecated)**:

```rust
use nestgate_core::zero_cost_security_provider::traits::ZeroCostSecurityProvider;

impl ZeroCostSecurityProvider for MyProvider {
    type Config = MyConfig;
    type Health = MyHealth;
    type Metrics = MyMetrics;
    
    async fn authenticate(&self, credentials: &ZeroCostCredentials) 
        -> Result<ZeroCostAuthToken> 
    {
        // implementation
    }
    
    async fn validate_token(&self, token: &str) -> Result<bool> {
        // implementation
    }
    
    // ... other methods
}
```

**After (Canonical)**:

```rust
use nestgate_core::traits::canonical_provider_unification::SecurityProvider;

impl SecurityProvider for MyProvider {
    // No associated types needed!
    
    fn authenticate(&self, credentials: &[u8]) 
        -> impl Future<Output = Result<AuthToken>> + Send 
    {
        async move {
            // Convert from &[u8] to ZeroCostCredentials if needed
            let creds = ZeroCostCredentials::from_bytes(credentials)?;
            // Your existing logic, convert token type
            let zero_cost_token = /* existing logic */;
            Ok(AuthToken::from(zero_cost_token))
        }
    }
    
    fn validate_token(&self, token: &AuthToken) 
        -> impl Future<Output = Result<bool>> + Send 
    {
        async move {
            // Extract token string
            self.internal_validate(&token.token).await
        }
    }
    
    // ... implement all 14 methods
}
```

---

## Key Differences

### 1. Credential Format

| Old | New |
|-----|-----|
| `&Credentials` struct | `&[u8]` raw bytes |
| `&ZeroCostCredentials` | `&[u8]` raw bytes |

**Migration**: Parse bytes in your implementation

### 2. Token Type

| Old | New |
|-----|-----|
| Return implementation-specific token | Return `AuthToken` |
| `ZeroCostAuthToken` | `AuthToken` |

**Migration**: Convert your token to `AuthToken` struct

### 3. Associated Types

| Old | New |
|-----|-----|
| `type Config = ...;` | No associated types |
| `type Health = ...;` | Use `ProviderHealth` |
| `type Metrics = ...;` | Use `ProviderMetrics` |

**Migration**: Use canonical types or convert in method implementations

### 4. Return Types

| Method | Old | New |
|--------|-----|-----|
| `decrypt` | `Result<Vec<u8>>` | `Result<Option<Vec<u8>>>` |
| `verify` | `Result<bool>` | `Result<Option<(String, Vec<u8>)>>` |
| `sign` | `Result<Signature>` | `Result<()>` |

**Migration**: Adapt return values to new signatures

### 5. New Methods to Implement

The canonical `SecurityProvider` adds these methods:

1. **`authorize`**: Authorization with token + data
2. **`validate_token`**: Token validation
3. **`refresh_token`**: Token refresh
4. **`revoke_token`**: Token revocation
5. **`supported_algorithms`**: List supported algorithms

**Migration**: Implement these new methods (can start with simple/stub implementations)

---

## Step-by-Step Migration

### Step 1: Update Imports

```rust
// OLD
use nestgate_core::universal_traits::SecurityPrimalProvider;
use nestgate_core::zero_cost_security_provider::traits::ZeroCostSecurityProvider;

// NEW
use nestgate_core::traits::canonical_provider_unification::{
    SecurityProvider, CanonicalUniversalProvider, AuthToken, SecurityService,
};
```

### Step 2: Update Trait Implementation

```rust
// OLD
impl SecurityPrimalProvider for MyProvider {

// NEW
impl SecurityProvider for MyProvider {
```

### Step 3: Update Method Signatures

Convert each method to the new signature:

```rust
// OLD
async fn authenticate(&self, credentials: &Credentials) -> Result<AuthToken>

// NEW
fn authenticate(&self, credentials: &[u8]) 
    -> impl Future<Output = Result<AuthToken>> + Send
{
    async move {
        // Implementation
    }
}
```

### Step 4: Add New Methods

Implement the 5 new required methods:
- `authorize`
- `validate_token`
- `refresh_token`
- `revoke_token`
- `supported_algorithms`

### Step 5: Implement `CanonicalUniversalProvider`

Add the provider interface implementation:

```rust
impl CanonicalUniversalProvider<Box<dyn SecurityService>> for MyProvider {
    type Config = MyConfig;
    type Error = MyError;
    type Metadata = MyMetadata;
    
    async fn initialize(&self, config: Self::Config) -> Result<()> {
        // Initialization logic
        Ok(())
    }
    
    async fn provide(&self) -> Result<Box<dyn SecurityService>> {
        // Return service instance
        Err(NestGateError::not_implemented("provide"))
    }
    
    // ... implement other required methods
}
```

### Step 6: Update Usage Sites

Update code that uses your provider:

```rust
// OLD
let provider: Box<dyn SecurityPrimalProvider> = Box::new(MyProvider::new());

// NEW
let provider: Box<dyn SecurityProvider> = Box::new(MyProvider::new());
```

### Step 7: Test Thoroughly

```rust
#[tokio::test]
async fn test_migration() -> Result<()> {
    let provider = MyProvider::new();
    
    // Test authentication
    let creds = b"username:password";
    let token = provider.authenticate(creds).await?;
    
    // Test new methods
    assert!(provider.validate_token(&token).await?);
    let refreshed = provider.refresh_token(&token).await?;
    provider.revoke_token(&refreshed).await?;
    
    Ok(())
}
```

---

## Common Pitfalls

### 1. Forgetting to Remove Associated Types

**Error**:
```rust
impl SecurityProvider for MyProvider {
    type Config = MyConfig;  // ❌ No associated types!
```

**Fix**:
```rust
impl SecurityProvider for MyProvider {
    // ✅ No type declarations
```

### 2. Wrong Method Signature Format

**Error**:
```rust
async fn authenticate(&self, ...) -> Result<AuthToken> {  // ❌ Old async syntax
```

**Fix**:
```rust
fn authenticate(&self, ...) -> impl Future<Output = Result<AuthToken>> + Send {
    async move {  // ✅ Native async (RPITIT)
        // implementation
    }
}
```

### 3. Not Implementing New Methods

**Error**: Compiler error about missing methods

**Fix**: Implement all 14 required methods

### 4. Incorrect Return Types

**Error**:
```rust
fn decrypt(&self, data: &[u8]) -> impl Future<Output = Result<Vec<u8>>> + Send  // ❌
```

**Fix**:
```rust
fn decrypt(&self, data: &[u8]) -> impl Future<Output = Result<Option<Vec<u8>>>> + Send  // ✅
```

---

## Testing Your Migration

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_core::traits::canonical_provider_unification::SecurityProvider;
    
    #[tokio::test]
    async fn test_security_provider_migration() -> Result<()> {
        let provider = MySecurityProvider::new();
        
        // Test authentication
        let creds = b"test:credentials";
        let token = provider.authenticate(creds).await?;
        assert!(!token.token.is_empty());
        
        // Test token operations (new methods)
        assert!(provider.validate_token(&token).await?);
        let refreshed = provider.refresh_token(&token).await?;
        assert_ne!(token.token, refreshed.token);
        provider.revoke_token(&refreshed).await?;
        
        // Test encryption
        let data = b"secret data";
        let encrypted = provider.encrypt(data, "AES-256-GCM").await?;
        let decrypted = provider.decrypt(&encrypted).await?;
        assert_eq!(decrypted, Some(data.to_vec()));
        
        // Test signing
        provider.sign(data).await?;
        let signature = b"signature_bytes";
        let result = provider.verify(data, signature).await?;
        assert!(result.is_some());
        
        // Test key management
        let key_id = provider.get_key_id().await?;
        assert!(!key_id.is_empty());
        let algos = provider.supported_algorithms().await?;
        assert!(!algos.is_empty());
        
        // Test utilities
        let hash = provider.hash_data(data, "SHA-256").await?;
        assert!(!hash.is_empty());
        let random = provider.generate_random(32).await?;
        assert_eq!(random.len(), 32);
        
        Ok(())
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_provider_integration() -> Result<()> {
    let provider = MySecurityProvider::new();
    
    // Test with CanonicalUniversalProvider interface
    let health = provider.health_check().await?;
    assert_eq!(health.status, HealthStatus::Healthy);
    
    let capabilities = provider.get_capabilities().await?;
    assert!(capabilities.operations.contains(&"authenticate".to_string()));
    
    Ok(())
}
```

---

## Checklist

Use this checklist to track your migration:

- [ ] Update imports to use `SecurityProvider`
- [ ] Change trait implementation declaration
- [ ] Convert all method signatures to native async (RPITIT)
- [ ] Update `authenticate` to accept `&[u8]`
- [ ] Implement `authorize` (new method)
- [ ] Implement `validate_token` (new method)
- [ ] Implement `refresh_token` (new method)
- [ ] Implement `revoke_token` (new method)
- [ ] Update `decrypt` return type to `Option<Vec<u8>>`
- [ ] Update `sign` to return `()`
- [ ] Update `verify` return type to `Option<(String, Vec<u8>)>`
- [ ] Implement `supported_algorithms` (new method)
- [ ] Implement `CanonicalUniversalProvider`
- [ ] Update all usage sites
- [ ] Write/update unit tests
- [ ] Write/update integration tests
- [ ] Run full test suite
- [ ] Update documentation
- [ ] Remove old trait implementations
- [ ] Remove deprecated imports

---

## Support

### Documentation

- **Architecture**: `docs/current/ZERO_COST_ARCHITECTURE.md`
- **Traits Overview**: `docs/current/TRAITS_SYSTEM.md`
- **Provider Unification**: `PROVIDER_TRAIT_CONSOLIDATION_EXECUTION_PLAN_NOV_10_2025.md`

### Examples

See example implementations in:
- `code/crates/nestgate-core/src/traits/canonical_provider_unification.rs` (trait definition)
- `code/crates/nestgate-core/src/security_provider.rs` (example implementation)

### Questions?

If you encounter issues during migration:
1. Check this guide's troubleshooting section
2. Review the canonical trait documentation
3. Look at example implementations
4. File an issue with migration questions

---

## Summary

**What Changed**: 9+ security traits → 1 canonical `SecurityProvider`  
**Why**: Simplification, enhanced functionality, maintainability  
**When**: Deprecated v0.11.3, removed v0.12.0 (6 months)  
**How**: Follow this guide's step-by-step migration path

**Benefits**:
- ✅ Simpler codebase (single trait to implement)
- ✅ More functionality (5 → 14 methods)
- ✅ Zero-cost abstractions maintained (native async)
- ✅ Clear migration timeline
- ✅ Professional deprecation period

---

**Migration Guide Version**: 1.0  
**Last Updated**: November 10, 2025  
**Target Release**: v0.12.0 (May 2026)

🚀 **Start your migration today!**

