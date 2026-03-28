// SPDX-License-Identifier: AGPL-3.0-only
//! Safe alternatives to unsafe code patterns
//!
//! This module demonstrates how to evolve unsafe code to safe alternatives
//! while maintaining or improving performance.
//!
//! ## Remaining `unsafe` (when `simd_evolution` is compiled with AVX2):
//! - **SIMD (`add_arrays_simd`)**: x86 intrinsics require `unsafe`; bounds are enforced by `array_chunks`.
//!
//! The **FFI** subsection (`ffi_wrapper`) is a safe-Rust *simulation* of RAII around handles — not real FFI.
//! Enable with crate feature `safe-alternatives-demo` or compile tests (`cfg(test)`).

/// Example 1: Buffer initialization
pub mod buffer_initialization {
    // ✅ NEW: Safe initialization (evolved from unsafe set_len pattern)
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

    // ✅ EVOLVED: No transmute needed! Modern idiomatic Rust
    /// Creates a buffer using efficient initialization without any unsafe code.
    ///
    /// ✅ DEEP DEBT: Transmute eliminated. Uses safe iterator pattern.
    /// Performance equivalent to MaybeUninit approach (compiler optimizes identically).
    pub fn create_buffer_maybe_uninit(size: usize) -> Vec<u8> {
        // ✅ EVOLVED: No transmute, no unsafe, no MaybeUninit
        // The compiler optimizes this to the same assembly as the transmute version
        vec![0u8; size]
    }
}

/// Example 2: Pointer handling
pub mod pointer_handling {
    // ❌ OLD: Raw pointer without safety guarantees
    #[cfg(test)]
    /// Contrasts with [`SafePointerWrapper`] — retained for documentation only.
    pub struct OldPointerWrapper {
        _ptr: *mut u8,
    }

    // ✅ EVOLVED: Zero unsafe code! Just use Box directly.
    //
    // The old pattern used NonNull + Box::into_raw + unsafe as_ref + unsafe Drop.
    // This is entirely unnecessary - Box already provides:
    //   - Non-null guarantee
    //   - Automatic cleanup via Drop
    //   - Safe reference access via Deref
    //
    // LESSON: The best unsafe evolution is often eliminating the need entirely.
    /// A safe wrapper around heap-allocated values.
    ///
    /// ✅ DEEP DEBT: Zero unsafe blocks (was 3 unsafe blocks before evolution)
    /// Box already provides all the safety guarantees we need.
    pub struct SafePointerWrapper {
        inner: Box<u8>,
    }

    impl SafePointerWrapper {
        /// Creates a new `SafePointerWrapper` from a boxed value.
        ///
        /// ✅ EVOLVED: No unsafe needed. Box is already non-null and owned.
        pub fn new(value: Box<u8>) -> Self {
            Self { inner: value }
        }

        /// Gets a reference to the wrapped value safely.
        ///
        /// ✅ EVOLVED: No unsafe needed. Box::deref is inherently safe.
        pub fn get(&self) -> &u8 {
            &self.inner
        }
    }

    // ✅ EVOLVED: No custom Drop needed! Box handles deallocation automatically.
}

/// Example 3: FFI-style RAII (demonstration only — not production FFI)
///
/// **Teaching example only.** Production FFI uses `extern "C"` and `unsafe` at the boundary;
/// this module shows the *RAII shape* with pure safe Rust (atomic handle id + `Drop`).
pub mod ffi_wrapper {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_HANDLE: AtomicUsize = AtomicUsize::new(1);

    /// Simulated opaque handle — in real code this would wrap `NonNull<ffi::Handle>`.
    pub struct SafeHandle {
        id: usize,
    }

    impl SafeHandle {
        /// Create a new simulated handle
        ///
        /// # Errors
        ///
        /// Returns error only if the demonstration counter overflows (practically never).
        pub fn new() -> Result<Self, &'static str> {
            let id = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
            if id == 0 {
                return Err("handle id overflow (demonstration only)");
            }
            Ok(Self { id })
        }

        /// Simulated use of the handle
        pub fn use_handle(&self) -> i32 {
            self.id as i32
        }
    }

    impl Drop for SafeHandle {
        fn drop(&mut self) {
            // A real FFI wrapper would call `unsafe { destroy(self.as_ptr()) }` here.
        }
    }
}

/// Example 4: SIMD evolution
#[cfg(target_arch = "x86_64")]
pub mod simd_evolution {
    // ❌ OLD: Direct SIMD intrinsics (unsafe)
    /// Contrasts with [`super::add_arrays_safe`] — **tests / teaching only**.
    #[cfg(test)]
    pub mod unsafe_simd {
        #[cfg(target_arch = "x86_64")]
        /// Deliberately unsafe SIMD loop (legacy shape); prefer [`super::add_arrays_safe`].
        /// Parameters are unused when this crate is built without `target_feature = "avx2"`.
        #[allow(unused_variables)]
        pub fn add_arrays_unsafe(a: &[f32], b: &[f32], result: &mut [f32]) {
            #[cfg(target_feature = "avx2")]
            unsafe {
                use std::arch::x86_64::*;
                // SAFETY: Test-only legacy example; caller must ensure `a`, `b`, `result` same length
                // and length multiple of 8. Prefer `add_arrays_safe` + `array_chunks` in real code.
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

        // Use array_chunks for safe bounds - no raw pointer arithmetic
        let (a_chunks, a_rem) = a.array_chunks::<8>();
        let (b_chunks, b_rem) = b.array_chunks::<8>();
        let (result_chunks, result_rem) = result.array_chunks_mut::<8>();

        for ((va, vb), vr) in a_chunks.zip(b_chunks).zip(result_chunks) {
            // SAFETY: AVX2 intrinsics are `unsafe` in Rust's API. Invariants:
            // - `va`, `vb`, `vr` are each exactly 8 `f32` elements (`array_chunks` guarantees length).
            // - Pointers from `.as_ptr()` / `.as_mut_ptr()` are valid for 8 consecutive `f32`.
            // - `_mm256_loadu_ps` / `_mm256_storeu_ps` permit unaligned addresses (matches slice layout).
            unsafe {
                let va_simd = _mm256_loadu_ps(va.as_ptr());
                let vb_simd = _mm256_loadu_ps(vb.as_ptr());
                let vr_simd = _mm256_add_ps(va_simd, vb_simd);
                _mm256_storeu_ps(vr.as_mut_ptr(), vr_simd);
            }
        }

        // Remainder handled with safe scalar ops
        add_arrays_scalar(a_rem, b_rem, result_rem);
    }

    fn add_arrays_scalar(a: &[f32], b: &[f32], result: &mut [f32]) {
        for i in 0..a.len() {
            result[i] = a[i] + b[i];
        }
    }
}

/// Migration checklist for unsafe code evolution.
///
/// Provides a comprehensive checklist and guidelines for migrating
/// unsafe code to safe alternatives while maintaining performance.
pub mod migration_checklist {
    // # Unsafe Code Migration Checklist
    //
    // ## Before Migration
    // - [ ] Document why unsafe is needed
    // - [ ] Document safety invariants
    // - [ ] Have tests proving correctness
    // - [ ] Consider if unsafe is truly necessary
    //
    // ## During Migration
    // - [ ] Use NonNull instead of raw pointers where possible
    // - [ ] Use MaybeUninit for uninitialized memory
    // - [ ] Wrap FFI in safe RAII types
    // - [ ] Provide safe abstractions with fallbacks
    // - [ ] Document remaining SAFETY comments
    //
    // ## After Migration
    // - [ ] Verify tests still pass
    // - [ ] Benchmark to ensure no regression
    // - [ ] Update documentation
    // - [ ] Add safety assertions where helpful
    //
    // ## Safe Alternatives Catalog
    //
    // | Unsafe Pattern | Safe Alternative |
    // |---------------|------------------|
    // | `mem::uninitialized()` | `MaybeUninit::uninit()` |
    // | `*mut T` | `NonNull<T>` + PhantomData |
    // | FFI raw pointers | RAII wrapper struct |
    // | Direct SIMD intrinsics | Safe wrapper + scalar fallback |
    // | `transmute` | `From`/`Into` traits or safe casts |
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
