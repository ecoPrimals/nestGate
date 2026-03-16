//! Data domain semantic methods
//!
//! Routes `data.*` semantic methods to NestGate's live data providers.
//! These handlers use the HTTP-based live providers (NCBI, NOAA CDO, IRIS)
//! to fetch real scientific data and return it through the semantic router.
//!
//! # Providers
//!
//! | Method            | Provider            | Data Source                    |
//! |-------------------|--------------------|---------------------------------|
//! | data.ncbi_search  | `NCBILiveProvider`  | NCBI E-utilities (genome, SRA) |
//! | data.ncbi_fetch   | `NCBILiveProvider`  | NCBI EFetch (sequences)        |
//! | data.noaa_ghcnd   | `NoaaCdoLiveProvider` | NOAA CDO GHCND daily weather |
//! | data.iris_stations| `IrisFdsnLiveProvider` | IRIS FDSN station metadata  |
//! | data.iris_events  | `IrisFdsnLiveProvider` | IRIS FDSN earthquake events |

use super::SemanticRouter;
use crate::error::{NestGateError, Result};
use serde_json::{json, Value};
use tracing::{debug, info};

/// Route data.ncbi_search to NCBI E-utilities search.
pub(super) async fn data_ncbi_search(_router: &SemanticRouter, params: Value) -> Result<Value> {
    let query = params["query"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input("query", "string required"))?;
    let database = params["database"].as_str().unwrap_or("nucleotide");

    debug!("data.ncbi_search: db={database}, query={query}");

    use crate::data_sources::providers::live_providers::NCBILiveProvider;
    use crate::data_sources::data_capabilities::GenomeDataCapability;

    let provider = NCBILiveProvider::new(
        std::env::var("NCBI_API_KEY").ok(),
        std::env::var("NCBI_EMAIL").ok(),
    ).map_err(|e| NestGateError::internal_error(format!("NCBI init: {e}")))?;

    let results = provider.search_genomes(query).await?;

    Ok(json!({
        "results": results,
        "total_count": results.len(),
        "database": database,
        "provider": "NCBI"
    }))
}

/// Route data.ncbi_fetch to NCBI EFetch.
pub(super) async fn data_ncbi_fetch(_router: &SemanticRouter, params: Value) -> Result<Value> {
    let accession = params["accession"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input("accession", "string required"))?;

    debug!("data.ncbi_fetch: accession={accession}");

    use crate::data_sources::providers::live_providers::NCBILiveProvider;
    use crate::data_sources::data_capabilities::GenomeDataCapability;

    let provider = NCBILiveProvider::new(
        std::env::var("NCBI_API_KEY").ok(),
        std::env::var("NCBI_EMAIL").ok(),
    ).map_err(|e| NestGateError::internal_error(format!("NCBI init: {e}")))?;

    let sequence = provider.get_genome_sequence(accession).await?;

    Ok(json!({
        "id": sequence.id,
        "sequence_length": sequence.sequence.len(),
        "metadata": sequence.metadata,
        "provider": "NCBI"
    }))
}

/// Route data.noaa_ghcnd to NOAA CDO GHCND provider.
pub(super) async fn data_noaa_ghcnd(_router: &SemanticRouter, params: Value) -> Result<Value> {
    let station_id = params["station_id"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input("station_id", "string required"))?;
    let start_date = params["start_date"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input("start_date", "string required"))?;
    let end_date = params["end_date"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input("end_date", "string required"))?;

    let datatypes: Vec<&str> = params["datatypes"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default();

    debug!("data.noaa_ghcnd: station={station_id}, {start_date} to {end_date}");

    use crate::data_sources::providers::live_providers::NoaaCdoLiveProvider;

    let provider = NoaaCdoLiveProvider::create_from_env()
        .map_err(|e| NestGateError::internal_error(format!("NOAA CDO init: {e}")))?;

    let data = provider.fetch_ghcnd(station_id, start_date, end_date, &datatypes).await?;

    Ok(json!({
        "station_id": station_id,
        "data": data,
        "provider": "NOAA_CDO"
    }))
}

/// Route data.iris_stations to IRIS FDSN station web service.
pub(super) async fn data_iris_stations(_router: &SemanticRouter, params: Value) -> Result<Value> {
    let min_lat = params["min_lat"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input("min_lat", "number required"))?;
    let max_lat = params["max_lat"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input("max_lat", "number required"))?;
    let min_lon = params["min_lon"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input("min_lon", "number required"))?;
    let max_lon = params["max_lon"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input("max_lon", "number required"))?;

    debug!("data.iris_stations: ({min_lat},{min_lon}) to ({max_lat},{max_lon})");

    use crate::data_sources::providers::live_providers::IrisFdsnLiveProvider;

    let provider = IrisFdsnLiveProvider::new()
        .map_err(|e| NestGateError::internal_error(format!("IRIS init: {e}")))?;

    let stations = provider.fetch_stations(min_lat, max_lat, min_lon, max_lon).await?;

    Ok(json!({
        "stations": stations,
        "provider": "IRIS_FDSN"
    }))
}

/// Route data.iris_events to IRIS FDSN event web service.
pub(super) async fn data_iris_events(_router: &SemanticRouter, params: Value) -> Result<Value> {
    let min_lat = params["min_lat"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input("min_lat", "number required"))?;
    let max_lat = params["max_lat"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input("max_lat", "number required"))?;
    let min_lon = params["min_lon"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input("min_lon", "number required"))?;
    let max_lon = params["max_lon"].as_f64()
        .ok_or_else(|| NestGateError::invalid_input("max_lon", "number required"))?;
    let start_date = params["start_date"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input("start_date", "string required"))?;
    let end_date = params["end_date"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input("end_date", "string required"))?;
    let min_magnitude = params["min_magnitude"].as_f64().unwrap_or(2.5);

    debug!("data.iris_events: ({min_lat},{min_lon}) to ({max_lat},{max_lon})");

    use crate::data_sources::providers::live_providers::IrisFdsnLiveProvider;

    let provider = IrisFdsnLiveProvider::new()
        .map_err(|e| NestGateError::internal_error(format!("IRIS init: {e}")))?;

    let events = provider.fetch_events(min_lat, max_lat, min_lon, max_lon, start_date, end_date, min_magnitude).await?;

    Ok(json!({
        "events": events,
        "provider": "IRIS_FDSN"
    }))
}
