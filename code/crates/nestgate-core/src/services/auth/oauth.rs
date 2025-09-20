// **OAUTH PROVIDER MANAGEMENT**

use super::types::OAuthProvider;

/// OAuth manager
pub struct OAuthManager {
    providers: std::collections::HashMap<String, OAuthProvider>,
}
impl OAuthManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
            providers: std::collections::HashMap::new(),
        }
    }
} 