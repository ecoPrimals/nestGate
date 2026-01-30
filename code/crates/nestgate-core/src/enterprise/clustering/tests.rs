//! Tests for clustering module

use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_cluster_manager_creation() -> Result<()> {
        let config = ClusterConfig::default();
        let manager = ClusterManager::new(config).await?;
        
        let status = manager.get_status().await?;
        assert_eq!(status.total_nodes, 1);
        assert_eq!(status.active_nodes, 1);
        assert!(status.leader_id.is_none()); // No leader initially
        
        println!("✅ Cluster manager creation test passed");
        Ok(())
    }
    
    #[tokio::test]
    async fn test_cluster_lifecycle() -> Result<()> {
        let config = ClusterConfig::default();
        let manager = ClusterManager::new(config).await?;
        
        // Test start
        manager.start().await?;
        
        // Modern pattern: Poll for leader election completion
        // Leader election should be deterministic, not timing-based
        let mut attempts = 0;
        let status = loop {
            let status = manager.get_status().await?;
            if status.local_node_role == NodeRole::Leader || attempts >= 50 {
                break status;
            }
            attempts += 1;
            tokio::task::yield_now().await; // Allow other tasks to run
        };
        
        assert_eq!(status.local_node_role, NodeRole::Leader); // Should become leader
        assert!(status.leader_id.is_some());
        
        // Test stop
        manager.stop().await?;
        
        println!("✅ Cluster lifecycle test passed");
        Ok(())
    }
