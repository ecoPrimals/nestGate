// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use crate::Result;

/// Native async security service trait - replaces #\[async_trait\] `SecurityService`
/// **DEPRECATED**: Service pattern consolidated into canonical security
#[deprecated(
    since = "0.9.0",
    note = "Use crate::traits::canonical::CanonicalSecurity for security services"
)]
/// NativeAsyncSecurityService trait
pub trait NativeAsyncSecurityService<
    const MAX_SESSIONS: usize = 1000,
    const SESSION_DURATION_SECS: u64 = 300,
>: Send + Sync
{
    /// Type alias for AuthRequest
    type AuthRequest: Clone + Send + Sync + 'static;
    /// Type alias for AuthResponse
    type AuthResponse: Clone + Send + Sync + 'static;
    /// Authenticate - native async, no Future boxing
    fn authenticate(
        &self,
        request: Self::AuthRequest,
    ) -> impl std::future::Future<Output = Result<Self::AuthResponse>> + Send;

    /// Validate token - direct async method
    fn validate_token(&self, token: &str)
    -> impl std::future::Future<Output = Result<bool>> + Send;

    /// Compile-time constants
    /// Returns the maximum number of sessions.
    #[must_use]
    fn max_sessions() -> usize {
        MAX_SESSIONS
    }
    /// Returns the session duration in seconds.
    #[must_use]
    fn session_duration_seconds() -> u64 {
        SESSION_DURATION_SECS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockSecurity;

    #[expect(
        deprecated,
        reason = "Exercises deprecated NativeAsyncSecurityService for coverage"
    )]
    impl NativeAsyncSecurityService<100, 200> for MockSecurity {
        type AuthRequest = String;
        type AuthResponse = String;

        fn authenticate(
            &self,
            _request: Self::AuthRequest,
        ) -> impl std::future::Future<Output = Result<Self::AuthResponse>> + Send {
            std::future::ready(Ok("tok".into()))
        }
        fn validate_token(
            &self,
            _token: &str,
        ) -> impl std::future::Future<Output = Result<bool>> + Send {
            std::future::ready(Ok(true))
        }
    }

    #[test]
    #[expect(
        deprecated,
        reason = "Exercises deprecated NativeAsyncSecurityService for coverage"
    )]
    fn security_service_constants() {
        assert_eq!(MockSecurity::max_sessions(), 100);
        assert_eq!(MockSecurity::session_duration_seconds(), 200);
    }

    #[tokio::test]
    #[expect(
        deprecated,
        reason = "Exercises deprecated NativeAsyncSecurityService for coverage"
    )]
    async fn security_service_async_methods() {
        let s = MockSecurity;
        s.authenticate("req".into()).await.expect("auth");
        s.validate_token("t").await.expect("val");
    }
}
