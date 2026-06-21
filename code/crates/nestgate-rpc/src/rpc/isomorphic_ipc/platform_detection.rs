// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Platform Constraint Detection
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
pub fn is_platform_constraint(error: &anyhow::Error) -> bool {
    if let Some(io_err) = find_io_error(error) {
        match io_err.kind() {
            // Permission denied - check for SELinux/Android restrictions OR
            // explicit PRIMAL_BIND_MODE=fallback (grapheneGate deploy)
            ErrorKind::PermissionDenied => {
                if is_bind_mode_fallback() {
                    debug!(
                        "Permission denied + PRIMAL_BIND_MODE=fallback — treating as platform constraint"
                    );
                    return true;
                }
                let is_selinux = is_selinux_enforcing();
                debug!("Permission denied - SELinux enforcing: {}", is_selinux);
                is_selinux
            }

            // Platform doesn't support Unix sockets
            ErrorKind::Unsupported => {
                debug!("Platform lacks Unix socket support");
                true
            }

            // Address family not supported (common on limited platforms)
            ErrorKind::AddrNotAvailable => {
                debug!("Address family not available");
                true
            }

            // All other errors are real failures
            _ => {
                debug!("Real error (not platform constraint): {:?}", io_err.kind());
                false
            }
        }
    } else {
        // Non-IO errors are not platform constraints
        debug!("Non-IO error (not platform constraint)");
        false
    }
}

/// Walk the full anyhow error chain to find an `std::io::Error`.
///
/// `anyhow::Error::downcast_ref` only checks the top-level type. When
/// the IO error is wrapped via `.context()` or a middleware error type,
/// the direct downcast misses it. This walks `source()` links so that
/// `is_platform_constraint` works even when the IO error is nested
/// inside `anyhow::anyhow!("Failed to bind: {e}")` or similar wrappers.
fn find_io_error(error: &anyhow::Error) -> Option<&std::io::Error> {
    let mut current: &dyn std::error::Error = error.as_ref();
    loop {
        if let Some(io_err) = current.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }
        current = current.source()?;
    }
}

/// Check if `PRIMAL_BIND_MODE` requests fallback behavior.
///
/// When `deploy_pixel.sh` exports `PRIMAL_BIND_MODE=fallback`, any
/// `PermissionDenied` on UDS bind should trigger TCP fallback regardless
/// of whether `SELinux` detection succeeds (Android sandbox may block
/// `/sys/fs/selinux/enforce` reads).
fn is_bind_mode_fallback() -> bool {
    matches!(
        std::env::var("PRIMAL_BIND_MODE")
            .unwrap_or_default()
            .to_lowercase()
            .as_str(),
        "fallback" | "auto" | "tcp_only" | "tcp"
    )
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
                debug!("SELinux detected: ENFORCING (likely blocking Unix sockets)");
                true
            }
            Ok(0) => {
                debug!("SELinux detected: PERMISSIVE (not blocking)");
                false
            }
            Ok(v) => {
                debug!("SELinux unknown mode: {}", v);
                false
            }
            Err(e) => {
                debug!("SELinux enforce value parse error: {}", e);
                false
            }
        },
        Err(e) => {
            debug!("SELinux not detected: {}", e);
            false
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Error as IoError;

    #[test]
    fn selinux_detection_does_not_panic() {
        let _result = is_selinux_enforcing();
    }

    #[test]
    fn unsupported_error_is_platform_constraint() {
        let io_err = IoError::new(ErrorKind::Unsupported, "Unix sockets not supported");
        let err = anyhow::Error::new(io_err);
        assert!(is_platform_constraint(&err));
    }

    #[test]
    fn addr_not_available_is_platform_constraint() {
        let io_err = IoError::new(ErrorKind::AddrNotAvailable, "Address family not supported");
        let err = anyhow::Error::new(io_err);
        assert!(is_platform_constraint(&err));
    }

    #[test]
    fn permission_denied_with_selinux() {
        let io_err = IoError::new(ErrorKind::PermissionDenied, "Permission denied");
        let err = anyhow::Error::new(io_err);
        let _result = is_platform_constraint(&err);
    }

    #[test]
    fn other_io_errors_not_platform_constraint() {
        let io_err = IoError::new(ErrorKind::NotFound, "File not found");
        let err = anyhow::Error::new(io_err);
        assert!(!is_platform_constraint(&err));
    }

    #[test]
    fn non_io_error_not_platform_constraint() {
        let err = anyhow::anyhow!("Some other error");
        assert!(!is_platform_constraint(&err));
    }

    // NG-DOWNCAST-01: io::Error wrapped via .context() was invisible to
    // the old direct downcast_ref. Verify chain-walking finds it.
    #[test]
    fn context_wrapped_io_error_detected() {
        let io_err = IoError::new(ErrorKind::Unsupported, "sockets not supported");
        let err: anyhow::Error = anyhow::Error::new(io_err).context("Failed to bind Unix socket");
        assert!(
            is_platform_constraint(&err),
            "chain-walking should find io::Error through .context()"
        );
    }

    #[test]
    fn anyhow_anyhow_stringified_io_error_not_detected() {
        let io_err = IoError::new(ErrorKind::Unsupported, "sockets not supported");
        let err = anyhow::anyhow!("Failed to bind: {io_err}");
        assert!(
            !is_platform_constraint(&err),
            "stringified io errors lose type information"
        );
    }

    #[test]
    fn deeply_nested_io_error_detected() {
        let io_err = IoError::new(ErrorKind::AddrNotAvailable, "no AF_UNIX");
        let err: anyhow::Error = anyhow::Error::new(io_err)
            .context("socket preparation")
            .context("server startup");
        assert!(is_platform_constraint(&err));
    }

    #[test]
    fn find_io_error_returns_none_for_non_io() {
        let err = anyhow::anyhow!("not an io error");
        assert!(find_io_error(&err).is_none());
    }

    #[test]
    fn find_io_error_returns_correct_kind() {
        let io_err = IoError::new(ErrorKind::PermissionDenied, "denied");
        let err: anyhow::Error = anyhow::Error::new(io_err).context("bind failed");
        let found = find_io_error(&err).expect("should find io::Error");
        assert_eq!(found.kind(), ErrorKind::PermissionDenied);
    }

    #[test]
    fn bind_mode_fallback_triggers_on_permission_denied() {
        let io_err = IoError::new(ErrorKind::PermissionDenied, "denied");
        let err: anyhow::Error = anyhow::Error::new(io_err).context("Failed to bind Unix socket");
        temp_env::with_vars([("PRIMAL_BIND_MODE", Some("fallback"))], || {
            assert!(
                is_platform_constraint(&err),
                "fallback mode should treat PermissionDenied as constraint"
            );
        });
    }

    #[test]
    fn tcp_only_mode_triggers_on_permission_denied() {
        let io_err = IoError::new(ErrorKind::PermissionDenied, "denied");
        let err: anyhow::Error = anyhow::Error::new(io_err).context("bind failed");
        temp_env::with_vars([("PRIMAL_BIND_MODE", Some("tcp_only"))], || {
            assert!(
                is_platform_constraint(&err),
                "tcp_only mode should treat PermissionDenied as constraint"
            );
        });
    }
}
