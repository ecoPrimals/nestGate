// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **G68 L3 — Platform process and host abstractions.**
//!
//! PID liveness probing, hostname resolution, and UID-based path derivation.
//! Callers never call `rustix::process::test_kill_process`, `rustix::system::uname`,
//! or `rustix::process::getuid` directly from RPC/config crates.

/// Check if a process with the given PID is alive (Unix: `kill(pid, 0)`; Windows: always `false`).
#[must_use]
pub fn is_pid_alive(pid: i32) -> bool {
    #[cfg(unix)]
    {
        let Some(rpid) = rustix::process::Pid::from_raw(pid) else {
            return false;
        };
        match rustix::process::test_kill_process(rpid) {
            Ok(()) => true,
            Err(e) if e == rustix::io::Errno::PERM => true,
            Err(_) => false,
        }
    }
    #[cfg(not(unix))]
    {
        let _ = pid;
        false
    }
}

/// Return the system hostname (Unix: `uname().nodename()`; Windows: `COMPUTERNAME` env var).
#[must_use]
pub fn hostname() -> String {
    #[cfg(unix)]
    {
        rustix::system::uname()
            .nodename()
            .to_string_lossy()
            .into_owned()
    }
    #[cfg(not(unix))]
    {
        std::env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".into())
    }
}

/// Platform-aware runtime base directory for socket/pipe discovery.
///
/// Unix: `$XDG_RUNTIME_DIR` or `/run/user/{uid}`.
/// Windows: `$TEMP` or system temp directory.
#[must_use]
pub fn runtime_base_dir() -> String {
    if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
        return xdg;
    }
    #[cfg(unix)]
    {
        format!("/run/user/{}", super::uid::get_current_uid())
    }
    #[cfg(not(unix))]
    {
        std::env::var("TEMP").unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_process_is_alive() {
        #[expect(clippy::cast_possible_wrap, reason = "PID fits i32 on all supported platforms")]
        let pid = std::process::id() as i32;
        #[cfg(unix)]
        assert!(is_pid_alive(pid));
        #[cfg(not(unix))]
        let _ = pid;
    }

    #[test]
    fn hostname_is_nonempty() {
        let h = hostname();
        assert!(!h.is_empty());
    }

    #[test]
    fn runtime_base_dir_is_nonempty() {
        let dir = runtime_base_dir();
        assert!(!dir.is_empty());
    }
}
