//! Clustering module for high-availability NestGate deployments
//!
//! This module provides clustering capabilities including leader election,
//! node discovery, heartbeat management, and automatic failover.

use crate::{Result, error::NestGateError};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::{RwLock, broadcast, mpsc};
use tokio::time::{interval, sleep};
use uuid::Uuid;

// ==================== MODULE DECLARATIONS ====================

pub mod config;
pub mod enums;
pub mod types;
pub mod events;
pub mod components;

#[cfg(test)]
mod tests;

// ==================== RE-EXPORTS ====================

// Config re-exports
pub use config::{ClusterConfig, ClusterNodeConfig};

// Enum re-exports
pub use enums::{
    NodeStatus, NodeRole, NodeCapability,
    ClusterHealthStatus, ConsistencyStatus, ElectionState,
};

// Type re-exports
pub use types::{
    ClusterNode, NodeMetadata, NodeResources,
    ClusterState, ClusterHealth, PartitionInfo, Partition,
    DiscoveredNode, HeartbeatInfo, ClusterStatus,
};

// Event re-exports
pub use events::ClusterEvent;

// Component re-exports (for internal use)
use components::{LeaderElection, NodeDiscovery, HeartbeatManager};

// ==================== CLUSTER MANAGER ====================

/// Cluster manager for coordinating multiple NestGate instances
pub struct ClusterManager {
    config: Arc<ClusterConfig>,
    local_node: Arc<RwLock<ClusterNode>>,
    cluster_state: Arc<RwLock<ClusterState>>,
    leader_election: Arc<RwLock<LeaderElection>>,
    node_discovery: Arc<RwLock<NodeDiscovery>>,
    heartbeat_manager: Arc<RwLock<HeartbeatManager>>,
    event_sender: broadcast::Sender<ClusterEvent>,
    shutdown_sender: mpsc::Sender<()>,
    shutdown_receiver: Arc<RwLock<Option<mpsc::Receiver<()>>>>,
}

impl ClusterManager {
    /// Create new cluster manager
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn new(config: ClusterConfig) -> Result<Self>  {
        let cluster_id = Uuid::new_v4().to_string();
        let node_id = config.node_id.clone();
        
        // Create local node
        let local_node = ClusterNode {
            node_id: node_id.clone(),
            endpoint: config.bind_endpoint,
            status: NodeStatus::Starting,
            role: NodeRole::Follower,
            last_heartbeat: SystemTime::now(),
            metadata: NodeMetadata {
                version: env!("CARGO_PKG_VERSION").to_string(),
                started_at: SystemTime::now(),
                region: None,
                zone: None,
                weight: 100,
                tags: HashMap::new(),
                resources: NodeResources {
                    cpu_cores: num_cpus::get() as u32,
                    memory_gb: 8, // Would detect actual memory
                    storage_gb: 1000, // Would detect actual storage
                    network_bandwidth_mbps: 1000,
                    load_average: 0.0,
                    memory_usage_percent: 0.0,
                    storage_usage_percent: 0.0,
                },
            },
            capabilities: vec![
                NodeCapability::Storage,
                NodeCapability::Compute,
                NodeCapability::Gateway,
            ],
        };
        
        // Initialize cluster state
        let mut nodes = HashMap::new();
        nodes.insert(node_id.clone(), local_node.clone());
        
        let cluster_state = ClusterState {
            cluster_id: cluster_id.clone(),
            nodes,
            leader_id: None,
            election_term: 0,
            cluster_health: ClusterHealth {
                overall_status: ClusterHealthStatus::Healthy,
                active_nodes: 1,
                failed_nodes: 0,
                degraded_nodes: 0,
                quorum_available: true,
                leader_available: false,
                data_consistency: ConsistencyStatus::Consistent,
            },
            partition_info: PartitionInfo {
                partitions: vec![],
                majority_partition: None,
                split_brain_detected: false,
            },
            last_updated: SystemTime::now(),
        };
        
        // Initialize components
        let leader_election = LeaderElection {
            current_term: 0,
            voted_for: None,
            election_timeout: Duration::from_millis(config.election_timeout_ms),
            last_election: None,
            votes_received: HashSet::new(),
            election_state: ElectionState::Follower,
        };
        
        let node_discovery = NodeDiscovery {
            discovery_enabled: config.discovery_enabled,
            multicast_endpoint: config.discovery_multicast_endpoint.clone(),
            discovery_port: config.discovery_port,
            discovered_nodes: HashMap::new(),
            last_discovery: SystemTime::now(),
        };
        
        let heartbeat_manager = HeartbeatManager {
            heartbeat_interval: Duration::from_millis(config.heartbeat_interval_ms),
            max_missed_heartbeats: config.max_missed_heartbeats,
            node_heartbeats: HashMap::new(),
            last_heartbeat_sent: None,
        };
        
        let (event_sender, _) = broadcast::channel(1000);
        let (shutdown_sender, shutdown_receiver) = mpsc::channel(1);
        
        Ok(Self {
            config: Arc::new(config),
            local_node: Arc::new(RwLock::new(local_node)),
            cluster_state: Arc::new(RwLock::new(cluster_state)),
            leader_election: Arc::new(RwLock::new(leader_election)),
            node_discovery: Arc::new(RwLock::new(node_discovery)),
            heartbeat_manager: Arc::new(RwLock::new(heartbeat_manager)),
            event_sender,
            shutdown_sender,
            shutdown_receiver: Arc::new(RwLock::new(Some(shutdown_receiver))),
        })
    }
    
    /// Start cluster services
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn start(&self) -> Result<()>  {
        println!("🔗 Starting cluster manager for cluster '{self.config.cluster_name}'...");
        
        // Update local node status
        {
            let mut local_node = self.local_node.write().await;
            local_node.status = NodeStatus::Active;
        }
        
        // Start background tasks
        self.start_heartbeat_task().await?;
        self.start_leader_election_task().await?;
        
        if self.config.discovery_enabled {
            self.start_node_discovery_task().await?;
        }
        
        self.start_health_monitoring_task().await?;
        
        // Send cluster joined event
        let _ = self.event_sender.send(ClusterEvent::NodeJoined(
            self.config.node_id.clone()
        ));
        
        println!("✅ Cluster manager started successfully");
        Ok(())
    }
    
    /// Stop cluster services
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn stop(&self) -> Result<()>  {
        println!("🛑 Stopping cluster manager...");
        
        // Update local node status
        {
            let mut local_node = self.local_node.write().await;
            local_node.status = NodeStatus::Leaving;
        }
        
        // Send shutdown signal
        let _ = self.shutdown_sender.send(()).await;
        
        // Send cluster left event
        let _ = self.event_sender.send(ClusterEvent::NodeLeft(
            self.config.node_id.clone()
        ));
        
        println!("✅ Cluster manager stopped");
        Ok(())
    }
    
    /// Get current cluster status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_status(&self) -> Result<ClusterStatus>  {
        let cluster_state = self.cluster_state.read().await;
        let local_node = self.local_node.read().await;
        
        Ok(ClusterStatus {
            cluster_name: self.config.cluster_name.clone(),
            cluster_id: cluster_state.cluster_id.clone(),
            total_nodes: cluster_state.nodes.len() as u32,
            active_nodes: cluster_state.cluster_health.active_nodes,
            leader_id: cluster_state.leader_id.clone(),
            local_node_id: local_node.node_id.clone(),
            local_node_role: local_node.role.clone(),
            cluster_health: cluster_state.cluster_health.overall_status.clone(),
            quorum_available: cluster_state.cluster_health.quorum_available,
            last_updated: cluster_state.last_updated,
        })
    }
    
    /// Subscribe to cluster events
    pub fn subscribe_events(&self) -> broadcast::Receiver<ClusterEvent> {
        self.event_sender.subscribe()
    }
    
    /// Start heartbeat background task
    async fn start_heartbeat_task(&self) -> Result<()> {
        let heartbeat_manager = self.heartbeat_manager.clone();
        let cluster_state = self.cluster_state.clone();
        let event_sender = self.event_sender.clone();
        let heartbeat_interval = self.config.heartbeat_interval_ms;
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(heartbeat_interval));
            
            loop {
                interval.tick().await;
                
                // Send heartbeats to all nodes
                let nodes: Vec<String> = {
                    let state = cluster_state.read().await;
                    state.nodes.keys().cloned().collect()
                };
                
                for node_id in nodes {
                    // In a real implementation, this would send actual network heartbeats
                    // For now, we'll simulate heartbeat processing
                    let mut manager = heartbeat_manager.write().await;
                    manager.node_heartbeats.entry(node_id.clone())
                        .and_modify(|info| {
                            info.last_received = SystemTime::now();
                            info.missed_count = 0;
                        })
                        .or_insert(HeartbeatInfo {
                            last_received: SystemTime::now(),
                            missed_count: 0,
                            rtt_ms: 10, // Simulated RTT
                        );
                }
                
                // Update last heartbeat sent time
                {
                    let mut manager = heartbeat_manager.write().await;
                    manager.last_heartbeat_sent = Some(SystemTime::now());
                }
            }
        );
        
        Ok(())
    }
    
    /// Start leader election background task
    async fn start_leader_election_task(&self) -> Result<()> {
        let leader_election = self.leader_election.clone();
        let local_node = self.local_node.clone();
        let cluster_state = self.cluster_state.clone();
        let event_sender = self.event_sender.clone();
        let election_timeout = self.config.election_timeout_ms;
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(election_timeout / 2));
            
            loop {
                interval.tick().await;
                
                let should_start_election = {
                    let election = leader_election.read().await;
                    let state = cluster_state.read().await;
                    
                    // Start election if no leader and we're not already a candidate/leader
                    state.leader_id.is_none() && 
                    election.election_state == ElectionState::Follower
                };
                
                if should_start_election {
                    // Start leader election
                    {
                        let mut election = leader_election.write().await;
                        let mut local = local_node.write().await;
                        
                        election.current_term += 1;
                        election.voted_for = Some(local.node_id.clone());
                        election.election_state = ElectionState::Candidate;
                        election.votes_received.clear();
                        election.votes_received.insert(local.node_id.clone());
                        election.last_election = Some(SystemTime::now());
                        
                        local.role = NodeRole::Candidate;
                    }
                    
                    // In a real implementation, this would send vote requests to other nodes
                    // For now, we'll simulate winning the election if we're the only node
                    let node_count = cluster_state.read().await.nodes.len();
                    
                    if node_count == 1 {
                        // Become leader
                        {
                            let mut election = leader_election.write().await;
                            let mut local = local_node.write().await;
                            let mut state = cluster_state.write().await;
                            
                            election.election_state = ElectionState::Leader;
                            local.role = NodeRole::Leader;
                            state.leader_id = Some(local.node_id.clone());
                            state.last_updated = SystemTime::now();
                        }
                        
                        // Send leader elected event
                        let local_id = local_node.read().await.node_id.clone();
                        let _ = event_sender.send(ClusterEvent::LeaderElected(local_id));
                    }
                }
            }
        );
        
        Ok(())
    }
    
    /// Start node discovery background task
    async fn start_node_discovery_task(&self) -> Result<()> {
        let node_discovery = self.node_discovery.clone();
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                // In a real implementation, this would perform multicast discovery
                // For now, we'll simulate discovery
                {
                    let mut discovery = node_discovery.write().await;
                    discovery.last_discovery = SystemTime::now();
                }
            }
        );
        
        Ok(())
    }
    
    /// Start health monitoring background task
    async fn start_health_monitoring_task(&self) -> Result<()> {
        let cluster_state = self.cluster_state.clone();
        let heartbeat_manager = self.heartbeat_manager.clone();
        let event_sender = self.event_sender.clone();
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));
            
            loop {
                interval.tick().await;
                
                // Update cluster health
                let mut health_changed = false;
                
                {
                    let mut state = cluster_state.write().await;
                    let manager = heartbeat_manager.read().await;
                    
                    let mut active_nodes = 0;
                    let mut failed_nodes = 0;
                    let mut degraded_nodes = 0;
                    
                    for (node_id, node) in state.nodes.iter_mut() {
                        match node.status {
                            NodeStatus::Active => active_nodes += 1,
                            NodeStatus::Failed => failed_nodes += 1,
                            NodeStatus::Degraded => degraded_nodes += 1,
                            _ => {}
                        }
                        
                        // Check for failed nodes based on missed heartbeats
                        if let Some(heartbeat) = manager.node_heartbeats.get(node_id) {
                            if heartbeat.missed_count > manager.max_missed_heartbeats {
                                if node.status != NodeStatus::Failed {
                                    node.status = NodeStatus::Failed;
                                    health_changed = true;
                                }
                            }
                        }
                    }
                    
                    let old_status = state.cluster_health.overall_status.clone();
                    
                    state.cluster_health.active_nodes = active_nodes;
                    state.cluster_health.failed_nodes = failed_nodes;
                    state.cluster_health.degraded_nodes = degraded_nodes;
                    
                    // Determine overall cluster health
                    state.cluster_health.overall_status = if failed_nodes > active_nodes {
                        ClusterHealthStatus::Failed
                    } else if failed_nodes > 0 || degraded_nodes > active_nodes / 2 {
                        ClusterHealthStatus::Degraded
                    } else {
                        ClusterHealthStatus::Healthy
                    };
                    
                    state.cluster_health.quorum_available = active_nodes > (state.nodes.len() as u32) / 2;
                    state.cluster_health.leader_available = state.leader_id.is_some();
                    
                    if old_status != state.cluster_health.overall_status {
                        health_changed = true;
                    }
                    
                    state.last_updated = SystemTime::now();
                }
                
                if health_changed {
                    let new_status = cluster_state.read().await.cluster_health.overall_status.clone();
                    let _ = event_sender.send(ClusterEvent::ClusterHealthChanged(new_status));
                }
            }
        );
        
        Ok(())
    }
}

