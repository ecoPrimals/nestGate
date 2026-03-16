// SPDX-License-Identifier: AGPL-3.0-or-later
//! NOAA CDO Live Provider — GHCND daily weather observations.
//!
//! Connects to the NOAA Climate Data Online API to provide
//! historical daily weather observations from US weather stations.
//!
//! NOAA CDO provides:
//! - GHCND daily observations (temperature, precipitation, wind, etc.)
//! - 100+ years of data at station level
//! - Global coverage with dense US network
//! - **Free with token** (instant registration)
//!
//! Used by airSpring for station-level weather validation alongside the
//! Open-Meteo ERA5 reanalysis (which provides gridded data).
//!
//! # Reference implementation
//!
//! Python baseline: `airSpring/scripts/download_noaa.py`
//! Validation: airSpring v0.4.6 — GHCND Lansing, 153 station-days

use crate::data_sources::data_capabilities::*;
use crate::data_sources::providers::universal_http_provider::{
    HttpProviderConfigBuilder, UniversalHttpProvider,
};
use crate::{NestGateError, Result};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, info};

/// NOAA Climate Data Online Live Provider
pub struct NoaaCdoLiveProvider {
    http_provider: UniversalHttpProvider,
    token: String,
}

impl NoaaCdoLiveProvider {
    /// Create a new NOAA CDO provider with API token.
    ///
    /// Register for a free token at https://www.ncdc.noaa.gov/cdo-web/token
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP provider cannot be constructed.
    pub fn new(token: String) -> Result<Self> {
        let config = HttpProviderConfigBuilder::new(
            "https://www.ncei.noaa.gov/cdo-web/api/v2".to_string(),
            "weather_observations".to_string(),
        )
        .with_timeout(60)
        .with_metadata("provider_name".to_string(), "NOAA CDO".to_string())
        .with_metadata(
            "provider_type".to_string(),
            "weather_observations".to_string(),
        )
        .with_metadata(
            "data_source".to_string(),
            "NOAA National Centers for Environmental Information".to_string(),
        )
        .with_metadata(
            "license".to_string(),
            "US Government Public Domain".to_string(),
        )
        .with_metadata(
            "attribution".to_string(),
            "Data provided by NOAA NCEI".to_string(),
        )
        .with_header("token".to_string(), token.clone())
        .with_header(
            "User-Agent".to_string(),
            "NestGate/1.0 (airSpring Sovereign Agriculture)".to_string(),
        )
        .build();

        let http_provider = UniversalHttpProvider::new(config)?;

        info!("Created NOAA CDO live provider (GHCND observations)");

        Ok(Self {
            http_provider,
            token,
        })
    }

    /// Create from environment variable `NOAA_CDO_TOKEN`.
    ///
    /// # Errors
    ///
    /// Returns an error if the environment variable is not set.
    pub fn create_from_env() -> Result<Self> {
        let token = std::env::var("NOAA_CDO_TOKEN").map_err(|_| {
            NestGateError::internal_error(
                "NOAA_CDO_TOKEN environment variable not set. \
                 Register at https://www.ncdc.noaa.gov/cdo-web/token"
                    .to_string(),
            )
        })?;
        Self::new(token)
    }

    /// Fetch GHCND daily data for a station and date range.
    ///
    /// Returns JSON array of daily observations.
    pub async fn fetch_ghcnd(
        &self,
        station_id: &str,
        start_date: &str,
        end_date: &str,
        datatypes: &[&str],
    ) -> Result<Value> {
        let mut params = HashMap::new();
        params.insert("datasetid".to_string(), "GHCND".to_string());
        params.insert("stationid".to_string(), station_id.to_string());
        params.insert("startdate".to_string(), start_date.to_string());
        params.insert("enddate".to_string(), end_date.to_string());
        params.insert("units".to_string(), "metric".to_string());
        params.insert("limit".to_string(), "1000".to_string());

        if !datatypes.is_empty() {
            params.insert("datatypeid".to_string(), datatypes.join(","));
        }

        debug!(
            "Fetching NOAA GHCND: {} {} to {}",
            station_id, start_date, end_date
        );

        self.http_provider.get_request("data", &params).await
    }

    /// Fetch GHCND daily data with standard FAO-56 variables.
    ///
    /// Requests TMAX, TMIN, PRCP, AWND (average wind), and available
    /// humidity/radiation variables.
    pub async fn fetch_fao56_variables(
        &self,
        station_id: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Value> {
        self.fetch_ghcnd(
            station_id,
            start_date,
            end_date,
            &["TMAX", "TMIN", "PRCP", "AWND", "TAVG"],
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_requires_token() {
        std::env::remove_var("NOAA_CDO_TOKEN");
        assert!(NoaaCdoLiveProvider::create_from_env().is_err());
    }
}
