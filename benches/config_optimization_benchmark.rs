// SPDX-License-Identifier: AGPL-3.0-or-later
#![expect(
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

//! Configuration Optimization Benchmarks
//!
//! Benchmarks to demonstrate the performance improvement from Config → Arc optimization.

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::sync::Arc;

// Mock config structure similar to ZfsServiceConfig
#[derive(Debug, Clone)]
#[expect(dead_code)]
struct MockConfig {
    service_name: String,
    bind_address: String,
    port: u16,
    orchestrator_endpoints: Vec<String>,
    health_check_interval: u64,
    capabilities: Vec<String>,
    metadata: std::collections::HashMap<String, String>,
}

impl Default for MockConfig {
    fn default() -> Self {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("version".to_string(), "1.0.0".to_string());
        metadata.insert("region".to_string(), "us-west-2".to_string());

        Self {
            service_name: "test-service".to_string(),
            bind_address: "0.0.0.0".to_string(),
            port: 8080,
            orchestrator_endpoints: vec![
                "http://orchestrator1:9090".to_string(),
                "http://orchestrator2:9090".to_string(),
                "http://orchestrator3:9090".to_string(),
            ],
            health_check_interval: 30,
            capabilities: vec![
                "storage".to_string(),
                "snapshot".to_string(),
                "replication".to_string(),
                "tier-management".to_string(),
            ],
            metadata,
        }
    }
}

// Service using direct clone (old way)
#[derive(Clone)]
#[expect(dead_code)]
struct ServiceWithClone {
    config: MockConfig,
    id: String,
}

impl ServiceWithClone {
    fn new(config: MockConfig) -> Self {
        Self {
            config,
            id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

// Service using Arc (optimized way)
#[derive(Clone)]
#[expect(dead_code)]
struct ServiceWithArc {
    config: Arc<MockConfig>,
    id: String,
}

impl ServiceWithArc {
    fn new(config: MockConfig) -> Self {
        Self {
            config: Arc::new(config),
            id: uuid::Uuid::new_v4().to_string(),
        }
    }

    fn from_arc(config: Arc<MockConfig>) -> Self {
        Self {
            config,
            id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

fn benchmark_config_clone(c: &mut Criterion) {
    let config = MockConfig::default();

    c.bench_function("config_direct_clone", |b| {
        b.iter(|| {
            let service = ServiceWithClone::new(black_box(config.clone()));
            black_box(service);
        })
    });
}

fn benchmark_config_arc(c: &mut Criterion) {
    let config = MockConfig::default();

    c.bench_function("config_arc_new", |b| {
        b.iter(|| {
            let service = ServiceWithArc::new(black_box(config.clone()));
            black_box(service);
        })
    });
}

fn benchmark_config_arc_from_arc(c: &mut Criterion) {
    let config = Arc::new(MockConfig::default());

    c.bench_function("config_arc_from_arc", |b| {
        b.iter(|| {
            let service = ServiceWithArc::from_arc(black_box(Arc::clone(&config)));
            black_box(service);
        })
    });
}

fn benchmark_multiple_services(c: &mut Criterion) {
    let config = MockConfig::default();

    c.bench_function("10_services_direct_clone", |b| {
        b.iter(|| {
            let services: Vec<_> = (0..10)
                .map(|_| ServiceWithClone::new(black_box(config.clone())))
                .collect();
            black_box(services);
        })
    });

    let config_arc = Arc::new(MockConfig::default());

    c.bench_function("10_services_arc", |b| {
        b.iter(|| {
            let services: Vec<_> = (0..10)
                .map(|_| ServiceWithArc::from_arc(black_box(Arc::clone(&config_arc))))
                .collect();
            black_box(services);
        })
    });
}

fn benchmark_service_clone(c: &mut Criterion) {
    let config = MockConfig::default();
    let service_clone = ServiceWithClone::new(config.clone());

    c.bench_function("service_clone_direct", |b| {
        b.iter(|| {
            let cloned = black_box(service_clone.clone());
            black_box(cloned);
        })
    });

    let config_arc = Arc::new(MockConfig::default());
    let service_arc = ServiceWithArc::from_arc(config_arc);

    c.bench_function("service_clone_arc", |b| {
        b.iter(|| {
            let cloned = black_box(service_arc.clone());
            black_box(cloned);
        })
    });
}

criterion_group!(
    benches,
    benchmark_config_clone,
    benchmark_config_arc,
    benchmark_config_arc_from_arc,
    benchmark_multiple_services,
    benchmark_service_clone
);

criterion_main!(benches);
