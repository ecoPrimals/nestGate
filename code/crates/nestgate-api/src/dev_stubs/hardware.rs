// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **HARDWARE TUNING — DEV STUB RE-EXPORTS**
//!
//! Re-exports procfs-backed helpers from [`crate::handlers::hardware_tuning::stub_helpers`].
//! Prefer importing [`crate::handlers::hardware_tuning::stub_helpers`] directly in new code.

pub use crate::handlers::hardware_tuning::stub_helpers::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reexported_helpers_are_callable() {
        let _ = create_stub_system_profile();
        let _ = create_stub_cpu_info();
    }
}
