#!/usr/bin/env python3
"""
Async Trait Migration Helper
Converts async_trait usage to native async patterns
"""

import re
import sys
import os

def migrate_trait_definition(content):
    """Convert async_trait trait definitions to native async"""
    # Remove #[async_trait] from trait definitions
    content = re.sub(r'#\[async_trait\]\s*\n\s*pub trait', 'pub trait', content)
    content = re.sub(r'#\[async_trait\]\s*\n\s*trait', 'trait', content)
    
    # Convert async fn to native async in trait definitions
    def convert_async_fn(match):
        indent = match.group(1)
        fn_signature = match.group(2)
        return_type = match.group(3)
        
        # Convert to impl Future pattern
        return f"{indent}fn {fn_signature} -> impl Future<Output = {return_type}> + Send;"
    
    content = re.sub(
        r'(\s+)async fn ([^{]+?) -> ([^{;]+);',
        convert_async_fn,
        content
    )
    
    return content

def migrate_trait_impl(content):
    """Convert async_trait implementations to native async"""
    # Remove #[async_trait] from implementations
    content = re.sub(r'#\[async_trait\]\s*\n\s*impl', 'impl', content)
    
    # Convert async fn implementations
    def convert_impl_fn(match):
        indent = match.group(1)
        fn_signature = match.group(2)
        return_type = match.group(3)
        body = match.group(4)
        
        # Convert to native async implementation
        return f"""{indent}fn {fn_signature} -> impl Future<Output = {return_type}> + Send {{
{indent}    async move {{
{body}
{indent}    }}
{indent}}}"""
    
    content = re.sub(
        r'(\s+)async fn ([^{]+?) -> ([^{]+?) \{([^}]+)\}',
        convert_impl_fn,
        content,
        flags=re.DOTALL
    )
    
    return content

def process_file(filepath):
    """Process a single file for async trait migration"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Skip if no async_trait usage
        if '#[async_trait]' not in content:
            return False
        
        # Apply migrations
        content = migrate_trait_definition(content)
        content = migrate_trait_impl(content)
        
        # Remove async_trait imports
        content = re.sub(r'use async_trait::async_trait;\s*\n', '', content)
        content = re.sub(r'use async_trait::\{async_trait\};\s*\n', '', content)
        
        # Only write if changed
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"   ✅ Migrated async_trait in {filepath}")
            return True
        
        return False
        
    except Exception as e:
        print(f"   ❌ Error processing {filepath}: {e}")
        return False

def main():
    if len(sys.argv) != 2:
        print("Usage: migrate-async-traits.py <directory>")
        sys.exit(1)
    
    directory = sys.argv[1]
    processed = 0
    
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith('.rs'):
                filepath = os.path.join(root, file)
                if process_file(filepath):
                    processed += 1
    
    print(f"Migrated {processed} files")

if __name__ == "__main__":
    main()
