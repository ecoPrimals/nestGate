// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    dead_code,
    missing_docs,
    unused_imports,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::float_cmp,
    clippy::uninlined_format_args,
    clippy::redundant_clone,
    clippy::needless_collect,
    clippy::unnecessary_wraps,
    clippy::doc_markdown,
    clippy::semicolon_if_nothing_returned
)]

//! Performance benchmarks for `nestgate-zfs`.
//!
//! ```bash
//! cargo bench -p nestgate-zfs --features benchmark
//! ```

#[cfg(feature = "benchmark")]
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
#[cfg(feature = "benchmark")]
use nestgate_zfs::performance::CurrentPerformanceMetrics;
#[cfg(feature = "benchmark")]
use nestgate_zfs::performance::TierMetrics;
#[cfg(feature = "benchmark")]
use nestgate_zfs::pool_setup::config::{StorageTier as SetupStorageTier, ZfsConfig};
#[cfg(feature = "benchmark")]
use nestgate_zfs::types::StorageTier;
#[cfg(feature = "benchmark")]
use std::time::Duration;

#[cfg(feature = "benchmark")]
#[derive(Debug, Clone)]
enum OptimizationComplexity {
    Low,
    Medium,
    High,
}

#[cfg(feature = "benchmark")]
#[derive(Debug, Clone)]
enum OptimizationType {
    TierMigration,
    Compression,
    Deduplication,
    Caching,
}

#[cfg(feature = "benchmark")]
#[derive(Debug, Clone)]
struct OptimizationOpportunity {
    optimization_type: OptimizationType,
    description: String,
    expected_impact: f64,
    confidence: f64,
    complexity: OptimizationComplexity,
    implementation_time: Duration,
}

#[cfg(feature = "benchmark")]
fn create_test_opportunities(count: usize) -> Vec<OptimizationOpportunity> {
    (0..count)
        .map(|i| OptimizationOpportunity {
            optimization_type: OptimizationType::TierMigration,
            description: format!("Optimization {i}"),
            expected_impact: (f64::from(i as u32) * 3.7) % 100.0,
            confidence: 0.5 + (f64::from(i as u32) * 0.1) % 0.5,
            complexity: OptimizationComplexity::Medium,
            implementation_time: Duration::from_secs(60 + (i as u64 * 13) % 300),
        })
        .collect()
}

#[cfg(feature = "benchmark")]
fn bench_config_creation(c: &mut Criterion) {
    c.bench_function("config_creation", |b| {
        b.iter(|| black_box(ZfsConfig::default()))
    });
}

#[cfg(feature = "benchmark")]
fn bench_config_validation(c: &mut Criterion) {
    let config = ZfsConfig::default();
    c.bench_function("config_validation", |b| {
        b.iter(|| black_box(config.clone()))
    });
}

#[cfg(feature = "benchmark")]
fn bench_tier_config_access(c: &mut Criterion) {
    let config = ZfsConfig::default();
    let tiers = [
        SetupStorageTier::Hot,
        SetupStorageTier::Warm,
        SetupStorageTier::Cold,
    ];
    c.bench_function("tier_config_access", |b| {
        b.iter(|| {
            for tier in &tiers {
                black_box(config.tier_properties.get(tier));
            }
        })
    });
}

#[cfg(feature = "benchmark")]
fn bench_performance_metrics(c: &mut Criterion) {
    c.bench_function("performance_metrics_creation", |b| {
        b.iter(|| black_box(CurrentPerformanceMetrics::default()))
    });
}

#[cfg(feature = "benchmark")]
fn bench_tier_metrics_generation(c: &mut Criterion) {
    let tiers = [
        StorageTier::Hot,
        StorageTier::Warm,
        StorageTier::Cold,
        StorageTier::Cache,
    ];
    for tier in &tiers {
        c.bench_with_input(
            BenchmarkId::new("tier_metrics_generation", format!("{tier:?}")),
            tier,
            |b, tier| b.iter(|| black_box(TierMetrics::default_for_tier(tier.clone()))),
        );
    }
}

#[cfg(feature = "benchmark")]
fn bench_ai_optimization_sorting(c: &mut Criterion) {
    let mut group = c.benchmark_group("ai_optimization");
    for size in [10_usize, 100, 1000] {
        group.bench_with_input(
            BenchmarkId::new("opportunity_sorting", size),
            &size,
            |b, &size| {
                let opportunities = create_test_opportunities(size);
                b.iter(|| {
                    let mut ops = opportunities.clone();
                    ops.sort_by(|a, b| {
                        b.expected_impact
                            .partial_cmp(&a.expected_impact)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                    black_box(ops)
                })
            },
        );
    }
    group.finish();
}

#[cfg(feature = "benchmark")]
criterion_group!(
    benches,
    bench_config_creation,
    bench_config_validation,
    bench_tier_config_access,
    bench_performance_metrics,
    bench_tier_metrics_generation,
    bench_ai_optimization_sorting
);

#[cfg(feature = "benchmark")]
criterion_main!(benches);

#[cfg(not(feature = "benchmark"))]
fn main() {
    eprintln!("Enable the `benchmark` feature to run Criterion benches, e.g.:");
    eprintln!("  cargo bench -p nestgate-zfs --features benchmark");
}
