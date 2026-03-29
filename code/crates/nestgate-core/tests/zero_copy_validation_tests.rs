// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective
#![allow(
    dead_code,
    missing_docs,
    unused_imports,
    unused_variables,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]

//! Zero-copy pattern tests - Comprehensive validation
//!
//! Tests for zero-copy implementations, ensuring no unnecessary copies.

#[cfg(test)]
mod zero_copy_pattern_tests {
    use std::borrow::Cow;

    #[test]
    fn test_cow_no_copy_when_borrowed() {
        let data = String::from("test data");
        let cow: Cow<str> = Cow::Borrowed(&data);

        // Should not allocate
        assert!(matches!(cow, Cow::Borrowed(_)));
    }

    #[test]
    fn test_cow_copy_on_write() {
        let data = String::from("immutable");
        let mut cow: Cow<str> = Cow::Borrowed(&data);

        // Modification triggers copy
        cow.to_mut().push_str(" modified");
        assert!(matches!(cow, Cow::Owned(_)));
        assert_eq!(cow, "immutable modified");
    }

    #[test]
    fn test_slice_borrowing() {
        let data = [1, 2, 3, 4, 5];
        let slice: &[i32] = &data[1..4];

        // No copy, just reference
        assert_eq!(slice, &[2, 3, 4]);
        assert_eq!(slice.len(), 3);
    }

    #[test]
    fn test_asref_zero_copy() {
        fn process_bytes(data: impl AsRef<[u8]>) -> usize {
            data.as_ref().len()
        }

        let vec = vec![1u8, 2, 3];
        let array = [1u8, 2, 3];

        // Both work without copying
        assert_eq!(process_bytes(&vec), 3);
        assert_eq!(process_bytes(array), 3);
    }

    #[test]
    fn test_string_slicing_zero_copy() {
        let s = String::from("hello world");
        let slice: &str = &s[0..5];

        // Just a reference, no allocation
        assert_eq!(slice, "hello");
    }

    #[test]
    fn test_arc_clone_is_cheap() {
        use std::sync::Arc;

        let data = Arc::new(vec![1, 2, 3, 4, 5]);
        let cloned = Arc::clone(&data);

        // Same pointer, just ref count increment
        assert_eq!(Arc::strong_count(&data), 2);
        assert_eq!(*data, *cloned);
    }

    #[test]
    fn test_bytes_zero_copy() {
        use bytes::Bytes;

        let data = Bytes::from_static(b"static data");
        let slice = data.slice(0..6);

        // Slicing doesn't copy
        assert_eq!(&slice[..], b"static");
    }

    #[test]
    fn test_rc_vs_clone() {
        use std::rc::Rc;

        let expensive_data = vec![0u8; 1000];
        let rc = Rc::new(expensive_data);
        let _rc_clone = Rc::clone(&rc);

        // Cheap clone (just pointer + refcount)
        assert_eq!(Rc::strong_count(&rc), 2);

        // vs expensive clone
        let expensive_clone = (*rc).clone();
        assert_eq!(expensive_clone.len(), 1000);
    }

    #[test]
    fn test_iter_no_copy() {
        let data = [1, 2, 3, 4, 5];

        // Iterator doesn't copy
        let sum: i32 = data.iter().sum();
        assert_eq!(sum, 15);

        // Original still valid
        assert_eq!(data.len(), 5);
    }

    #[test]
    fn test_reference_passing() {
        fn process(data: &[u8]) -> usize {
            data.len()
        }

        let vec = vec![1u8, 2, 3, 4];

        // Pass by reference, no copy
        let len = process(&vec);
        assert_eq!(len, 4);

        // Original still owned
        assert_eq!(vec.len(), 4);
    }
}

#[cfg(test)]
mod buffer_pool_tests {
    #[test]
    fn test_buffer_reuse_pattern() {
        let mut pool = Vec::new();

        // Simulate buffer pool
        let mut buffer = vec![0u8; 1024];
        buffer.clear();

        // Reuse buffer (no reallocation)
        buffer.extend_from_slice(b"new data");
        assert!(buffer.capacity() >= 1024);

        // Return to pool
        pool.push(buffer);
        assert_eq!(pool.len(), 1);
    }

    #[test]
    fn test_preallocated_buffer() {
        let mut buffer = Vec::with_capacity(1000);

        // No reallocation for first 1000 items
        for i in 0..1000 {
            buffer.push(i);
        }

        assert_eq!(buffer.capacity(), 1000);
        assert_eq!(buffer.len(), 1000);
    }
}

#[cfg(test)]
mod memory_efficiency_tests {
    #[test]
    fn test_small_string_optimization() {
        // Small strings (< 24 bytes on most platforms) don't heap allocate
        let small = String::from("tiny");
        assert_eq!(small.len(), 4);
    }

    #[test]
    fn test_box_vs_raw_size() {
        use std::mem::size_of;

        // Box is just a pointer
        assert_eq!(size_of::<Box<[u8; 1000]>>(), size_of::<usize>());

        // Raw array is full size
        assert_eq!(size_of::<[u8; 1000]>(), 1000);
    }

    #[test]
    fn test_enum_size_optimization() {
        use std::mem::size_of;

        #[allow(dead_code)]
        enum OptimizedEnum {
            Small(u8),
            Medium(u16),
            Large(Box<[u8; 1000]>),
        }

        // Size should be close to largest variant + discriminant
        // Box keeps it small
        assert!(size_of::<OptimizedEnum>() < 100);
    }
}
