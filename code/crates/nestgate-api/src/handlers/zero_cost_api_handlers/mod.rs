// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **ZERO-COST API HANDLERS**
//! This module replaces `async_trait` patterns in API handlers with native async methods
//! for maximum performance in high-frequency request handling.

mod dataset_handler;
mod migration;
mod pool_handler;
mod router;
mod serde_helpers;
mod types;

pub use dataset_handler::ZeroCostDatasetHandler;
#[cfg(any(test, feature = "dev-stubs"))]
pub use migration::ApiHandlerBenchmark;
pub use migration::ApiHandlerMigrationGuide;
pub use pool_handler::{
    DevelopmentPoolHandler, EnterprisePoolHandler, HighThroughputPoolHandler,
    ProductionPoolHandler, ZeroCostPoolHandler,
};
pub use router::ZeroCostRouterBuilder;
pub use types::{
    ApiError, ApiStatus, DatasetConfig, DatasetInfo, DatasetType, ZeroCostApiError,
    ZeroCostApiHandler, ZeroCostApiRequest, ZeroCostApiResponse, ZeroCostDatasetManager,
};

#[cfg(test)]
mod zero_cost_api_handlers_unit_tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn zero_cost_pool_handler_default_and_consts() {
        type H = ZeroCostPoolHandler<64, 1000>;
        let h = H::default();
        assert_eq!(H::max_requests(), 64);
        assert_eq!(H::timeout_ms(), 1000);
        let _ = &h;
    }

    #[test]
    fn zero_cost_request_response_serde_roundtrip() {
        let req = ZeroCostApiRequest {
            data: serde_json::json!({"k": 1}),
            request_id: Arc::new("rid-1".into()),
            timestamp: std::time::SystemTime::UNIX_EPOCH,
            metadata: Arc::new(HashMap::from([("a".into(), "b".into())])),
        };
        let j = serde_json::to_string(&req).unwrap();
        let back: ZeroCostApiRequest<serde_json::Value> = serde_json::from_str(&j).unwrap();
        assert_eq!(*back.request_id, "rid-1");

        let resp = ZeroCostApiResponse {
            data: serde_json::json!(null),
            request_id: Arc::new("r".into()),
            status: ApiStatus::Warning {
                message: "w".into(),
            },
            processing_time_ms: 0,
            metadata: HashMap::new(),
        };
        let j2 = serde_json::to_string(&resp).unwrap();
        let _: ZeroCostApiResponse<serde_json::Value> = serde_json::from_str(&j2).unwrap();
    }

    #[test]
    fn api_status_error_roundtrip() {
        let e = ApiStatus::Error {
            code: "E1".into(),
            message: "m".into(),
        };
        let s = serde_json::to_string(&e).unwrap();
        let back: ApiStatus = serde_json::from_str(&s).unwrap();
        match back {
            ApiStatus::Error { code, message } => {
                assert_eq!(code, "E1");
                assert_eq!(message, "m");
            }
            _ => panic!("expected error variant"),
        }
    }

    #[test]
    fn zero_cost_router_builder_bounds() {
        let b = ZeroCostRouterBuilder::<3, 2>::default();
        assert_eq!(ZeroCostRouterBuilder::<3, 2>::max_routes(), 3);
        assert_eq!(ZeroCostRouterBuilder::<3, 2>::max_middleware(), 2);
        assert!(b.can_add_route());
        assert!(b.can_add_middleware());
    }

    #[test]
    fn migration_guide_and_benchmark_helpers() {
        assert!(!ApiHandlerMigrationGuide::migration_steps().is_empty());
        let (a, b, p) = ApiHandlerBenchmark::performance_comparison();
        assert!(p > 0.0);
        assert!(a > b);
    }

    #[test]
    fn dataset_type_and_info_serde() {
        let dt = DatasetType::Volume { size: 1024 };
        let j = serde_json::to_string(&dt).unwrap();
        let _: DatasetType = serde_json::from_str(&j).unwrap();
    }
}
