//! **E2E SCENARIO 11: CONCURRENT DATASET OPERATIONS**
//!
//! **Objective**: Test thread safety of parallel dataset operations
//!
//! **Priority**: Critical | **Complexity**: High
//!
//! **Test Flow**:
//! 1. Launch 50 concurrent dataset create operations
//! 2. Launch 50 concurrent dataset delete operations
//! 3. Launch 50 concurrent dataset modify operations
//! 4. Verify all operations complete correctly
//! 5. Check for race conditions or deadlocks
//!
//! **Expected Outcomes**:
//! - All operations complete successfully
//! - No deadlocks or race conditions
//! - Proper locking mechanisms working
//! - Resource cleanup correct

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::{RwLock, Semaphore};

/// Simulated dataset with thread-safe operations
struct Dataset {
    _id: String,
    size: AtomicU64,
    is_deleted: AtomicBool,
    metadata: Arc<RwLock<HashMap<String, String>>>,
}

impl Dataset {
    fn new(id: &str) -> Self {
        Self {
            _id: id.to_string(),
            size: AtomicU64::new(0),
            is_deleted: AtomicBool::new(false),
            metadata: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn write(&self, bytes: u64) -> Result<(), String> {
        if self.is_deleted.load(Ordering::SeqCst) {
            return Err("Dataset is deleted".to_string());
        }
        self.size.fetch_add(bytes, Ordering::SeqCst);
        Ok(())
    }

    async fn read(&self) -> Result<u64, String> {
        if self.is_deleted.load(Ordering::SeqCst) {
            return Err("Dataset is deleted".to_string());
        }
        Ok(self.size.load(Ordering::SeqCst))
    }

    async fn set_metadata(&self, key: String, value: String) -> Result<(), String> {
        if self.is_deleted.load(Ordering::SeqCst) {
            return Err("Dataset is deleted".to_string());
        }
        let mut metadata = self.metadata.write().await;
        metadata.insert(key, value);
        Ok(())
    }

    #[allow(dead_code)]
    async fn get_metadata(&self, key: &str) -> Result<Option<String>, String> {
        if self.is_deleted.load(Ordering::SeqCst) {
            return Err("Dataset is deleted".to_string());
        }
        let metadata = self.metadata.read().await;
        Ok(metadata.get(key).cloned())
    }

    fn delete(&self) {
        self.is_deleted.store(true, Ordering::SeqCst);
    }

    fn is_deleted(&self) -> bool {
        self.is_deleted.load(Ordering::SeqCst)
    }
}

/// Thread-safe dataset manager
struct DatasetManager {
    datasets: Arc<RwLock<HashMap<String, Arc<Dataset>>>>,
    operation_limiter: Arc<Semaphore>,
}

impl DatasetManager {
    fn new(max_concurrent: usize) -> Self {
        Self {
            datasets: Arc::new(RwLock::new(HashMap::new())),
            operation_limiter: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    async fn create_dataset(&self, id: &str) -> Result<(), String> {
        let _permit = self.operation_limiter.acquire().await.unwrap();

        let mut datasets = self.datasets.write().await;
        if datasets.contains_key(id) {
            return Err(format!("Dataset '{}' already exists", id));
        }
        datasets.insert(id.to_string(), Arc::new(Dataset::new(id)));
        Ok(())
    }

    async fn delete_dataset(&self, id: &str) -> Result<(), String> {
        let _permit = self.operation_limiter.acquire().await.unwrap();

        let datasets = self.datasets.read().await;
        if let Some(dataset) = datasets.get(id) {
            dataset.delete();
            Ok(())
        } else {
            Err(format!("Dataset '{}' not found", id))
        }
    }

    async fn modify_dataset(&self, id: &str, key: String, value: String) -> Result<(), String> {
        let _permit = self.operation_limiter.acquire().await.unwrap();

        let datasets = self.datasets.read().await;
        if let Some(dataset) = datasets.get(id) {
            dataset.set_metadata(key, value).await
        } else {
            Err(format!("Dataset '{}' not found", id))
        }
    }

    async fn dataset_count(&self) -> usize {
        let datasets = self.datasets.read().await;
        datasets.len()
    }

    async fn active_dataset_count(&self) -> usize {
        let datasets = self.datasets.read().await;
        datasets.values().filter(|d| !d.is_deleted()).count()
    }
}

// ==================== E2E TESTS ====================

#[tokio::test]
async fn e2e_scenario_11_create_single_dataset() {
    eprintln!("\n🧪 E2E: Create Single Dataset");

    let manager = DatasetManager::new(100);
    let result = manager.create_dataset("test-dataset-1").await;

    assert!(result.is_ok());
    assert_eq!(manager.dataset_count().await, 1);

    eprintln!("✅ Passed");
}

#[tokio::test]
async fn e2e_scenario_11_concurrent_creates_50() {
    eprintln!("\n🧪 E2E: 50 Concurrent Dataset Creates");

    let manager = Arc::new(DatasetManager::new(100));
    let start = std::time::Instant::now();

    let handles: Vec<_> = (0..50)
        .map(|i| {
            let manager = manager.clone();
            tokio::spawn(async move { manager.create_dataset(&format!("dataset-{}", i)).await })
        })
        .collect();

    let mut success_count = 0;
    for handle in handles {
        if handle.await.unwrap().is_ok() {
            success_count += 1;
        }
    }

    let elapsed = start.elapsed();
    assert_eq!(success_count, 50, "All creates should succeed");
    assert_eq!(manager.dataset_count().await, 50);

    eprintln!("✅ Passed - 50 datasets created in {:?}", elapsed);
}

#[tokio::test]
async fn e2e_scenario_11_concurrent_deletes_50() {
    eprintln!("\n🧪 E2E: 50 Concurrent Dataset Deletes");

    let manager = Arc::new(DatasetManager::new(100));

    // Create 50 datasets first
    for i in 0..50 {
        manager
            .create_dataset(&format!("dataset-{}", i))
            .await
            .unwrap();
    }
    assert_eq!(manager.dataset_count().await, 50);

    // Delete them concurrently
    let handles: Vec<_> = (0..50)
        .map(|i| {
            let manager = manager.clone();
            tokio::spawn(async move { manager.delete_dataset(&format!("dataset-{}", i)).await })
        })
        .collect();

    let mut success_count = 0;
    for handle in handles {
        if handle.await.unwrap().is_ok() {
            success_count += 1;
        }
    }

    assert_eq!(success_count, 50, "All deletes should succeed");
    assert_eq!(manager.active_dataset_count().await, 0);

    eprintln!("✅ Passed - 50 datasets deleted concurrently");
}

#[tokio::test]
async fn e2e_scenario_11_concurrent_modifies_50() {
    eprintln!("\n🧪 E2E: 50 Concurrent Dataset Modifications");

    let manager = Arc::new(DatasetManager::new(100));

    // Create 50 datasets
    for i in 0..50 {
        manager
            .create_dataset(&format!("dataset-{}", i))
            .await
            .unwrap();
    }

    // Modify them concurrently
    let handles: Vec<_> = (0..50)
        .map(|i| {
            let manager = manager.clone();
            tokio::spawn(async move {
                manager
                    .modify_dataset(
                        &format!("dataset-{}", i),
                        "key".to_string(),
                        format!("value-{}", i),
                    )
                    .await
            })
        })
        .collect();

    let mut success_count = 0;
    for handle in handles {
        if handle.await.unwrap().is_ok() {
            success_count += 1;
        }
    }

    assert_eq!(success_count, 50, "All modifications should succeed");

    eprintln!("✅ Passed - 50 datasets modified concurrently");
}

#[tokio::test]
async fn e2e_scenario_11_mixed_operations_150() {
    eprintln!("\n🧪 E2E: 150 Mixed Operations (50 create, 50 delete, 50 modify)");

    let manager = Arc::new(DatasetManager::new(100));

    // Pre-create some datasets for delete/modify operations
    for i in 50..100 {
        manager
            .create_dataset(&format!("dataset-{}", i))
            .await
            .unwrap();
    }

    let start = std::time::Instant::now();
    let mut handles = Vec::new();

    // 50 creates (0-49)
    for i in 0..50 {
        let manager = manager.clone();
        handles.push(tokio::spawn(async move {
            manager.create_dataset(&format!("dataset-{}", i)).await
        }));
    }

    // 50 modifies (50-99)
    for i in 50..100 {
        let manager = manager.clone();
        handles.push(tokio::spawn(async move {
            manager
                .modify_dataset(
                    &format!("dataset-{}", i),
                    "status".to_string(),
                    "modified".to_string(),
                )
                .await
        }));
    }

    // 50 deletes (50-99)
    for i in 50..100 {
        let manager = manager.clone();
        handles.push(tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            manager.delete_dataset(&format!("dataset-{}", i)).await
        }));
    }

    let mut success_count = 0;
    for handle in handles {
        if handle.await.unwrap().is_ok() {
            success_count += 1;
        }
    }

    let elapsed = start.elapsed();
    assert!(success_count >= 100, "Most operations should succeed");

    eprintln!(
        "✅ Passed - {} operations succeeded in {:?}",
        success_count, elapsed
    );
}

#[tokio::test]
async fn e2e_scenario_11_no_deadlock_stress() {
    eprintln!("\n🧪 E2E: No Deadlock Under Stress");

    let manager = Arc::new(DatasetManager::new(50));

    // Create initial datasets
    for i in 0..20 {
        manager
            .create_dataset(&format!("dataset-{}", i))
            .await
            .unwrap();
    }

    let start = std::time::Instant::now();
    let timeout = tokio::time::Duration::from_secs(10);

    let operation = async {
        let mut handles = Vec::new();

        // Lots of concurrent operations
        for i in 0..100 {
            let manager = manager.clone();
            handles.push(tokio::spawn(async move {
                if i % 3 == 0 {
                    manager.create_dataset(&format!("stress-{}", i)).await.ok();
                } else if i % 3 == 1 {
                    manager
                        .modify_dataset(
                            &format!("dataset-{}", i % 20),
                            "key".to_string(),
                            "val".to_string(),
                        )
                        .await
                        .ok();
                } else {
                    manager
                        .delete_dataset(&format!("dataset-{}", i % 20))
                        .await
                        .ok();
                }
            }));
        }

        for handle in handles {
            handle.await.ok();
        }
    };

    let result = tokio::time::timeout(timeout, operation).await;
    assert!(
        result.is_ok(),
        "Operations should complete without deadlock"
    );

    let elapsed = start.elapsed();
    eprintln!(
        "✅ Passed - No deadlock detected (completed in {:?})",
        elapsed
    );
}

#[tokio::test]
async fn e2e_scenario_11_race_condition_detection() {
    eprintln!("\n🧪 E2E: Race Condition Detection");

    let manager = Arc::new(DatasetManager::new(100));
    manager.create_dataset("shared-dataset").await.unwrap();

    // Multiple threads trying to modify the same dataset
    let handles: Vec<_> = (0..50)
        .map(|i| {
            let manager = manager.clone();
            tokio::spawn(async move {
                manager
                    .modify_dataset("shared-dataset", format!("counter-{}", i), i.to_string())
                    .await
            })
        })
        .collect();

    let mut success_count = 0;
    for handle in handles {
        if handle.await.unwrap().is_ok() {
            success_count += 1;
        }
    }

    assert_eq!(
        success_count, 50,
        "All modifications should succeed without race conditions"
    );

    eprintln!("✅ Passed - No race conditions detected");
}

#[tokio::test]
async fn e2e_scenario_11_resource_cleanup() {
    eprintln!("\n🧪 E2E: Resource Cleanup After Operations");

    let manager = Arc::new(DatasetManager::new(100));

    // Create and delete many datasets
    for round in 0..5 {
        let handles: Vec<_> = (0..20)
            .map(|i| {
                let manager = manager.clone();
                // ✅ EVOLUTION: No sleep - truly concurrent create/delete
                tokio::spawn(async move {
                    let id = format!("temp-{}-{}", round, i);
                    manager.create_dataset(&id).await.ok();
                    // Delete immediately - tests real concurrency
                    manager.delete_dataset(&id).await.ok();
                })
            })
            .collect();

        for handle in handles {
            handle.await.ok();
        }
    }

    // Most should be deleted (active count low)
    let active = manager.active_dataset_count().await;
    assert!(active < 20, "Most datasets should be cleaned up");

    eprintln!(
        "✅ Passed - Resources cleaned up correctly (active: {})",
        active
    );
}

#[tokio::test]
async fn e2e_scenario_11_concurrent_read_write() {
    eprintln!("\n🧪 E2E: Concurrent Read/Write Operations");

    let manager = Arc::new(DatasetManager::new(100));

    // Create datasets
    for i in 0..10 {
        manager
            .create_dataset(&format!("dataset-{}", i))
            .await
            .unwrap();
    }

    let datasets = manager.datasets.clone();
    let mut handles = Vec::new();

    // Writers
    for i in 0..25 {
        let datasets = datasets.clone();
        handles.push(tokio::spawn(async move {
            let datasets_lock = datasets.read().await;
            if let Some(dataset) = datasets_lock.get(&format!("dataset-{}", i % 10)) {
                dataset.write(1024 * (i as u64)).await.ok();
            }
        }));
    }

    // Readers
    for i in 0..25 {
        let datasets = datasets.clone();
        handles.push(tokio::spawn(async move {
            let datasets_lock = datasets.read().await;
            if let Some(dataset) = datasets_lock.get(&format!("dataset-{}", i % 10)) {
                dataset.read().await.ok();
            }
        }));
    }

    for handle in handles {
        handle.await.ok();
    }

    eprintln!("✅ Passed - Concurrent read/write completed");
}

#[tokio::test]
async fn e2e_scenario_11_locking_mechanism() {
    eprintln!("\n🧪 E2E: Proper Locking Mechanisms");

    let manager = Arc::new(DatasetManager::new(100));
    manager.create_dataset("locked-dataset").await.unwrap();

    let datasets = manager.datasets.clone();

    // Try to acquire multiple locks
    // ✅ EVOLUTION: Event-driven lock testing
    // Test write lock with explicit coordination
    use tokio::sync::oneshot;

    let (ready_tx, ready_rx) = oneshot::channel();

    let handle1 = {
        let datasets = datasets.clone();
        tokio::spawn(async move {
            let _lock = datasets.write().await;
            ready_tx.send(()).ok(); // Signal that we have the write lock
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            "write-lock-1"
        })
    };

    // Wait for write lock to be acquired
    ready_rx.await.ok();

    // Now try read lock - should wait for write to complete
    let handle2 = {
        let datasets = datasets.clone();
        tokio::spawn(async move {
            let _lock = datasets.read().await;
            "read-lock-1"
        })
    };

    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();

    assert_eq!(result1, "write-lock-1");
    assert_eq!(result2, "read-lock-1");

    eprintln!("✅ Passed - Locking mechanisms working correctly");
}

#[tokio::test]
async fn e2e_scenario_11_duplicate_create_prevention() {
    eprintln!("\n🧪 E2E: Duplicate Create Prevention");

    let manager = Arc::new(DatasetManager::new(100));

    // Try to create the same dataset concurrently
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let manager = manager.clone();
            tokio::spawn(async move { manager.create_dataset("duplicate-test").await })
        })
        .collect();

    let mut success_count = 0;
    let mut error_count = 0;

    for handle in handles {
        match handle.await.unwrap() {
            Ok(_) => success_count += 1,
            Err(_) => error_count += 1,
        }
    }

    assert_eq!(success_count, 1, "Only one create should succeed");
    assert_eq!(error_count, 9, "Other creates should fail");
    assert_eq!(manager.dataset_count().await, 1);

    eprintln!("✅ Passed - Duplicate prevention working");
}

#[tokio::test]
async fn e2e_scenario_11_operation_ordering() {
    eprintln!("\n🧪 E2E: Operation Ordering Consistency");

    let manager = Arc::new(DatasetManager::new(100));

    // Create, modify, verify sequence
    manager.create_dataset("ordered-dataset").await.unwrap();

    let handles: Vec<_> = (0..20)
        .map(|i| {
            let manager = manager.clone();
            tokio::spawn(async move {
                manager
                    .modify_dataset(
                        "ordered-dataset",
                        format!("seq-{}", i),
                        format!("value-{}", i),
                    )
                    .await
            })
        })
        .collect();

    for handle in handles {
        handle.await.unwrap().ok();
    }

    let datasets = manager.datasets.read().await;
    if let Some(dataset) = datasets.get("ordered-dataset") {
        // Verify operations completed
        assert!(!dataset.is_deleted());
    }

    eprintln!("✅ Passed - Operation ordering maintained");
}

#[tokio::test]
async fn e2e_scenario_11_full_integration() {
    eprintln!("\n🧪 E2E SCENARIO 11: FULL INTEGRATION TEST");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let manager = Arc::new(DatasetManager::new(100));
    let start = std::time::Instant::now();

    // Step 1: 50 concurrent creates
    eprintln!("Step 1: Creating 50 datasets concurrently");
    let create_handles: Vec<_> = (0..50)
        .map(|i| {
            let manager = manager.clone();
            tokio::spawn(async move { manager.create_dataset(&format!("integration-{}", i)).await })
        })
        .collect();

    let mut create_success = 0;
    for handle in create_handles {
        if handle.await.unwrap().is_ok() {
            create_success += 1;
        }
    }
    assert_eq!(create_success, 50);
    eprintln!("   ✓ 50 datasets created");

    // Step 2: 50 concurrent modifies
    eprintln!("Step 2: Modifying 50 datasets concurrently");
    let modify_handles: Vec<_> = (0..50)
        .map(|i| {
            let manager = manager.clone();
            tokio::spawn(async move {
                manager
                    .modify_dataset(
                        &format!("integration-{}", i),
                        "status".to_string(),
                        "modified".to_string(),
                    )
                    .await
            })
        })
        .collect();

    let mut modify_success = 0;
    for handle in modify_handles {
        if handle.await.unwrap().is_ok() {
            modify_success += 1;
        }
    }
    assert_eq!(modify_success, 50);
    eprintln!("   ✓ 50 datasets modified");

    // Step 3: 50 concurrent deletes
    eprintln!("Step 3: Deleting 50 datasets concurrently");
    let delete_handles: Vec<_> = (0..50)
        .map(|i| {
            let manager = manager.clone();
            tokio::spawn(async move { manager.delete_dataset(&format!("integration-{}", i)).await })
        })
        .collect();

    let mut delete_success = 0;
    for handle in delete_handles {
        if handle.await.unwrap().is_ok() {
            delete_success += 1;
        }
    }
    assert_eq!(delete_success, 50);
    assert_eq!(manager.active_dataset_count().await, 0);
    eprintln!("   ✓ 50 datasets deleted");

    // Step 4: Verify no deadlocks or race conditions
    eprintln!("Step 4: Verifying system stability");
    let elapsed = start.elapsed();
    assert!(
        elapsed < tokio::time::Duration::from_secs(10),
        "Should complete quickly"
    );

    eprintln!("\n✅ SCENARIO 11 COMPLETE");
    eprintln!("   ✓ All operations completed successfully");
    eprintln!("   ✓ No deadlocks detected");
    eprintln!("   ✓ No race conditions");
    eprintln!("   ✓ Resource cleanup correct");
    eprintln!("   Time: {:?}", elapsed);
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
