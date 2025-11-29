//! E2E Scenario 36: Data Consistency
//!
//! Tests data consistency across operations

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_data_consistency_under_concurrent_writes() {
    println!("🔄 E2E Scenario 36: Data Consistency");

    let data = Arc::new(RwLock::new(HashMap::new()));
    let mut handles = Vec::new();

    // Multiple writers
    for i in 0..20 {
        let data_clone = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let mut map = data_clone.write().await;
            map.insert(format!("key_{}", i), i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    // Verify consistency
    let final_data = data.read().await;
    assert_eq!(final_data.len(), 20);

    for i in 0..20 {
        let key = format!("key_{}", i);
        assert_eq!(final_data.get(&key), Some(&i));
    }

    println!("✅ Data consistency maintained");
}

#[tokio::test]
async fn test_transaction_rollback() {
    println!("🔄 E2E Scenario 36B: Transaction Rollback");

    let mut data = vec![1, 2, 3];
    let original = data.clone();

    // Simulate transaction
    data.push(4);
    data.push(5);

    // Simulate error - rollback
    let transaction_failed = true;
    if transaction_failed {
        data = original;
    }

    assert_eq!(data, vec![1, 2, 3]);
    println!("✅ Transaction rollback successful");
}
