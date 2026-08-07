// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **G68 L1 — Platform link abstraction.**
//!
//! Symlink on Unix, junction/hard-link on Windows. Callers never import
//! `std::os::unix::fs::symlink` directly — this module is the single surface.

use std::io;
use std::path::Path;

/// Create a platform-appropriate filesystem link (symlink on Unix, hard-link on Windows).
///
/// `target` is the destination the link points to (relative or absolute);
/// `link_path` is the path of the new link to create.
///
/// # Errors
///
/// Returns [`io::Error`] if the link cannot be created (permissions, existing file, etc.).
pub fn create_link(target: &Path, link_path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(target, link_path)
    }
    #[cfg(not(unix))]
    {
        std::fs::hard_link(target, link_path)
    }
}

/// Remove a link if it exists and is a symbolic link (Unix) or file (Windows).
///
/// Returns `Ok(true)` if something was removed, `Ok(false)` if the path did not exist
/// or was not a link.
///
/// # Errors
///
/// Returns [`io::Error`] if the link exists but cannot be removed.
pub fn remove_link(link_path: &Path) -> io::Result<bool> {
    match std::fs::symlink_metadata(link_path) {
        Ok(meta) => {
            let is_link = {
                #[cfg(unix)]
                {
                    meta.file_type().is_symlink()
                }
                #[cfg(not(unix))]
                {
                    meta.is_file()
                }
            };
            if is_link {
                std::fs::remove_file(link_path)?;
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}

/// Returns `true` when `path` is a symbolic link (Unix) or a reparse point / junction (Windows).
///
/// Returns `false` if the path does not exist or metadata cannot be read.
#[must_use]
pub fn is_link(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|m| {
        #[cfg(unix)]
        {
            m.file_type().is_symlink()
        }
        #[cfg(not(unix))]
        {
            m.file_type().is_symlink()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_remove_link() {
        let dir = std::env::temp_dir().join("nestgate_platform_links_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let target = dir.join("target_file");
        std::fs::write(&target, b"hello").unwrap();

        let link = dir.join("the_link");
        create_link(&target, &link).unwrap();
        assert!(is_link(&link));

        let removed = remove_link(&link).unwrap();
        assert!(removed);
        assert!(!is_link(&link));

        let removed_again = remove_link(&link).unwrap();
        assert!(!removed_again);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn remove_nonexistent_is_ok_false() {
        let result = remove_link(Path::new("/tmp/nestgate_platform_links_nonexistent_42"));
        assert!(matches!(result, Ok(false)));
    }
}
