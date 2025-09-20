// **PASSWORD MANAGEMENT**

use super::config::PasswordPolicy;

/// Password manager
pub struct PasswordManager {
    policy: PasswordPolicy,
}
impl PasswordManager {
    pub const fn new(policy: PasswordPolicy) -> Self {
        Self { policy }
    }
} 