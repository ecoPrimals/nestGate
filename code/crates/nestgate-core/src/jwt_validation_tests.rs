//! Comprehensive tests for JWT validation module
//! Added: November 14, 2025 - Coverage Sprint

use crate::jwt_validation::{validate_jwt_secret, JwtSecretError};

#[cfg(test)]
mod jwt_secret_validation_tests {
    use super::*;

    #[test]
    fn test_valid_jwt_secret_minimum_length() {
        // Test minimum valid length (32 characters)
        let secret = "a".repeat(32);
        assert!(validate_jwt_secret(&secret).is_ok());
    }

    #[test]
    fn test_valid_jwt_secret_recommended_length() {
        // Test recommended length (64 characters)
        let secret = "a".repeat(64);
        assert!(validate_jwt_secret(&secret).is_ok());
    }

    #[test]
    fn test_valid_jwt_secret_with_special_chars() {
        // Test with various special characters
        let secret = "ThisIsA!Valid@JWT#Secret$With%Special^Chars&*()1234";
        assert!(validate_jwt_secret(&secret).is_ok());
    }

    #[test]
    fn test_invalid_jwt_secret_too_short() {
        // Test secret that's too short
        let secret = "short";
        match validate_jwt_secret(&secret) {
            Err(JwtSecretError::TooShort { .. }) => (),
            _ => panic!("Expected TooShort error"),
        }
    }

    #[test]
    fn test_invalid_jwt_secret_empty() {
        // Test empty secret
        let secret = "";
        match validate_jwt_secret(&secret) {
            Err(JwtSecretError::TooShort { .. }) => (),
            _ => panic!("Expected TooShort error for empty string"),
        }
    }

    #[test]
    fn test_invalid_jwt_secret_whitespace_only() {
        // Test secret with only whitespace
        let secret = "   ";
        match validate_jwt_secret(&secret) {
            Err(JwtSecretError::TooShort { .. }) => (),
            _ => panic!("Expected TooShort error for whitespace-only string"),
        }
    }

    #[test]
    fn test_jwt_secret_error_display() {
        // Test error message formatting
        let error = JwtSecretError::TooShort {
            actual: 10,
            minimum: 32,
        };
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("10"));
        assert!(error_msg.contains("32"));
    }

    #[test]
    fn test_jwt_secret_boundary_cases() {
        // Test exactly at minimum boundary
        let secret_31 = "a".repeat(31);
        assert!(validate_jwt_secret(&secret_31).is_err());

        let secret_32 = "a".repeat(32);
        assert!(validate_jwt_secret(&secret_32).is_ok());

        let secret_33 = "a".repeat(33);
        assert!(validate_jwt_secret(&secret_33).is_ok());
    }

    #[test]
    fn test_jwt_secret_with_unicode() {
        // Test with unicode characters
        let secret = "ThisIsAValidSecret🔒WithUnicode✓Chars🎉";
        if secret.len() >= 32 {
            assert!(validate_jwt_secret(&secret).is_ok());
        }
    }

    #[test]
    fn test_jwt_secret_with_newlines() {
        // Test with embedded newlines (should still work if long enough)
        let secret = format!("{}\n{}", "a".repeat(20), "b".repeat(20));
        if secret.len() >= 32 {
            assert!(validate_jwt_secret(&secret).is_ok());
        }
    }
}

