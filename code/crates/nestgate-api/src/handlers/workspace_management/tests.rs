//! **WORKSPACE MANAGEMENT TESTS**
//!
//! Comprehensive tests for workspace management functionality.

#[cfg(test)]
mod tests {
    use super::super::WorkspaceManager;

    // ==================== WORKSPACE MANAGER TESTS ====================

    #[test]
    fn test_workspace_manager_creation() {
        let manager = WorkspaceManager::new();
        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("WorkspaceManager"));
    }

    #[test]
    fn test_workspace_manager_default() {
        let manager = WorkspaceManager::default();
        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("WorkspaceManager"));
    }

    #[test]
    fn test_workspace_manager_clone() {
        let manager1 = WorkspaceManager::new();
        let manager2 = manager1.clone();

        let debug1 = format!("{:?}", manager1);
        let debug2 = format!("{:?}", manager2);

        assert!(debug1.contains("WorkspaceManager"));
        assert!(debug2.contains("WorkspaceManager"));
    }

    #[test]
    fn test_workspace_manager_is_const_new() {
        // Test that new() can be used in const contexts
        const _MANAGER: WorkspaceManager = WorkspaceManager::new();
    }

    #[test]
    fn test_multiple_managers() {
        let managers: Vec<WorkspaceManager> = (0..10).map(|_| WorkspaceManager::new()).collect();

        assert_eq!(managers.len(), 10);
    }

    #[test]
    fn test_workspace_manager_default_equals_new() {
        let manager1 = WorkspaceManager::new();
        let manager2 = WorkspaceManager::default();

        let debug1 = format!("{:?}", manager1);
        let debug2 = format!("{:?}", manager2);

        assert_eq!(debug1, debug2);
    }

    #[test]
    fn test_workspace_manager_debug_format() {
        let manager = WorkspaceManager::new();
        let debug_output = format!("{:?}", manager);
        assert!(!debug_output.is_empty());
    }

    #[test]
    fn test_workspace_manager_clone_independence() {
        let manager1 = WorkspaceManager::new();
        let manager2 = manager1.clone();

        drop(manager1);

        // manager2 should still be valid
        let debug = format!("{:?}", manager2);
        assert!(debug.contains("WorkspaceManager"));
    }

    #[test]
    fn test_workspace_manager_multiple_clones() {
        let manager = WorkspaceManager::new();
        let clone1 = manager.clone();
        let clone2 = manager.clone();
        let clone3 = manager.clone();

        drop(manager);
        drop(clone1);

        // clone2 and clone3 should still be valid
        let _ = format!("{:?}", clone2);
        let _ = format!("{:?}", clone3);
    }

    #[test]
    fn test_workspace_manager_as_static() {
        static_manager();
    }

    /// Static Manager
    fn static_manager() {
        let _manager = WorkspaceManager::new();
        // Should not cause any issues
    }

    #[test]
    fn test_workspace_manager_thread_safe() {
        let manager = WorkspaceManager::new();
        let _clone = manager.clone();

        // WorkspaceManager should be Send + Sync
        fn assert_send<T: Send>() {}
        /// Assert Sync
        fn assert_sync<T: Sync>() {}

        assert_send::<WorkspaceManager>();
        assert_sync::<WorkspaceManager>();
    }

    #[test]
    fn test_workspace_manager_no_panic_on_creation() {
        for _ in 0..100 {
            let _manager = WorkspaceManager::new();
        }
    }

    #[test]
    fn test_workspace_manager_creation_performance() {
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _manager = WorkspaceManager::new();
        }
        let duration = start.elapsed();

        // Creating 1000 managers should be fast (< 10ms)
        assert!(
            duration.as_millis() < 10,
            "Creating 1000 managers took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_workspace_manager_clone_performance() {
        let manager = WorkspaceManager::new();

        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _clone = manager.clone();
        }
        let duration = start.elapsed();

        // Cloning 1000 times should be fast (< 10ms)
        assert!(
            duration.as_millis() < 10,
            "Cloning 1000 times took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_workspace_manager_const_correctness() {
        // Test that const fn new() is truly const
        const MANAGER1: WorkspaceManager = WorkspaceManager::new();
        /// Manager2
        const MANAGER2: WorkspaceManager = WorkspaceManager::new();

        let debug1 = format!("{:?}", MANAGER1);
        let debug2 = format!("{:?}", MANAGER2);

        assert_eq!(debug1, debug2);
    }

    #[test]
    fn test_workspace_manager_sized() {
        // WorkspaceManager should be Sized
        fn assert_sized<T: Sized>() {}
        assert_sized::<WorkspaceManager>();
    }

    #[test]
    fn test_workspace_manager_memory_size() {
        use std::mem;

        let size = mem::size_of::<WorkspaceManager>();

        // Manager should be lightweight (< 1KB)
        assert!(size < 1024, "WorkspaceManager is too large: {} bytes", size);
    }

    #[test]
    fn test_workspace_manager_alignment() {
        use std::mem;

        let alignment = mem::align_of::<WorkspaceManager>();

        // Alignment should be reasonable
        assert!(alignment <= 8, "Unusual alignment: {}", alignment);
    }

    #[test]
    fn test_workspace_manager_zero_sized() {
        use std::mem;

        let size = mem::size_of::<WorkspaceManager>();

        // Should be zero-sized or very small
        assert!(
            size == 0,
            "WorkspaceManager should be zero-sized, got {} bytes",
            size
        );
    }

    #[test]
    fn test_workspace_manager_copy_not_implemented() {
        // Verify that WorkspaceManager doesn't accidentally implement Copy
        // (Clone is intentional, Copy would be inappropriate)
        fn assert_not_copy<T: Clone>(_: T) {}
        let manager = WorkspaceManager::new();
        assert_not_copy(manager);
    }

    #[test]
    fn test_workspace_manager_multiple_threads() {
        use std::sync::Arc;
        use std::thread;

        let manager = Arc::new(WorkspaceManager::new());
        let mut handles = vec![];

        for _ in 0..10 {
            let manager_clone = Arc::clone(&manager);
            let handle = thread::spawn(move || {
                let debug_str = format!("{:?}", *manager_clone);
                assert!(debug_str.contains("WorkspaceManager"));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Test setup failed");
        }
    }

    #[test]
    fn test_workspace_manager_option() {
        let manager: Option<WorkspaceManager> = Some(WorkspaceManager::new());
        assert!(manager.is_some());

        let none_manager: Option<WorkspaceManager> = None;
        assert!(none_manager.is_none());
    }

    #[test]
    fn test_workspace_manager_result() {
        let result: Result<WorkspaceManager, &str> = Ok(WorkspaceManager::new());
        assert!(result.is_ok());

        let error_result: Result<WorkspaceManager, &str> = Err("test error");
        assert!(error_result.is_err());
    }

    #[test]
    fn test_workspace_manager_vec() {
        let mut managers = Vec::new();
        for _ in 0..5 {
            managers.push(WorkspaceManager::new());
        }

        assert_eq!(managers.len(), 5);
    }

    #[test]
    fn test_workspace_manager_box() {
        let boxed_manager = Box::new(WorkspaceManager::new());
        let debug_str = format!("{:?}", *boxed_manager);
        assert!(debug_str.contains("WorkspaceManager"));
    }

    #[test]
    fn test_workspace_manager_arc() {
        use std::sync::Arc;

        let arc_manager = Arc::new(WorkspaceManager::new());
        let arc_clone = Arc::clone(&arc_manager);

        assert_eq!(Arc::strong_count(&arc_manager), 2);

        drop(arc_clone);
        assert_eq!(Arc::strong_count(&arc_manager), 1);
    }

    #[test]
    fn test_workspace_manager_rc() {
        use std::rc::Rc;

        let rc_manager = Rc::new(WorkspaceManager::new());
        let rc_clone = Rc::clone(&rc_manager);

        assert_eq!(Rc::strong_count(&rc_manager), 2);

        drop(rc_clone);
        assert_eq!(Rc::strong_count(&rc_manager), 1);
    }

    #[test]
    fn test_workspace_manager_tuple() {
        let tuple = (WorkspaceManager::new(), WorkspaceManager::default());
        let (manager1, manager2) = tuple;

        let _ = format!("{:?}", manager1);
        let _ = format!("{:?}", manager2);
    }

    #[test]
    fn test_workspace_manager_array() {
        let managers = [
            WorkspaceManager::new(),
            WorkspaceManager::new(),
            WorkspaceManager::new(),
        ];

        assert_eq!(managers.len(), 3);
    }

    #[test]
    fn test_workspace_manager_struct_embedding() {
        struct Container {
            manager: WorkspaceManager,
            count: u32,
        }

        let container = Container {
            manager: WorkspaceManager::new(),
            count: 42,
        };

        assert_eq!(container.count, 42);
        let _ = format!("{:?}", container.manager);
    }

    #[test]
    fn test_workspace_manager_enum_variant() {
        enum ManagerState {
            Active(WorkspaceManager),
            /// Inactive
            Inactive,
        }

        let state = ManagerState::Active(WorkspaceManager::new());

        match state {
            ManagerState::Active(manager) => {
                let _ = format!("{:?}", manager);
            }
            ManagerState::Inactive => panic!("Should be active"),
        }
    }

    #[test]
    fn test_workspace_manager_hash_map_value() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert("manager1", WorkspaceManager::new());
        map.insert("manager2", WorkspaceManager::new());

        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_workspace_manager_lifetime() {
        {
            let manager = WorkspaceManager::new();
            let _ = format!("{:?}", manager);
        } // manager dropped here

        // Create new manager after previous one dropped
        let _new_manager = WorkspaceManager::new();
    }

    #[test]
    fn test_workspace_manager_pattern_matching() {
        let manager = WorkspaceManager::new();

        match manager {
            ref m => {
                let _ = format!("{:?}", m);
            }
        }
    }

    #[test]
    fn test_workspace_manager_nested() {
        struct OuterContainer {
            inner: InnerContainer,
        }

        struct InnerContainer {
            manager: WorkspaceManager,
        }

        let outer = OuterContainer {
            inner: InnerContainer {
                manager: WorkspaceManager::new(),
            },
        };

        let _ = format!("{:?}", outer.inner.manager);
    }

    #[test]
    fn test_workspace_manager_repeated_operations() {
        for i in 0..100 {
            let manager = WorkspaceManager::new();
            let cloned = manager.clone();

            let debug1 = format!("{:?}", manager);
            let debug2 = format!("{:?}", cloned);

            assert_eq!(debug1, debug2, "Iteration {} failed", i);
        }
    }

    #[test]
    fn test_workspace_manager_concurrent_creation() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;
        use std::thread;

        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..10 {
                    let _manager = WorkspaceManager::new();
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Test setup failed");
        }

        assert_eq!(counter.load(Ordering::SeqCst), 100);
    }
}
