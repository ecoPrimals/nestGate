//! Unit tests for semantic router

#[test]
fn test_semantic_method_names() {
    // Verify semantic method naming conventions
    let storage_methods = vec![
        "storage.put",
        "storage.get",
        "storage.delete",
        "storage.list",
        "storage.dataset.create",
    ];

    for method in storage_methods {
        assert!(method.contains('.'), "Method should use dot notation: {}", method);
        assert!(method.starts_with("storage."), "Storage method should start with storage.: {}", method);
    }
}
