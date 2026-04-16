// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use crate::Result;

/// Native async universal service provider trait - replaces #\[async_trait\] `UniversalServiceProvider`
pub trait NativeAsyncUniversalServiceProvider<
    const MAX_SERVICES: usize = 1000,
    const SERVICE_TIMEOUT_SECS: u64 = 300,
>: Send + Sync
{
    /// Type alias for ServiceDefinition
    type ServiceDefinition: Clone + Send + Sync + 'static;
    /// Type alias for ServiceInstance
    type ServiceInstance: Clone + Send + Sync + 'static;
    /// Register service - native async, no Future boxing
    fn register_service(
        &self,
        definition: Self::ServiceDefinition,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Unregister service - direct async method
    fn unregister_service(
        &self,
        service_id: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Get service instance - native async
    fn get_service_instance(
        &self,
        service_id: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self::ServiceInstance>>> + Send;

    /// List services - no Future boxing
    fn list_services(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Self::ServiceDefinition>>> + Send;

    /// Compile-time constants
    /// Returns the maximum number of services.
    #[must_use]
    fn max_services() -> usize {
        MAX_SERVICES
    }
    /// Returns the service timeout in seconds.
    #[must_use]
    fn service_timeout_seconds() -> u64 {
        SERVICE_TIMEOUT_SECS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_service_provider_constants() {
        struct MockProvider;
        impl NativeAsyncUniversalServiceProvider<200, 600> for MockProvider {
            type ServiceDefinition = String;
            type ServiceInstance = String;

            fn register_service(
                &self,
                _def: Self::ServiceDefinition,
            ) -> impl std::future::Future<Output = Result<String>> + Send {
                std::future::ready(Ok("id".to_string()))
            }
            fn unregister_service(
                &self,
                _id: &str,
            ) -> impl std::future::Future<Output = Result<()>> + Send {
                std::future::ready(Ok(()))
            }
            fn get_service_instance(
                &self,
                _id: &str,
            ) -> impl std::future::Future<Output = Result<Option<Self::ServiceInstance>>> + Send
            {
                std::future::ready(Ok(None))
            }
            fn list_services(
                &self,
            ) -> impl std::future::Future<Output = Result<Vec<Self::ServiceDefinition>>> + Send
            {
                std::future::ready(Ok(vec![]))
            }
        }
        assert_eq!(MockProvider::max_services(), 200);
        assert_eq!(MockProvider::service_timeout_seconds(), 600);
    }
}
