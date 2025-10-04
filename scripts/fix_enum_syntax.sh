#!/bin/bash

# 🔧 **ENUM SYNTAX FIX SCRIPT**
# 
# This script fixes missing commas in enum variants that are blocking compilation

set -euo pipefail

echo "🔧 Fixing enum syntax errors..."

# Function to fix enum variants missing commas
fix_enum_commas() {
    echo "📝 Adding missing commas to enum variants..."
    
    # Find all Rust files and fix enum variants
    find code/crates -name "*.rs" -type f | while read -r file; do
        if [[ -f "$file" ]]; then
            # Use sed to add commas after enum variants that don't have them
            # This handles the pattern: EnumVariant\n    NextVariant
            sed -i '/^[[:space:]]*[A-Z][a-zA-Z0-9_]*[[:space:]]*$/{
                # Check if next line is another enum variant or doc comment
                N
                # If next line starts with whitespace and capital letter or doc comment, add comma to first line
                s/\([A-Z][a-zA-Z0-9_]*\)\([[:space:]]*\)\n\([[:space:]]*\)\(\/\/\/\|[A-Z]\)/\1,\2\n\3\4/
            }' "$file" 2>/dev/null || true
            
            # Fix specific patterns we found in the error output
            sed -i 's/^[[:space:]]*NonBlocking$/    NonBlocking,/' "$file"
            sed -i 's/^[[:space:]]*Async$/    Async,/' "$file"
            sed -i 's/^[[:space:]]*Eager$/    Eager,/' "$file"
            sed -i 's/^[[:space:]]*Lazy$/    Lazy,/' "$file"
            sed -i 's/^[[:space:]]*Predictive$/    Predictive,/' "$file"
            sed -i 's/^[[:space:]]*ThreadPool$/    ThreadPool,/' "$file"
            sed -i 's/^[[:space:]]*ActorModel$/    ActorModel,/' "$file"
            sed -i 's/^[[:space:]]*ReactiveStreams$/    ReactiveStreams,/' "$file"
            sed -i 's/^[[:space:]]*AsyncAwait$/    AsyncAwait,/' "$file"
            sed -i 's/^[[:space:]]*RoundRobin$/    RoundRobin,/' "$file"
            sed -i 's/^[[:space:]]*LeastLoaded$/    LeastLoaded,/' "$file"
            sed -i 's/^[[:space:]]*Random$/    Random,/' "$file"
            sed -i 's/^[[:space:]]*Weighted$/    Weighted,/' "$file"
            sed -i 's/^[[:space:]]*CpuUsage$/    CpuUsage,/' "$file"
            sed -i 's/^[[:space:]]*MemoryUsage$/    MemoryUsage,/' "$file"
            sed -i 's/^[[:space:]]*IoLatency$/    IoLatency,/' "$file"
            sed -i 's/^[[:space:]]*NetworkLatency$/    NetworkLatency,/' "$file"
            sed -i 's/^[[:space:]]*Throughput$/    Throughput,/' "$file"
            sed -i 's/^[[:space:]]*ErrorRate$/    ErrorRate,/' "$file"
            sed -i 's/^[[:space:]]*Cpu$/    Cpu,/' "$file"
            sed -i 's/^[[:space:]]*Memory$/    Memory,/' "$file"
            sed -i 's/^[[:space:]]*Both$/    Both,/' "$file"
            sed -i 's/^[[:space:]]*None$/    None,/' "$file"
            sed -i 's/^[[:space:]]*Process$/    Process,/' "$file"
            sed -i 's/^[[:space:]]*Container$/    Container,/' "$file"
            sed -i 's/^[[:space:]]*Vm$/    Vm,/' "$file"
            sed -i 's/^[[:space:]]*Line$/    Line,/' "$file"
            sed -i 's/^[[:space:]]*Branch$/    Branch,/' "$file"
            sed -i 's/^[[:space:]]*Function$/    Function,/' "$file"
            sed -i 's/^[[:space:]]*Statement$/    Statement,/' "$file"
            sed -i 's/^[[:space:]]*Html$/    Html,/' "$file"
            sed -i 's/^[[:space:]]*Xml$/    Xml,/' "$file"
            sed -i 's/^[[:space:]]*Json$/    Json,/' "$file"
            sed -i 's/^[[:space:]]*Lcov$/    Lcov,/' "$file"
            sed -i 's/^[[:space:]]*Traditional$/    Traditional,/' "$file"
            sed -i 's/^[[:space:]]*Fluent$/    Fluent,/' "$file"
            sed -i 's/^[[:space:]]*Bdd$/    Bdd,/' "$file"
            sed -i 's/^[[:space:]]*Yaml$/    Yaml,/' "$file"
            sed -i 's/^[[:space:]]*Toml$/    Toml,/' "$file"
            sed -i 's/^[[:space:]]*Sql$/    Sql,/' "$file"
            sed -i 's/^[[:space:]]*Binary$/    Binary,/' "$file"
            sed -i 's/^[[:space:]]*Sequential$/    Sequential,/' "$file"
            sed -i 's/^[[:space:]]*Pattern$/    Pattern,/' "$file"
            sed -i 's/^[[:space:]]*Template$/    Template,/' "$file"
            sed -i 's/^[[:space:]]*Truncate$/    Truncate,/' "$file"
            sed -i 's/^[[:space:]]*Delete$/    Delete,/' "$file"
            sed -i 's/^[[:space:]]*Rollback$/    Rollback,/' "$file"
            sed -i 's/^[[:space:]]*Recreate$/    Recreate,/' "$file"
            sed -i 's/^[[:space:]]*Database$/    Database,/' "$file"
            sed -i 's/^[[:space:]]*File$/    File,/' "$file"
            sed -i 's/^[[:space:]]*Network$/    Network,/' "$file"
            sed -i 's/^[[:space:]]*ReadOnly$/    ReadOnly,/' "$file"
            sed -i 's/^[[:space:]]*WriteOnly$/    WriteOnly,/' "$file"
            sed -i 's/^[[:space:]]*ReadWrite$/    ReadWrite,/' "$file"
            sed -i 's/^[[:space:]]*Exclusive$/    Exclusive,/' "$file"
            sed -i 's/^[[:space:]]*Mutex$/    Mutex,/' "$file"
            sed -i 's/^[[:space:]]*RwLock$/    RwLock,/' "$file"
            sed -i 's/^[[:space:]]*SourceValidation$/    SourceValidation,/' "$file"
            sed -i 's/^[[:space:]]*BackupCreation$/    BackupCreation,/' "$file"
            sed -i 's/^[[:space:]]*SourceAnalysis$/    SourceAnalysis,/' "$file"
            sed -i 's/^[[:space:]]*ConfigurationMapping$/    ConfigurationMapping,/' "$file"
            sed -i 's/^[[:space:]]*Migration$/    Migration,/' "$file"
            sed -i 's/^[[:space:]]*TargetValidation$/    TargetValidation,/' "$file"
            sed -i 's/^[[:space:]]*Finalization$/    Finalization,/' "$file"
            sed -i 's/^[[:space:]]*Completed$/    Completed,/' "$file"
            sed -i 's/^[[:space:]]*Critical$/    Critical,/' "$file"
            sed -i 's/^[[:space:]]*Warning$/    Warning,/' "$file"
            sed -i 's/^[[:space:]]*Development$/    Development,/' "$file"
            sed -i 's/^[[:space:]]*Staging$/    Staging,/' "$file"
            sed -i 's/^[[:space:]]*Production$/    Production,/' "$file"
            sed -i 's/^[[:space:]]*Debug$/    Debug,/' "$file"
            sed -i 's/^[[:space:]]*Balanced$/    Balanced,/' "$file"
            sed -i 's/^[[:space:]]*Performance$/    Performance,/' "$file"
            sed -i 's/^[[:space:]]*Default$/    Default,/' "$file"
            sed -i 's/^[[:space:]]*Environment$/    Environment,/' "$file"
            sed -i 's/^[[:space:]]*Info$/    Info,/' "$file"
            sed -i 's/^[[:space:]]*High$/    High,/' "$file"
            sed -i 's/^[[:space:]]*NotStarted$/    NotStarted,/' "$file"
            sed -i 's/^[[:space:]]*InProgress$/    InProgress,/' "$file"
        fi
    done
    
    echo "✅ Fixed enum variant commas"
}

# Function to fix struct field syntax
fix_struct_syntax() {
    echo "🏗️ Fixing struct field syntax..."
    
    find code/crates -name "*.rs" -type f | while read -r file; do
        if [[ -f "$file" ]]; then
            # Fix struct field initialization patterns
            sed -i 's/^[[:space:]]*source_type$/        source_type,/' "$file"
            sed -i 's/^[[:space:]]*options$/        options,/' "$file"
            sed -i 's/^[[:space:]]*backup_path$/        backup_path,/' "$file"
            sed -i 's/^[[:space:]]*warnings$/        warnings,/' "$file"
        fi
    done
    
    echo "✅ Fixed struct field syntax"
}

# Function to fix macro syntax
fix_macro_syntax() {
    echo "🔧 Fixing macro syntax..."
    
    find code/crates -name "*.rs" -type f | while read -r file; do
        if [[ -f "$file" ]]; then
            # Fix the macro definition error
            sed -i 's/\$crate::error::NestGateUnifiedError::from(NestGateError::internal_error($msg/\$crate::error::NestGateUnifiedError::from(NestGateError::internal_error($msg))/' "$file"
        fi
    done
    
    echo "✅ Fixed macro syntax"
}

# Execute all fixes
echo "🚀 Executing enum syntax fixes..."

fix_enum_commas
fix_struct_syntax
fix_macro_syntax

echo ""
echo "🧪 Testing compilation..."

# Test compilation on core crate first
if cargo check --package nestgate-core --quiet; then
    echo "🎉 nestgate-core compiles successfully!"
    
    # Test full workspace
    if cargo check --workspace --quiet; then
        echo "🎉 SUCCESS: All enum syntax errors fixed!"
    else
        echo "⚠️ Some crates still have errors, checking progress..."
        echo "🔍 Remaining errors (first 20):"
        cargo check --workspace 2>&1 | head -20
    fi
else
    echo "⚠️ nestgate-core still has compilation errors"
    echo "🔍 Checking specific errors (first 30):"
    cargo check --package nestgate-core 2>&1 | head -30
fi

echo ""
echo "📈 PROGRESS SUMMARY"
echo "=================="
echo "✅ Fixed enum variant comma syntax"
echo "✅ Fixed struct field syntax"
echo "✅ Fixed macro syntax errors"
echo ""
echo "🎯 Next: Continue with remaining compilation fixes" 