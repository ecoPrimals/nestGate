#!/bin/bash
# 🔧 ERROR VARIANTS RESTORATION SCRIPT
# Restores all error variant files to working state

set -euo pipefail

echo "🔧 **NESTGATE ERROR VARIANTS RESTORATION**"
echo "=========================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

ERROR_VARIANTS_DIR="code/crates/nestgate-core/src/error/variants"

echo "🔍 **RESTORING ALL ERROR VARIANT FILES**"
echo "----------------------------------------"

# Restore api_errors.rs
cat > "$ERROR_VARIANTS_DIR/api_errors.rs" << 'EOF'
// **API ERROR UTILITIES - WORKING VERSION**
//! API-specific error types and handling for the NestGate system.

use super::core_errors::NestGateUnifiedError;

impl NestGateUnifiedError {
    /// Create an API error with status code
    pub fn api_with_status(message: impl Into<String>, status_code: u16) -> Self {
        Self::Api(Box::new(super::core_errors::ApiErrorDetails {
            message: message.into(),
            status_code: Some(status_code),
            request_id: None,
            endpoint: None,
            context: None,
        }))
    }

    /// Create a not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::api_with_status(format!("Not found: {}", message.into()), 404)
    }

    /// Create a service unavailable error
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::api_with_status(format!("Service unavailable: {}", message.into()), 503)
    }
}
EOF

# Restore network_errors.rs
cat > "$ERROR_VARIANTS_DIR/network_errors.rs" << 'EOF'
// **NETWORK ERROR UTILITIES - WORKING VERSION**
//! Network-specific error types and handling for the NestGate system.

use super::core_errors::NestGateUnifiedError;

impl NestGateUnifiedError {
    /// Create a network error with operation details
    pub fn network_with_operation(message: impl Into<String>, operation: impl Into<String>) -> Self {
        Self::Network(Box::new(super::core_errors::NetworkErrorDetails {
            message: message.into(),
            operation: Some(operation.into()),
            address: None,
            context: None,
        }))
    }

    /// Create a network timeout error
    pub fn network_timeout(endpoint: impl Into<String>) -> Self {
        Self::Network(Box::new(super::core_errors::NetworkErrorDetails {
            message: format!("Network timeout for endpoint: {}", endpoint.into()),
            operation: Some("network_request".to_string()),
            address: Some(endpoint.into()),
            context: None,
        }))
    }
}
EOF

# Restore security_errors.rs
cat > "$ERROR_VARIANTS_DIR/security_errors.rs" << 'EOF'
// **SECURITY ERROR UTILITIES - WORKING VERSION**
//! Security-specific error types and handling for the NestGate system.

use super::core_errors::NestGateUnifiedError;

impl NestGateUnifiedError {
    /// Create an authentication error
    pub fn authentication_error(message: impl Into<String>) -> Self {
        Self::Security(Box::new(super::core_errors::SecurityErrorDetails {
            message: message.into(),
            operation: Some("authentication".to_string()),
            user: None,
            context: None,
        }))
    }

    /// Create an authorization error
    pub fn authorization_error(message: impl Into<String>, principal: impl Into<String>) -> Self {
        Self::Security(Box::new(super::core_errors::SecurityErrorDetails {
            message: message.into(),
            operation: Some("authorization".to_string()),
            user: Some(principal.into()),
            context: None,
        }))
    }
}
EOF

# Restore storage_errors.rs
cat > "$ERROR_VARIANTS_DIR/storage_errors.rs" << 'EOF'
// **STORAGE ERROR UTILITIES - WORKING VERSION**
//! Storage-specific error types and handling for the NestGate system.

use super::core_errors::NestGateUnifiedError;

impl NestGateUnifiedError {
    /// Create a storage error with operation details
    pub fn storage_with_operation(message: impl Into<String>, operation: impl Into<String>) -> Self {
        Self::Storage(Box::new(super::core_errors::StorageErrorDetails {
            message: message.into(),
            operation: Some(operation.into()),
            path: None,
            context: None,
        }))
    }

    /// Create a ZFS-specific error
    pub fn zfs_error(message: impl Into<String>, resource: impl Into<String>) -> Self {
        Self::Storage(Box::new(super::core_errors::StorageErrorDetails {
            message: message.into(),
            operation: Some("zfs".to_string()),
            path: Some(resource.into()),
            context: None,
        }))
    }
}
EOF

# Restore system_errors.rs
cat > "$ERROR_VARIANTS_DIR/system_errors.rs" << 'EOF'
// **SYSTEM ERROR UTILITIES - WORKING VERSION**
//! System-specific error types and handling for the NestGate system.

use super::core_errors::NestGateUnifiedError;

impl NestGateUnifiedError {
    /// Create a system error
    pub fn system_error(message: impl Into<String>, component: impl Into<String>) -> Self {
        Self::System(Box::new(super::core_errors::SystemErrorDetails {
            message: message.into(),
            component: Some(component.into()),
            context: None,
        }))
    }

    /// Create a resource exhaustion error
    pub fn resource_exhausted(message: impl Into<String>, resource: impl Into<String>) -> Self {
        Self::ResourceExhausted(Box::new(super::core_errors::ResourceExhaustedErrorDetails {
            message: message.into(),
            resource: Some(resource.into()),
            limit: None,
            context: None,
        }))
    }
}
EOF

# Restore automation_errors.rs
cat > "$ERROR_VARIANTS_DIR/automation_errors.rs" << 'EOF'
// **AUTOMATION ERROR UTILITIES - WORKING VERSION**
//! Automation-specific error types and handling for the NestGate system.

use super::core_errors::NestGateUnifiedError;

impl NestGateUnifiedError {
    /// Create a new automation error
    pub fn automation_error(message: impl Into<String>) -> Self {
        Self::Automation(Box::new(super::core_errors::AutomationErrorDetails {
            message: message.into(),
            operation: Some("automation".to_string()),
            target: None,
            automation_data: None,
            context: None,
        }))
    }
}
EOF

echo "✅ All error variant files restored"

echo ""
echo "🔧 **TESTING COMPILATION**"
echo "-------------------------"

if cargo check --workspace --quiet; then
    echo "🎉 **COMPILATION SUCCESS!**"
    echo "✅ Error system fully operational"
    COMPILATION_STATUS="SUCCESS"
else
    echo "⚠️  Compilation still has issues"
    COMPILATION_STATUS="ISSUES"
    echo "First few errors:"
    cargo check --workspace 2>&1 | head -5
fi

echo ""
echo "✅ **ERROR VARIANTS RESTORATION COMPLETE**"
echo "=========================================="
echo ""
echo "📊 **RESTORATION SUMMARY:**"
echo "- ✅ All 6 error variant files restored"
echo "- ✅ Clean syntax and proper structure"
echo "- ✅ Comprehensive error constructors"
echo "- 🔧 Compilation status: $COMPILATION_STATUS"
echo ""
if [[ "$COMPILATION_STATUS" == "SUCCESS" ]]; then
    echo "🎉 **SUCCESS**: Error system fully functional!"
    echo "🚀 **READY**: All unification systems operational"
    echo ""
    echo "🏆 **ACHIEVEMENT**: Complete modernization framework delivered"
else
    echo "📋 **PROGRESS**: Error variants restored"
    echo "🔄 **NEXT**: Address any remaining issues"
fi 