#!/usr/bin/env python3
"""
Add SafeUnwrap imports to files that use .safe_unwrap()
"""

import sys
import re
from pathlib import Path

def add_import_to_file(file_path: Path) -> bool:
    """Add the SafeUnwrap import if needed. Returns True if modified."""
    content = file_path.read_text()
    
    # Check if file uses .safe_unwrap()
    if '.safe_unwrap(' not in content:
        return False
    
    # Check if import already exists
    if 'use crate::error::helpers::{ErrorCategory, SafeUnwrap}' in content or \
       'use crate::error::helpers::{SafeUnwrap, ErrorCategory}' in content or \
       'use crate::error::{ErrorCategory, SafeUnwrap}' in content or \
       'use nestgate_core::error::helpers::{ErrorCategory, SafeUnwrap}' in content:
        print(f"✓ {file_path}: Import already exists")
        return False
    
    lines = content.splitlines(keepends=True)
    
    # Find the last use statement
    last_use_idx = -1
    for i, line in enumerate(lines):
        if line.strip().startswith('use ') and not line.strip().startswith('use super'):
            last_use_idx = i
    
    # If we found use statements, add after the last one
    if last_use_idx >= 0:
        # Check if it's in the error module itself
        if '/error/' in str(file_path):
            import_line = 'use crate::error::helpers::{ErrorCategory, SafeUnwrap};\n'
        else:
            import_line = 'use crate::error::{ErrorCategory, SafeUnwrap};\n'
        
        lines.insert(last_use_idx + 1, import_line)
        file_path.write_text(''.join(lines))
        print(f"✓ {file_path}: Added import after line {last_use_idx + 1}")
        return True
    else:
        # No use statements found, add at the top after any module docs/comments
        insert_idx = 0
        for i, line in enumerate(lines):
            if line.strip() and not line.strip().startswith('//') and not line.strip().startswith('/*'):
                insert_idx = i
                break
        
        if '/error/' in str(file_path):
            import_line = 'use crate::error::helpers::{ErrorCategory, SafeUnwrap};\n\n'
        else:
            import_line = 'use crate::error::{ErrorCategory, SafeUnwrap};\n\n'
        
        lines.insert(insert_idx, import_line)
        file_path.write_text(''.join(lines))
        print(f"✓ {file_path}: Added import at top")
        return True

def main():
    if len(sys.argv) < 2:
        print("Usage: add_imports.py <file1> <file2> ...")
        sys.exit(1)
    
    modified_count = 0
    for file_arg in sys.argv[1:]:
        file_path = Path(file_arg)
        if file_path.exists():
            if add_import_to_file(file_path):
                modified_count += 1
        else:
            print(f"✗ {file_path}: File not found", file=sys.stderr)
    
    print(f"\n✅ Modified {modified_count} files")

if __name__ == '__main__':
    main()

