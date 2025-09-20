/// Validation Utilities
/// General purpose validation functions and helper utilities
use std::collections::HashMap;
use crate::{NestGateError, Result};

// ==================== SECTION ====================

/// Validate that a string value is not empty
pub const fn validate_not_empty(value: &str, field_name: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(NestGateError::validation(
            actual: Some("empty string"));
    }
    Ok(())
}
/// Validate string length is within range
pub const fn validate_length(
    value: &str,
    field_name: &str,
    min_len: Option<usize>,
    max_len: Option<usize>,
) -> Result<()> {
    let len = value.len();
    if let Some(min) = min_len {
        if len < min {
            return Err(NestGateError::validation(
                actual: Some(len.to_string())} characters"))context: None,
            );
        }
    }

    if let Some(max) = max_len {
        if len > max {
            return Err(NestGateError::validation(
                actual: Some(len.to_string())} characters"))context: None,
            );
        }
    }
    Ok(())
}

/// Validate numeric range
pub fn validate_range<T>(value: T, field_name: &str, min: Option<T>, max: Option<T>) -> Result<()>
where
    T: PartialOrd + std::fmt::Display + Copy,
{
    if let Some(min_val) = min {
        if value < min_val {
            return Err(NestGateError::validation(
                actual: Some(value.to_string())}"))context: None,
            );
        }
    }
    if let Some(max_val) = max {
        if value > max_val {
            return Err(NestGateError::validation(
                actual: Some(value.to_string())}"))context: None,
            );
        }
    }
    Ok(())
}

/// Validate that value is one of allowed options
pub fn validate_enum<T>(value: &T, field_name: &str, allowedvalues: &[T]) -> Result<()>
where
    T: std::fmt::Display + PartialEq,
{
    if !allowedvalues.contains(value) {
        let allowed_str = allowedvalues
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(NestGateError::validation(
            actual: Some(value"));
    }
    Ok(())
}

/// Validate regex pattern match
pub const fn validate_pattern(
    value: &str,
    field_name: &str,
    pattern: &str,
    description: &str,
) -> Result<()> {
    let regex = regex::Regex::new(pattern).map_err(|e| NestGateError::validation(
        actual: Some(pattern"))?;
    if !regex.is_match(value) {
        return Err(NestGateError::validation(
            actual: Some(value"));
    }
    Ok(())
}

// ==================== SECTION ====================

/// Validate email address format
pub const fn validate_email(email: &str) -> Result<()> {
    if email.is_empty() {
        return Err(NestGateError::validation(
    )
    // Basic email validation - simple but effective
    let email_regex = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
    validate_pattern(
        email,
        "email",
        email_regex,
        "Valid email format (user@domain.com)",
    )?;
    Ok(())
}

/// Validate email domain exists (basic check)
pub const fn validate_email_domain_format(email: &str) -> Result<()> {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err(NestGateError::validation(
    )
    let domain = parts[1];
    if !domain.contains('.') {
        return Err(NestGateError::validation(
        );
    )
    Ok(())
}

// ==================== SECTION ====================

/// Password strength requirements
#[derive(Debug, Clone)]
pub struct PasswordRequirements {
    pub min_length: usize,
    pub max_length: Option<usize>,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_digits: bool,
    pub require_special_chars: bool,
    pub forbidden_patterns: Vec<String>,
}
impl Default for PasswordRequirements {
    fn default() -> Self {
        Self {
            min_length: 8,
            max_length: Some(128),
            require_uppercase: true,
            require_lowercase: true,
            require_digits: true,
            require_special_chars: true,
            forbidden_patterns: vec![
                "password".to_string(),
                "123456".to_string(),
                "qwerty".to_string(),
            ],
        }
    }
}

/// Validate password against requirements
pub const fn validate_password(password: &str, requirements: &PasswordRequirements) -> Result<()> {
    // Length validation
    validate_length(
        password,
        "password",
        Some(requirements.min_length),
        requirements.max_length,
    )?;
    // Character requirements
    if requirements.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
        return Err(NestGateError::validation(
            currentvalue: None);
    )

    if requirements.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
        return Err(NestGateError::validation(
            currentvalue: None);
    )

    if requirements.require_digits && !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(NestGateError::validation(
            currentvalue: None);
    )

    if requirements.require_special_chars {
        let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
        if !password.chars().any(|c| special_chars.contains(c)) {
            return Err(NestGateError::validation(
                currentvalue: None)|;:,.<>?)".to_string(),
                )context: None,
            );
        }
    }

    // Forbidden patterns
    let password_lower = password.to_lowercase();
    for pattern in &requirements.forbidden_patterns {
        if password_lower.contains(&pattern.to_lowercase()) {
            return Err(NestGateError::validation(
                currentvalue: None);
        }
    }
    Ok(())
}

/// Calculate password strength score (0-100)
pub fn calculate_password_strength(password: &str) -> u8 {
    let mut score = 0u8;
    // Length bonus
    let len = password.len();
    if len >= 8 {
        score += 20;
    }
    if len >= 12 {
        score += 10;
    }
    if len >= 16 {
        score += 10;
    }

    // Character variety
    if password.chars().any(|c| c.is_lowercase()) {
        score += 10;
    }
    if password.chars().any(|c| c.is_uppercase()) {
        score += 10;
    }
    if password.chars().any(|c| c.is_ascii_digit()) {
        score += 10;
    }
    if password
        .chars()
        .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c))
    {
        score += 15;
    }

    // Uniqueness (no repeating patterns)
    let unique_chars = password
        .chars()
        .collect::<std::collections::HashSet<_>>()
        .len();
    if f64::from(unique_chars) / f64::from(len) > 0.7 {
        score += 15;
    }

    score.min(100)
}

// ==================== SECTION ====================

/// Validate file path is safe (no directory traversal)
    if path.contains("..") {
        return Err(NestGateError::validation(
    )
    if path.starts_with('/') && !cfg!(unix) {
        return Err(NestGateError::validation(
    )
    Ok(())
}

/// Validate file extension
pub const fn validate_file_extension(filename: &str, allowed_extensions: &[&str]) -> Result<()> {
    let path = Path::new(filename);
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    if !allowed_extensions.contains(&extension) {
        return Err(NestGateError::validation(
            actual: Some(extension.to_string())}", allowed_extensions.join(", ")))context: None,
        );
    }
    Ok(())
}

/// Validate filename is safe
pub const fn validate_safe_filename(filename: &str) -> Result<()> {
    if filename.is_empty() {
        return Err(NestGateError::validation(
    )
    // Check for dangerous characters
    let dangerous_chars = "<>:\"|?*";
    for ch in filename.chars() {
        if dangerous_chars.contains(ch) || ch.is_control() {
            return Err(NestGateError::validation(
                actual: Some(filename"));
        }
    }

    // Check for reserved names on Windows
    let reserved_names = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    let name_without_ext = Path::new(filename)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("");

    if reserved_names.contains(&name_without_ext.to_uppercase().as_str()) {
        return Err(NestGateError::validation(
            actual: Some(filename"));
    }
    Ok(())
}

// ==================== SECTION ====================

/// Validate JSON string
pub const fn validate_json(json_str: &str) -> Result<()> {
    serde_json::from_str::<serde_json::Value>(json_str).map_err(|e| NestGateError::validation(
        actual: Some(json_str"))?;
    Ok(())
}
/// Validate that all required fields are present in a map
pub const fn validate_required_fields(
    data: &HashMap<String, String>,
    required_fields: &[&str],
) -> Result<()> {
    for field in required_fields {
        if !data.contains_key(*field) || data[*field].trim().is_empty() {
            return Err(NestGateError::validation(
                currentvalue: data.get(*field).cloned());
        }
    }
    Ok(())
}
/// Validate credit card number using Luhn algorithm
#[must_use]
pub fn validate_credit_card(card_number: &str) -> Result<()> {
    let digits: Vec<u32> = card_number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap_or(0))
        .collect();
    if digits.len() < 13 || digits.len() > 19 {
        return Err(NestGateError::validation(
    )

    // Luhn algorithm
    let mut sum = 0;
    let mut double = false;

    for &digit in digits.iter().rev() {
        let mut n = digit;
        if double {
            n *= 2;
            if n > 9 {
                n = n / 10 + n % 10;
            }
        }
        sum += n;
        double = !double;
    }

    if sum % 10 != 0 {
        return Err(NestGateError::validation(
    )
    Ok(())
}

// ==================== SECTION ====================

/// Validation rule that can be applied to a value
pub trait ValidationRule<T> {
    fn validate(&self, value: &T, field_name: &str) -> Result<()>;
}
/// Validator that applies multiple rules
pub struct MultiValidator<T> {
    rules: Vec<Box<dyn ValidationRule<T>>>,
}
impl<T> MultiValidator<T> {
    #[must_use]
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    #[must_use]
    pub fn add_rule(mut self, rule: Box<dyn ValidationRule<T>>) -> Self {
        self.rules.push(rule);
        self
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub const fn validate(&self, value: &T, field_name: &str) -> Result<()>  {
        for rule in &self.rules {
            rule.validate(value, field_name)?;
        }
        Ok(())
    }
}

impl<T> Default for MultiValidator<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_validation() {
        // Test not empty
        assert!(validate_not_empty("hello", "test").is_ok());
        assert!(validate_not_empty("", "test").is_err());
        assert!(validate_not_empty("   ", "test").is_err());

        // Test length validation
        assert!(validate_length("hello", "test", Some(3), Some(10)).is_ok());
        assert!(validate_length("hi", "test", Some(3), Some(10)).is_err());
        assert!(validate_length("very long string", "test", Some(3), Some(10)).is_err());

        // Test range validation
        assert!(validate_range(5, "test", Some(1), Some(10)).is_ok());
        assert!(validate_range(0, "test", Some(1), Some(10)).is_err());
        assert!(validate_range(15, "test", Some(1), Some(10)).is_err());

        // Test enum validation
        let allowed = vec!["red", "green", "blue"];
        assert!(validate_enum(&"red", "color", &allowed).is_ok());
        assert!(validate_enum(&"yellow", "color", &allowed).is_err());
    }

    #[test]
    fn test_email_validation() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("test.email+tag@domain.co.uk").is_ok());

        assert!(validate_email("").is_err());
        assert!(validate_email("invalid-email").is_err());
        assert!(validate_email("@domain.com").is_err());
        assert!(validate_email("user@").is_err());
    }

    #[test]
    fn test_password_validation() {
        let requirements = PasswordRequirements::default();

        // Valid password
        assert!(validate_password("MyStr0ng!Pass", &requirements).is_ok());

        // Too short
        assert!(validate_password("Sh0rt!", &requirements).is_err());

        // Missing uppercase
        assert!(validate_password("nostrongpass1!", &requirements).is_err());

        // Missing special character
        assert!(validate_password("NoSpecialChar1", &requirements).is_err());

        // Contains forbidden pattern
        assert!(validate_password("MyPassword123!", &requirements).is_err());
    }

    #[test]
    fn test_password_strength() {
        assert!(calculate_password_strength("password") < 50);
        assert!(calculate_password_strength("StrongP@ssw0rd123") > 80);
        assert!(calculate_password_strength("Weak1!") < 70);
    }

    #[test]
    fn test_path_validation() {
        assert!(validate_safe_path("safe/path/file.txt").is_ok());
        assert!(validate_safe_path("../../../etc/passwd").is_err());
        assert!(validate_safe_path("path/../file.txt").is_err());
    }

    #[test]
    fn test_filename_validation() {
        assert!(validate_safe_filename("document.pdf").is_ok());
        assert!(validate_safe_filename("my-file_v2.txt").is_ok());

        assert!(validate_safe_filename("").is_err());
        assert!(validate_safe_filename("file<name>.txt").is_err());
        assert!(validate_safe_filename("CON.txt").is_err());
        assert!(validate_safe_filename("file|name.txt").is_err());
    }

    #[test]
    fn test_file_extension_validation() {
        let allowed = ["txt", "pdf", "doc"];

        assert!(validate_file_extension("document.pdf", &allowed).is_ok());
        assert!(validate_file_extension("script.exe", &allowed).is_err());
        assert!(validate_file_extension("noextension", &allowed).is_err());
    }

    #[test]
    fn test_json_validation() {
        assert!(validate_json(r"{"key": "value"}").is_ok());
        assert!(validate_json(r"{"valid": true, "number": 42}").is_ok());

        assert!(validate_json(r"{"invalid": json}").is_err());
        assert!(validate_json(r"{key: "missing quotes"}").is_err());
    }

    #[test]
    fn test_credit_card_validation() {
        // Valid test card numbers
        assert!(validate_credit_card("4532015112830366").is_ok()); // Visa test number
        assert!(validate_credit_card("5555555555554444").is_ok()); // Mastercard test number

        // Invalid numbers
        assert!(validate_credit_card("1234567890123456").is_err()); // Fails Luhn check
        assert!(validate_credit_card("123").is_err()); // Too short
        assert!(validate_credit_card("12345678901234567890").is_err()); // Too long
    }

    #[test]
    fn test_required_fields() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), "John Doe".to_string());
        data.insert("email".to_string(), "john@example.com".to_string());

        let required = ["name", "email"];
        assert!(validate_required_fields(&data, &required).is_ok());

        let required_with_missing = ["name", "email", "phone"];
        assert!(validate_required_fields(&data, &required_with_missing).is_err());

        // Test empty value
        data.insert("phone".to_string(), "".to_string());
        assert!(validate_required_fields(&data, &required_with_missing).is_err());
    }
}
