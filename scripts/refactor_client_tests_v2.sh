#!/usr/bin/env bash
# Smart refactoring script for client_tests.rs - v2
# Only replaces patterns that already have .expect()

FILE="code/crates/nestgate-core/src/network/client_tests.rs"

# Create a backup
cp "$FILE" "$FILE.backup"

# Replace Port::new(...).expect(...) with test helpers
sed -i 's/Port::new(crate::config::port_config::api_port())\.expect("Network operation failed")/test_api_port()/g' "$FILE"
sed -i 's/Port::new(crate::config::port_config::grafana_port())\.expect("Network operation failed")/test_grafana_port()/g' "$FILE"
sed -i 's/Port::new(crate::config::port_config::admin_port())\.expect("Network operation failed")/test_admin_port()/g' "$FILE"

# Count reductions
BEFORE=$(wc -l < "$FILE.backup")
AFTER=$(wc -l < "$FILE")
SAVED=$((BEFORE - AFTER))

echo "Refactoring complete. Backup saved to $FILE.backup"
echo "Lines before: $BEFORE"
echo "Lines after: $AFTER"
echo "Lines saved: $SAVED"
echo ""
echo "Testing..."
cargo test --package nestgate-core --lib network::client_tests --quiet

