//! Physical storage substrate tier mapping (warm/cold paths).
//!
//! See [`SubstrateTiers`] and [`SubstrateMount`] for runtime discovery.

use std::env;
use std::path::{Path, PathBuf};

use super::storage_paths::get_storage_paths;

/// Physical storage substrate tier mapping.
///
/// Maps logical warm/cold tiers to actual filesystem mount points,
/// enabling nestGate to place data on the appropriate physical medium.
///
/// - **Warm**: NVMe/SSD paths for active data, caches, metadata
/// - **Cold**: HDD paths for archival data, snapshots, bulk storage
///
/// Discovered at runtime from environment variables or filesystem detection.
#[derive(Debug, Clone)]
pub struct SubstrateTiers {
    /// Warm tier root paths (NVMe/SSD), ordered by preference
    pub warm: Vec<SubstrateMount>,
    /// Cold tier root paths (HDD), ordered by preference
    pub cold: Vec<SubstrateMount>,
}

/// A single substrate mount point with detected metadata
#[derive(Debug, Clone)]
pub struct SubstrateMount {
    /// Mount path (e.g., `/mnt/nestgate/cold/zfs`)
    pub path: PathBuf,
    /// Filesystem type (e.g., `zfs`, `btrfs`, `ext4`, `xfs`)
    pub fs_type: String,
    /// Whether this is rotational storage (HDD=true, SSD/NVMe=false)
    pub rotational: bool,
    /// Total capacity in bytes (0 if unknown)
    pub capacity_bytes: u64,
}

impl SubstrateTiers {
    /// Discover substrate tiers from environment and system state.
    ///
    /// Priority:
    /// 1. `NESTGATE_WARM_PATHS` / `NESTGATE_COLD_PATHS` (colon-separated)
    /// 2. Well-known mount base at `NESTGATE_SUBSTRATE_BASE` (default `/mnt/nestgate`)
    /// 3. Fallback to XDG data dir as sole warm tier
    #[must_use]
    pub fn from_environment() -> Self {
        let warm = Self::resolve_tier_paths("NESTGATE_WARM_PATHS");
        let cold = Self::resolve_tier_paths("NESTGATE_COLD_PATHS");

        if !warm.is_empty() || !cold.is_empty() {
            return Self { warm, cold };
        }

        let base =
            env::var("NESTGATE_SUBSTRATE_BASE").unwrap_or_else(|_| "/mnt/nestgate".to_string());
        let base = PathBuf::from(base);

        let mut warm_mounts = Vec::new();
        let mut cold_mounts = Vec::new();

        if let Ok(entries) = std::fs::read_dir(base.join("warm")) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    warm_mounts.push(SubstrateMount::detect(&entry.path()));
                }
            }
        }

        if let Ok(entries) = std::fs::read_dir(base.join("cold")) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    cold_mounts.push(SubstrateMount::detect(&entry.path()));
                }
            }
        }

        if warm_mounts.is_empty() {
            warm_mounts.push(SubstrateMount {
                path: get_storage_paths().storage_base_path(),
                fs_type: "auto".to_string(),
                rotational: false,
                capacity_bytes: 0,
            });
        }

        Self {
            warm: warm_mounts,
            cold: cold_mounts,
        }
    }

    fn resolve_tier_paths(env_key: &str) -> Vec<SubstrateMount> {
        env::var(env_key)
            .unwrap_or_default()
            .split(':')
            .filter(|s| !s.is_empty())
            .map(|p| SubstrateMount::detect(&PathBuf::from(p)))
            .collect()
    }

    /// Returns all available substrate paths across both tiers
    pub fn all_paths(&self) -> Vec<&Path> {
        self.warm
            .iter()
            .chain(self.cold.iter())
            .map(|m| m.path.as_path())
            .collect()
    }

    /// Returns substrate paths grouped by filesystem type
    pub fn by_fs_type(&self) -> std::collections::HashMap<&str, Vec<&SubstrateMount>> {
        let mut map: std::collections::HashMap<&str, Vec<&SubstrateMount>> =
            std::collections::HashMap::new();
        for m in self.warm.iter().chain(self.cold.iter()) {
            map.entry(m.fs_type.as_str()).or_default().push(m);
        }
        map
    }
}

impl SubstrateMount {
    /// Detect filesystem type and capacity for a path
    fn detect(path: &Path) -> Self {
        let (fs_type, rotational, capacity) = Self::probe_mount(path);
        Self {
            path: path.to_path_buf(),
            fs_type,
            rotational,
            capacity_bytes: capacity,
        }
    }

    #[cfg(target_os = "linux")]
    fn probe_mount(path: &Path) -> (String, bool, u64) {
        let fs_type =
            Self::detect_fs_type_from_mounts(path).unwrap_or_else(|| "unknown".to_string());

        let rotational = Self::detect_rotational(path);

        let capacity = rustix::fs::statvfs(path)
            .map(|st| st.f_blocks * st.f_frsize)
            .unwrap_or(0);

        (fs_type, rotational, capacity)
    }

    #[cfg(not(target_os = "linux"))]
    fn probe_mount(path: &Path) -> (String, bool, u64) {
        let _ = path;
        ("unknown".to_string(), false, 0)
    }

    #[cfg(target_os = "linux")]
    fn detect_fs_type_from_mounts(path: &Path) -> Option<String> {
        let mounts = std::fs::read_to_string("/proc/mounts").ok()?;
        let canonical = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let path_str = canonical.to_string_lossy();

        let mut best_match: Option<(&str, usize)> = None;

        for line in mounts.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let mount_point = parts[1];
                if path_str.starts_with(mount_point) {
                    let len = mount_point.len();
                    if best_match.is_none_or(|(_, best_len)| len > best_len) {
                        best_match = Some((parts[2], len));
                    }
                }
            }
        }

        best_match.map(|(fs, _)| fs.to_string())
    }

    #[cfg(target_os = "linux")]
    fn detect_rotational(path: &Path) -> bool {
        let canonical = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let path_str = canonical.to_string_lossy();

        let mounts = std::fs::read_to_string("/proc/mounts").unwrap_or_default();
        let mut device = String::new();
        let mut best_len = 0;

        for line in mounts.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && path_str.starts_with(parts[1]) && parts[1].len() > best_len {
                device = parts[0].to_string();
                best_len = parts[1].len();
            }
        }

        if device.is_empty() {
            return false;
        }

        let dev_name = device.rsplit('/').next().unwrap_or("");
        let base_dev: String = dev_name.chars().take_while(|c| c.is_alphabetic()).collect();

        if base_dev.is_empty() {
            return false;
        }

        let rotational_path = format!("/sys/block/{base_dev}/queue/rotational");
        std::fs::read_to_string(&rotational_path)
            .map(|s| s.trim() == "1")
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substrate_tiers_from_environment_does_not_panic() {
        let tiers = SubstrateTiers::from_environment();
        assert!(!tiers.warm.is_empty(), "should have at least one warm tier");
    }

    #[test]
    fn substrate_tiers_all_paths_and_by_fs_type() {
        let tiers = SubstrateTiers::from_environment();
        let all = tiers.all_paths();
        assert!(!all.is_empty());
        let by_type = tiers.by_fs_type();
        for (_fs, mounts) in &by_type {
            assert!(!mounts.is_empty());
        }
    }

    #[test]
    fn substrate_mount_detect_current_dir() {
        let mount = SubstrateMount::detect(std::path::Path::new("."));
        assert!(!mount.fs_type.is_empty());
    }

    #[test]
    fn substrate_tiers_from_env_vars() {
        let orig_warm = std::env::var("NESTGATE_WARM_PATHS").ok();
        let orig_cold = std::env::var("NESTGATE_COLD_PATHS").ok();

        std::env::set_var("NESTGATE_WARM_PATHS", "/tmp");
        std::env::set_var("NESTGATE_COLD_PATHS", "/tmp");

        let tiers = SubstrateTiers::from_environment();
        assert_eq!(tiers.warm.len(), 1);
        assert_eq!(tiers.cold.len(), 1);

        match orig_warm {
            Some(v) => std::env::set_var("NESTGATE_WARM_PATHS", v),
            None => std::env::remove_var("NESTGATE_WARM_PATHS"),
        }
        match orig_cold {
            Some(v) => std::env::set_var("NESTGATE_COLD_PATHS", v),
            None => std::env::remove_var("NESTGATE_COLD_PATHS"),
        }
    }
}
