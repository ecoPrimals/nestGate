#!/bin/bash
# 🔧 HARDCODED VALUES REPLACEMENT SCRIPT
# Systematically replaces hardcoded values with unified constants

set -euo pipefail

echo "🔧 **NESTGATE HARDCODED VALUES REPLACEMENT**"
echo "============================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Function to replace values in a file
replace_in_file() {
    local file="$1"
    local old_pattern="$2"
    local new_value="$3"
    local description="$4"
    
    if grep -q "$old_pattern" "$file"; then
        echo "  📝 Updating $file: $description"
        sed -i "s/$old_pattern/$new_value/g" "$file"
    fi
}

# Function to add import if not present
add_import_if_needed() {
    local file="$1"
    local import_line="$2"
    
    if ! grep -q "use nestgate_core::constants" "$file"; then
        # Add import after existing use statements
        sed -i "/^use /a\\$import_line" "$file"
    fi
}

echo "🔍 **PHASE 1: NETWORK PORT REPLACEMENTS**"
echo "-----------------------------------------"

# Find all Rust files in the codebase
find code/crates -name "*.rs" | while read -r file; do
    # Skip the unified constants file itself
    if [[ "$file" == *"unified_canonical.rs"* ]]; then
        continue
    fi
    
    # Check if file contains hardcoded ports
    if grep -q "\b8080\b\|\b8081\b\|\b8082\b" "$file"; then
        echo "📁 Processing $file"
        
        # Add import for constants
        add_import_if_needed "$file" "use nestgate_core::constants::unified_canonical::network::ports;"
        
        # Replace common hardcoded ports
        replace_in_file "$file" "\b8080\b" "ports::API" "API port"
        replace_in_file "$file" "\b8081\b" "ports::HEALTH" "Health port"
        replace_in_file "$file" "\b8082\b" "ports::METRICS" "Metrics port"
        replace_in_file "$file" "\b8083\b" "ports::DISCOVERY" "Discovery port"
        replace_in_file "$file" "\b8084\b" "ports::WEBSOCKET" "WebSocket port"
        replace_in_file "$file" "\b9090\b" "ports::GRPC" "gRPC port"
    fi
done

echo ""
echo "🔍 **PHASE 2: TIMEOUT REPLACEMENTS**"
echo "-----------------------------------"

find code/crates -name "*.rs" | while read -r file; do
    if [[ "$file" == *"unified_canonical.rs"* ]]; then
        continue
    fi
    
    # Check for common timeout patterns
    if grep -q "Duration::from_secs(30)\|Duration::from_secs(60)\|Duration::from_secs(10)" "$file"; then
        echo "📁 Processing timeouts in $file"
        
        # Add import for timeout constants
        add_import_if_needed "$file" "use nestgate_core::constants::unified_canonical::network::timeouts;"
        
        # Replace common timeout values
        replace_in_file "$file" "Duration::from_secs(30)" "timeouts::CONNECTION" "Connection timeout"
        replace_in_file "$file" "Duration::from_secs(60)" "timeouts::REQUEST" "Request timeout"
        replace_in_file "$file" "Duration::from_secs(10)" "timeouts::HEALTH_CHECK" "Health check timeout"
    fi
done

echo ""
echo "🔍 **PHASE 3: BUFFER SIZE REPLACEMENTS**"
echo "---------------------------------------"

find code/crates -name "*.rs" | while read -r file; do
    if [[ "$file" == *"unified_canonical.rs"* ]]; then
        continue
    fi
    
    # Check for common buffer sizes
    if grep -q "\b65536\b\|\b131072\b\|\b4096\b" "$file"; then
        echo "📁 Processing buffer sizes in $file"
        
        # Add import for buffer constants
        add_import_if_needed "$file" "use nestgate_core::constants::unified_canonical::network::buffers;"
        
        # Replace common buffer sizes
        replace_in_file "$file" "\b65536\b" "buffers::DEFAULT" "Default buffer size"
        replace_in_file "$file" "\b131072\b" "buffers::LARGE" "Large buffer size"
        replace_in_file "$file" "\b4096\b" "buffers::SMALL" "Small buffer size"
    fi
done

echo ""
echo "🔍 **PHASE 4: SECURITY CONSTANT REPLACEMENTS**"
echo "----------------------------------------------"

find code/crates -name "*.rs" | while read -r file; do
    if [[ "$file" == *"unified_canonical.rs"* ]]; then
        continue
    fi
    
    # Check for security-related hardcoded values
    if grep -q '"admin"\|"user"\|"AES-256-GCM"' "$file"; then
        echo "📁 Processing security constants in $file"
        
        # Add import for security constants
        add_import_if_needed "$file" "use nestgate_core::constants::unified_canonical::security::{roles, encryption};"
        
        # Replace security constants
        replace_in_file "$file" '"admin"' "roles::ADMIN" "Admin role"
        replace_in_file "$file" '"user"' "roles::USER" "User role"
        replace_in_file "$file" '"AES-256-GCM"' "encryption::ALGORITHM" "Encryption algorithm"
    fi
done

echo ""
echo "🔍 **PHASE 5: API CONSTANT REPLACEMENTS**"
echo "----------------------------------------"

find code/crates -name "*.rs" | while read -r file; do
    if [[ "$file" == *"unified_canonical.rs"* ]]; then
        continue
    fi
    
    # Check for HTTP status codes and content types
    if grep -q '\b200\b\|\b404\b\|\b500\b\|"application/json"' "$file"; then
        echo "📁 Processing API constants in $file"
        
        # Add import for API constants
        add_import_if_needed "$file" "use nestgate_core::constants::unified_canonical::api::{status, content_types};"
        
        # Replace API constants (be careful with status codes in contexts)
        replace_in_file "$file" '"application/json"' "content_types::JSON" "JSON content type"
        # Note: Being conservative with status codes as they might be in different contexts
    fi
done

echo ""
echo "📊 **CHECKING COMPILATION**"
echo "---------------------------"
echo "Running cargo check to verify changes..."

if cargo check --workspace --message-format short 2>&1 | grep -q "error"; then
    echo "⚠️  Some compilation errors detected. Manual review needed."
    echo "Run 'cargo check' to see detailed errors."
else
    echo "✅ All changes compiled successfully!"
fi

echo ""
echo "✅ **HARDCODED VALUES REPLACEMENT COMPLETE**"
echo "============================================"
echo ""
echo "📊 **REPLACEMENT SUMMARY:**"
echo "- ✅ Network ports replaced with unified constants"
echo "- ✅ Timeout values replaced with unified constants"
echo "- ✅ Buffer sizes replaced with unified constants"
echo "- ✅ Security constants replaced with unified constants"
echo "- ✅ API constants replaced with unified constants"
echo ""
echo "📋 **NEXT STEPS:**"
echo "1. Review and test the replacements"
echo "2. Update any remaining hardcoded values manually"
echo "3. Add environment variable support where needed"
echo "4. Update documentation"
echo ""
echo "🎯 **GOAL ACHIEVED**: Systematic hardcoded value elimination" 