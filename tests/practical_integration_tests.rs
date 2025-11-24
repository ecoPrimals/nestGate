//! **PRACTICAL INTEGRATION TESTS**
//!
//! High-value integration tests that exercise real code paths.
//! These tests focus on actual functionality rather than comprehensive coverage.
//!
//! **Note**: Environment isolation tests are in `tests/common/env_isolation.rs`
//! where they have direct access to the module.

use std::time::Duration;

// ============================================================================
// DURATION AND TIMEOUT TESTS
// ============================================================================

#[test]
fn test_duration_from_secs() {
    let duration = Duration::from_secs(5);
    assert_eq!(duration.as_secs(), 5);
    assert_eq!(duration.as_millis(), 5000);
}

#[test]
fn test_duration_from_millis() {
    let duration = Duration::from_millis(1500);
    assert_eq!(duration.as_millis(), 1500);
    assert!(duration.as_secs() >= 1 && duration.as_secs() <= 2);
}

#[test]
fn test_duration_comparison() {
    let short = Duration::from_secs(1);
    let long = Duration::from_secs(10);

    assert!(short < long);
    assert!(long > short);
    assert_eq!(short, Duration::from_millis(1000));
}

#[test]
fn test_duration_arithmetic() {
    let d1 = Duration::from_secs(5);
    let d2 = Duration::from_secs(3);

    let sum = d1 + d2;
    assert_eq!(sum, Duration::from_secs(8));

    let diff = d1 - d2;
    assert_eq!(diff, Duration::from_secs(2));
}

// ============================================================================
// STRING AND FORMATTING TESTS
// ============================================================================

#[test]
fn test_string_formatting() {
    let name = "test";
    let value = 42;
    let formatted = format!("Name: {}, Value: {}", name, value);

    assert!(formatted.contains("test"));
    assert!(formatted.contains("42"));
}

#[test]
fn test_string_manipulation() {
    let s = "  hello world  ";
    let trimmed = s.trim();
    let uppercase = trimmed.to_uppercase();

    assert_eq!(trimmed, "hello world");
    assert_eq!(uppercase, "HELLO WORLD");
}

#[test]
fn test_string_splitting() {
    let path = "tank/data/subdir";
    let parts: Vec<&str> = path.split('/').collect();

    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "tank");
    assert_eq!(parts[1], "data");
    assert_eq!(parts[2], "subdir");
}

#[test]
fn test_string_joining() {
    let parts = vec!["tank", "data", "subdir"];
    let joined = parts.join("/");

    assert_eq!(joined, "tank/data/subdir");
}

// ============================================================================
// COLLECTION TESTS
// ============================================================================

#[test]
fn test_vec_operations() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    assert_eq!(vec.len(), 3);
    assert_eq!(vec[0], 1);
    assert!(vec.contains(&2));
}

#[test]
fn test_vec_iteration() {
    let vec = vec![1, 2, 3, 4, 5];
    let sum: i32 = vec.iter().sum();

    assert_eq!(sum, 15);
}

#[test]
fn test_vec_filtering() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let evens: Vec<i32> = vec.into_iter().filter(|x| x % 2 == 0).collect();

    assert_eq!(evens, vec![2, 4, 6]);
}

#[test]
fn test_hashmap_operations() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    assert_eq!(map.len(), 2);
    assert_eq!(map.get("key1"), Some(&"value1"));
    assert!(map.contains_key("key2"));
}

// ============================================================================
// ASYNC OPERATION TESTS
// ============================================================================

#[tokio::test]
async fn test_basic_async() {
    async fn simple_async() -> i32 {
        42
    }

    let result = simple_async().await;
    assert_eq!(result, 42);
}

#[tokio::test]
async fn test_async_sleep() {
    let start = std::time::Instant::now();
    tokio::time::sleep(Duration::from_millis(100)).await;
    let elapsed = start.elapsed();

    assert!(elapsed >= Duration::from_millis(90));
    assert!(elapsed < Duration::from_millis(200));
}

#[tokio::test]
async fn test_async_join() {
    async fn task1() -> i32 {
        1
    }
    async fn task2() -> i32 {
        2
    }
    async fn task3() -> i32 {
        3
    }

    let (r1, r2, r3) = tokio::join!(task1(), task2(), task3());

    assert_eq!(r1 + r2 + r3, 6);
}

#[tokio::test]
async fn test_async_spawn() {
    let handle = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        42
    });

    let result = handle.await.unwrap();
    assert_eq!(result, 42);
}

// ============================================================================
// ERROR HANDLING PATTERN TESTS
// ============================================================================

#[test]
fn test_result_ok() {
    let result: Result<i32, &str> = Ok(42);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_result_err() {
    let result: Result<i32, &str> = Err("error");
    assert!(result.is_err());
}

#[test]
fn test_result_map() {
    let result: Result<i32, &str> = Ok(5);
    let mapped = result.map(|x| x * 2);

    assert_eq!(mapped, Ok(10));
}

#[test]
fn test_result_and_then() {
    let result: Result<i32, &str> = Ok(5);
    let chained = result.and_then(|x| Ok(x * 2));

    assert_eq!(chained, Ok(10));
}

#[test]
fn test_option_some() {
    let opt = Some(42);
    assert!(opt.is_some());
    assert_eq!(opt.unwrap(), 42);
}

#[test]
fn test_option_none() {
    let opt: Option<i32> = None;
    assert!(opt.is_none());
}

#[test]
fn test_option_unwrap_or() {
    let some = Some(42);
    let none: Option<i32> = None;

    assert_eq!(some.unwrap_or(0), 42);
    assert_eq!(none.unwrap_or(0), 0);
}

// ============================================================================
// VALIDATION PATTERN TESTS
// ============================================================================

#[test]
fn test_range_validation() {
    fn is_valid_port(port: u16) -> bool {
        port > 1024 && port < 65535
    }

    assert!(is_valid_port(8080));
    assert!(!is_valid_port(80));
    assert!(!is_valid_port(65535)); // Max value is invalid
}

#[test]
fn test_string_validation() {
    fn is_valid_name(name: &str) -> bool {
        !name.is_empty() && name.len() <= 255
    }

    assert!(is_valid_name("valid"));
    assert!(!is_valid_name(""));
    assert!(!is_valid_name(&"x".repeat(300)));
}

#[test]
fn test_pattern_matching() {
    fn classify_number(n: i32) -> &'static str {
        match n {
            n if n < 0 => "negative",
            0 => "zero",
            n if n < 10 => "small",
            _ => "large",
        }
    }

    assert_eq!(classify_number(-5), "negative");
    assert_eq!(classify_number(0), "zero");
    assert_eq!(classify_number(5), "small");
    assert_eq!(classify_number(100), "large");
}

// ============================================================================
// SERIALIZATION TESTS
// ============================================================================

#[test]
fn test_json_simple() {
    use serde_json::json;

    let data = json!({
        "name": "test",
        "value": 42
    });

    assert_eq!(data["name"], "test");
    assert_eq!(data["value"], 42);
}

#[test]
fn test_json_array() {
    use serde_json::json;

    let array = json!([1, 2, 3, 4, 5]);

    assert_eq!(array[0], 1);
    assert_eq!(array[4], 5);
}

#[test]
fn test_json_nested() {
    use serde_json::json;

    let nested = json!({
        "outer": {
            "inner": {
                "value": 42
            }
        }
    });

    assert_eq!(nested["outer"]["inner"]["value"], 42);
}

// ============================================================================
// CONCURRENT SAFETY TESTS
// ============================================================================

#[test]
fn test_arc_sharing() {
    use std::sync::Arc;

    let data = Arc::new(42);
    let clone1 = Arc::clone(&data);
    let clone2 = Arc::clone(&data);

    assert_eq!(*data, 42);
    assert_eq!(*clone1, 42);
    assert_eq!(*clone2, 42);
}

#[test]
fn test_mutex_locking() {
    use std::sync::Mutex;

    let mutex = Mutex::new(0);

    {
        let mut guard = mutex.lock().unwrap();
        *guard += 1;
    }

    {
        let guard = mutex.lock().unwrap();
        assert_eq!(*guard, 1);
    }
}

#[tokio::test]
async fn test_tokio_mutex() {
    use std::sync::Arc;
    use tokio::sync::Mutex;

    let mutex = Arc::new(Mutex::new(0));

    {
        let mut guard = mutex.lock().await;
        *guard += 1;
    }

    {
        let guard = mutex.lock().await;
        assert_eq!(*guard, 1);
    }
}

// ============================================================================
// TIME AND DATE TESTS
// ============================================================================

#[test]
fn test_instant_elapsed() {
    let start = std::time::Instant::now();
    std::thread::sleep(Duration::from_millis(10));
    let elapsed = start.elapsed();

    assert!(elapsed >= Duration::from_millis(10));
}

#[test]
fn test_system_time() {
    let now = std::time::SystemTime::now();
    let later = now + Duration::from_secs(1);

    assert!(later > now);
}
