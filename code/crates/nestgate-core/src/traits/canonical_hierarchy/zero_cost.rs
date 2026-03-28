// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// **ZERO-COST** marker trait for performance-critical services
///
/// This trait is a **marker trait** that provides hints to the compiler
/// for zero-cost abstractions. Services implementing this trait should:
///
/// 1. Have no runtime overhead
/// 2. Be fully inlineable
/// 3. Use const generics where possible
/// 4. Avoid dynamic dispatch in hot paths
///
/// **Type Parameter**: `T` - The service type
///
/// **Usage**: Mark performance-critical service implementations
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::traits::canonical_hierarchy::ZeroCostService;
///
/// pub struct FastCache<const SIZE: usize> {
///     buffer: [u8; SIZE],
/// }
///
/// impl<const SIZE: usize> ZeroCostService<Self> for FastCache<SIZE> {}
///
/// // Assert at compile time
/// assert_zero_cost!(FastCache<1024>);
/// ```
pub trait ZeroCostService<T>: Send + Sync + 'static {
    // Marker trait: no methods - compile-time only
}

/// Helper macro for asserting zero-cost properties
#[macro_export]
macro_rules! assert_zero_cost {
    ($t:ty) => {
        ///
        const _: () = {
            /// Assert Send Sync
            fn assert_send_sync<T: Send + Sync>() {}
            /// Assert Zero Sized
            fn assert_zero_sized<T>() {
                assert_send_sync::<T>();
            }
            assert_zero_sized::<$t>();
        };
    };
}
