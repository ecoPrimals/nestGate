# 🎮 **STEAM DATA SERVICE SPECIFICATION**

**Version**: 1.0.0  
**Date**: September 16, 2025  
**Status**: 🚧 **SPECIFICATION COMPLETE** - Implementation pending  
**Parent**: NestGate Data Service Specification

---

## 🎯 **STEAM DATA SERVICE MISSION**

### **Core Responsibility**
> *"NestGate handles Steam data storage, federation, and access - NOT Steam client management or game analytics."*

### **Primal Boundaries for Steam** 🏛️
- **🏠 NestGate**: Steam data storage, save file federation, library caching
- **🎵 Songbird**: Steam client orchestration, download workflows, update management
- **🐿️ Squirrel**: Game analytics, recommendation engines, play pattern analysis
- **🍄 Toadstool**: Steam network protocols, P2P game connections
- **🛡️ BearDog**: Steam account security, family sharing permissions

---

## 🎮 **STEAM DATA ARCHITECTURE**

### **Steam Data Types** 📊
```rust
// Core Steam data structures for NestGate storage
pub struct SteamGameMetadata {
    pub app_id: u32,                    // Steam App ID
    pub name: String,                   // Game name
    pub install_dir: String,            // Installation directory name
    pub size_on_disk: u64,             // Bytes
    pub last_updated: SystemTime,       // Last update timestamp
    pub build_id: u32,                 // Steam build ID
    pub executable_path: PathBuf,       // Main executable
    pub launch_options: Vec<String>,    // Launch parameters
    pub workshop_items: Vec<u64>,       // Subscribed workshop item IDs
}

pub struct SteamSaveData {
    pub app_id: u32,                    // Game App ID
    pub save_files: HashMap<PathBuf, SaveFile>,
    pub cloud_enabled: bool,            // Steam Cloud enabled
    pub last_sync: Option<SystemTime>,  // Last cloud sync
    pub conflict_resolution: ConflictStrategy,
}

pub struct SteamAchievementData {
    pub app_id: u32,
    pub achievements: HashMap<String, Achievement>,
    pub stats: HashMap<String, StatValue>,
    pub last_updated: SystemTime,
}
```

### **Steam Federation Targets** 🌐
```rust
pub struct SteamFederationConfig {
    pub gaming_rigs: Vec<GamingRigNode>,
    pub backup_nodes: Vec<BackupNode>,
    pub sync_strategy: SteamSyncStrategy,
    pub encryption: SteamDataEncryption,
}

pub enum SteamSyncStrategy {
    RealTime,           // Immediate sync on save
    Periodic(Duration), // Sync every N minutes
    OnGameExit,         // Sync when game closes
    Manual,             // User-triggered sync
}
```

---

## 🏠 **NESTGATE STEAM RESPONSIBILITIES**

### **✅ What NestGate Handles**

#### **Game Library Data Storage** 📚
```rust
impl SteamDataService {
    // Store and retrieve game metadata
    pub async fn store_game_metadata(&self, metadata: SteamGameMetadata) -> Result<()>;
    pub async fn get_game_metadata(&self, app_id: u32) -> Result<SteamGameMetadata>;
    pub async fn list_installed_games(&self) -> Result<Vec<SteamGameMetadata>>;
    
    // Installation path management
    pub async fn register_game_installation(&self, app_id: u32, path: PathBuf) -> Result<()>;
    pub async fn get_installation_path(&self, app_id: u32) -> Result<PathBuf>;
}
```

#### **Save Data Federation** 💾
```rust
impl SteamSaveDataFederation {
    // Real-time save sync between gaming rigs
    pub async fn sync_save_data(&self, app_id: u32, save_files: &[SaveFile]) -> Result<()>;
    pub async fn resolve_save_conflicts(&self, app_id: u32) -> Result<ConflictResolution>;
    pub async fn backup_save_data(&self, app_id: u32, target: BackupTarget) -> Result<()>;
    
    // Cross-rig save access
    pub async fn get_latest_saves(&self, app_id: u32) -> Result<Vec<SaveFile>>;
    pub async fn restore_saves_from_node(&self, app_id: u32, node: &str) -> Result<()>;
}
```

#### **Asset Cache Management** 🗃️
```rust
impl SteamAssetCache {
    // Cache frequently accessed game assets
    pub async fn cache_game_assets(&self, app_id: u32, assets: &[AssetFile]) -> Result<()>;
    pub async fn get_cached_asset(&self, app_id: u32, asset_path: &str) -> Result<Vec<u8>>;
    pub async fn preload_common_assets(&self, app_id: u32) -> Result<()>;
    
    // Distributed asset sharing
    pub async fn share_assets_with_nodes(&self, app_id: u32, nodes: &[String]) -> Result<()>;
    pub async fn download_assets_from_node(&self, app_id: u32, node: &str) -> Result<()>;
}
```

#### **Achievement & Stats Storage** 🏆
```rust
impl SteamAchievementService {
    // Store achievement data locally and federated
    pub async fn store_achievements(&self, app_id: u32, data: AchievementData) -> Result<()>;
    pub async fn get_achievements(&self, app_id: u32) -> Result<AchievementData>;
    pub async fn sync_achievements_across_rigs(&self, app_id: u32) -> Result<()>;
}
```

### **❌ What NestGate Does NOT Handle**

#### **Steam Client Management** (→ Songbird)
- Steam client installation/updates
- Game download orchestration  
- Steam service lifecycle management
- Download queue management
- Bandwidth throttling

#### **Game Analytics** (→ Squirrel)
- Play time analysis
- Game recommendation engines
- Performance analytics
- User behavior analysis
- Achievement difficulty scoring

#### **Network Protocols** (→ Toadstool)
- Steam network protocol implementation
- P2P game connections
- Voice chat protocols
- Streaming protocols

#### **Account Security** (→ BearDog)
- Steam account authentication
- Family sharing permissions
- Parental controls
- Account recovery

---

## 🌐 **STEAM FEDERATION SCENARIOS**

### **Gaming Biome Setup** 🏠
```yaml
# Example: Home gaming biome with multiple rigs
steam_federation:
  primary_rig:
    node_id: "gaming-rig-main"
    role: "leader"
    steam_library_path: "/mnt/games/steam"
    
  secondary_rigs:
    - node_id: "gaming-rig-bedroom"
      role: "follower" 
      sync_strategy: "real_time"
      
    - node_id: "gaming-rig-living-room"
      role: "follower"
      sync_strategy: "on_game_exit"
      
  backup_nodes:
    - node_id: "nas-server"
      role: "backup"
      sync_strategy: "periodic:1h"
```

### **Save Sync Workflow** 🔄
```rust
// Example: Real-time save sync between rigs
pub async fn handle_save_file_change(&self, app_id: u32, save_path: &Path) -> Result<()> {
    // 1. NestGate: Detect save file change
    let save_data = self.read_save_file(save_path).await?;
    
    // 2. NestGate: Encrypt and replicate to federation nodes
    self.replicate_save_data(app_id, &save_data, &self.federation_nodes).await?;
    
    // 3. NestGate: Update metadata and conflict resolution data
    self.update_save_metadata(app_id, save_path, SystemTime::now()).await?;
    
    // 4. Optional: Trigger Songbird workflow for additional processing
    if let Some(workflow_trigger) = &self.songbird_integration {
        workflow_trigger.notify_save_sync(app_id).await?;
    }
    
    Ok(())
}
```

### **Cross-Rig Game Launch** 🚀
```rust
// Example: Launch game with saves from another rig
pub async fn prepare_game_launch(&self, app_id: u32, target_rig: &str) -> Result<()> {
    // 1. NestGate: Check if saves are available locally
    if !self.has_latest_saves(app_id).await? {
        // 2. NestGate: Fetch latest saves from federation
        self.sync_saves_from_federation(app_id).await?;
    }
    
    // 3. NestGate: Ensure game assets are cached locally
    self.preload_game_assets(app_id).await?;
    
    // 4. Songbird: Handle actual game launch orchestration
    // (This is NOT NestGate's responsibility)
    
    Ok(())
}
```

---

## 🔐 **STEAM DATA SECURITY**

### **Encryption Standards** 🛡️
```rust
pub struct SteamDataEncryption {
    pub save_data_encryption: EncryptionConfig,     // AES-256-GCM for saves
    pub asset_encryption: EncryptionConfig,         // Optional for assets
    pub metadata_encryption: EncryptionConfig,      // Always encrypted
    pub key_rotation_policy: KeyRotationPolicy,
}

// Save data always encrypted in federation
impl SteamSaveDataFederation {
    async fn encrypt_save_data(&self, save_data: &[u8]) -> Result<Vec<u8>> {
        // Use NestGate's universal encryption system
        self.encryption_service.encrypt_data(save_data, &self.save_encryption_key).await
    }
}
```

### **Access Control Integration** 🔑
```rust
// Integration with the discovered security capability provider
pub struct SteamAccessControl {
    pub security_provider: SecurityProviderClient,
    pub access_policies: Vec<SteamAccessPolicy>,
}

impl SteamAccessControl {
    pub async fn authorize_save_access(&self, user_id: &str, app_id: u32) -> Result<bool> {
        // 1. Security provider: Verify user identity and permissions
        let authorized = self.security_provider.authorize_user(user_id, &format!("steam:{}:saves", app_id)).await?;
        
        if authorized {
            // 2. NestGate: Grant access to save data
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
```

---

## 🚀 **IMPLEMENTATION PHASES**

### **Phase 1: Basic Steam Data Storage** (Week 1-2)
- [ ] Steam game metadata storage
- [ ] Save file detection and storage
- [ ] Basic achievement data handling
- [ ] Local asset caching

### **Phase 2: Federation Infrastructure** (Week 3-4)
- [ ] Save data federation between nodes
- [ ] Conflict resolution for saves
- [ ] Encrypted replication
- [ ] Node discovery and health monitoring

### **Phase 3: Advanced Features** (Week 5-6)
- [ ] Real-time save sync
- [ ] Asset sharing between rigs
- [ ] Cross-rig game preparation
- [ ] Performance optimization

### **Phase 4: Ecosystem Integration** (Week 7-8)
- [ ] Songbird workflow integration
- [ ] BearDog security integration
- [ ] Monitoring and alerting
- [ ] Production deployment

---

## 🎯 **SUCCESS CRITERIA**

### **Functional Requirements**
- [ ] Game saves sync in real-time across gaming rigs
- [ ] No save data loss during node failures
- [ ] Games launch with correct saves on any rig
- [ ] Achievement data consistent across nodes
- [ ] Asset cache reduces game load times

### **Non-Functional Requirements**
- [ ] Save sync latency < 5 seconds
- [ ] Federation handles 10+ gaming rigs
- [ ] 99.9% save data availability
- [ ] Encryption for all federated data
- [ ] Zero manual configuration for new rigs

### **Integration Requirements**
- [ ] Clear separation from Songbird responsibilities
- [ ] No overlap with Squirrel analytics
- [ ] Proper BearDog security integration
- [ ] Toadstool handles network protocols only

---

**🎮 NestGate Steam Service: Intelligent gaming data management with seamless federation**

---

*Specification created: September 16, 2025*  
*Implementation target: Phase 1 completion by October 2025*  
*Focus: Data storage and federation excellence for gaming biomes* 