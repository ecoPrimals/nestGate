// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # 🔍 Platform Constraint Detection
//!
//! **RUNTIME DETECTION**: Determines if errors are platform constraints vs real errors
//!
//! ## Philosophy
//!
//! Platform constraints are **DATA** (detected at runtime), not **CONFIG** (hardcoded at compile time).
//!
//! This module implements the "DETECT" phase of Try→Detect→Adapt→Succeed:
//! - Analyzes errors to identify platform limitations
//! - Distinguishes platform constraints from real failures
//! - Enables automatic adaptation (TCP fallback)
//!
//! ## Examples
//!
//! **Platform Constraint** (adapt):
//! ```text
//! ErrorKind::PermissionDenied + SELinux enforcing
//! → Android blocking Unix sockets → Use TCP fallback
//! ```
//!
//! **Real Error** (fail):
//! ```text
//! ErrorKind::PermissionDenied + SELinux disabled
//! → Actual permission issue → Report to user
//! ```
//!
//! ## Reference
//!
//! Pattern validated in orchestration provider v3.33.0

use std::io::ErrorKind;
use tracing::debug;

/// Detects if an error is due to platform constraints (not a real error)
///
/// **Platform constraints** are environmental limitations that require adaptation:
/// - `SELinux` blocking Unix sockets (Android)
/// - Platform lacking Unix socket support
/// - Address family not supported
///
/// **Real errors** are actual failures that should be reported:
/// - Disk full
/// - Insufficient permissions (non-SELinux)
/// - Network failures
///
/// # Arguments
///
/// * `error` - The error to analyze
///
/// # Returns
///
/// * `true` - Error is a platform constraint (should adapt)
/// * `false` - Error is a real failure (should report)
#[expect(clippy::option_if_let_else)] // Readable IO-kind ladder; `map_or_else` obscures match flow.
pub fn is_platform_constraint(error: &anyhow::Error) -> bool {
    if let Some(io_err) = error.downcast_ref::<std::io::Error>() {
        match io_err.kind() {
            // Permission denied - check for SELinux/Android restrictions
            ErrorKind::PermissionDenied => {
                let is_selinux = is_selinux_enforcing();
                debug!("🔍 Permission denied - SELinux enforcing: {}", is_selinux);
                is_selinux
            }

            // Platform doesn't support Unix sockets
            ErrorKind::Unsupported => {
                debug!("🔍 Platform lacks Unix socket support");
                true
            }

            // Address family not supported (common on limited platforms)
            ErrorKind::AddrNotAvailable => {
                debug!("🔍 Address family not available");
                true
            }

            // All other errors are real failures
            _ => {
                debug!(
                    "🔍 Real error (not platform constraint): {:?}",
                    io_err.kind()
                );
                false
            }
        }
    } else {
        // Non-IO errors are not platform constraints
        debug!("🔍 Non-IO error (not platform constraint)");
        false
    }
}

/// Check if `SELinux` is enforcing (Android/Linux)
///
/// **`SELinux` (Security-Enhanced Linux)** is a mandatory access control system.
/// On Android, it often blocks Unix socket creation in app sandboxes.
///
/// # Detection
///
/// Reads `/sys/fs/selinux/enforce`:
/// - `1` = Enforcing (blocks operations)
/// - `0` = Permissive (logs only)
/// - Missing file = Not present
///
/// # Returns
///
/// * `true` - `SELinux` is enforcing (likely blocking Unix sockets)
/// * `false` - `SELinux` not enforcing or not present
fn is_selinux_enforcing() -> bool {
    match std::fs::read_to_string("/sys/fs/selinux/enforce") {
        Ok(contents) => match contents.trim().parse::<u8>() {
            Ok(1) => {
                debug!("✅ SELinux detected: ENFORCING (likely blocking Unix sockets)");
                true
            }
            Ok(0) => {
                debug!("ℹ️  SELinux detected: PERMISSIVE (not blocking)");
                false
            }
            Ok(v) => {
                debug!("⚠️  SELinux unknown mode: {}", v);
                false
            }
            Err(e) => {
                debug!("⚠️  SELinux enforce value parse error: {}", e);
                false
            }
        },
        Err(e) => {
            debug!("ℹ️  SELinux not detected: {}", e);
            false
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Error as IoError;

    #[test]
    fn test_selinux_detection_does_not_panic() {
        // Should not panic regardless of platform
        let result = is_selinux_enforcing();
        // Result depends on platform, but function should complete
        println!("SELinux enforcing: {}", result);
    }

    #[test]
    fn test_unsupported_error_is_platform_constraint() {
        let io_err = IoError::new(ErrorKind::Unsupported, "Unix sockets not supported");
        let err = anyhow::Error::new(io_err);

        assert!(is_platform_constraint(&err));
    }

    #[test]
    fn test_addr_not_available_is_platform_constraint() {
        let io_err = IoError::new(ErrorKind::AddrNotAvailable, "Address family not supported");
        let err = anyhow::Error::new(io_err);

        assert!(is_platform_constraint(&err));
    }

    #[test]
    fn test_permission_denied_with_selinux() {
        let io_err = IoError::new(ErrorKind::PermissionDenied, "Permission denied");
        let err = anyhow::Error::new(io_err);

        // Result depends on whether SELinux is actually enforcing
        let result = is_platform_constraint(&err);
        println!("Permission denied is platform constraint: {}", result);
        // Don't assert - depends on platform
    }

    #[test]
    fn test_other_io_errors_not_platform_constraint() {
        let io_err = IoError::new(ErrorKind::NotFound, "File not found");
        let err = anyhow::Error::new(io_err);

        assert!(!is_platform_constraint(&err));
    }

    #[test]
    fn test_non_io_error_not_platform_constraint() {
        let err = anyhow::anyhow!("Some other error");

        assert!(!is_platform_constraint(&err));
    }
}
