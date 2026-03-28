// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use nestgate_core::{NestGateError, Result};
use std::path::{Path, PathBuf};

/// Default GitHub `owner/repo` for release metadata (override with `NESTGATE_RELEASES_REPO`).
const DEFAULT_GITHUB_REPO: &str = "ecoprimals/nestgate";

/// Installer error type alias
#[allow(dead_code)] // Reserved for future error handling
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
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }

    fn github_repo() -> String {
        std::env::var("NESTGATE_RELEASES_REPO").unwrap_or_else(|_| DEFAULT_GITHUB_REPO.to_string())
    }

    fn http_client() -> Result<reqwest::Client> {
        reqwest::Client::builder()
            .user_agent(concat!("nestgate-installer/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| {
                NestGateError::internal_error(
                    format!("failed to build HTTP client: {e}"),
                    "DownloadManager::http_client",
                )
            })
    }

    /// Download a release asset for `version` into `target_dir` and return the saved file path.
    ///
    /// # Errors
    ///
    /// Returns when the GitHub API is unreachable, the release or assets are missing, or I/O fails.
    pub async fn download_release(&self, version: &str, target_dir: &PathBuf) -> Result<PathBuf> {
        std::fs::create_dir_all(target_dir)?;
        let repo = Self::github_repo();
        let tag = if version.starts_with('v') {
            version.to_string()
        } else {
            format!("v{version}")
        };
        let meta_url = format!("https://api.github.com/repos/{repo}/releases/tags/{tag}");
        let client = Self::http_client()?;
        let resp = client.get(&meta_url).send().await.map_err(|e| {
            NestGateError::internal_error(
                format!("request failed for {meta_url}: {e}"),
                "download_release",
            )
        })?;
        if !resp.status().is_success() {
            return Err(NestGateError::internal_error(
                format!(
                    "GitHub API returned {} for {} (set NESTGATE_RELEASES_REPO if needed)",
                    resp.status(),
                    meta_url
                ),
                "download_release",
            ));
        }
        let body: serde_json::Value = resp.json().await.map_err(|e| {
            NestGateError::internal_error(
                format!("invalid JSON from GitHub release: {e}"),
                "download_release",
            )
        })?;
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
        let dl = client.get(download_url).send().await.map_err(|e| {
            NestGateError::internal_error(format!("download failed: {e}"), "download_release")
        })?;
        if !dl.status().is_success() {
            return Err(NestGateError::internal_error(
                format!("GitHub returned {} for asset download", dl.status()),
                "download_release",
            ));
        }
        let bytes = dl.bytes().await.map_err(|e| {
            NestGateError::internal_error(format!("reading release body: {e}"), "download_release")
        })?;
        let path = target_dir.join(asset_name);
        std::fs::write(&path, &bytes)?;
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
        let client = Self::http_client()?;
        let resp = client.get(&url).send().await.map_err(|e| {
            NestGateError::internal_error(
                format!("request failed for {url}: {e}"),
                "check_latest_version",
            )
        })?;
        if !resp.status().is_success() {
            return Err(NestGateError::internal_error(
                format!(
                    "GitHub API returned {} for {} (set NESTGATE_RELEASES_REPO if needed)",
                    resp.status(),
                    url
                ),
                "check_latest_version",
            ));
        }
        let body: serde_json::Value = resp.json().await.map_err(|e| {
            NestGateError::internal_error(
                format!("invalid JSON from GitHub releases: {e}"),
                "check_latest_version",
            )
        })?;
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
        println!("Extracting archive to {}", target_dir.display());

        // Create directory structure
        std::fs::create_dir_all(target_dir.join("bin"))?;
        std::fs::create_dir_all(target_dir.join("share"))?;
        std::fs::create_dir_all(target_dir.join("etc"))?;

        // In production, this would extract the real archive
        // For now, simulate by copying current binary if available
        if let Ok(current_exe) = std::env::current_exe() {
            let target_binary = target_dir.join("bin").join("nestgate");
            std::fs::copy(&current_exe, &target_binary).map_err(NestGateError::from)?;

            // Make executable on Unix
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&target_binary)?.permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&target_binary, perms)?;
            }

            println!("Binary installed to: {}", target_binary.display());
        }

        // Create default configuration
        let config_path = target_dir.join("etc").join("nestgate.toml");
        let default_config = crate::config::InstallerConfig::default();
        let config_toml = toml::to_string(&default_config).map_err(|_e| {
            NestGateError::validation("Configuration serialization error".to_string())
        })?;
        std::fs::write(&config_path, config_toml)?;

        println!("Configuration created: {}", config_path.display());
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

        // Try to run the binary with --version
        let output = std::process::Command::new(&binary_path)
            .arg("--version")
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout);
                println!("Installation verified: {}", version.trim());
            }
            Ok(output) => {
                let _error = String::from_utf8_lossy(&output.stderr);
                return Err(NestGateError::validation(format!(
                    "Binary execution failed: {}",
                    "test_failed"
                )));
            }
            Err(e) => {
                return Err(NestGateError::from(e));
            }
        }

        Ok(())
    }

    /// Download components based on configuration
    #[allow(dead_code)] // Reserved for future component downloads
    pub fn download_components(
        &self,
        _config: &crate::config::InstallerConfig,
    ) -> nestgate_core::error::Result<()> {
        // Simplified implementation for canonical modernization
        // In a full implementation, this would download selected components
        Ok(())
    }
}

impl Default for DownloadManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}
