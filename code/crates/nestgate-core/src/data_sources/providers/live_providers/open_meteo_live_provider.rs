// SPDX-License-Identifier: AGPL-3.0-or-later
//! Open-Meteo Live Provider — free historical weather data for sovereign agriculture.
//!
//! Connects to the Open-Meteo Archive API (https://open-meteo.com/) to provide
//! historical weather data for FAO-56 ET₀ computation and agricultural modeling.
//!
//! Open-Meteo provides:
//! - 80+ years of hourly/daily weather data
//! - 10km global resolution (ERA5 reanalysis)
//! - All variables needed for FAO-56 Penman-Monteith ET₀
//! - **Free, no API key required**, CC BY 4.0 license
//!
//! This provider was created for the airSpring ecological sciences validation
//! study, where Open-Meteo is the primary historical data source for the
//! Michigan Crop Water Atlas (100 stations, 80 years, 10 crops).
//!
//! # Reference implementation
//!
//! Python baseline: `airSpring/scripts/download_open_meteo.py`
//! Validation: airSpring v0.4.6 — 918 station-days, ET₀ R²=0.967

use crate::data_sources::data_capabilities::*;
use crate::data_sources::providers::universal_http_provider::{
    HttpProviderConfigBuilder, UniversalHttpProvider,
};
use crate::{NestGateError, Result};
use serde_json::{json, Value};
use std::collections::HashMap;
use tracing::{debug, info};

/// Open-Meteo Live Provider — free historical weather data
pub struct OpenMeteoLiveProvider {
    http_provider: UniversalHttpProvider,
}

/// Variables needed for FAO-56 Penman-Monteith ET₀
const DAILY_VARIABLES: &str = "temperature_2m_max,temperature_2m_min,\
    relative_humidity_2m_max,relative_humidity_2m_min,\
    precipitation_sum,wind_speed_10m_max,\
    shortwave_radiation_sum,et0_fao_evapotranspiration";

const HOURLY_VARIABLES: &str = "temperature_2m,relative_humidity_2m,\
    wind_speed_10m,shortwave_radiation,precipitation";

impl OpenMeteoLiveProvider {
    /// Create a new Open-Meteo provider.
    ///
    /// No API key required — Open-Meteo is free and open.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP provider cannot be constructed.
    pub fn new() -> Result<Self> {
        let config = HttpProviderConfigBuilder::new(
            "https://archive-api.open-meteo.com/v1".to_string(),
            "weather_data".to_string(),
        )
        .with_timeout(120)
        .with_metadata("provider_name".to_string(), "Open-Meteo".to_string())
        .with_metadata(
            "provider_type".to_string(),
            "weather_reanalysis".to_string(),
        )
        .with_metadata(
            "data_source".to_string(),
            "Open-Meteo ERA5 Archive".to_string(),
        )
        .with_metadata("license".to_string(), "CC BY 4.0".to_string())
        .with_metadata(
            "attribution".to_string(),
            "Weather data by Open-Meteo.com (ERA5 via Copernicus Climate Data Store)"
                .to_string(),
        )
        .with_metadata("cost".to_string(), "Free — no API key".to_string())
        .with_header(
            "User-Agent".to_string(),
            "NestGate/1.0 (airSpring Sovereign Agriculture)".to_string(),
        )
        .build();

        let http_provider = UniversalHttpProvider::new(config)?;

        info!("Created Open-Meteo live provider (free, no key)");

        Ok(Self { http_provider })
    }

    /// Download daily weather data for a location and date range.
    ///
    /// Returns JSON with daily arrays for all FAO-56 variables.
    pub async fn fetch_daily(
        &self,
        latitude: f64,
        longitude: f64,
        start_date: &str,
        end_date: &str,
    ) -> Result<Value> {
        let mut params = HashMap::new();
        params.insert("latitude".to_string(), latitude.to_string());
        params.insert("longitude".to_string(), longitude.to_string());
        params.insert("start_date".to_string(), start_date.to_string());
        params.insert("end_date".to_string(), end_date.to_string());
        params.insert("daily".to_string(), DAILY_VARIABLES.to_string());
        params.insert("timezone".to_string(), "America/Detroit".to_string());
        params.insert("wind_speed_unit".to_string(), "ms".to_string());

        debug!(
            "Fetching Open-Meteo daily: ({}, {}) {} to {}",
            latitude, longitude, start_date, end_date
        );

        self.http_provider
            .get_request("archive", &params)
            .await
    }

    /// Download hourly weather data for a location and date range.
    pub async fn fetch_hourly(
        &self,
        latitude: f64,
        longitude: f64,
        start_date: &str,
        end_date: &str,
    ) -> Result<Value> {
        let mut params = HashMap::new();
        params.insert("latitude".to_string(), latitude.to_string());
        params.insert("longitude".to_string(), longitude.to_string());
        params.insert("start_date".to_string(), start_date.to_string());
        params.insert("end_date".to_string(), end_date.to_string());
        params.insert("hourly".to_string(), HOURLY_VARIABLES.to_string());
        params.insert("timezone".to_string(), "America/Detroit".to_string());
        params.insert("wind_speed_unit".to_string(), "ms".to_string());

        debug!(
            "Fetching Open-Meteo hourly: ({}, {}) {} to {}",
            latitude, longitude, start_date, end_date
        );

        self.http_provider
            .get_request("archive", &params)
            .await
    }

    /// Fetch daily data for a named Michigan station from the atlas list.
    ///
    /// Station coordinates are looked up from the airSpring atlas station list.
    pub async fn fetch_michigan_station(
        &self,
        station_id: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Value> {
        let (lat, lon) = michigan_station_coords(station_id).ok_or_else(|| {
            NestGateError::internal_error(format!(
                "Unknown Michigan station: {station_id}"
            ))
        })?;

        self.fetch_daily(lat, lon, start_date, end_date).await
    }
}

/// Look up coordinates for a named Michigan station.
///
/// Returns (latitude, longitude) or None if unknown.
/// Stations from airSpring atlas (specs/ATLAS_STATION_LIST.md).
fn michigan_station_coords(station_id: &str) -> Option<(f64, f64)> {
    match station_id {
        "east_lansing" => Some((42.727, -84.474)),
        "grand_junction" => Some((42.375, -86.060)),
        "sparta" => Some((43.160, -85.710)),
        "hart" => Some((43.698, -86.364)),
        "west_olive" => Some((42.917, -86.167)),
        "manchester" => Some((42.153, -84.037)),
        "traverse_city" => Some((44.763, -85.621)),
        "saginaw" => Some((43.420, -83.951)),
        "kalamazoo" => Some((42.292, -85.587)),
        "marquette" => Some((46.544, -87.396)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_michigan_station_lookup() {
        assert_eq!(
            michigan_station_coords("east_lansing"),
            Some((42.727, -84.474))
        );
        assert!(michigan_station_coords("nonexistent").is_none());
    }

    #[test]
    fn test_daily_variables_complete() {
        assert!(DAILY_VARIABLES.contains("temperature_2m_max"));
        assert!(DAILY_VARIABLES.contains("et0_fao_evapotranspiration"));
        assert!(DAILY_VARIABLES.contains("shortwave_radiation_sum"));
    }
}
