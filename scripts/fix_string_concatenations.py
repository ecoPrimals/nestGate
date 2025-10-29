#!/usr/bin/env python3

"""
Canonical Modernization: String Concatenation Fixer

This script fixes malformed string concatenations throughout the codebase
and modernizes them to use canonical format! patterns.
"""

import re
import os
import sys
from pathlib import Path

def fix_string_concatenation(content):
    """Fix malformed string concatenation patterns."""
    
    # Pattern 1: Basic localhost pattern
    pattern1 = r'"http://localhost:"\.to_string\(\) \+ ":" \+ &std::env::var\("NESTGATE_PORT"\)\.unwrap_or_else\(\|_\| "\$\{NESTGATE_PORT:-8080\}"\.to_string\(\)\) \+ ""'
    replacement1 = r'format!("http://localhost:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))'
    content = re.sub(pattern1, replacement1, content)
    
    # Pattern 2: Test service pattern
    pattern2 = r'"http://test-zfs-service:"\.to_string\(\) \+ ":" \+ &std::env::var\("NESTGATE_PORT"\)\.unwrap_or_else\(\|_\| "\$\{NESTGATE_PORT:-8080\}"\.to_string\(\)\) \+ ""'
    replacement2 = r'format!("http://test-zfs-service:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))'
    content = re.sub(pattern2, replacement2, content)
    
    # Pattern 3: Mock service pattern
    pattern3 = r'"http://mock-service:"\.to_string\(\) \+ ":" \+ &std::env::var\("NESTGATE_PORT"\)\.unwrap_or_else\(\|_\| "\$\{NESTGATE_PORT:-8080\}"\.to_string\(\)\) \+ ""'
    replacement3 = r'format!("http://mock-service:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))'
    content = re.sub(pattern3, replacement3, content)
    
    # Pattern 4: Mock toadstool pattern
    pattern4 = r'"http://mock-toadstool:"\.to_string\(\) \+ ":" \+ &std::env::var\("NESTGATE_PORT"\)\.unwrap_or_else\(\|_\| "\$\{NESTGATE_PORT:-8080\}"\.to_string\(\)\) \+ ""'
    replacement4 = r'format!("http://mock-toadstool:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))'
    content = re.sub(pattern4, replacement4, content)
    
    # Pattern 5: 127.0.0.1 pattern
    pattern5 = r'"http://127\.0\.0\.1:"\.to_string\(\) \+ ":" \+ &std::env::var\("NESTGATE_PORT"\)\.unwrap_or_else\(\|_\| "\$\{NESTGATE_PORT:-8080\}"\.to_string\(\)\) \+ ""'
    replacement5 = r'format!("http://127.0.0.1:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))'
    content = re.sub(pattern5, replacement5, content)
    
    # Pattern 6: Bind address patterns
    pattern6 = r'"0\.0\.0\.0:"\.to_string\(\) \+ ":" \+ &std::env::var\("NESTGATE_PORT"\)\.unwrap_or_else\(\|_\| "\$\{NESTGATE_PORT:-8080\}"\.to_string\(\)\) \+ ""'
    replacement6 = r'format!("0.0.0.0:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))'
    content = re.sub(pattern6, replacement6, content)
    
    pattern7 = r'"127\.0\.0\.1:"\.to_string\(\) \+ ":" \+ &std::env::var\("NESTGATE_PORT"\)\.unwrap_or_else\(\|_\| "\$\{NESTGATE_PORT:-8080\}"\.to_string\(\)\) \+ ""'
    replacement7 = r'format!("127.0.0.1:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))'
    content = re.sub(pattern7, replacement7, content)
    
    # Pattern 8: WebSocket patterns
    pattern8 = r'"ws://localhost:"\.to_string\(\) \+ ":" \+ &std::env::var\("NESTGATE_PORT"\)\.unwrap_or_else\(\|_\| "\$\{NESTGATE_PORT:-8080\}"\.to_string\(\)\) \+ ""'
    replacement8 = r'format!("ws://localhost:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))'
    content = re.sub(pattern8, replacement8, content)
    
    # Pattern 9: Simple localhost pattern
    pattern9 = r'"localhost:"\.to_string\(\) \+ ":" \+ &std::env::var\("NESTGATE_PORT"\)\.unwrap_or_else\(\|_\| "\$\{NESTGATE_PORT:-8080\}"\.to_string\(\)\) \+ ""'
    replacement9 = r'format!("localhost:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))'
    content = re.sub(pattern9, replacement9, content)
    
    # Fix any remaining ${NESTGATE_PORT:-8080} patterns
    content = re.sub(r'\$\{NESTGATE_PORT:-8080\}', '8080', content)
    
    return content

def process_file(file_path):
    """Process a single file to fix string concatenations."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            original_content = f.read()
        
        fixed_content = fix_string_concatenation(original_content)
        
        if fixed_content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(fixed_content)
            print(f"✅ Fixed: {file_path}")
            return True
        else:
            return False
    except Exception as e:
        print(f"❌ Error processing {file_path}: {e}")
        return False

def main():
    """Main function to process all files."""
    print("🚀 Starting canonical string concatenation fixes...")
    
    # Get all the files that need fixing
    files_to_fix = [
        "code/crates/nestgate-network/src/connection_manager.rs",
        "code/crates/nestgate-network/src/service_discovery.rs", 
        "code/crates/nestgate-performance/src/zero_copy_networking.rs",
        "code/crates/nestgate-middleware/src/config/security.rs",
        "code/crates/nestgate-automation/src/types/config.rs",
        "code/crates/nestgate-automation/src/types/ecosystem.rs",
        "code/crates/nestgate-core/src/config/federation.rs",
        "code/crates/nestgate-core/src/config/network.rs",
        "code/crates/nestgate-core/src/config/environment.rs",
        "code/crates/nestgate-core/src/config/canonical/domain_configs/test_configs.rs",
        "code/crates/nestgate-core/src/unified_benchmark_config.rs",
        "code/crates/nestgate-core/src/zero_cost/migrated_storage_provider.rs",
        "code/crates/nestgate-core/src/enterprise/clustering.rs",
        "code/crates/nestgate-core/src/services/native_async/development.rs",
        "code/crates/nestgate-core/src/utils/network.rs",
        "code/crates/nestgate-api/src/standards_integration_example.rs",
        "code/crates/nestgate-api/src/handlers/zfs/universal_zfs/backends/remote/tests.rs",
        "code/crates/nestgate-api/src/handlers/hardware_tuning_test.rs",
        "code/crates/nestgate-api/src/unified_api_config/primal_extensions.rs",
        "code/crates/nestgate-api/src/rest/rpc/json_rpc_service.rs"
    ]
    
    fixed_count = 0
    for file_path in files_to_fix:
        if os.path.exists(file_path):
            if process_file(file_path):
                fixed_count += 1
        else:
            print(f"⚠️ File not found: {file_path}")
    
    print(f"\n🎉 Canonical modernization complete!")
    print(f"📊 Fixed {fixed_count} files")
    print(f"🔄 Next: Run 'cargo fmt' to apply formatting")

if __name__ == "__main__":
    main() 