#!/usr/bin/env bash
# Smart refactoring script for client_tests.rs
# Replaces common patterns with helper functions

FILE="code/crates/nestgate-core/src/network/client_tests.rs"

# Create a backup
cp "$FILE" "$FILE.backup"

# Replace Port::new with test helper functions
sed -i 's/Port::new(crate::config::port_config::api_port())\.expect("Network operation failed")/test_api_port()/g' "$FILE"
sed -i 's/Port::new(crate::config::port_config::grafana_port())\.expect("Network operation failed")/test_grafana_port()/g' "$FILE"
sed -i 's/Port::new(crate::config::port_config::admin_port())\.expect("Network operation failed")/test_admin_port()/g' "$FILE"

# Replace standalone Port::new calls (without expect)
sed -i 's/Port::new(crate::config::port_config::api_port())/Ok(test_api_port())/g' "$FILE"
sed -i 's/Port::new(crate::config::port_config::grafana_port())/Ok(test_grafana_port())/g' "$FILE"
sed -i 's/Port::new(crate::config::port_config::admin_port())/Ok(test_admin_port())/g' "$FILE"

echo "Refactoring complete. Backup saved to $FILE.backup"
echo "Changes made:"
echo "  - Replaced Port::new(...) with test helper functions"
echo ""
echo "Testing..."
cargo test --package nestgate-core --lib network::client_tests

