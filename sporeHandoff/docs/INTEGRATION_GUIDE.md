# 🔧 Universal Cryptographic Spore - Integration Guide

**Complete step-by-step guide for integrating spores into any primal**

## 📋 **Prerequisites**

- Rust 1.70+ with Cargo
- Your primal crate structure set up
- Basic understanding of async Rust
- Access to BearDog endpoint (optional but recommended)

---

## 🚀 **Step-by-Step Integration**

### **Step 1: Copy Core Files**

```bash
# Navigate to your primal's source directory
cd your-primal/src/

# Copy the universal spore system
cp /path/to/sporeHandoff/src/universal_spore.rs ./

# Optional: Copy integration example for reference
cp /path/to/sporeHandoff/src/crypto_locks_integration_example.rs ./examples/
```

### **Step 2: Add Dependencies**

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["sync", "time"] }
tracing = "0.1"
uuid = { version = "1.0", features = ["v4"] }

# Optional: For BearDog integration
# beardog-client = { path = "../beardog" }  # Adjust path as needed
```

### **Step 3: Module Declaration**

Add to your primal's `src/lib.rs`:

```rust
// Add the universal spore module
pub mod universal_spore;

// Re-export key types for convenience
pub use universal_spore::{
    UniversalCryptographicSpore, 
    OperationRequest, 
    UserContext, 
    AuthorizationDecision
};
```

### **Step 4: Error Type Integration**

Add these error variants to your primal's error enum:

```rust
#[derive(Debug, thiserror::Error)]
pub enum YourPrimalError {
    // Your existing errors...

    /// Access denied by spore system
    #[error("Access Denied: {reason}")]
    AccessDenied { reason: String },

    /// Corporate license required
    #[error("License Required: {message}")]
    LicenseRequired { message: String },

    /// Spore system error
    #[error("Spore Error: {0}")]
    SporeError(String),
}
```

### **Step 5: Main Struct Integration**

Integrate the spore into your main primal struct:

```rust
use crate::universal_spore::UniversalCryptographicSpore;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

pub struct YourPrimal {
    // Your existing fields
    pub config: YourConfig,
    pub services: YourServices,
    
    /// Universal cryptographic spore for autonomous security
    crypto_spore: Arc<RwLock<UniversalCryptographicSpore>>,
}

impl YourPrimal {
    /// Create a new instance with spore integration
    pub async fn new(config: YourConfig) -> Result<Self, YourPrimalError> {
        // Initialize your existing components
        let services = YourServices::new(&config).await?;
        
        // Create spore for your primal
        let mut spore = UniversalCryptographicSpore::new_for_primal("your-primal-name")
            .map_err(|e| YourPrimalError::SporeError(e.to_string()))?;
        
        // Optional: Initialize BearDog integration
        if let Some(beardog_endpoint) = &config.beardog_endpoint {
            spore.initialize_with_beardog(Some(beardog_endpoint.clone())).await
                .map_err(|e| YourPrimalError::SporeError(e.to_string()))?;
            info!("🧬 BearDog integration initialized for {}", "your-primal-name");
        } else {
            info!("🧬 Spore operating autonomously for {}", "your-primal-name");
        }
        
        Ok(Self {
            config,
            services,
            crypto_spore: Arc::new(RwLock::new(spore)),
        })
    }
}
```

### **Step 6: Authorization Integration**

Add spore authorization to your sensitive operations:

```rust
use crate::universal_spore::{OperationRequest, UserContext, AuthorizationDecision};
use std::collections::HashMap;
use std::time::SystemTime;

impl YourPrimal {
    /// Example: Sensitive operation with spore authorization
    pub async fn sensitive_operation(
        &self,
        operation_type: &str,
        resource_path: &str,
        user_context: UserContext,
    ) -> Result<OperationResult, YourPrimalError> {
        // Create operation request for spore authorization
        let operation = OperationRequest {
            operation_type: operation_type.to_string(),
            resource_path: resource_path.to_string(),
            user_context,
            metadata: HashMap::new(),
            timestamp: SystemTime::now(),
        };

        // Ask spore for authorization
        let spore = self.crypto_spore.read().await;
        let decision = spore.authorize_operation(&operation).await
            .map_err(|e| YourPrimalError::SporeError(e.to_string()))?;

        match decision {
            AuthorizationDecision::Allow { enhanced_by_beardog, restrictions, .. } => {
                info!("✅ Operation '{}' authorized (BearDog: {})", operation_type, enhanced_by_beardog);
                
                // Apply any restrictions
                for restriction in restrictions {
                    warn!("⚠️ Restriction applied: {}", restriction);
                }
                
                // Perform the actual operation
                self.perform_operation_internal(operation_type, resource_path).await
            },
            
            AuthorizationDecision::RequireLicense { terms, contact, organization_profile } => {
                let message = format!(
                    "Corporate license required for organization '{}'. \
                     Base rate: ${}/month, Automation tax: {}x. \
                     Contact: {}",
                    organization_profile.organization_name,
                    terms.base_monthly_rate,
                    terms.automation_tax_multiplier,
                    contact
                );
                warn!("🏢 {}", message);
                Err(YourPrimalError::LicenseRequired { message })
            },
            
            AuthorizationDecision::Deny { reason, remediation, .. } => {
                warn!("❌ Operation '{}' denied: {} ({})", operation_type, reason, remediation);
                Err(YourPrimalError::AccessDenied { reason })
            }
        }
    }
    
    /// Internal operation implementation (no authorization needed here)
    async fn perform_operation_internal(
        &self,
        operation_type: &str,
        resource_path: &str,
    ) -> Result<OperationResult, YourPrimalError> {
        // Your actual operation logic here
        info!("🔧 Performing {} on {}", operation_type, resource_path);
        Ok(OperationResult::Success)
    }
}
```

### **Step 7: User Context Creation**

Create helper functions to build user contexts:

```rust
use crate::universal_spore::UserContext;
use std::collections::HashMap;

impl YourPrimal {
    /// Create user context from request information
    pub fn create_user_context(
        &self,
        user_id: Option<String>,
        session_id: String,
        ip_address: String,
        user_agent: Option<String>,
        additional_info: HashMap<String, String>,
    ) -> UserContext {
        let mut environment_info = HashMap::new();
        
        // Add primal-specific environment information
        environment_info.insert("primal".to_string(), "your-primal-name".to_string());
        environment_info.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
        
        // Merge additional information
        environment_info.extend(additional_info);
        
        UserContext {
            user_id,
            session_id,
            ip_address,
            user_agent,
            environment_info,
        }
    }
    
    /// Create user context from HTTP request (example)
    pub fn user_context_from_http_request(&self, req: &HttpRequest) -> UserContext {
        let mut additional_info = HashMap::new();
        
        // Extract information that might indicate corporate usage
        if let Some(forwarded_for) = req.headers().get("X-Forwarded-For") {
            additional_info.insert("x_forwarded_for".to_string(), 
                                 forwarded_for.to_str().unwrap_or("").to_string());
        }
        
        // Check for automation indicators
        if let Some(user_agent) = req.headers().get("User-Agent") {
            let ua_str = user_agent.to_str().unwrap_or("");
            if ua_str.contains("bot") || ua_str.contains("automation") || ua_str.contains("curl") {
                additional_info.insert("automation_indicator".to_string(), "true".to_string());
            }
        }
        
        self.create_user_context(
            None, // Extract from session/auth if available
            req.session_id().unwrap_or("unknown".to_string()),
            req.peer_addr().unwrap_or("unknown".to_string()),
            req.headers().get("User-Agent").and_then(|h| h.to_str().ok()).map(String::from),
            additional_info,
        )
    }
}
```

### **Step 8: Spore Monitoring and Evolution**

Add monitoring and evolution management:

```rust
impl YourPrimal {
    /// Check if spore needs evolution and handle it
    pub async fn check_spore_evolution(&self) -> Result<bool, YourPrimalError> {
        let mut spore = self.crypto_spore.write().await;
        
        match spore.spawn_child().await {
            Ok(child_spore) => {
                info!("🌱 Spore evolved: {} -> {} (generation {})", 
                      spore.spore_id, child_spore.spore_id, child_spore.generation);
                
                // Replace current spore with evolved child
                *spore = child_spore;
                Ok(true)
            },
            Err(e) if e.to_string().contains("Evolution not required") => {
                tracing::debug!("Spore evolution not required at this time");
                Ok(false)
            },
            Err(e) => {
                warn!("Spore evolution failed: {}", e);
                Err(YourPrimalError::SporeError(e.to_string()))
            }
        }
    }
    
    /// Get spore status for health checks
    pub async fn get_spore_status(&self) -> SporeStatus {
        let spore = self.crypto_spore.read().await;
        
        SporeStatus {
            spore_id: spore.spore_id.clone(),
            generation: spore.generation,
            beardog_integrated: spore.beardog_integration.is_some(),
            operations_count: spore.usage_stats.operations_count,
            last_evolution: spore.last_evolution,
            primal_name: "your-primal-name".to_string(),
        }
    }
    
    /// Start background spore evolution monitoring
    pub async fn start_spore_monitoring(&self) {
        let spore = self.crypto_spore.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // Check hourly
            
            loop {
                interval.tick().await;
                
                // Check if evolution is needed
                let mut spore_guard = spore.write().await;
                if let Ok(child) = spore_guard.spawn_child().await {
                    info!("🌱 Background spore evolution: {} -> {}", 
                          spore_guard.spore_id, child.spore_id);
                    *spore_guard = child;
                }
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct SporeStatus {
    pub spore_id: String,
    pub generation: u32,
    pub beardog_integrated: bool,
    pub operations_count: u64,
    pub last_evolution: SystemTime,
    pub primal_name: String,
}
```

### **Step 9: Configuration Integration**

Add spore configuration to your config:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YourConfig {
    // Your existing config fields...
    
    /// BearDog endpoint for spore integration (optional)
    pub beardog_endpoint: Option<String>,
    
    /// Spore configuration
    pub spore: SporeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SporeConfig {
    /// Enable spore system (default: true)
    pub enabled: bool,
    
    /// Custom corporate terms (optional - uses defaults if not provided)
    pub corporate_base_rate: Option<f64>,
    
    /// Custom automation tax multiplier (optional)
    pub automation_tax_multiplier: Option<f64>,
    
    /// License contact (optional - uses default beardog contact)
    pub license_contact: Option<String>,
}

impl Default for SporeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            corporate_base_rate: None,
            automation_tax_multiplier: None,
            license_contact: None,
        }
    }
}
```

### **Step 10: Testing Integration**

Create tests to verify spore integration:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[tokio::test]
    async fn test_spore_integration() {
        // Create test config
        let config = YourConfig {
            beardog_endpoint: None, // Test autonomous mode
            spore: SporeConfig::default(),
            // ... other config fields
        };
        
        // Initialize primal with spore
        let primal = YourPrimal::new(config).await.unwrap();
        
        // Test individual user (should get access)
        let individual_context = primal.create_user_context(
            Some("alice".to_string()),
            "session_123".to_string(),
            "192.168.1.100".to_string(),
            Some("Individual Developer Environment".to_string()),
            {
                let mut env = HashMap::new();
                env.insert("user_type".to_string(), "individual".to_string());
                env
            },
        );
        
        let result = primal.sensitive_operation(
            "test_operation",
            "/test/resource",
            individual_context,
        ).await;
        
        assert!(result.is_ok(), "Individual users should get access");
        
        // Test corporate user (should require license)
        let corporate_context = primal.create_user_context(
            Some("corp_user".to_string()),
            "corp_session".to_string(),
            "10.0.0.50".to_string(),
            Some("Corporate Automation System".to_string()),
            {
                let mut env = HashMap::new();
                env.insert("user_type".to_string(), "corporate".to_string());
                env.insert("automation_level".to_string(), "high".to_string());
                env
            },
        );
        
        let result = primal.sensitive_operation(
            "test_operation",
            "/test/resource",
            corporate_context,
        ).await;
        
        match result {
            Err(YourPrimalError::LicenseRequired { .. }) => {
                // This is expected for corporate users
            },
            _ => panic!("Corporate users should require license"),
        }
    }
    
    #[tokio::test]
    async fn test_spore_evolution() {
        let config = YourConfig::default();
        let primal = YourPrimal::new(config).await.unwrap();
        
        // Check initial spore status
        let initial_status = primal.get_spore_status().await;
        assert_eq!(initial_status.generation, 0);
        
        // Force evolution check (may or may not evolve depending on conditions)
        let _evolved = primal.check_spore_evolution().await.unwrap();
        
        // Verify spore is still functional
        let final_status = primal.get_spore_status().await;
        assert!(!final_status.spore_id.is_empty());
    }
}
```

---

## 🔧 **Common Integration Patterns**

### **HTTP API Integration**

```rust
// Example with axum web framework
use axum::{extract::Request, response::Response};

async fn api_handler(req: Request) -> Result<Response, ApiError> {
    let primal = get_primal_instance();
    
    // Create user context from HTTP request
    let user_context = primal.user_context_from_http_request(&req);
    
    // Authorize the API operation
    primal.sensitive_operation(
        "api_access",
        req.uri().path(),
        user_context,
    ).await?;
    
    // Proceed with API logic
    handle_api_request(req).await
}
```

### **CLI Tool Integration**

```rust
// Example for CLI tools
pub async fn cli_command(args: &CliArgs) -> Result<(), CliError> {
    let primal = YourPrimal::new(load_config().await?).await?;
    
    // Create user context for CLI usage
    let user_context = UserContext {
        user_id: Some(whoami::username()),
        session_id: format!("cli_{}", std::process::id()),
        ip_address: "127.0.0.1".to_string(),
        user_agent: Some(format!("CLI/{}", env!("CARGO_PKG_VERSION"))),
        environment_info: {
            let mut env = HashMap::new();
            env.insert("interface".to_string(), "cli".to_string());
            env.insert("command".to_string(), args.command.clone());
            env
        },
    };
    
    // Authorize CLI operation
    primal.sensitive_operation(
        &args.command,
        &args.target,
        user_context,
    ).await?;
    
    // Execute CLI command
    execute_cli_command(args).await
}
```

---

## 🚨 **Critical Integration Points**

### **DO's**
- ✅ Copy `universal_spore.rs` exactly without modifications
- ✅ Use the same integration pattern across all primals
- ✅ Test with both individual and corporate user contexts
- ✅ Initialize BearDog integration when available
- ✅ Handle all three authorization decision types
- ✅ Log spore operations for monitoring

### **DON'Ts**
- ❌ Don't modify the core spore system
- ❌ Don't hardcode other primal names
- ❌ Don't bypass spore authorization for "trusted" operations
- ❌ Don't ignore license requirements for corporate users
- ❌ Don't disable spore system in production

### **Security Considerations**
- 🔒 Always authorize before performing sensitive operations
- 🔒 Validate user context information
- 🔒 Log authorization decisions for audit trails
- 🔒 Handle spore errors gracefully
- 🔒 Monitor spore evolution and status

---

## 🎯 **Next Steps**

1. **Complete Integration**: Follow all steps above
2. **Test Thoroughly**: Test individual and corporate scenarios
3. **Deploy to Staging**: Test in staging environment
4. **Monitor Spore Status**: Set up monitoring and alerting
5. **Production Deployment**: Deploy with confidence

---

**Your primal now has autonomous security with sovereignty preservation! The spore system will handle individual access seamlessly while requiring corporate license negotiation on your terms.** 🧬🚀 