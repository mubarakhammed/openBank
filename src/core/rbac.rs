use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, BTreeMap};
use uuid::Uuid;
use crate::core::error::{AppError, AppResult};

/// System roles with hierarchical permissions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// Super administrator - full system access
    SuperAdmin,
    /// Platform administrator - manage developers and projects
    Admin,
    /// Developer - manage own projects and access APIs
    Developer,
    /// Read-only access - view only permissions
    ReadOnly,
    /// Support staff - limited access for customer support
    Support,
    /// Auditor - access to audit logs and compliance reports
    Auditor,
}

impl Role {
    /// Get all roles that this role inherits permissions from
    pub fn get_inherited_roles(&self) -> Vec<Role> {
        match self {
            Role::SuperAdmin => vec![
                Role::Admin,
                Role::Developer,
                Role::ReadOnly,
                Role::Support,
                Role::Auditor,
            ],
            Role::Admin => vec![Role::Developer, Role::ReadOnly, Role::Support],
            Role::Developer => vec![Role::ReadOnly],
            Role::Support => vec![Role::ReadOnly],
            Role::Auditor => vec![Role::ReadOnly],
            Role::ReadOnly => vec![],
        }
    }

    /// Check if this role has permission for a specific action
    pub fn has_permission(&self, permission: &Permission) -> bool {
        let role_permissions = self.get_permissions();
        role_permissions.contains(permission)
    }

    /// Get all permissions for this role (including inherited)
    pub fn get_permissions(&self) -> HashSet<Permission> {
        let mut permissions = HashSet::new();
        
        // Add direct permissions
        permissions.extend(self.get_direct_permissions());
        
        // Add inherited permissions
        for inherited_role in self.get_inherited_roles() {
            permissions.extend(inherited_role.get_direct_permissions());
        }
        
        permissions
    }

    /// Get permissions directly assigned to this role (no inheritance)
    fn get_direct_permissions(&self) -> HashSet<Permission> {
        let mut permissions = HashSet::new();
        
        match self {
            Role::SuperAdmin => {
                permissions.insert(Permission::new("system", "manage"));
                permissions.insert(Permission::new("users", "delete"));
                permissions.insert(Permission::new("developers", "suspend"));
                permissions.insert(Permission::new("audit", "configure"));
            }
            Role::Admin => {
                permissions.insert(Permission::new("developers", "create"));
                permissions.insert(Permission::new("developers", "update"));
                permissions.insert(Permission::new("developers", "read"));
                permissions.insert(Permission::new("projects", "manage"));
                permissions.insert(Permission::new("audit", "read"));
                permissions.insert(Permission::new("system", "monitor"));
            }
            Role::Developer => {
                permissions.insert(Permission::new("projects", "create"));
                permissions.insert(Permission::new("projects", "update_own"));
                permissions.insert(Permission::new("projects", "delete_own"));
                permissions.insert(Permission::new("tokens", "generate"));
                permissions.insert(Permission::new("tokens", "refresh"));
                permissions.insert(Permission::new("api", "access"));
                permissions.insert(Permission::new("profile", "update_own"));
            }
            Role::Support => {
                permissions.insert(Permission::new("developers", "read"));
                permissions.insert(Permission::new("projects", "read"));
                permissions.insert(Permission::new("tokens", "read"));
                permissions.insert(Permission::new("support", "assist"));
            }
            Role::Auditor => {
                permissions.insert(Permission::new("audit", "read"));
                permissions.insert(Permission::new("compliance", "report"));
                permissions.insert(Permission::new("logs", "read"));
                permissions.insert(Permission::new("security", "monitor"));
            }
            Role::ReadOnly => {
                permissions.insert(Permission::new("profile", "read_own"));
                permissions.insert(Permission::new("projects", "read_own"));
                permissions.insert(Permission::new("documentation", "read"));
            }
        }
        
        permissions
    }
}

/// Represents a specific permission
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Permission {
    pub resource: String,
    pub action: String,
    pub conditions: Option<std::collections::BTreeMap<String, String>>,
}

impl Permission {
    pub fn new(resource: &str, action: &str) -> Self {
        Self {
            resource: resource.to_string(),
            action: action.to_string(),
            conditions: None,
        }
    }

    pub fn with_condition(mut self, key: &str, value: &str) -> Self {
        if self.conditions.is_none() {
            self.conditions = Some(BTreeMap::new());
        }
        self.conditions.as_mut().unwrap().insert(key.to_string(), value.to_string());
        self
    }

    /// Check if permission matches a required permission with conditions
    pub fn matches(&self, required: &Permission, context: &PermissionContext) -> bool {
        // Resource and action must match
        if self.resource != required.resource || self.action != required.action {
            return false;
        }

        // Check conditions
        if let Some(conditions) = &required.conditions {
            for (key, value) in conditions {
                match key.as_str() {
                    "owner" => {
                        if value == "self" && context.resource_owner_id != Some(context.user_id) {
                            return false;
                        }
                    }
                    "project_owner" => {
                        if value == "self" && context.project_owner_id != Some(context.user_id) {
                            return false;
                        }
                    }
                    "environment" => {
                        if context.environment.as_ref() != Some(value) {
                            return false;
                        }
                    }
                    _ => {
                        // Unknown condition, default to false for security
                        return false;
                    }
                }
            }
        }

        true
    }
}

/// Context for permission checking
#[derive(Debug, Clone)]
pub struct PermissionContext {
    pub user_id: Uuid,
    pub resource_owner_id: Option<Uuid>,
    pub project_owner_id: Option<Uuid>,
    pub environment: Option<String>,
    pub ip_address: String,
    pub additional_context: HashMap<String, String>,
}

impl PermissionContext {
    pub fn new(user_id: Uuid, ip_address: String) -> Self {
        Self {
            user_id,
            resource_owner_id: None,
            project_owner_id: None,
            environment: None,
            ip_address,
            additional_context: HashMap::new(),
        }
    }

    pub fn with_resource_owner(mut self, owner_id: Uuid) -> Self {
        self.resource_owner_id = Some(owner_id);
        self
    }

    pub fn with_project_owner(mut self, owner_id: Uuid) -> Self {
        self.project_owner_id = Some(owner_id);
        self
    }

    pub fn with_environment(mut self, env: String) -> Self {
        self.environment = Some(env);
        self
    }
}

/// User with roles and permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRoles {
    pub user_id: Uuid,
    pub roles: HashSet<Role>,
    pub custom_permissions: HashSet<Permission>,
    pub denied_permissions: HashSet<Permission>,
}

impl UserRoles {
    pub fn new(user_id: Uuid, role: Role) -> Self {
        let mut roles = HashSet::new();
        roles.insert(role);
        
        Self {
            user_id,
            roles,
            custom_permissions: HashSet::new(),
            denied_permissions: HashSet::new(),
        }
    }

    pub fn add_role(&mut self, role: Role) {
        self.roles.insert(role);
    }

    pub fn remove_role(&mut self, role: &Role) {
        self.roles.remove(role);
    }

    pub fn add_permission(&mut self, permission: Permission) {
        self.custom_permissions.insert(permission);
    }

    pub fn deny_permission(&mut self, permission: Permission) {
        self.denied_permissions.insert(permission);
    }

    /// Check if user has permission considering roles, custom permissions, and denials
    pub fn has_permission(&self, required: &Permission, context: &PermissionContext) -> bool {
        // First check if explicitly denied
        for denied in &self.denied_permissions {
            if denied.matches(required, context) {
                return false;
            }
        }

        // Check custom permissions
        for permission in &self.custom_permissions {
            if permission.matches(required, context) {
                return true;
            }
        }

        // Check role-based permissions
        for role in &self.roles {
            for permission in role.get_permissions() {
                if permission.matches(required, context) {
                    return true;
                }
            }
        }

        false
    }

    /// Get all effective permissions (roles + custom - denied)
    pub fn get_effective_permissions(&self) -> HashSet<Permission> {
        let mut permissions = HashSet::new();
        
        // Add role permissions
        for role in &self.roles {
            permissions.extend(role.get_permissions());
        }
        
        // Add custom permissions
        permissions.extend(self.custom_permissions.clone());
        
        // Remove denied permissions
        for denied in &self.denied_permissions {
            permissions.retain(|p| !p.matches(denied, &PermissionContext::new(self.user_id, "unknown".to_string())));
        }
        
        permissions
    }
}

/// RBAC Service for managing roles and permissions
#[derive(Clone)]
pub struct RbacService {
    // In production, this would be backed by a database
    user_roles: std::sync::Arc<std::sync::Mutex<HashMap<Uuid, UserRoles>>>,
}

impl RbacService {
    pub fn new() -> Self {
        Self {
            user_roles: std::sync::Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    /// Assign role to user
    pub fn assign_role(&self, user_id: Uuid, role: Role) -> AppResult<()> {
        let mut user_roles = self.user_roles.lock().unwrap();
        let user_role_entry = user_roles.entry(user_id).or_insert_with(|| {
            UserRoles::new(user_id, Role::ReadOnly)
        });
        
        user_role_entry.add_role(role);
        Ok(())
    }

    /// Remove role from user
    pub fn remove_role(&self, user_id: Uuid, role: Role) -> AppResult<()> {
        let mut user_roles = self.user_roles.lock().unwrap();
        if let Some(user_role_entry) = user_roles.get_mut(&user_id) {
            user_role_entry.remove_role(&role);
        }
        Ok(())
    }

    /// Check if user has permission
    pub fn check_permission(
        &self,
        user_id: Uuid,
        permission: &Permission,
        context: &PermissionContext,
    ) -> bool {
        let user_roles = self.user_roles.lock().unwrap();
        if let Some(user_role_entry) = user_roles.get(&user_id) {
            user_role_entry.has_permission(permission, context)
        } else {
            // Default to ReadOnly role for unknown users
            let default_roles = UserRoles::new(user_id, Role::ReadOnly);
            default_roles.has_permission(permission, context)
        }
    }

    /// Get user's roles
    pub fn get_user_roles(&self, user_id: Uuid) -> Option<UserRoles> {
        let user_roles = self.user_roles.lock().unwrap();
        user_roles.get(&user_id).cloned()
    }

    /// Authorize action (throws error if not permitted)
    pub fn authorize(
        &self,
        user_id: Uuid,
        permission: Permission,
        context: PermissionContext,
    ) -> AppResult<()> {
        if self.check_permission(user_id, &permission, &context) {
            Ok(())
        } else {
            Err(AppError::Authorization(format!(
                "User {} does not have permission for {}:{}",
                user_id, permission.resource, permission.action
            )))
        }
    }

    /// Create permission check middleware
    pub fn require_permission(
        _permission: Permission,
    ) -> impl Fn(Uuid, PermissionContext) -> AppResult<()> {
        move |_user_id: Uuid, _context: PermissionContext| {
            // This would typically be injected with the service instance
            // For now, this is a placeholder for the middleware pattern
            Ok(())
        }
    }
}

/// Convenience macros for common permissions
pub mod permissions {
    use super::Permission;

    pub fn read_own_projects() -> Permission {
        Permission::new("projects", "read_own")
    }

    pub fn create_projects() -> Permission {
        Permission::new("projects", "create")
    }

    pub fn manage_projects() -> Permission {
        Permission::new("projects", "manage")
    }

    pub fn generate_tokens() -> Permission {
        Permission::new("tokens", "generate")
    }

    pub fn read_audit_logs() -> Permission {
        Permission::new("audit", "read")
    }

    pub fn manage_developers() -> Permission {
        Permission::new("developers", "manage")
    }

    pub fn system_admin() -> Permission {
        Permission::new("system", "manage")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_inheritance() {
        let admin = Role::Admin;
        let permissions = admin.get_permissions();
        
        // Admin should have developer permissions
        assert!(permissions.contains(&Permission::new("projects", "create")));
        // Admin should have read-only permissions
        assert!(permissions.contains(&Permission::new("profile", "read_own")));
    }

    #[test]
    fn test_permission_matching() {
        let permission = Permission::new("projects", "update_own");
        let required = Permission::new("projects", "update_own")
            .with_condition("owner", "self");
        
        let context = PermissionContext::new(
            Uuid::new_v4(),
            "127.0.0.1".to_string(),
        ).with_resource_owner(Uuid::new_v4());
        
        // Should not match because resource owner is different from user
        assert!(!permission.matches(&required, &context));
    }

    #[test]
    fn test_user_roles() {
        let user_id = Uuid::new_v4();
        let user_roles = UserRoles::new(user_id, Role::Developer);
        
        let context = PermissionContext::new(user_id, "127.0.0.1".to_string());
        let permission = Permission::new("projects", "create");
        
        assert!(user_roles.has_permission(&permission, &context));
    }
}