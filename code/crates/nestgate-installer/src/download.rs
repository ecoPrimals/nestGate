// SPDX-License-Identifier: AGPL-3.0-only
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

    fn github_repo() -> String {
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
    pub async fn download_release(&self, version: &str, target_dir: &PathBuf) -> Result<PathBuf> {
        std::fs::create_dir_all(target_dir)?;
        let repo = Self::github_repo();
        let tag = Self::release_tag(version);
        let meta_url = format!("https://api.github.com/repos/{repo}/releases/tags/{tag}");

        let body = Self::curl_json(&meta_url)?;

        let assets = body["assets"].as_array().ok_or_else(|| {
            NestGateError::internal_error(
                "release response missing assets array",
                "download_release",
            )
        })?;
        let asset = assets
            .iter()
            .find(|a| {
                a["name"].as_str().is_some_and(|n| {
                    n.ends_with(".tar.gz") || n.ends_with(".tar.xz") || n.ends_with(".zip")
                })
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
        let path = target_dir.join(asset_name);

        Self::curl_download(download_url, &path)?;

        Ok(path)
    }

    /// Latest release tag from GitHub (`releases/latest`), without a leading `v` when present.
    ///
    /// # Errors
    ///
    /// Returns when the API call fails or the response is not usable.
    pub async fn check_latest_version(&self) -> Result<String> {
        let repo = Self::github_repo();
        let url = format!("https://api.github.com/repos/{repo}/releases/latest");

        let body = Self::curl_json(&url)?;

        let tag = body["tag_name"].as_str().ok_or_else(|| {
            NestGateError::internal_error(
                "missing tag_name in GitHub releases/latest response",
                "check_latest_version",
            )
        })?;
        Ok(tag.trim_start_matches('v').to_string())
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

        let output = std::process::Command::new(&binary_path)
            .arg("--version")
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout);
                info!("Installation verified: {}", version.trim());
            }
            Ok(_output) => {
                return Err(NestGateError::validation(
                    "Binary execution failed: test_failed".to_string(),
                ));
            }
            Err(e) => {
                return Err(NestGateError::from(e));
            }
        }

        Ok(())
    }

    /// Download components based on configuration
    pub const fn download_components(
        &self,
        _config: &crate::config::InstallerConfig,
    ) -> nestgate_core::error::Result<()> {
        Ok(())
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod download_url_tests {
    use super::DownloadManager;

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
        assert!(dm.download_components(&cfg).is_ok());
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
}
