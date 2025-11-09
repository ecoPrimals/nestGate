// **ZFS COMMAND EXECUTION**
///
// Command execution and caching for ZFS operations

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tokio::process::Command;
use serde::{Deserialize, Serialize};

use nestgate_core::error::Result;
use nestgate_core::constants::zfs;
use nestgate_core::config::canonical_primary::ZfsOperationsConfig;

// ==================== COMMAND EXECUTION ====================

/// **COMMAND EXECUTOR**
///
/// Executes ZFS commands with caching and error handling
pub struct CommandExecutor {
    /// Configuration
    config: ZfsOperationsConfig,
    /// Command cache
    cache: Arc<RwLock<CommandCache>>,
}

/// **COMMAND CACHE**
///
/// Caches command results to improve performance
pub struct CommandCache {
    /// Cached commands
    commands: HashMap<String, CachedCommand>,
    /// Cache statistics
    stats: CacheStats,
}

/// **CACHED COMMAND**
///
/// A cached command result with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedCommand {
    /// Command that was executed
    pub command: String,
    /// Command output
    pub output: String,
    /// Exit code
    pub exit_code: i32,
    /// Timestamp when cached
    pub cached_at: SystemTime,
    /// TTL for this command
    pub ttl: Duration,
}

/// **CACHE STATISTICS**
///
/// Statistics about cache performance
#[derive(Debug, Default)]
pub struct CacheStats {
    /// Total cache hits
    pub hits: u64,
    /// Total cache misses
    pub misses: u64,
    /// Total commands executed
    pub executions: u64,
}

impl CommandExecutor {
    /// Create a new command executor
    pub fn new(config: &ZfsOperationsConfig) -> impl std::future::Future<Output = Result<Self, NestGateUnifiedError>> + Send {
        Ok(Self {
            config: config.clone(),
            cache: Arc::new(RwLock::new(CommandCache::new())),
        })
    }

    /// Execute a ZFS command with caching
    pub fn execute(&self, command: &str, args: &[&str]) -> impl std::future::Future<Output = Result<String, NestGateUnifiedError>> + Send {
        let full_command = format!("{} {}", command, args.join(" "));
        
        // Check cache first
        if self.config.enable_caching {
                if let Some(cached) = self.get_cached(&full_command).await? {
                return Ok(cached.output);
            }
        }

        // Execute the command
            let output = self.execute_command(command, args).await?;
        
        // Cache the result
        if self.config.enable_caching {
                self.cache_command(&full_command, &output, 0).await?;
        }

        Ok(output)
    }

    /// Execute a ZFS command without caching
    fn execute_command(&self, command: &str, args: &[&str]) -> impl std::future::Future<Output = Result<String, NestGateUnifiedError>> + Send {
        let mut cmd = if self.config.use_sudo {
            let mut c = Command::new("sudo");
            c.arg(command);
            c.args(args);
            c
        } else {
            let mut c = Command::new(command);
            c.args(args);
            c
        };

        let output = tokio::time::timeout(self.config.command_timeout, cmd.output())
                .await
            .map_err(|_| nestgate_core::error::NestGateUnifiedError::System(Box::new(
                nestgate_core::error::SystemErrorDetails {
                    operation: "zfs_command_execution".to_string(),
                    message: format!("Command timeout after {:?}", self.config.command_timeout),
                    context: format!("{} {}", command, args.join(" ")),
                    system_info: std::env::consts::OS.to_string(),
                }
            )))?
            .map_err(|e| nestgate_core::error::NestGateUnifiedError::System(Box::new(
                nestgate_core::error::SystemErrorDetails {
                    operation: "zfs_command_execution".to_string(),
                    message: format!("Command execution failed: {}", e),
                    context: format!("{} {}", command, args.join(" ")),
                    system_info: std::env::consts::OS.to_string(),
                }
            )))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(nestgate_core::error::NestGateUnifiedError::System(Box::new(
                nestgate_core::error::SystemErrorDetails {
                    operation: "zfs_command_execution".to_string(),
                    message: format!("Command failed with exit code {:?}", output.status.code()),
                    context: stderr.to_string(),
                    system_info: format!("{} {}", command, args.join(" ")),
                }
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Get cached command result
    fn get_cached(&self, command: &str) -> impl std::future::Future<Output = Result<Option<CachedCommand>> + Send> {
            let cache = self.cache.read();
        
        if let Some(cached) = cache.commands.get(command) {
            // Check if cache entry is still valid
            if cached.cached_at.elapsed().unwrap_or(Duration::MAX) < cached.ttl {
                // Update cache stats
                drop(cache);
                    let mut cache_write = self.cache.write();
                cache_write.stats.hits += 1;
                return Ok(Some(cached.clone()));
            }
        }
        
        // Update cache stats
        drop(cache);
            let mut cache_write = self.cache.write().await;
        cache_write.stats.misses += 1;
        Ok(None)
    }

    /// Cache command result
    fn cache_command(&self, command: &str, output: &str, exit_code: i32) -> impl std::future::Future<Output = Result<(), NestGateUnifiedError>> + Send {
            let mut cache = self.cache.write();
        
        let cached_command = CachedCommand {
            command: command.to_string(),
            output: output.to_string(),
            exit_code,
            cached_at: SystemTime::now(),
            ttl: self.config.cache_ttl,
        };
        
        cache.commands.insert(command.to_string(), cached_command);
        cache.stats.executions += 1;
        
        // Cleanup old entries if cache is too large
        if cache.commands.len() > zfs::COMMAND_CACHE_SIZE {
                self.cleanup_cache(&mut cache).await;
        }
        
        Ok(())
    }

    /// Cleanup old cache entries
    async fn cleanup_cache(&self, cache: &mut CommandCache) {
        let now = SystemTime::now();
        cache.commands.retain(|_, cached| {
            now.duration_since(cached.cached_at).unwrap_or(Duration::MAX) < cached.ttl
        });
    }

    /// Clear all cached commands
    pub fn clear_cache(&self) -> impl std::future::Future<Output = Result<(), NestGateUnifiedError>> + Send {
            let mut cache = self.cache.write().await;
        cache.commands.clear();
        Ok(())
    }

    /// Get cache statistics
    pub async fn cache_stats(&self) -> CacheStats {
            let cache = self.cache.read().await;
        CacheStats {
            hits: cache.stats.hits,
            misses: cache.stats.misses,
            executions: cache.stats.executions,
        }
    }
}

impl CommandCache {
    /// Create a new command cache
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            stats: CacheStats::default(),
        }
    }
}

impl Default for CommandCache {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheStats {
    /// Calculate cache hit rate
    pub fn hit_rate(&self) -> f64 {
        if self.hits + self.misses == 0 {
            0.0
        } else {
            self.hits as f64 / (self.hits + self.misses) as f64
        }
    }
}

#[cfg(test)]
mod tests {
    

    #[tokio::test]
    async fn test_command_executor_creation() {
        let config = ZfsOperationsConfig::development();
        let executor = CommandExecutor::new(&config).await;
        assert!(executor.is_ok());
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let stats = CacheStats::default();
        assert_eq!(stats.hit_rate(), 0.0);
        
        let stats = CacheStats {
            hits: 8,
            misses: 2,
            executions: 10,
        };
        assert_eq!(stats.hit_rate(), 0.8);
    }

    #[test]
    fn test_cached_command() {
        let cached = CachedCommand {
            command: "zfs list".to_string(),
            output: "tank\n".to_string(),
            exit_code: 0,
            cached_at: SystemTime::now(),
            ttl: Duration::from_secs(300),
        };
        
        assert_eq!(cached.command, "zfs list");
        assert_eq!(cached.exit_code, 0);
    }
} 