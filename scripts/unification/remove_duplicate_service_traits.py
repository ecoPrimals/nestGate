#!/usr/bin/env python3
"""
DUPLICATE SERVICE TRAIT REMOVAL SCRIPT

Removes 100+ duplicate Service trait definitions and replaces them
with re-exports of the canonical trait from traits_root::service

Usage: python3 remove_duplicate_service_traits.py
"""

import os
import re
import shutil
from pathlib import Path
from datetime import datetime

# Configuration
REPO_ROOT = Path(__file__).parent.parent.parent
CORE_SRC = REPO_ROOT / "code" / "crates" / "nestgate-core" / "src"
BACKUP_DIR = REPO_ROOT / "backups" / f"trait-cleanup-{datetime.now().strftime('%Y%m%d_%H%M%S')}"
CANONICAL_FILE = "traits_root/service.rs"

# Statistics
stats = {
    "total_files": 0,
    "modified_files": 0,
    "skipped_files": 0,
    "error_files": 0,
}

# Pattern to match the duplicate Service trait
SERVICE_TRAIT_PATTERN = re.compile(
    r"(?:///.*\n)*"  # Optional doc comments
    r"pub trait Service: Send \+ Sync \{\s*\n"
    r"(?:.*\n)*?"  # Trait body (non-greedy)
    r"^\}",
    re.MULTILINE
)

# Replacement text
REPLACEMENT = """/// Service interface re-exported from canonical source
/// See: `crate::traits_root::service::Service` for the unified implementation
pub use crate::traits_root::service::Service;"""


def backup_file(file_path: Path):
    """Create a backup of the file"""
    rel_path = file_path.relative_to(CORE_SRC)
    backup_path = BACKUP_DIR / rel_path
    backup_path.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(file_path, backup_path)


def has_duplicate_trait(content: str, file_path: Path) -> bool:
    """Check if file has duplicate Service trait"""
    # Skip the canonical file
    if CANONICAL_FILE in str(file_path):
        return False
    
    # Check for the pattern
    return bool(SERVICE_TRAIT_PATTERN.search(content))


def remove_duplicate_trait(file_path: Path) -> bool:
    """Remove duplicate Service trait and add re-export"""
    try:
        # Read file
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Check if already has re-export
        if "pub use crate::traits_root::service::Service" in content:
            print(f"  ⚠️  SKIP: {file_path.name} (already has re-export)")
            stats["skipped_files"] += 1
            return True
        
        # Check for duplicate trait
        if not has_duplicate_trait(content, file_path):
            return True
        
        # Backup file
        backup_file(file_path)
        
        # Replace duplicate trait with re-export
        new_content = SERVICE_TRAIT_PATTERN.sub(REPLACEMENT, content)
        
        # Check if changes were made
        if content == new_content:
            print(f"  ⚠️  SKIP: {file_path.name} (no changes)")
            stats["skipped_files"] += 1
            return True
        
        # Write changes
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        
        print(f"  ✅ DONE: {file_path.name}")
        stats["modified_files"] += 1
        return True
        
    except Exception as e:
        print(f"  ❌ ERROR: {file_path.name} - {e}")
        stats["error_files"] += 1
        return False


def main():
    print("=" * 70)
    print("  DUPLICATE SERVICE TRAIT REMOVAL")
    print("=" * 70)
    print()
    print(f"Target: {CORE_SRC}")
    print(f"Backup: {BACKUP_DIR}")
    print()
    
    # Create backup directory
    BACKUP_DIR.mkdir(parents=True, exist_ok=True)
    
    print("Scanning for duplicate Service traits...")
    print()
    
    # Find all Rust files
    rust_files = list(CORE_SRC.rglob("*.rs"))
    stats["total_files"] = len(rust_files)
    
    # Process each file
    for file_path in rust_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            if has_duplicate_trait(content, file_path):
                remove_duplicate_trait(file_path)
                
        except Exception as e:
            print(f"  ❌ ERROR: {file_path.name} - {e}")
            stats["error_files"] += 1
    
    # Print summary
    print()
    print("=" * 70)
    print("  SUMMARY")
    print("=" * 70)
    print()
    print(f"Total files scanned:    {stats['total_files']}")
    print(f"✅ Files modified:         {stats['modified_files']}")
    print(f"⚠️  Files skipped:          {stats['skipped_files']}")
    print(f"❌ Files with errors:      {stats['error_files']}")
    print()
    print(f"Backup location: {BACKUP_DIR}")
    print()
    
    if stats["modified_files"] > 0:
        print("=" * 70)
        print("  NEXT STEPS")
        print("=" * 70)
        print()
        print("1. Verify compilation:")
        print("   cargo check --package nestgate-core --lib")
        print()
        print("2. If successful, commit changes:")
        print("   git add -A")
        print(f"   git commit -m 'refactor: Remove {stats['modified_files']} duplicate Service trait definitions'")
        print()
        print("3. If errors occur, restore from backup:")
        print(f"   cp -r {BACKUP_DIR}/* {CORE_SRC}/")
        print()
    
    print("=" * 70)
    print("  COMPLETE")
    print("=" * 70)


if __name__ == "__main__":
    main() 