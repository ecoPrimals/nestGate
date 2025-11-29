// **PASSWORD MANAGEMENT**

use super::config::PasswordPolicy;

/// Password manager
pub struct PasswordManager {
    policy: PasswordPolicy,
}
impl PasswordManager {
    /// Creates a new instance
    pub fn new(policy: PasswordPolicy) -> Self {
        Self { policy }
    }
} 