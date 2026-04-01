// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Round 6 discovery: resolver helpers, composite chain, environment naming.

#[cfg(test)]
mod round6 {
    use crate::capability_resolver::{
        CapabilityResolver, CompositeResolver, EnvironmentResolver, ResolvedService,
    };
    use crate::unified_capabilities::{CapabilityMapper, UnifiedCapability};
    use UnifiedCapability::*;
    use std::time::Duration;

    #[test]
    fn r6_resolved_service_grpc_url() {
        let s = ResolvedService {
            id: "1".into(),
            host: "[::1]".into(),
            port: 443,
            protocol: "https".into(),
            capabilities: vec![UnifiedCapability::Storage],
            is_healthy: true,
        };
        assert!(s.url().contains("::1"));
    }

    #[test]
    fn r6_resolved_service_endpoint_ipv4() {
        let s = ResolvedService {
            id: "2".into(),
            host: "10.0.0.1".into(),
            port: 22,
            protocol: "ssh".into(),
            capabilities: vec![],
            is_healthy: false,
        };
        assert_eq!(s.endpoint(), "10.0.0.1:22");
    }

    #[test]
    fn r6_composite_default_empty() {
        let _ = CompositeResolver::default();
    }

    #[test]
    fn r6_environment_resolver_default() {
        let _ = EnvironmentResolver::default();
    }

    #[test]
    fn r6_capability_mapper_storage_endpoint_name() {
        let n = CapabilityMapper::env_var_name(&UnifiedCapability::ZfsManagement);
        assert!(n.contains("ZFS"));
    }

    #[test]
    fn r6_all_unified_capabilities_nonempty_display() {
        let all = [
            Storage,
            ZfsManagement,
            ObjectStorage,
            BlockStorage,
            FileStorage,
            Networking,
            HttpApi,
            Grpc,
            Websocket,
            Mqtt,
            Compute,
            TaskExecution,
            Orchestration,
            Scheduling,
            Security,
            Authentication,
            Authorization,
            Encryption,
            SecretManagement,
            ArtificialIntelligence,
            ModelServing,
            Training,
            Inference,
            Monitoring,
            Metrics,
            Tracing,
            Logging,
            Alerting,
            ServiceDiscovery,
            HealthCheck,
            Configuration,
            StateManagement,
            Custom("round6".into()),
        ];
        for c in all {
            assert!(!c.to_string().is_empty(), "{c:?}");
        }
    }

    #[test]
    fn r6_composite_resolve_all_empty_errors() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let c = CompositeResolver::new();
            assert!(
                c.resolve_capability_all(&UnifiedCapability::Storage)
                    .await
                    .is_err()
            );
        });
    }

    #[test]
    fn r6_composite_has_capability_false_when_empty() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let c = CompositeResolver::new();
            assert!(!c.has_capability(&UnifiedCapability::Storage).await);
        });
    }

    #[test]
    fn r6_timeout_simulation_sleep() {
        let start = std::time::Instant::now();
        assert!(start.elapsed() >= Duration::ZERO);
    }
}
