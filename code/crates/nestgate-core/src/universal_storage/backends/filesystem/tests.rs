// Comprehensive tests for the filesystem storage backend

use super::*;
use std::fs;
use tempfile::TempDir;
use tokio::runtime::Runtime;

/// Test filesystem backend initialization and configuration
#[test]
async fn test_filesystem_backend_initialization() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new().unwrap();
    let mut config = std::collections::HashMap::new();
    config.insert(
        "root_dir".to_string(),
        temp_dir.path().to_string_lossy().to_string(),
    );
    config.insert("atomic_writes".to_string(), "true".to_string());
    config.insert("track_metadata".to_string(), "true".to_string());
    config.insert("max_file_size".to_string(), (1024 * 1024).to_string()); // 1MB
    let backend = FilesystemBackend::new(&config);
    assert!(backend.is_ok());

    println!("✅ Filesystem backend initialization tested");
    Ok(())
}

/// Test file operations error paths
#[tokio::test]
async fn test_file_operation_errors() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new().unwrap();
    let mut config = std::collections::HashMap::new();
    config.insert(
        "root_dir".to_string(),
        temp_dir.path().to_string_lossy().to_string(),
    );
    config.insert("max_file_size".to_string(), "1024".to_string());
    let backend = FilesystemBackend::new(&config).unwrap();

    // Test file not found
    let result = backend.read_file("nonexistent.txt").await;
    assert!(result.is_err());

    // Test file too large
    let large_content = vec![0u8; 2048]; // Larger than max_file_size
    let result = backend.write_file("large.txt", &large_content).await;
    assert!(result.is_err());

    println!("✅ File operation error paths tested");
    Ok(())
}

/// Test path traversal security
#[tokio::test]
async fn test_path_traversal_security() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new().unwrap();
    let mut config = std::collections::HashMap::new();
    config.insert(
        "root_dir".to_string(),
        temp_dir.path().to_string_lossy().to_string(),
    );
    let backend = FilesystemBackend::new(&config).unwrap();

    // Test various path traversal attempts
    let malicious_paths = [
        "../../../etc/passwd",
        "..\\..\\..\\windows\\system32\\config\\sam",
        "test/../../../sensitive.txt",
        "/absolute/path",
        "valid\\..\\..\\traversal",
    ];

    for malicious_path in &malicious_paths {
        let result = backend.read_file(malicious_path).await;
        assert!(
            result.is_err(),
            "Should reject path traversal: {}",
            malicious_path
        );
    Ok(())
    }

    println!("✅ Path traversal security tested");
    Ok(())
}

/// Test concurrent operations
#[tokio::test]
async fn test_concurrent_operations() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new().unwrap();
    let mut config = std::collections::HashMap::new();
    config.insert(
        "root_dir".to_string(),
        temp_dir.path().to_string_lossy().to_string(),
    );
    let backend = std::sync::Arc::new(FilesystemBackend::new(&config).unwrap());

    // Test concurrent reads and writes
    let mut handles = Vec::new();

    for i in 0..10 {
        let backend_clone = backend.clone();
        let handle = tokio::spawn(async move {
            let filename = format!("concurrent_{"actual_error_details"}.txt");
            let content = format!("Content for file {"actual_error_details"}");

            // Write file
            backend_clone
                .write_file(&filename, content.as_bytes())
                .await
                .unwrap();

            // Read file back
            let read_content = backend_clone.read_file(&filename).await.unwrap();
            assert_eq!(read_content, content.as_bytes());
        );
        handles.push(handle);
    Ok(())
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    Ok(())
    }

    println!("✅ Concurrent operations tested");
    Ok(())
}

/// Test atomic write operations
#[tokio::test]
async fn test_atomic_operations() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new().unwrap();
    let mut config = std::collections::HashMap::new();
    config.insert(
        "root_dir".to_string(),
        temp_dir.path().to_string_lossy().to_string(),
    );
    config.insert("atomic_writes".to_string(), "true".to_string());
    let backend = FilesystemBackend::new(&config).unwrap();

    // Test atomic write operations
    let content = b"atomic test content";
    backend
        .write_file("atomic_test.txt", content)
        .await
        .unwrap();

    // Verify file was written atomically
    let read_content = backend.read_file("atomic_test.txt").await.unwrap();
    assert_eq!(read_content, content);

    println!("✅ Atomic operations tested");
    Ok(())
}

/// Test directory operations
#[tokio::test]
async fn test_directory_operations() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new().unwrap();
    let mut config = std::collections::HashMap::new();
    config.insert(
        "root_dir".to_string(),
        temp_dir.path().to_string_lossy().to_string(),
    );
    let backend = FilesystemBackend::new(&config).unwrap();

    // Test directory creation
    backend.create_directory("test_dir").await.unwrap();
    assert!(backend.file_exists("test_dir").await.unwrap());

    // Test file creation in directory
    backend
        .write_file("test_dir/file.txt", b"content")
        .await
        .unwrap();
    let content = backend.read_file("test_dir/file.txt").await.unwrap();
    assert_eq!(content, b"content");

    // Test directory listing
    let files = backend.list_directory("test_dir").await.unwrap();
    assert!(files.contains(&"file.txt"));

    // Test directory deletion
    backend.delete_directory("test_dir").await.unwrap();
    assert!(!backend.file_exists("test_dir").await.unwrap());

    println!("✅ Directory operations tested");
    Ok(())
}

/// Test metadata operations
#[tokio::test]
async fn test_metadata_operations() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new().unwrap();
    let mut config = std::collections::HashMap::new();
    config.insert(
        "root_dir".to_string(),
        temp_dir.path().to_string_lossy().to_string(),
    );
    let backend = FilesystemBackend::new(&config).unwrap();

    // Create test file
    let content = b"test metadata content";
    backend
        .write_file("metadata_test.txt", content)
        .await
        .unwrap();

    // Test metadata retrieval
    let metadata = backend.get_metadata("metadata_test.txt").await.unwrap();
    assert_eq!(metadata.size, content.len() as u64);
    assert!(metadata.created.is_some());
    assert!(metadata.modified.is_some());
    assert_eq!(metadata.mime_type, Some("text/plain"));

    println!("✅ Metadata operations tested");
    Ok(())
}

/// Test edge cases and boundary conditions
#[tokio::test]
async fn test_edge_cases() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new().unwrap();
    let mut config = std::collections::HashMap::new();
    config.insert(
        "root_dir".to_string(),
        temp_dir.path().to_string_lossy().to_string(),
    );
    let backend = FilesystemBackend::new(&config).unwrap();

    // Test empty file
    backend.write_file("empty.txt", b").await.unwrap();
    let content = backend.read_file("empty.txt").await.unwrap();
    assert_eq!(content.len(), 0);

    // Test file with special characters in name
    backend
        .write_file("special-file_123.txt", b"special content")
        .await
        .unwrap();
    let content = backend.read_file("special-file_123.txt").await.unwrap();
    assert_eq!(content, b"special content");

    // Test overwriting existing file
    backend
        .write_file("overwrite.txt", b"original")
        .await
        .unwrap();
    backend
        .write_file("overwrite.txt", b"updated")
        .await
        .unwrap();
    let content = backend.read_file("overwrite.txt").await.unwrap();
    assert_eq!(content, b"updated");

    println!("✅ Edge cases tested");
    Ok(())
}

/// Test error recovery scenarios
#[tokio::test]
async fn test_error_recovery() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new().unwrap();
    let mut config = std::collections::HashMap::new();
    config.insert(
        "root_dir".to_string(),
        temp_dir.path().to_string_lossy().to_string(),
    );
    let backend = FilesystemBackend::new(&config).unwrap();

    // Test recovery from non-existent directory
    let result = backend.list_directory("nonexistent").await;
    assert!(result.is_err());

    // Test recovery from invalid file operations
    let result = backend.delete_file("nonexistent.txt").await;
    assert!(result.is_err());

    println!("✅ Error recovery scenarios tested");
    Ok(())
}

/// Test configuration variations
#[test]
fn test_configuration_variations() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new().unwrap();
    // Test minimal configuration
    let mut minimal_config = std::collections::HashMap::new();
    minimal_config.insert(
        "root_dir".to_string(),
        temp_dir.path().to_string_lossy().to_string(),
    );

    let backend = FilesystemBackend::new(&minimal_config);
    assert!(backend.is_ok());

    // Test full configuration
    let mut full_config = std::collections::HashMap::new();
    full_config.insert(
        "root_dir".to_string(),
        temp_dir.path().to_string_lossy().to_string(),
    );
    full_config.insert("atomic_writes".to_string(), "false".to_string());
    full_config.insert("track_metadata".to_string(), "false".to_string());
    full_config.insert("max_file_size".to_string(), "0".to_string()); // Unlimited

    let backend = FilesystemBackend::new(&full_config);
    assert!(backend.is_ok());

    println!("✅ Configuration variations tested");
    Ok(())
}
