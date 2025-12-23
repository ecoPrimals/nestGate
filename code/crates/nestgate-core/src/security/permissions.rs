use crate::error::{NetworkError};
/// Permissions module for NestGate
///
/// This module provides permission-related functionality for the NestGate system.
use std::collections::{HashMap, HashSet};
// Removed unused std import

use crate::security::{Role, Permission, AccessLevel};

/// Permission manager for handling access control
#[derive(Debug, Clone)]
/// Manager for Permission operations
pub struct PermissionManager {
    /// Permissions by role
    role_permissions: HashMap<Role, HashSet<String>>,
    /// Custom permissions by user
    user_permissions: HashMap<String, HashSet<String>>,

    /// All defined permissions
    permissions: HashMap<String, Permission>,
}

impl PermissionManager {
    /// Create a new permission manager
    #[must_use]
    pub fn new() -> Self {
        let mut manager = Self {
            role_permissions: HashMap::new(),
            user_permissions: HashMap::new(),
            permissions: HashMap::new(),
        };

        // Set up default permissions for roles
        manager.setup_default_permissions();

        manager
    }

    /// Set up default permissions for roles
    fn setup_default_permissions(&mut self) {
        // Admin role has all permissions
        self.role_permissions.insert(Role::Admin, HashSet::new());

        // Operator role has most permissions except admin-specific ones
        let mut operator_perms = HashSet::new();
        operator_perms.insert("system.read".to_string());
        operator_perms.insert("system.write".to_string());
        operator_perms.insert("nfs.read".to_string());
        operator_perms.insert("nfs.write".to_string());
        operator_perms.insert("smb.read".to_string());
        operator_perms.insert("smb.write".to_string());
        operator_perms.insert("zfs.read".to_string());
        operator_perms.insert("zfs.write".to_string());
        operator_perms.insert("network.read".to_string());
        operator_perms.insert("network.write".to_string());
        operator_perms.insert("user.read".to_string());
        self.role_permissions.insert(Role::Operator, operator_perms);

        // User role has basic permissions
        let mut user_perms = HashSet::new();
        user_perms.insert("system.read".to_string());
        user_perms.insert("nfs.read".to_string());
        user_perms.insert("smb.read".to_string());
        user_perms.insert("zfs.read".to_string());
        user_perms.insert("network.read".to_string());
        self.role_permissions.insert(Role::User, user_perms);

        // ReadOnly role has only read permissions
        let mut readonly_perms = HashSet::new();
        readonly_perms.insert("system.read".to_string());
        readonly_perms.insert("nfs.read".to_string());
        readonly_perms.insert("smb.read".to_string());
        readonly_perms.insert("zfs.read".to_string());
        readonly_perms.insert("network.read".to_string());
        self.role_permissions.insert(Role::ReadOnly, readonly_perms);

        // Guest role has minimal permissions
        let mut guest_perms = HashSet::new();
        guest_perms.insert("system.info".to_string());
        self.role_permissions.insert(Role::Guest, guest_perms);

        // Define all permissions
        self.register_permission(Permission::new("system.read"));
        self.register_permission(Permission::new("system.write"));
        self.register_permission(Permission::new("system.admin"));
        self.register_permission(Permission::new("system.info"));

        // NFS permissions
        self.register_permission(Permission::new("nfs.read"));
        self.register_permission(Permission::new("nfs.write"));
        self.register_permission(Permission::new("nfs.admin"));

        // SMB permissions
        self.register_permission(Permission::new("smb.read"));
        self.register_permission(Permission::new("smb.write"));
        self.register_permission(Permission::new("smb.admin"));

        // ZFS permissions
        self.register_permission(Permission::new("zfs.read"));
        self.register_permission(Permission::new("zfs.write"));
        self.register_permission(Permission::new("zfs.admin"));

        // Network permissions
        self.register_permission(Permission::new("network.read"));
        self.register_permission(Permission::new("network.write"));
        self.register_permission(Permission::new("network.admin"));

        // User permissions
        self.register_permission(Permission::new("user.read"));
        self.register_permission(Permission::new("user.write"));
        self.register_permission(Permission::new("user.admin"));
    }

    /// Register a new permission
    pub fn register_permission(&mut self, permission: Permission) {
        self.permissions.insert(permission.name.clone(), permission);
    }

    /// Check if a role has a permission
    pub fn role_has_permission(&self, role: Role, permission: &str) -> bool {
        // Admin role has all permissions
        if role == Role::Admin {
            return true;
        }

        // Check if the role has the specific permission
        if let Some(perms) = self.role_permissions.get(&role) {
            return perms.contains(permission);
        }

        false
    }

    /// Check if a user has a permission
    pub fn user_has_permission(&self, user_id: &str, role: Role, permission: &str) -> bool {
        // First check role permissions
        if self.role_has_permission(role, permission) {
            return true;
        }

        // Then check user-specific permissions
        if let Some(perms) = self.user_permissions.get(user_id) {
            return perms.contains(permission);
        }

        false
    }

    /// Grant a permission to a user
    pub fn grant_permission(&mut self, user_id: &str, permission: &str) {
        self.user_permissions
            .entry(user_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(permission.to_string());
    }

    /// Revoke a permission from a user
    pub fn revoke_permission(&mut self, user_id: &str, permission: &str) {
        if let Some(perms) = self.user_permissions.get_mut(user_id) {
            perms.remove(permission);
        }
    }

    /// Get all permissions for a role
    pub fn get_role_permissions(&self, role: Role) -> Vec<Permission> {
        let mut result = Vec::new();

        // Admin role has all permissions
        if role == Role::Admin {
            return self.permissions.values().cloned().collect();
        }

        // Get permissions for the role
        if let Some(perms) = self.role_permissions.get(&role) {
            for perm_name in perms {
                if let Some(perm) = self.permissions.get(perm_name) {
                    result.push(perm.clone());
                }
            }
        }

        result
    }

    /// Get all permissions for a user
    pub fn get_user_permissions(&self, user_id: &str, role: Role) -> Vec<Permission> {
        let mut result = self.get_role_permissions(role);

        // Add user-specific permissions
        if let Some(perms) = self.user_permissions.get(user_id) {
            for perm_name in perms {
                if let Some(perm) = self.permissions.get(perm_name) {
                    // Only add if not already in the list
                    if !result.iter().any(|p| p.name == perm.name) {
                        result.push(perm.clone());
                    }
                }
            }
        }

        result
    }
}

impl Default for PermissionManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_manager_new() {
        let manager = PermissionManager::new();
        assert!(!manager.role_permissions.is_empty());
        assert!(!manager.permissions.is_empty());
        assert!(manager.user_permissions.is_empty());
    }

    #[test]
    fn test_permission_manager_default() {
        let manager = PermissionManager::default();
        assert!(!manager.role_permissions.is_empty());
    }

    #[test]
    fn test_admin_has_all_permissions() {
        let manager = PermissionManager::new();
        assert!(manager.role_has_permission(Role::Admin, "system.read"));
        assert!(manager.role_has_permission(Role::Admin, "system.write"));
        assert!(manager.role_has_permission(Role::Admin, "system.admin"));
        assert!(manager.role_has_permission(Role::Admin, "nonexistent.permission"));
    }

    #[test]
    fn test_operator_has_read_write_permissions() {
        let manager = PermissionManager::new();
        assert!(manager.role_has_permission(Role::Operator, "system.read"));
        assert!(manager.role_has_permission(Role::Operator, "system.write"));
        assert!(manager.role_has_permission(Role::Operator, "zfs.read"));
        assert!(manager.role_has_permission(Role::Operator, "zfs.write"));
    }

    #[test]
    fn test_operator_lacks_admin_permissions() {
        let manager = PermissionManager::new();
        assert!(!manager.role_has_permission(Role::Operator, "system.admin"));
        assert!(!manager.role_has_permission(Role::Operator, "user.admin"));
    }

    #[test]
    fn test_user_has_read_permissions() {
        let manager = PermissionManager::new();
        assert!(manager.role_has_permission(Role::User, "system.read"));
        assert!(manager.role_has_permission(Role::User, "nfs.read"));
        assert!(manager.role_has_permission(Role::User, "zfs.read"));
    }

    #[test]
    fn test_user_lacks_write_permissions() {
        let manager = PermissionManager::new();
        assert!(!manager.role_has_permission(Role::User, "system.write"));
        assert!(!manager.role_has_permission(Role::User, "nfs.write"));
        assert!(!manager.role_has_permission(Role::User, "zfs.write"));
    }

    #[test]
    fn test_readonly_only_has_read_permissions() {
        let manager = PermissionManager::new();
        assert!(manager.role_has_permission(Role::ReadOnly, "system.read"));
        assert!(manager.role_has_permission(Role::ReadOnly, "nfs.read"));
        assert!(!manager.role_has_permission(Role::ReadOnly, "system.write"));
        assert!(!manager.role_has_permission(Role::ReadOnly, "nfs.write"));
    }

    #[test]
    fn test_guest_minimal_permissions() {
        let manager = PermissionManager::new();
        assert!(manager.role_has_permission(Role::Guest, "system.info"));
        assert!(!manager.role_has_permission(Role::Guest, "system.read"));
        assert!(!manager.role_has_permission(Role::Guest, "system.write"));
    }

    #[test]
    fn test_grant_permission_to_user() {
        let mut manager = PermissionManager::new();
        let user_id = "user123";
        
        assert!(!manager.user_has_permission(user_id, Role::User, "special.permission"));
        
        manager.grant_permission(user_id, "special.permission");
        
        assert!(manager.user_has_permission(user_id, Role::User, "special.permission"));
    }

    #[test]
    fn test_revoke_permission_from_user() {
        let mut manager = PermissionManager::new();
        let user_id = "user123";
        
        manager.grant_permission(user_id, "special.permission");
        assert!(manager.user_has_permission(user_id, Role::User, "special.permission"));
        
        manager.revoke_permission(user_id, "special.permission");
        assert!(!manager.user_has_permission(user_id, Role::User, "special.permission"));
    }

    #[test]
    fn test_revoke_nonexistent_permission() {
        let mut manager = PermissionManager::new();
        let user_id = "user123";
        
        // Should not panic when revoking a permission the user doesn't have
        manager.revoke_permission(user_id, "nonexistent.permission");
    }

    #[test]
    fn test_user_inherits_role_permissions() {
        let manager = PermissionManager::new();
        let user_id = "user123";
        
        // User role has system.read permission
        assert!(manager.user_has_permission(user_id, Role::User, "system.read"));
    }

    #[test]
    fn test_user_specific_permissions_supplement_role() {
        let mut manager = PermissionManager::new();
        let user_id = "user123";
        
        // User role doesn't have system.write, but we grant it specifically
        assert!(!manager.role_has_permission(Role::User, "system.write"));
        
        manager.grant_permission(user_id, "system.write");
        
        assert!(manager.user_has_permission(user_id, Role::User, "system.write"));
    }

    #[test]
    fn test_get_role_permissions_admin() {
        let manager = PermissionManager::new();
        let perms = manager.get_role_permissions(Role::Admin);
        
        // Admin should have all registered permissions
        assert!(perms.len() > 10);
    }

    #[test]
    fn test_get_role_permissions_operator() {
        let manager = PermissionManager::new();
        let perms = manager.get_role_permissions(Role::Operator);
        
        assert!(!perms.is_empty());
        assert!(perms.iter().any(|p| p.name == "system.read"));
        assert!(perms.iter().any(|p| p.name == "system.write"));
    }

    #[test]
    fn test_get_role_permissions_readonly() {
        let manager = PermissionManager::new();
        let perms = manager.get_role_permissions(Role::ReadOnly);
        
        // ReadOnly should only have read permissions
        assert!(perms.iter().all(|p| p.name.ends_with(".read")));
    }

    #[test]
    fn test_get_user_permissions() {
        let mut manager = PermissionManager::new();
        let user_id = "user123";
        
        manager.grant_permission(user_id, "custom.permission");
        
        let perms = manager.get_user_permissions(user_id, Role::User);
        
        // Should include both role and user-specific permissions
        assert!(perms.iter().any(|p| p.name == "system.read")); // From role
    }

    #[test]
    fn test_get_user_permissions_no_duplicates() {
        let mut manager = PermissionManager::new();
        let user_id = "user123";
        
        // Grant a permission that the role already has
        manager.grant_permission(user_id, "system.read");
        
        let perms = manager.get_user_permissions(user_id, Role::User);
        
        // Should not have duplicates
        let system_read_count = perms.iter().filter(|p| p.name == "system.read").count();
        assert_eq!(system_read_count, 1);
    }

    #[test]
    fn test_register_custom_permission() {
        let mut manager = PermissionManager::new();
        let custom_perm = Permission::new("custom.action");
        
        manager.register_permission(custom_perm);
        
        assert!(manager.permissions.contains_key("custom.action"));
    }

    #[test]
    fn test_multiple_users_independent_permissions() {
        let mut manager = PermissionManager::new();
        
        manager.grant_permission("user1", "perm1");
        manager.grant_permission("user2", "perm2");
        
        assert!(manager.user_has_permission("user1", Role::User, "perm1"));
        assert!(!manager.user_has_permission("user1", Role::User, "perm2"));
        assert!(manager.user_has_permission("user2", Role::User, "perm2"));
        assert!(!manager.user_has_permission("user2", Role::User, "perm1"));
    }

    #[test]
    fn test_permission_manager_clone() {
        let mut manager = PermissionManager::new();
        manager.grant_permission("user1", "special.perm");
        
        let cloned = manager.clone();
        
        assert!(cloned.user_has_permission("user1", Role::User, "special.perm"));
    }

    #[test]
    fn test_all_default_permissions_registered() {
        let manager = PermissionManager::new();
        
        let expected_permissions = vec![
            "system.read", "system.write", "system.admin", "system.info",
            "nfs.read", "nfs.write", "nfs.admin",
            "smb.read", "smb.write", "smb.admin",
            "zfs.read", "zfs.write", "zfs.admin",
            "network.read", "network.write", "network.admin",
            "user.read", "user.write", "user.admin",
        ];
        
        for perm in expected_permissions {
            assert!(manager.permissions.contains_key(perm), "Missing permission: {}", perm);
        }
    }
}