use std::collections::HashMap;
/// **STORAGE ACCESS CONTROL AND METADATA TYPES**
///
/// This module contains storage access control, permissions, and metadata types.
/// Split from consolidated_storage_types.rs for better maintainability and 2000-line compliance.
use serde::{Deserialize, Serialize};
// Import unified enums
// use crate::canonical_modernization::UnifiedAccessType; // Currently unused

// ==================== SECTION ====================

/// Storage access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccessControl {
    /// Access control enabled
    pub enabled: bool,
    /// Access control type
    pub control_type: AccessControlType,

    /// User-based permissions
    pub user_permissions: HashMap<String, Vec<StoragePermission>>,

    /// Group-based permissions
    pub group_permissions: HashMap<String, Vec<StoragePermission>>,

    /// Role-based permissions
    pub role_permissions: HashMap<String, Vec<StoragePermission>>,

    /// Default permissions for new access
    pub default_permissions: Vec<StoragePermission>,

    /// Audit logging enabled
    pub audit_enabled: bool,
}

/// Access control types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccessControlType {
    /// POSIX access control (Unix-style)
    Posix,
    /// Access Control Lists (ACL)
    Acl,
    /// Role-Based Access Control (RBAC)
    Rbac,
    /// Attribute-Based Access Control (ABAC)
    Abac,
    /// Custom access control
    Custom(String),
}
/// Storage permissions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StoragePermission {
    /// Read permission
    Read,
    /// Write permission
    Write,
    /// Execute permission
    Execute,
    /// Delete permission
    Delete,
    /// List permission (for directories)
    List,
    /// Create permission
    Create,
    /// Modify metadata permission
    ModifyMetadata,
    /// Change permissions
    ChangePermissions,
    /// Take ownership
    TakeOwnership,
    /// Full control
    FullControl,
    /// Custom permission
    Custom(String),
}
/// Storage access audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccessAuditEntry {
    /// Audit entry ID
    pub id: String,
    /// Timestamp of the access
    pub timestamp: DateTime<Utc>,

    /// User who accessed the resource
    pub user: String,

    /// Resource that was accessed

    /// Action that was performed
    pub action: StorageAccessAction,

    /// Result of the access attempt
    pub result: AccessResult,

    /// Additional context
    pub context: HashMap<String, String>,

    /// Source IP address
    pub source_ip: Option<String>,

    /// User agent or client information
    pub user_agent: Option<String>,
}

/// Storage access actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageAccessAction {
    /// Read operation
    Read,
    /// Write operation
    Write,
    /// Delete operation
    Delete,
    /// Create operation
    Create,
    /// List operation
    List,
    /// Metadata modification
    ModifyMetadata,
    /// Permission change
    ChangePermissions,
    /// Ownership change
    ChangeOwnership,
    /// Mount operation
    Mount,
    /// Unmount operation
    Unmount,
    /// Custom action
    Custom(String),
}
/// Access attempt result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccessResult {
    /// Access granted
    Granted,
    /// Access denied - insufficient permissions
    Denied,
    /// Access denied - resource not found
    NotFound,
    /// Access denied - authentication required
    AuthenticationRequired,
    /// Access denied - authorization failed
    AuthorizationFailed,
    /// Access denied - quota exceeded
    QuotaExceeded,
    /// Access denied - resource locked
    ResourceLocked,
    /// Access denied - custom reason
    Custom(String),
}
// ==================== SECTION ====================

/// Storage resource metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResourceMetadata {
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,

    /// Last access timestamp
    pub accessed_at: Option<DateTime<Utc>>,

    /// Resource version
    pub version: String,

    /// Content checksum/hash
    pub checksum: Option<String>,

    /// Content type/MIME type
    pub content_type: Option<String>,

    /// Custom metadata fields
    pub custom_fields: HashMap<String, String>,

    /// Resource owner
    pub owner: Option<String>,

    /// Resource creator
    pub creator: Option<String>,
}

/// Storage resource tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResourceTags {
    /// System tags (managed by the system)
    pub system_tags: HashMap<String, String>,
    /// User tags (managed by users)
    pub user_tags: HashMap<String, String>,

    /// Automatic tags (generated by analysis)
    pub auto_tags: HashMap<String, String>,

    /// Tag creation timestamp
    pub created_at: DateTime<Utc>,

    /// Tag last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Storage lifecycle policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLifecyclePolicy {
    /// Policy name
    pub name: String,
    /// Policy enabled
    pub enabled: bool,

    /// Lifecycle rules
    pub rules: Vec<LifecycleRule>,

    /// Policy creation timestamp
    pub created_at: DateTime<Utc>,

    /// Policy last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Lifecycle rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleRule {
    /// Rule name
    pub name: String,
    /// Rule enabled
    pub enabled: bool,

    /// Conditions for rule activation
    pub conditions: LifecycleConditions,

    /// Actions to take when conditions are met
    pub actions: Vec<LifecycleAction>,

    /// Rule priority (lower number = higher priority)
    pub priority: u32,
}

/// Lifecycle rule conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleConditions {
    /// Age-based conditions
    pub age: Option<LifecycleAgeCondition>,
    /// Size-based conditions
    pub size: Option<LifecycleSizeCondition>,

    /// Access-based conditions
    pub access: Option<LifecycleAccessCondition>,

    /// Tag-based conditions
    pub tags: Option<HashMap<String, String>>,

    /// Custom conditions
    pub custom: Option<HashMap<String, serde_json::Value>>,
}

/// Age-based lifecycle condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleAgeCondition {
    /// Minimum age for rule activation
    pub min_age: Option<chrono::Duration>,
    /// Maximum age for rule activation
    pub max_age: Option<chrono::Duration>,

    /// Age calculation method
    pub age_method: AgeCalculationMethod,
}

/// Age calculation methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgeCalculationMethod {
    /// Age since creation
    Creation,
    /// Age since last modification
    LastModification,
    /// Age since last access
    LastAccess,
    /// Custom age calculation
    Custom(String),
}
/// Size-based lifecycle condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleSizeCondition {
    /// Minimum size in bytes
    pub min_size: Option<u64>,
    /// Maximum size in bytes
    pub cache_size_bytes: Option<u64>,
}

/// Access-based lifecycle condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleAccessCondition {
    /// Minimum access frequency
    pub min_access_frequency: Option<f64>,
    /// Maximum access frequency
    pub max_access_frequency: Option<f64>,

    /// Access frequency calculation period
    pub frequency_period: chrono::Duration,
}

/// Lifecycle actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LifecycleAction {
    /// Move to different storage tier
    Transition { tier: String },
    /// Delete the resource
    Delete,
    /// Archive the resource
    Archive { destination: String },
    /// Compress the resource
    Compress,
    /// Encrypt the resource
    Encrypt,
    /// Create snapshot
    Snapshot,
    /// Send notification
    Notify { recipients: Vec<String> },
    /// Custom action
    Custom {
        action: String,
        parameters: HashMap<String, serde_json::Value>,
    },
}
// ==================== SECTION ====================

impl Default for StorageAccessControl {
    fn default() -> Self {
        Self {
            enabled: true,
            control_type: AccessControlType::Posix,
            user_permissions: HashMap::new(),
            group_permissions: HashMap::new(),
            role_permissions: HashMap::new(),
            default_permissions: vec![StoragePermission::Read],
            audit_enabled: false,
        }
    }
}

impl Default for StorageResourceMetadata {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            modified_at: now,
            accessed_at: None,
            version: "1.0.0".to_string(),
            checksum: None,
            content_type: None,
            custom_fields: HashMap::new(),
            owner: None,
            creator: None,
        }
    }
}

impl Default for StorageResourceTags {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            system_tags: HashMap::new(),
            user_tags: HashMap::new(),
            auto_tags: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

// ==================== SECTION ====================

impl StorageAccessControl {
    /// Check if a user has specific permission
    pub const fn user_has_permission(&self, user: &str, permission: &StoragePermission) -> bool {
        if !self.enabled {
            return true; // Access control disabled
        }

        // Check user permissions
        if let Some(user_perms) = self.user_permissions.get(user) {
            if user_perms.contains(permission)
                || user_perms.contains(&StoragePermission::FullControl)
            {
                return true;
            }
        }

        // Check default permissions
        self.default_permissions.contains(permission)
            || self
                .default_permissions
                .contains(&StoragePermission::FullControl)
    }

    /// Grant permission to user
    pub fn grant_user_permission(&mut self, user: String, permission: StoragePermission) {
        self.user_permissions
            .entry(user)
            .or_default()
            .push(permission);
    }

    /// Revoke permission from user
    pub fn revoke_user_permission(&mut self, user: &str, permission: &StoragePermission) {
        if let Some(perms) = self.user_permissions.get_mut(user) {
            perms.retain(|p| p != permission);
        }
    }
}

impl StorageResourceMetadata {
    /// Update modification timestamp
    pub fn touch(&mut self) {
        self.modified_at = Utc::now();
    }

    /// Record access
    pub fn record_access(&mut self) {
        self.accessed_at = Some(Utc::now());
    }

    /// Calculate age since creation
    pub const fn age_since_creation(&self) -> chrono::Duration {
        Utc::now() - self.created_at
    }

    /// Calculate age since last modification
    pub const fn age_since_modification(&self) -> chrono::Duration {
        Utc::now() - self.modified_at
    }

    /// Calculate age since last access
    pub const fn age_since_access(&self) -> Option<chrono::Duration> {
        self.accessed_at.map(|access_time| Utc::now() - access_time)
    }
}

impl StorageResourceTags {
    /// Add system tag
    pub fn add_system_tag(&mut self, key: String, value: String) {
        self.system_tags.insert(key, value);
        self.updated_at = Utc::now();
    }

    /// Add user tag
    pub fn add_user_tag(&mut self, key: String, value: String) {
        self.user_tags.insert(key, value);
        self.updated_at = Utc::now();
    }

    /// Get all tags combined
    pub fn all_tags(&self) -> HashMap<String, String> {
        let mut all_tags = HashMap::new();
        all_tags.extend(self.system_tags.clone());
        all_tags.extend(self.user_tags.clone());
        all_tags.extend(self.auto_tags.clone());
        all_tags
    }

    /// Check if tag exists
    pub const fn has_tag(&self, key: &str) -> bool {
        self.system_tags.contains_key(key)
            || self.user_tags.contains_key(key)
            || self.auto_tags.contains_key(key)
    }
}

impl LifecycleRule {
    /// Check if rule conditions are met for a resource
    pub const fn matches_conditions(
        &self,
        metadata: &StorageResourceMetadata,
        tags: &StorageResourceTags,
    ) -> bool {
        if !self.enabled {
            return false;
        }

        // Check age conditions
        if let Some(age_condition) = &self.conditions.age {
            let age = match age_condition.age_method {
                AgeCalculationMethod::Creation => metadata.age_since_creation(),
                AgeCalculationMethod::LastModification => metadata.age_since_modification(),
                AgeCalculationMethod::LastAccess => metadata
                    .age_since_access()
                    .unwrap_or(chrono::Duration::zero()),
                AgeCalculationMethod::Custom(_) => chrono::Duration::zero(), // Custom logic needed
            };

            if let Some(min_age) = age_condition.min_age {
                if age < min_age {
                    return false;
                }
            }

            if let Some(max_age) = age_condition.max_age {
                if age > max_age {
                    return false;
                }
            }
        }

        // Check tag conditions
        if let Some(required_tags) = &self.conditions.tags {
            let all_tags = tags.all_tags();
            for (key, value) in required_tags {
                if all_tags.get(key) != Some(value) {
                    return false;
                }
            }
        }

        true
    }
}
