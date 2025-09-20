/// Fallback utilities for universal primal discovery
/// This module provides fallback values when universal adapter discovery fails
/// Get fallback port for a service when discovery fails
#[must_use]
pub const fn get_fallback_port(service_name: &str) -> u16 {
    match service_name {
        "api" => 8080,
        "web" => 3000,
        "metrics" => 9090,
        "metrics_export" => 9090, // Capability-based instead of vendor-specific
        "nfs" => 2049,
        "smb" => 445,
        "cifs" => 445,
        "ftp" => 21,
        "ssh" => 22,
        "http" => 80,
        "https" => 443,
        "orchestration" => 8081,
        "coordination" => 8082,
        "compute" => 8083,
        "ai" => 8084,
        "security" => 8085,
        "auth" => 8086,
        _ => 8080, // Default fallback
    }
}
