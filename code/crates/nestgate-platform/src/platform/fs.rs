// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **G68 L2 — Platform filesystem permission abstraction.**
//!
//! Mode bits on Unix, no-op (or future ACL) on Windows. Callers never import
//! `PermissionsExt` or `set_mode()` directly — this module is the single surface.

use std::io;
use std::path::Path;

/// Mark a file as executable for the owning user (Unix: `chmod u+x`; Windows: no-op).
///
/// # Errors
///
/// Returns [`io::Error`] if the file metadata cannot be read or permissions cannot be set.
pub fn set_executable(path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let meta = std::fs::metadata(path)?;
        let mut perms = meta.permissions();
        let mode = perms.mode();
        perms.set_mode(mode | 0o111);
        std::fs::set_permissions(path, perms)
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        Ok(())
    }
}

/// Set exact permission mode bits (Unix: `chmod`; Windows: no-op).
///
/// `mode` uses the standard Unix octal convention (e.g. `0o755`).
///
/// # Errors
///
/// Returns [`io::Error`] if the file metadata cannot be read or permissions cannot be set.
pub fn set_mode(path: &Path, mode: u32) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(mode);
        std::fs::set_permissions(path, perms)
    }
    #[cfg(not(unix))]
    {
        let _ = (path, mode);
        Ok(())
    }
}

/// Read the current permission mode bits (Unix); returns `0o777` on non-Unix.
///
/// # Errors
///
/// Returns [`io::Error`] if the file metadata cannot be read.
pub fn get_mode(path: &Path) -> io::Result<u32> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let meta = std::fs::metadata(path)?;
        Ok(meta.permissions().mode())
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        Ok(0o777)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_executable_on_regular_file() {
        let dir = std::env::temp_dir().join("nestgate_platform_fs_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let file = dir.join("script.sh");
        std::fs::write(&file, b"#!/bin/sh\necho hi").unwrap();

        set_executable(&file).unwrap();

        #[cfg(unix)]
        {
            let mode = get_mode(&file).unwrap();
            assert_ne!(mode & 0o111, 0, "expected executable bits set");
        }

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn set_mode_round_trip() {
        let dir = std::env::temp_dir().join("nestgate_platform_fs_mode_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let file = dir.join("data.txt");
        std::fs::write(&file, b"data").unwrap();

        set_mode(&file, 0o644).unwrap();

        #[cfg(unix)]
        {
            let mode = get_mode(&file).unwrap() & 0o777;
            assert_eq!(mode, 0o644);
        }

        let _ = std::fs::remove_dir_all(&dir);
    }
}
