#!/usr/bin/env python3
"""
Safe migration: Convert .unwrap() to .expect() with descriptive messages.
This is a conservative first step that doesn't introduce Result propagation.
"""

import os
import re
import sys
from pathlib import Path
from typing import Dict, List, Tuple

# Category mappings based on file path and context
CATEGORY_MAPPINGS = {
    'config': 'Configuration error',
    'network': 'Network operation failed',
    'storage': 'Storage operation failed',
    'security': 'Security operation failed',
    'zfs': 'ZFS operation failed',
    'auth': 'Authentication failed',
    'cache': 'Cache operation failed',
    'cert': 'Certificate operation failed',
    'test': 'Test setup failed',
}

def infer_context(file_path: str, line: str) -> str:
    """Infer error context from file path and line content."""
    path_lower = file_path.lower()
    line_lower = line.lower()
    
    # Check file path
    for key, msg in CATEGORY_MAPPINGS.items():
        if key in path_lower:
            return msg
    
    # Check line content
    if 'parse' in line_lower:
        return 'Failed to parse value'
    if 'env::var' in line_lower or 'std::env::var' in line_lower:
        return 'Failed to read environment variable'
    if 'from_str' in line_lower:
        return 'Failed to convert from string'
    if 'to_string' in line_lower or 'format!' in line_lower:
        return 'String operation failed'
    if 'lock()' in line_lower:
        return 'Failed to acquire lock'
    if 'join()' in line_lower:
        return 'Thread join failed'
    if 'recv()' in line_lower or 'send()' in line_lower:
        return 'Channel operation failed'
    
    return 'Operation failed'

def is_safe_to_migrate(line: str) -> bool:
    """Check if this line is safe to migrate."""
    # Skip if already has expect
    if '.expect(' in line:
        return False
    
    # Skip if line is commented out
    stripped = line.lstrip()
    if stripped.startswith('//'):
        return False
    
    # Skip if in a string literal
    if is_in_string(line):
        return False
    
    return '.unwrap()' in line

def is_in_string(line: str) -> bool:
    """Check if .unwrap() appears inside a string literal."""
    # Simple heuristic: check if there are unbalanced quotes before .unwrap()
    unwrap_pos = line.find('.unwrap()')
    if unwrap_pos == -1:
        return False
    
    before_unwrap = line[:unwrap_pos]
    # Count double quotes
    double_quotes = before_unwrap.count('"') - before_unwrap.count('\\"')
    if double_quotes % 2 != 0:
        return True
    
    return False

def migrate_line(line: str, file_path: str) -> str:
    """Convert .unwrap() to .expect() with descriptive message."""
    if not is_safe_to_migrate(line):
        return line
    
    context = infer_context(file_path, line)
    
    # Replace .unwrap() with .expect("descriptive message")
    migrated = line.replace('.unwrap()', f'.expect("{context}")')
    
    return migrated

def migrate_file(file_path: Path) -> Tuple[bool, int]:
    """Migrate a single file. Returns (modified, count)."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return False, 0
    
    modified_lines = []
    migration_count = 0
    
    for line in lines:
        migrated_line = migrate_line(line, str(file_path))
        if migrated_line != line:
            migration_count += 1
        modified_lines.append(migrated_line)
    
    if migration_count > 0:
        try:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.writelines(modified_lines)
            return True, migration_count
        except Exception as e:
            print(f"Error writing {file_path}: {e}", file=sys.stderr)
            return False, 0
    
    return False, 0

def main():
    if len(sys.argv) < 2:
        print("Usage: safe_unwrap_to_expect.py <directory>")
        sys.exit(1)
    
    root_dir = Path(sys.argv[1])
    
    if not root_dir.exists():
        print(f"Directory not found: {root_dir}")
        sys.exit(1)
    
    # Find all .rs files
    rust_files = list(root_dir.rglob('*.rs'))
    
    # Exclude target, backup directories
    rust_files = [
        f for f in rust_files 
        if 'target' not in f.parts and 'backup' not in f.parts and '.git' not in f.parts
    ]
    
    print(f"Found {len(rust_files)} Rust files to process")
    
    total_files_modified = 0
    total_migrations = 0
    
    for rust_file in rust_files:
        modified, count = migrate_file(rust_file)
        if modified:
            total_files_modified += 1
            total_migrations += count
            print(f"✅ {rust_file}: {count} migrations")
    
    print(f"\n📊 Summary:")
    print(f"   Files modified: {total_files_modified}")
    print(f"   Total migrations: {total_migrations}")

if __name__ == '__main__':
    main()

