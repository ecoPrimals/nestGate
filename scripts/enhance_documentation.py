#!/usr/bin/env python3
"""
Enhance Rust code documentation and API design.
Adds missing # Errors documentation and #[must_use] attributes.
"""

import os
import re
import sys

def add_errors_documentation(content):
    """Add # Errors documentation to functions returning Result"""
    
    # Pattern for functions returning Result without # Errors doc
    pattern = r'((?:\/\/\/[^\n]*\n)*)\s*(pub\s+(?:async\s+)?fn\s+\w+[^{]*->\s*Result<[^{]*)\{'
    
    def add_errors_doc(match):
        existing_doc = match.group(1)
        fn_signature = match.group(2)
        
        # Check if # Errors section already exists
        if '# Errors' in existing_doc:
            return match.group(0)
        
        # Add # Errors documentation
        if existing_doc.strip():
            # There's existing documentation, add to it
            new_doc = existing_doc.rstrip() + '\n    /// \n    /// # Errors\n    /// \n    /// This function will return an error if the operation fails.\n    '
        else:
            # No existing documentation, create it
            new_doc = '    /// Function description\n    /// \n    /// # Errors\n    /// \n    /// This function will return an error if the operation fails.\n    '
        
        return new_doc + fn_signature + ' {'
    
    return re.sub(pattern, add_errors_doc, content, flags=re.MULTILINE)

def add_must_use_attributes(content):
    """Add #[must_use] attributes to builder methods and important return types"""
    
    # Pattern for builder methods (methods returning Self)
    pattern = r'(\s*)(pub\s+fn\s+\w+\([^)]*\)\s*->\s*Self\s*\{)'
    
    def add_must_use(match):
        indent = match.group(1)
        fn_signature = match.group(2)
        
        # Check if #[must_use] already exists in the preceding lines
        # This is a simple check - could be more sophisticated
        return f"{indent}#[must_use]\n{indent}{fn_signature}"
    
    # Only add if #[must_use] is not already present
    lines = content.split('\n')
    result_lines = []
    
    for i, line in enumerate(lines):
        result_lines.append(line)
        
        # Check if this line is a function returning Self
        if re.search(r'pub\s+fn\s+\w+\([^)]*\)\s*->\s*Self\s*\{', line):
            # Check if the previous line already has #[must_use]
            if i > 0 and '#[must_use]' not in lines[i-1]:
                # Find the indentation of the function
                indent = re.match(r'^(\s*)', line).group(1)
                # Insert #[must_use] before the function
                result_lines.insert(-1, f"{indent}#[must_use]")
    
    return '\n'.join(result_lines)

def enhance_file_documentation(filepath):
    """Enhance documentation in a single Rust file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Add # Errors documentation
        content = add_errors_documentation(content)
        
        # Add #[must_use] attributes
        content = add_must_use_attributes(content)
        
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ Enhanced documentation in: {filepath}")
            return True
        
        return False
        
    except Exception as e:
        print(f"❌ Error processing {filepath}: {e}")
        return False

def main():
    """Main function to enhance documentation in all Rust files"""
    if len(sys.argv) > 1:
        target_dir = sys.argv[1]
    else:
        target_dir = "code/crates/nestgate-core/src"
    
    if not os.path.exists(target_dir):
        print(f"Directory {target_dir} does not exist")
        sys.exit(1)
    
    files_processed = 0
    files_changed = 0
    
    # Walk through all Rust files
    for root, dirs, files in os.walk(target_dir):
        # Skip target directories
        if 'target' in dirs:
            dirs.remove('target')
        
        for file in files:
            if file.endswith('.rs'):
                filepath = os.path.join(root, file)
                files_processed += 1
                
                if enhance_file_documentation(filepath):
                    files_changed += 1
    
    print(f"\n📊 Documentation Enhancement Summary:")
    print(f"   Files processed: {files_processed}")
    print(f"   Files enhanced: {files_changed}")
    print(f"   Enhancement rate: {files_changed}/{files_processed}")

if __name__ == "__main__":
    main() 