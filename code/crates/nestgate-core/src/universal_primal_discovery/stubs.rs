/// **DEPRECATED**: Universal Primal Discovery Stub Implementations
///
/// **This module has been moved to `crate::dev_stubs::primal_discovery`.**
///
/// # Migration
///
/// **BEFORE** (deprecated):
/// ```rust,ignore
/// use nestgate_core::universal_primal_discovery::stubs::*;
/// ```
///
/// **AFTER** (new location):
/// ```rust,ignore
/// use nestgate_core::dev_stubs::primal_discovery::*;
/// // or
/// use nestgate_core::dev_stubs::*;
/// ```
///
/// # Deprecation Timeline
///
/// - **Moved**: November 10, 2025 (v0.11.2)
/// - **Removal**: May 2026 (v0.12.0)
///
/// **⚠️ DEVELOPMENT ONLY**: This module is only available with `dev-stubs` feature
#[deprecated(
    since = "0.11.2",
    note = "Moved to crate::dev_stubs::primal_discovery. \
            Update imports: use nestgate_core::dev_stubs::primal_discovery::*; \
            This location will be removed in v0.12.0 (May 2026)."
)]
// Re-export everything from the new location - no duplicate implementations
pub use crate::dev_stubs::primal_discovery::*;
