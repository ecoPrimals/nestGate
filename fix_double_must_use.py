#!/usr/bin/env python3
"""
Automated fix for double_must_use clippy errors.
Removes #[must_use] from functions returning Result<>, which is already must_use.
"""

import re
import sys
from pathlib import Path

def fix_file(filepath):
    """Fix double_must_use in a single file."""
    print(f"Processing: {filepath}")
    
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original_content = content
    
    # Pattern to match #[must_use] followed by a function returning Result<>
    # This handles multiline cases
    pattern = r'(\s+)#\[must_use\]\s*\n(\s+pub\s+(?:async\s+)?fn\s+\w+[^{]*Result<[^{]+)'
    
    # Replace: remove the #[must_use] line
    content = re.sub(pattern, r'\1\2', content)
    
    if content != original_content:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        return True
    return False

def main():
    """Process all files listed in /tmp/must_use_results.txt"""
    
    # Read the list of files with line numbers
    try:
        with open('/tmp/must_use_results.txt', 'r') as f:
            lines = f.readlines()
    except FileNotFoundError:
        print("Error: /tmp/must_use_results.txt not found")
        print("Run ./fix_clippy.sh first to generate the file list")
        sys.exit(1)
    
    # Extract unique file paths
    files = set()
    for line in lines:
        if ':' in line:
            filepath = line.split(':')[0].strip()
            files.add(filepath)
    
    print(f"Found {len(files)} files to process")
    print()
    
    fixed_count = 0
    for filepath in sorted(files):
        if Path(filepath).exists():
            if fix_file(filepath):
                fixed_count += 1
        else:
            print(f"Warning: File not found: {filepath}")
    
    print()
    print(f"Fixed {fixed_count} out of {len(files)} files")
    print()
    print("Next steps:")
    print("1. Run: cargo fmt")
    print("2. Run: cargo clippy --lib -- -D warnings")
    print("3. Verify all errors are resolved")

if __name__ == '__main__':
    main()

