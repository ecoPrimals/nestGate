// **JWT TOKEN MANAGEMENT**

use super::types::TokenClaims;

/// Token manager
pub struct TokenManager {
    secret: String,
}
impl TokenManager {
    /// Creates a new instance
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
} 