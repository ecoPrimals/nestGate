//! **JWT SECRET VALIDATION**
//!
//! Security validation for JWT secrets to prevent production deployment
//! with insecure default values.

use std::env;

/// Default JWT secret that must NOT be used in production
const INSECURE_DEFAULT_SECRET: &str = "CHANGE_ME_IN_PRODUCTION";
/// Insecure Alternate 1
const INSECURE_ALTERNATE_1: &str = "change-me-in-production";
/// Insecure Alternate 2
const INSECURE_ALTERNATE_2: &str = "default";
/// Insecure Alternate 3
const INSECURE_ALTERNATE_3: &str = "secret";
/// Insecure Alternate 4
const INSECURE_ALTERNATE_4: &str = "test";

/// Minimum secure JWT secret length (in bytes)
const MINIMUM_SECRET_LENGTH: usize = 32;

/// JWT secret validation error
#[derive(Debug, Clone)]
/// Error type for JwtSecret operations
pub struct JwtSecretError {
    pub message: String,
    /// Help
    pub help: String,
}

impl std::fmt::Display for JwtSecretError {
    /// Fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JWT Security Error: {}\n\nHelp: {}",
            self.message, self.help
        )
    }
}

impl std::error::Error for JwtSecretError {}

/// Validate JWT secret at startup
///
/// # Errors
///
/// Returns `JwtSecretError` if:
/// - JWT secret is set to a known insecure default
/// - JWT secret is too short (< 32 bytes)
/// - JWT secret is not set and no default override is allowed
///
/// # Security
///
/// This function enforces secure JWT secret configuration to prevent
/// production deployments with compromised security.
pub fn validate_jwt_secret() -> Result<(), JwtSecretError> {
    // Get JWT secret from environment or constants
    let jwt_secret = env::var("NESTGATE_JWT_SECRET")
        .or_else(|_| env::var("JWT_SECRET"))
        .unwrap_or_else(|_| {
            // Use the same default as SecurityConstants
            use crate::constants::consolidated::SecurityConstants;
            SecurityConstants::default().jwt_secret().to_string()
        });

    // Check for known insecure defaults
    if jwt_secret == INSECURE_DEFAULT_SECRET
        || jwt_secret == INSECURE_ALTERNATE_1
        || jwt_secret == INSECURE_ALTERNATE_2
        || jwt_secret == INSECURE_ALTERNATE_3
        || jwt_secret == INSECURE_ALTERNATE_4
    {
        return Err(JwtSecretError {
            message: format!(
                "CRITICAL SECURITY ERROR: JWT secret is set to insecure default value: '{}'",
                jwt_secret
            ),
            help: "To fix this, set a secure JWT secret using environment variables:\n\n\
                 # Generate a secure random secret (recommended):\n\
                 export NESTGATE_JWT_SECRET=$(openssl rand -base64 48)\n\n\
                 # Or set your own strong secret (minimum 32 characters):\n\
                 export NESTGATE_JWT_SECRET=\"your-very-secure-secret-key-here\"\n\n\
                 The JWT secret protects authentication tokens and must be kept secure.\n\
                 Using default values in production is a critical security vulnerability."
                .to_string(),
        });
    }

    // Check minimum length
    if jwt_secret.len() < MINIMUM_SECRET_LENGTH {
        return Err(JwtSecretError {
            message: format!(
                "CRITICAL SECURITY ERROR: JWT secret is too short ({} bytes, minimum {} bytes required)",
                jwt_secret.len(),
                MINIMUM_SECRET_LENGTH
            ),
            help: format!(
                "To fix this, generate a secure JWT secret:\n\n\
                 # Generate a secure random secret:\n\
                 export NESTGATE_JWT_SECRET=$(openssl rand -base64 48)\n\n\
                 This will create a {}-byte secret that meets security requirements.\n\
                 Strong secrets are essential for protecting user authentication.",
                48 // base64(48) = 64 chars
            ),
        });
    }

    // Check for common weak patterns
    if jwt_secret.chars().all(|c| c.is_ascii_digit())
        || jwt_secret.chars().all(|c| c.is_ascii_lowercase())
        || jwt_secret.chars().all(|c| c.is_ascii_uppercase())
    {
        eprintln!(
            "⚠️  WARNING: JWT secret appears to be weak (only digits, only lowercase, or only uppercase).\n\
             Consider using a randomly generated secret for better security:\n\
             export NESTGATE_JWT_SECRET=$(openssl rand -base64 48)\n"
        );
    }

    Ok(())
}

/// Validate JWT secret with detailed error information
///
/// This is the recommended function to call at startup for user-friendly error messages.
pub fn validate_jwt_secret_or_exit() {
    match validate_jwt_secret() {
        Ok(()) => {
            #[cfg(debug_assertions)]
            eprintln!("✅ JWT secret validation passed");
        }
        Err(e) => {
            eprintln!("\n{}\n", "=".repeat(80));
            eprintln!("🚨 NESTGATE STARTUP BLOCKED - SECURITY VALIDATION FAILED");
            eprintln!("{}", "=".repeat(80));
            eprintln!("\n{}\n", e);
            eprintln!("{}", "=".repeat(80));
            eprintln!("\nNestGate will not start with insecure JWT configuration.");
            eprintln!("Fix the security issue above and try again.\n");

            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial] // Serialize env var tests to avoid conflicts
    fn test_insecure_defaults_rejected() {
        // Save current env state
        let original = env::var("NESTGATE_JWT_SECRET").ok();

        // These should all fail validation
        let insecure_values = vec![
            "CHANGE_ME_IN_PRODUCTION",
            "change-me-in-production",
            "default",
            "secret",
            "test",
        ];

        for value in insecure_values {
            env::set_var("NESTGATE_JWT_SECRET", value);
            let result = validate_jwt_secret();
            assert!(result.is_err(), "Should reject insecure value: {}", value);
            env::remove_var("NESTGATE_JWT_SECRET");
        }

        // Restore original env state
        if let Some(val) = original {
            env::set_var("NESTGATE_JWT_SECRET", val);
        }
    }

    #[test]
    #[serial] // Serialize env var tests to avoid conflicts
    fn test_short_secrets_rejected() {
        let original = env::var("NESTGATE_JWT_SECRET").ok();

        env::set_var("NESTGATE_JWT_SECRET", "short");
        let result = validate_jwt_secret();
        assert!(result.is_err(), "Should reject short secret");
        env::remove_var("NESTGATE_JWT_SECRET");

        if let Some(val) = original {
            env::set_var("NESTGATE_JWT_SECRET", val);
        }
    }

    #[test]
    #[serial] // Serialize env var tests to avoid conflicts
    fn test_secure_secret_accepted() {
        let original = env::var("NESTGATE_JWT_SECRET").ok();

        // 48-byte base64 encoded secret (recommended)
        let secure_secret = "dGhpcyBpcyBhIHNlY3VyZSBzZWNyZXQgd2l0aCBzdWZmaWNpZW50IGVudHJvcHk=";
        env::set_var("NESTGATE_JWT_SECRET", secure_secret);
        let result = validate_jwt_secret();
        assert!(result.is_ok(), "Should accept secure secret");
        env::remove_var("NESTGATE_JWT_SECRET");

        if let Some(val) = original {
            env::set_var("NESTGATE_JWT_SECRET", val);
        }
    }

    #[test]
    #[serial] // Serialize env var tests to avoid conflicts
    fn test_minimum_length_secret_accepted() {
        let original = env::var("NESTGATE_JWT_SECRET").ok();

        // Exactly 32 characters (minimum)
        let min_secret = "a".repeat(MINIMUM_SECRET_LENGTH);
        env::set_var("NESTGATE_JWT_SECRET", &min_secret);
        let result = validate_jwt_secret();
        assert!(result.is_ok(), "Should accept minimum length secret");
        env::remove_var("NESTGATE_JWT_SECRET");

        if let Some(val) = original {
            env::set_var("NESTGATE_JWT_SECRET", val);
        }
    }

    #[test]
    fn test_error_message_formatting() {
        env::set_var("NESTGATE_JWT_SECRET", "CHANGE_ME_IN_PRODUCTION");
        let result = validate_jwt_secret();
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.message.contains("CRITICAL SECURITY ERROR"));
        assert!(err.help.contains("openssl rand -base64"));
        env::remove_var("NESTGATE_JWT_SECRET");
    }
}
