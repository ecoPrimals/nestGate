#!/usr/bin/env python3
"""
Auto-Documentation Tool for Rust Struct Fields

Automatically adds documentation comments to undocumented struct and enum fields
in Rust source files. Uses intelligent heuristics to generate meaningful docs.
"""

import re
import sys
from pathlib import Path
from typing import List, Tuple

def generate_field_doc(field_name: str, field_type: str) -> str:
    """Generate documentation for a field based on its name and type."""
    
    # Common patterns for field names
    if field_name == "id":
        return "Unique identifier"
    elif field_name == "name":
        return "Name"
    elif field_name.endswith("_id"):
        entity = field_name[:-3].replace("_", " ").title()
        return f"{entity} identifier"
    elif field_name.endswith("_name"):
        entity = field_name[:-5].replace("_", " ").title()
        return f"{entity} name"
    elif field_name == "enabled":
        return "Whether this feature is enabled"
    elif field_name == "description":
        return "Human-readable description"
    elif field_name == "metadata":
        return "Additional metadata key-value pairs"
    elif field_name == "created_at":
        return "Timestamp when this was created"
    elif field_name == "updated_at":
        return "Timestamp of last update"
    elif field_name == "capacity" or field_name.endswith("_capacity"):
        return f"{field_name.replace('_', ' ').title()}"
    elif field_name.startswith("is_"):
        return f"Whether {field_name[3:].replace('_', ' ')}"
    elif field_name.startswith("has_"):
        return f"Whether this has {field_name[4:].replace('_', ' ')}"
    elif field_name.startswith("num_") or field_name.startswith("count_"):
        return f"Number of {field_name.split('_', 1)[1].replace('_', ' ')}"
    elif field_name.endswith("_count"):
        return f"Count of {field_name[:-6].replace('_', ' ')}"
    elif field_name.endswith("_size"):
        return f"Size of {field_name[:-5].replace('_', ' ')}"
    elif field_name.endswith("_mb"):
        return f"{field_name[:-3].replace('_', ' ').title()} in megabytes"
    elif field_name.endswith("_gb"):
        return f"{field_name[:-3].replace('_', ' ').title()} in gigabytes"
    elif "config" in field_name.lower():
        return f"Configuration for {field_name.replace('config', '').replace('_', ' ').strip()}"
    elif "settings" in field_name.lower():
        return f"Settings for {field_name.replace('settings', '').replace('_', ' ').strip()}"
    else:
        # Generic fallback based on field name
        return field_name.replace("_", " ").title()

def add_field_documentation(content: str) -> Tuple[str, int]:
    """Add documentation to undocumented struct/enum fields."""
    
    lines = content.split('\n')
    result_lines = []
    additions = 0
    i = 0
    
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        # Check if this is a public field without documentation
        if stripped.startswith('pub ') and ':' in stripped:
            # Check if previous line is a doc comment
            if i > 0 and result_lines:
                prev_line = result_lines[-1].strip()
                if not prev_line.startswith('///') and not prev_line.startswith('//'):
                    # Extract field name and type
                    field_match = re.match(r'pub\s+(\w+):\s*(.+?)(,|$)', stripped)
                    if field_match:
                        field_name = field_match.group(1)
                        field_type = field_match.group(2).strip()
                        
                        # Generate documentation
                        doc = generate_field_doc(field_name, field_type)
                        indent = line[:len(line) - len(line.lstrip())]
                        doc_line = f"{indent}/// {doc}"
                        
                        result_lines.append(doc_line)
                        additions += 1
        
        result_lines.append(line)
        i += 1
    
    return '\n'.join(result_lines), additions

def process_file(file_path: Path) -> int:
    """Process a single Rust file and add documentation."""
    
    try:
        content = file_path.read_text()
        new_content, additions = add_field_documentation(content)
        
        if additions > 0:
            file_path.write_text(new_content)
            print(f"✅ {file_path}: Added {additions} field docs")
            return additions
        else:
            print(f"⏭️  {file_path}: No changes needed")
            return 0
    except Exception as e:
        print(f"❌ {file_path}: Error - {e}")
        return 0

def main():
    if len(sys.argv) < 2:
        print("Usage: auto_document_fields.py <path-to-rust-file-or-directory>")
        sys.exit(1)
    
    target = Path(sys.argv[1])
    
    if target.is_file():
        if target.suffix == '.rs':
            additions = process_file(target)
            print(f"\n✨ Total: {additions} documentation comments added")
        else:
            print("Error: Not a Rust file")
            sys.exit(1)
    elif target.is_dir():
        total_additions = 0
        rust_files = list(target.rglob('*.rs'))
        
        for rust_file in rust_files:
            # Skip test files and generated files
            if 'target' in rust_file.parts or 'tests.rs' in rust_file.name:
                continue
            
            total_additions += process_file(rust_file)
        
        print(f"\n✨ Total: {total_additions} documentation comments added across {len(rust_files)} files")
    else:
        print("Error: Path does not exist")
        sys.exit(1)

if __name__ == "__main__":
    main()

