#!/usr/bin/env bash
#
# **WORKSPACE METADATA UNIFICATION SCRIPT**
# Automatically migrates crate Cargo.toml files to use workspace inheritance
#
# Usage: ./unify_workspace_metadata.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🚀 Starting workspace metadata unification..."
echo ""

# Colors for output
GREEN='\033[0.32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Counter for progress
TOTAL_CRATES=13
CURRENT=0

# List of crates to migrate (excluding nestgate-canonical which is already done)
CRATES=(
    "nestgate-core"
    "nestgate-network"
    "nestgate-middleware"
    "nestgate-api"
    "nestgate-zfs"
    "nestgate-performance"
    "nestgate-automation"
    "nestgate-nas"
    "nestgate-mcp"
    "nestgate-installer"
    "nestgate-fsmonitor"
    "nestgate-bin"
)

migrate_crate() {
    local crate_name=$1
    local crate_path="$PROJECT_ROOT/code/crates/$crate_name"
    local cargo_toml="$crate_path/Cargo.toml"
    
    ((CURRENT++))
    echo -e "${YELLOW}[$CURRENT/$TOTAL_CRATES]${NC} Migrating ${GREEN}$crate_name${NC}..."
    
    if [ ! -f "$cargo_toml" ]; then
        echo -e "  ${RED}✗${NC} Cargo.toml not found at $cargo_toml"
        return 1
    fi
    
    # Backup original
    cp "$cargo_toml" "$cargo_toml.backup"
    
    # Use Rust to do the migration (more reliable than sed)
    python3 << 'PYTHON_SCRIPT' "$cargo_toml"
import sys
import re

cargo_toml_path = sys.argv[1]

with open(cargo_toml_path, 'r') as f:
    content = f.read()

# Replace metadata fields with workspace inheritance
replacements = [
    # Version
    (r'^version\s*=\s*"[^"]*"', 'version.workspace = true'),
    # Edition
    (r'^edition\s*=\s*"[^"]*"', 'edition.workspace = true'),
    # License
    (r'^license\s*=\s*"[^"]*"', 'license.workspace = true'),
    # Authors
    (r'^authors\s*=\s*\[.*?\]', 'authors.workspace = true'),
    # Repository
    (r'^repository\s*=\s*"[^"]*"', 'repository.workspace = true'),
    # Homepage (but not if it's crate-specific like docs.rs)
    (r'^homepage\s*=\s*"https://nestgate\.io"', 'homepage.workspace = true'),
]

for pattern, replacement in replacements:
    content = re.sub(pattern, replacement, content, flags=re.MULTILINE)

# Replace common dependency versions with workspace = true
common_deps = [
    'tokio', 'serde', 'serde_json', 'serde_yaml', 'anyhow', 'thiserror',
    'axum', 'tracing', 'tracing-subscriber', 'uuid', 'chrono', 'config',
    'toml', 'futures', 'dirs', 'num_cpus', 'parking_lot', 'async-trait',
    'reqwest', 'tokio-test', 'tempfile', 'criterion', 'regex', 'rand',
    'mockall', 'axum-test', 'rstest'
]

for dep in common_deps:
    # Pattern: dep = "version" or dep = { version = "..." }
    # Replace with: dep = { workspace = true }
    
    # Simple version
    pattern1 = rf'^{dep}\s*=\s*"[^"]*"'
    if re.search(pattern1, content, re.MULTILINE):
        content = re.sub(pattern1, f'{dep} = {{ workspace = true }}', content, flags=re.MULTILINE)
    
    # Complex version with features
    pattern2 = rf'^{dep}\s*=\s*\{{\s*version\s*=\s*"[^"]*"(.*?)\}}'
    def replace_version(match):
        rest = match.group(1).strip()
        if rest and rest != ',':
            # Has features or other config
            rest_clean = rest.lstrip(',').strip()
            if rest_clean:
                return f'{dep} = {{ workspace = true, {rest_clean}}}'
        return f'{dep} = {{ workspace = true }}'
    
    content = re.sub(pattern2, replace_version, content, flags=re.MULTILINE)

with open(cargo_toml_path, 'w') as f:
    f.write(content)

print(f"✓ Migrated {cargo_toml_path}")
PYTHON_SCRIPT
    
    # Verify it builds
    if cargo build -p "$crate_name" --quiet 2>/dev/null; then
        echo -e "  ${GREEN}✓${NC} Build successful"
        # Remove backup
        rm "$cargo_toml.backup"
        return 0
    else
        echo -e "  ${RED}✗${NC} Build failed, restoring backup..."
        mv "$cargo_toml.backup" "$cargo_toml"
        return 1
    fi
}

# Migrate each crate
SUCCESS_COUNT=0
FAILED_CRATES=()

for crate in "${CRATES[@]}"; do
    if migrate_crate "$crate"; then
        ((SUCCESS_COUNT++))
    else
        FAILED_CRATES+=("$crate")
    fi
done

echo ""
echo "═══════════════════════════════════════════════════════"
echo "🎉 Migration Complete!"
echo "═══════════════════════════════════════════════════════"
echo ""
echo "✅ Successfully migrated: $SUCCESS_COUNT/$TOTAL_CRATES crates"

if [ ${#FAILED_CRATES[@]} -gt 0 ]; then
    echo "❌ Failed crates:"
    for crate in "${FAILED_CRATES[@]}"; do
        echo "   - $crate"
    done
    exit 1
fi

echo ""
echo "Running final workspace build verification..."
if cargo build --workspace --quiet; then
    echo -e "${GREEN}✓ Workspace builds successfully!${NC}"
else
    echo -e "${RED}✗ Workspace build failed${NC}"
    exit 1
fi

echo ""
echo "🎯 Next steps:"
echo "  1. Review changes: git diff"
echo "  2. Run tests: cargo test --workspace --lib"
echo "  3. Commit: git add -A && git commit -m 'Unify workspace metadata'"

