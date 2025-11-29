#!/usr/bin/env python3
"""
Unwrap Migration Tool

Automatically migrates .unwrap() and .expect() calls to proper error handling
using Result types and the ? operator.
"""

import re
import sys
from pathlib import Path
from typing import List, Tuple, Optional

def detect_error_type(file_content: str) -> str:
    """Detect the error type used in the file."""
    
    # Check for common error types
    if "NestGateError" in file_content:
        return "NestGateError"
    elif "anyhow::Error" in file_content or "use anyhow" in file_content:
        return "anyhow::Error"
    elif "Box<dyn std::error::Error>" in file_content:
        return "Box<dyn std::error::Error>"
    elif "Box<dyn Error>" in file_content:
        return "Box<dyn Error>"
    else:
        return "NestGateError"  # Default

def detect_return_type(function_def: str) -> Optional[str]:
    """Extract return type from function definition."""
    
    match = re.search(r'->\s*([^{]+)\s*{', function_def)
    if match:
        return match.group(1).strip()
    return None

def needs_result_wrapper(return_type: Optional[str]) -> bool:
    """Check if function needs Result wrapper."""
    
    if not return_type:
        return True
    
    return "Result" not in return_type

def migrate_unwrap_to_context(line: str, error_type: str) -> Tuple[str, bool]:
    """Migrate .unwrap() to .context() or ? operator."""
    
    # Pattern: something.unwrap()
    unwrap_pattern = r'(\w+(?:\.\w+|\(.*?\))*?)\.unwrap\(\)'
    
    if '.unwrap()' in line:
        # Try to add context
        if error_type == "anyhow::Error":
            new_line = re.sub(
                unwrap_pattern,
                r'\1.context("Failed to unwrap")?',
                line
            )
        else:
            new_line = re.sub(
                unwrap_pattern,
                r'\1.map_err(|e| NestGateError::Internal(format!("Unwrap failed: {:?}", e)))?',
                line
            )
        return new_line, new_line != line
    
    return line, False

def migrate_expect_to_context(line: str, error_type: str) -> Tuple[str, bool]:
    """Migrate .expect("msg") to proper error handling."""
    
    # Pattern: something.expect("message")
    expect_pattern = r'(\w+(?:\.\w+|\(.*?\))*?)\.expect\("([^"]+)"\)'
    
    match = re.search(expect_pattern, line)
    if match:
        expr = match.group(1)
        msg = match.group(2)
        
        if error_type == "anyhow::Error":
            new_line = line.replace(
                match.group(0),
                f'{expr}.context("{msg}")?'
            )
        else:
            new_line = line.replace(
                match.group(0),
                f'{expr}.map_err(|e| NestGateError::Internal(format!("{msg}: {{:?}}", e)))?'
            )
        return new_line, True
    
    return line, False

def migrate_file(file_path: Path, dry_run: bool = False) -> Tuple[int, int]:
    """Migrate unwraps in a file."""
    
    try:
        content = file_path.read_text()
        lines = content.split('\n')
        
        error_type = detect_error_type(content)
        unwraps_migrated = 0
        expects_migrated = 0
        
        result_lines = []
        in_test = False
        
        for i, line in enumerate(lines):
            # Skip test code
            if '#[test]' in line or '#[cfg(test)]' in line:
                in_test = True
            elif in_test and (line.strip().startswith('fn ') or line.strip().startswith('pub fn ')):
                if '{' in line:
                    in_test = False
            
            if in_test or line.strip().startswith('//'):
                result_lines.append(line)
                continue
            
            # Try to migrate unwrap
            new_line, changed = migrate_unwrap_to_context(line, error_type)
            if changed:
                unwraps_migrated += 1
                result_lines.append(new_line)
                continue
            
            # Try to migrate expect
            new_line, changed = migrate_expect_to_context(line, error_type)
            if changed:
                expects_migrated += 1
                result_lines.append(new_line)
                continue
            
            result_lines.append(line)
        
        if not dry_run and (unwraps_migrated > 0 or expects_migrated > 0):
            new_content = '\n'.join(result_lines)
            file_path.write_text(new_content)
            print(f"✅ {file_path}: {unwraps_migrated} unwraps, {expects_migrated} expects")
        elif unwraps_migrated > 0 or expects_migrated > 0:
            print(f"🔍 {file_path}: Would migrate {unwraps_migrated} unwraps, {expects_migrated} expects")
        
        return unwraps_migrated, expects_migrated
        
    except Exception as e:
        print(f"❌ {file_path}: Error - {e}")
        return 0, 0

def main():
    if len(sys.argv) < 2:
        print("Usage: unwrap_migrator.py <path> [--dry-run]")
        sys.exit(1)
    
    target = Path(sys.argv[1])
    dry_run = '--dry-run' in sys.argv
    
    if target.is_file():
        if target.suffix == '.rs':
            unwraps, expects = migrate_file(target, dry_run)
            print(f"\n✨ Total: {unwraps} unwraps, {expects} expects migrated")
        else:
            print("Error: Not a Rust file")
            sys.exit(1)
    elif target.is_dir():
        total_unwraps = 0
        total_expects = 0
        rust_files = list(target.rglob('*.rs'))
        
        for rust_file in rust_files:
            if 'target' in rust_file.parts:
                continue
            
            unwraps, expects = migrate_file(rust_file, dry_run)
            total_unwraps += unwraps
            total_expects += expects
        
        print(f"\n✨ Total: {total_unwraps} unwraps, {total_expects} expects across {len(rust_files)} files")
    else:
        print("Error: Path does not exist")
        sys.exit(1)

if __name__ == "__main__":
    main()

