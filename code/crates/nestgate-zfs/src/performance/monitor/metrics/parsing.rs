// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Parse `zpool get` / iostat text into structured fields.

use crate::performance::types::PoolProperties;

/// Parse tab-separated lines from `zpool get all <pool>` (property name in column 2, value in 3).
pub(super) fn parse_zpool_get_pool_properties(output: &str) -> PoolProperties {
    let mut properties = PoolProperties::default();
    for line in output.lines() {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() >= 3 {
            match fields[1] {
                "fragmentation" => {
                    if let Ok(frag) = fields[2].trim_end_matches('%').parse::<f64>() {
                        properties.fragmentation = frag;
                    }
                }
                "compressratio" => {
                    if let Ok(ratio) = fields[2].trim_end_matches('x').parse::<f64>() {
                        properties.compression_ratio = ratio;
                    }
                }
                "dedupratio" => {
                    if let Ok(ratio) = fields[2].trim_end_matches('x').parse::<f64>() {
                        properties.dedup_ratio = ratio;
                    }
                }
                _ => {}
            }
        }
    }
    properties
}
