//! DashMap Concurrency Tests - WebSocket Manager
//!
//! Tests for lock-free concurrent access to WebSocket connections

use dashmap::DashMap;
use nestgate_api::websocket::{ConnectionInfo, WebSocketManager};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::task;
use uuid::Uuid;

#[tokio::test]
async fn test_concurrent_websocket_connections() {
    // Test adding connections from multiple tasks concurrently
    let manager = Arc::new(WebSocketManager::new());
    let num_tasks = 100;
    let connections_per_task = 10;
    
    let handles: Vec<_> = (0..num_tasks)
        .map(|task_id| {
            let manager = Arc::clone(&manager);
            task::spawn(async move {
                for i in 0..connections_per_task {
                    let client_id = Uuid::new_v4();
                    let info = ConnectionInfo {
                        client_id,
                        client_type: format!("client-{}-{}", task_id, i),
                        connected_at: SystemTime::now(),
                        last_activity: SystemTime::now(),
                        subscriptions: vec![],
                    };
                    
                    // Insert connection (lock-free!)
                    manager.connections.insert(client_id, info);
                }
            })
        })
        .collect();
    
    // Wait for all tasks
    for handle in handles {
        handle.await.expect("Task failed");
    }
    
    // Verify count
    let expected = num_tasks * connections_per_task;
    let actual = manager.get_connection_count();
    assert_eq!(actual, expected, "Should have all connections");
}

#[tokio::test]
async fn test_concurrent_websocket_broadcast() {
    // Test concurrent broadcasts
    use nestgate_api::websocket::WebSocketEvent;
    
    let manager = Arc::new(WebSocketManager::new());
    
    // Add some connections first
    for i in 0..50 {
        let client_id = Uuid::new_v4();
        let info = ConnectionInfo {
            client_id,
            client_type: format!("client-{}", i),
            connected_at: SystemTime::now(),
            last_activity: SystemTime::now(),
            subscriptions: vec![],
        };
        manager.connections.insert(client_id, info);
    }
    
    // Broadcast from multiple tasks concurrently
    let handles: Vec<_> = (0..20)
        .map(|i| {
            let manager = Arc::clone(&manager);
            task::spawn(async move {
                let event = WebSocketEvent::Custom {
                    event_type: format!("test-{}", i),
                    data: serde_json::json!({"index": i}),
                };
                
                manager.broadcast_event(event).expect("Broadcast failed");
            })
        })
        .collect();
    
    // All should succeed
    for handle in handles {
        handle.await.expect("Broadcast task failed");
    }
}

#[tokio::test]
async fn test_concurrent_stats_updates() {
    // Test concurrent stats updates (lock-free!)
    let manager = Arc::new(WebSocketManager::new());
    let num_tasks = 50;
    let updates_per_task = 100;
    
    let handles: Vec<_> = (0..num_tasks)
        .map(|_| {
            let manager = Arc::clone(&manager);
            task::spawn(async move {
                for _ in 0..updates_per_task {
                    // Update stats (lock-free atomic operations)
                    manager.stats.alter("messages_sent", |_, v| v + 1);
                    manager.stats.alter("bytes_transferred", |_, v| v + 100);
                }
            })
        })
        .collect();
    
    // Wait for all
    for handle in handles {
        handle.await.expect("Stats task failed");
    }
    
    // Verify totals
    let stats = manager.get_stats();
    assert_eq!(
        stats.messages_sent,
        (num_tasks * updates_per_task) as u64,
        "All stat updates should be counted"
    );
}

#[tokio::test]
async fn test_concurrent_read_write() {
    // Test concurrent reads and writes
    let manager = Arc::new(WebSocketManager::new());
    
    // Writers
    let writers: Vec<_> = (0..20)
        .map(|_| {
            let manager = Arc::clone(&manager);
            task::spawn(async move {
                for _ in 0..100 {
                    let client_id = Uuid::new_v4();
                    let info = ConnectionInfo {
                        client_id,
                        client_type: "writer".to_string(),
                        connected_at: SystemTime::now(),
                        last_activity: SystemTime::now(),
                        subscriptions: vec![],
                    };
                    manager.connections.insert(client_id, info);
                    
                    tokio::time::sleep(Duration::from_micros(1)).await;
                }
            })
        })
        .collect();
    
    // Readers
    let readers: Vec<_> = (0..20)
        .map(|_| {
            let manager = Arc::clone(&manager);
            task::spawn(async move {
                for _ in 0..100 {
                    let _count = manager.get_connection_count();
                    tokio::time::sleep(Duration::from_micros(1)).await;
                }
            })
        })
        .collect();
    
    // Wait for all
    for handle in writers.into_iter().chain(readers) {
        handle.await.expect("Read/write task failed");
    }
}

#[tokio::test]
async fn test_stress_high_contention() {
    // Stress test: many tasks hammering the same DashMap
    let manager = Arc::new(WebSocketManager::new());
    let num_tasks = 100;
    let operations_per_task = 1000;
    
    let handles: Vec<_> = (0..num_tasks)
        .map(|task_id| {
            let manager = Arc::clone(&manager);
            task::spawn(async move {
                for i in 0..operations_per_task {
                    // Insert
                    let client_id = Uuid::new_v4();
                    let info = ConnectionInfo {
                        client_id,
                        client_type: format!("task-{}", task_id),
                        connected_at: SystemTime::now(),
                        last_activity: SystemTime::now(),
                        subscriptions: vec![],
                    };
                    manager.connections.insert(client_id, info);
                    
                    // Read
                    let _count = manager.get_connection_count();
                    
                    // Stats update
                    manager.stats.alter("messages_sent", |_, v| v + 1);
                    
                    // Periodic yield
                    if i % 100 == 0 {
                        tokio::task::yield_now().await;
                    }
                }
            })
        })
        .collect();
    
    // All should complete without panics
    for handle in handles {
        handle.await.expect("Stress task failed");
    }
    
    println!(
        "Stress test complete: {} connections, {} stats updates",
        manager.get_connection_count(),
        manager.get_stats().messages_sent
    );
}

#[tokio::test]
async fn test_no_deadlock() {
    // Verify no deadlocks with DashMap (should be impossible with lock-free design)
    let manager = Arc::new(WebSocketManager::new());
    
    // Create circular dependency scenario (that would deadlock with RwLock)
    let handle1 = {
        let manager = Arc::clone(&manager);
        task::spawn(async move {
            for _ in 0..1000 {
                let id = Uuid::new_v4();
                manager.connections.insert(
                    id,
                    ConnectionInfo {
                        client_id: id,
                        client_type: "task1".to_string(),
                        connected_at: SystemTime::now(),
                        last_activity: SystemTime::now(),
                        subscriptions: vec![],
                    },
                );
                let _stats = manager.get_stats(); // Read from stats
            }
        })
    };
    
    let handle2 = {
        let manager = Arc::clone(&manager);
        task::spawn(async move {
            for _ in 0..1000 {
                manager.stats.alter("messages_sent", |_, v| v + 1); // Write to stats
                let _count = manager.get_connection_count(); // Read from connections
            }
        })
    };
    
    // Should complete quickly (no deadlock)
    tokio::time::timeout(Duration::from_secs(5), async {
        handle1.await.expect("Task 1 failed");
        handle2.await.expect("Task 2 failed");
    })
    .await
    .expect("Should not deadlock");
}

#[test]
fn test_dashmap_send_sync() {
    // Compile-time test: DashMap is Send + Sync
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    
    assert_send::<DashMap<Uuid, ConnectionInfo>>();
    assert_sync::<DashMap<Uuid, ConnectionInfo>>();
    assert_send::<WebSocketManager>();
    assert_sync::<WebSocketManager>();
}
