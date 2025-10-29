#!/usr/bin/env python3
import re
import sys
import os

def fix_async_function(content, func_name):
    """Fix a specific async function by wrapping the body in async move"""
    # Pattern to match function signature and body
    pattern = rf'(pub fn {func_name}\([^)]*\) -> impl std::future::Future<[^>]*> \+ Send \{{)'
    
    def replacement(match):
        return match.group(1) + '\n        async move {'
    
    # Replace the function start
    content = re.sub(pattern, replacement, content)
    
    # Find the matching closing brace and add an extra one for async move
    # This is a simplified approach - may need manual adjustment
    return content

def process_file(filepath):
    """Process a file to fix async issues"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Look for functions that use await but aren't properly wrapped
        if '.await' in content and 'async move {' not in content:
            # This file likely needs async move blocks
            lines = content.split('\n')
            new_lines = []
            in_async_fn = False
            brace_count = 0
            
            for line in lines:
                if 'impl std::future::Future' in line and '-> impl std::future::Future' in line:
                    in_async_fn = True
                    new_lines.append(line)
                    if '{' in line:
                        new_lines.append('        async move {')
                        brace_count = 1
                elif in_async_fn:
                    if '{' in line:
                        brace_count += line.count('{')
                    if '}' in line:
                        brace_count -= line.count('}')
                        if brace_count == 0:
                            new_lines.append('        }')
                            in_async_fn = False
                    new_lines.append(line)
                else:
                    new_lines.append(line)
            
            content = '\n'.join(new_lines)
        
        # Only write if changed
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"Fixed async issues in: {filepath}")
            return True
            
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        
    return False

if __name__ == "__main__":
    if len(sys.argv) > 1:
        process_file(sys.argv[1])
    else:
        print("Usage: fix_specific_async_files.py <file_path>")
