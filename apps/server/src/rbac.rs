use loco_rs::app::AppContext;
use rustok_core::{PermissionSet, RolePermissions, UserRole};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct AppSettings {
    #[serde(default)]
    rbac: Option<RbacSettings>,
}

#[derive(Debug, Deserialize)]
struct RbacSettings {
    #[serde(default)]
    roles: HashMap<String, Vec<String>>,
}

pub fn role_permissions_from_ctx(ctx: &AppContext) -> RolePermissions {
    let mut permissions = RolePermissions::default();
    let settings = ctx
        .config
        .settings
        .clone()
        .and_then(|value| serde_json::from_value::<AppSettings>(value).ok())
        .and_then(|settings| settings.rbac);

    let Some(settings) = settings else {
        return permissions;
    };

    for (role, role_permissions) in settings.roles {
        match role.parse::<UserRole>() {
            Ok(role) => {
                permissions.insert_role_permissions(role, PermissionSet::new(role_permissions));
            }
            Err(err) => {
                tracing::warn!(role = %role, error = %err, "Skipping invalid RBAC role");
            }
        }
    }

    permissions
}
