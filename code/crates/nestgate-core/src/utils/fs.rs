// Removed unused imports - will add back when needed
/// File System Utilities
/// Comprehensive file and directory operations for NestGate
use std::fs as stdfs;
use std::io;
// ==================== SECTION ====================

/// Checks if a path exists.
#[must_use]
    path.exists()
}
/// Creates a directory and all its parent directories if they don't exist.
///
/// # Errors
/// Returns an error if the directory cannot be created.
    stdfs::create_dir_all(path)
}
/// Removes a file or directory and all its contents.
///
/// # Errors
/// Returns an error if the path cannot be removed.
    if path.is_dir() {
        stdfs::remove_dir_all(path)
    } else {
        stdfs::remove_file(path)
    }
}
/// Gets the size of a file in bytes.
///
/// # Errors
/// Returns an error if the file size cannot be determined.
    stdfs::metadata(path).map(|m| m.len())
}
/// Recursively calculates the size of a directory
    let mut total_size = 0;
    if path.is_dir() {
        for entry in stdfs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;

            if metadata.is_dir() {
                total_size += get_directory_size(&entry.path())?;
            } else {
                total_size += metadata.len();
            }
        }
    } else {
        total_size = stdfs::metadata(path)?.len();
    }

    Ok(total_size)
}

/// Copy files and directories recursively  
pub fn copy_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if src.is_dir() {
        stdfs::create_dir_all(dst)?;
        for entry in stdfs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            copy_recursive(&src_path, &dst_path)?;
        }
        Ok(())
    } else {
        if let Some(parent) = dst.parent() {
            stdfs::create_dir_all(parent)?;
        }
        stdfs::copy(src, dst)?;
        Ok(())
    }
}
// ==================== SECTION ====================

/// Check if a path is readable
    match stdfs::metadata(path) {
        Ok(metadata) => {
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                let mode = metadata.mode();
                // Check if owner has read permission (simplified check)
                (mode & 0o400) != 0
            }
            #[cfg(not(unix))]
            {
                // On non-Unix systems, assume readable if metadata can be read
                true
            }
        }
        Err(_) => false,
    }
}
/// Check if a path is writable
    match stdfs::metadata(path) {
        Ok(metadata) => {
            #[cfg(unix)]
            {
                let mode = metadata.mode();
                // Check if owner has write permission (simplified check)
                (mode & 0o200) != 0
            }
            #[cfg(not(unix))]
            {
                // On non-Unix systems, try to open for writing
                if path.is_dir() {
                    // For directories, try to create a temporary file
                    let temp_file = path.join(".write_test");
                    match stdfs::File::create(&temp_file) {
                        Ok(_) => {
                            let _ = stdfs::remove_file(&temp_file);
                            true
                        }
                        Err(_) => false,
                    }
                } else {
                    // For files, try to open for writing
                    stdfs::OpenOptions::new().write(true).open(path).is_ok()
                }
            }
        }
        Err(_) => false,
    }
}
/// Check if a path is executable
    match stdfs::metadata(path) {
        Ok(metadata) => {
            #[cfg(unix)]
            {
                let mode = metadata.mode();
                // Check if owner has execute permission (simplified check)
                (mode & 0o100) != 0
            }
            #[cfg(not(unix))]
            {
                // On Windows, check file extension
                if let Some(ext) = path.extension() {
                    matches!(
                        ext.to_str().unwrap_or("").to_lowercase().as_str(),
                        "exe" | "bat" | "cmd" | "com" | "scr" | "msi"
                    )
                } else {
                    false
                }
            }
        }
        Err(_) => false,
    }
}
// ==================== SECTION ====================

/// Find files matching a pattern recursively
pub fn find_files<P: AsRef<Path>>(dir: P, pattern: &str) -> io::Result<Vec<PathBuf>> {
    let mut results = Vec::new();
    find_files_recursive(dir.as_ref(), pattern, &mut results)?;
    Ok(results)
}
fn find_files_recursive(dir: &Path, pattern: &str, results: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in stdfs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                find_files_recursive(&path, pattern, results)?;
            } else if let Some(filename) = path.file_name() {
                if let Some(filename_str) = filename.to_str() {
                    if filename_str.contains(pattern) {
                        results.push(path);
                    }
                }
            }
        }
    }
    Ok(())
}

/// Get file extension as string
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}
/// Get filename without extension
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(|s| s.to_string())
}
/// Check if a path has a specific extension
    get_extension(path)
        .map(|e| e == ext.to_lowercase())
        .unwrap_or(false)
}
/// Create a unique filename by appending a number if the file exists
    if !base_path.exists() {
        return base_path.to_path_buf();
    }
    let parent = base_path.parent().unwrap_or(Path::new(""));
    let stem = get_stem(base_path).unwrap_or_else(|| "file".to_string());
    let ext = get_extension(base_path);

    for i in 1..10000 {
        let filename = if let Some(extension) = &ext {
            format!("{stem}_{i}.{extension}")
        } else {
            format!("{stem}_{i}")
        };

        let new_path = parent.join(filename);
        if !new_path.exists() {
            return new_path;
        }
    }

    // Fallback - just append timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let filename = if let Some(extension) = &ext {
        format!("{stem}_{timestamp}.{extension}")
    } else {
        format!("{stem}_{timestamp}")
    };

    parent.join(filename)
}

// ==================== SECTION ====================

/// Get temporary directory path
pub fn temp_dir() -> PathBuf {
    std::env::temp_dir()
}
/// Create a temporary file with a unique name
pub fn create_temp_file(prefix: &str, suffix: &str) -> io::Result<PathBuf> {
    let temp_dir = temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let filename = format!("{prefix}{timestamp}{suffix}");
    let temp_path = temp_dir.join(filename);

    // Create the file to reserve the name
    stdfs::File::create(&temp_path)?;

    Ok(temp_path)
}

/// Get home directory path
pub fn home_dir() -> Option<PathBuf> {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()
        .map(PathBuf::from)
}
// ==================== SECTION ====================

/// Check if a path is a directory
    path.is_dir()
}
/// Check if a path is a file
    path.is_file()
}
/// Check if a path is absolute
    path.is_absolute()
}
/// Convert path to absolute path
    stdfs::canonicalize(path)
}
/// Get parent directory of a path
    path.parent()
}
/// Join two paths
    base.as_ref().join(path)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_filesystem_operations() {
        let dir = tempdir().unwrap_or_else(|e| {
            tracing::error!("Failed to create temporary directory: {:?}", e);
            panic!(
                "Test setup failed: cannot create temporary directory: {:?}",
                e
            );
        );
        let file_path = dir.path().join("test.txt");
        let dir_path = dir.path().join("subdir");

        // Test file creation and size
        stdfs::File::create(&file_path)
            .unwrap_or_else(|e| {
                tracing::error!("Expect failed ({}): {:?}", "Failed to create test file", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Operation failed - {}: {:?}",
                        "Failed to create test file", e
                    ),
                )
                .into());
            })
            .write_all(b"test")
            .unwrap_or_else(|e| {
                tracing::error!("Expect failed ({}): {:?}", "Failed to write test data", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Operation failed - {}: {:?}",
                        "Failed to write test data", e
                    ),
                )
                .into());
            );

        assert_eq!(
            get_file_size(&file_path).unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {e:?}"),
                )
                .into());
            }),
            4
        );
        assert!(exists(&file_path));
        assert!(is_file(&file_path));

        // Test directory creation
        ensure_dir(&dir_path).unwrap_or_else(|e| {
            tracing::error!("Expect failed ({}): {:?}", "Failed to create directory", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Operation failed - {}: {:?}",
                    "Failed to create directory", e
                ),
            )
            .into());
        );
        assert!(exists(&dir_path));
        assert!(is_directory(&dir_path));

        // Test file removal
        remove_path(&file_path).unwrap_or_else(|e| {
            tracing::error!("Expect failed ({}): {:?}", "Failed to remove file", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed - {}: {:?}", "Failed to remove file", e),
            )
            .into());
        );
        assert!(!exists(&file_path));

        // Test directory removal
        remove_path(&dir_path).unwrap_or_else(|e| {
            tracing::error!("Expect failed ({}): {:?}", "Failed to remove directory", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Operation failed - {}: {:?}",
                    "Failed to remove directory", e
                ),
            )
            .into());
        );
        assert!(!exists(&dir_path));
    }

    #[test]
    fn test_path_utilities() {
        let path = Path::new("/home/user/file.txt");

        assert_eq!(get_extension(path), Some("txt"));
        assert_eq!(get_stem(path), Some("file"));
        assert!(has_extension(path, "txt"));
        assert!(!has_extension(path, "rs"));
    }

    #[test]
    fn test_unique_filename() {
        let temp_dir = tempdir().unwrap_or_else(|e| {
            tracing::error!("Expect failed ({}): {:?}", "Failed to create temp dir", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Operation failed - {}: {:?}",
                    "Failed to create temp dir", e
                ),
            )
            .into());
        );
        let base_path = temp_dir.path().join("test.txt");

        // Create the base file
        stdfs::File::create(&base_path).unwrap_or_else(|e| {
            tracing::error!("Expect failed ({}): {:?}", "Failed to create base file", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Operation failed - {}: {:?}",
                    "Failed to create base file", e
                ),
            )
            .into());
        );

        // Get unique filename
        let unique_path = make_unique_filename(&base_path);
        assert_ne!(unique_path, base_path);
        assert!(!unique_path.exists());
    }
}
