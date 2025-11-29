#!/usr/bin/env python3
"""
Auto-Documentation Tool for Methods, Functions, and Variants

Adds documentation for:
- Methods (impl blocks)
- Associated functions
- Module-level functions
- Enum variants with data
"""

import re
import sys
from pathlib import Path
from typing import List, Tuple

def to_sentence_case(snake_case: str) -> str:
    """Convert snake_case to Sentence case."""
    return snake_case.replace('_', ' ').title()

def generate_method_doc(method_name: str, params: str, return_type: str = "") -> str:
    """Generate documentation for a method."""
    
    # Common method patterns
    if method_name == "new":
        return "Creates a new instance"
    elif method_name == "default":
        return "Returns the default instance"
    elif method_name.startswith("get_"):
        return f"Gets {to_sentence_case(method_name[4:])}"
    elif method_name.startswith("set_"):
        return f"Sets {to_sentence_case(method_name[4:])}"
    elif method_name.startswith("is_"):
        return f"Checks if {to_sentence_case(method_name[3:])}"
    elif method_name.startswith("has_"):
        return f"Checks if has {to_sentence_case(method_name[4:])}"
    elif method_name.startswith("with_"):
        return f"Builder method to set {to_sentence_case(method_name[5:])}"
    elif method_name.startswith("build"):
        return "Builds the final instance"
    elif method_name.startswith("create"):
        return f"Creates {to_sentence_case(method_name[6:]) if len(method_name) > 6 else 'instance'}"
    elif method_name.startswith("update"):
        return f"Updates {to_sentence_case(method_name[6:]) if len(method_name) > 6 else 'state'}"
    elif method_name.startswith("delete"):
        return f"Deletes {to_sentence_case(method_name[6:]) if len(method_name) > 6 else 'resource'}"
    elif method_name.startswith("validate"):
        return f"Validates {to_sentence_case(method_name[8:]) if len(method_name) > 8 else 'data'}"
    elif method_name.startswith("process"):
        return f"Processes {to_sentence_case(method_name[7:]) if len(method_name) > 7 else 'data'}"
    elif method_name.startswith("handle"):
        return f"Handles {to_sentence_case(method_name[6:]) if len(method_name) > 6 else 'request'}"
    elif method_name.startswith("parse"):
        return f"Parses {to_sentence_case(method_name[5:]) if len(method_name) > 5 else 'input'}"
    elif method_name.startswith("format"):
        return f"Formats {to_sentence_case(method_name[6:]) if len(method_name) > 6 else 'output'}"
    elif method_name.startswith("convert"):
        return f"Converts {to_sentence_case(method_name[7:]) if len(method_name) > 7 else 'value'}"
    elif method_name.startswith("to_"):
        return f"Converts to {to_sentence_case(method_name[3:])}"
    elif method_name.startswith("from_"):
        return f"Creates from {to_sentence_case(method_name[5:])}"
    elif method_name.startswith("as_"):
        return f"Returns as {to_sentence_case(method_name[3:])}"
    else:
        return to_sentence_case(method_name)

def add_method_documentation(content: str) -> Tuple[str, int]:
    """Add documentation to methods and functions."""
    
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
        
        if not has_doc and not stripped.startswith('//'):
            # Public function/method
            fn_match = re.match(r'(pub\s+)?(async\s+)?fn\s+(\w+)', stripped)
            if fn_match:
                method_name = fn_match.group(3)
                
                # Skip test functions
                if not method_name.startswith('test_'):
                    # Try to extract parameters and return type
                    params = ""
                    return_type = ""
                    
                    # Look for -> return type
                    if '->' in stripped:
                        return_match = re.search(r'->\s*([^{]+)', stripped)
                        if return_match:
                            return_type = return_match.group(1).strip()
                    
                    doc = generate_method_doc(method_name, params, return_type)
                    result_lines.append(f"{indent}/// {doc}")
                    additions += 1
            
            # Enum variant with data
            variant_match = re.match(r'(\w+)\s*\{', stripped)
            if variant_match and i > 0:
                # Check if we're in an enum
                for j in range(max(0, i-10), i):
                    if 'enum' in lines[j]:
                        variant_name = variant_match.group(1)
                        if variant_name not in ['Some', 'None', 'Ok', 'Err']:
                            doc = to_sentence_case(variant_name)
                            result_lines.append(f"{indent}/// {doc}")
                            additions += 1
                        break
        
        result_lines.append(line)
        i += 1
    
    return '\n'.join(result_lines), additions

def process_file(file_path: Path) -> int:
    """Process a single Rust file."""
    
    try:
        content = file_path.read_text()
        new_content, additions = add_method_documentation(content)
        
        if additions > 0:
            file_path.write_text(new_content)
            print(f"✅ {file_path}: Added {additions} method/function docs")
            return additions
        return 0
    except Exception as e:
        print(f"❌ {file_path}: Error - {e}")
        return 0

def main():
    if len(sys.argv) < 2:
        print("Usage: auto_document_methods.py <path>")
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

