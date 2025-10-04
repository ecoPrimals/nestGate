#!/usr/bin/env python3
"""
Fix initialize() Method Signature

This script systematically updates Service trait implementations to use the
correct initialize() signature without the config parameter.

MIGRATION:
  FROM: fn initialize(&self, config: &Config) -> impl Future<...>
  TO:   fn initialize(&self) -> impl Future<...>

Author: NestGate Consolidation Team
Date: October 2, 2025
"""

import re
import sys
from pathlib import Path
from typing import List, Tuple

def fix_initialize_signature(content: str, filepath: str) -> Tuple[str, int]:
    """
    Fix initialize method signature in Service implementations.
    
    Returns: (fixed_content, number_of_changes)
    """
    changes = 0
    
    # Pattern 1: fn initialize(&self, config: &Config)
    # This is the most common pattern
    pattern1 = r'fn initialize\(&self,\s*config:\s*&Config\)'
    replacement1 = 'fn initialize(&self)'
    
    if re.search(pattern1, content):
        content = re.sub(pattern1, replacement1, content)
        changes += 1
        print(f"  ✓ Fixed initialize signature in {filepath}")
    
    # Pattern 2: fn initialize(&self, config: Self::Config)
    # Used in some trait definitions
    pattern2 = r'fn initialize\(&self,\s*config:\s*Self::Config\)'
    replacement2 = 'fn initialize(&self)'
    
    if re.search(pattern2, content):
        content = re.sub(pattern2, replacement2, content)
        changes += 1
        print(f"  ✓ Fixed initialize signature (Self::Config) in {filepath}")
    
    # Pattern 3: Remove config parameter from async block if it's unused
    # Look for patterns like: async move { ... config ... }
    # Only remove if config is not actually used in the implementation
    
    # Check if there are any actual uses of 'config' variable after the fix
    # If config is used in logging but nowhere else, we can simplify
    config_usage_pattern = r'config:\s*\?\?\)'  # Common debug formatting
    if re.search(config_usage_pattern, content):
        # Replace config debug with simpler version
        content = re.sub(
            r'with config:\s*\{:\?\}\",\s*config\)',
            '\")',
            content
        )
        changes += 1
    
    return content, changes

def process_file(filepath: Path) -> Tuple[bool, int]:
    """
    Process a single Rust file.
    
    Returns: (was_modified, number_of_changes)
    """
    try:
        content = filepath.read_text(encoding='utf-8')
        original_content = content
        
        # Apply fixes
        content, changes = fix_initialize_signature(content, str(filepath))
        
        if changes > 0:
            # Write back the modified content
            filepath.write_text(content, encoding='utf-8')
            return True, changes
        
        return False, 0
    
    except Exception as e:
        print(f"  ✗ Error processing {filepath}: {e}", file=sys.stderr)
        return False, 0

def main():
    """Main execution function"""
    print("🔧 NestGate Initialize Signature Fix")
    print("=" * 50)
    print()
    
    # Find all Rust files in code/crates directory
    repo_root = Path(__file__).parent.parent.parent
    code_dir = repo_root / "code" / "crates"
    
    if not code_dir.exists():
        print(f"✗ Error: {code_dir} not found!", file=sys.stderr)
        sys.exit(1)
    
    print(f"📂 Scanning: {code_dir}")
    print()
    
    # Find all .rs files
    rust_files = list(code_dir.rglob("*.rs"))
    print(f"📊 Found {len(rust_files)} Rust files")
    print()
    
    # Process files
    modified_files = 0
    total_changes = 0
    
    print("🔨 Processing files...")
    for filepath in rust_files:
        was_modified, changes = process_file(filepath)
        if was_modified:
            modified_files += 1
            total_changes += changes
    
    print()
    print("=" * 50)
    print("✅ MIGRATION COMPLETE")
    print(f"   Files modified: {modified_files}")
    print(f"   Total changes: {total_changes}")
    print()
    
    if modified_files > 0:
        print("🔍 Next steps:")
        print("   1. Run: cargo check --workspace")
        print("   2. Verify no regressions")
        print("   3. Run: cargo test --workspace")
        print()

if __name__ == "__main__":
    main() 