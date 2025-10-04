#!/usr/bin/env python3
"""
Fix const function violations in nestgate-core
This script systematically replaces const functions that can't be const.
"""

import os
import re
import glob

def fix_const_functions():
    """Fix const function violations in all Rust files"""
    
    # Pattern to match const functions returning String, HashMap, or using complex operations
    patterns_to_fix = [
        # Functions returning String
        (r'pub const fn ([^(]+\([^)]*\)) -> String', r'pub fn \1 -> String'),
        # Functions returning HashMap  
        (r'pub const fn ([^(]+\([^)]*\)) -> HashMap<[^>]+>', r'pub fn \1 -> HashMap<\2>'),
        # Functions returning Vec<String>
        (r'pub const fn ([^(]+\([^)]*\)) -> Vec<String>', r'pub fn \1 -> Vec<String>'),
        # Functions returning Result<String>
        (r'pub const fn ([^(]+\([^)]*\)) -> Result<String[^>]*>', r'pub fn \1 -> Result<String\2>'),
        # Functions returning Option<String>
        (r'pub const fn ([^(]+\([^)]*\)) -> Option<String>', r'pub fn \1 -> Option<String>'),
        # Functions returning complex types with generics
        (r'pub const fn ([^(]+\([^)]*\)) -> Result<[^,]+, String>', r'pub fn \1 -> Result<\2, String>'),
    ]
    
    # Find all Rust files in nestgate-core
    rust_files = glob.glob('code/crates/nestgate-core/src/**/*.rs', recursive=True)
    
    fixed_files = []
    
    for file_path in rust_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            
            # Apply all patterns
            for pattern, replacement in patterns_to_fix:
                content = re.sub(pattern, replacement, content)
            
            # If content changed, write it back
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                fixed_files.append(file_path)
                print(f"Fixed: {file_path}")
        
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    print(f"\nFixed {len(fixed_files)} files:")
    for file_path in fixed_files:
        print(f"  - {file_path}")

if __name__ == "__main__":
    fix_const_functions() 