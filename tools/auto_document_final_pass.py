#!/usr/bin/env python3
"""
Final Pass Auto-Documentation Tool

Handles remaining documentation warnings:
- Module-level documentation
- Associated types
- Type aliases
- Traits
- Anything else missed
"""

import re
import sys
from pathlib import Path

def add_module_doc_if_missing(content: str, file_path: Path) -> tuple[str, int]:
    """Add module-level documentation if missing."""
    
    lines = content.split('\n')
    
    # Check if file already has module doc
    has_module_doc = False
    for i, line in enumerate(lines[:10]):  # Check first 10 lines
        if line.strip().startswith('//!') or line.strip().startswith('/// '):
            has_module_doc = True
            break
    
    if not has_module_doc:
        # Generate module doc from file path
        module_name = file_path.stem
        if module_name == 'mod':
            module_name = file_path.parent.name
        
        doc = f"//! {module_name.replace('_', ' ').title()} module\n"
        
        # Insert after any attributes but before first code
        insert_pos = 0
        for i, line in enumerate(lines):
            if line.strip() and not line.strip().startswith('#') and not line.strip().startswith('//'):
                insert_pos = i
                break
        
        lines.insert(insert_pos, doc)
        return '\n'.join(lines), 1
    
    return content, 0

def add_missing_docs(content: str) -> tuple[str, int]:
    """Add any other missing documentation."""
    
    lines = content.split('\n')
    result_lines = []
    additions = 0
    i = 0
    
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        indent = line[:len(line) - len(line.lstrip())]
        
        # Check if previous line has documentation
        has_doc = False
        if result_lines:
            prev_stripped = result_lines[-1].strip()
            has_doc = prev_stripped.startswith('///') or prev_stripped.startswith('//') or prev_stripped.startswith('#[')
        
        if not has_doc and stripped and not stripped.startswith('//'):
            # Type alias
            if stripped.startswith('pub type ') or stripped.startswith('type '):
                type_match = re.match(r'(?:pub\s+)?type\s+(\w+)', stripped)
                if type_match:
                    type_name = type_match.group(1)
                    doc = f"Type alias for {type_name.replace('_', ' ')}"
                    result_lines.append(f"{indent}/// {doc}")
                    additions += 1
            
            # Trait
            elif stripped.startswith('pub trait ') or stripped.startswith('trait '):
                trait_match = re.match(r'(?:pub\s+)?trait\s+(\w+)', stripped)
                if trait_match:
                    trait_name = trait_match.group(1)
                    doc = f"{trait_name.replace('_', ' ')} trait"
                    result_lines.append(f"{indent}/// {doc}")
                    additions += 1
            
            # Associated type
            elif stripped.startswith('type ') and i > 0:
                # Check if we're in a trait
                for j in range(max(0, i-10), i):
                    if 'trait' in lines[j]:
                        type_match = re.match(r'type\s+(\w+)', stripped)
                        if type_match:
                            type_name = type_match.group(1)
                            doc = f"Associated type for {type_name.replace('_', ' ')}"
                            result_lines.append(f"{indent}/// {doc}")
                            additions += 1
                        break
            
            # Const
            elif stripped.startswith('pub const ') or stripped.startswith('const '):
                const_match = re.match(r'(?:pub\s+)?const\s+(\w+)', stripped)
                if const_match:
                    const_name = const_match.group(1)
                    if 'DEFAULT' in const_name:
                        doc = f"Default value for {const_name.replace('DEFAULT_', '').replace('_', ' ').lower()}"
                    elif 'MAX' in const_name:
                        doc = f"Maximum {const_name.replace('MAX_', '').replace('_', ' ').lower()}"
                    elif 'MIN' in const_name:
                        doc = f"Minimum {const_name.replace('MIN_', '').replace('_', ' ').lower()}"
                    else:
                        doc = const_name.replace('_', ' ').title()
                    result_lines.append(f"{indent}/// {doc}")
                    additions += 1
        
        result_lines.append(line)
        i += 1
    
    return '\n'.join(result_lines), additions

def process_file(file_path: Path) -> int:
    """Process a single Rust file."""
    
    try:
        content = file_path.read_text()
        total_additions = 0
        
        # Add module doc
        content, mod_additions = add_module_doc_if_missing(content, file_path)
        total_additions += mod_additions
        
        # Add other missing docs
        content, other_additions = add_missing_docs(content)
        total_additions += other_additions
        
        if total_additions > 0:
            file_path.write_text(content)
            print(f"✅ {file_path}: Added {total_additions} docs")
        
        return total_additions
    except Exception as e:
        print(f"❌ {file_path}: Error - {e}")
        return 0

def main():
    if len(sys.argv) < 2:
        print("Usage: auto_document_final_pass.py <path>")
        sys.exit(1)
    
    target = Path(sys.argv[1])
    
    if target.is_file():
        additions = process_file(target)
        print(f"\n✨ Total: {additions} documentation comments added")
    elif target.is_dir():
        total_additions = 0
        rust_files = list(target.rglob('*.rs'))
        
        for rust_file in rust_files:
            if 'target' in rust_file.parts:
                continue
            total_additions += process_file(rust_file)
        
        print(f"\n✨ Total: {total_additions} docs added across {len(rust_files)} files")
    else:
        print("Error: Path does not exist")
        sys.exit(1)

if __name__ == "__main__":
    main()

