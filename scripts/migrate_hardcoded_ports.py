#!/usr/bin/env python3
"""
Automated hardcoded port migration tool.
Replaces hardcoded port literals with port_config calls.
"""

import re
import sys
from pathlib import Path
from typing import Dict, List, Tuple

# Port mapping: literal -> config function
PORT_MIGRATIONS = {
    "8080": "port_config::api_port()",
    "8081": "port_config::admin_port()",
    "8082": "port_config::health_port()",
    "8083": "port_config::websocket_port()",
    "9090": "port_config::metrics_port()",
    "9091": "port_config::prometheus_port()",
    "3000": "port_config::grafana_port()",
    "5432": "port_config::postgres_port()",
    "6379": "port_config::redis_port()",
    "50051": "port_config::grpc_port()",
}

# Host migrations
HOST_MIGRATIONS = {
    '"127.0.0.1"': 'config::default_host()',
    '"localhost"': 'config::default_host()',
}

def should_skip_file(filepath: Path) -> bool:
    """Check if file should be skipped."""
    skip_patterns = [
        'target/',
        'port_config.rs',  # Don't modify the config file itself
        'hardcoding.rs',   # Don't modify the constants file
        '_tests.rs',       # Skip most test files for now
    ]
    path_str = str(filepath)
    return any(pattern in path_str for pattern in skip_patterns)

def find_port_literals(content: str) -> List[Tuple[str, int]]:
    """Find hardcoded port literals in content."""
    found = []
    for port, replacement in PORT_MIGRATIONS.items():
        # Match port as a standalone number (not part of larger number)
        pattern = r'\b' + port + r'\b'
        for match in re.finditer(pattern, content):
            found.append((port, match.start()))
    return found

def migrate_file(filepath: Path, dry_run: bool = True) -> Dict[str, int]:
    """Migrate a single file."""
    stats = {'ports': 0, 'hosts': 0, 'imports': 0}
    
    try:
        content = filepath.read_text()
        original = content
        
        # Check if file already imports port_config
        has_port_config_import = 'port_config' in content
        
        # Migrate port literals
        for port, replacement in PORT_MIGRATIONS.items():
            pattern = r'\b' + port + r'\b'
            matches = len(re.findall(pattern, content))
            if matches > 0:
                content = re.sub(pattern, replacement, content)
                stats['ports'] += matches
        
        # Migrate host literals (more conservative)
        for host, replacement in HOST_MIGRATIONS.items():
            if host in content:
                # Only replace in obvious contexts (like "host = ")
                pattern = r'(host\s*[=:]\s*)' + re.escape(host)
                matches = len(re.findall(pattern, content))
                if matches > 0:
                    content = re.sub(pattern, r'\1' + replacement, content)
                    stats['hosts'] += matches
        
        # Add import if needed and changes were made
        if (stats['ports'] > 0 or stats['hosts'] > 0) and not has_port_config_import:
            # Add import after existing use statements
            use_pattern = r'(use\s+[^;]+;)\n'
            matches = list(re.finditer(use_pattern, content))
            if matches:
                last_use = matches[-1]
                insert_pos = last_use.end()
                import_line = 'use crate::config::port_config;\n'
                content = content[:insert_pos] + import_line + content[insert_pos:]
                stats['imports'] = 1
        
        # Write if changes were made and not dry run
        if content != original:
            if not dry_run:
                filepath.write_text(content)
            return stats
        
    except Exception as e:
        print(f"Error processing {filepath}: {e}", file=sys.stderr)
    
    return {'ports': 0, 'hosts': 0, 'imports': 0}

def main():
    dry_run = '--apply' not in sys.argv
    code_dir = Path('code/crates')
    
    if not code_dir.exists():
        print("Error: Run from project root", file=sys.stderr)
        return 1
    
    print(f"🔍 Scanning for hardcoded values...")
    print(f"Mode: {'DRY RUN' if dry_run else 'APPLY CHANGES'}")
    print()
    
    total_files = 0
    total_ports = 0
    total_hosts = 0
    total_imports = 0
    modified_files = []
    
    for rs_file in code_dir.rglob('*.rs'):
        if should_skip_file(rs_file):
            continue
        
        stats = migrate_file(rs_file, dry_run)
        if stats['ports'] > 0 or stats['hosts'] > 0:
            total_files += 1
            total_ports += stats['ports']
            total_hosts += stats['hosts']
            total_imports += stats['imports']
            modified_files.append((rs_file, stats))
    
    # Print summary
    print(f"📊 Migration Summary:")
    print(f"  Files processed: {total_files}")
    print(f"  Port migrations: {total_ports}")
    print(f"  Host migrations: {total_hosts}")
    print(f"  Imports added: {total_imports}")
    print()
    
    if modified_files and dry_run:
        print("📝 Files that would be modified:")
        for filepath, stats in modified_files[:20]:  # Show first 20
            rel_path = filepath.relative_to(code_dir)
            print(f"  {rel_path}: {stats['ports']} ports, {stats['hosts']} hosts")
        if len(modified_files) > 20:
            print(f"  ... and {len(modified_files) - 20} more")
        print()
        print("✅ Dry run complete. Run with --apply to make changes.")
    elif modified_files:
        print("✅ Migration complete!")
    else:
        print("✅ No changes needed.")
    
    return 0

if __name__ == '__main__':
    sys.exit(main())

