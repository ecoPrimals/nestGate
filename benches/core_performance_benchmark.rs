use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::Arc;

// Core performance benchmarks that don't depend on problematic crates
// Focus on the core improvements we've implemented

/// Benchmark error context addition performance
fn benchmark_error_context(c: &mut Criterion) {
    use std::collections::HashMap;

    c.bench_function("error_context_addition", |b| {
        b.iter(|| {
            let mut context_map = HashMap::new();
            context_map.insert("operation".to_string(), "benchmark_test".to_string());
            context_map.insert(
                "component".to_string(),
                "performance_validation".to_string(),
            );
            context_map.insert("timestamp".to_string(), "2025-01-30".to_string());
            black_box(context_map);
        })
    });
}

/// Benchmark configuration unification patterns
fn benchmark_config_patterns(c: &mut Criterion) {
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    struct UnifiedConfig {
        system: SystemConfig,
        network: NetworkConfig,
        storage: StorageConfig,
        security: SecurityConfig,
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    struct SystemConfig {
        service_name: String,
        environment: String,
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    struct NetworkConfig {
        host: String,
        port: u16,
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    struct StorageConfig {
        base_path: String,
        cache_size_mb: u64,
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    struct SecurityConfig {
        enabled: bool,
        auth_timeout_secs: u64,
    }

    c.bench_function("unified_config_creation", |b| {
        b.iter(|| {
            let config = UnifiedConfig {
                system: SystemConfig {
                    service_name: "nestgate".to_string(),
                    environment: "production".to_string(),
                },
                network: NetworkConfig {
                    host: "0.0.0.0".to_string(),
                    port: 8080,
                },
                storage: StorageConfig {
                    base_path: "/var/lib/nestgate".to_string(),
                    cache_size_mb: 1024,
                },
                security: SecurityConfig {
                    enabled: true,
                    auth_timeout_secs: 3600,
                },
            };
            black_box(config);
        })
    });

    c.bench_function("fragmented_config_creation", |b| {
        b.iter(|| {
            let system = Arc::new(SystemConfig {
                service_name: "nestgate".to_string(),
                environment: "production".to_string(),
            });
            let network = Arc::new(NetworkConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
            });
            let storage = Arc::new(StorageConfig {
                base_path: "/var/lib/nestgate".to_string(),
                cache_size_mb: 1024,
            });
            let security = Arc::new(SecurityConfig {
                enabled: true,
                auth_timeout_secs: 3600,
            });
            black_box((system, network, storage, security));
        })
    });
}

/// Benchmark zero-cost trait patterns vs boxed futures
fn benchmark_trait_overhead(c: &mut Criterion) {
    use std::future::Future;
    use std::pin::Pin;

    // Zero-cost native async trait
    trait ModernAsyncTrait {
        fn process(&self, data: Vec<u8>) -> impl Future<Output = Vec<u8>> + Send;
    }

    struct ModernImpl;

    impl ModernAsyncTrait for ModernImpl {
        async fn process(&self, data: Vec<u8>) -> Vec<u8> {
            let mut result = data;
            result.reverse();
            result
        }
    }

    // Legacy boxed future pattern (simulating async_trait overhead)
    trait LegacyAsyncTrait {
        fn process(&self, data: Vec<u8>) -> Pin<Box<dyn Future<Output = Vec<u8>> + Send>>;
    }

    struct LegacyImpl;

    impl LegacyAsyncTrait for LegacyImpl {
        fn process(&self, data: Vec<u8>) -> Pin<Box<dyn Future<Output = Vec<u8>> + Send>> {
            Box::pin(async move {
                let mut result = data;
                result.reverse();
                result
            })
        }
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("zero_cost_async", |b| {
        let service = ModernImpl;
        b.iter(|| {
            rt.block_on(async {
                let data = vec![1u8; 1024];
                let result = service.process(data).await;
                black_box(result);
            })
        })
    });

    c.bench_function("boxed_future_async", |b| {
        let service = LegacyImpl;
        b.iter(|| {
            rt.block_on(async {
                let data = vec![1u8; 1024];
                let result = service.process(data).await;
                black_box(result);
            })
        })
    });
}

/// Benchmark memory layout improvements
fn benchmark_memory_layout(c: &mut Criterion) {
    // Unified data structure with better cache locality
    #[derive(Debug)]
    struct UnifiedData {
        values: Vec<(String, String)>,
    }

    impl UnifiedData {
        fn new() -> Self {
            Self {
                values: vec![
                    ("system".to_string(), "value1".to_string()),
                    ("network".to_string(), "value2".to_string()),
                    ("storage".to_string(), "value3".to_string()),
                    ("security".to_string(), "value4".to_string()),
                ],
            }
        }

        fn get(&self, key: &str) -> Option<&String> {
            self.values.iter().find(|(k, _)| k == key).map(|(_, v)| v)
        }
    }

    // Fragmented data structure with poor cache locality
    #[derive(Debug)]
    struct FragmentedData {
        system: Arc<Vec<(String, String)>>,
        network: Arc<Vec<(String, String)>>,
        storage: Arc<Vec<(String, String)>>,
        security: Arc<Vec<(String, String)>>,
    }

    impl FragmentedData {
        fn new() -> Self {
            Self {
                system: Arc::new(vec![("system".to_string(), "value1".to_string())]),
                network: Arc::new(vec![("network".to_string(), "value2".to_string())]),
                storage: Arc::new(vec![("storage".to_string(), "value3".to_string())]),
                security: Arc::new(vec![("security".to_string(), "value4".to_string())]),
            }
        }

        fn get(&self, key: &str) -> Option<String> {
            match key {
                "system" => self.system.first().map(|(_, v)| v.clone()),
                "network" => self.network.first().map(|(_, v)| v.clone()),
                "storage" => self.storage.first().map(|(_, v)| v.clone()),
                "security" => self.security.first().map(|(_, v)| v.clone()),
                _ => None,
            }
        }
    }

    c.bench_function("unified_data_access", |b| {
        let data = UnifiedData::new();
        b.iter(|| {
            let result = data.get("system");
            black_box(result);
        })
    });

    c.bench_function("fragmented_data_access", |b| {
        let data = FragmentedData::new();
        b.iter(|| {
            let result = data.get("system");
            black_box(result);
        })
    });
}

/// Benchmark const generics vs runtime dispatch
fn benchmark_const_generics(c: &mut Criterion) {
    // Const generic version - compile-time optimization
    fn process_buffer_const<const SIZE: usize>(buffer: &[u8; SIZE]) -> usize {
        buffer.iter().map(|&b| b as usize).sum()
    }

    // Runtime version - dynamic dispatch
    fn process_buffer_runtime(buffer: &[u8]) -> usize {
        buffer.iter().map(|&b| b as usize).sum()
    }

    c.bench_function("const_generic_processing", |b| {
        let buffer: [u8; 1024] = [1; 1024];
        b.iter(|| {
            let result = process_buffer_const(&buffer);
            black_box(result);
        })
    });

    c.bench_function("runtime_processing", |b| {
        let buffer = vec![1u8; 1024];
        b.iter(|| {
            let result = process_buffer_runtime(&buffer);
            black_box(result);
        })
    });
}

/// Benchmark string processing efficiency
fn benchmark_string_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_processing");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("unified_string_ops", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut result = String::with_capacity(size);
                    for i in 0..size {
                        result.push_str(&format!("item_{}", i));
                    }
                    black_box(result);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("fragmented_string_ops", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut parts = Vec::new();
                    for i in 0..size {
                        parts.push(format!("item_{}", i));
                    }
                    let result = parts.join("");
                    black_box(result);
                })
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    benchmark_error_context,
    benchmark_config_patterns,
    benchmark_trait_overhead,
    benchmark_memory_layout,
    benchmark_const_generics,
    benchmark_string_processing
);

criterion_main!(benches);
