//! Permissions module for NestGate
//!
//! This module provides permission-related functionality for the NestGate system.

use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::security::{Role, Permission, AccessLevel};

/// Permission manager for handling access control
#[derive(Debug, Clone)]
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
    fn default() -> Self {
        Self::new()
    }
} 