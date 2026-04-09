// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Capability symlink tests (CAPABILITY_BASED_DISCOVERY_STANDARD) — Unix only.

use super::*;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn biomeos_parent_detected() {
    let p = PathBuf::from("/run/user/1000/biomeos/nestgate.sock");
    assert!(socket_parent_is_biomeos_standard_dir(&p));
}

#[test]
fn custom_socket_dir_name_is_not_biomeos_standard() {
    let p = PathBuf::from("/tmp/biomeos-test-dir/nestgate.sock");
    assert!(!socket_parent_is_biomeos_standard_dir(&p));
}

#[test]
fn install_creates_symlink_under_biomeos_only() {
    let root = tempdir().expect("tempdir");
    let biomeos = root.path().join("biomeos");
    fs::create_dir_all(&biomeos).expect("mkdir");
    let sock = biomeos.join("nestgate.sock");
    fs::write(&sock, b"").expect("touch");

    assert!(install_storage_capability_symlink(&sock, "standalone"));
    let link = biomeos.join(STORAGE_CAPABILITY_SOCK_NAME);
    assert!(link.exists());
    assert!(
        fs::symlink_metadata(&link)
            .expect("meta")
            .file_type()
            .is_symlink()
    );

    remove_storage_capability_symlink(&sock, "standalone", true);
    assert!(!link.exists());
}

#[test]
fn install_creates_family_scoped_symlink() {
    let root = tempdir().expect("tempdir");
    let biomeos = root.path().join("biomeos");
    fs::create_dir_all(&biomeos).expect("mkdir");
    let sock = biomeos.join("nestgate-myfamily.sock");
    fs::write(&sock, b"").expect("touch");

    assert!(install_storage_capability_symlink(&sock, "myfamily"));
    let link = biomeos.join("storage-myfamily.sock");
    assert!(link.exists());
    assert!(
        fs::symlink_metadata(&link)
            .expect("meta")
            .file_type()
            .is_symlink()
    );

    remove_storage_capability_symlink(&sock, "myfamily", true);
    assert!(!link.exists());
}

#[test]
fn install_skips_non_biomeos_directory() {
    let root = tempdir().expect("tempdir");
    let other = root.path().join("other");
    fs::create_dir_all(&other).expect("mkdir");
    let sock = other.join("nestgate.sock");

    assert!(!install_storage_capability_symlink(&sock, "standalone"));
    assert!(!other.join(STORAGE_CAPABILITY_SOCK_NAME).exists());
}

/// Parent directory must be named exactly `biomeos`, not e.g. `biomeos-extra`.
#[test]
fn symlink_skipped_when_socket_not_under_biomeos_directory() {
    let root = tempdir().expect("tempdir");
    let not_biomeos = root.path().join("biomeos-extra");
    fs::create_dir_all(&not_biomeos).expect("mkdir");
    let sock = not_biomeos.join("nestgate.sock");
    fs::write(&sock, b"").expect("touch");

    assert!(!socket_parent_is_biomeos_standard_dir(&sock));
    assert!(!install_storage_capability_symlink(&sock, "standalone"));
    assert!(!not_biomeos.join(STORAGE_CAPABILITY_SOCK_NAME).exists());
}

#[test]
fn storage_capability_symlink_guard_drop_removes_link_when_installed() {
    let root = tempdir().expect("tempdir");
    let biomeos = root.path().join("biomeos");
    fs::create_dir_all(&biomeos).expect("mkdir");
    let sock = biomeos.join("nestgate.sock");
    fs::write(&sock, b"").expect("touch");

    {
        let _guard = StorageCapabilitySymlinkGuard::new(&sock, "standalone");
        let link = biomeos.join(STORAGE_CAPABILITY_SOCK_NAME);
        assert!(link.exists());
    }
    assert!(!biomeos.join(STORAGE_CAPABILITY_SOCK_NAME).exists());
}

#[test]
fn remove_noops_when_installed_flag_false() {
    let root = tempdir().expect("tempdir");
    let biomeos = root.path().join("biomeos");
    fs::create_dir_all(&biomeos).expect("mkdir");
    let sock = biomeos.join("nestgate.sock");
    fs::write(&sock, b"").expect("touch");
    assert!(install_storage_capability_symlink(&sock, "standalone"));
    let link = biomeos.join(STORAGE_CAPABILITY_SOCK_NAME);
    remove_storage_capability_symlink(&sock, "standalone", false);
    assert!(link.exists(), "symlink preserved when installed=false");
    remove_storage_capability_symlink(&sock, "standalone", true);
    assert!(!link.exists());
}

/// Cleanup must not run unless we recorded `install_storage_capability_symlink` as successful.
#[test]
fn cleanup_only_runs_when_symlink_was_installed() {
    let root = tempdir().expect("tempdir");
    let biomeos = root.path().join("biomeos");
    fs::create_dir_all(&biomeos).expect("mkdir");
    let sock = biomeos.join("nestgate.sock");
    fs::write(&sock, b"").expect("touch");
    let link = biomeos.join(STORAGE_CAPABILITY_SOCK_NAME);
    std::os::unix::fs::symlink("nestgate.sock", &link).expect("manual symlink");

    remove_storage_capability_symlink(&sock, "standalone", false);
    assert!(
        link.exists(),
        "with installed=false, remove must not unlink (even if link exists)"
    );

    remove_storage_capability_symlink(&sock, "standalone", true);
    assert!(!link.exists());
}
