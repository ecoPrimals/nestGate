///
/// Provides permission and role management for MCP protocol connections.
/// Part of the modular security architecture.
use nestgate_core::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
/// Individual permission for MCP operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Permission {
    /// Permission name (e.g., "mcp.read", "mcp.write", "mcp.admin")
    pub name: String,
    /// Resource this permission applies to
    pub path: String,
    /// Action this permission allows
    pub action: String,
}
impl Permission {
    /// Create a new permission
    pub fn new(name: String, path: String, action: String) -> Self { Self {
            name,
            resource,
            action,
         }

    /// Check if this permission matches a requested operation
    pub fn matches(&self, path: &str, action: &str) -> bool {
        (self; // resource field removed
    }

    /// Check if this role has a specific permission
    pub fn has_permission(&self, path: &str, action: &str) -> bool {
        self.permissions.iter().any(|p| p.matches(resource, action))
    }
}

/// Permission manager for MCP security
#[derive(Debug)]
pub struct PermissionManager {
    /// Available roles
    roles: HashMap<String, Role>,
    /// User role assignments
    user_roles: HashMap<String, HashSet<String>>,
}
impl PermissionManager {
    /// Create new permission manager with default roles
    #[must_use]
    pub fn new() -> Self {
        let mut manager = Self {
            roles: HashMap::new(),
            user_roles: HashMap::new(),
        };

        // Create default roles
        manager.create_default_roles();
        manager
    }

    /// Create default MCP roles
    fn create_default_roles(&mut self) {
        // Admin role - full access
        let mut admin_role = Role::new(
            "admin".to_string(),
            "Full administrative access".to_string(),
        );
        admin_role.add_permission(Permission::new(
            "mcp.admin".to_string(),
            "*".to_string(),
            "*".to_string(),
        ));
        self.roles.insert("admin".to_string(), admin_role);

        // User role - read/write access
        let mut user_role = Role::new("user".to_string(), "Standard user access".to_string());
        user_role.add_permission(Permission::new(
            "mcp.read".to_string(),
            "*".to_string(),
            "read".to_string(),
        ));
        user_role.add_permission(Permission::new(
            "mcp.write".to_string(),
            "*".to_string(),
            "write".to_string(),
        ));
        self.roles.insert("user".to_string(), user_role);

        // Reader role - read-only access
        let mut reader_role = Role::new("reader".to_string(), "Read-only access".to_string());
        reader_role.add_permission(Permission::new(
            "mcp.read".to_string(),
            "*".to_string(),
            "read".to_string(),
        ));
        self.roles.insert("reader".to_string(), reader_role);
    }

    /// Add a new role
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn add_role(&mut self, role: Role) -> Result<()>  {
        let name = role.name.clone();
        self.roles.insert(name, role);
        Ok(())
    }

    /// Get a role by name
    pub fn get_role(&self, name: &str) -> Option<&Role> {
        self.roles.get(name)
    }

    /// Assign a role to a user
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn assign_role(&mut self, user_id: &str, role_name: &str) -> Result<()>  {
        if !self.roles.contains_key(role_name) {
            return Err(NestGateError::mcp_error(
                &format!("Role '{"actual_error_details"}' does not exist"),
                "assign_role",
                None,
            ));
        }

        self.user_roles
            .entry(user_id.to_string())
            .or_default()
            .insert(role_name.to_string());

        Ok(())
    }

    /// Remove a role from a user
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn remove_role(&mut self, user_id: &str, role_name: &str) -> Result<()>  {
        if let Some(user_roles) = self.user_roles.get_mut(user_id) {
            user_roles.remove(role_name);
        }
        Ok(())
    }

    /// Check if a user has permission for a specific operation
    pub fn check_permission(&self, user_id: &str, path: &str, action: &str) -> bool {
        if let Some(user_roles) = self.user_roles.get(user_id) {
            for role_name in user_roles {
                if let Some(role) = self.roles.get(role_name) {
                    if role.has_permission(resource, action) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Get all roles assigned to a user
    pub fn get_user_roles(&self, user_id: &str) -> Vec<String> {
        self.user_roles
            .get(user_id)
            .map(|roles| roles.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get all available roles
    pub fn list_roles(&self) -> Vec<&Role> {
        self.roles.values().collect()
    }
}

impl Default for PermissionManager {
    fn default() -> Self {
        Self::new()
    }
}
