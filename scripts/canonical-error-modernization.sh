#!/bin/bash
# Canonical Error Modernization Script
# Systematically converts old error struct syntax to new boxed tuple syntax

set -e

echo "🔧 CANONICAL ERROR MODERNIZATION - Starting systematic conversion..."

# Navigate to project root
cd "$(dirname "$0")/.."

echo "📍 Working directory: $(pwd)"

# Function to fix Internal error patterns
fix_internal_errors() {
    echo "🔧 Converting Internal error patterns..."
    
    # Find all files with Internal error struct syntax
    find code/crates -name "*.rs" -exec grep -l "NestGateError::Internal {" {} \; | while read file; do
        echo "  📝 Fixing: $file"
        
        # Use sed to convert Internal error patterns
        sed -i 's/NestGateError::Internal {/NestGateError::internal_error_with_debug(/g' "$file"
        
        # Clean up the closing braces and convert to function call
        sed -i '/message: format!/,/}$/{
            s/message: format!(\([^)]*\))/\1/
            /component:/d
            /location:/d
            /is_bug:/d
            /context:/d
            /debug_info:/d
            s/}$/)/
        }' "$file"
    done
}

# Function to fix System error patterns
fix_system_errors() {
    echo "🔧 Converting System error patterns..."
    
    find code/crates -name "*.rs" -exec grep -l "NestGateError::System {" {} \; | while read file; do
        echo "  📝 Fixing: $file"
        
        # Convert System errors to use helper function
        sed -i 's/NestGateError::System {/NestGateError::system_error(/g' "$file"
        
        # Clean up the fields and convert to function call
        sed -i '/message: format!/,/}$/{
            s/message: format!(\([^)]*\))/\1/
            /operation:/d
            /resource:/d
            /utilization:/d
            /retryable:/d
            /context:/d
            s/}$/)/
        }' "$file"
    done
}

# Function to fix Storage error patterns
fix_storage_errors() {
    echo "🔧 Converting Storage error patterns..."
    
    find code/crates -name "*.rs" -exec grep -l "NestGateError::Storage {" {} \; | while read file; do
        echo "  📝 Fixing: $file"
        
        sed -i 's/NestGateError::Storage {/NestGateError::storage_error(/g' "$file"
        
        sed -i '/message: format!/,/}$/{
            s/message: format!(\([^)]*\))/\1/
            /operation:/d
            /resource:/d
            /retryable:/d
            /storage_data:/d
            /context:/d
            s/}$/)/
        }' "$file"
    done
}

# Function to fix Network error patterns
fix_network_errors() {
    echo "🔧 Converting Network error patterns..."
    
    find code/crates -name "*.rs" -exec grep -l "NestGateError::Network {" {} \; | while read file; do
        echo "  📝 Fixing: $file"
        
        sed -i 's/NestGateError::Network {/NestGateError::network_error(/g' "$file"
        
        sed -i '/message: format!/,/}$/{
            s/message: format!(\([^)]*\))/\1/
            /operation:/d
            /address:/d
            /remote_address:/d
            /endpoint:/d
            /retry_after:/d
            /network_code:/d
            /recoverable:/d
            /retryable:/d
            /network_data:/d
            /context:/d
            s/}$/)/
        }' "$file"
    done
}

# Function to fix Security error patterns
fix_security_errors() {
    echo "🔧 Converting Security error patterns..."
    
    find code/crates -name "*.rs" -exec grep -l "NestGateError::Security {" {} \; | while read file; do
        echo "  📝 Fixing: $file"
        
        sed -i 's/NestGateError::Security {/NestGateError::permission_denied(/g' "$file"
        
        sed -i '/message: format!/,/}$/{
            s/message: format!(\([^)]*\))/\1/
            /operation:/d
            /subject:/d
            /retryable:/d
            /security_data:/d
            /context:/d
            s/}$/)/
        }' "$file"
    done
}

# Add helper functions to NestGateError if they don't exist
add_helper_functions() {
    echo "🔧 Adding helper functions to error module..."
    
    # Check if helper functions exist in variants.rs
    if ! grep -q "pub fn system_error" code/crates/nestgate-core/src/error/variants.rs; then
        cat >> code/crates/nestgate-core/src/error/variants.rs << 'EOF'

    /// Create system error
    pub fn system_error(message: impl Into<String>) -> Self {
        Self::System(Box::new(SystemErrorData {
            message: message.into(),
            operation: "system_operation".to_string(),
            error_code: None,
            component: None,
            retryable: false,
            context: None,
        }))
    }
EOF
    fi
}

# Main execution
echo "🚀 Starting canonical error modernization..."

# Add helper functions first
add_helper_functions

# Fix error patterns systematically
fix_internal_errors
fix_system_errors
fix_storage_errors
fix_network_errors
fix_security_errors

echo "✅ Canonical error modernization complete!"
echo "🔍 Testing compilation..."

# Test compilation
if cargo check --package nestgate-core --quiet; then
    echo "✅ Core compilation successful!"
else
    echo "⚠️  Some compilation issues remain - manual fixes may be needed"
fi

echo "🎉 Canonical modernization phase complete!" 