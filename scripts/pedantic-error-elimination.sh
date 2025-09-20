#!/bin/bash

# 🎯 **PEDANTIC ERROR ELIMINATION SCRIPT**
# Systematically fix ALL 258 compilation errors for 100% canonical modernization

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# **PHASE 1: Remove invalid fields from error constructors**
fix_invalid_fields() {
    log_info "🎯 PHASE 1: Removing invalid fields (location, is_bug, retryable)"
    
    # Remove 'location' and 'is_bug' fields from Internal errors
    find code/crates/nestgate-core/src -name "*.rs" -type f -exec sed -i '/location: Some(/d' {} \;
    find code/crates/nestgate-core/src -name "*.rs" -type f -exec sed -i '/is_bug: false,/d' {} \;
    find code/crates/nestgate-core/src -name "*.rs" -type f -exec sed -i '/is_bug: true,/d' {} \;
    
    # Remove 'retryable' field from Io errors
    find code/crates/nestgate-core/src -name "*.rs" -type f -exec sed -i '/retryable: true,/d' {} \;
    find code/crates/nestgate-core/src -name "*.rs" -type f -exec sed -i '/retryable: false,/d' {} \;
    
    # Remove 'max_capacity' field from ResourceExhausted errors
    find code/crates/nestgate-core/src -name "*.rs" -type f -exec sed -i '/max_capacity: Some(/d' {} \;
    
    log_success "✅ Invalid fields removed"
}

# **PHASE 2: Fix Box<ErrorContext> vs ErrorContext mismatches**
fix_context_boxing() {
    log_info "🎯 PHASE 2: Fixing Box<ErrorContext> mismatches"
    
    # Add Box::new() around ErrorContext constructors
    find code/crates/nestgate-core/src -name "*.rs" -type f -exec sed -i 's/context: Some(crate::error::context::ErrorContext {/context: Some(Box::new(crate::error::context::ErrorContext {/g' {} \;
    find code/crates/nestgate-core/src -name "*.rs" -type f -exec sed -i 's/}),$/})),/g' {} \;
    
    log_success "✅ Context boxing fixed"
}

# **PHASE 3: Add missing constraint fields**
fix_missing_constraints() {
    log_info "🎯 PHASE 3: Adding missing constraint fields"
    
    # This requires more targeted fixes per file - will be handled individually
    log_warning "⚠️ Constraint fields require individual file fixes"
}

# **PHASE 4: Fix type mismatches**
fix_type_mismatches() {
    log_info "🎯 PHASE 4: Fixing type mismatches"
    
    # Fix u64 to String conversions
    find code/crates/nestgate-core/src -name "*.rs" -type f -exec sed -i 's/current_usage: Some(buffer\.len() as u64),/current_usage: Some((buffer.len() as u64).to_string()),/g' {} \;
    
    log_success "✅ Type mismatches fixed"
}

# **PHASE 5: Validate compilation**
validate_compilation() {
    log_info "🎯 PHASE 5: Validating compilation"
    
    if cargo check --workspace --quiet; then
        log_success "🎉 PEDANTIC SUCCESS: Zero compilation errors achieved!"
        return 0
    else
        log_error "❌ Compilation errors remain - running detailed check"
        cargo check --workspace 2>&1 | head -50
        return 1
    fi
}

# **MAIN EXECUTION**
main() {
    log_info "🚀 Starting PEDANTIC error elimination..."
    
    fix_invalid_fields
    fix_context_boxing
    fix_type_mismatches
    
    log_info "📊 Checking compilation status..."
    if validate_compilation; then
        log_success "🏆 PEDANTIC PERFECTION ACHIEVED!"
    else
        log_warning "🔧 Additional manual fixes required"
    fi
}

# Execute if run directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi 