/*!
 * API server for the Port Manager
 */

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response, Sse},
    routing::{delete, get, post},
    Json, Router,
};
use axum::response::sse::Event;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use std::convert::Infallible;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::errors::{Error, Result};
use crate::service::{ServiceDefinition, ServiceInstance, ServiceStatus, ServiceType};
use crate::PortManager;

// API state containing the port manager
struct ApiState {
    port_manager: Arc<PortManager>,
}

// Response types
#[derive(Debug, Clone, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// Request types
#[derive(Debug, Clone, Deserialize)]
struct RegisterServiceRequest {
    name: String,
    service_type: ServiceType,
    startup_command: String,
    shutdown_command: Option<String>,
    working_directory: Option<String>,
    environment: Option<HashMap<String, String>>,
    preferred_port: Option<u16>,
    port_range: Option<(u16, u16)>,
    dependencies: Option<Vec<String>>,
    health_checks: Option<Vec<crate::service::HealthCheck>>,
    auto_restart: Option<bool>,
    max_restart_attempts: Option<u32>,
}

// Helper functions to create responses
fn success<T>(data: T) -> (StatusCode, Json<ApiResponse<T>>) {
    (
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }),
    )
}

/// Helper function to build error response
fn error<T>(code: StatusCode, message: String) -> (StatusCode, Json<ApiResponse<T>>) {
    (
        code,
        Json(ApiResponse {
            success: false,
            error: Some(message),
            data: None,
        }),
    )
}

/// Start the API server
pub async fn start_server(port_manager: &PortManager) -> Result<()> {
    // Create a clone to store in Arc
    let port_manager_clone = port_manager.clone();
    
    let state = Arc::new(ApiState {
        port_manager: Arc::new(port_manager_clone),
    });
    
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Create the router with corrected handlers
    let app = Router::new()
        // Service routes
        .route("/services", get(get_services))
        .route("/services", post(register_service))
        .route("/services/:id", get(get_service))
        .route("/services/:id", delete(unregister_service))
        .route("/services/:id/start", post(start_service))
        .route("/services/:id/stop", post(stop_service))
        .route("/services/:id/restart", post(restart_service))
        
        // Port routes
        .route("/ports", get(get_ports))
        .route("/ports/available", get(get_available_ports))
        .route("/ports/:port", get(get_port_info))
        .route("/ports/allocations", get(get_port_allocations))
        
        // System routes
        .route("/system/info", get(get_system_info))
        .route("/system/metrics", get(get_system_metrics))
        .route("/system/performance", get(get_performance_metrics))
        .route("/health", get(health_check))
        .route("/health/status", get(get_health_status))
        
        // Service metrics and monitoring
        .route("/metrics/services", get(get_service_metrics))
        .route("/metrics/stream", get(metrics_stream))
        .route("/metrics/prometheus", get(prometheus_metrics))
        .route("/metrics/timeseries/:metric", get(get_time_series))
        
        // Network routes
        .route("/network/routes", get(get_network_routes))
        .route("/network/stats", get(get_connection_stats))
        
        // Real-time streams
        .route("/stream/services", get(service_events_stream))
        
        // Dashboard comprehensive endpoint
        .route("/dashboard", get(get_dashboard_data))
        
        // Add middleware
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state);
    
    // Get the configured port from the port manager
    let config = port_manager.config.clone();
    
    // Parse the host IP address
    let ip = config.server.host.parse::<std::net::IpAddr>()
        .map_err(|e| Error::Api(format!("Failed to parse host IP address: {}", e)))?;
    
    let addr = SocketAddr::new(ip, config.server.port);
    
    tracing::info!("Starting API server on {}", addr);
    
    // Start the server in a non-blocking way
    tokio::spawn(async move {
        match axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
        {
            Ok(_) => tracing::info!("API server exited"),
            Err(e) => tracing::error!("API server error: {}", e),
        }
    });
    
    // Return immediately after spawning the server
    Ok(())
}

// Simple health check endpoint
async fn health_check() -> (StatusCode, Json<ApiResponse<&'static str>>) {
    success("ok")
}

// Service routes handlers
async fn get_services(
    State(state): State<Arc<ApiState>>,
) -> (StatusCode, Json<ApiResponse<Vec<ServiceInstance>>>) {
    let services = state.port_manager.service_registry.get_all_services();
    success(services)
}

async fn register_service(
    State(state): State<Arc<ApiState>>,
    Json(request): Json<RegisterServiceRequest>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    // Check if service already exists
    let service_id = request.name.to_lowercase().replace(' ', "_");
    
    // If service exists, stop it first to allow clean re-registration
    if let Ok(existing_service) = state.port_manager.service_registry.get_service(&service_id) {
        tracing::info!("Service {} already exists, stopping it for re-registration", service_id);
        
        // Stop health monitoring
        let _ = state.port_manager.health_monitor.stop_monitoring(&service_id);
        
        // Stop the existing service if it's running
        if existing_service.status == ServiceStatus::Running {
            match state.port_manager.process_manager.stop_service(&existing_service).await {
                Ok(_) => {
                    tracing::info!("Successfully stopped existing service {} for re-registration", service_id);
                },
                Err(e) => {
                    tracing::warn!("Failed to stop existing service {} for re-registration: {}", service_id, e);
                    // Continue anyway
                }
            }
        }
        
        // Unregister the old service
        match state.port_manager.service_registry.unregister_service(&service_id) {
            Ok(_) => {
                tracing::info!("Successfully unregistered existing service {}", service_id);
            },
            Err(e) => {
                tracing::warn!("Failed to unregister existing service {}: {}", service_id, e);
                // Continue anyway
            }
        }
        
        // Wait for cleanup to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    // Create the new service definition
    let definition = ServiceDefinition {
        id: service_id.clone(),
        name: request.name,
        service_type: request.service_type,
        startup_command: request.startup_command,
        shutdown_command: request.shutdown_command,
        working_directory: request.working_directory.map(std::path::PathBuf::from),
        environment: request.environment.unwrap_or_default(),
        preferred_port: request.preferred_port,
        port_range: request.port_range,
        dependencies: request.dependencies.unwrap_or_default(),
        health_checks: request.health_checks.unwrap_or_default(),
        auto_restart: request.auto_restart.unwrap_or(true),
        max_restart_attempts: request.max_restart_attempts.unwrap_or(3),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    // Register the new service
    match state.port_manager.service_registry.register_service(definition) {
        Ok(id) => {
            tracing::info!("Successfully registered service: {}", id);
            success(id)
        },
        Err(e) => error(StatusCode::CONFLICT, e.to_string()),
    }
}

async fn get_service(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<ServiceInstance>>) {
    match state.port_manager.service_registry.get_service(&id) {
        Ok(service) => success(service),
        Err(e) => error(StatusCode::NOT_FOUND, e.to_string()),
    }
}

async fn unregister_service(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<()>>) {
    match state.port_manager.service_registry.unregister_service(&id) {
        Ok(_) => success(()),
        Err(e) => error(StatusCode::NOT_FOUND, e.to_string()),
    }
}

async fn start_service(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<u32>>) {
    // Check if service exists and is already running - if so, stop it first
    match state.port_manager.service_registry.get_service(&id) {
        Ok(existing_service) => {
            // If service is running, stop it first for a clean restart
            if existing_service.status == ServiceStatus::Running {
                tracing::info!("Service {} is already running, stopping it first for clean restart", id);
                
                // Stop health monitoring
                let _ = state.port_manager.health_monitor.stop_monitoring(&id);
                
                // Stop the existing service
                match state.port_manager.process_manager.stop_service(&existing_service).await {
                    Ok(_) => {
                        tracing::info!("Successfully stopped existing service {}", id);
                    },
                    Err(e) => {
                        tracing::warn!("Failed to cleanly stop existing service {}: {}", id, e);
                        // Continue anyway - the process manager will handle cleanup
                    }
                }
                
                // Update service status to stopped
                let _ = state.port_manager.service_registry.update_service_status(&id, ServiceStatus::Stopped);
                
                // Wait a moment for cleanup to complete
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            }
        },
        Err(_) => {
            return error(StatusCode::NOT_FOUND, format!("Service {} not found", id));
        }
    };
    
    // Get the service (fresh copy after potential cleanup)
    let service = match state.port_manager.service_registry.get_service(&id) {
        Ok(service) => service,
        Err(e) => return error(StatusCode::NOT_FOUND, e.to_string()),
    };
    
    // Allocate port if not already allocated
    if service.port.is_none() {
        // Allocate a port
        let port = match state.port_manager.port_allocator.allocate_port(
            &id,
            service.definition.service_type.clone(),
            service.definition.preferred_port,
        ).await {
            Ok(port) => port,
            Err(e) => return error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };
        
        // Update service with allocated port
        match state.port_manager.service_registry.update_service_port(&id, port) {
            Ok(_) => {},
            Err(e) => return error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
    }
    
    // Get updated service
    let service = match state.port_manager.service_registry.get_service(&id) {
        Ok(service) => service,
        Err(e) => return error(StatusCode::NOT_FOUND, e.to_string()),
    };
    
    // Update service status
    match state.port_manager.service_registry.update_service_status(&id, ServiceStatus::Starting) {
        Ok(_) => {},
        Err(e) => return error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
    
    // Start the service (the process manager will handle cleanup of any existing processes)
    match state.port_manager.process_manager.start_service(&service).await {
        Ok(pid) => {
            // Update service status and PID
            let _ = state.port_manager.service_registry.update_service_pid(&id, pid);
            let _ = state.port_manager.service_registry.update_service_status(&id, ServiceStatus::Running);
            
            // Start health monitoring
            let _ = state.port_manager.health_monitor.start_monitoring(&id, service.definition.health_checks);
            
            tracing::info!("Successfully started service {} with PID {}", id, pid);
            success(pid)
        },
        Err(e) => {
            // Update service status
            let _ = state.port_manager.service_registry.update_service_status(
                &id,
                ServiceStatus::Failed(e.to_string()),
            );
            
            error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        },
    }
}

async fn stop_service(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<()>>) {
    // Get the service
    let service = match state.port_manager.service_registry.get_service(&id) {
        Ok(service) => service,
        Err(e) => return error(StatusCode::NOT_FOUND, e.to_string()),
    };
    
    // Update service status
    match state.port_manager.service_registry.update_service_status(&id, ServiceStatus::Stopping) {
        Ok(_) => {},
        Err(e) => return error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
    
    // Stop health monitoring
    let _ = state.port_manager.health_monitor.stop_monitoring(&id);
    
    // Stop the service
    match state.port_manager.process_manager.stop_service(&service).await {
        Ok(_) => {
            // Clean up allocated ports for this service
            match state.port_manager.port_allocator.deallocate_service_ports(&id) {
                Ok(deallocated_ports) => {
                    if !deallocated_ports.is_empty() {
                        tracing::info!("Deallocated ports {:?} for service {}", deallocated_ports, id);
                    }
                },
                Err(e) => {
                    tracing::warn!("Failed to deallocate ports for service {}: {}", id, e);
                    // Continue anyway - ports will eventually be cleaned up
                }
            }
            
            // Update service status and clear PID
            let _ = state.port_manager.service_registry.update_service_status(&id, ServiceStatus::Stopped);
            let _ = state.port_manager.service_registry.update_service_pid(&id, 0); // Clear PID
            
            tracing::info!("Successfully stopped service {}", id);
            success(())
        },
        Err(e) => {
            // Update service status
            let _ = state.port_manager.service_registry.update_service_status(
                &id,
                ServiceStatus::Failed(e.to_string()),
            );
            
            error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        },
    }
}

async fn restart_service(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<u32>>) {
    // Get the service
    let service = match state.port_manager.service_registry.get_service(&id) {
        Ok(service) => service,
        Err(e) => return error(StatusCode::NOT_FOUND, e.to_string()),
    };
    
    // Update service status
    match state.port_manager.service_registry.update_service_status(&id, ServiceStatus::Restarting) {
        Ok(_) => {},
        Err(e) => return error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
    
    // Stop health monitoring
    let _ = state.port_manager.health_monitor.stop_monitoring(&id);
    
    // Stop the service
    match state.port_manager.process_manager.stop_service(&service).await {
        Ok(_) => {},
        Err(e) => {
            tracing::warn!("Failed to stop service {}: {}", id, e);
            // Continue with restart anyway
        },
    }
    
    // Get updated service
    let service = match state.port_manager.service_registry.get_service(&id) {
        Ok(service) => service,
        Err(e) => return error(StatusCode::NOT_FOUND, e.to_string()),
    };
    
    // Start the service
    match state.port_manager.process_manager.start_service(&service).await {
        Ok(pid) => {
            // Update service status and PID
            let _ = state.port_manager.service_registry.update_service_pid(&id, pid);
            let _ = state.port_manager.service_registry.update_service_status(&id, ServiceStatus::Running);
            
            // Start health monitoring
            let _ = state.port_manager.health_monitor.start_monitoring(&id, service.definition.health_checks);
            
            success(pid)
        },
        Err(e) => {
            // Update service status
            let _ = state.port_manager.service_registry.update_service_status(
                &id,
                ServiceStatus::Failed(e.to_string()),
            );
            
            error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        },
    }
}

// Port routes handlers
async fn get_ports(
    State(state): State<Arc<ApiState>>,
) -> (StatusCode, Json<ApiResponse<HashMap<u16, crate::port::AllocatedPort>>>) {
    let ports = state.port_manager.port_allocator.get_all_allocated_ports();
    success(ports)
}

#[derive(Debug, Clone, Serialize)]
struct AvailablePortsResponse {
    available_ports: HashMap<ServiceType, Vec<u16>>,
}

async fn get_available_ports(
    State(state): State<Arc<ApiState>>,
) -> (StatusCode, Json<ApiResponse<AvailablePortsResponse>>) {
    let mut available_ports = HashMap::new();
    
    for (service_type, range) in &state.port_manager.config.port_ranges {
        let mut ports = Vec::new();
        
        for port in range.start..=range.end {
            if state.port_manager.port_allocator.is_port_available(port).await {
                ports.push(port);
            }
        }
        
        available_ports.insert(service_type.clone(), ports);
    }
    
    success(AvailablePortsResponse { available_ports })
}

async fn get_port_info(
    State(state): State<Arc<ApiState>>,
    Path(port): Path<u16>,
) -> (StatusCode, Json<ApiResponse<Option<crate::port::AllocatedPort>>>) {
    let port_info = state.port_manager.port_allocator.get_port_info(port);
    success(port_info)
}

// System routes handlers
#[derive(Debug, Clone, Serialize)]
struct SystemInfo {
    version: String,
    port_manager_port: u16,
    total_services: usize,
    running_services: usize,
    allocated_ports: usize,
    active_processes: usize,
    uptime_seconds: u64,
}

async fn get_system_info(
    State(state): State<Arc<ApiState>>,
) -> (StatusCode, Json<ApiResponse<SystemInfo>>) {
    let services = state.port_manager.service_registry.get_all_services();
    let running_services = services
        .iter()
        .filter(|s| matches!(s.status, ServiceStatus::Running))
        .count();
    
    let ports = state.port_manager.port_allocator.get_all_allocated_ports();
    let processes = state.port_manager.process_manager.get_all_processes();
    
    let info = SystemInfo {
        version: crate::VERSION.to_string(),
        port_manager_port: state.port_manager.config.server.port,
        total_services: services.len(),
        running_services,
        allocated_ports: ports.len(),
        active_processes: processes.len(),
        uptime_seconds: 0, // This would need to be tracked separately
    };
    
    success(info)
}

/// Get system metrics
async fn get_system_metrics(
    State(state): State<Arc<ApiState>>,
) -> (StatusCode, Json<ApiResponse<crate::metrics::SystemMetrics>>) {
    let metrics = state.port_manager.metrics_collector.get_system_metrics().await;
    success(metrics)
}

/// Get service metrics
async fn get_service_metrics(
    State(state): State<Arc<ApiState>>,
) -> (StatusCode, Json<ApiResponse<HashMap<String, crate::metrics::ServiceMetrics>>>) {
    let metrics = state.port_manager.metrics_collector.get_service_metrics().await;
    success(metrics)
}

/// Get performance metrics
async fn get_performance_metrics(
    State(state): State<Arc<ApiState>>,
) -> (StatusCode, Json<ApiResponse<crate::metrics::PerformanceMetrics>>) {
    let metrics = state.port_manager.metrics_collector.get_performance_metrics().await;
    success(metrics)
}

/// Get all health status
async fn get_health_status(
    State(state): State<Arc<ApiState>>,
) -> (StatusCode, Json<ApiResponse<HashMap<String, crate::health::HealthCheckResult>>>) {
    let health = state.port_manager.health_monitor.get_all_health_status().await;
    success(health)
}

/// Get network routes
async fn get_network_routes(
    State(state): State<Arc<ApiState>>,
) -> (StatusCode, Json<ApiResponse<HashMap<String, crate::network::ProxyRoute>>>) {
    let routes = state.port_manager.network_manager.get_routes().await;
    success(routes)
}

/// Get connection statistics
async fn get_connection_stats(
    State(state): State<Arc<ApiState>>,
) -> (StatusCode, Json<ApiResponse<crate::network::ConnectionStats>>) {
    let stats = state.port_manager.network_manager.get_connection_stats().await;
    success(stats)
}

/// Get port allocations
async fn get_port_allocations(
    State(state): State<Arc<ApiState>>,
) -> (StatusCode, Json<ApiResponse<HashMap<String, u16>>>) {
    let allocations = state.port_manager.port_allocator.get_all_allocations().await;
    success(allocations)
}

/// Get live dashboard data (comprehensive endpoint)
async fn get_dashboard_data(
    State(state): State<Arc<ApiState>>,
) -> (StatusCode, Json<ApiResponse<DashboardData>>) {
    let system_metrics = state.port_manager.metrics_collector.get_system_metrics().await;
    let service_metrics = state.port_manager.metrics_collector.get_service_metrics().await;
    let health_status = state.port_manager.health_monitor.get_all_health_status().await;
    let services = state.port_manager.service_registry.get_all_services();
    let port_allocations = state.port_manager.port_allocator.get_all_allocations().await;
    let connection_stats = state.port_manager.network_manager.get_connection_stats().await;
    
    let dashboard_data = DashboardData {
        system_metrics,
        service_metrics,
        health_status,
        services,
        port_allocations,
        connection_stats,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    success(dashboard_data)
}

#[derive(Serialize)]
struct DashboardData {
    system_metrics: crate::metrics::SystemMetrics,
    service_metrics: HashMap<String, crate::metrics::ServiceMetrics>,
    health_status: HashMap<String, crate::health::HealthCheckResult>,
    services: Vec<ServiceInstance>,
    port_allocations: HashMap<String, u16>,
    connection_stats: crate::network::ConnectionStats,
    timestamp: u64,
}

/// Real-time metrics stream (Server-Sent Events)
async fn metrics_stream(
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    let port_manager = Arc::clone(&state.port_manager);
    
    let stream = async_stream::stream! {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        
        loop {
            interval.tick().await;
            
            let system_metrics = port_manager.metrics_collector.get_system_metrics().await;
            let data = serde_json::to_string(&system_metrics).unwrap_or_default();
            
            yield Ok::<_, Infallible>(Event::default()
                .event("metrics")
                .data(data));
        }
    };

    Sse::new(stream)
        .keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(30))
                .text("keep-alive"),
        )
}

/// Service events stream (Server-Sent Events)
async fn service_events_stream(
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    let port_manager = Arc::clone(&state.port_manager);
    
    let stream = async_stream::stream! {
        let mut interval = tokio::time::interval(Duration::from_secs(2));
        
        loop {
            interval.tick().await;
            
            let services = port_manager.service_registry.get_all_services();
            let data = serde_json::to_string(&services).unwrap_or_default();
            
            yield Ok::<_, Infallible>(Event::default()
                .event("services")
                .data(data));
        }
    };

    Sse::new(stream)
        .keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(30))
                .text("keep-alive"),
        )
}

/// Get time series metrics
async fn get_time_series(
    State(state): State<Arc<ApiState>>,
    Path(metric_name): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> (StatusCode, Json<ApiResponse<Vec<crate::metrics::MetricsPoint>>>) {
    let since = params.get("since")
        .and_then(|s| s.parse::<u64>().ok());
        
    let data = state.port_manager.metrics_collector
        .get_time_series(&metric_name, since)
        .await;
        
    success(data)
}

/// Prometheus metrics endpoint
async fn prometheus_metrics(
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    let metrics = state.port_manager.metrics_collector.export_prometheus_metrics().await;
    
    Response::builder()
        .header("Content-Type", "text/plain; version=0.0.4")
        .body(metrics)
        .unwrap()
} 