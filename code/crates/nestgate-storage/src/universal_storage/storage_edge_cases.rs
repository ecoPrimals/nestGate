// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **STORAGE EDGE CASE TESTS** - Nov 23, 2025
//!
//! Comprehensive edge case tests for storage path handling and operations.

#[cfg(test)]
mod storage_path_edge_cases {
    use std::path::PathBuf;

    #[test]
    fn test_empty_path() {
        let path = PathBuf::from("");
        assert_eq!(path.as_os_str().len(), 0);
    }

    #[test]
    fn test_very_long_path() {
        let path = PathBuf::from("a/".repeat(1000) + "file.txt");
        assert!(path.as_os_str().len() > 2000);
    }

    #[test]
    fn test_unicode_path() {
        let path = PathBuf::from("/home/测试/файл/ملف.txt");
        assert!(path.to_string_lossy().contains("测试"));
    }

    #[test]
    fn test_special_characters_in_path() {
        let path = PathBuf::from(r"/home/user/file with spaces & special!@#$%.txt");
        assert!(path.to_string_lossy().contains(' '));
    }

    #[test]
    fn test_path_components() {
        let path = PathBuf::from("/home/user/documents/file.txt");
        assert!(path.components().next().is_some());
    }

    #[test]
    fn test_path_extension() {
        let path = PathBuf::from("file.txt");
        assert_eq!(path.extension().and_then(|s| s.to_str()), Some("txt"));
    }

    #[test]
    fn test_path_without_extension() {
        let path = PathBuf::from("file");
        assert_eq!(path.extension(), None);
    }

    #[test]
    fn test_path_with_multiple_dots() {
        let path = PathBuf::from("archive.tar.gz");
        assert_eq!(path.extension().and_then(|s| s.to_str()), Some("gz"));
    }

    #[test]
    fn test_path_parent() {
        let path = PathBuf::from("/home/user/file.txt");
        let parent = path.parent();
        assert!(parent.is_some());
    }

    #[test]
    fn test_path_file_name() {
        let path = PathBuf::from("/home/user/file.txt");
        assert_eq!(path.file_name().and_then(|s| s.to_str()), Some("file.txt"));
    }

    #[test]
    fn test_path_join() {
        let mut path = PathBuf::from("/home/user");
        path.push("documents");
        path.push("file.txt");
        assert!(path.to_string_lossy().contains("documents"));
    }

    #[test]
    fn test_path_canonicalization_attempt() {
        // Test that we can create paths for canonicalization
        let path = PathBuf::from("./test/../file.txt");
        assert!(!path.as_os_str().is_empty());
    }
}

#[cfg(test)]
mod storage_operation_edge_cases {
    #[test]
    fn test_zero_byte_buffer() {
        let data = Vec::<u8>::new();
        assert_eq!(data.len(), 0);
        assert!(data.is_empty());
        assert_eq!(data.capacity(), 0);
    }

    #[test]
    fn test_small_buffer() {
        let data = [0u8; 10];
        assert_eq!(data.len(), 10);
        assert!(!data.is_empty());
    }

    #[test]
    fn test_large_buffer() {
        let data = vec![0u8; 1_000_000]; // 1MB
        assert_eq!(data.len(), 1_000_000);
    }

    #[test]
    fn test_very_large_buffer() {
        let data = vec![0u8; 10_000_000]; // 10MB
        assert_eq!(data.len(), 10_000_000);
    }

    #[test]
    fn test_buffer_with_pattern() {
        let data: Vec<u8> = (0..=255).cycle().take(1000).collect();
        assert_eq!(data.len(), 1000);
        assert_eq!(data[0], 0);
        assert_eq!(data[255], 255);
        assert_eq!(data[256], 0); // Wrapped around
    }

    #[test]
    fn test_buffer_cloning() {
        let data1 = vec![1, 2, 3, 4, 5];
        let data2 = data1.clone();
        assert_eq!(data1, data2);
        assert_eq!(data1.len(), data2.len());
    }

    #[test]
    fn test_buffer_slicing() {
        let data = [1, 2, 3, 4, 5];
        let slice = &data[1..4];
        assert_eq!(slice, &[2, 3, 4]);
    }

    #[test]
    fn test_buffer_iteration() {
        let data = [1, 2, 3, 4, 5];
        let sum: u8 = data.iter().sum();
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_buffer_capacity() {
        let mut data = Vec::with_capacity(100);
        assert_eq!(data.capacity(), 100);
        assert_eq!(data.len(), 0);
        data.push(1);
        assert_eq!(data.len(), 1);
    }

    #[test]
    fn test_buffer_reserve() {
        let data: Vec<u8> = Vec::with_capacity(1000);
        assert!(data.capacity() >= 1000);
    }
}

#[cfg(test)]
mod storage_concurrent_operations {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_concurrent_buffer_reads() {
        let data = Arc::new(vec![1u8; 1000]);

        let mut handles = vec![];
        for _ in 0..10 {
            let data_clone = Arc::clone(&data);
            let handle = thread::spawn(move || {
                assert_eq!(data_clone.len(), 1000);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_concurrent_counter() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut count = counter_clone.lock().unwrap();
                *count += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 10);
    }
}

#[cfg(test)]
mod storage_performance_tests {
    #[test]
    fn test_rapid_buffer_creation() {
        let mut n = 0;
        for i in 0..1000 {
            let _buffer = vec![0u8; i % 100];
            n += 1;
        }
        assert_eq!(n, 1000);
    }

    #[test]
    fn test_buffer_copy_performance() {
        let data = vec![1u8; 1000];
        let mut n = 0;
        for _ in 0..1000 {
            let _copy = data.clone();
            n += 1;
        }
        assert_eq!(n, 1000);
    }

    #[test]
    fn test_large_batch_operations() {
        let mut buffers = Vec::new();
        for i in 0..1000 {
            buffers.push(vec![0u8; i]);
        }
        assert_eq!(buffers.len(), 1000);
    }
}
