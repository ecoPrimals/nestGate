// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Pool create, list, and property fetch (inherent implementation).

use super::ZeroCostZfsManager;
use crate::error::{ZfsOperation, create_zfs_error};
use crate::zero_cost_zfs_operations::types::ZeroCostPoolInfo;
use nestgate_core::Result;
use std::collections::HashMap;
use std::time::SystemTime;

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

        let mut args = vec!["create", name];
        args.extend(devices);

        self.execute_zfs_command(&args).await?;

        let properties_output = self
            .execute_zfs_command(&["get", "all", "-H", "-p", name])
            .await?;

        let properties = self.parse_pool_properties(&properties_output);

        let size: u64 = properties
            .get("size")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let used: u64 = properties
            .get("allocated")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let available = size.saturating_sub(used);

        let pool_info = ZeroCostPoolInfo {
            name: name.to_string(),
            size,
            used,
            available,
            health: properties
                .get("health")
                .map_or("UNKNOWN".to_string(), std::string::ToString::to_string),
            properties: properties.clone(),
            created_at: SystemTime::now(),
        };

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
            let pool_info = ZeroCostPoolInfo {
                name: pool.name.clone(),
                size: properties
                    .get("size")
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(0),
                used: properties
                    .get("allocated")
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(0),
                available: properties
                    .get("size")
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(0)
                    .saturating_sub(
                        properties
                            .get("allocated")
                            .and_then(|s| s.parse::<u64>().ok())
                            .unwrap_or(0),
                    ),
                health: properties
                    .get("health")
                    .map_or("UNKNOWN".to_string(), std::string::ToString::to_string),
                properties: properties.clone(),
                created_at: SystemTime::now(),
            };
            pools_map.insert(pool.name.clone(), pool_info);
        }

        Ok(properties)
    }

    pub(super) async fn pool_list(&self) -> Result<Vec<ZeroCostPoolInfo>> {
        let output = self
            .execute_zfs_command(&["list", "-H", "-o", "name,size,used,avail,health"])
            .await?;

        let mut pools = Vec::with_capacity(MAX_POOLS);

        for line in output.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 5 {
                let name = parts[0].to_string();
                let size = parts[1].parse().unwrap_or(0);
                let used = parts[2].parse().unwrap_or(0);
                let available = parts[3].parse().unwrap_or(0);
                let health = parts[4].to_string();

                pools.push(ZeroCostPoolInfo {
                    name: name.clone(),
                    size,
                    used,
                    available,
                    health,
                    properties: HashMap::new(), // Would be populated on demand
                    created_at: SystemTime::now(), // Approximation
                });

                if pools.len() >= MAX_POOLS {
                    break;
                }
            }
        }

        Ok(pools)
    }
}
