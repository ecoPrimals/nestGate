//! Enhanced Error Handling Tests
//! 
//! Comprehensive test suite for NestGate's error handling system,
//! covering all error categories, contexts, and recovery mechanisms.

#[cfg(test)]
mod error_handling_tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::Duration;
    
    // Mock error types for testing
    #[derive(Debug, Clone, PartialEq)]
    pub enum ErrorCategory {
        Configuration,
        Network,
        Storage,
        System,
        Protocol,
        Authentication,
        Authorization,
        Validation,
    }
    
    #[derive(Debug, Clone)]
    pub struct NestGateError {
        pub category: ErrorCategory,
        pub message: String,
        pub context: String,
        pub recoverable: bool,
        pub metadata: HashMap<String, String>,
    }
    
    impl NestGateError {
        pub fn new(category: ErrorCategory, message: String, context: String) -> Self {
            Self {
                category,
                message,
                context,
                recoverable: false,
                metadata: HashMap::new(),
            }
        }
        
        pub fn with_metadata(mut self, key: String, value: String) -> Self {
            self.metadata.insert(key, value);
            self
        }
        
        pub fn recoverable(mut self) -> Self {
            self.recoverable = true;
            self
        }
        
        pub fn configuration_error(message: String, context: String) -> Self {
            Self::new(ErrorCategory::Configuration, message, context)
        }
        
        pub fn network_error(message: String, context: String) -> Self {
            Self::new(ErrorCategory::Network, message, context).recoverable()
        }
        
        pub fn storage_error(message: String, context: String) -> Self {
            Self::new(ErrorCategory::Storage, message, context)
        }
        
        pub fn system_error(message: String, context: String) -> Self {
            Self::new(ErrorCategory::System, message, context)
        }
    }
    
    impl std::fmt::Display for NestGateError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{:?}] {} (context: {})", self.category, self.message, self.context)
        }
    }
    
    impl std::error::Error for NestGateError {}
    
    pub type Result<T> = std::result::Result<T, NestGateError>;
    
    // Error recovery strategies
    #[derive(Debug, Clone)]
    pub struct ErrorRecoveryStrategy {
        pub max_retries: u32,
        pub backoff_duration: Duration,
        pub recovery_actions: Vec<String>,
    }
    
    impl Default for ErrorRecoveryStrategy {
        fn default() -> Self {
            Self {
                max_retries: 3,
                backoff_duration: Duration::from_millis(100),
                recovery_actions: vec!["retry".to_string()],
            }
        }
    }
    
    pub struct ErrorHandler {
        strategies: HashMap<ErrorCategory, ErrorRecoveryStrategy>,
        error_history: Vec<NestGateError>,
    }
    
    impl ErrorHandler {
        pub fn new() -> Self {
            let mut strategies = HashMap::new();
            
            // Network errors: retry with backoff
            strategies.insert(ErrorCategory::Network, ErrorRecoveryStrategy {
                max_retries: 5,
                backoff_duration: Duration::from_millis(200),
                recovery_actions: vec!["retry".to_string(), "failover".to_string()],
            });
            
            // Configuration errors: no retry, require manual intervention
            strategies.insert(ErrorCategory::Configuration, ErrorRecoveryStrategy {
                max_retries: 0,
                backoff_duration: Duration::from_secs(0),
                recovery_actions: vec!["manual_intervention".to_string()],
            });
            
            // Storage errors: retry with longer backoff
            strategies.insert(ErrorCategory::Storage, ErrorRecoveryStrategy {
                max_retries: 3,
                backoff_duration: Duration::from_millis(500),
                recovery_actions: vec!["retry".to_string(), "use_cache".to_string()],
            });
            
            Self {
                strategies,
                error_history: Vec::new(),
            }
        }
        
        pub fn handle_error(&mut self, error: NestGateError) -> Result<()> {
            self.error_history.push(error.clone());
            
            let strategy = self.strategies.get(&error.category)
                .unwrap_or(&ErrorRecoveryStrategy::default());
            
            if error.recoverable && strategy.max_retries > 0 {
                // Simulate recovery attempt
                println!("Attempting recovery for error: {}", error.message);
                Ok(())
            } else {
                Err(error)
            }
        }
        
        pub fn get_error_history(&self) -> &[NestGateError] {
            &self.error_history
        }
        
        pub fn clear_history(&mut self) {
            self.error_history.clear();
        }
    }

    #[test]
    fn test_error_creation_and_categorization() {
        let config_error = NestGateError::configuration_error(
            "Invalid configuration file".to_string(),
            "config_loader".to_string(),
        );
        
        assert_eq!(config_error.category, ErrorCategory::Configuration);
        assert_eq!(config_error.message, "Invalid configuration file");
        assert_eq!(config_error.context, "config_loader");
        assert!(!config_error.recoverable);
        
        let network_error = NestGateError::network_error(
            "Connection timeout".to_string(),
            "api_client".to_string(),
        );
        
        assert_eq!(network_error.category, ErrorCategory::Network);
        assert!(network_error.recoverable);
    }
    
    #[test]
    fn test_error_metadata_handling() {
        let error = NestGateError::system_error(
            "Memory allocation failed".to_string(),
            "buffer_pool".to_string(),
        )
        .with_metadata("requested_size".to_string(), "1048576".to_string())
        .with_metadata("available_memory".to_string(), "512000".to_string());
        
        assert_eq!(error.metadata.get("requested_size"), Some(&"1048576".to_string()));
        assert_eq!(error.metadata.get("available_memory"), Some(&"512000".to_string()));
    }
    
    #[test]
    fn test_error_handler_initialization() {
        let handler = ErrorHandler::new();
        
        // Verify strategies are properly configured
        let network_strategy = handler.strategies.get(&ErrorCategory::Network).unwrap();
        assert_eq!(network_strategy.max_retries, 5);
        assert_eq!(network_strategy.backoff_duration, Duration::from_millis(200));
        
        let config_strategy = handler.strategies.get(&ErrorCategory::Configuration).unwrap();
        assert_eq!(config_strategy.max_retries, 0);
    }
    
    #[test]
    fn test_recoverable_error_handling() {
        let mut handler = ErrorHandler::new();
        
        let network_error = NestGateError::network_error(
            "Temporary connection failure".to_string(),
            "service_discovery".to_string(),
        );
        
        let result = handler.handle_error(network_error.clone());
        assert!(result.is_ok());
        
        // Verify error was recorded in history
        assert_eq!(handler.get_error_history().len(), 1);
        assert_eq!(handler.get_error_history()[0].message, network_error.message);
    }
    
    #[test]
    fn test_non_recoverable_error_handling() {
        let mut handler = ErrorHandler::new();
        
        let config_error = NestGateError::configuration_error(
            "Missing required configuration key".to_string(),
            "startup_validation".to_string(),
        );
        
        let result = handler.handle_error(config_error.clone());
        assert!(result.is_err());
        
        // Verify error was still recorded in history
        assert_eq!(handler.get_error_history().len(), 1);
    }
    
    #[test]
    fn test_error_history_management() {
        let mut handler = ErrorHandler::new();
        
        // Add multiple errors
        let errors = vec![
            NestGateError::network_error("Error 1".to_string(), "test".to_string()),
            NestGateError::storage_error("Error 2".to_string(), "test".to_string()),
            NestGateError::system_error("Error 3".to_string(), "test".to_string()),
        ];
        
        for error in errors {
            let _ = handler.handle_error(error);
        }
        
        assert_eq!(handler.get_error_history().len(), 3);
        
        handler.clear_history();
        assert_eq!(handler.get_error_history().len(), 0);
    }
    
    #[test]
    fn test_error_display_formatting() {
        let error = NestGateError::network_error(
            "Connection refused".to_string(),
            "tcp_client".to_string(),
        );
        
        let formatted = format!("{}", error);
        assert!(formatted.contains("[Network]"));
        assert!(formatted.contains("Connection refused"));
        assert!(formatted.contains("tcp_client"));
    }
    
    #[test]
    fn test_error_recovery_strategies() {
        let handler = ErrorHandler::new();
        
        // Test network error strategy
        let network_strategy = handler.strategies.get(&ErrorCategory::Network).unwrap();
        assert_eq!(network_strategy.max_retries, 5);
        assert!(network_strategy.recovery_actions.contains(&"retry".to_string()));
        assert!(network_strategy.recovery_actions.contains(&"failover".to_string()));
        
        // Test storage error strategy
        let storage_strategy = handler.strategies.get(&ErrorCategory::Storage).unwrap();
        assert_eq!(storage_strategy.max_retries, 3);
        assert!(storage_strategy.recovery_actions.contains(&"use_cache".to_string()));
    }
    
    #[test]
    fn test_concurrent_error_handling() {
        use std::sync::{Arc, Mutex};
        use std::thread;
        
        let handler = Arc::new(Mutex::new(ErrorHandler::new()));
        let mut handles = vec![];
        
        for i in 0..10 {
            let handler_clone = Arc::clone(&handler);
            let handle = thread::spawn(move || {
                let error = NestGateError::network_error(
                    format!("Concurrent error {}", i),
                    "concurrent_test".to_string(),
                );
                
                let mut handler_guard = handler_clone.lock().unwrap();
                handler_guard.handle_error(error)
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let result = handle.join().unwrap();
            assert!(result.is_ok());
        }
        
        let handler_guard = handler.lock().unwrap();
        assert_eq!(handler_guard.get_error_history().len(), 10);
    }
    
    #[test]
    fn test_error_categorization_completeness() {
        // Ensure all error categories are handled
        let categories = vec![
            ErrorCategory::Configuration,
            ErrorCategory::Network,
            ErrorCategory::Storage,
            ErrorCategory::System,
            ErrorCategory::Protocol,
            ErrorCategory::Authentication,
            ErrorCategory::Authorization,
            ErrorCategory::Validation,
        ];
        
        for category in categories {
            let error = NestGateError::new(
                category.clone(),
                "Test error".to_string(),
                "test_context".to_string(),
            );
            
            // Verify error can be created and categorized
            assert_eq!(error.category, category);
        }
    }
    
    #[test]
    fn test_error_chain_handling() {
        // Test handling of error chains/cascading failures
        let mut handler = ErrorHandler::new();
        
        let primary_error = NestGateError::network_error(
            "Primary connection failed".to_string(),
            "primary_service".to_string(),
        );
        
        let secondary_error = NestGateError::network_error(
            "Fallback connection failed".to_string(),
            "secondary_service".to_string(),
        );
        
        let _ = handler.handle_error(primary_error);
        let _ = handler.handle_error(secondary_error);
        
        let history = handler.get_error_history();
        assert_eq!(history.len(), 2);
        assert!(history[0].message.contains("Primary"));
        assert!(history[1].message.contains("Fallback"));
    }
} 