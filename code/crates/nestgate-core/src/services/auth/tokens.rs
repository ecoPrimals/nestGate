// **JWT TOKEN MANAGEMENT**

use super::types::TokenClaims;

/// Token manager
pub struct TokenManager {
    secret: String,
}
impl TokenManager {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
} 