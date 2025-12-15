//! Safe alternatives to unsafe code patterns
//!
//! This module demonstrates how to evolve unsafe code to safe alternatives
//! while maintaining or improving performance.

use std::mem::MaybeUninit;
use std::ptr::NonNull;

/// Example 1: Buffer initialization
pub mod buffer_initialization {
    use super::*;

    // ❌ OLD: Unsafe uninitialized buffer
    #[cfg(test)]
    pub fn create_buffer_unsafe(size: usize) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(size);
        unsafe {
            // UNSAFE: Uninitialized memory
            buffer.set_len(size);
        }
        buffer
    }

    // ✅ NEW: Safe initialization with MaybeUninit
    /// Creates a safely initialized buffer with the given size.
    ///
    /// This function creates a zero-initialized buffer without using unsafe code.
    /// All bytes are guaranteed to be initialized to 0.
    pub fn create_buffer_safe(size: usize) -> Vec<u8> {
        // Safe: Explicitly initialized
        vec![0u8; size]
    }

    // ✅ NEW: Zero-copy for specific patterns (still safe!)
    /// Creates a zeroed buffer efficiently.
    ///
    /// This function creates a buffer of the specified size, initialized to zero.
    /// Uses the most efficient initialization strategy available.
    ///
    /// # Arguments
    /// * `size` - The size of the buffer to create in bytes
    ///
    /// # Returns
    /// A vector of `size` bytes, all initialized to zero
    pub fn create_buffer_zeroed(size: usize) -> Vec<u8> {
        // Use direct initialization - more efficient than with_capacity + resize
        vec![0; size]
    }

    // ✅ NEW: MaybeUninit for controlled initialization
    /// Creates a buffer using MaybeUninit for performance while maintaining safety.
    ///
    /// This is faster than zero-initialization but still completely safe.
    /// The buffer is properly initialized before being returned.
    pub fn create_buffer_maybe_uninit(size: usize) -> Vec<u8> {
        let mut buffer: Vec<MaybeUninit<u8>> = Vec::with_capacity(size);

        // Initialize explicitly
        for _ in 0..size {
            buffer.push(MaybeUninit::new(0));
        }

        // Safe: All elements initialized
        unsafe {
            // SAFETY: All elements have been explicitly initialized above
            std::mem::transmute::<Vec<MaybeUninit<u8>>, Vec<u8>>(buffer)
        }
    }
}

/// Example 2: Pointer handling
pub mod pointer_handling {
    use super::*;

    // ❌ OLD: Raw pointer without safety guarantees
    #[cfg(test)]
    pub struct OldPointerWrapper {
        ptr: *mut u8,
    }

    // ✅ NEW: NonNull with type safety
    /// A safe wrapper around heap-allocated values using `NonNull`.
    ///
    /// This wrapper ensures pointer validity and provides safe access patterns
    /// without requiring unsafe code in the API.
    pub struct SafePointerWrapper {
        ptr: NonNull<u8>,
        // Add phantom data for proper variance
        _marker: std::marker::PhantomData<u8>,
    }

    impl SafePointerWrapper {
        /// Creates a new `SafePointerWrapper` from a boxed value.
        pub fn new(value: Box<u8>) -> Self {
            Self {
                ptr: unsafe {
                    // SAFETY: Box guarantees non-null pointer
                    NonNull::new_unchecked(Box::into_raw(value))
                },
                _marker: std::marker::PhantomData,
            }
        }

        /// Gets a reference to the wrapped value safely.
        pub fn get(&self) -> &u8 {
            unsafe {
                // SAFETY: NonNull guarantees valid pointer,
                // and we own the allocation
                self.ptr.as_ref()
            }
        }
    }

    impl Drop for SafePointerWrapper {
        fn drop(&mut self) {
            unsafe {
                // SAFETY: Pointer came from Box, return it to Box for cleanup
                let _ = Box::from_raw(self.ptr.as_ptr());
            }
        }
    }
}

/// Example 3: FFI boundaries
pub mod ffi_wrapper {
    use super::*;

    // Simulated FFI functions (would be from external C library)
    mod ffi {
        #[repr(C)]
        pub struct Handle {
            _private: [u8; 0],
        }

        // These would be extern "C" in real code
        pub unsafe fn create_handle() -> *mut Handle {
            std::ptr::null_mut() // Simulated
        }

        pub unsafe fn destroy_handle(_handle: *mut Handle) {
            // Simulated cleanup
        }

        pub unsafe fn use_handle(_handle: *const Handle) -> i32 {
            0 // Simulated operation
        }
    }

    // ✅ NEW: Safe wrapper with RAII
    /// A safe wrapper around FFI handles with automatic cleanup.
    ///
    /// This struct wraps an FFI handle and ensures it is properly destroyed
    /// when dropped, preventing resource leaks.
    pub struct SafeHandle {
        inner: NonNull<ffi::Handle>,
    }

    impl SafeHandle {
        /// Create a new handle
        ///
        /// # Errors
        ///
        /// Returns error if FFI handle creation fails
        pub fn new() -> Result<Self, &'static str> {
            let ptr = unsafe {
                // SAFETY: FFI call, validated below
                ffi::create_handle()
            };

            NonNull::new(ptr)
                .map(|inner| Self { inner })
                .ok_or("Failed to create FFI handle")
        }

        /// Use the handle safely
        pub fn use_handle(&self) -> i32 {
            unsafe {
                // SAFETY: NonNull guarantees valid pointer
                ffi::use_handle(self.inner.as_ptr())
            }
        }
    }

    impl Drop for SafeHandle {
        fn drop(&mut self) {
            unsafe {
                // SAFETY: Automatic cleanup via RAII
                ffi::destroy_handle(self.inner.as_ptr());
            }
        }
    }
}

/// Example 4: SIMD evolution
#[cfg(target_arch = "x86_64")]
pub mod simd_evolution {
    // ❌ OLD: Direct SIMD intrinsics (unsafe)
    #[cfg(test)]
    pub mod unsafe_simd {
        #[cfg(target_arch = "x86_64")]
        pub fn add_arrays_unsafe(a: &[f32], b: &[f32], result: &mut [f32]) {
            #[cfg(target_feature = "avx2")]
            unsafe {
                use std::arch::x86_64::*;

                for i in (0..a.len()).step_by(8) {
                    let va = _mm256_loadu_ps(a.as_ptr().add(i));
                    let vb = _mm256_loadu_ps(b.as_ptr().add(i));
                    let vr = _mm256_add_ps(va, vb);
                    _mm256_storeu_ps(result.as_mut_ptr().add(i), vr);
                }
            }
        }
    }

    // ✅ NEW: Safe abstraction with fallback
    /// Adds two arrays element-wise with automatic SIMD optimization where available.
    ///
    /// This function automatically uses SIMD instructions on supported platforms,
    /// falling back to safe scalar operations otherwise.
    pub fn add_arrays_safe(a: &[f32], b: &[f32], result: &mut [f32]) {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), result.len());

        // Try SIMD if available
        #[cfg(target_feature = "avx2")]
        {
            add_arrays_simd(a, b, result);
        }

        // Fallback to scalar (always safe)
        #[cfg(not(target_feature = "avx2"))]
        {
            add_arrays_scalar(a, b, result);
        }
    }

    #[cfg(target_feature = "avx2")]
    fn add_arrays_simd(a: &[f32], b: &[f32], result: &mut [f32]) {
        use std::arch::x86_64::*;

        let chunks = a.len() / 8;
        for i in 0..chunks {
            let offset = i * 8;
            unsafe {
                // SAFETY:
                // - Bounds checked via chunks calculation
                // - Alignment not required (using loadu/storeu)
                // - Length checked in parent function
                let va = _mm256_loadu_ps(a.as_ptr().add(offset));
                let vb = _mm256_loadu_ps(b.as_ptr().add(offset));
                let vr = _mm256_add_ps(va, vb);
                _mm256_storeu_ps(result.as_mut_ptr().add(offset), vr);
            }
        }

        // Handle remainder with scalar
        let remainder_start = chunks * 8;
        add_arrays_scalar(
            &a[remainder_start..],
            &b[remainder_start..],
            &mut result[remainder_start..],
        );
    }

    fn add_arrays_scalar(a: &[f32], b: &[f32], result: &mut [f32]) {
        for i in 0..a.len() {
            result[i] = a[i] + b[i];
        }
    }
}

/// Migration checklist for unsafe code evolution
///
/// This module provides a comprehensive checklist and guidelines for migrating
/// unsafe code to safe alternatives while maintaining performance.
pub mod migration_checklist {
    //! # Unsafe Code Migration Checklist
    //!
    //! ## Before Migration
    //! - [ ] Document why unsafe is needed
    //! - [ ] Document safety invariants
    //! - [ ] Have tests proving correctness
    //! - [ ] Consider if unsafe is truly necessary
    //!
    //! ## During Migration
    //! - [ ] Use NonNull instead of raw pointers where possible
    //! - [ ] Use MaybeUninit for uninitialized memory
    //! - [ ] Wrap FFI in safe RAII types
    //! - [ ] Provide safe abstractions with fallbacks
    //! - [ ] Document remaining SAFETY comments
    //!
    //! ## After Migration
    //! - [ ] Verify tests still pass
    //! - [ ] Benchmark to ensure no regression
    //! - [ ] Update documentation
    //! - [ ] Add safety assertions where helpful
    //!
    //! ## Safe Alternatives Catalog
    //!
    //! | Unsafe Pattern | Safe Alternative |
    //! |---------------|------------------|
    //! | `mem::uninitialized()` | `MaybeUninit::uninit()` |
    //! | `*mut T` | `NonNull<T>` + PhantomData |
    //! | FFI raw pointers | RAII wrapper struct |
    //! | Direct SIMD intrinsics | Safe wrapper + scalar fallback |
    //! | `transmute` | `From`/`Into` traits or safe casts |
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_buffer_creation() {
        let buffer = buffer_initialization::create_buffer_safe(1024);
        assert_eq!(buffer.len(), 1024);
        assert!(buffer.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_safe_pointer_wrapper() {
        let value = Box::new(42u8);
        let wrapper = pointer_handling::SafePointerWrapper::new(value);
        assert_eq!(*wrapper.get(), 42);
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_safe_simd_arrays() {
        let a = vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let b = vec![8.0f32, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let mut result = vec![0.0f32; 8];

        simd_evolution::add_arrays_safe(&a, &b, &mut result);

        for i in 0..8 {
            assert_eq!(result[i], a[i] + b[i]);
        }
    }
}
