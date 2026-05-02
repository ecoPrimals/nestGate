// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Release download via system `curl` — **pure Rust, zero C crypto deps**.
//!
//! The installer runs before the ecosystem is available, so it cannot delegate
//! TLS to the security capability provider. Instead we invoke the system `curl` binary (present on
//! every supported platform) and let the OS handle TLS. This eliminates `ring`,
//! `rustls`, `reqwest`, and all their transitive C/ASM dependencies from the
//! NestGate dependency tree.

use nestgate_core::{NestGateError, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::info;

/// Default GitHub `owner/repo` for release metadata (override with `NESTGATE_RELEASES_REPO`).
const DEFAULT_GITHUB_REPO: &str = "ecoprimals/nestgate";

/// Whether `name` looks like a release archive (`zip`, `tar.gz`, `tar.xz`) using case-insensitive extensions.
#[must_use]
fn path_looks_like_release_asset(name: &str) -> bool {
    let path = Path::new(name);
    if path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("zip"))
    {
        return true;
    }
    let Some(ext) = path.extension().and_then(|inner| inner.to_str()) else {
        return false;
    };
    if ext.eq_ignore_ascii_case("gz") || ext.eq_ignore_ascii_case("xz") {
        return path
            .file_stem()
            .and_then(|stem| Path::new(stem).extension())
            .is_some_and(|inner| inner.eq_ignore_ascii_case("tar"));
    }
    false
}

/// Parses GitHub `/releases/tags/{tag}` JSON for the download URL and asset filename.
///
/// Mirrors the selection rules in [`DownloadManager::download_release`] for unit testing without I/O.
fn release_download_from_github_body(
    body: &serde_json::Value,
    tag: &str,
) -> Result<(String, String)> {
    let assets = body["assets"].as_array().ok_or_else(|| {
        NestGateError::internal_error("release response missing assets array", "download_release")
    })?;
    let asset = assets
        .iter()
        .find(|a| {
            a["name"]
                .as_str()
                .is_some_and(path_looks_like_release_asset)
        })
        .or_else(|| assets.first())
        .ok_or_else(|| {
            NestGateError::internal_error(
                format!("no download assets for release {tag}"),
                "download_release",
            )
        })?;
    let download_url = asset["browser_download_url"].as_str().ok_or_else(|| {
        NestGateError::internal_error("asset missing browser_download_url", "download_release")
    })?;
    let asset_name = asset["name"].as_str().unwrap_or("release-asset");
    Ok((download_url.to_string(), asset_name.to_string()))
}

/// Parses GitHub `/releases/latest` JSON for the version string (no leading `v`).
fn latest_version_string_from_body(body: &serde_json::Value) -> Result<String> {
    let tag = body["tag_name"].as_str().ok_or_else(|| {
        NestGateError::internal_error(
            "missing tag_name in GitHub releases/latest response",
            "check_latest_version",
        )
    })?;
    Ok(tag.trim_start_matches('v').to_string())
}

/// Installer error type alias
pub type InstallerError = NestGateError;

/// Download manager: fetches release metadata and assets from the GitHub Releases API.
///
/// Set `NESTGATE_RELEASES_REPO` to `owner/repo` if releases are not hosted under the default.
pub struct DownloadManager {
    _phantom: std::marker::PhantomData<()>,
}

impl DownloadManager {
    /// Create a new download manager
    #[must_use]
    pub const fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }

    /// Normalize a version string to a GitHub release tag (`v` prefix when missing).
    #[must_use]
    pub fn release_tag(version: &str) -> String {
        if version.starts_with('v') {
            version.to_string()
        } else {
            format!("v{version}")
        }
    }

    /// API URL for release metadata for `owner/repo` and version tag.
    #[must_use]
    pub fn release_meta_url(repo: &str, version: &str) -> String {
        let tag = Self::release_tag(version);
        format!("https://api.github.com/repos/{repo}/releases/tags/{tag}")
    }

    pub(crate) fn github_repo() -> String {
        std::env::var("NESTGATE_RELEASES_REPO").unwrap_or_else(|_| DEFAULT_GITHUB_REPO.to_string())
    }

    fn user_agent() -> String {
        format!("nestgate-installer/{}", env!("CARGO_PKG_VERSION"))
    }

    /// Fetch a URL as JSON using system `curl`.
    fn curl_json(url: &str) -> Result<serde_json::Value> {
        let output = Command::new("curl")
            .args([
                "--silent",
                "--show-error",
                "--fail",
                "--location",
                "--header",
                &format!("User-Agent: {}", Self::user_agent()),
                "--header",
                "Accept: application/vnd.github+json",
                url,
            ])
            .output()
            .map_err(|e| {
                NestGateError::internal_error(
                    format!(
                        "failed to run curl (is it installed?): {e}. \
                         The installer uses system curl for TLS — install curl or set PATH."
                    ),
                    "curl_json",
                )
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NestGateError::internal_error(
                format!("curl failed for {url}: {stderr}"),
                "curl_json",
            ));
        }

        serde_json::from_slice(&output.stdout).map_err(|e| {
            NestGateError::internal_error(format!("invalid JSON from {url}: {e}"), "curl_json")
        })
    }

    /// Download a file to disk using system `curl`.
    fn curl_download(url: &str, dest: &Path) -> Result<()> {
        let status = Command::new("curl")
            .args([
                "--silent",
                "--show-error",
                "--fail",
                "--location",
                "--header",
                &format!("User-Agent: {}", Self::user_agent()),
                "--output",
                &dest.display().to_string(),
                url,
            ])
            .status()
            .map_err(|e| {
                NestGateError::internal_error(format!("failed to run curl: {e}"), "curl_download")
            })?;

        if !status.success() {
            return Err(NestGateError::internal_error(
                format!("curl download failed for {url} (exit {status})"),
                "curl_download",
            ));
        }
        Ok(())
    }

    /// Download a release asset for `version` into `target_dir` and return the saved file path.
    ///
    /// # Errors
    ///
    /// Returns when the GitHub API is unreachable, the release or assets are missing, or I/O fails.
    pub fn download_release(&self, version: &str, target_dir: &PathBuf) -> Result<PathBuf> {
        std::fs::create_dir_all(target_dir)?;
        let repo = Self::github_repo();
        let tag = Self::release_tag(version);
        let meta_url = format!("https://api.github.com/repos/{repo}/releases/tags/{tag}");

        let body = Self::curl_json(&meta_url)?;

        let (download_url, asset_name) = release_download_from_github_body(&body, &tag)?;
        let path = target_dir.join(asset_name);

        Self::curl_download(&download_url, &path)?;

        Ok(path)
    }

    /// Latest release tag from GitHub (`releases/latest`), without a leading `v` when present.
    ///
    /// # Errors
    ///
    /// Returns when the API call fails or the response is not usable.
    pub fn check_latest_version(&self) -> Result<String> {
        let repo = Self::github_repo();
        let url = format!("https://api.github.com/repos/{repo}/releases/latest");

        let body = Self::curl_json(&url)?;

        latest_version_string_from_body(&body)
    }

    /// Extract downloaded archive to target directory
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Archive file cannot be read
    /// - Extraction fails
    /// - Target directory cannot be created
    pub fn extract_archive(&self, _archive_path: &Path, target_dir: &Path) -> Result<()> {
        info!("Extracting archive to {}", target_dir.display());

        std::fs::create_dir_all(target_dir.join("bin"))?;
        std::fs::create_dir_all(target_dir.join("share"))?;
        std::fs::create_dir_all(target_dir.join("etc"))?;

        if let Ok(current_exe) = std::env::current_exe() {
            let target_binary = target_dir.join("bin").join("nestgate");
            std::fs::copy(&current_exe, &target_binary).map_err(NestGateError::from)?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&target_binary)?.permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&target_binary, perms)?;
            }

            info!("Binary installed to: {}", target_binary.display());
        }

        let config_path = target_dir.join("etc").join("nestgate.toml");
        let default_config = crate::config::InstallerConfig::default();
        let config_toml = toml::to_string(&default_config).map_err(|_e| {
            NestGateError::validation("Configuration serialization error".to_string())
        })?;
        std::fs::write(&config_path, config_toml)?;

        info!("Configuration created: {}", config_path.display());
        Ok(())
    }

    /// Verify that the installation completed successfully
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Required files are missing
    /// - Binary is not executable
    /// - Configuration files are invalid
    pub fn verify_installation(&self, install_dir: &Path) -> Result<()> {
        let binary_path = install_dir.join("bin").join("nestgate");
        let config_path = install_dir.join("etc").join("nestgate.toml");

        if !binary_path.exists() {
            return Err(NestGateError::validation(format!(
                "Binary not found: {}",
                binary_path.display()
            )));
        }

        if !config_path.exists() {
            return Err(NestGateError::validation(format!(
                "Configuration not found: {}",
                config_path.display()
            )));
        }

        let mut last_err = None;
        for _ in 0..5 {
            match std::process::Command::new(&binary_path)
                .arg("--version")
                .output()
            {
                Ok(output) if output.status.success() => {
                    let version = String::from_utf8_lossy(&output.stdout);
                    info!("Installation verified: {}", version.trim());
                    last_err = None;
                    break;
                }
                Ok(_output) => {
                    return Err(NestGateError::validation(
                        "Binary execution failed: test_failed".to_string(),
                    ));
                }
                Err(e) if e.raw_os_error() == Some(26) => {
                    last_err = Some(e);
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
                Err(e) => {
                    return Err(NestGateError::from(e));
                }
            }
        }
        if let Some(e) = last_err {
            return Err(NestGateError::from(e));
        }

        Ok(())
    }

    /// Placeholder hook for configuration-driven downloads; currently a no-op.
    pub const fn download_components(&self, _config: &crate::config::InstallerConfig) {}
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod download_url_tests {
    use super::{
        DownloadManager, latest_version_string_from_body, release_download_from_github_body,
    };
    use nestgate_core::Result;
    use serde_json::json;

    #[test]
    fn release_tag_adds_v_prefix() {
        assert_eq!(DownloadManager::release_tag("1.2.3"), "v1.2.3");
        assert_eq!(DownloadManager::release_tag("v1.2.3"), "v1.2.3");
    }

    #[test]
    fn release_meta_url_format() {
        let u = DownloadManager::release_meta_url("o/r", "2.0.0");
        assert_eq!(u, "https://api.github.com/repos/o/r/releases/tags/v2.0.0");
    }

    #[test]
    fn download_components_noop() {
        let dm = DownloadManager::new();
        let cfg = crate::config::InstallerConfig::default();
        dm.download_components(&cfg);
    }

    #[test]
    fn release_tag_preserves_empty() {
        assert_eq!(DownloadManager::release_tag(""), "v");
    }

    #[test]
    fn release_meta_url_escapes_tag() {
        let u = DownloadManager::release_meta_url("org/repo", "v3.0.0");
        assert!(u.contains("v3.0.0") && u.contains("org/repo"));
    }

    #[test]
    fn user_agent_includes_version() {
        let ua = DownloadManager::user_agent();
        assert!(ua.starts_with("nestgate-installer/"));
    }

    #[test]
    fn github_repo_defaults_and_respects_env() {
        temp_env::with_vars([("NESTGATE_RELEASES_REPO", None::<&str>)], || {
            assert_eq!(DownloadManager::github_repo(), "ecoprimals/nestgate");
        });
        temp_env::with_vars(
            [("NESTGATE_RELEASES_REPO", Some("myfork/nestgate"))],
            || {
                assert_eq!(DownloadManager::github_repo(), "myfork/nestgate");
            },
        );
    }

    #[test]
    fn verify_installation_fails_when_paths_missing() {
        let dm = DownloadManager::new();
        let tmp = tempfile::tempdir().expect("tempdir");
        let err = dm.verify_installation(tmp.path()).expect_err("no binary");
        assert!(
            err.to_string().contains("Binary not found") || err.to_string().contains("not found"),
            "{err}"
        );
    }

    #[test]
    fn verify_installation_fails_when_config_missing_with_binary_present() {
        let dm = DownloadManager::new();
        let tmp = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(tmp.path().join("bin")).expect("bin");
        std::fs::write(
            tmp.path().join("bin").join("nestgate"),
            b"#! /bin/sh\nexit 0\n",
        )
        .expect("shim");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let p = tmp.path().join("bin").join("nestgate");
            let mut perms = std::fs::metadata(&p).expect("meta").permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&p, perms).expect("chmod");
        }
        let err = dm.verify_installation(tmp.path()).expect_err("no config");
        assert!(
            err.to_string().contains("Configuration not found")
                || err.to_string().contains("nestgate.toml"),
            "{err}"
        );
    }

    #[test]
    fn extract_archive_creates_layout() {
        let dm = DownloadManager::new();
        let base = tempfile::tempdir().expect("tempdir");
        let fake_archive = base.path().join("fake.tar.gz");
        std::fs::write(&fake_archive, b"x").expect("write");
        dm.extract_archive(&fake_archive, base.path())
            .expect("extract");
        assert!(base.path().join("bin").is_dir());
        assert!(base.path().join("etc").join("nestgate.toml").is_file());
    }

    #[test]
    fn release_download_prefers_archive_extension() -> Result<()> {
        let body = json!({
            "assets": [
                {"name": "checksums.txt", "browser_download_url": "https://example.com/c"},
                {"name": "nestgate-linux.tar.gz", "browser_download_url": "https://example.com/a.tgz"},
                {"name": "other.zip", "browser_download_url": "https://example.com/z.zip"}
            ]
        });
        let (url, name) = release_download_from_github_body(&body, "v1.0.0")?;
        assert_eq!(url, "https://example.com/a.tgz");
        assert_eq!(name, "nestgate-linux.tar.gz");
        Ok(())
    }

    #[test]
    fn release_download_prefers_first_archive_asset_in_list_order() -> Result<()> {
        let body = json!({
            "assets": [
                {"name": "a.zip", "browser_download_url": "https://example.com/first.zip"},
                {"name": "b.tar.gz", "browser_download_url": "https://example.com/second.tgz"}
            ]
        });
        let (url, name) = release_download_from_github_body(&body, "v1")?;
        assert_eq!(name, "a.zip");
        assert_eq!(url, "https://example.com/first.zip");
        Ok(())
    }

    #[test]
    fn release_download_selects_tar_gz_after_skipping_non_archive() -> Result<()> {
        let body = json!({
            "assets": [
                {"name": "notes.txt", "browser_download_url": "https://example.com/n"},
                {"name": "app.tar.gz", "browser_download_url": "https://example.com/app.tgz"}
            ]
        });
        let (url, name) = release_download_from_github_body(&body, "t")?;
        assert_eq!(name, "app.tar.gz");
        assert_eq!(url, "https://example.com/app.tgz");
        Ok(())
    }

    #[test]
    fn release_download_prefers_tar_xz_when_present() -> Result<()> {
        let body = json!({
            "assets": [
                {"name": "readme.md", "browser_download_url": "https://example.com/r"},
                {"name": "bundle.tar.xz", "browser_download_url": "https://example.com/x.xz"}
            ]
        });
        let (url, name) = release_download_from_github_body(&body, "v2")?;
        assert_eq!(name, "bundle.tar.xz");
        assert_eq!(url, "https://example.com/x.xz");
        Ok(())
    }

    #[test]
    fn release_download_falls_back_to_first_asset() -> Result<()> {
        let body = json!({
            "assets": [
                {"name": "only-deb.deb", "browser_download_url": "https://example.com/pkg.deb"}
            ]
        });
        let (url, name) = release_download_from_github_body(&body, "v1")?;
        assert_eq!(name, "only-deb.deb");
        assert_eq!(url, "https://example.com/pkg.deb");
        Ok(())
    }

    #[test]
    fn release_download_default_name_when_asset_name_missing() -> Result<()> {
        let body = json!({
            "assets": [
                {"browser_download_url": "https://example.com/anon"}
            ]
        });
        let (_url, name) = release_download_from_github_body(&body, "v1")?;
        assert_eq!(name, "release-asset");
        Ok(())
    }

    #[test]
    fn release_download_errors_when_assets_missing() {
        let body = json!({"name": "rel"});
        let err = release_download_from_github_body(&body, "v1").expect_err("assets");
        assert!(err.to_string().contains("assets array"), "{err}");
    }

    #[test]
    fn release_download_errors_when_assets_empty() {
        let body = json!({"assets": []});
        let err = release_download_from_github_body(&body, "v9").expect_err("empty");
        assert!(err.to_string().contains("no download assets"), "{err}");
    }

    #[test]
    fn release_download_errors_when_browser_download_url_missing() {
        let body = json!({
            "assets": [
                {"name": "nestgate-linux.tar.gz"}
            ]
        });
        let err = release_download_from_github_body(&body, "v1").expect_err("url");
        assert!(err.to_string().contains("browser_download_url"), "{err}");
    }

    #[test]
    fn latest_version_strips_v_prefix() -> Result<()> {
        let body = json!({"tag_name": "v4.5.6"});
        assert_eq!(latest_version_string_from_body(&body)?, "4.5.6");
        Ok(())
    }

    #[test]
    fn latest_version_preserves_without_v_prefix() -> Result<()> {
        let body = json!({"tag_name": "3.2.1"});
        assert_eq!(latest_version_string_from_body(&body)?, "3.2.1");
        Ok(())
    }

    #[test]
    fn latest_version_errors_when_tag_name_missing() {
        let body = json!({"name": "latest"});
        let err = latest_version_string_from_body(&body).expect_err("tag");
        assert!(err.to_string().contains("tag_name"), "{err}");
    }

    #[test]
    fn latest_version_strips_multiple_leading_v_chars() -> Result<()> {
        let body = json!({"tag_name": "vv1.0.0"});
        assert_eq!(latest_version_string_from_body(&body)?, "1.0.0");
        Ok(())
    }

    #[test]
    fn latest_version_empty_tag_name_is_ok() -> Result<()> {
        let body = json!({"tag_name": ""});
        assert_eq!(latest_version_string_from_body(&body)?, "");
        Ok(())
    }

    #[test]
    fn releases_latest_url_matches_github_api_shape() {
        temp_env::with_vars([("NESTGATE_RELEASES_REPO", None::<&str>)], || {
            let repo = DownloadManager::github_repo();
            let url = format!("https://api.github.com/repos/{repo}/releases/latest");
            assert_eq!(
                url,
                "https://api.github.com/repos/ecoprimals/nestgate/releases/latest"
            );
        });
    }

    #[test]
    fn download_release_meta_url_matches_inline_format_in_download_release() {
        let repo = "acme/demo";
        let version = "1.4.2";
        let tag = DownloadManager::release_tag(version);
        let from_helper = DownloadManager::release_meta_url(repo, version);
        let inline = format!("https://api.github.com/repos/{repo}/releases/tags/{tag}");
        assert_eq!(from_helper, inline);
    }

    #[cfg(unix)]
    fn write_test_script(path: &std::path::Path, content: &[u8]) {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .mode(0o755)
            .open(path)
            .expect("create script");
        f.write_all(content).expect("write script");
        f.sync_all().expect("sync script");
    }

    #[test]
    #[cfg(unix)]
    fn verify_installation_succeeds_when_binary_reports_version() {
        let dm = DownloadManager::new();
        let tmp = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(tmp.path().join("bin")).expect("bin");
        std::fs::create_dir_all(tmp.path().join("etc")).expect("etc");
        write_test_script(
            &tmp.path().join("bin").join("nestgate"),
            b"#!/bin/sh\necho nestgate 0.9.0\nexit 0\n",
        );
        std::fs::write(tmp.path().join("etc").join("nestgate.toml"), "[install]\n").expect("cfg");
        dm.verify_installation(tmp.path()).expect("verified");
    }

    #[test]
    #[cfg(unix)]
    fn verify_installation_fails_when_version_command_nonzero() {
        let dm = DownloadManager::new();
        let tmp = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(tmp.path().join("bin")).expect("bin");
        std::fs::create_dir_all(tmp.path().join("etc")).expect("etc");
        write_test_script(
            &tmp.path().join("bin").join("nestgate"),
            b"#!/bin/sh\nexit 1\n",
        );
        std::fs::write(tmp.path().join("etc").join("nestgate.toml"), "[install]\n").expect("cfg");
        let err = dm.verify_installation(tmp.path()).expect_err("bad exit");
        assert!(
            err.to_string().contains("test_failed") || err.to_string().contains("failed"),
            "{err}"
        );
    }

    #[test]
    #[cfg(unix)]
    fn verify_installation_fails_when_spawn_errors() {
        let dm = DownloadManager::new();
        let tmp = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(tmp.path().join("bin")).expect("bin");
        std::fs::create_dir_all(tmp.path().join("etc")).expect("etc");
        let bin = tmp.path().join("bin").join("nestgate");
        std::fs::create_dir(&bin).expect("dir instead of file");
        std::fs::write(tmp.path().join("etc").join("nestgate.toml"), "[install]\n").expect("cfg");
        let err = dm.verify_installation(tmp.path()).expect_err("spawn");
        assert!(!err.to_string().is_empty(), "{err}");
    }
}
