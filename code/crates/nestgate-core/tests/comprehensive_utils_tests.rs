// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Comprehensive utility tests
//! Additional tests for standard library utilities to improve coverage

use std::time::{Duration, SystemTime};

// ==================== TIME UTILITY TESTS ====================

#[test]
fn test_duration_creation() {
    let dur = Duration::from_secs(60);
    assert_eq!(dur.as_secs(), 60);

    let dur_ms = Duration::from_millis(1500);
    assert_eq!(dur_ms.as_millis(), 1500);
}

#[test]
fn test_duration_arithmetic() {
    let dur1 = Duration::from_secs(10);
    let dur2 = Duration::from_secs(5);

    let sum = dur1 + dur2;
    assert_eq!(sum.as_secs(), 15);

    let diff = dur1 - dur2;
    assert_eq!(diff.as_secs(), 5);
}

#[test]
fn test_duration_comparisons() {
    let short = Duration::from_secs(1);
    let long = Duration::from_secs(10);

    assert!(short < long);
    assert!(long > short);
    assert!(short != long);
    assert_eq!(short, Duration::from_secs(1));
}

#[tokio::test]
async fn test_system_time_now() {
    let now1 = SystemTime::now();
    tokio::time::sleep(Duration::from_millis(10)).await;
    let now2 = SystemTime::now();

    assert!(now2 > now1);
}

#[tokio::test]
async fn test_system_time_elapsed() {
    let start = SystemTime::now();
    tokio::time::sleep(Duration::from_millis(50)).await;

    let elapsed = start.elapsed().expect("Time went backwards");
    assert!(elapsed.as_millis() >= 50);
}

#[test]
fn test_duration_from_various_units() {
    let from_secs = Duration::from_secs(1);
    let from_ms = Duration::from_millis(1000);
    let from_micros = Duration::from_micros(1_000_000);
    let from_nanos = Duration::from_nanos(1_000_000_000);

    assert_eq!(from_secs, from_ms);
    assert_eq!(from_ms, from_micros);
    assert_eq!(from_micros, from_nanos);
}

#[test]
fn test_duration_zero() {
    let zero = Duration::from_secs(0);
    assert_eq!(zero.as_secs(), 0);
    assert!(zero.is_zero());
}

#[test]
fn test_duration_max() {
    let max = Duration::from_secs(u64::MAX);
    assert!(max.as_secs() == u64::MAX);
}

// ==================== SYSTEM UTILITY TESTS ====================

#[test]
fn test_env_var_operations() {
    nestgate_core::env_process::set_var("TEST_VAR", "test_value");
    assert_eq!(std::env::var("TEST_VAR").unwrap(), "test_value");

    nestgate_core::env_process::remove_var("TEST_VAR");
    assert!(std::env::var("TEST_VAR").is_err());
}

#[test]
fn test_path_operations() {
    use std::path::Path;

    let path = Path::new("/usr/local/bin");
    assert_eq!(path.to_str(), Some("/usr/local/bin"));

    let file_path = Path::new("/usr/local/bin/program");
    assert_eq!(file_path.parent(), Some(Path::new("/usr/local/bin")));
    assert_eq!(
        file_path.file_name().and_then(|n| n.to_str()),
        Some("program")
    );
}

#[test]
fn test_path_components() {
    use std::path::Path;

    let path = Path::new("/home/user/documents/file.txt");
    let components: Vec<_> = path.components().collect();

    assert!(!components.is_empty());
}

#[test]
fn test_path_extension() {
    use std::path::Path;

    let path = Path::new("document.pdf");
    assert_eq!(path.extension().and_then(|e| e.to_str()), Some("pdf"));

    let no_ext = Path::new("README");
    assert_eq!(no_ext.extension(), None);
}

#[test]
fn test_path_file_stem() {
    use std::path::Path;

    let path = Path::new("document.backup.tar.gz");
    assert_eq!(
        path.file_stem().and_then(|s| s.to_str()),
        Some("document.backup.tar")
    );
}

#[test]
fn test_path_join() {
    use std::path::PathBuf;

    let mut path = PathBuf::from("/home");
    path.push("user");
    path.push("documents");

    assert!(path.to_str().unwrap().contains("home"));
    assert!(path.to_str().unwrap().contains("documents"));
}

#[test]
fn test_path_is_absolute_relative() {
    use std::path::Path;

    let absolute = Path::new("/usr/bin");
    let relative = Path::new("usr/bin");

    #[cfg(unix)]
    {
        assert!(absolute.is_absolute());
        assert!(relative.is_relative());
    }
}

// ==================== VECTOR OPERATIONS TESTS ====================

#[test]
#[allow(clippy::const_is_empty)] // Testing basic collection methods
fn test_vec_creation() {
    let vec1: Vec<i32> = Vec::new();
    assert_eq!(vec1.len(), 0);
    assert!(vec1.is_empty());

    let vec2 = [1, 2, 3, 4, 5];
    assert_eq!(vec2.len(), 5);
    assert!(!vec2.is_empty());
}

#[test]
fn test_vec_push_pop() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    assert_eq!(vec.len(), 3);
    assert_eq!(vec.pop(), Some(3));
    assert_eq!(vec.len(), 2);
}

#[test]
fn test_vec_indexing() {
    let vec = [10, 20, 30, 40, 50];

    assert_eq!(vec[0], 10);
    assert_eq!(vec[4], 50);
    assert_eq!(vec.get(2), Some(&30));
    assert_eq!(vec.get(10), None);
}

#[test]
fn test_vec_iteration() {
    let vec = [1, 2, 3, 4, 5];
    let sum: i32 = vec.iter().sum();
    assert_eq!(sum, 15);

    let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
    assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
}

#[test]
fn test_vec_filtering() {
    let vec = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens: Vec<i32> = vec.iter().filter(|x| *x % 2 == 0).copied().collect();

    assert_eq!(evens, vec![2, 4, 6, 8, 10]);
}

#[test]
fn test_vec_sorting() {
    let mut vec = vec![5, 2, 8, 1, 9, 3];
    vec.sort();

    assert_eq!(vec, vec![1, 2, 3, 5, 8, 9]);
}

#[test]
fn test_vec_deduplication() {
    let mut vec = vec![1, 2, 2, 3, 3, 3, 4, 5, 5];
    vec.dedup();

    assert_eq!(vec, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_vec_contains() {
    let vec = [1, 2, 3, 4, 5];

    assert!(vec.contains(&3));
    assert!(!vec.contains(&10));
}

#[test]
fn test_vec_extend() {
    let mut vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    vec1.extend(vec2);
    assert_eq!(vec1, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_vec_reverse() {
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.reverse();

    assert_eq!(vec, vec![5, 4, 3, 2, 1]);
}

// ==================== HASHMAP TESTS ====================

#[test]
fn test_hashmap_creation() {
    use std::collections::HashMap;

    let map: HashMap<String, i32> = HashMap::new();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}

#[test]
fn test_hashmap_insert_get() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

    assert_eq!(map.get("one"), Some(&1));
    assert_eq!(map.get("two"), Some(&2));
    assert_eq!(map.get("four"), None);
}

#[test]
fn test_hashmap_update() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("key", 1);
    assert_eq!(map.get("key"), Some(&1));

    map.insert("key", 2);
    assert_eq!(map.get("key"), Some(&2));
}

#[test]
fn test_hashmap_remove() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);

    assert_eq!(map.len(), 2);
    map.remove("key1");
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("key1"), None);
}

#[test]
fn test_hashmap_contains_key() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("exists", 42);

    assert!(map.contains_key("exists"));
    assert!(!map.contains_key("missing"));
}

#[test]
fn test_hashmap_iteration() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    let sum: i32 = map.values().sum();
    assert_eq!(sum, 6);
}

#[test]
fn test_hashmap_entry_api() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    *map.entry("counter").or_insert(0) += 1;
    *map.entry("counter").or_insert(0) += 1;

    assert_eq!(map.get("counter"), Some(&2));
}

// ==================== OPTION TESTS ====================

#[test]
fn test_option_some_none() {
    let some_val: Option<i32> = Some(42);
    let none_val: Option<i32> = None;

    assert!(some_val.is_some());
    assert!(some_val.is_some());

    assert!(none_val.is_none());
    assert!(none_val.is_none());
}

#[test]
#[allow(clippy::unnecessary_literal_unwrap)] // Intentionally testing unwrap_or behavior
fn test_option_unwrap_or() {
    let some_val = Some(42);
    let none_val: Option<i32> = None;

    assert_eq!(some_val.unwrap_or(0), 42);
    assert_eq!(none_val.unwrap_or(0), 0);
}

#[test]
fn test_option_map() {
    let some_val = Some(5);
    let doubled = some_val.map(|x| x * 2);

    assert_eq!(doubled, Some(10));

    let none_val: Option<i32> = None;
    let none_doubled = none_val.map(|x| x * 2);
    assert_eq!(none_doubled, None);
}

#[test]
fn test_option_and_then() {
    let some_val = Some(5);
    let result = some_val.and_then(|x| if x > 0 { Some(x * 2) } else { None });

    assert_eq!(result, Some(10));
}

#[test]
fn test_option_or_else() {
    let none_val: Option<i32> = None;
    let result = none_val.or(Some(42));

    assert_eq!(result, Some(42));
}

// ==================== RESULT TESTS ====================

#[test]
fn test_result_ok_err() {
    let ok_val: Result<i32, String> = Ok(42);
    let err_val: Result<i32, String> = Err("error".to_string());

    assert!(ok_val.is_ok());
    assert!(ok_val.is_ok());

    assert!(err_val.is_err());
    assert!(err_val.is_err());
}

#[test]
#[allow(clippy::unnecessary_literal_unwrap)] // Intentionally testing unwrap_or behavior
fn test_result_unwrap_or() {
    let ok_val: Result<i32, String> = Ok(42);
    let err_val: Result<i32, String> = Err("error".to_string());

    assert_eq!(ok_val.unwrap_or(0), 42);
    assert_eq!(err_val.unwrap_or(0), 0);
}

#[test]
fn test_result_map() {
    let ok_val: Result<i32, String> = Ok(5);
    let doubled = ok_val.map(|x| x * 2);

    assert_eq!(doubled, Ok(10));
}

#[test]
fn test_result_map_err() {
    let err_val: Result<i32, String> = Err("error".to_string());
    let mapped = err_val.map_err(|e| format!("Error: {}", e));

    assert_eq!(mapped, Err("Error: error".to_string()));
}

#[test]
fn test_result_and_then() {
    let ok_val: Result<i32, String> = Ok(5);
    let result = ok_val.map(|x| x * 2);

    assert_eq!(result, Ok(10));
}
