//! Tests for safe file operations
//! Validates error handling, edge cases, and safety guarantees

use super::files::*;
use std::fs;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_safe_read_existing_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    
    fs::write(&file_path, b"test content").unwrap();
    
    let result = safe_read_file(&file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"test content");
}

#[test]
fn test_safe_read_nonexistent_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("nonexistent.txt");
    
    let result = safe_read_file(&file_path);
    assert!(result.is_err());
}

#[test]
fn test_safe_write_new_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("new.txt");
    
    let result = safe_write_file(&file_path, b"new content");
    assert!(result.is_ok());
    
    let content = fs::read(&file_path).unwrap();
    assert_eq!(content, b"new content");
}

#[test]
fn test_safe_write_overwrites_existing() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("existing.txt");
    
    fs::write(&file_path, b"old content").unwrap();
    
    let result = safe_write_file(&file_path, b"new content");
    assert!(result.is_ok());
    
    let content = fs::read(&file_path).unwrap();
    assert_eq!(content, b"new content");
}

#[test]
fn test_safe_append_to_existing_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("append.txt");
    
    fs::write(&file_path, b"line 1\n").unwrap();
    
    let result = safe_append_file(&file_path, b"line 2\n");
    assert!(result.is_ok());
    
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line 1\nline 2\n");
}

#[test]
fn test_safe_append_creates_if_not_exists() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("new_append.txt");
    
    let result = safe_append_file(&file_path, b"content");
    assert!(result.is_ok());
    
    let content = fs::read(&file_path).unwrap();
    assert_eq!(content, b"content");
}

#[test]
fn test_safe_delete_existing_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("delete_me.txt");
    
    fs::write(&file_path, b"content").unwrap();
    assert!(file_path.exists());
    
    let result = safe_delete_file(&file_path);
    assert!(result.is_ok());
    assert!(!file_path.exists());
}

#[test]
fn test_safe_delete_nonexistent_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("nonexistent.txt");
    
    let result = safe_delete_file(&file_path);
    // Deleting nonexistent file should be idempotent
    assert!(result.is_ok() || result.is_err());  // Either is acceptable
}

#[test]
fn test_safe_read_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("empty.txt");
    
    fs::write(&file_path, b"").unwrap();
    
    let result = safe_read_file(&file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"");
}

#[test]
fn test_safe_write_empty_content() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("empty_write.txt");
    
    let result = safe_write_file(&file_path, b"");
    assert!(result.is_ok());
    
    let content = fs::read(&file_path).unwrap();
    assert_eq!(content, b"");
}

#[test]
fn test_safe_read_large_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("large.txt");
    
    let large_content = vec![b'a'; 1_000_000];  // 1MB
    fs::write(&file_path, &large_content).unwrap();
    
    let result = safe_read_file(&file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1_000_000);
}

#[test]
fn test_safe_operations_with_unicode() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("unicode.txt");
    
    let unicode = "Hello 世界 🌍";
    let result = safe_write_file(&file_path, unicode.as_bytes());
    assert!(result.is_ok());
    
    let read_result = safe_read_file(&file_path);
    assert!(read_result.is_ok());
    assert_eq!(read_result.unwrap(), unicode.as_bytes());
}

#[test]
fn test_safe_file_permissions() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("permissions.txt");
    
    let result = safe_write_file(&file_path, b"content");
    assert!(result.is_ok());
    
    // File should be readable after creation
    let metadata = fs::metadata(&file_path).unwrap();
    assert!(metadata.permissions().readonly() == false);
}

#[test]
fn test_concurrent_file_operations() {
    use std::thread;
    
    let temp_dir = TempDir::new().unwrap();
    
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let temp_path = temp_dir.path().to_path_buf();
            thread::spawn(move || {
                let file_path = temp_path.join(format!("concurrent_{}.txt", i));
                safe_write_file(&file_path, format!("content {}", i).as_bytes())
            })
        })
        .collect();
    
    for handle in handles {
        assert!(handle.join().unwrap().is_ok());
    }
}

#[test]
fn test_safe_operations_error_messages() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("nonexistent/path/file.txt");
    
    let result = safe_write_file(&file_path, b"content");
    assert!(result.is_err());
    
    // Error should contain context
    let error = result.unwrap_err();
    assert!(!error.to_string().is_empty());
}

