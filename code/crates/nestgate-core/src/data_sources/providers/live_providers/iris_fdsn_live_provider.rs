// SPDX-License-Identifier: AGPL-3.0-or-later
//! IRIS FDSN Live Provider — Seismic station metadata and earthquake events.
//!
//! Connects to the IRIS FDSN Web Services to provide:
//! - Station metadata (network, location, elevation)
//! - Earthquake events (magnitude, depth, location)
//!
//! IRIS provides:
//! - Free, public access (no API key required)
//! - Global seismic network coverage
//! - Decades of earthquake catalogs
//! - FDSN standard web services
//!
//! Ported from: `groundSpring/scripts/download_iris.py`
//!
//! Used by groundSpring Exp 005 (seismic inversion) for real NMSZ data.

use crate::data_sources::providers::universal_http_provider::{
    HttpProviderConfigBuilder, UniversalHttpProvider,
};
use crate::{NestGateError, Result};
use serde_json::{json, Value};
use std::collections::HashMap;
use tracing::{debug, info};

const IRIS_STATION_BASE: &str = "https://service.iris.edu/fdsnws/station/1";
const IRIS_EVENT_BASE: &str = "https://service.iris.edu/fdsnws/event/1";

/// IRIS FDSN Live Provider — seismic data from IRIS Web Services.
pub struct IrisFdsnLiveProvider {
    station_provider: UniversalHttpProvider,
    event_provider: UniversalHttpProvider,
}

impl IrisFdsnLiveProvider {
    /// Create a new IRIS FDSN provider.
    ///
    /// No API key required — IRIS provides free public access.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP providers cannot be constructed.
    pub fn new() -> Result<Self> {
        let station_config = HttpProviderConfigBuilder::new(
            IRIS_STATION_BASE.to_string(),
            "seismic_stations".to_string(),
        )
        .with_timeout(30)
        .with_metadata("provider_name".to_string(), "IRIS".to_string())
        .with_metadata(
            "provider_type".to_string(),
            "seismic_data".to_string(),
        )
        .with_metadata(
            "data_source".to_string(),
            "Incorporated Research Institutions for Seismology".to_string(),
        )
        .with_metadata("license".to_string(), "Public Domain".to_string())
        .with_metadata(
            "attribution".to_string(),
            "Data provided by IRIS (www.iris.edu)".to_string(),
        )
        .with_header(
            "User-Agent".to_string(),
            "NestGate/1.0 (groundSpring Seismic Analysis)".to_string(),
        )
        .build();

        let event_config = HttpProviderConfigBuilder::new(
            IRIS_EVENT_BASE.to_string(),
            "seismic_events".to_string(),
        )
        .with_timeout(30)
        .with_metadata("provider_name".to_string(), "IRIS".to_string())
        .with_metadata("provider_type".to_string(), "seismic_data".to_string())
        .with_header(
            "User-Agent".to_string(),
            "NestGate/1.0 (groundSpring Seismic Analysis)".to_string(),
        )
        .build();

        let station_provider = UniversalHttpProvider::new(station_config)?;
        let event_provider = UniversalHttpProvider::new(event_config)?;

        info!("Created IRIS FDSN live provider (stations + events)");

        Ok(Self {
            station_provider,
            event_provider,
        })
    }

    /// Fetch station metadata within a geographic bounding box.
    ///
    /// Returns JSON array of station records with network, station code,
    /// latitude, longitude, elevation, and site name.
    pub async fn fetch_stations(
        &self,
        min_lat: f64,
        max_lat: f64,
        min_lon: f64,
        max_lon: f64,
    ) -> Result<Value> {
        let mut params = HashMap::new();
        params.insert("format".to_string(), "text".to_string());
        params.insert("level".to_string(), "station".to_string());
        params.insert("minlatitude".to_string(), min_lat.to_string());
        params.insert("maxlatitude".to_string(), max_lat.to_string());
        params.insert("minlongitude".to_string(), min_lon.to_string());
        params.insert("maxlongitude".to_string(), max_lon.to_string());
        params.insert("channel".to_string(), "BH?".to_string());

        debug!(
            "Fetching IRIS stations: ({min_lat},{min_lon}) to ({max_lat},{max_lon})"
        );

        let response = self.station_provider.get_request("query", &params).await?;

        let text = response
            .as_str()
            .unwrap_or("");

        let stations = parse_iris_text_table(text);

        info!("IRIS stations: found {} in region", stations.len());

        Ok(json!(stations))
    }

    /// Fetch earthquake events within a geographic bounding box and time range.
    ///
    /// Returns JSON array of event records with time, latitude, longitude,
    /// depth, magnitude, and event description.
    pub async fn fetch_events(
        &self,
        min_lat: f64,
        max_lat: f64,
        min_lon: f64,
        max_lon: f64,
        start_date: &str,
        end_date: &str,
        min_magnitude: f64,
    ) -> Result<Value> {
        let mut params = HashMap::new();
        params.insert("format".to_string(), "text".to_string());
        params.insert("minlatitude".to_string(), min_lat.to_string());
        params.insert("maxlatitude".to_string(), max_lat.to_string());
        params.insert("minlongitude".to_string(), min_lon.to_string());
        params.insert("maxlongitude".to_string(), max_lon.to_string());
        params.insert("starttime".to_string(), start_date.to_string());
        params.insert("endtime".to_string(), end_date.to_string());
        params.insert("minmagnitude".to_string(), min_magnitude.to_string());
        params.insert("orderby".to_string(), "magnitude".to_string());

        debug!(
            "Fetching IRIS events: ({min_lat},{min_lon}) to ({max_lat},{max_lon}), {start_date} to {end_date}"
        );

        let response = self.event_provider.get_request("query", &params).await?;

        let text = response
            .as_str()
            .unwrap_or("");

        let events = parse_iris_text_table(text);

        info!("IRIS events: found {} in region/period", events.len());

        Ok(json!(events))
    }
}

/// Parse IRIS pipe-delimited text table into a JSON array of objects.
///
/// IRIS text format uses `|` as delimiter with a `#`-prefixed header line.
fn parse_iris_text_table(text: &str) -> Vec<Value> {
    let lines: Vec<&str> = text.lines().collect();
    if lines.len() < 2 {
        return Vec::new();
    }

    let header_line = lines[0].trim_start_matches('#').trim();
    let headers: Vec<&str> = header_line.split('|').map(str::trim).collect();

    let mut records = Vec::new();
    for line in &lines[1..] {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split('|').map(str::trim).collect();
        if fields.len() < headers.len() {
            continue;
        }
        let mut record = serde_json::Map::new();
        for (i, &header) in headers.iter().enumerate() {
            let value = fields[i];
            if let Ok(n) = value.parse::<f64>() {
                record.insert(header.to_string(), json!(n));
            } else {
                record.insert(header.to_string(), json!(value));
            }
        }
        records.push(Value::Object(record));
    }

    records
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_iris_text_table() {
        let text = "#Network|Station|Latitude|Longitude|Elevation|SiteName\n\
                    NM|PENM1|36.5|-89.9|100.0|Pemiscot County\n\
                    NM|SIUC|37.7|-89.2|200.0|Southern Illinois";
        let records = parse_iris_text_table(text);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0]["Network"], "NM");
        assert_eq!(records[0]["Latitude"], 36.5);
        assert_eq!(records[1]["Station"], "SIUC");
    }

    #[test]
    fn test_parse_empty_table() {
        assert!(parse_iris_text_table("").is_empty());
        assert!(parse_iris_text_table("#Header only").is_empty());
    }
}
