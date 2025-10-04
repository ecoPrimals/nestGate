#!/usr/bin/env python3
"""
Unwrap Elimination Helper
Systematically replaces .unwrap() and .expect() with proper error handling
"""

import re
import sys
import os

def process_file(filepath):
    """Process a single file to eliminate unwrap usage"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Replace common unwrap patterns with proper error handling
        patterns = [
            # .unwrap() in return statements
            (r'(\w+)\.unwrap\(\)', r'\1.map_err(|e| NestGateError::internal(format!("Operation failed: {}", e)))?'),
            
            # .expect() with messages
            (r'(\w+)\.expect\("([^"]+)"\)', r'\1.map_err(|_| NestGateError::internal("\2"))?'),
            
            # Simple unwrap in assignments
            (r'let (\w+) = ([^;]+)\.unwrap\(\);', r'let \1 = \2.map_err(|e| NestGateError::internal(format!("Failed to get {}: {}", "\1", e)))?;'),
        ]
        
        for pattern, replacement in patterns:
            content = re.sub(pattern, replacement, content)
        
        # Only write if changed
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"   ✅ Eliminated unwrap usage in {filepath}")
            return True
        
        return False
        
    except Exception as e:
        print(f"   ❌ Error processing {filepath}: {e}")
        return False

def main():
    if len(sys.argv) != 2:
        print("Usage: eliminate-unwrap.py <directory>")
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
