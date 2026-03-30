// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! ZFS size and capacity string parsing utilities.

use nestgate_zfs::numeric::f64_to_u64_saturating;

/// Parse pool capacity from a [`nestgate_zfs::command::ZfsPool`].
///
/// Returns (`total_gb`, `used_gb`, `available_gb`).
pub(crate) fn parse_pool_capacity(pool: &nestgate_zfs::command::ZfsPool) -> (u64, u64, u64) {
    let total = parse_zfs_size(&pool.size);
    let allocated = parse_zfs_size(&pool.allocated);
    let free = parse_zfs_size(&pool.free);
    (total, allocated, free)
}

/// Parse a human-readable ZFS size string ("500G", "1.5T", "100M", "2048K") to
/// an approximate value in gigabytes.
pub(crate) fn parse_zfs_size(s: &str) -> u64 {
    let s = s.trim();
    if let Some(rest) = s.strip_suffix('T') {
        f64_to_u64_saturating(rest.parse::<f64>().unwrap_or(0.0) * 1024.0)
    } else if let Some(rest) = s.strip_suffix('G') {
        f64_to_u64_saturating(rest.parse::<f64>().unwrap_or(0.0))
    } else if let Some(rest) = s.strip_suffix('M') {
        f64_to_u64_saturating(rest.parse::<f64>().unwrap_or(0.0) / 1024.0)
    } else if let Some(rest) = s.strip_suffix('K') {
        f64_to_u64_saturating(rest.parse::<f64>().unwrap_or(0.0) / (1024.0 * 1024.0))
    } else {
        s.parse::<u64>().unwrap_or(0) / (1024 * 1024 * 1024)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_zfs::command::ZfsPool;

    #[test]
    fn parse_pool_capacity_t_g_m_and_bytes() {
        let p = ZfsPool {
            name: "t".into(),
            size: "2T".into(),
            allocated: "500G".into(),
            free: "100M".into(),
            health: "ONLINE".into(),
        };
        let (t, u, f) = parse_pool_capacity(&p);
        assert!(t >= 2000);
        assert!(u <= t);
        assert!(f < t);

        let p2 = ZfsPool {
            name: "b".into(),
            size: "1073741824".into(),
            allocated: "0".into(),
            free: "0".into(),
            health: "ONLINE".into(),
        };
        let (tb, _, _) = parse_pool_capacity(&p2);
        assert!(tb <= 1);
    }

    #[test]
    fn parse_zfs_size_edge_units() {
        assert_eq!(parse_zfs_size("  2T  "), 2048);
        assert_eq!(parse_zfs_size("1.5G"), 1);
        assert_eq!(parse_zfs_size("512M"), 0);
        assert!(parse_zfs_size("2048M") >= 1);
        assert!(parse_zfs_size("1048576K") > 0);
        assert_eq!(parse_zfs_size("not_a_number"), 0);
    }
}
