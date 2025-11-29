#!/bin/bash
# Fix all doc list item indentation issues

set -e

cd "$(dirname "$0")"

echo "🔧 Fixing doc list item indentation issues..."

# Fix ZfsDomainConfig
sed -i '66s|/// Configuration for ZfsDomain|///\n  /// Configuration for ZFS domain operations and storage management.|' \
    code/crates/nestgate-core/src/config/canonical_primary/domains/consolidated_domains.rs

# Fix ApiDomainConfig
sed -i '167s|/// Configuration for ApiDomain|///\n  /// Configuration for API domain and REST endpoints.|' \
    code/crates/nestgate-core/src/config/canonical_primary/domains/consolidated_domains.rs

# Fix McpDomainConfig
sed -i '245s|/// Configuration for McpDomain|///\n  /// Configuration for MCP (Model Context Protocol) domain.|' \
    code/crates/nestgate-core/src/config/canonical_primary/domains/consolidated_domains.rs

# Fix AutomationConfig
sed -i '22s|/// Configuration for Automation|///\n  /// Configuration for automation domain and policies.|' \
    code/crates/nestgate-core/src/config/canonical_primary/domains/automation/mod.rs

# Fix ApiConfig
sed -i '22s|/// Configuration for Api|///\n  /// Configuration for API server and REST endpoints.|' \
    code/crates/nestgate-core/src/config/canonical_primary/domains/network/api.rs || true

# Fix FederationConfig
find code/crates -name "*.rs" -exec sed -i 's|^/// Configuration for Federation$|///\n  /// Configuration for federation and distributed operations.|g' {} \;

# Fix CanonicalStorage
find code/crates -name "*.rs" -exec sed -i 's|^/// Configuration for CanonicalStorage$|///\n  /// Configuration for canonical storage operations.|g' {} \;

echo "✅ Fixed doc list item indentation issues"

