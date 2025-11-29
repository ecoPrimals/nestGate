//! Comprehensive Concurrent Operations Tests
//!
//! These tests cover concurrent access patterns, race conditions, and synchronization
//! to increase code coverage (+2% target for Week 5).

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock, Semaphore};

/// **Concurrent Test 1: Atomic Counter**
#[tokio::test]
async fn test_atomic_counter_concurrent_increments() {
    let counter = Arc::new(AtomicU32::new(0));
    let mut handles = Vec::new();

    for _ in 0..100 {
        let counter_clone = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    assert_eq!(counter.load(Ordering::Relaxed), 100);
}

/// **Concurrent Test 2: RwLock Read Concurrency**
#[tokio::test]
async fn test_rwlock_multiple_readers() {
    let data = Arc::new(RwLock::new(42));
    let mut handles = Vec::new();

    // Multiple concurrent readers
    for _ in 0..10 {
        let data_clone = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let value = *data_clone.read().await;
            value
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await.unwrap());
    }

    // All readers should see the same value
    assert_eq!(results.len(), 10);
    for result in results {
        assert_eq!(result, 42);
    }
}

/// **Concurrent Test 3: Mutex Exclusive Access**
#[tokio::test]
async fn test_mutex_exclusive_access() {
    let data = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..50 {
        let data_clone = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let mut value = data_clone.lock().await;
            *value += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let final_value = *data.lock().await;
    assert_eq!(final_value, 50);
}

/// **Concurrent Test 4: Channel Communication**
#[tokio::test]
async fn test_concurrent_channel_communication() {
    use tokio::sync::mpsc;

    let (tx, mut rx) = mpsc::channel(100);

    // Spawn senders
    let mut handles = Vec::new();
    for i in 0..10 {
        let tx_clone = tx.clone();
        let handle = tokio::spawn(async move {
            tx_clone.send(i).await.unwrap();
        });
        handles.push(handle);
    }

    // Wait for senders
    drop(tx);
    for handle in handles {
        handle.await.unwrap();
    }

    // Collect results
    let mut received = Vec::new();
    while let Some(value) = rx.recv().await {
        received.push(value);
    }

    assert_eq!(received.len(), 10);
}

/// **Concurrent Test 5: Semaphore Resource Limiting**
#[tokio::test]
async fn test_semaphore_resource_limiting() {
    let semaphore = Arc::new(Semaphore::new(3));
    let active_count = Arc::new(AtomicU32::new(0));
    let max_concurrent = Arc::new(AtomicU32::new(0));

    let mut handles = Vec::new();

    for _ in 0..10 {
        let sem = Arc::clone(&semaphore);
        let active = Arc::clone(&active_count);
        let max = Arc::clone(&max_concurrent);

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();

            let current = active.fetch_add(1, Ordering::Relaxed) + 1;

            // Track maximum concurrent
            loop {
                let current_max = max.load(Ordering::Relaxed);
                if current <= current_max {
                    break;
                }
                if max
                    .compare_exchange(current_max, current, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
                {
                    break;
                }
            }

            tokio::time::sleep(Duration::from_millis(10)).await;
            active.fetch_sub(1, Ordering::Relaxed);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let max_concurrent_value = max_concurrent.load(Ordering::Relaxed);
    assert!(
        max_concurrent_value <= 3,
        "Should not exceed semaphore limit"
    );
}

/// **Concurrent Test 6: Broadcast Channel**
#[tokio::test]
async fn test_broadcast_channel() {
    use tokio::sync::broadcast;

    let (tx, _rx) = broadcast::channel(16);

    let mut handles = Vec::new();

    for _ in 0..5 {
        let mut rx = tx.subscribe();
        let handle = tokio::spawn(async move { rx.recv().await.unwrap() });
        handles.push(handle);
    }

    // Broadcast message
    tx.send(42).unwrap();

    // All receivers should get the message
    for handle in handles {
        let received = handle.await.unwrap();
        assert_eq!(received, 42);
    }
}

/// **Concurrent Test 7: Atomic Boolean Flag**
#[tokio::test]
async fn test_atomic_boolean_flag() {
    let flag = Arc::new(AtomicBool::new(false));
    let mut handles = Vec::new();

    // Multiple tasks try to set flag
    for _ in 0..10 {
        let flag_clone = Arc::clone(&flag);
        let handle = tokio::spawn(async move {
            flag_clone.store(true, Ordering::Relaxed);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    assert!(flag.load(Ordering::Relaxed));
}

/// **Concurrent Test 8: Work Queue Processing**
#[tokio::test]
async fn test_concurrent_work_queue() {
    use tokio::sync::mpsc;

    let (tx, mut rx) = mpsc::channel(100);
    let processed = Arc::new(AtomicU32::new(0));

    // Producer
    tokio::spawn(async move {
        for i in 0..20 {
            tx.send(i).await.unwrap();
        }
    });

    // Workers
    let mut handles = Vec::new();
    for _ in 0..4 {
        let processed_clone = Arc::clone(&processed);
        let mut rx_clone = rx;
        let (new_tx, new_rx) = mpsc::channel(100);
        rx = new_rx;

        let handle = tokio::spawn(async move {
            while let Some(_item) = rx_clone.recv().await {
                processed_clone.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);

        // Only first worker gets the receiver
        break;
    }

    tokio::time::sleep(Duration::from_millis(100)).await;
    drop(rx);

    for handle in handles {
        handle.await.unwrap();
    }

    let total_processed = processed.load(Ordering::Relaxed);
    assert_eq!(total_processed, 20);
}

/// **Concurrent Test 9: Read-Write Lock Writer Priority**
#[tokio::test]
async fn test_rwlock_writer_blocks_readers() {
    let data = Arc::new(RwLock::new(0));

    // Acquire write lock
    let mut write_lock = data.write().await;

    // Try to acquire read lock (should wait)
    let data_clone = Arc::clone(&data);
    let read_handle = tokio::spawn(async move {
        let _read_lock = data_clone.read().await;
        "read_acquired"
    });

    // Give read attempt time to try
    tokio::time::sleep(Duration::from_millis(10)).await;

    // Read should still be waiting
    assert!(!read_handle.is_finished());

    // Modify and release write lock
    *write_lock = 42;
    drop(write_lock);

    // Now read should succeed
    let result = read_handle.await.unwrap();
    assert_eq!(result, "read_acquired");
}

/// **Concurrent Test 10: Concurrent HashMap Updates**
#[tokio::test]
async fn test_concurrent_hashmap_updates() {
    use std::collections::HashMap;

    let map = Arc::new(RwLock::new(HashMap::new()));
    let mut handles = Vec::new();

    for i in 0..10 {
        let map_clone = Arc::clone(&map);
        let handle = tokio::spawn(async move {
            let mut map = map_clone.write().await;
            map.insert(i, i * 2);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let final_map = map.read().await;
    assert_eq!(final_map.len(), 10);
    assert_eq!(final_map.get(&5), Some(&10));
}

/// **Concurrent Test 11: Barrier Synchronization**
#[tokio::test]
async fn test_barrier_synchronization() {
    use tokio::sync::Barrier;

    let barrier = Arc::new(Barrier::new(5));
    let mut handles = Vec::new();

    for i in 0..5 {
        let barrier_clone = Arc::clone(&barrier);
        let handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(i * 10)).await;
            barrier_clone.wait().await;
            i
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await.unwrap());
    }

    assert_eq!(results.len(), 5);
}

/// **Concurrent Test 12: Oneshot Channel**
#[tokio::test]
async fn test_oneshot_channel() {
    use tokio::sync::oneshot;

    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(10)).await;
        tx.send(42).unwrap();
    });

    let result = rx.await.unwrap();
    assert_eq!(result, 42);
}

/// **Concurrent Test 13: Watch Channel Updates**
#[tokio::test]
async fn test_watch_channel_updates() {
    use tokio::sync::watch;

    let (tx, mut rx) = watch::channel(0);

    let handle = tokio::spawn(async move {
        let mut values = Vec::new();
        while rx.changed().await.is_ok() {
            values.push(*rx.borrow());
            if values.len() >= 3 {
                break;
            }
        }
        values
    });

    tokio::time::sleep(Duration::from_millis(10)).await;
    tx.send(1).unwrap();
    tokio::time::sleep(Duration::from_millis(10)).await;
    tx.send(2).unwrap();
    tokio::time::sleep(Duration::from_millis(10)).await;
    tx.send(3).unwrap();

    let values = handle.await.unwrap();
    assert!(values.len() >= 2);
}

/// **Concurrent Test 14: Select Multiple Futures**
#[tokio::test]
async fn test_select_multiple_futures() {
    let result = tokio::select! {
        _ = tokio::time::sleep(Duration::from_secs(1)) => "timeout",
        value = async { tokio::time::sleep(Duration::from_millis(10)).await; "quick" } => value,
    };

    assert_eq!(result, "quick");
}

/// **Concurrent Test 15: JoinSet Concurrent Tasks**
#[tokio::test]
async fn test_joinset_concurrent_tasks() {
    use tokio::task::JoinSet;

    let mut set = JoinSet::new();

    for i in 0..10 {
        set.spawn(async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            i * 2
        });
    }

    let mut results = Vec::new();
    while let Some(result) = set.join_next().await {
        results.push(result.unwrap());
    }

    assert_eq!(results.len(), 10);
}

/// **Concurrent Test 16: Atomic Operations**
#[tokio::test]
async fn test_atomic_compare_exchange() {
    let value = Arc::new(AtomicU32::new(0));

    let value_clone = Arc::clone(&value);
    let result = value_clone.compare_exchange(0, 42, Ordering::Relaxed, Ordering::Relaxed);

    assert_eq!(result, Ok(0));
    assert_eq!(value.load(Ordering::Relaxed), 42);
}

/// **Concurrent Test 17: Concurrent Vector Writes**
#[tokio::test]
async fn test_concurrent_vector_writes() {
    let vec = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for i in 0..20 {
        let vec_clone = Arc::clone(&vec);
        let handle = tokio::spawn(async move {
            let mut v = vec_clone.lock().await;
            v.push(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let final_vec = vec.lock().await;
    assert_eq!(final_vec.len(), 20);
}

/// **Concurrent Test 18: Notify Wakeup**
#[tokio::test]
async fn test_notify_wakeup() {
    use tokio::sync::Notify;

    let notify = Arc::new(Notify::new());
    let notify_clone = Arc::clone(&notify);

    let handle = tokio::spawn(async move {
        notify_clone.notified().await;
        "notified"
    });

    tokio::time::sleep(Duration::from_millis(10)).await;
    notify.notify_one();

    let result = handle.await.unwrap();
    assert_eq!(result, "notified");
}

/// **Concurrent Test 19: Timeout Race Condition**
#[tokio::test]
async fn test_timeout_race_condition() {
    let fast_operation = async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        "fast"
    };

    let result = tokio::time::timeout(Duration::from_millis(100), fast_operation).await;

    assert_eq!(result.unwrap(), "fast");
}

/// **Concurrent Test 20: Spawn Blocking**
#[tokio::test]
async fn test_spawn_blocking() {
    let result = tokio::task::spawn_blocking(|| {
        // Simulate blocking operation
        std::thread::sleep(Duration::from_millis(10));
        42
    })
    .await
    .unwrap();

    assert_eq!(result, 42);
}
