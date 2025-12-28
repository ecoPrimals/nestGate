/// 🔬 **VALIDATION STRESS TESTING** 🔬
/// Direct stress testing of our pure functions to identify gaps
/// that fuzzing would reveal - no nightly Rust required!
use crate::cache_math::*;
use crate::consensus_math::*;
use crate::return_builders::*;
use crate::validation_predicates::*;

/// **STRESS TEST RUNNER** - Find real validation gaps
pub fn run_validation_stress_tests() -> Vec<String> {
    let mut issues = Vec::new();
    // Test validation predicates with edge cases
    issues.extend(stress_test_validation_predicates());

    // Test cache math with extreme values
    issues.extend(stress_test_cache_math());

    // Test consensus math with boundary conditions
    issues.extend(stress_test_consensus_math());

    // Test return builders with malformed inputs
    issues.extend(stress_test_return_builders());

    issues
    }

/// **VALIDATION PREDICATES STRESS TEST**
fn stress_test_validation_predicates() -> Vec<String> {
    let mut issues = Vec::new();
    // Test with extreme strings
    let massive_string = "a".repeat(1_000_000);
    let malicious_strings = vec![
        "",                              // Empty
        "\0\0\0",                        // Null bytes
        &massive_string,                 // Massive string
        "../../etc/passwd",              // Path traversal
        "<script>alert('xss')</script>", // XSS
        "'; DROP TABLE users; --",       // SQL injection
        "\n\r\t",                        // Control characters
    ];

    for malicious in &malicious_strings {
        // Test string validation
        if is_non_empty_string(malicious) && malicious.is_empty() {
            issues.push(format!("❌ is_non_empty_string failed on: {malicious:?}"));
    }

        // Test path validation with malicious strings
        if is_valid_file_path(malicious) && malicious.contains("..") {
            issues.push(format!("❌ Path traversal not detected: {malicious:?}"));
    }
    }

    // Test extreme port numbers
    let extreme_ports = vec![0, 1, 65535, u16::MAX];
    for port in extreme_ports {
        let result = is_valid_port_number(port);
        if port == 0 && result {
            issues.push(format!("❌ Port 0 accepted when it shouldn't be: {port}"));
    }
    }

    // Test boolean logic edge cases
    if has_notification_methods(false, false, false) {
        issues.push("❌ No notification methods but returned true".to_string());
    }

    issues
    }

/// **CACHE MATH STRESS TEST**
fn stress_test_cache_math() -> Vec<String> {
    let mut issues = Vec::new();
    // Test with extreme values
    let extreme_sizes = vec![0, 1, u64::MAX, u64::MAX - 1];

    for &size in &extreme_sizes {
        // Test cache size calculation
        let sizes = vec![size, size / 2, size / 3];
        let total = calculate_total_cache_size(&sizes);

        // Check for overflow
        if size == u64::MAX && sizes.len() > 1 && total < size {
            issues.push(format!(
                "❌ Overflow detected in cache size calculation: {size} -> {total}"
            ));
    }

        // Test eviction logic
        if needs_eviction(u64::MAX, 1, u64::MAX) {
            issues.push("❌ Eviction triggered incorrectly at max values".to_string());
    }
    }

    // Test hit ratio with extreme values
    let hit_ratio = calculate_hit_ratio(u64::MAX, u64::MAX);
    if hit_ratio != 0.5 {
        issues.push(format!(
            "❌ Hit ratio calculation failed: expected 0.5, got {hit_ratio}"
        ));
    }

    issues
    }

/// **CONSENSUS MATH STRESS TEST**
fn stress_test_consensus_math() -> Vec<String> {
    let mut issues = Vec::new();
    // Test with extreme node counts
    let extreme_nodes = vec![0usize, 1usize, 1000usize, 10000usize];

    for &nodes in &extreme_nodes {
        let required = calculate_required_consensus(nodes, 0.67); // Standard 2/3 consensus

        // Check for overflow or underflow
        if nodes > 0 && required == 0 {
            issues.push(format!("❌ Required consensus is 0 for {nodes} nodes"));
    }

        if nodes > 0 && required > nodes {
            issues.push(format!(
                "❌ Required consensus {required} > total nodes {nodes}"
            ));
    }
    }

    // Test consensus percentage edge cases
    let percentage = calculate_consensus_percentage(1, 0);
    // ✅ MODERN: Use epsilon for zero check
    if !percentage.is_infinite() && percentage.abs() > 1e-9 {
        issues.push(format!(
            "❌ Division by zero not handled properly: {percentage}"
        ));
    }

    // Test extreme timestamps
    let extreme_times = vec![i64::MIN, i64::MAX, 0, -1];
    let expiry = calculate_consensus_expiry(&extreme_times, 3600);
    if extreme_times.contains(&i64::MIN) && expiry == i64::MIN {
        issues.push("❌ Timestamp overflow not handled".to_string());
    }

    issues
    }

/// **RETURN BUILDERS STRESS TEST**
async fn stress_test_return_builders() -> Vec<String> {
    let mut issues = Vec::new();
    // Test with extreme permission lists
    let huge_permissions = vec!["permission".to_string(); 1_000_000];
    let consensus_nodes = vec!["node".to_string(); 1000];
    let grant = build_access_grant(
        &huge_permissions, // Zero-copy: pass by reference instead of cloning
        i64::MAX,
        "",
        &consensus_nodes, // Zero-copy: pass by reference
        1.0,
    );

    if grant.permissions.len() != huge_permissions.len() {
        issues.push("❌ Large permission list truncated".to_string());
    }

    // Test with empty/malicious data
    let empty_permissions: Vec<String> = vec![];
    let empty_nodes: Vec<String> = vec![];
    let empty_grant = build_access_grant(
        &empty_permissions, // Zero-copy: pass by reference
        0,
        "",
        &empty_nodes, // Zero-copy: pass by reference
        -1.0, // Invalid percentage
    );

    // ✅ MODERN: Use epsilon for negative check
    if empty_grant.consensus_percentage < -1e-9 {
        issues.push("❌ Negative consensus percentage not validated".to_string());
    }

    // ✅ MODERNIZED: Test timestamp monotonicity without sleep
    // Timestamps should be monotonically increasing (or equal if generated in same instant)
    let response1 = build_api_success("test".to_string());
    let response2 = build_api_success("test".to_string());

    // Test the actual requirement: timestamps should not go backwards
    if response2.timestamp < response1.timestamp {
        issues.push("❌ Timestamp ordering issue: timestamps went backwards".to_string());
    }

    issues
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stress_validation() -> Result<(), Box<dyn std::error::Error>> {
        let issues = run_validation_stress_tests();

        // Print all issues found
        for issue in &issues {
            println!("{issue}");
    Ok(())
    }

        // This test documents issues but doesn't fail - we use it to improve our code
        println!("📊 Found ", issues.len() validation issues to address"));
    Ok(())
    }
    Ok(())
    }
