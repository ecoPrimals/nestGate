// SPDX-License-Identifier: AGPL-3.0-or-later
//! USDA NASS Live Provider — county-level crop yield data.
//!
//! Connects to the USDA NASS Quick Stats API to provide
//! county-level crop yield data for validation of yield response models.
//!
//! USDA NASS Quick Stats provides:
//! - County-level crop yields for all US states
//! - Annual survey data going back decades
//! - Production, area, yield for major commodities
//! - **Free with instant API key registration**
//!
//! Used by airSpring for validating the Stewart (1977) yield response model
//! predictions against actual Michigan crop harvests.
//!
//! # Reference implementation
//!
//! Python baseline: `airSpring/scripts/download_usda_nass.py`
//! Validation target: airSpring yield_validation.md baseCamp paper

use crate::data_sources::data_capabilities::*;
use crate::data_sources::providers::universal_http_provider::{
    HttpProviderConfigBuilder, UniversalHttpProvider,
};
use crate::{NestGateError, Result};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, info};

/// USDA NASS Quick Stats Live Provider
pub struct UsdaNassLiveProvider {
    http_provider: UniversalHttpProvider,
    api_key: String,
}

impl UsdaNassLiveProvider {
    /// Create a new USDA NASS provider with API key.
    ///
    /// Register for a free key at https://quickstats.nass.usda.gov/api/
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP provider cannot be constructed.
    pub fn new(api_key: String) -> Result<Self> {
        let config = HttpProviderConfigBuilder::new(
            "https://quickstats.nass.usda.gov/api".to_string(),
            "crop_yield_data".to_string(),
        )
        .with_timeout(60)
        .with_api_key(api_key.clone())
        .with_metadata("provider_name".to_string(), "USDA NASS".to_string())
        .with_metadata(
            "provider_type".to_string(),
            "agricultural_statistics".to_string(),
        )
        .with_metadata(
            "data_source".to_string(),
            "USDA National Agricultural Statistics Service".to_string(),
        )
        .with_metadata(
            "license".to_string(),
            "US Government Public Domain".to_string(),
        )
        .with_metadata(
            "attribution".to_string(),
            "Data provided by USDA NASS Quick Stats".to_string(),
        )
        .with_header(
            "User-Agent".to_string(),
            "NestGate/1.0 (airSpring Sovereign Agriculture)".to_string(),
        )
        .build();

        let http_provider = UniversalHttpProvider::new(config)?;

        info!("Created USDA NASS live provider (Quick Stats crop yields)");

        Ok(Self {
            http_provider,
            api_key,
        })
    }

    /// Create from environment variable `USDA_NASS_API_KEY`.
    ///
    /// # Errors
    ///
    /// Returns an error if the environment variable is not set.
    pub fn create_from_env() -> Result<Self> {
        let api_key = std::env::var("USDA_NASS_API_KEY").map_err(|_| {
            NestGateError::internal_error(
                "USDA_NASS_API_KEY environment variable not set. \
                 Register at https://quickstats.nass.usda.gov/api/"
                    .to_string(),
            )
        })?;
        Self::new(api_key)
    }

    /// Fetch county-level crop yields for a state.
    ///
    /// Returns JSON array of yield records with county, year, value.
    pub async fn fetch_county_yields(
        &self,
        commodity: &str,
        state: &str,
        year_start: u32,
        year_end: u32,
    ) -> Result<Value> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), self.api_key.clone());
        params.insert("source_desc".to_string(), "SURVEY".to_string());
        params.insert("sector_desc".to_string(), "CROPS".to_string());
        params.insert("commodity_desc".to_string(), commodity.to_string());
        params.insert(
            "statisticcat_desc".to_string(),
            "YIELD".to_string(),
        );
        params.insert("state_alpha".to_string(), state.to_string());
        params.insert("agg_level_desc".to_string(), "COUNTY".to_string());
        params.insert("year__GE".to_string(), year_start.to_string());
        params.insert("year__LE".to_string(), year_end.to_string());
        params.insert("format".to_string(), "JSON".to_string());

        debug!(
            "Fetching USDA NASS yields: {} in {} ({}-{})",
            commodity, state, year_start, year_end
        );

        self.http_provider
            .get_request("api_GET/", &params)
            .await
    }

    /// Fetch Michigan crop yields for the five primary field crops.
    ///
    /// Downloads corn, soybeans, wheat, sugar beets, and dry beans.
    pub async fn fetch_michigan_yields(
        &self,
        year_start: u32,
        year_end: u32,
    ) -> Result<Vec<Value>> {
        let crops = ["CORN", "SOYBEANS", "WHEAT", "SUGARBEETS", "BEANS, DRY EDIBLE"];
        let mut results = Vec::new();

        for crop in &crops {
            match self
                .fetch_county_yields(crop, "MI", year_start, year_end)
                .await
            {
                Ok(data) => results.push(data),
                Err(e) => {
                    tracing::warn!("Failed to fetch {crop}: {e}");
                }
            }
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_requires_key() {
        std::env::remove_var("USDA_NASS_API_KEY");
        assert!(UsdaNassLiveProvider::create_from_env().is_err());
    }
}
