use std::env;
use std::sync::Arc;

/// Configuration for safe system operations, capturing environment variables
/// that provide hints about the system's user/container context.
///
/// This config is used as a fallback mechanism when direct system calls
/// (like /proc/self/status or `id` command) are not available.
#[derive(Debug, Clone)]
/// Configuration for SafeSystem
pub struct SafeSystemConfig {
    uid_hint: Option<u32>,
    user_hint: Option<String>,
    gid_hint: Option<u32>,
    hostname_hint: Option<String>,
    container_hint: bool,
    docker_container_hint: bool,
}

/// Type alias for Sharedsafesystemconfig
pub type SharedSafeSystemConfig = Arc<SafeSystemConfig>;

impl SafeSystemConfig {
    /// Creates a new `SafeSystemConfig` with no hints (all None).
    pub fn new() -> Self {
        Self {
            uid_hint: None,
            user_hint: None,
            gid_hint: None,
            hostname_hint: None,
            container_hint: false,
            docker_container_hint: false,
        }
    }

    /// Creates a new `SafeSystemConfig` by reading environment variables.
    /// These are used as FALLBACK hints when direct system detection fails.
    pub fn from_env() -> Self {
        let uid_hint = env::var("UID")
            .ok()
            .and_then(|s| s.parse::<u32>().ok());
        
        let user_hint = env::var("USER").ok();
        
        let gid_hint = env::var("GID")
            .ok()
            .and_then(|s| s.parse::<u32>().ok());
        
        let hostname_hint = env::var("HOSTNAME").ok();
        
        let container_hint = env::var("container").is_ok();
        let docker_container_hint = env::var("DOCKER_CONTAINER").is_ok();

        Self {
            uid_hint,
            user_hint,
            gid_hint,
            hostname_hint,
            container_hint,
            docker_container_hint,
        }
    }

    // Builder methods for testing

    /// Builder method to set Uid Hint
    pub fn with_uid_hint(mut self, uid: u32) -> Self {
        self.uid_hint = Some(uid);
        self
    }

    /// Builder method to set User Hint
    pub fn with_user_hint(mut self, user: String) -> Self {
        self.user_hint = Some(user);
        self
    }

    /// Builder method to set Gid Hint
    pub fn with_gid_hint(mut self, gid: u32) -> Self {
        self.gid_hint = Some(gid);
        self
    }

    /// Builder method to set Hostname Hint
    pub fn with_hostname_hint(mut self, hostname: String) -> Self {
        self.hostname_hint = Some(hostname);
        self
    }

    /// Builder method to set Container Hint
    pub fn with_container_hint(mut self, is_container: bool) -> Self {
        self.container_hint = is_container;
        self
    }

    /// Builder method to set Docker Container Hint
    pub fn with_docker_container_hint(mut self, is_docker: bool) -> Self {
        self.docker_container_hint = is_docker;
        self
    }

    // Getter methods

    /// Get UID hint from environment (fallback when /proc unavailable)
    pub fn uid_hint(&self) -> Option<u32> {
        self.uid_hint
    }

    /// Get user name hint from environment (fallback)
    pub fn user_hint(&self) -> Option<&str> {
        self.user_hint.as_deref()
    }

    /// Get GID hint from environment (fallback)
    pub fn gid_hint(&self) -> Option<u32> {
        self.gid_hint
    }

    /// Get hostname hint from environment (fallback)
    pub fn hostname_hint(&self) -> Option<&str> {
        self.hostname_hint.as_deref()
    }

    /// Check if container environment variable is present
    pub fn has_container_hint(&self) -> bool {
        self.container_hint || self.docker_container_hint
    }

    /// Check if running as root based on USER hint
    pub fn is_root_user_hint(&self) -> bool {
        self.user_hint.as_deref() == Some("root")
    }
}

impl Default for SafeSystemConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SafeSystemConfig::new();
        assert!(config.uid_hint().is_none());
        assert!(config.user_hint().is_none());
        assert!(config.gid_hint().is_none());
        assert!(config.hostname_hint().is_none());
        assert!(!config.has_container_hint());
        assert!(!config.is_root_user_hint());
    }

    #[test]
    fn test_builder_pattern() {
        let config = SafeSystemConfig::new()
            .with_uid_hint(1000)
            .with_user_hint("testuser".to_string())
            .with_gid_hint(1000)
            .with_hostname_hint("testhost".to_string())
            .with_container_hint(true);

        assert_eq!(config.uid_hint(), Some(1000));
        assert_eq!(config.user_hint(), Some("testuser"));
        assert_eq!(config.gid_hint(), Some(1000));
        assert_eq!(config.hostname_hint(), Some("testhost"));
        assert!(config.has_container_hint());
    }

    #[test]
    fn test_root_user_detection() {
        let config = SafeSystemConfig::new()
            .with_user_hint("root".to_string());

        assert!(config.is_root_user_hint());
    }

    #[test]
    fn test_container_hints() {
        let config1 = SafeSystemConfig::new()
            .with_container_hint(true);
        assert!(config1.has_container_hint());

        let config2 = SafeSystemConfig::new()
            .with_docker_container_hint(true);
        assert!(config2.has_container_hint());

        let config3 = SafeSystemConfig::new()
            .with_container_hint(true)
            .with_docker_container_hint(true);
        assert!(config3.has_container_hint());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_access() {
        let config = Arc::new(SafeSystemConfig::new()
            .with_uid_hint(1000)
            .with_user_hint("testuser".to_string())
            .with_gid_hint(1000));

        let handles: Vec<_> = (0..100)
            .map(|_| {
                let cfg = config.clone();
                tokio::spawn(async move {
                    let _ = cfg.uid_hint();
                    let _ = cfg.user_hint();
                    let _ = cfg.gid_hint();
                    let _ = cfg.hostname_hint();
                    let _ = cfg.has_container_hint();
                    let _ = cfg.is_root_user_hint();
                })
            })
            .collect();

        for handle in handles {
            handle.await.expect("Task should complete successfully");
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_different_configs() {
        let config1 = Arc::new(
            SafeSystemConfig::new()
                .with_uid_hint(0)
                .with_user_hint("root".to_string()),
        );
        let config2 = Arc::new(
            SafeSystemConfig::new()
                .with_uid_hint(1000)
                .with_user_hint("user".to_string()),
        );

        let handle1 = tokio::spawn({
            let cfg = config1.clone();
            async move { (cfg.uid_hint(), cfg.is_root_user_hint()) }
        });
        let handle2 = tokio::spawn({
            let cfg = config2.clone();
            async move { (cfg.uid_hint(), cfg.is_root_user_hint()) }
        });

        let (uid1, is_root1) = handle1.await.unwrap();
        let (uid2, is_root2) = handle2.await.unwrap();

        assert_eq!(uid1, Some(0));
        assert!(is_root1);
        assert_eq!(uid2, Some(1000));
        assert!(!is_root2);
    }
}

