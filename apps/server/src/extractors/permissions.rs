use crate::rbac::role_permissions_from_ctx;
use crate::models::users;
use loco_rs::{app::AppContext, Error, Result};

pub fn ensure_permission(
    ctx: &AppContext,
    user: &users::Model,
    permission: &str,
) -> Result<()> {
    let role_permissions = role_permissions_from_ctx(ctx);
    if role_permissions.allows(&user.role, permission) {
        Ok(())
    } else {
        Err(Error::Forbidden("Missing required permission".to_string()))
    }
}
