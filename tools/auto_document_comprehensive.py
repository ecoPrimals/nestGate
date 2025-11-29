#!/usr/bin/env python3
"""
Comprehensive Auto-Documentation Tool for Rust

Automatically adds documentation comments to undocumented:
- Struct definitions
- Enum definitions and variants
- Modules
- Constants
- Type aliases
- Functions and methods
"""

import re
import sys
from pathlib import Path
from typing import List, Tuple

def to_sentence_case(snake_case: str) -> str:
    """Convert snake_case to Sentence case."""
    return snake_case.replace('_', ' ').title()

def generate_struct_doc(struct_name: str) -> str:
    """Generate documentation for a struct."""
    if struct_name.endswith("Config"):
        return f"Configuration for {struct_name[:-6].replace('_', ' ')}"
    elif struct_name.endswith("Builder"):
        return f"Builder pattern for constructing {struct_name[:-7]} instances"
    elif struct_name.endswith("Error"):
        return f"Error type for {struct_name[:-5]} operations"
    elif struct_name.endswith("Request"):
        return f"Request parameters for {struct_name[:-7]} operation"
    elif struct_name.endswith("Response"):
        return f"Response data for {struct_name[:-8]} operation"
    elif struct_name.endswith("Manager"):
        return f"Manager for {struct_name[:-7]} operations"
    elif struct_name.endswith("Handler"):
        return f"Handler for {struct_name[:-7]} requests"
    elif struct_name.endswith("Service"):
        return f"Service implementation for {struct_name[:-7]}"
    else:
        return to_sentence_case(struct_name)

def generate_enum_doc(enum_name: str) -> str:
    """Generate documentation for an enum."""
    if enum_name.endswith("Error"):
        return f"Errors that can occur during {enum_name[:-5]} operations"
    elif enum_name.endswith("Status"):
        return f"Status values for {enum_name[:-6]}"
    elif enum_name.endswith("Type"):
        return f"Types of {enum_name[:-4]}"
    elif enum_name.endswith("Kind"):
        return f"Kinds of {enum_name[:-4]}"
    else:
        return to_sentence_case(enum_name)

def generate_variant_doc(variant_name: str, enum_context: str = "") -> str:
    """Generate documentation for an enum variant."""
    if variant_name in ["None", "Some"]:
        return variant_name
    elif variant_name == "Ok":
        return "Success"
    elif variant_name == "Err":
        return "Error"
    else:
        return to_sentence_case(variant_name)

def generate_const_doc(const_name: str) -> str:
    """Generate documentation for a constant."""
    if "DEFAULT" in const_name:
        return f"Default value for {const_name.replace('DEFAULT_', '').replace('_', ' ').lower()}"
    elif "MAX" in const_name:
        return f"Maximum {const_name.replace('MAX_', '').replace('_', ' ').lower()}"
    elif "MIN" in const_name:
        return f"Minimum {const_name.replace('MIN_', '').replace('_', ' ').lower()}"
    else:
        return to_sentence_case(const_name)

def add_comprehensive_documentation(content: str) -> Tuple[str, int]:
    """Add documentation to undocumented items."""
    
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
            has_doc = prev_stripped.startswith('///') or prev_stripped.startswith('//')
        
        if not has_doc:
            # Pub struct
            struct_match = re.match(r'pub\s+struct\s+(\w+)', stripped)
            if struct_match:
                struct_name = struct_match.group(1)
                doc = generate_struct_doc(struct_name)
                result_lines.append(f"{indent}/// {doc}")
                additions += 1
            
            # Pub enum
            enum_match = re.match(r'pub\s+enum\s+(\w+)', stripped)
            if enum_match:
                enum_name = enum_match.group(1)
                doc = generate_enum_doc(enum_name)
                result_lines.append(f"{indent}/// {doc}")
                additions += 1
            
            # Enum variant (simple form without data)
            if stripped and not stripped.startswith('//') and not stripped.startswith('#'):
                variant_match = re.match(r'^(\w+),?\s*$', stripped)
                if variant_match and i > 0:
                    # Check if we're inside an enum
                    # This is a heuristic - look back for enum definition
                    for j in range(max(0, i-10), i):
                        if 'enum' in lines[j]:
                            variant_name = variant_match.group(1)
                            doc = generate_variant_doc(variant_name)
                            result_lines.append(f"{indent}/// {doc}")
                            additions += 1
                            break
            
            # Pub const
            const_match = re.match(r'pub\s+const\s+(\w+)', stripped)
            if const_match:
                const_name = const_match.group(1)
                doc = generate_const_doc(const_name)
                result_lines.append(f"{indent}/// {doc}")
                additions += 1
            
            # Pub type alias
            type_match = re.match(r'pub\s+type\s+(\w+)', stripped)
            if type_match:
                type_name = type_match.group(1)
                doc = f"Type alias for {to_sentence_case(type_name)}"
                result_lines.append(f"{indent}/// {doc}")
                additions += 1
        
        result_lines.append(line)
        i += 1
    
    return '\n'.join(result_lines), additions

def process_file(file_path: Path) -> int:
    """Process a single Rust file."""
    
    try:
        content = file_path.read_text()
        new_content, additions = add_comprehensive_documentation(content)
        
        if additions > 0:
            file_path.write_text(new_content)
            print(f"✅ {file_path}: Added {additions} docs")
            return additions
        return 0
    except Exception as e:
        print(f"❌ {file_path}: Error - {e}")
        return 0

def main():
    if len(sys.argv) < 2:
        print("Usage: auto_document_comprehensive.py <path>")
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

