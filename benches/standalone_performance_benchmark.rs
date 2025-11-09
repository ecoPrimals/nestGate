use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;
use std::sync::Arc;

/// Standalone performance benchmarks demonstrating NestGate modernization patterns
/// These benchmarks validate the performance improvements without depending on
/// potentially problematic crates, focusing on the core architectural patterns.
/// Benchmark: Configuration Unification vs Fragmentation
/// Tests: Memory layout, cache locality, allocation efficiency
fn benchmark_configuration_patterns(c: &mut Criterion) {
    // Unified configuration pattern (NestGate modern approach)
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    struct UnifiedConfig {
        system_service_name: String,
        system_environment: String,
        network_host: String,
        network_port: u16,
        storage_base_path: String,
        storage_cache_size_mb: u64,
        security_enabled: bool,
        security_timeout_secs: u64,
    }

    impl UnifiedConfig {
        fn new() -> Self {
            Self {
                system_service_name: "nestgate".to_string(),
                system_environment: "production".to_string(),
                network_host: "0.0.0.0".to_string(),
                network_port: 8080,
                storage_base_path: "/var/lib/nestgate".to_string(),
                storage_cache_size_mb: 1024,
                security_enabled: true,
                security_timeout_secs: 3600,
            }
        }

        fn get_service_name(&self) -> &str {
            &self.system_service_name
        }

        fn get_port(&self) -> u16 {
            self.network_port
        }
    }

    // Fragmented configuration pattern (legacy approach)
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    struct FragmentedConfigs {
        system: Arc<HashMap<String, String>>,
        network: Arc<HashMap<String, String>>,
        storage: Arc<HashMap<String, String>>,
        security: Arc<HashMap<String, String>>,
    }

    impl FragmentedConfigs {
        fn new() -> Self {
            let mut system = HashMap::new();
            system.insert("service_name".to_string(), "nestgate".to_string());
            system.insert("environment".to_string(), "production".to_string());

            let mut network = HashMap::new();
            network.insert("host".to_string(), "0.0.0.0".to_string());
            network.insert("port".to_string(), "8080".to_string());

            let mut storage = HashMap::new();
            storage.insert("base_path".to_string(), "/var/lib/nestgate".to_string());
            storage.insert("cache_size_mb".to_string(), "1024".to_string());

            let mut security = HashMap::new();
            security.insert("enabled".to_string(), "true".to_string());
            security.insert("timeout_secs".to_string(), "3600".to_string());

            Self {
                system: Arc::new(system),
                network: Arc::new(network),
                storage: Arc::new(storage),
                security: Arc::new(security),
            }
        }

        fn get_service_name(&self) -> Option<&String> {
            self.system.get("service_name")
        }

        fn get_port(&self) -> Option<u16> {
            self.network.get("port")?.parse().ok()
        }
    }

    c.bench_function("unified_config_creation", |b| {
        b.iter(|| {
            let config = UnifiedConfig::new();
            black_box(config);
        })
    });

    c.bench_function("fragmented_config_creation", |b| {
        b.iter(|| {
            let config = FragmentedConfigs::new();
            black_box(config);
        })
    });

    c.bench_function("unified_config_access", |b| {
        let config = UnifiedConfig::new();
        b.iter(|| {
            let service_name = config.get_service_name();
            let port = config.get_port();
            black_box((service_name, port));
        })
    });

    c.bench_function("fragmented_config_access", |b| {
        let config = FragmentedConfigs::new();
        b.iter(|| {
            let service_name = config.get_service_name();
            let port = config.get_port();
            black_box((service_name, port));
        })
    });
}

/// Benchmark: Error Handling Patterns
/// Tests: Context addition, error creation, memory efficiency
fn benchmark_error_patterns(c: &mut Criterion) {
    // Modern unified error pattern
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    struct UnifiedError {
        message: String,
        field: Option<String>,
        context: HashMap<String, String>,
    }

    impl UnifiedError {
        fn new(message: &str) -> Self {
            Self {
                message: message.to_string(),
                field: None,
                context: HashMap::new(),
            }
        }

        fn with_field(mut self, field: &str) -> Self {
            self.field = Some(field.to_string());
            self
        }

        fn add_context(&mut self, key: &str, value: &str) {
            self.context.insert(key.to_string(), value.to_string());
        }
    }

    // Legacy fragmented error pattern
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    enum FragmentedError {
        Validation {
            message: String,
            field: Option<String>,
        },
        Network {
            message: String,
            endpoint: Option<String>,
        },
        Storage {
            message: String,
            path: Option<String>,
        },
        Security {
            message: String,
            resource: Option<String>,
        },
    }

    c.bench_function("unified_error_creation", |b| {
        b.iter(|| {
            let mut error = UnifiedError::new("Test validation error").with_field("test_field");
            error.add_context("operation", "benchmark_test");
            error.add_context("component", "performance_validation");
            error.add_context("timestamp", "2025-01-30");
            black_box(error);
        })
    });

    c.bench_function("fragmented_error_creation", |b| {
        b.iter(|| {
            let errors = vec![
                FragmentedError::Validation {
                    message: "Test validation error".to_string(),
                    field: Some("test_field".to_string()),
                },
                FragmentedError::Network {
                    message: "Network error".to_string(),
                    endpoint: Some("localhost:8080".to_string()),
                },
                FragmentedError::Storage {
                    message: "Storage error".to_string(),
                    path: Some("/tmp/test".to_string()),
                },
            ];
            black_box(errors);
        })
    });
}

/// Benchmark: Zero-Cost Async Patterns vs Boxed Futures
/// Tests: Runtime overhead, memory allocation, CPU efficiency
fn benchmark_async_patterns(c: &mut Criterion) {
    use std::future::Future;
    use std::pin::Pin;

    // Modern zero-cost async trait pattern
    trait ModernAsyncService {
        fn process_data(&self, data: Vec<u8>) -> impl Future<Output = Vec<u8>> + Send;
    }

    struct ModernService;

    impl ModernAsyncService for ModernService {
        async fn process_data(&self, data: Vec<u8>) -> Vec<u8> {
            // Simulate processing
            let mut result = data;
            result.reverse();
            result.extend_from_slice(b"_processed");
            result
        }
    }

    // Legacy async_trait pattern (with boxing overhead)
    trait LegacyAsyncService {
        fn process_data(&self, data: Vec<u8>) -> Pin<Box<dyn Future<Output = Vec<u8>> + Send>>;
    }

    struct LegacyService;

    impl LegacyAsyncService for LegacyService {
        fn process_data(&self, data: Vec<u8>) -> Pin<Box<dyn Future<Output = Vec<u8>> + Send>> {
            Box::pin(async move {
                // Simulate processing
                let mut result = data;
                result.reverse();
                result.extend_from_slice(b"_processed");
                result
            })
        }
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("zero_cost_async_processing", |b| {
        let service = ModernService;
        b.iter(|| {
            rt.block_on(async {
                let data = vec![1u8; 1024];
                let result = service.process_data(data).await;
                black_box(result);
            })
        })
    });

    c.bench_function("boxed_future_processing", |b| {
        let service = LegacyService;
        b.iter(|| {
            rt.block_on(async {
                let data = vec![1u8; 1024];
                let result = service.process_data(data).await;
                black_box(result);
            })
        })
    });
}

/// Benchmark: Memory Layout and Cache Locality
/// Tests: Data structure efficiency, access patterns, cache performance
fn benchmark_memory_patterns(c: &mut Criterion) {
    // Unified data structure with optimal cache locality
    #[derive(Debug)]
    struct UnifiedDataStore {
        // All data in contiguous memory for better cache locality
        entries: Vec<(String, String, u64)>, // key, value, timestamp
    }

    impl UnifiedDataStore {
        fn new() -> Self {
            let mut entries = Vec::with_capacity(1000);
            for i in 0..1000 {
                entries.push((format!("key_{}", i), format!("value_{}", i), i as u64));
            }
            Self { entries }
        }

        fn get(&self, key: &str) -> Option<&String> {
            self.entries
                .iter()
                .find(|(k, _, _)| k == key)
                .map(|(_, v, _)| v)
        }

        fn update(&mut self, key: &str, value: String) {
            if let Some(entry) = self.entries.iter_mut().find(|(k, _, _)| k == key) {
                entry.1 = value;
                entry.2 += 1; // increment timestamp
            }
        }
    }

    // Fragmented data structure with poor cache locality
    #[allow(dead_code)]
    #[derive(Debug)]
    struct FragmentedDataStore {
        // Data scattered across multiple allocations
        keys: Arc<Vec<String>>,
        values: Arc<HashMap<String, Arc<String>>>,
        timestamps: Arc<HashMap<String, Arc<u64>>>,
    }

    impl FragmentedDataStore {
        fn new() -> Self {
            let keys: Vec<String> = (0..1000).map(|i| format!("key_{}", i)).collect();
            let mut values = HashMap::new();
            let mut timestamps = HashMap::new();

            for i in 0..1000 {
                let key = format!("key_{}", i);
                values.insert(key.clone(), Arc::new(format!("value_{}", i)));
                timestamps.insert(key, Arc::new(i as u64));
            }

            Self {
                keys: Arc::new(keys),
                values: Arc::new(values),
                timestamps: Arc::new(timestamps),
            }
        }

        fn get(&self, key: &str) -> Option<Arc<String>> {
            self.values.get(key).cloned()
        }
    }

    c.bench_function("unified_data_access", |b| {
        let store = UnifiedDataStore::new();
        b.iter(|| {
            let result = store.get("key_500");
            black_box(result);
        })
    });

    c.bench_function("fragmented_data_access", |b| {
        let store = FragmentedDataStore::new();
        b.iter(|| {
            let result = store.get("key_500");
            black_box(result);
        })
    });

    c.bench_function("unified_data_update", |b| {
        let mut store = UnifiedDataStore::new();
        b.iter(|| {
            store.update("key_500", "new_value".to_string());
            black_box(&store);
        })
    });
}

/// Benchmark: Const Generics vs Runtime Dispatch
/// Tests: Compile-time optimization, code specialization, performance
fn benchmark_const_generic_patterns(c: &mut Criterion) {
    // Const generic version - compile-time specialization
    fn process_buffer_const<const SIZE: usize>(buffer: &[u8; SIZE]) -> u64 {
        let mut hash = 0u64;
        for &byte in buffer.iter() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }

    // Runtime version - dynamic dispatch
    fn process_buffer_runtime(buffer: &[u8], size: usize) -> u64 {
        assert_eq!(buffer.len(), size);
        let mut hash = 0u64;
        for &byte in buffer.iter() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }

    let mut group = c.benchmark_group("const_generics");

    for size in [64, 256, 1024, 4096].iter() {
        group.bench_with_input(
            BenchmarkId::new("const_generic", size),
            size,
            |b, &size| match size {
                64 => {
                    let buffer: [u8; 64] = [1; 64];
                    b.iter(|| {
                        let result = process_buffer_const(&buffer);
                        black_box(result);
                    });
                }
                256 => {
                    let buffer: [u8; 256] = [1; 256];
                    b.iter(|| {
                        let result = process_buffer_const(&buffer);
                        black_box(result);
                    });
                }
                1024 => {
                    let buffer: [u8; 1024] = [1; 1024];
                    b.iter(|| {
                        let result = process_buffer_const(&buffer);
                        black_box(result);
                    });
                }
                4096 => {
                    let buffer: [u8; 4096] = [1; 4096];
                    b.iter(|| {
                        let result = process_buffer_const(&buffer);
                        black_box(result);
                    });
                }
                _ => unreachable!(),
            },
        );

        group.bench_with_input(
            BenchmarkId::new("runtime_dispatch", size),
            size,
            |b, &size| {
                let buffer = vec![1u8; size];
                b.iter(|| {
                    let result = process_buffer_runtime(&buffer, size);
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark: Build System and Compilation Efficiency
/// Tests: Compilation patterns, type system efficiency
fn benchmark_compilation_patterns(c: &mut Criterion) {
    // Unified type system approach
    trait UnifiedService<T> {
        type Output;
        fn process(&self, input: T) -> Self::Output;
    }

    struct StringProcessor;

    impl UnifiedService<String> for StringProcessor {
        type Output = String;

        fn process(&self, input: String) -> Self::Output {
            format!("processed: {}", input)
        }
    }

    impl UnifiedService<Vec<u8>> for StringProcessor {
        type Output = Vec<u8>;

        fn process(&self, mut input: Vec<u8>) -> Self::Output {
            input.extend_from_slice(b"_processed");
            input
        }
    }

    // Fragmented type system approach
    trait StringService {
        fn process_string(&self, input: String) -> String;
    }

    trait ByteService {
        fn process_bytes(&self, input: Vec<u8>) -> Vec<u8>;
    }

    struct FragmentedProcessor;

    impl StringService for FragmentedProcessor {
        fn process_string(&self, input: String) -> String {
            format!("processed: {}", input)
        }
    }

    impl ByteService for FragmentedProcessor {
        fn process_bytes(&self, mut input: Vec<u8>) -> Vec<u8> {
            input.extend_from_slice(b"_processed");
            input
        }
    }

    c.bench_function("unified_type_processing", |b| {
        let processor = StringProcessor;
        b.iter(|| {
            let string_result = processor.process("test".to_string());
            let bytes_result = processor.process(vec![1, 2, 3, 4]);
            black_box((string_result, bytes_result));
        })
    });

    c.bench_function("fragmented_type_processing", |b| {
        let processor = FragmentedProcessor;
        b.iter(|| {
            let string_result = processor.process_string("test".to_string());
            let bytes_result = processor.process_bytes(vec![1, 2, 3, 4]);
            black_box((string_result, bytes_result));
        })
    });
}

criterion_group!(
    benches,
    benchmark_configuration_patterns,
    benchmark_error_patterns,
    benchmark_async_patterns,
    benchmark_memory_patterns,
    benchmark_const_generic_patterns,
    benchmark_compilation_patterns
);

criterion_main!(benches);
