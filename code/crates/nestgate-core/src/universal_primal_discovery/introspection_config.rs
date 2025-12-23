use std::sync::Arc;

/// Thread-safe configuration for system introspection
/// Captures environment variables at initialization to prevent race conditions
#[derive(Debug, Clone)]
/// Configuration for Introspection
pub struct IntrospectionConfig {
    // Container/orchestration detection
    kubernetes_namespace: Option<String>,
    docker_compose_project: Option<String>,
    compute_capability_type: Option<String>,

    // Resource limits
    max_file_handles: Option<usize>,
}

/// Shared immutable reference to IntrospectionConfig
pub type SharedIntrospectionConfig = Arc<IntrospectionConfig>;

impl IntrospectionConfig {
    /// Create a new empty configuration (all values None, will use system detection)
    pub fn new() -> Self {
        Self {
            kubernetes_namespace: None,
            docker_compose_project: None,
            compute_capability_type: None,
            max_file_handles: None,
        }
    }

    /// Create configuration from current environment variables
    /// This captures env vars at initialization time, making it thread-safe
    pub fn from_env() -> Self {
        Self {
            kubernetes_namespace: std::env::var("KUBERNETES_NAMESPACE").ok(),
            docker_compose_project: std::env::var("DOCKER_COMPOSE_PROJECT").ok(),
            compute_capability_type: std::env::var("COMPUTE_CAPABILITY_TYPE").ok(),
            max_file_handles: std::env::var("NESTGATE_MAX_FILE_HANDLES")
                .ok()
                .and_then(|s| s.parse().ok()),
        }
    }

    // Accessors

    /// Check if running in Kubernetes
    pub fn is_kubernetes(&self) -> bool {
        self.kubernetes_namespace.is_some()
    }

    /// Check if running in Docker Compose
    pub fn is_docker_compose(&self) -> bool {
        self.docker_compose_project.is_some()
    }

    /// Get compute capability type (modern detection)
    pub fn get_compute_capability_type(&self) -> Option<&str> {
        self.compute_capability_type.as_deref()
    }

    /// Get max file handles if configured
    pub fn get_max_file_handles(&self) -> Option<usize> {
        self.max_file_handles
    }

    /// Estimate memory based on environment
    /// Returns estimated memory in GB
    pub fn estimate_memory_gb(&self) -> f64 {
        if self.is_kubernetes() {
            2.0 // Assume 2GB in containerized environment
        } else {
            8.0 // Assume 8GB for desktop/server environment
        }
    }

    /// Detect container runtime using capability-based approach
    pub fn detect_container_runtime(&self) -> Option<String> {
        // Modern capability-based detection (preferred)
        if let Some(compute_type) = &self.compute_capability_type {
            return Some(format!("capability-based-{compute_type}"));
        }

        // DEPRECATED: Legacy vendor-specific detection patterns
        // These will be removed in version 4.0.0

        // DEPRECATED: Kubernetes namespace detection
        if self.is_kubernetes() {
            return Some("legacy-orchestrated".to_string());
        }

        // DEPRECATED: Docker Compose detection
        if self.is_docker_compose() {
            return Some("legacy-containerized".to_string());
        }

        None // Native/bare-metal execution
    }

    // Builder methods for tests

    /// Builder method to set Kubernetes Namespace
    pub fn with_kubernetes_namespace(mut self, namespace: String) -> Self {
        self.kubernetes_namespace = Some(namespace);
        self
    }

    /// Builder method to set Docker Compose Project
    pub fn with_docker_compose_project(mut self, project: String) -> Self {
        self.docker_compose_project = Some(project);
        self
    }

    /// Builder method to set Compute Capability Type
    pub fn with_compute_capability_type(mut self, compute_type: String) -> Self {
        self.compute_capability_type = Some(compute_type);
        self
    }

    /// Builder method to set Max File Handles
    pub fn with_max_file_handles(mut self, limit: usize) -> Self {
        self.max_file_handles = Some(limit);
        self
    }
}

impl Default for IntrospectionConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_introspection_config_new() {
        let config = IntrospectionConfig::new();

        // Should use defaults
        assert!(!config.is_kubernetes());
        assert!(!config.is_docker_compose());
        assert!(config.get_compute_capability_type().is_none());
        assert!(config.get_max_file_handles().is_none());
        assert_eq!(config.estimate_memory_gb(), 8.0); // Default to non-k8s
    }

    #[test]
    fn test_introspection_config_kubernetes() {
        let config = IntrospectionConfig::new().with_kubernetes_namespace("default".to_string());

        assert!(config.is_kubernetes());
        assert_eq!(config.estimate_memory_gb(), 2.0); // K8s environment
        assert_eq!(
            config.detect_container_runtime(),
            Some("legacy-orchestrated".to_string())
        );
    }

    #[test]
    fn test_introspection_config_docker_compose() {
        let config = IntrospectionConfig::new().with_docker_compose_project("myapp".to_string());

        assert!(config.is_docker_compose());
        assert_eq!(
            config.detect_container_runtime(),
            Some("legacy-containerized".to_string())
        );
    }

    #[test]
    fn test_introspection_config_modern_detection() {
        let config =
            IntrospectionConfig::new().with_compute_capability_type("orchestrated".to_string());

        assert_eq!(
            config.detect_container_runtime(),
            Some("capability-based-orchestrated".to_string())
        );
    }

    #[test]
    fn test_introspection_config_file_handles() {
        let config = IntrospectionConfig::new().with_max_file_handles(4096);

        assert_eq!(config.get_max_file_handles(), Some(4096));
    }

    #[test]
    fn test_introspection_config_priority() {
        // Modern detection should take priority over legacy
        let config = IntrospectionConfig::new()
            .with_compute_capability_type("modern".to_string())
            .with_kubernetes_namespace("default".to_string())
            .with_docker_compose_project("myapp".to_string());

        // Modern should win
        assert_eq!(
            config.detect_container_runtime(),
            Some("capability-based-modern".to_string())
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_introspection_config_access() {
        // Create two different configurations
        let config1 = Arc::new(
            IntrospectionConfig::new()
                .with_kubernetes_namespace("namespace1".to_string())
                .with_max_file_handles(2048),
        );
        let config2 = Arc::new(
            IntrospectionConfig::new()
                .with_docker_compose_project("project2".to_string())
                .with_max_file_handles(4096),
        );

        // Spawn concurrent tasks accessing different configs
        let handle1 = {
            let config = Arc::clone(&config1);
            tokio::spawn(async move {
                for _ in 0..100 {
                    assert!(config.is_kubernetes());
                    assert_eq!(config.get_max_file_handles(), Some(2048));
                }
            })
        };

        let handle2 = {
            let config = Arc::clone(&config2);
            tokio::spawn(async move {
                for _ in 0..100 {
                    assert!(config.is_docker_compose());
                    assert_eq!(config.get_max_file_handles(), Some(4096));
                }
            })
        };

        handle1.await.unwrap();
        handle2.await.unwrap();
    }

    #[test]
    fn test_memory_estimation() {
        let k8s_config = IntrospectionConfig::new().with_kubernetes_namespace("prod".to_string());
        assert_eq!(k8s_config.estimate_memory_gb(), 2.0);

        let native_config = IntrospectionConfig::new();
        assert_eq!(native_config.estimate_memory_gb(), 8.0);
    }

    #[test]
    fn test_bare_metal_detection() {
        let config = IntrospectionConfig::new();

        // No container runtime detected
        assert!(config.detect_container_runtime().is_none());
        assert!(!config.is_kubernetes());
        assert!(!config.is_docker_compose());
    }
}
