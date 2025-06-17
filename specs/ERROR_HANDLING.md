---
title: NestGate Error Handling Specification
description: Comprehensive error handling approach for the NestGate storage system
version: 1.0.0
date: July 2024
priority: High
---

# NestGate Error Handling Specification

## Executive Summary

This document outlines the comprehensive error handling strategy implemented in the NestGate storage system. The approach uses structured error types with proper context propagation, ensuring errors are informative and actionable across all layers of the application.

## Key Design Principles

1. **Context-Rich Errors**: All errors include context about what operation was being performed and relevant identifiers
2. **Structured Error Types**: Specific error variants for different categories of failures
3. **Proper Error Propagation**: Errors flow through the system with appropriate context added at each layer
4. **Retry Mechanisms**: Intelligent retry logic for transient failures
5. **User-Friendly Messages**: Translation of technical errors into user-understandable messages
6. **Comprehensive Logging**: Detailed error logging for debugging and troubleshooting

## Core Error Structure

```rust
pub enum NestGateError {
    // System-level errors
    Internal { message: String, source: Option<Box<dyn std::error::Error + Send + Sync>> },
    
    // I/O and file system errors
    Io { source: std::io::Error, context: String },
    
    // Network-related errors
    Network { message: String, interface: Option<String>, source: Option<Box<dyn std::error::Error + Send + Sync>> },
    Timeout { operation: String, duration: Duration },
    RateLimit { message: String, limit: Option<u32>, reset_after: Option<Duration> },
    
    // Validation errors
    Validation { message: String, field: Option<String> },
    
    // Authentication and authorization errors
    Auth { message: String, user: Option<String> },
    
    // Storage subsystem errors
    Storage { source: Box<StorageError> },
    
    // ZFS-specific errors
    Zfs { message: String, dataset: Option<String>, source: Option<Box<dyn std::error::Error + Send + Sync>> },
    
    // Protocol-specific errors
    Protocol { message: String, protocol: Option<String> },
    
    // Configuration errors
    Config { message: String, key: Option<String> },
}
```

## Error Context and Propagation

Every error in NestGate includes:

1. A descriptive message explaining what went wrong
2. Context about the operation that failed (e.g., dataset name, user ID)
3. The original source error when available
4. Additional metadata relevant to the error type

Example of error propagation:

```rust
async fn create_snapshot(&self, volume_id: &str, name: &str) -> Result<Snapshot, NestGateError> {
    let dataset = match self.get_dataset(volume_id).await {
        Ok(dataset) => dataset,
        Err(e) => return Err(NestGateError::Zfs {
            message: format!("Failed to get dataset for volume {}", volume_id),
            dataset: Some(volume_id.to_string()),
            source: Some(Box::new(e)),
        }),
    };
    
    match dataset.snapshot(name, false).await {
        Ok(snapshot) => Ok(snapshot),
        Err(e) => Err(NestGateError::Zfs {
            message: format!("Failed to create snapshot {} for volume {}", name, volume_id),
            dataset: Some(volume_id.to_string()),
            source: Some(Box::new(e)),
        }),
    }
}
```

## Retry Mechanism

NestGate implements an intelligent retry system for transient failures:

```rust
pub struct RetryConfig {
    pub max_attempts: usize,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub jitter: bool,
    pub backoff_factor: f64,
}

pub trait Retry<T> {
    async fn retry(self) -> Result<T, NestGateError>;
    async fn retry_with_config(self, config: RetryConfig) -> Result<T, NestGateError>;
}
```

The retry mechanism includes:

1. Exponential backoff with configurable parameters
2. Jitter to prevent thundering herd problems
3. Intelligent categorization of which errors are retriable
4. Detailed logging of retry attempts

## Error Logging

All errors are logged with appropriate severity levels:

1. **Trace**: For detailed debugging information
2. **Debug**: For developer-focused error details
3. **Info**: For expected error conditions during normal operation
4. **Warn**: For unexpected but recoverable errors
5. **Error**: For serious errors that affect functionality
6. **Fatal**: For critical errors requiring immediate attention

Example logging:

```rust
match operation().await {
    Ok(result) => result,
    Err(e) => {
        error!(
            error.kind = %e.kind(),
            error.message = %e,
            operation = "create_volume",
            volume_id = %volume_id,
            "Failed to create volume"
        );
        return Err(e);
    }
}
```

## User-Facing Error Messages

NestGate provides user-friendly error messages while preserving technical details for troubleshooting:

```rust
impl Display for NestGateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal { message, .. } => write!(f, "Internal error: {}", message),
            Self::Io { context, .. } => write!(f, "I/O error: {}", context),
            Self::Network { message, .. } => write!(f, "Network error: {}", message),
            Self::Timeout { operation, duration } => {
                write!(f, "{} timed out after {:?}", operation, duration)
            }
            Self::RateLimit { message, .. } => write!(f, "Rate limit exceeded: {}", message),
            Self::Validation { message, field } => {
                if let Some(field_name) = field {
                    write!(f, "Validation error in field '{}': {}", field_name, message)
                } else {
                    write!(f, "Validation error: {}", message)
                }
            }
            // Other variants...
        }
    }
}
```

## Error Handling in CLI and UI

1. **CLI**: Provides error codes, concise messages, and detailed verbose output when requested
2. **UI**: Shows user-friendly error notifications with actionable steps for resolution
3. **API**: Returns structured error responses with appropriate HTTP status codes and error details

## Examples

### Network Request Error Handling

```rust
pub async fn simulate_network_request(attempt: usize) -> Result<String, NestGateError> {
    if attempt == 0 {
        return Err(NestGateError::Timeout {
            operation: "Connection".to_string(),
            duration: Duration::from_secs(5),
        });
    } else if attempt == 1 {
        return Err(NestGateError::Network {
            message: "Connection refused".to_string(),
            interface: Some("eth0".to_string()),
            source: None,
        });
    } else if attempt == 2 {
        return Err(NestGateError::RateLimit {
            message: "Too many requests, try again later".to_string(),
            limit: Some(60),
            reset_after: Some(Duration::from_secs(10)),
        });
    }

    Ok("Response data".to_string())
}
```

### File Error Handling with Context

```rust
pub async fn read_file_with_context(path: &str) -> Result<String, NestGateError> {
    match tokio::fs::File::open(path).await {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents).await {
                Ok(_) => Ok(contents),
                Err(e) => Err(NestGateError::Io {
                    source: e,
                    context: format!("Failed to read from file at {}", path),
                })
            }
        },
        Err(e) => Err(NestGateError::Io {
            source: e,
            context: format!("Failed to open file at {}", path),
        })
    }
}
```

### ZFS Operations with Error Handling

```rust
pub async fn simulate_zfs_operations() -> Result<(), NestGateError> {
    // Create dataset
    let dataset_name = "tank/data/ai-models";
    info!("Creating dataset {}", dataset_name);
    
    // Simulated dataset creation
    let dataset = match create_dataset(dataset_name).await {
        Ok(dataset) => dataset,
        Err(e) => {
            return Err(NestGateError::Zfs {
                message: format!("Failed to create dataset {}", dataset_name),
                dataset: Some(dataset_name.to_string()),
                source: Some(Box::new(e)),
            });
        }
    };
    
    // Create snapshot
    let snapshot_name = "backup-20230101";
    info!("Creating snapshot {}", snapshot_name);
    
    // Simulated snapshot creation
    if let Err(e) = create_snapshot(dataset_name, snapshot_name).await {
        return Err(NestGateError::Zfs {
            message: format!("Failed to create snapshot {}", snapshot_name),
            dataset: Some(dataset_name.to_string()),
            source: Some(Box::new(e)),
        });
    }
    
    // Mount volume
    info!("Mounting volume {}", dataset_name);
    
    // Simulated mount operation
    if let Err(e) = mount_volume(dataset_name).await {
        return Err(NestGateError::Zfs {
            message: format!("Failed to mount volume {}", dataset_name),
            dataset: Some(dataset_name.to_string()),
            source: Some(Box::new(e)),
        });
    }
    
    info!("ZFS operations completed successfully");
    Ok(())
}
```

### Validation Error Handling

```rust
pub fn validate_user_input(username: &str, email: &str) -> Result<(), NestGateError> {
    if username.is_empty() {
        return Err(NestGateError::Validation {
            message: "Username cannot be empty".to_string(),
            field: Some("username".to_string()),
        });
    }
    
    if username.len() < 3 {
        return Err(NestGateError::Validation {
            message: "Username must be at least 3 characters".to_string(),
            field: Some("username".to_string()),
        });
    }
    
    if !email.contains('@') {
        return Err(NestGateError::Validation {
            message: "Invalid email address format".to_string(),
            field: Some("email".to_string()),
        });
    }
    
    Ok(())
}
```

## Conclusion

The NestGate error handling system provides a comprehensive approach to managing errors throughout the application. By using structured error types, proper context, and intelligent retry mechanisms, the system ensures that errors are informative, actionable, and help maintain system reliability. 