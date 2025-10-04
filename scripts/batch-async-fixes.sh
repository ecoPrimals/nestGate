#!/bin/bash
# 🔧 **BATCH ASYNC FIXES**
# Apply async move wrapping to files with simple sync patterns

set -euo pipefail

echo "🔧 **BATCH ASYNC FIXES**"
echo "======================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Colors for output
BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# List of files with similar patterns to fix
FILES_TO_FIX=(
    "code/crates/nestgate-core/src/perf_monitor.rs"
    "code/crates/nestgate-core/src/caching.rs"
    "code/crates/nestgate-core/src/canonical_types/network.rs"
)

echo ""
echo -e "${BLUE}Processing files with simple async patterns...${NC}"

for file in "${FILES_TO_FIX[@]}"; do
    if [[ -f "$file" ]]; then
        echo -e "${BLUE}   📝 Processing: $file${NC}"
        
        # Backup the file
        cp "$file" "$file.backup-$(date +%Y%m%d-%H%M%S)"
        
        # Apply the async move wrapping pattern
        # Pattern 1: initialize function
        sed -i '/fn initialize(&self, config: &Config) -> impl std::future::Future<Output = Result<()>> + Send {/,/^    }$/ {
            /^        \/\/ [^}]*$/ {
                N
                /Ok(())/ {
                    i\        async move {
                    s/Ok(())/            Ok(())
                    a\        }
                }
            }
            /^        Ok(())$/ {
                i\        async move {
                s/.*/            &/
                a\        }
            }
        }' "$file"
        
        # Pattern 2: health_check function
        sed -i '/fn health_check(&self) -> impl std::future::Future<Output = Result<HealthStatus>> + Send {/,/^    }$/ {
            /^        \/\/ [^}]*$/ {
                N
                /Ok(HealthStatus::Healthy)/ {
                    i\        async move {
                    s/Ok(HealthStatus::Healthy)/            Ok(HealthStatus::Healthy)
                    a\        }
                }
            }
            /^        Ok(HealthStatus::Healthy)$/ {
                i\        async move {
                s/.*/            &/
                a\        }
            }
        }' "$file"
        
        # Pattern 3: shutdown function
        sed -i '/fn shutdown(&self) -> impl std::future::Future<Output = Result<()>> + Send {/,/^    }$/ {
            /^        \/\/ [^}]*$/ {
                N
                N
                /Ok(())/ {
                    i\        async move {
                    s/Ok(())/            Ok(())
                    a\        }
                }
            }
            /^        Ok(())$/ {
                i\        async move {
                s/.*/            &/
                a\        }
            }
        }' "$file"
        
        echo -e "${GREEN}     ✅ Fixed: $file${NC}"
    else
        echo -e "${BLUE}     ⚠️  File not found: $file${NC}"
    fi
done

echo ""
echo -e "${BLUE}Testing compilation progress...${NC}"

ERROR_COUNT=$(cargo check --package nestgate-core --message-format short 2>&1 | grep -c "error\|warning" || echo "0")
echo -e "${BLUE}   Current errors/warnings: $ERROR_COUNT${NC}"

echo ""
echo -e "${GREEN}✅ **BATCH ASYNC FIXES COMPLETED**${NC}"

echo ""
echo -e "${GREEN}🚀 Batch async fixes completed!${NC}" 