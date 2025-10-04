#!/bin/bash
#
# **DUPLICATE SERVICE TRAIT REMOVAL SCRIPT**
#
# This script removes 100+ duplicate Service trait definitions and replaces
# them with re-exports of the canonical trait from traits_root::service
#
# Usage: ./remove-duplicate-service-traits.sh
#
# Safety: Makes backups before modifying files

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
CORE_SRC="$REPO_ROOT/code/crates/nestgate-core/src"
BACKUP_DIR="$REPO_ROOT/backups/trait-cleanup-$(date +%Y%m%d_%H%M%S)"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Statistics
TOTAL_FILES=0
MODIFIED_FILES=0
SKIPPED_FILES=0
ERROR_FILES=0

echo "======================================================================"
echo "  DUPLICATE SERVICE TRAIT REMOVAL"
echo "======================================================================"
echo ""
echo "Target: $CORE_SRC"
echo "Backup: $BACKUP_DIR"
echo ""

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Function to backup a file
backup_file() {
    local file="$1"
    local rel_path="${file#$CORE_SRC/}"
    local backup_path="$BACKUP_DIR/$rel_path"
    mkdir -p "$(dirname "$backup_path")"
    cp "$file" "$backup_path"
}

# Function to check if file has the duplicate trait
has_duplicate_trait() {
    local file="$1"
    # Look for the duplicate Service trait definition
    grep -q "^pub trait Service: Send + Sync {" "$file" 2>/dev/null || return 1
    # Make sure it's not the canonical file
    [[ "$file" != *"traits_root/service.rs" ]] || return 1
    return 0
}

# Function to remove duplicate Service trait and add re-export
remove_duplicate_trait() {
    local file="$1"
    
    echo -n "  Processing: $(basename "$file")... "
    
    # Backup first
    backup_file "$file"
    
    # Check if file already has the re-export
    if grep -q "pub use crate::traits_root::service::Service;" "$file" 2>/dev/null; then
        echo -e "${YELLOW}SKIP${NC} (already has re-export)"
        ((SKIPPED_FILES++))
        return 0
    fi
    
    # Create a temporary file
    local tmp_file=$(mktemp)
    
    # Strategy: Remove the duplicate trait definition and add a re-export comment
    awk '
        BEGIN { in_service_trait = 0; trait_removed = 0 }
        
        # Detect start of Service trait
        /^pub trait Service: Send \+ Sync \{/ {
            in_service_trait = 1
            if (!trait_removed) {
                print "// Service trait re-exported from canonical source"
                print "pub use crate::traits_root::service::Service;"
                print ""
                trait_removed = 1
            }
            next
        }
        
        # Inside trait, look for closing brace
        in_service_trait == 1 {
            if (/^}/) {
                in_service_trait = 0
            }
            next
        }
        
        # Print all other lines
        { print }
    ' "$file" > "$tmp_file"
    
    # Check if changes were made
    if diff -q "$file" "$tmp_file" > /dev/null 2>&1; then
        echo -e "${YELLOW}SKIP${NC} (no changes)"
        ((SKIPPED_FILES++))
        rm "$tmp_file"
        return 0
    fi
    
    # Apply changes
    mv "$tmp_file" "$file"
    echo -e "${GREEN}DONE${NC}"
    ((MODIFIED_FILES++))
}

# Find all Rust files with duplicate Service traits
echo "Scanning for duplicate Service traits..."
echo ""

while IFS= read -r -d '' file; do
    ((TOTAL_FILES++))
    
    if has_duplicate_trait "$file"; then
        if ! remove_duplicate_trait "$file"; then
            echo -e "${RED}ERROR${NC}"
            ((ERROR_FILES++))
        fi
    fi
done < <(find "$CORE_SRC" -type f -name "*.rs" -print0)

echo ""
echo "======================================================================"
echo "  SUMMARY"
echo "======================================================================"
echo ""
echo "Total files scanned:    $TOTAL_FILES"
echo -e "${GREEN}Files modified:         $MODIFIED_FILES${NC}"
echo -e "${YELLOW}Files skipped:          $SKIPPED_FILES${NC}"
echo -e "${RED}Files with errors:      $ERROR_FILES${NC}"
echo ""
echo "Backup location: $BACKUP_DIR"
echo ""

if [ $MODIFIED_FILES -gt 0 ]; then
    echo "======================================================================"
    echo "  NEXT STEPS"
    echo "======================================================================"
    echo ""
    echo "1. Verify compilation:"
    echo "   cargo check --package nestgate-core --lib"
    echo ""
    echo "2. If successful, commit changes:"
    echo "   git add -A"
    echo "   git commit -m 'refactor: Remove $MODIFIED_FILES duplicate Service trait definitions'"
    echo ""
    echo "3. If errors occur, restore from backup:"
    echo "   cp -r $BACKUP_DIR/* $CORE_SRC/"
    echo ""
fi

echo "======================================================================"
echo "  COMPLETE"
echo "======================================================================" 