#!/usr/bin/env python3
"""
Hardcoding Elimination Helper
Replaces hardcoded values with environment-aware configuration
"""

import re
import sys
import os

def process_file(filepath):
    """Process a single file to eliminate hardcoded values"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Replace hardcoded values with configuration calls
        replacements = [
            # Hardcoded ports
            (r'8080', r'crate::constants::network::api_port()'),
            (r'3000', r'crate::constants::network::DEFAULT_WEB_PORT'),
            (r'5432', r'crate::constants::storage::DEFAULT_DB_PORT'),
            (r'6379', r'crate::constants::storage::DEFAULT_REDIS_PORT'),
            
            # Hardcoded IPs and hostnames
            (r'"127\.0\.0\.1"', r'crate::constants::network::LOCALHOST'),
            (r'"localhost"', r'&format!("localhost")'),
            
            # Hardcoded timeouts
            (r'30000', r'crate::constants::network::connection_timeout().as_millis() as u64'),
            (r'5000', r'crate::constants::network::DEFAULT_TIMEOUT_MS'),
            
            # Hardcoded buffer sizes
            (r'65536', r'crate::constants::system::DEFAULT_BUFFER_SIZE'),
            (r'8192', r'crate::constants::system::DEFAULT_SMALL_BUFFER_SIZE'),
            (r'1024', r'crate::constants::system::DEFAULT_TINY_BUFFER_SIZE'),
        ]
        
        for pattern, replacement in replacements:
            # Only replace if it looks like a standalone value (not part of a larger number)
            content = re.sub(f'\\b{pattern}\\b', replacement, content)
        
        # Only write if changed
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"   ✅ Eliminated hardcoding in {filepath}")
            return True
        
        return False
        
    except Exception as e:
        print(f"   ❌ Error processing {filepath}: {e}")
        return False

def main():
    if len(sys.argv) != 2:
        print("Usage: eliminate-hardcoding.py <directory>")
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
