//! Data Provider JSON-RPC Handlers
//!
//! Routes `data.*` semantic methods to NestGate's live providers.
//! Handles: data.ncbi_search, data.ncbi_fetch, data.noaa_ghcnd,
//! data.iris_stations, data.iris_events

use crate::error::{NestGateError, Result};
use serde_json::{json, Value};
use tracing::{debug, info, warn};

/// data.ncbi_search — Search NCBI databases via E-utilities.
///
/// Expects params: `{ "database": "sra", "query": "...", "max_results": 20 }`
pub(super) async fn data_ncbi_search(params: &Option<Value>) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let database = params["database"]
        .as_str()
        .unwrap_or("nucleotide");
    let query = params["query"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("query", "query string required"))?;
    let max_results = params["max_results"]
        .as_u64()
        .map(|n| n as u32);

    debug!("data.ncbi_search: db={database}, query={query}");

    use crate::data_sources::providers::live_providers::NCBILiveProvider;
    use crate::data_sources::data_capabilities::GenomeDataCapability;

    let provider = NCBILiveProvider::new(
        std::env::var("NCBI_API_KEY").ok(),
        std::env::var("NCBI_EMAIL").ok(),
    ).map_err(|e| NestGateError::internal_error(format!("NCBI provider init: {e}")))?;

    let results = provider.search_genomes(query).await?;

    info!("data.ncbi_search: found {} results for '{query}'", results.len());

    Ok(json!({
        "results": results,
        "total_count": results.len(),
        "database": database,
        "query": query,
        "provider": "NCBI"
    }))
}

/// data.ncbi_fetch — Fetch a sequence from NCBI by accession.
///
/// Expects params: `{ "database": "nucleotide", "accession": "NC_000001" }`
pub(super) async fn data_ncbi_fetch(params: &Option<Value>) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let database = params["database"]
        .as_str()
        .unwrap_or("nucleotide");
    let accession = params["accession"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("accession", "accession string required"))?;

    debug!("data.ncbi_fetch: db={database}, accession={accession}");

    use crate::data_sources::providers::live_providers::NCBILiveProvider;
    use crate::data_sources::data_capabilities::GenomeDataCapability;

    let provider = NCBILiveProvider::new(
        std::env::var("NCBI_API_KEY").ok(),
        std::env::var("NCBI_EMAIL").ok(),
    ).map_err(|e| NestGateError::internal_error(format!("NCBI provider init: {e}")))?;

    let sequence = provider.get_genome_sequence(accession).await?;

    info!("data.ncbi_fetch: fetched {accession} ({} bp)", sequence.sequence.len());

    Ok(json!({
        "id": sequence.id,
        "sequence_length": sequence.sequence.len(),
        "metadata": sequence.metadata,
        "provider": "NCBI"
    }))
}

/// data.noaa_ghcnd — Fetch GHCND daily weather observations.
///
/// Expects params: `{ "station_id": "USW00094847", "start_date": "2024-01-01",
///                     "end_date": "2024-12-31", "datatypes": ["TMAX","TMIN"] }`
pub(super) async fn data_noaa_ghcnd(params: &Option<Value>) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let station_id = params["station_id"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("station_id", "station_id required"))?;
    let start_date = params["start_date"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("start_date", "start_date required"))?;
    let end_date = params["end_date"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("end_date", "end_date required"))?;

    let datatypes: Vec<&str> = params["datatypes"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default();

    debug!("data.noaa_ghcnd: station={station_id}, {start_date} to {end_date}");

    use crate::data_sources::providers::live_providers::NoaaCdoLiveProvider;

    let provider = NoaaCdoLiveProvider::create_from_env()
        .map_err(|e| NestGateError::internal_error(format!("NOAA CDO provider init: {e}")))?;

    let data = provider.fetch_ghcnd(station_id, start_date, end_date, &datatypes).await?;

    info!("data.noaa_ghcnd: fetched {station_id} data");

    Ok(json!({
        "station_id": station_id,
        "start_date": start_date,
        "end_date": end_date,
        "data": data,
        "provider": "NOAA_CDO"
    }))
}

/// data.iris_stations — Fetch seismic station metadata from IRIS FDSN.
///
/// Expects params: `{ "min_lat": 34.0, "max_lat": 40.0,
///                     "min_lon": -92.0, "max_lon": -85.0 }`
pub(super) async fn data_iris_stations(params: &Option<Value>) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let min_lat = params["min_lat"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input_with_field("min_lat", "number required"))?;
    let max_lat = params["max_lat"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input_with_field("max_lat", "number required"))?;
    let min_lon = params["min_lon"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input_with_field("min_lon", "number required"))?;
    let max_lon = params["max_lon"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input_with_field("max_lon", "number required"))?;

    debug!("data.iris_stations: ({min_lat},{min_lon}) to ({max_lat},{max_lon})");

    use crate::data_sources::providers::live_providers::IrisFdsnLiveProvider;

    let provider = IrisFdsnLiveProvider::new()
        .map_err(|e| NestGateError::internal_error(format!("IRIS provider init: {e}")))?;

    let stations = provider.fetch_stations(min_lat, max_lat, min_lon, max_lon).await?;

    info!("data.iris_stations: found {} stations", stations.as_array().map_or(0, |a| a.len()));

    Ok(json!({
        "stations": stations,
        "region": { "min_lat": min_lat, "max_lat": max_lat, "min_lon": min_lon, "max_lon": max_lon },
        "provider": "IRIS_FDSN"
    }))
}

/// data.iris_events — Fetch earthquake events from IRIS FDSN.
///
/// Expects params: `{ "min_lat": 34.0, ..., "start_date": "2023-01-01",
///                     "end_date": "2024-01-01", "min_magnitude": 2.5 }`
pub(super) async fn data_iris_events(params: &Option<Value>) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let min_lat = params["min_lat"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input_with_field("min_lat", "number required"))?;
    let max_lat = params["max_lat"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input_with_field("max_lat", "number required"))?;
    let min_lon = params["min_lon"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input_with_field("min_lon", "number required"))?;
    let max_lon = params["max_lon"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input_with_field("max_lon", "number required"))?;
    let start_date = params["start_date"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("start_date", "required"))?;
    let end_date = params["end_date"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("end_date", "required"))?;
    let min_magnitude = params["min_magnitude"].as_f64().unwrap_or(2.5);

    debug!("data.iris_events: ({min_lat},{min_lon}) to ({max_lat},{max_lon}), {start_date} to {end_date}");

    use crate::data_sources::providers::live_providers::IrisFdsnLiveProvider;

    let provider = IrisFdsnLiveProvider::new()
        .map_err(|e| NestGateError::internal_error(format!("IRIS provider init: {e}")))?;

    let events = provider.fetch_events(min_lat, max_lat, min_lon, max_lon, start_date, end_date, min_magnitude).await?;

    info!("data.iris_events: found {} events", events.as_array().map_or(0, |a| a.len()));

    Ok(json!({
        "events": events,
        "region": { "min_lat": min_lat, "max_lat": max_lat, "min_lon": min_lon, "max_lon": max_lon },
        "time_range": { "start": start_date, "end": end_date },
        "min_magnitude": min_magnitude,
        "provider": "IRIS_FDSN"
    }))
}
