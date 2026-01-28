use crate::types::UserRole;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Default)]
pub struct PermissionSet {
    permissions: HashSet<String>,
}

impl PermissionSet {
    pub fn new<I, S>(permissions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            permissions: permissions.into_iter().map(Into::into).collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.permissions.is_empty()
    }

    pub fn allows(&self, permission: &str) -> bool {
        if self.permissions.contains("*") {
            return true;
        }

        if self.permissions.contains(permission) {
            return true;
        }

        self.permissions.iter().any(|entry| {
            entry
                .strip_suffix(".*")
                .is_some_and(|prefix| permission.starts_with(prefix) && permission.len() > prefix.len())
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct RolePermissions {
    permissions: HashMap<UserRole, PermissionSet>,
}

impl RolePermissions {
    pub fn new() -> Self {
        Self {
            permissions: HashMap::new(),
        }
    }

    pub fn insert_role_permissions(&mut self, role: UserRole, permissions: PermissionSet) {
        self.permissions.insert(role, permissions);
    }

    pub fn permissions_for(&self, role: &UserRole) -> Option<&PermissionSet> {
        self.permissions.get(role)
    }

    pub fn allows(&self, role: &UserRole, permission: &str) -> bool {
        if matches!(role, UserRole::SuperAdmin) {
            return true;
        }

        self.permissions
            .get(role)
            .map(|permissions| permissions.allows(permission))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_set_allows_exact_and_wildcards() {
        let set = PermissionSet::new(["products.read", "orders.*", "*"]);
        assert!(set.allows("products.read"));
        assert!(set.allows("orders.write"));
        assert!(set.allows("anything"));
    }

    #[test]
    fn role_permissions_allows_super_admin() {
        let permissions = RolePermissions::default();
        assert!(permissions.allows(&UserRole::SuperAdmin, "anything"));
        assert!(!permissions.allows(&UserRole::Admin, "anything"));
    }
}
