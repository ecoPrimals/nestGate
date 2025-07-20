//! Decentralized Security Performance Benchmarks
//!
//! Performance benchmarks for security operations using stable Criterion framework

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

// Mock security context for benchmarking
#[derive(Clone)]
struct SecurityContext {
    user_id: String,
    permissions: Vec<String>,
    auth_token: String,
    expires_at: u64,
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self {
            user_id: "user123".to_string(),
            permissions: vec!["read".to_string(), "write".to_string()],
            auth_token: "mock_token".to_string(),
            expires_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                + 3600,
        }
    }
}

// Mock distributed security manager
struct DistributedSecurityManager {
    contexts: Arc<Mutex<HashMap<String, SecurityContext>>>,
}

impl DistributedSecurityManager {
    fn new() -> Self {
        Self {
            contexts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn authenticate(&self, token: &str) -> bool {
        let contexts = self.contexts.lock().unwrap();
        contexts.values().any(|ctx| ctx.auth_token == token)
    }

    fn authorize(&self, user_id: &str, permission: &str) -> bool {
        let contexts = self.contexts.lock().unwrap();
        if let Some(ctx) = contexts.get(user_id) {
            ctx.permissions.contains(&permission.to_string())
        } else {
            false
        }
    }

    fn add_context(&self, user_id: String, context: SecurityContext) {
        let mut contexts = self.contexts.lock().unwrap();
        contexts.insert(user_id, context);
    }
}

fn bench_authentication(c: &mut Criterion) {
    let manager = DistributedSecurityManager::new();

    // Setup test data
    for i in 0..100 {
        let context = SecurityContext {
            user_id: format!("user{i}"),
            auth_token: format!("token{i}"),
            ..Default::default()
        };
        manager.add_context(format!("user{i}"), context);
    }

    c.bench_function("authentication", |b| {
        b.iter(|| {
            for i in 0..100 {
                let token = format!("token{i}");
                black_box(manager.authenticate(&token));
            }
        })
    });
}

fn bench_authorization(c: &mut Criterion) {
    let manager = DistributedSecurityManager::new();

    // Setup test data
    for i in 0..100 {
        let context = SecurityContext {
            user_id: format!("user{i}"),
            permissions: vec![
                "read".to_string(),
                "write".to_string(),
                "execute".to_string(),
            ],
            ..Default::default()
        };
        manager.add_context(format!("user{i}"), context);
    }

    c.bench_function("authorization", |b| {
        b.iter(|| {
            for i in 0..100 {
                let user_id = format!("user{i}");
                black_box(manager.authorize(&user_id, "read"));
                black_box(manager.authorize(&user_id, "write"));
            }
        })
    });
}

fn bench_concurrent_access(c: &mut Criterion) {
    let manager = Arc::new(DistributedSecurityManager::new());

    // Setup test data
    for i in 0..50 {
        let context = SecurityContext {
            user_id: format!("user{i}"),
            auth_token: format!("token{i}"),
            ..Default::default()
        };
        manager.add_context(format!("user{i}"), context);
    }

    c.bench_function("concurrent_security_operations", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..10)
                .map(|thread_id| {
                    let manager_clone = Arc::clone(&manager);
                    std::thread::spawn(move || {
                        for i in 0..10 {
                            let user_id = format!("user{}", (thread_id * 10 + i) % 50);
                            let token = format!("token{}", (thread_id * 10 + i) % 50);
                            black_box(manager_clone.authenticate(&token));
                            black_box(manager_clone.authorize(&user_id, "read"));
                        }
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

fn bench_context_management(c: &mut Criterion) {
    c.bench_function("context_creation_and_storage", |b| {
        b.iter(|| {
            let manager = DistributedSecurityManager::new();
            for i in 0..1000 {
                let context = SecurityContext {
                    user_id: format!("user{i}"),
                    auth_token: format!("token{i}"),
                    permissions: vec!["read".to_string(), "write".to_string()],
                    expires_at: black_box(1700000000 + i as u64), // Use expires_at field
                };

                // Actually access all fields to eliminate dead code
                let total_field_size = context.user_id.len()
                    + context.auth_token.len()
                    + context.permissions.len()
                    + (context.expires_at as usize % 1000);
                black_box(total_field_size);

                manager.add_context(format!("user{i}"), context);
            }
            black_box(manager);
        })
    });
}

fn bench_token_validation(c: &mut Criterion) {
    let manager = DistributedSecurityManager::new();

    // Setup large dataset for realistic performance testing
    for i in 0..1000 {
        let context = SecurityContext {
            user_id: format!("user{i}"),
            auth_token: format!("very_long_secure_token_with_lots_of_entropy_{i}"),
            permissions: vec!["read".to_string(), "write".to_string()],
            expires_at: 1700000000 + i as u64, // Use the expires_at field
        };
        manager.add_context(format!("user{i}"), context);
    }

    c.bench_function("token_validation", |b| {
        b.iter(|| {
            // Test both valid and invalid tokens
            for i in 0..100 {
                let valid_token = format!("very_long_secure_token_with_lots_of_entropy_{i}");
                let invalid_token = format!("invalid_token_{i}");

                black_box(manager.authenticate(&valid_token));
                black_box(manager.authenticate(&invalid_token));
            }
        })
    });
}

fn bench_permission_checking(c: &mut Criterion) {
    let manager = DistributedSecurityManager::new();

    // Setup users with different permission sets
    for i in 0..500 {
        let permissions = match i % 4 {
            0 => vec!["read".to_string()],
            1 => vec!["read".to_string(), "write".to_string()],
            2 => vec![
                "read".to_string(),
                "write".to_string(),
                "execute".to_string(),
            ],
            _ => vec![
                "read".to_string(),
                "write".to_string(),
                "execute".to_string(),
                "admin".to_string(),
            ],
        };

        let context = SecurityContext {
            user_id: format!("user{i}"),
            permissions,
            ..Default::default()
        };
        manager.add_context(format!("user{i}"), context);
    }

    c.bench_function("permission_checking", |b| {
        b.iter(|| {
            for i in 0..500 {
                let user_id = format!("user{i}");
                black_box(manager.authorize(&user_id, "read"));
                black_box(manager.authorize(&user_id, "write"));
                black_box(manager.authorize(&user_id, "execute"));
                black_box(manager.authorize(&user_id, "admin"));
            }
        })
    });
}

criterion_group!(
    security_benches,
    bench_authentication,
    bench_authorization,
    bench_concurrent_access,
    bench_context_management,
    bench_token_validation,
    bench_permission_checking
);
criterion_main!(security_benches);
