#!/usr/bin/env python3
"""
Systematically fix unused async functions in Rust code.
This script removes 'async' from functions that don't contain 'await' statements.
"""

import os
import re
import sys

def should_keep_async(function_body):
    """Check if function should keep async (has await or spawn statements)"""
    # Keep async if it has await statements
    if re.search(r'\.await\b', function_body):
        return True
    # Keep async if it spawns tasks
    if re.search(r'tokio::spawn|spawn\(', function_body):
        return True
    # Keep async if it's a trait implementation that might need async
    if re.search(r'#\[async_trait\]', function_body):
        return True
    return False

def fix_unused_async_in_file(filepath):
    """Fix unused async functions in a single file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Pattern to match async functions
        # This matches: pub async fn name(...) -> ReturnType {
        pattern = r'(\s*)(pub\s+)?async\s+(fn\s+\w+\s*\([^)]*\)\s*(?:->\s*[^{]+)?)\s*\{'
        
        def replace_async(match):
            indent = match.group(1)
            pub_prefix = match.group(2) or ''
            fn_signature = match.group(3)
            
            # Find the complete function body
            start_pos = match.end() - 1  # Position of opening brace
            brace_count = 1
            pos = start_pos + 1
            
            while pos < len(content) and brace_count > 0:
                if content[pos] == '{':
                    brace_count += 1
                elif content[pos] == '}':
                    brace_count -= 1
                pos += 1
            
            if brace_count == 0:
                function_body = content[start_pos:pos]
                if not should_keep_async(function_body):
                    # Remove async keyword
                    return f"{indent}{pub_prefix}{fn_signature} {{"
            
            # Keep original if we should keep async
            return match.group(0)
        
        # Apply the replacement
        new_content = re.sub(pattern, replace_async, content)
        
        if new_content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(new_content)
            print(f"✅ Fixed async functions in: {filepath}")
            return True
        
        return False
        
    except Exception as e:
        print(f"❌ Error processing {filepath}: {e}")
        return False

def main():
    """Main function to process all Rust files"""
    if len(sys.argv) > 1:
        target_dir = sys.argv[1]
    else:
        target_dir = "code/crates"
    
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
                
                if fix_unused_async_in_file(filepath):
                    files_changed += 1
    
    print(f"\n📊 Summary:")
    print(f"   Files processed: {files_processed}")
    print(f"   Files changed: {files_changed}")
    print(f"   Success rate: {files_changed}/{files_processed}")

if __name__ == "__main__":
    main() 