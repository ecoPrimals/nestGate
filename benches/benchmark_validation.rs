use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Validation benchmark to test if our benchmarks are doing real work
/// This will help us identify if compiler optimizations are eliminating work

fn validate_real_work_vs_stub(c: &mut Criterion) {
    // Test 1: Real work that should take measurable time
    c.bench_function("real_work_validation", |b| {
        b.iter(|| {
            // Real computation that can't be optimized away
            let mut sum = 0u64;
            for i in 0..1000 {
                sum += black_box(i * i); // Prevent optimization
            }
            // Access memory to force real work
            let mut data = vec![0u8; 1000];
            for (i, byte) in data.iter_mut().enumerate() {
                *byte = (sum + i as u64) as u8;
            }
            black_box(sum + data.len() as u64)
        })
    });

    // Test 2: Stub work that might get optimized away (bad benchmark)
    c.bench_function("potential_stub_validation", |b| {
        b.iter(|| {
            // This might get optimized away if not careful
            let value = 42;
            black_box(value)
        })
    });

    // Test 3: Memory allocation validation
    c.bench_function("memory_allocation_validation", |b| {
        b.iter(|| {
            // Real memory allocation work
            let data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
            // Force use of the data
            let checksum: u32 = data.iter().map(|&x| x as u32).sum();
            black_box((data.len(), checksum))
        })
    });

    // Test 4: String processing validation
    c.bench_function("string_processing_validation", |b| {
        let test_strings = ["hello", "world", "test", "benchmark", "validation"];
        b.iter(|| {
            let results: Vec<String> = test_strings
                .iter()
                .map(|s| format!("processed_{}_end", black_box(s)))
                .collect();

            // Force evaluation
            let total_len: usize = results.iter().map(|s| s.len()).sum();
            black_box((results, total_len))
        })
    });

    // Test 5: Arc vs Clone validation with real field access
    c.bench_function("arc_clone_real_work", |b| {
        #[derive(Clone)]
        struct TestData {
            id: String,
            values: Vec<i32>,
            metadata: std::collections::HashMap<String, String>,
        }

        impl TestData {
            fn compute_signature(&self) -> u64 {
                let id_hash = self.id.len() as u64;
                let values_sum: i32 = self.values.iter().sum();
                let metadata_count = self.metadata.len() as u64;
                id_hash + values_sum as u64 + metadata_count
            }
        }

        let test_data = TestData {
            id: "test_service_12345".to_string(),
            values: (0..100).collect(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                for i in 0..10 {
                    map.insert(format!("key_{i}"), format!("value_{i}"));
                }
                map
            },
        };

        b.iter(|| {
            // Clone the data multiple times and actually use all fields
            let copies: Vec<_> = (0..10).map(|_| test_data.clone()).collect();

            // Force computation using all fields
            let total_signature: u64 = copies.iter().map(|d| d.compute_signature()).sum();
            black_box(total_signature)
        })
    });
}

/// Test to verify if our UUID cache is actually faster or just measuring overhead
fn validate_uuid_cache_if_exists(c: &mut Criterion) {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use uuid::Uuid;

    // Mock UUID cache implementation to test the concept
    struct MockUuidCache {
        cache: Arc<Mutex<HashMap<String, Arc<Uuid>>>>,
    }

    impl MockUuidCache {
        fn new() -> Self {
            Self {
                cache: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn get_or_create(&self, key: &str) -> Arc<Uuid> {
            let mut cache = nestgate_core::safe_operations::safe_mutex_lock(&self.cache)?;
            cache
                .entry(key.to_string())
                .or_insert_with(|| Arc::new(Uuid::new_v4()))
                .clone()
        }
    }

    let cache = MockUuidCache::new();

    // Pre-warm the cache
    for i in 0..10 {
        let _ = cache.get_or_create(&format!("service_{i}"));
    }

    c.bench_function("uuid_cache_hit_validation", |b| {
        let mut counter = 0;
        b.iter(|| {
            counter += 1;
            let key = format!("service_{}", counter % 10); // Should hit cache
            let uuid = cache.get_or_create(&key);
            // Actually use the UUID to prevent optimization
            black_box(uuid.to_string().len())
        })
    });

    c.bench_function("uuid_fresh_generation", |b| {
        b.iter(|| {
            let uuid = Uuid::new_v4();
            // Actually use the UUID to prevent optimization
            black_box(uuid.to_string().len())
        })
    });
}

/// Validate memory pool concept vs regular allocation
fn validate_memory_pool_concept(c: &mut Criterion) {
    use std::sync::{Arc, Mutex};

    // Mock memory pool to test the concept
    struct MockMemoryPool {
        buffers: Arc<Mutex<Vec<Vec<u8>>>>,
        size: usize,
    }

    impl MockMemoryPool {
        fn new(size: usize, pre_allocate: usize) -> Self {
            let mut buffers = Vec::new();
            for _ in 0..pre_allocate {
                buffers.push(Vec::with_capacity(size));
            }

            Self {
                buffers: Arc::new(Mutex::new(buffers)),
                size,
            }
        }

        fn get_buffer(&self) -> Vec<u8> {
            let mut buffers = nestgate_core::safe_operations::safe_mutex_lock(&self.buffers)?;
            if let Some(mut buf) = buffers.pop() {
                buf.clear();
                buf.reserve(self.size);
                buf
            } else {
                Vec::with_capacity(self.size)
            }
        }

        fn return_buffer(&self, buffer: Vec<u8>) {
            let mut buffers = nestgate_core::safe_operations::safe_mutex_lock(&self.buffers)?;
            if buffers.len() < 10 {
                // Limit pool size
                buffers.push(buffer);
            }
        }
    }

    let pool = MockMemoryPool::new(4096, 5);

    c.bench_function("memory_pool_allocation", |b| {
        b.iter(|| {
            let mut buffer = pool.get_buffer();
            // Do real work with the buffer
            for i in 0..100 {
                buffer.push((i % 256) as u8);
            }
            let checksum: u32 = buffer.iter().map(|&x| x as u32).sum();
            pool.return_buffer(buffer);
            black_box(checksum)
        })
    });

    c.bench_function("regular_allocation", |b| {
        b.iter(|| {
            let mut buffer = Vec::with_capacity(4096);
            // Do the same real work
            for i in 0..100 {
                buffer.push((i % 256) as u8);
            }
            let checksum: u32 = buffer.iter().map(|&x| x as u32).sum();
            black_box(checksum)
        })
    });
}

criterion_group!(
    validation_benches,
    validate_real_work_vs_stub,
    validate_uuid_cache_if_exists,
    validate_memory_pool_concept
);
criterion_main!(validation_benches);
