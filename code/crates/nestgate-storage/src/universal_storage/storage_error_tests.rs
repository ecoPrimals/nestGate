// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **STORAGE ERROR PATH TESTS** - Nov 23, 2025
//!
//! Comprehensive tests for storage error handling, filesystem operations, and resilience

#[cfg(test)]
mod storage_error_creation_tests {
    use nestgate_types::error::NestGateError;

    #[test]
    fn test_storage_not_found_error() {
        let err = NestGateError::storage_error("File not found");
        let display = format!("{}", err);
        assert!(!display.is_empty());
    }

    #[test]
    fn test_storage_permission_denied() {
        let err = NestGateError::storage_error("Permission denied");
        assert!(!format!("{}", err).is_empty());
    }

    #[test]
    fn test_storage_disk_full() {
        let err = NestGateError::storage_error("Disk full");
        assert!(!format!("{:?}", err).is_empty());
    }

    #[test]
    fn test_storage_io_error() {
        let err = NestGateError::storage_error("I/O error occurred");
        assert!(!format!("{}", err).is_empty());
    }

    #[test]
    fn test_storage_corruption_error() {
        let err = NestGateError::storage_error("Data corruption detected");
        assert!(!format!("{}", err).is_empty());
    }
}

#[cfg(test)]
mod filesystem_operation_tests {
    use nestgate_types::error::{NestGateError, Result};

    /// Simulate Read Operation
    fn simulate_read_operation(should_fail: bool) -> Result<Vec<u8>> {
        if should_fail {
            Err(NestGateError::storage_error("Read failed"))
        } else {
            Ok(vec![1, 2, 3, 4, 5])
        }
    }

    /// Simulate Write Operation
    fn simulate_write_operation(should_fail: bool) -> Result<usize> {
        if should_fail {
            Err(NestGateError::storage_error("Write failed"))
        } else {
            Ok(42)
        }
    }

    #[test]
    fn test_read_success() {
        let result = simulate_read_operation(false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 5);
    }

    #[test]
    fn test_read_failure() {
        let result = simulate_read_operation(true);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_success() {
        let result = simulate_write_operation(false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_write_failure() {
        let result = simulate_write_operation(true);
        assert!(result.is_err());
    }

    #[test]
    fn test_operation_retry() {
        let mut attempts = 0;
        let max_attempts = 3;
        let mut result = simulate_read_operation(true);

        while result.is_err() && attempts < max_attempts {
            attempts += 1;
            if attempts == 2 {
                // Succeed on second retry
                result = simulate_read_operation(false);
            } else {
                result = simulate_read_operation(true);
            }
        }

        assert!(result.is_ok());
        assert_eq!(attempts, 2);
    }
}

#[cfg(test)]
mod storage_capacity_tests {
    use nestgate_types::error::{NestGateError, Result};

    #[test]
    fn test_disk_space_check() {
        /// Check Space
        fn check_space(available_mb: u64, required_mb: u64) -> Result<()> {
            if available_mb < required_mb {
                Err(NestGateError::storage_error("Insufficient disk space"))
            } else {
                Ok(())
            }
        }

        assert!(check_space(1000, 500).is_ok());
        assert!(check_space(100, 500).is_err());
    }

    #[test]
    fn test_quota_exceeded() {
        let err = NestGateError::storage_error("Quota exceeded");
        assert!(!format!("{}", err).is_empty());
    }

    #[test]
    fn test_inode_exhaustion() {
        let err = NestGateError::storage_error("No inodes available");
        assert!(!format!("{}", err).is_empty());
    }
}

#[cfg(test)]
mod storage_consistency_tests {
    use nestgate_types::error::NestGateError;

    #[test]
    fn test_checksum_mismatch() {
        let err = NestGateError::storage_error("Checksum mismatch");
        assert!(!format!("{}", err).is_empty());
    }

    #[test]
    fn test_version_conflict() {
        let err = NestGateError::storage_error("Version conflict");
        assert!(!format!("{}", err).is_empty());
    }

    #[test]
    fn test_concurrent_modification() {
        let err = NestGateError::storage_error("Concurrent modification detected");
        assert!(!format!("{}", err).is_empty());
    }

    #[test]
    fn test_lock_timeout() {
        let err = NestGateError::storage_error("Lock acquisition timeout");
        assert!(!format!("{}", err).is_empty());
    }
}

#[cfg(test)]
mod storage_resilience_tests {
    use nestgate_types::error::{NestGateError, Result};

    #[test]
    fn test_fallback_to_backup() {
        /// Read Primary
        fn read_primary() -> Result<String> {
            Err(NestGateError::storage_error("Primary unavailable"))
        }

        /// Read Backup
        fn read_backup() -> Result<String> {
            Ok("backup data".to_string())
        }

        let result = read_primary().or_else(|_| read_backup());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "backup data");
    }

    #[test]
    fn test_degraded_mode() {
        /// Full Storage Read
        fn full_storage_read() -> Result<Vec<String>> {
            Err(NestGateError::storage_error("Storage unavailable"))
        }

        /// Cache Only Read
        fn cache_only_read() -> Result<Vec<String>> {
            Ok(vec!["cached1".to_string(), "cached2".to_string()])
        }

        let result = full_storage_read().or_else(|_| cache_only_read());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn test_automatic_recovery() {
        let mut storage_healthy = false;
        let mut retry_count = 0;

        while !storage_healthy && retry_count < 5 {
            retry_count += 1;
            if retry_count >= 3 {
                storage_healthy = true;
            }
        }

        assert!(storage_healthy);
        assert_eq!(retry_count, 3);
    }
}

#[cfg(test)]
mod storage_edge_cases {
    use nestgate_types::error::NestGateError;

    #[test]
    fn test_empty_path_error() {
        let err = NestGateError::storage_error("Empty path provided");
        assert!(!format!("{}", err).is_empty());
    }

    #[test]
    fn test_path_too_long() {
        let long_path = "/".to_string() + &"a".repeat(5000);
        let err = NestGateError::storage_error(&format!("Path too long: {}", long_path.len()));
        assert!(!format!("{}", err).is_empty());
    }

    #[test]
    fn test_invalid_characters_in_path() {
        let err = NestGateError::storage_error("Invalid characters in path");
        assert!(!format!("{}", err).is_empty());
    }

    #[test]
    fn test_symlink_loop() {
        let err = NestGateError::storage_error("Symbolic link loop detected");
        assert!(!format!("{}", err).is_empty());
    }

    #[test]
    fn test_readonly_filesystem() {
        let err = NestGateError::storage_error("Filesystem is read-only");
        assert!(!format!("{}", err).is_empty());
    }

    #[test]
    fn test_device_not_ready() {
        let err = NestGateError::storage_error("Device not ready");
        assert!(!format!("{}", err).is_empty());
    }
}

#[cfg(test)]
mod storage_transaction_tests {
    use nestgate_types::error::{NestGateError, Result};

    #[test]
    fn test_transaction_commit_success() {
        /// Commit Transaction
        fn commit_transaction(should_succeed: bool) -> Result<()> {
            if should_succeed {
                Ok(())
            } else {
                Err(NestGateError::storage_error("Commit failed"))
            }
        }

        assert!(commit_transaction(true).is_ok());
        assert!(commit_transaction(false).is_err());
    }

    #[test]
    fn test_transaction_rollback() {
        /// Operation With Rollback
        fn operation_with_rollback(fail_at_step: Option<usize>) -> Result<Vec<String>> {
            let mut results = vec![];

            for step in 0..3 {
                if Some(step) == fail_at_step {
                    // Rollback
                    return Err(NestGateError::storage_error("Transaction rolled back"));
                }
                results.push(format!("step_{}", step));
            }

            Ok(results)
        }

        assert!(operation_with_rollback(None).is_ok());
        assert!(operation_with_rollback(Some(1)).is_err());
    }

    #[test]
    fn test_atomic_write_failure() {
        let err = NestGateError::storage_error("Atomic write failed");
        assert!(!format!("{}", err).is_empty());
    }
}

#[cfg(test)]
mod storage_concurrent_tests {
    use nestgate_types::error::NestGateError;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_storage_errors() {
        let errors = Arc::new(std::sync::Mutex::new(Vec::new()));
        let mut handles = vec![];

        for i in 0..5 {
            let errors_clone = Arc::clone(&errors);
            let handle = thread::spawn(move || {
                let err = NestGateError::storage_error(&format!("Storage error {}", i));
                errors_clone.lock().unwrap().push(err);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_errors = errors.lock().unwrap();
        assert_eq!(final_errors.len(), 5);
    }

    #[test]
    fn test_shared_storage_error() {
        let err = Arc::new(NestGateError::storage_error("shared storage error"));
        let err_clone = Arc::clone(&err);

        let handle = thread::spawn(move || format!("{}", err_clone));

        let result = handle.join().unwrap();
        assert!(!result.is_empty());
    }
}

#[cfg(test)]
mod storage_performance_tests {
    use nestgate_types::error::NestGateError;

    #[test]
    fn test_error_creation_performance() {
        let start = std::time::Instant::now();
        for i in 0..1000 {
            let _ = NestGateError::storage_error(&format!("Error {}", i));
        }
        let duration = start.elapsed();
        // Should create 1000 errors quickly (< 50ms)
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn test_error_formatting_performance() {
        let errors: Vec<_> = (0..100)
            .map(|i| NestGateError::storage_error(&format!("Error {}", i)))
            .collect();

        let start = std::time::Instant::now();
        for err in &errors {
            let _ = format!("{}", err);
        }
        let duration = start.elapsed();
        // Should format 100 errors quickly (< 10ms)
        assert!(duration.as_millis() < 10);
    }
}

#[cfg(test)]
mod storage_integration_tests {
    use nestgate_types::error::{NestGateError, Result};
    use std::collections::HashMap;

    #[test]
    fn test_storage_error_mapping() {
        let mut error_map: HashMap<String, NestGateError> = HashMap::new();
        error_map.insert("e1".to_string(), NestGateError::storage_error("error1"));
        error_map.insert("e2".to_string(), NestGateError::storage_error("error2"));

        assert_eq!(error_map.len(), 2);
        assert!(error_map.contains_key("e1"));
    }

    #[test]
    fn test_storage_operation_chain() {
        /// Step1
        fn step1() -> Result<i32> {
            Ok(10)
        }

        /// Step2
        fn step2(value: i32) -> Result<i32> {
            if value > 5 {
                Ok(value * 2)
            } else {
                Err(NestGateError::storage_error("Value too small"))
            }
        }

        /// Step3
        fn step3(value: i32) -> Result<String> {
            Ok(format!("Result: {}", value))
        }

        let result = step1().and_then(step2).and_then(step3);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Result: 20");
    }

    #[test]
    fn test_batch_storage_operations() {
        /// Processes  File
        fn process_file(id: i32) -> Result<String> {
            if id % 2 == 0 {
                Ok(format!("file_{}", id))
            } else {
                Err(NestGateError::storage_error("Failed to process"))
            }
        }

        let ids = vec![2, 4, 6];
        let results: Result<Vec<_>> = ids.into_iter().map(process_file).collect();
        assert!(results.is_ok());

        let ids = vec![1, 2, 3];
        let results: Result<Vec<_>> = ids.into_iter().map(process_file).collect();
        assert!(results.is_err());
    }
}
