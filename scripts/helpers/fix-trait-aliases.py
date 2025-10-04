#!/usr/bin/env python3
"""
Fix trait type aliases with missing associated types
"""

import re
import sys

def fix_trait_aliases(filepath):
    """Fix trait type aliases by removing problematic ones"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Comment out problematic type aliases that need associated types
        problematic_aliases = [
            'pub type Service = dyn UnifiedCanonicalService;',
            'pub type Storage = dyn UnifiedCanonicalStorage;',
            'pub type Network = dyn UnifiedCanonicalNetwork;',
            'pub type Security = dyn UnifiedCanonicalSecurity;',
            'pub type Automation = dyn UnifiedCanonicalAutomation;',
            'pub type Zfs = dyn UnifiedCanonicalZfs;',
            'pub type Provider<T> = dyn UnifiedCanonicalProvider<T>;'
        ]
        
        for alias in problematic_aliases:
            content = content.replace(alias, f'// FIXME: {alias} // Needs associated type specification')
        
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"   ✅ Fixed trait aliases in {filepath}")
            return True
        
        return False
        
    except Exception as e:
        print(f"   ❌ Error processing {filepath}: {e}")
        return False

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: fix-trait-aliases.py <filepath>")
        sys.exit(1)
    
    fix_trait_aliases(sys.argv[1])
