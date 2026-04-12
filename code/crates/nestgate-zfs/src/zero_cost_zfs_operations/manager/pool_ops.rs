// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Pool create, list, and property fetch (inherent implementation).

use super::ZeroCostZfsManager;
use crate::error::{ZfsOperation, create_zfs_error};
use crate::zero_cost_zfs_operations::types::ZeroCostPoolInfo;
use nestgate_core::Result;
use std::collections::HashMap;
use std::time::SystemTime;

/// Build `zfs create` argv for pool creation: `create <name> <devices...>` (no execution).
pub fn build_pool_create_zfs_args<'a>(name: &'a str, devices: &[&'a str]) -> Vec<&'a str> {
    let mut args = vec!["create", name];
    args.extend_from_slice(devices);
    args
}

/// Build `ZeroCostPoolInfo` from `zfs get -H -p` tabular properties.
pub fn zero_cost_pool_from_zfs_properties(
    name: &str,
    properties: &HashMap<String, String>,
    created_at: SystemTime,
) -> ZeroCostPoolInfo {
    let size: u64 = properties
        .get("size")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let used: u64 = properties
        .get("allocated")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let available = size.saturating_sub(used);

    ZeroCostPoolInfo {
        name: name.to_string(),
        size,
        used,
        available,
        health: properties
            .get("health")
            .map_or_else(|| "UNKNOWN".to_string(), std::string::ToString::to_string),
        properties: properties.clone(),
        created_at,
    }
}

/// Parse `zfs list -H -o name,size,used,avail,health` stdout into [`ZeroCostPoolInfo`] values (no ZFS).
pub fn pools_from_zfs_list_output(
    output: &str,
    max_pools: usize,
    created_at: SystemTime,
) -> Vec<ZeroCostPoolInfo> {
    let mut pools = Vec::new();
    for line in output.lines() {
        if let Some(p) = parse_pool_list_line(line, created_at) {
            pools.push(p);
            if pools.len() >= max_pools {
                break;
            }
        }
    }
    pools
}

/// Parse one `zfs list -H` line (`name\tsize\tused\tavail\thealth`).
pub fn parse_pool_list_line(line: &str, created_at: SystemTime) -> Option<ZeroCostPoolInfo> {
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() < 5 {
        return None;
    }
    let name = parts[0].to_string();
    let size = parts[1].parse().unwrap_or(0);
    let used = parts[2].parse().unwrap_or(0);
    let available = parts[3].parse().unwrap_or(0);
    let health = parts[4].to_string();

    Some(ZeroCostPoolInfo {
        name,
        size,
        used,
        available,
        health,
        properties: HashMap::new(),
        created_at,
    })
}

impl<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> ZeroCostZfsManager<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS, COMMAND_TIMEOUT_MS>
{
    pub(super) async fn pool_create(
        &self,
        name: &str,
        devices: &[&str],
    ) -> Result<ZeroCostPoolInfo> {
        if !self.can_create_more_pools().await {
            return Err(create_zfs_error(
                "Cannot create pool: maximum pools reached".to_string(),
                ZfsOperation::PoolCreate,
            ));
        }

        let args = build_pool_create_zfs_args(name, devices);

        self.execute_zfs_command(&args).await?;

        let properties_output = self
            .execute_zfs_command(&["get", "all", "-H", "-p", name])
            .await?;

        let properties = self.parse_pool_properties(&properties_output);

        let pool_info = zero_cost_pool_from_zfs_properties(name, &properties, SystemTime::now());

        {
            let mut pools_map = self.pools.write().await;
            pools_map.insert(pool_info.name.clone(), pool_info.clone());
        }
        Ok(pool_info)
    }

    pub(super) async fn pool_get_properties(
        &self,
        pool: &ZeroCostPoolInfo,
    ) -> Result<HashMap<String, String>> {
        {
            let pools_map = self.pools.read().await;
            if let Some(cached_pool) = pools_map.get(&pool.name) {
                return Ok(cached_pool.properties.clone());
            }
        }

        let properties_output = self
            .execute_zfs_command(&["get", "all", "-H", "-p", &pool.name])
            .await?;

        let properties = self.parse_pool_properties(&properties_output);

        {
            let mut pools_map = self.pools.write().await;
            let pool_info =
                zero_cost_pool_from_zfs_properties(&pool.name, &properties, SystemTime::now());
            pools_map.insert(pool.name.clone(), pool_info);
        }

        Ok(properties)
    }

    pub(super) async fn pool_list(&self) -> Result<Vec<ZeroCostPoolInfo>> {
        let output = self
            .execute_zfs_command(&["list", "-H", "-o", "name,size,used,avail,health"])
            .await?;

        Ok(pools_from_zfs_list_output(
            &output,
            MAX_POOLS,
            SystemTime::now(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::traits::ZeroCostZfsOperations;
    use super::super::super::types::ZeroCostPoolInfo;
    use super::super::TestingZfsManager;
    use super::super::test_zfs_stub::ZfsCommandStubGuard;
    use super::{
        build_pool_create_zfs_args, parse_pool_list_line, pools_from_zfs_list_output,
        zero_cost_pool_from_zfs_properties,
    };
    use nestgate_core::Result;
    use std::collections::HashMap;
    use std::time::SystemTime;

    #[tokio::test]
    async fn test_can_create_more_pools_false_when_pool_capacity_full() {
        let m = TestingZfsManager::new();
        m.test_insert_pool_entry("p0".into()).await;
        m.test_insert_pool_entry("p1".into()).await;
        assert_eq!(m.test_pool_map_len().await, 2);
        assert!(
            !m.test_can_create_more_pools().await,
            "TestingZfsManager allows at most 2 pools"
        );
    }

    #[test]
    fn build_pool_create_zfs_args_empty_devices() {
        let args = build_pool_create_zfs_args("tank", &[]);
        assert_eq!(args, vec!["create", "tank"]);
    }

    #[test]
    fn zero_cost_pool_from_zfs_properties_invalid_numbers_default_to_zero_and_saturating_available()
    {
        let mut p = HashMap::new();
        p.insert("size".into(), "not-a-number".into());
        p.insert("allocated".into(), "also-bad".into());
        p.insert("health".into(), "DEGRADED".into());
        let info = zero_cost_pool_from_zfs_properties("z", &p, SystemTime::UNIX_EPOCH);
        assert_eq!(info.size, 0);
        assert_eq!(info.used, 0);
        assert_eq!(info.available, 0);
        assert_eq!(info.health, "DEGRADED");
        assert_eq!(info.properties, p);
    }

    #[test]
    fn zero_cost_pool_from_zfs_properties_used_exceeds_size_saturates_available() {
        let mut p = HashMap::new();
        p.insert("size".into(), "100".into());
        p.insert("allocated".into(), "500".into());
        let info = zero_cost_pool_from_zfs_properties("z", &p, SystemTime::UNIX_EPOCH);
        assert_eq!(info.available, 0);
    }

    #[test]
    fn parse_pool_list_line_non_numeric_columns_use_zero() {
        let p = parse_pool_list_line("tank\tx\ty\tz\tHEALTH", SystemTime::UNIX_EPOCH)
            .expect("five columns");
        assert_eq!(p.size, 0);
        assert_eq!(p.used, 0);
        assert_eq!(p.available, 0);
        assert_eq!(p.health, "HEALTH");
    }

    #[test]
    fn parse_pool_list_line_accepts_extra_tab_columns() {
        let p = parse_pool_list_line("tank\t1\t2\t3\tONLINE\textra", SystemTime::UNIX_EPOCH)
            .expect("pool");
        assert_eq!(p.name, "tank");
    }

    #[test]
    fn pools_from_zfs_list_output_skips_bad_lines_and_respects_max() {
        let t = SystemTime::UNIX_EPOCH;
        let out = "bad\np0\t1\t0\t1\tONLINE\np1\t1\t0\t1\tONLINE\np2\t1\t0\t1\tONLINE\n";
        let v = pools_from_zfs_list_output(out, 2, t);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].name, "p0");
        assert_eq!(v[1].name, "p1");
    }

    #[test]
    fn pools_from_zfs_list_output_empty_stdout() {
        assert!(pools_from_zfs_list_output("", 10, SystemTime::UNIX_EPOCH).is_empty());
    }

    fn stub_ok_pool_flow() -> impl Fn(&[&str]) -> Result<String> + Send + Sync + 'static {
        |args: &[&str]| -> Result<String> {
            match args.first().copied() {
                Some("get") => Ok("size\t4096\nallocated\t1024\nhealth\tONLINE\n".to_string()),
                Some("list") => Ok("stub-pool\t100\t10\t90\tONLINE\n".to_string()),
                _ => Ok(String::new()),
            }
        }
    }

    #[tokio::test]
    async fn pool_list_uses_stub_and_parses_rows() {
        let _g = ZfsCommandStubGuard::set(Box::new(stub_ok_pool_flow()));
        let m = TestingZfsManager::new();
        let pools = ZeroCostZfsOperations::list_pools(&m)
            .await
            .expect("list pools");
        assert_eq!(pools.len(), 1);
        assert_eq!(pools[0].name, "stub-pool");
    }

    #[tokio::test]
    async fn pool_get_properties_cache_miss_uses_stub_get() {
        let _g = ZfsCommandStubGuard::set(Box::new(stub_ok_pool_flow()));
        let m = TestingZfsManager::new();
        let pool = ZeroCostPoolInfo {
            name: "fresh-pool".into(),
            size: 0,
            used: 0,
            available: 0,
            health: "UNKNOWN".into(),
            properties: HashMap::new(),
            created_at: SystemTime::UNIX_EPOCH,
        };
        let props = ZeroCostZfsOperations::get_pool_properties(&m, &pool)
            .await
            .expect("props");
        assert_eq!(props.get("size").map(String::as_str), Some("4096"));
    }

    #[tokio::test]
    async fn pool_create_with_stub_inserts_pool() {
        let _g = ZfsCommandStubGuard::set(Box::new(stub_ok_pool_flow()));
        let m = TestingZfsManager::new();
        let p = ZeroCostZfsOperations::create_pool(&m, "newpool", &["/dev/loop0"])
            .await
            .expect("create");
        assert_eq!(p.name, "newpool");
        assert_eq!(m.test_pool_map_len().await, 1);
    }
}
