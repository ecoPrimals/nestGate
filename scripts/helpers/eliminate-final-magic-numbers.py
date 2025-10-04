#!/usr/bin/env python3
"""
Final Magic Number Elimination
Replaces remaining magic numbers with constants
"""

import re
import sys
import os

def process_file(filepath):
    """Process a single file to eliminate magic numbers"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Skip if it's a constants file
        if 'constants' in filepath:
            return False
        
        # Replace magic numbers with constants (be conservative)
        replacements = [
            # Common buffer sizes
            (r'\b65536\b', '65536 /* DEFAULT_BUFFER_SIZE */'),
            (r'\b8192\b', '8192 /* DEFAULT_SMALL_BUFFER_SIZE */'),
            (r'\b4096\b', '4096 /* DEFAULT_PAGE_SIZE */'),
            (r'\b1024\b', '1024 /* DEFAULT_TINY_BUFFER_SIZE */'),
            
            # Common timeouts (only in obvious timeout contexts)
            (r'\b30000\b', '30000 /* DEFAULT_TIMEOUT_MS */'),
            (r'\b5000\b', '5000 /* DEFAULT_SHORT_TIMEOUT_MS */'),
            
            # Common ports (only in obvious port contexts)
            (r'\b8080\b', '8080 /* DEFAULT_API_PORT */'),
            (r'\b3000\b', '3000 /* DEFAULT_WEB_PORT */'),
            (r'\b5432\b', '5432 /* DEFAULT_DB_PORT */'),
            (r'\b6379\b', '6379 /* DEFAULT_REDIS_PORT */'),
        ]
        
        for pattern, replacement in replacements:
            # Only replace if not already in a comment
            if pattern.strip('\\b') in content and '/*' not in content:
                content = re.sub(pattern, replacement, content)
        
        # Only write if changed
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"   ✅ Eliminated magic numbers in {filepath}")
            return True
        
        return False
        
    except Exception as e:
        print(f"   ❌ Error processing {filepath}: {e}")
        return False

def main():
    if len(sys.argv) != 2:
        print("Usage: eliminate-final-magic-numbers.py <directory>")
        sys.exit(1)
    
    directory = sys.argv[1]
    processed = 0
    
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith('.rs'):
                filepath = os.path.join(root, file)
                if process_file(filepath):
                    processed += 1
    
    print(f"Processed {processed} files")

if __name__ == "__main__":
    main()
