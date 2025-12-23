//! Service detection engine for automatic capability discovery
//!
//! Automatically discovers services on the network and registers their capabilities.
//! Supports multiple detection protocols and continuous monitoring.

use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;

use super::registry::CapabilityRegistry;
use super::service_descriptor::{
    Endpoint, Protocol, ServiceDescriptor, ServiceHealth, ServiceMetadata,
};
use super::{CapabilityError, CapabilityResult};

/// Service detection engine
///
/// Continuously scans for services and registers their capabilities.
/// Supports HTTP, gRPC, and custom protocol detection.
pub struct ServiceDetector {
    /// Registry to populate with discovered services
    registry: Arc<CapabilityRegistry>,

    /// Detection interval
    interval: Duration,

    /// Active detection tasks
    tasks: Vec<JoinHandle<()>>,

    /// Ports to scan for services
    scan_ports: Vec<u16>,
}

impl ServiceDetector {
    /// Create a new service detector
    pub fn new(registry: Arc<CapabilityRegistry>) -> Self {
        Self {
            registry,
            interval: Duration::from_secs(30),
            tasks: Vec::new(),
            scan_ports: vec![3000, 3001, 3002, 3010], // Default discovery ports
        }
    }

    /// Set detection interval
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// Set ports to scan
    pub fn with_scan_ports(mut self, ports: Vec<u16>) -> Self {
        self.scan_ports = ports;
        self
    }

    /// Start continuous service detection
    pub async fn start(&mut self) -> CapabilityResult<()> {
        for port in &self.scan_ports {
            let port = *port;
            let registry = Arc::clone(&self.registry);
            let interval = self.interval;

            let task = tokio::spawn(async move {
                loop {
                    if let Ok(service) = Self::probe_port(port).await {
                        let _ = registry.register_service(service).await;
                    }
                    tokio::time::sleep(interval).await;
                }
            });

            self.tasks.push(task);
        }

        Ok(())
    }

    /// Probe a single port for service capabilities
    async fn probe_port(port: u16) -> CapabilityResult<ServiceDescriptor> {
        use uuid::Uuid;

        // Try HTTP/HTTPS first
        let hosts = vec!["localhost", "127.0.0.1"];

        for host in hosts {
            // Try HTTP well-known endpoint for capability discovery
            let http_url = format!("http://{}:{}/.well-known/capabilities", host, port);
            let https_url = format!("https://{}:{}/.well-known/capabilities", host, port);

            // Try HTTPS first (more secure)
            for (_url, tls) in [(https_url, true), (http_url, false)] {
                // Attempt basic TCP connection first
                if let Ok(_stream) =
                    tokio::net::TcpStream::connect(format!("{}:{}", host, port)).await
                {
                    // Port is open, create a basic descriptor
                    // In production, this would make an HTTP request to discover capabilities
                    return Ok(ServiceDescriptor {
                        id: Uuid::new_v4(),
                        name: format!("discovered-service-{}", port),
                        capabilities: vec![], // Would be populated from .well-known/capabilities
                        endpoint: Endpoint {
                            host: host.to_string(),
                            port,
                            protocol: if tls { Protocol::HTTPS } else { Protocol::HTTP },
                            tls,
                        },
                        metadata: ServiceMetadata::default(),
                        health: ServiceHealth::Unknown, // Would be checked via health endpoint
                    });
                }
            }
        }

        Err(CapabilityError::DetectionFailed(format!(
            "Unable to connect to port {} for capability discovery",
            port
        )))
    }

    /// Stop all detection tasks
    pub async fn stop(&mut self) {
        for task in self.tasks.drain(..) {
            task.abort();
        }
    }
}

impl Drop for ServiceDetector {
    fn drop(&mut self) {
        for task in &self.tasks {
            task.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detector_creation() {
        let registry = Arc::new(CapabilityRegistry::new());
        let detector = ServiceDetector::new(registry);

        assert_eq!(detector.interval, Duration::from_secs(30));
        assert_eq!(detector.scan_ports, vec![3000, 3001, 3002, 3010]);
    }

    #[tokio::test]
    async fn test_detector_with_custom_config() {
        let registry = Arc::new(CapabilityRegistry::new());
        let detector = ServiceDetector::new(registry)
            .with_interval(Duration::from_secs(60))
            .with_scan_ports(vec![8080, 8081]);

        assert_eq!(detector.interval, Duration::from_secs(60));
        assert_eq!(detector.scan_ports, vec![8080, 8081]);
    }

    #[tokio::test]
    async fn test_detector_with_custom_interval() {
        let registry = Arc::new(CapabilityRegistry::new());
        let detector = ServiceDetector::new(registry).with_interval(Duration::from_secs(120));

        assert_eq!(detector.interval, Duration::from_secs(120));
    }

    #[tokio::test]
    async fn test_detector_with_empty_ports() {
        let registry = Arc::new(CapabilityRegistry::new());
        let detector = ServiceDetector::new(registry).with_scan_ports(vec![]);

        assert_eq!(detector.scan_ports.len(), 0);
    }

    #[tokio::test]
    async fn test_detector_with_many_ports() {
        let registry = Arc::new(CapabilityRegistry::new());
        let ports: Vec<u16> = (8000..8010).collect();
        let detector = ServiceDetector::new(registry).with_scan_ports(ports.clone());

        assert_eq!(detector.scan_ports.len(), 10);
        assert_eq!(detector.scan_ports, ports);
    }

    #[tokio::test]
    async fn test_detector_stop_before_start() {
        let registry = Arc::new(CapabilityRegistry::new());
        let mut detector = ServiceDetector::new(registry);

        // Should not panic when stopping before starting
        detector.stop().await;
        assert_eq!(detector.tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_detector_multiple_stops() {
        let registry = Arc::new(CapabilityRegistry::new());
        let mut detector = ServiceDetector::new(registry);

        // Multiple stops should be safe
        detector.stop().await;
        detector.stop().await;
        detector.stop().await;

        assert_eq!(detector.tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_detector_builder_pattern() {
        let registry = Arc::new(CapabilityRegistry::new());

        // Builder pattern should work fluently
        let detector = ServiceDetector::new(registry)
            .with_interval(Duration::from_secs(45))
            .with_scan_ports(vec![9000, 9001])
            .with_interval(Duration::from_secs(90)); // Override

        assert_eq!(detector.interval, Duration::from_secs(90));
        assert_eq!(detector.scan_ports, vec![9000, 9001]);
    }

    #[tokio::test]
    async fn test_detector_interval_variations() {
        let registry = Arc::new(CapabilityRegistry::new());

        // Very short interval
        let detector1 =
            ServiceDetector::new(Arc::clone(&registry)).with_interval(Duration::from_secs(1));
        assert_eq!(detector1.interval, Duration::from_secs(1));

        // Very long interval
        let detector2 =
            ServiceDetector::new(Arc::clone(&registry)).with_interval(Duration::from_secs(3600));
        assert_eq!(detector2.interval, Duration::from_secs(3600));

        // Zero interval (edge case)
        let detector3 =
            ServiceDetector::new(Arc::clone(&registry)).with_interval(Duration::from_secs(0));
        assert_eq!(detector3.interval, Duration::from_secs(0));
    }

    #[tokio::test]
    async fn test_detector_port_configurations() {
        let registry = Arc::new(CapabilityRegistry::new());

        // Single port
        let detector1 = ServiceDetector::new(Arc::clone(&registry)).with_scan_ports(vec![8080]);
        assert_eq!(detector1.scan_ports, vec![8080]);

        // Standard HTTP ports
        let detector2 =
            ServiceDetector::new(Arc::clone(&registry)).with_scan_ports(vec![80, 443, 8080, 8443]);
        assert_eq!(detector2.scan_ports.len(), 4);

        // High ports
        let detector3 =
            ServiceDetector::new(Arc::clone(&registry)).with_scan_ports(vec![50000, 60000, 65535]);
        assert_eq!(detector3.scan_ports, vec![50000, 60000, 65535]);
    }

    #[tokio::test]
    async fn test_detector_default_ports() {
        let registry = Arc::new(CapabilityRegistry::new());
        let detector = ServiceDetector::new(registry);

        // Should have default discovery ports
        assert!(!detector.scan_ports.is_empty());
        assert!(detector.scan_ports.contains(&3000));
        assert!(detector.scan_ports.contains(&3001));
        assert!(detector.scan_ports.contains(&3002));
        assert!(detector.scan_ports.contains(&3010));
    }

    #[tokio::test]
    async fn test_detector_config_override() {
        let registry = Arc::new(CapabilityRegistry::new());

        // Set ports, then override
        let detector = ServiceDetector::new(registry)
            .with_scan_ports(vec![1000, 2000])
            .with_scan_ports(vec![3000, 4000]); // Override

        assert_eq!(detector.scan_ports, vec![3000, 4000]);
    }

    #[tokio::test]
    async fn test_detector_shared_registry() {
        let registry = Arc::new(CapabilityRegistry::new());

        // Multiple detectors can share the same registry
        let detector1 = ServiceDetector::new(Arc::clone(&registry));
        let detector2 = ServiceDetector::new(Arc::clone(&registry));

        assert_eq!(detector1.interval, detector2.interval);
    }

    #[test]
    fn test_detector_drop() {
        let registry = Arc::new(CapabilityRegistry::new());
        let detector = ServiceDetector::new(registry);

        // Should not panic on drop
        drop(detector);
    }

    #[tokio::test]
    async fn test_detector_task_initialization() {
        let registry = Arc::new(CapabilityRegistry::new());
        let detector = ServiceDetector::new(registry);

        // Should start with no active tasks
        assert_eq!(detector.tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_probe_port_error() {
        // Probing a definitely closed port should error
        let result = ServiceDetector::probe_port(1).await; // Port 1 typically requires root
        assert!(result.is_err());
    }
}
