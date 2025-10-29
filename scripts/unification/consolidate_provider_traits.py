#!/usr/bin/env python3
"""
Provider Trait Consolidation Script
Consolidates generic Provider trait duplicates to canonical sources.

IMPORTANT: Only consolidates GENERIC provider patterns, not domain-specific providers.
Preserves: SteamDataProvider, HealthCheckProvider, CacheProvider, etc.
"""

import re
import shutil
from pathlib import Path
from datetime import datetime
from typing import Tuple

# Canonical provider trait locations
CANONICAL_PROVIDER = "crate::traits::canonical_hierarchy::CanonicalProvider"
CANONICAL_UNIVERSAL_PROVIDER = "crate::traits::canonical_provider_unification::CanonicalUniversalProvider"

# Generic provider trait patterns to consolidate (NOT domain-specific ones)
PROVIDER_TRAIT_PATTERNS = [
    r"pub\s+trait\s+UnifiedProvider\s*[\s{:]",
    r"pub\s+trait\s+ZeroCostUniversalServiceProvider\s*:\s*Send",
    r"pub\s+trait\s+NativeAsyncUniversalProvider\s*<",
]

# Files to skip
SKIP_PATTERNS = [
    "templates/",
    "tests/",
    "benches/",
    "examples/",
    "target/",
    "traits/canonical_hierarchy.rs",  # Canonical source
    "traits/canonical_unified_traits.rs",  # Canonical source
    "traits/canonical_provider_unification.rs",  # Canonical source
]

# Domain-specific providers to PRESERVE (don't consolidate these)
PRESERVE_PATTERNS = [
    "SteamDataProvider",
    "HealthCheckProvider",
    "CacheProvider",
    "ConfigProvider",
    "FallbackProvider",
    "AuthenticationProvider",
    "EncryptionProvider",
    "SigningProvider",
    "ComputePrimalProvider",
    "OrchestrationPrimalProvider",
    "UniversalPrimalProvider",
]


def should_skip_file(file_path: Path) -> bool:
    """Check if file should be skipped."""
    path_str = str(file_path)
    return any(pattern in path_str for pattern in SKIP_PATTERNS)


def should_preserve_trait(content: str, trait_name: str) -> bool:
    """Check if this is a domain-specific trait that should be preserved."""
    return trait_name in PRESERVE_PATTERNS


def backup_file(file_path: Path, backup_dir: Path) -> None:
    """Create timestamped backup of file."""
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    relative_path = file_path.relative_to(file_path.parents[len(file_path.parents) - 1])
    backup_path = backup_dir / f"{relative_path.name}.{timestamp}.backup"
    backup_path.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(file_path, backup_path)
    print(f"   📦 Backed up: {backup_path.name}")


def find_provider_trait_definition(content: str) -> Tuple[bool, str, int, int]:
    """
    Find generic Provider trait definition in content.
    Returns: (found, trait_name, start_pos, end_pos)
    """
    for pattern in PROVIDER_TRAIT_PATTERNS:
        match = re.search(pattern, content)
        if match:
            # Extract trait name
            trait_match = re.search(r"trait\s+(\w+)", match.group(0))
            if not trait_match:
                continue
                
            trait_name = trait_match.group(1)
            
            # Check if this is a domain-specific trait to preserve
            if should_preserve_trait(content, trait_name):
                continue
            
            start_pos = match.start()
            
            # Find the end of the trait (closing brace at the same level)
            brace_count = 0
            in_trait = False
            end_pos = start_pos
            
            for i in range(start_pos, len(content)):
                char = content[i]
                if char == '{':
                    brace_count += 1
                    in_trait = True
                elif char == '}':
                    brace_count -= 1
                    if in_trait and brace_count == 0:
                        end_pos = i + 1
                        break
            
            if end_pos > start_pos:
                return True, trait_name, start_pos, end_pos
    
    return False, "", 0, 0


def determine_canonical_provider(trait_name: str, content: str) -> str:
    """Determine which canonical provider to use."""
    # If it's a universal/service provider pattern, use CanonicalUniversalProvider
    if "Universal" in trait_name or "Service" in trait_name:
        return CANONICAL_UNIVERSAL_PROVIDER
    
    # Default to CanonicalProvider
    return CANONICAL_PROVIDER


def consolidate_provider_trait(file_path: Path, backup_dir: Path, dry_run: bool = False) -> bool:
    """
    Consolidate generic Provider trait in a single file.
    Returns True if consolidation was performed.
    """
    try:
        content = file_path.read_text(encoding='utf-8')
        
        # Check if file has a generic Provider trait definition to consolidate
        found, trait_name, start_pos, end_pos = find_provider_trait_definition(content)
        
        if not found:
            return False
        
        # Check if already consolidated
        if "pub use crate::traits::" in content and "Provider" in content:
            # Check if this specific trait is already consolidated
            if f"as {trait_name}" in content:
                print(f"   ✅ Already consolidated: {file_path.name}")
                return False
        
        print(f"\n   🔄 Consolidating: {file_path.name}")
        print(f"      Trait: {trait_name}")
        
        if dry_run:
            print(f"      [DRY RUN] Would replace with re-export")
            return True
        
        # Backup original file
        backup_file(file_path, backup_dir)
        
        # Determine canonical provider
        canonical_import = determine_canonical_provider(trait_name, content)
        canonical_name = canonical_import.split("::")[-1]
        
        # Create replacement comment and re-export
        replacement = f'''/// Provider trait re-exported from canonical source
/// 
/// **CONSOLIDATED**: This trait definition was replaced with a re-export to eliminate duplication.
/// See: `{canonical_import}` for the unified implementation.
/// 
/// **Migration**: Update implementations to use `{canonical_name}` directly.
/// ```rust
/// use nestgate_core::traits::{{{canonical_name}}};
/// 
/// impl {canonical_name}<MyService> for MyProvider {{
///     // ... implementation
/// }}
/// ```
pub use {canonical_import} as {trait_name};
'''
        
        # Replace trait definition with re-export
        new_content = content[:start_pos] + replacement + content[end_pos:]
        
        # Write back
        file_path.write_text(new_content, encoding='utf-8')
        
        print(f"      ✅ Consolidated to: {canonical_name}")
        return True
        
    except Exception as e:
        print(f"   ❌ Error processing {file_path.name}: {e}")
        return False


def main():
    """Main consolidation process."""
    print("=" * 70)
    print("🔧 PROVIDER TRAIT CONSOLIDATION")
    print("=" * 70)
    print(f"Canonical sources:")
    print(f"  - {CANONICAL_PROVIDER}")
    print(f"  - {CANONICAL_UNIVERSAL_PROVIDER}")
    print("\n⚠️  PRESERVING domain-specific providers:")
    for pattern in PRESERVE_PATTERNS:
        print(f"  - {pattern}")
    print("=" * 70)
    
    # Setup paths
    script_dir = Path(__file__).parent
    project_root = script_dir.parent.parent
    code_dir = project_root / "code" / "crates"
    backup_dir = project_root / "backups" / f"provider_traits_{datetime.now().strftime('%Y%m%d_%H%M%S')}"
    
    backup_dir.mkdir(parents=True, exist_ok=True)
    print(f"\n📦 Backups: {backup_dir}")
    
    # Find all Rust files
    rust_files = list(code_dir.glob("**/*.rs"))
    print(f"\n🔍 Scanning {len(rust_files)} Rust files...")
    
    # Process files
    consolidated_files = []
    
    for file_path in rust_files:
        if should_skip_file(file_path):
            continue
        
        if consolidate_provider_trait(file_path, backup_dir, dry_run=False):
            consolidated_files.append(file_path)
    
    # Summary
    print("\n" + "=" * 70)
    print("📊 CONSOLIDATION SUMMARY")
    print("=" * 70)
    print(f"Files scanned:      {len(rust_files)}")
    print(f"Files consolidated: {len(consolidated_files)}")
    print(f"Success rate:       {(len(consolidated_files) / len(rust_files) * 100) if rust_files else 0:.1f}%")
    print(f"Backups created:    {len(consolidated_files)}")
    
    if consolidated_files:
        print("\n✅ CONSOLIDATED FILES:")
        for file_path in consolidated_files:
            print(f"   - {file_path.relative_to(project_root)}")
    else:
        print("\n✅ No generic provider duplicates found - all appear to be consolidated or domain-specific!")
    
    print("\n" + "=" * 70)
    print("✅ PROVIDER TRAIT CONSOLIDATION COMPLETE!")
    print("=" * 70)
    print("\n📋 Next steps:")
    print("   1. Run: cargo check --all-targets")
    print("   2. Fix any compilation errors")
    print("   3. Update implementations to use canonical providers")
    print("   4. Celebrate 100% trait unification! 🎉")


if __name__ == "__main__":
    main() 