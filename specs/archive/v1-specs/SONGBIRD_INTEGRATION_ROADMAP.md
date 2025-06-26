---
title: Songbird Integration Implementation Roadmap
description: Technical roadmap for integrating NestGate with Songbird orchestrator
version: 1.0.0
date: 2025-01-26
author: NestGate Team
priority: CRITICAL
status: ACTIVE
---

# 🛣️ Songbird Integration Implementation Roadmap

## Overview

This roadmap guides the NestGate team through the technical implementation of Songbird orchestrator integration, eliminating code duplication and providing a production-ready foundation.

---

## 📅 **Week 1: Foundation & Identity Fix**

### **Day 1-2: Project Identity Crisis**
```bash
# 🔥 CRITICAL: Fix Cargo.toml immediately
git checkout -b songbird-integration

# Edit nestgate/Cargo.toml
[package]
name = "nestgate"  # NOT "songbird-orchestrator"
version = "2.0.0"
description = "Sovereign NAS system with ZFS integration"

[dependencies]
songbird-orchestrator = { path = "../songbird" }
```

### **Day 3-4: Code Audit**
```bash
# Identify files to remove (duplicating Songbird)
find src/ -name "*orchestrator*" -type f
find src/ -name "*registry*" -type f
find src/ -name "*proxy*" -type f
find src/ -name "*health_monitor*" -type f

# Create removal plan
echo "Files to remove:" > REMOVAL_PLAN.md
ls -la src/orchestrator/ >> REMOVAL_PLAN.md
ls -la src/service_registry.rs >> REMOVAL_PLAN.md
```

### **Day 5: Integration Scaffolding**
```rust
// Create src/songbird_integration.rs
use songbird_orchestrator::prelude::*;
use async_trait::async_trait;

pub mod services {
    pub use super::zfs::NestGateZfsService;
    pub use super::storage::NestGateStorageService;
    pub use super::tiers::NestGateTierService;
}

pub mod zfs;
pub mod storage;
pub mod tiers;
```

---

## 📅 **Week 2: Core Service Migration**

### **Day 1-3: ZFS Service Implementation**
```rust
// src/songbird_integration/zfs.rs
use songbird_orchestrator::prelude::*;
use async_trait::async_trait;

#[derive(Clone)]
pub struct NestGateZfsService {
    config: Option<ZfsServiceConfig>,
    zfs_manager: Arc<crate::zfs::ZfsManager>,
}

#[async_trait]
impl UniversalService for NestGateZfsService {
    type Config = ZfsServiceConfig;
    type Health = ZfsHealthStatus;
    type Error = crate::error::NestGateError;

    async fn initialize(&mut self, config: Self::Config) -> Result<(), Self::Error> {
        self.config = Some(config.clone());
        self.zfs_manager.initialize(&config).await?;
        Ok(())
    }

    async fn start(&mut self) -> Result<(), Self::Error> {
        self.zfs_manager.start().await?;
        Ok(())
    }

    async fn handle_request(&self, request: ServiceRequest) -> Result<ServiceResponse, Self::Error> {
        match request.path.as_str() {
            "/pools" => self.handle_pools(request).await,
            "/datasets" => self.handle_datasets(request).await,
            "/snapshots" => self.handle_snapshots(request).await,
            "/health" => {
                let health = self.health_check().await?;
                Ok(ServiceResponse::success(request.id, serde_json::to_value(health)?))
            }
            _ => Ok(ServiceResponse::error(request.id, 404, "Endpoint not found".to_string())),
        }
    }

    async fn health_check(&self) -> Result<Self::Health, Self::Error> {
        // Implement ZFS health check
        let pools = self.zfs_manager.list_pools().await?;
        let datasets = self.zfs_manager.list_datasets().await?;
        
        Ok(ZfsHealthStatus {
            pools_count: pools.len(),
            datasets_count: datasets.len(),
            total_space: self.zfs_manager.total_space().await?,
            used_space: self.zfs_manager.used_space().await?,
            status: "healthy".to_string(),
        })
    }

    fn service_info(&self) -> ServiceInfo {
        ServiceInfo {
            name: "nestgate-zfs".to_string(),
            version: "2.0.0".to_string(),
            description: "ZFS management service".to_string(),
            endpoints: vec![
                "/pools".to_string(),
                "/datasets".to_string(),
                "/snapshots".to_string(),
                "/health".to_string(),
            ],
        }
    }

    // ... implement remaining methods
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsServiceConfig {
    pub pools_path: String,
    pub enable_snapshots: bool,
    pub tier_management: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsHealthStatus {
    pub pools_count: usize,
    pub datasets_count: usize,
    pub total_space: u64,
    pub used_space: u64,
    pub status: String,
}
```

### **Day 4-5: Storage Protocols Service**
```rust
// src/songbird_integration/storage.rs
#[derive(Clone)]
pub struct NestGateStorageService {
    protocols: HashMap<String, Box<dyn StorageProtocol>>,
}

#[async_trait]
impl UniversalService for NestGateStorageService {
    type Config = StorageServiceConfig;
    type Health = StorageHealthStatus;
    type Error = crate::error::NestGateError;

    async fn handle_request(&self, request: ServiceRequest) -> Result<ServiceResponse, Self::Error> {
        match request.path.as_str() {
            "/protocols" => self.list_protocols(request).await,
            "/nfs/status" => self.nfs_status(request).await,
            "/smb/status" => self.smb_status(request).await,
            "/iscsi/status" => self.iscsi_status(request).await,
            _ => Ok(ServiceResponse::error(request.id, 404, "Unknown protocol endpoint".to_string())),
        }
    }

    // ... implement other methods
}
```

---

## 📅 **Week 3: Complete Service Migration**

### **Day 1-2: Tier Management Service**
```rust
// src/songbird_integration/tiers.rs
pub struct NestGateTierService {
    tier_manager: Arc<crate::tiers::TierManager>,
    ai_predictor: Option<Arc<crate::ai::TierPredictor>>,
}

// Implement UniversalService for tier management
// Handle hot/warm/cold tier operations
// Integrate AI-based tier predictions
```

### **Day 3-4: Main Application Integration**
```rust
// src/main.rs
use songbird_orchestrator::prelude::*;
use crate::songbird_integration::services::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Songbird orchestrator
    let config = OrchestratorConfig::from_file("config/orchestrator.yaml")?;
    let orchestrator = Orchestrator::new(config).await?;

    // Register NestGate services
    register_nestgate_services(&orchestrator).await?;

    // Start orchestrator (handles all service management)
    orchestrator.start().await?;
    
    println!("🎼 NestGate NAS services running under Songbird orchestration");
    
    // Songbird manages the lifecycle
    orchestrator.wait_for_shutdown().await?;
    
    Ok(())
}

async fn register_nestgate_services(orchestrator: &Orchestrator) -> Result<(), Box<dyn std::error::Error>> {
    // ZFS Service
    let zfs_service = NestGateZfsService::new();
    let zfs_config = ZfsServiceConfig::from_env()?;
    orchestrator.register_service("nestgate-zfs", zfs_service, zfs_config).await?;

    // Storage Protocols Service
    let storage_service = NestGateStorageService::new();
    let storage_config = StorageServiceConfig::from_env()?;
    orchestrator.register_service("nestgate-storage", storage_service, storage_config).await?;

    // Tier Management Service
    let tier_service = NestGateTierService::new();
    let tier_config = TierServiceConfig::from_env()?;
    orchestrator.register_service("nestgate-tiers", tier_service, tier_config).await?;

    Ok(())
}
```

### **Day 5: Configuration Integration**
```yaml
# config/nestgate.yaml
orchestrator:
  bind_address: "0.0.0.0"
  port: 8080
  max_connections: 1000
  health_check_interval: 30

services:
  nestgate-zfs:
    enabled: true
    config:
      pools_path: "/zpool"
      enable_snapshots: true
      tier_management: true
  
  nestgate-storage:
    enabled: true
    config:
      protocols: ["nfs", "smb", "iscsi"]
      nfs_exports_path: "/etc/exports"
      smb_config_path: "/etc/samba/smb.conf"
  
  nestgate-tiers:
    enabled: true
    config:
      hot_threshold_gb: 100
      warm_threshold_gb: 1000
      ai_prediction: true
```

---

## 📅 **Week 4: Cleanup & Production Readiness**

### **Day 1-2: Remove Duplicate Code**
```bash
# Remove duplicate orchestration infrastructure
rm -rf src/orchestrator/
rm src/service_registry.rs
rm src/connection_proxy.rs
rm src/health_monitor.rs

# Update imports throughout codebase
find src/ -name "*.rs" -exec sed -i 's/crate::orchestrator/songbird_orchestrator/g' {} \;
find src/ -name "*.rs" -exec sed -i 's/crate::service_registry/songbird_orchestrator::registry/g' {} \;

# Clean up Cargo.toml dependencies
# Remove orchestrator-specific dependencies
```

### **Day 3: Testing Integration**
```rust
// tests/songbird_integration_test.rs
#[tokio::test]
async fn test_zfs_service_through_orchestrator() {
    let orchestrator = setup_test_orchestrator().await;
    
    // Test ZFS operations through Songbird
    let response = orchestrator
        .handle_request("nestgate-zfs", ServiceRequest::new("/pools", "GET"))
        .await
        .unwrap();
    
    assert_eq!(response.status_code, 200);
}

#[tokio::test]
async fn test_storage_protocols_through_orchestrator() {
    let orchestrator = setup_test_orchestrator().await;
    
    // Test storage protocol operations
    let response = orchestrator
        .handle_request("nestgate-storage", ServiceRequest::new("/protocols", "GET"))
        .await
        .unwrap();
    
    assert_eq!(response.status_code, 200);
}
```

### **Day 4-5: Documentation & Deployment**
```bash
# Update documentation
# Create deployment scripts
# Performance testing
# Final integration verification
```

---

## 🎯 **Success Criteria**

### **Technical Checkpoints**
- [ ] All NestGate services implement `UniversalService` trait
- [ ] ZFS operations work through Songbird orchestrator
- [ ] Storage protocols accessible via orchestrator endpoints
- [ ] Health monitoring integrated with Songbird
- [ ] Configuration unified under Songbird patterns
- [ ] All duplicate orchestration code removed
- [ ] Tests pass with Songbird integration
- [ ] Performance equivalent or better than before

### **Functional Verification**
- [ ] NAS functionality unchanged from user perspective
- [ ] ZFS pools/datasets/snapshots fully operational
- [ ] NFS/SMB/iSCSI protocols working
- [ ] Tier management and AI predictions functional
- [ ] UI connects to new Songbird endpoints
- [ ] Configuration management simplified

---

## 🚨 **Risk Mitigation**

### **Rollback Plan**
```bash
# Keep original code in separate branch
git checkout main
git branch -c backup-pre-songbird

# If integration fails, can quickly revert
git checkout backup-pre-songbird
```

### **Testing Strategy**
1. **Unit Tests**: Each service independently
2. **Integration Tests**: Services through Songbird
3. **End-to-End Tests**: Complete user workflows
4. **Performance Tests**: Ensure no regression
5. **Load Tests**: Verify orchestrator can handle NAS workloads

---

## 📞 **Support Resources**

### **Songbird Team Contact**
- Architecture questions: Direct consultation available
- Implementation issues: Code review support
- Performance concerns: Load testing assistance

### **Key References**
- `songbird/examples/nestgate_integration.rs` - Working example
- `songbird/README.md` - Quick start guide
- `songbird/docs/` - Comprehensive documentation
- `SONGBIRD_ORCHESTRATOR_HANDOFF.md` - Complete handoff guide

---

*This roadmap should be updated as implementation progresses and issues are discovered.* 