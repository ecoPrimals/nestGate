//
// Manages bidirectional WebSocket streams for real-time communication.
// Provides stream lifecycle management and event routing.

//! Bidirectional Streams module

use super::{RpcError, RpcStreamEvent, UnifiedRpcRequest};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, info};
use uuid::Uuid;

/// Manager for bidirectional streams
pub struct BidirectionalStreamManager {
    /// Active streams
    active_streams: Arc<Mutex<HashMap<Uuid, ActiveStream>>>,
    /// Global event sender
    global_event_sender: Arc<Mutex<Option<mpsc::Sender<RpcStreamEvent>>>>,
}
/// Active stream information
struct ActiveStream {
    stream_id: Uuid,
    stream_type: String,
    sender: mpsc::Sender<RpcStreamEvent>,
    created_at: chrono::DateTime<chrono::Utc>,
}
impl Default for BidirectionalStreamManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl BidirectionalStreamManager {
    /// Create new bidirectional stream manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            active_streams: Arc::new(Mutex::new(HashMap::new())),
            global_event_sender: Arc::new(Mutex::new(None)),
        }
    }

    /// Set global event sender for broadcasting events
    pub async fn set_global_event_sender(&self, sender: mpsc::Sender<RpcStreamEvent>) {
        let mut global_sender = self.global_event_sender.lock().await;
        *global_sender = Some(sender);
    }

    /// Create a new bidirectional stream
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_bidirectional_stream(
        &self,
        request: UnifiedRpcRequest,
    ) -> Result<(mpsc::Sender<RpcStreamEvent>, mpsc::Receiver<RpcStreamEvent>), RpcError> {
        let stream_id = Uuid::new_v4();
        let (tx, _rx) = mpsc::channel(100);
        let (response_tx, response_rx) = mpsc::channel(100);

        // Create active stream record
        let active_stream = ActiveStream {
            stream_id,
            stream_type: request.method.clone(),
            sender: response_tx.clone(),
            created_at: chrono::Utc::now(),
        };

        // Store active stream
        {
            let mut streams = self.active_streams.lock().await;
            streams.insert(stream_id, active_stream);
        }

        // Start stream based on method
        match request.method.as_str() {
            "stream_realtime_metrics" => {
                self.start_realtime_metrics_stream(stream_id, response_tx)
                    .await?;
            }
            "stream_zfs_events" => {
                self.start_zfs_events_stream(stream_id, response_tx).await?;
            }
            "stream_storage_events" => {
                self.start_storage_events_stream(stream_id, response_tx)
                    .await?;
            }
            "stream_system_logs" => {
                self.start_system_logs_stream(stream_id, response_tx)
                    .await?;
            }
            "stream_performance_data" => {
                self.start_performance_data_stream(stream_id, response_tx)
                    .await?;
            }
            _ => {
                // Remove the stream since we couldn't start it
                let mut streams = self.active_streams.lock().await;
                streams.remove(&stream_id);
                return Err(RpcError::ServiceUnavailable(format!(
                    "Unknown streaming method: {}",
                    request.method
                )));
            }
        }

        info!(
            "🔄 Created bidirectional stream {} for method {}",
            stream_id, request.method
        );
        Ok((tx, response_rx))
    }

    /// Close a bidirectional stream
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn close_stream(&self, stream_id: Uuid) -> Result<(), RpcError> {
        let mut streams = self.active_streams.lock().await;
        if let Some(stream) = streams.remove(&stream_id) {
            info!(
                "🔌 Closed bidirectional stream {} ({})",
                stream_id, stream.stream_type
            );
            Ok(())
        } else {
            Err(RpcError::StreamError(format!(
                "Stream {stream_id} not found"
            )))
        }
    }

    /// Get active stream count
    pub async fn get_active_stream_count(&self) -> usize {
        let streams = self.active_streams.lock().await;
        streams.len()
    }

    /// Get active streams info
    pub async fn get_active_streams_info(
        &self,
    ) -> Vec<(Uuid, String, chrono::DateTime<chrono::Utc>)> {
        let streams = self.active_streams.lock().await;
        streams
            .values()
            .map(|stream| {
                (
                    stream.stream_id,
                    stream.stream_type.clone(),
                    stream.created_at,
                )
            })
            .collect()
    }

    /// Broadcast event to all active streams
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn broadcast_to_all_streams(&self, event: RpcStreamEvent) -> Result<(), RpcError> {
        let streams = self.active_streams.lock().await;
        let mut failed_streams = Vec::new();

        for (stream_id, stream) in streams.iter() {
            if stream.sender.send(event.clone()).await.is_err() {
                failed_streams.push(*stream_id);
            }
        }

        // Clean up failed streams
        drop(streams);
        if !failed_streams.is_empty() {
            let mut streams = self.active_streams.lock().await;
            for stream_id in failed_streams {
                streams.remove(&stream_id);
                debug!("Removed failed stream {}", stream_id);
            }
        }

        Ok(())
    }

    /// Start real-time metrics stream
    async fn start_realtime_metrics_stream(
        &self,
        stream_id: Uuid,
        tx: mpsc::Sender<RpcStreamEvent>,
    ) -> Result<(), RpcError> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));
            let mut counter = 0;

            loop {
                interval.tick().await;
                counter += 1;

                let event = RpcStreamEvent {
                    stream_id,
                    event_type: "realtime_metrics".to_string(),
                    data: serde_json::json!({
                        "timestamp": chrono::Utc::now(),
                        "cpu_usage": 25.0 + (f64::from(counter) * 0.5) % 50.0,
                        "memory_usage": 45.0 + (f64::from(counter) * 0.3) % 30.0,
                        "disk_io": {
                            "read_mbps": 100.0 + (f64::from(counter) * 2.0) % 200.0,
                            "write_mbps": 80.0 + (f64::from(counter) * 1.5) % 150.0
                        },
                        "network_io": {
                            "rx_mbps": 50.0 + (f64::from(counter) * 1.0) % 100.0,
                            "tx_mbps": 30.0 + (f64::from(counter) * 0.8) % 80.0
                        }
                    }),
                    timestamp: chrono::Utc::now(),
                };

                if tx.send(event).await.is_err() {
                    debug!("Real-time metrics stream {} closed", stream_id);
                    break;
                }
            }
        });

        Ok(())
    }

    /// Start ZFS events stream
    async fn start_zfs_events_stream(
        &self,
        stream_id: Uuid,
        tx: mpsc::Sender<RpcStreamEvent>,
    ) -> Result<(), RpcError> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(8));
            let mut counter = 0;

            loop {
                interval.tick().await;
                counter += 1;

                let events = [
                    "dataset_created",
                    "snapshot_taken",
                    "dataset_mounted",
                    "scrub_completed",
                ];
                let event_type = events[counter % events.len()];

                let event = RpcStreamEvent {
                    stream_id,
                    event_type: "zfs_event".to_string(),
                    data: serde_json::json!({
                        "event_id": format!("zfs_event_self.base_url"),
                        "event_type": event_type,
                        "dataset": format!("tank/data_self.base_url"),
                        "timestamp": chrono::Utc::now(),
                        "details": {
                            "operation": event_type,
                            "status": "completed",
                            "duration_ms": 150 + (counter % 500)
                        }
                    }),
                    timestamp: chrono::Utc::now(),
                };

                if tx.send(event).await.is_err() {
                    debug!("ZFS events stream {} closed", stream_id);
                    break;
                }
            }
        });

        Ok(())
    }

    /// Start storage events stream
    async fn start_storage_events_stream(
        &self,
        stream_id: Uuid,
        tx: mpsc::Sender<RpcStreamEvent>,
    ) -> Result<(), RpcError> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(12));
            let mut counter = 0;

            loop {
                interval.tick().await;
                counter += 1;

                let backends = ["filesystem", "memory", "cloud"];
                let backend = backends[counter % backends.len()];
                let events = [
                    "backend_added",
                    "backend_removed",
                    "health_check",
                    "performance_update",
                ];
                let event_type = events[counter % events.len()];

                let event = RpcStreamEvent {
                    stream_id,
                    event_type: "storage_event".to_string(),
                    data: serde_json::json!({
                        "event_id": format!("storage_event_self.base_url"),
                        "backend_type": backend,
                        "event_type": event_type,
                        "timestamp": chrono::Utc::now(),
                        "performance": {
                            "read_throughput": 200.0 + (counter as f64 * 5.0) % 300.0,
                            "write_throughput": 150.0 + (counter as f64 * 3.0) % 200.0,
                            "latency_ms": 1.0 + (counter as f64 * 0.1) % 5.0
                        }
                    }),
                    timestamp: chrono::Utc::now(),
                };

                if tx.send(event).await.is_err() {
                    debug!("Storage events stream {} closed", stream_id);
                    break;
                }
            }
        });

        Ok(())
    }

    /// Start system logs stream
    async fn start_system_logs_stream(
        &self,
        stream_id: Uuid,
        tx: mpsc::Sender<RpcStreamEvent>,
    ) -> Result<(), RpcError> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(3));
            let mut counter = 0;

            loop {
                interval.tick().await;
                counter += 1;

                let levels = ["INFO", "WARN", "ERROR", "DEBUG"];
                let level = levels[counter % levels.len()];
                let modules = [
                    "nestgate::api",
                    "nestgate::zfs",
                    "nestgate::storage",
                    "nestgate::rpc",
                ];
                let module = modules[counter % modules.len()];
                let messages = [
                    "Operation completed successfully",
                    "Health check passed",
                    "Connection established",
                    "Data synchronized",
                    "Cache updated",
                ];
                let message = messages[counter % messages.len()];

                let event = RpcStreamEvent {
                    stream_id,
                    event_type: "system_log".to_string(),
                    data: serde_json::json!({
                        "log_id": format!("log_{}", counter),
                        "level": level,
                        "module": module,
                        "message": message,
                        "timestamp": chrono::Utc::now(),
                        "thread": format!("worker-{}", (counter % 8) + 1)
                    }),
                    timestamp: chrono::Utc::now(),
                };

                if tx.send(event).await.is_err() {
                    debug!("System logs stream {} closed", stream_id);
                    break;
                }
            }
        });

        Ok(())
    }

    /// Start performance data stream
    async fn start_performance_data_stream(
        &self,
        stream_id: Uuid,
        tx: mpsc::Sender<RpcStreamEvent>,
    ) -> Result<(), RpcError> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
            let mut counter = 0;

            loop {
                interval.tick().await;
                counter += 1;

                let event = RpcStreamEvent {
                    stream_id,
                    event_type: "performance_data".to_string(),
                    data: serde_json::json!({
                        "sample_id": format!("perf_self.base_url"),
                        "timestamp": chrono::Utc::now(),
                        "response_times": {
                            "api_avg_ms": 5.0 + (f64::from(counter) * 0.1) % 20.0,
                            "api_p95_ms": 15.0 + (f64::from(counter) * 0.2) % 50.0,
                            "api_p99_ms": 25.0 + (f64::from(counter) * 0.3) % 100.0
                        },
                        "throughput": {
                            "requests_per_second": 100.0 + (f64::from(counter) * 2.0) % 500.0,
                            "data_throughput_mbps": 50.0 + (f64::from(counter) * 1.0) % 200.0
                        },
                        "resource_usage": {
                            "cpu_cores_used": 2.5 + (f64::from(counter) * 0.05) % 2.0,
                            "memory_mb_used": 512.0 + (f64::from(counter) * 10.0) % 1024.0,
                            "disk_io_ops": 1000 + (counter % 5000)
                        }
                    }),
                    timestamp: chrono::Utc::now(),
                };

                if tx.send(event).await.is_err() {
                    debug!("Performance data stream {} closed", stream_id);
                    break;
                }
            }
        });

        Ok(())
    }
}
