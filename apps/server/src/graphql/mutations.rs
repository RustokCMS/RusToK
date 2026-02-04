use async_graphql::{Context, FieldError, Object, Result};
use rustok_core::auth::password::hash_password;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::collections::HashSet;

use crate::context::{AuthContext, TenantContext};
use crate::graphql::errors::GraphQLError;
use crate::graphql::types::{CreateUserInput, TenantModule, UpdateUserInput, User};
use crate::models::_entities::tenant_modules::Entity as TenantModulesEntity;
use crate::models::_entities::users::Column as UsersColumn;
use crate::models::users;
use rustok_core::{ModuleContext, ModuleRegistry};

#[derive(Default)]
pub struct RootMutation;

#[Object]
impl RootMutation {
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let tenant = ctx.data::<TenantContext>()?;
        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;

        if !rustok_core::Rbac::has_any_permission(
            &auth.role,
            &[
                rustok_core::Permission::USERS_CREATE,
                rustok_core::Permission::USERS_MANAGE,
            ],
        ) {
            return Err(<FieldError as GraphQLError>::permission_denied(
                "Permission denied: users:create required",
            ));
        }

        let existing = users::Entity::find_by_email(&app_ctx.db, tenant.id, &input.email)
            .await
            .map_err(|err| <FieldError as GraphQLError>::internal_error(&err.to_string()))?;

        if existing.is_some() {
            return Err(FieldError::new("User with this email already exists"));
        }

        let password_hash = hash_password(&input.password)
            .map_err(|err| <FieldError as GraphQLError>::internal_error(&err.to_string()))?;

        let mut model = users::ActiveModel::new(tenant.id, &input.email, &password_hash);

        if let Some(name) = input.name {
            model.name = Set(Some(name));
        }

        if let Some(role) = input.role {
            let role: rustok_core::UserRole = role.into();
            model.role = Set(role);
        }

        if let Some(status) = input.status {
            let status: rustok_core::UserStatus = status.into();
            model.status = Set(status);
        }

        let user = model
            .insert(&app_ctx.db)
            .await
            .map_err(|err| <FieldError as GraphQLError>::internal_error(&err.to_string()))?;

        Ok(User::from(&user))
    }

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        id: uuid::Uuid,
        input: UpdateUserInput,
    ) -> Result<User> {
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let tenant = ctx.data::<TenantContext>()?;
        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;

        if !rustok_core::Rbac::has_any_permission(
            &auth.role,
            &[
                rustok_core::Permission::USERS_UPDATE,
                rustok_core::Permission::USERS_MANAGE,
            ],
        ) {
            return Err(<FieldError as GraphQLError>::permission_denied(
                "Permission denied: users:update required",
            ));
        }

        let user = users::Entity::find_by_id(id)
            .filter(UsersColumn::TenantId.eq(tenant.id))
            .one(&app_ctx.db)
            .await
            .map_err(|err| <FieldError as GraphQLError>::internal_error(&err.to_string()))?
            .ok_or_else(|| FieldError::new("User not found"))?;

        if let Some(email) = &input.email {
            let existing = users::Entity::find_by_email(&app_ctx.db, tenant.id, email)
                .await
                .map_err(|err| <FieldError as GraphQLError>::internal_error(&err.to_string()))?;

            if existing
                .as_ref()
                .is_some_and(|existing| existing.id != user.id)
            {
                return Err(FieldError::new("User with this email already exists"));
            }
        }

        let mut model: users::ActiveModel = user.into();

        if let Some(email) = input.email {
            model.email = Set(email.to_lowercase());
        }

        if let Some(name) = input.name {
            model.name = Set(Some(name));
        }

        if let Some(role) = input.role {
            let role: rustok_core::UserRole = role.into();
            model.role = Set(role);
        }

        if let Some(status) = input.status {
            let status: rustok_core::UserStatus = status.into();
            model.status = Set(status);
        }

        if let Some(password) = input.password {
            let password_hash = hash_password(&password)
                .map_err(|err| <FieldError as GraphQLError>::internal_error(&err.to_string()))?;
            model.password_hash = Set(password_hash);
        }

        let user = model
            .update(&app_ctx.db)
            .await
            .map_err(|err| <FieldError as GraphQLError>::internal_error(&err.to_string()))?;

        Ok(User::from(&user))
    }

    async fn disable_user(&self, ctx: &Context<'_>, id: uuid::Uuid) -> Result<User> {
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;
        let tenant = ctx.data::<TenantContext>()?;
        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;

        if !rustok_core::Rbac::has_permission(&auth.role, &rustok_core::Permission::USERS_MANAGE) {
            return Err(<FieldError as GraphQLError>::permission_denied(
                "Permission denied: users:manage required",
            ));
        }

        let user = users::Entity::find_by_id(id)
            .filter(UsersColumn::TenantId.eq(tenant.id))
            .one(&app_ctx.db)
            .await
            .map_err(|err| <FieldError as GraphQLError>::internal_error(&err.to_string()))?
            .ok_or_else(|| FieldError::new("User not found"))?;

        let mut model: users::ActiveModel = user.into();
        model.status = Set(rustok_core::UserStatus::Inactive);

        let user = model
            .update(&app_ctx.db)
            .await
            .map_err(|err| <FieldError as GraphQLError>::internal_error(&err.to_string()))?;

        Ok(User::from(&user))
    }

    async fn toggle_module(
        &self,
        ctx: &Context<'_>,
        module_slug: String,
        enabled: bool,
    ) -> Result<TenantModule> {
        let auth = ctx
            .data::<AuthContext>()
            .map_err(|_| <FieldError as GraphQLError>::unauthenticated())?;

        if !matches!(
            auth.role,
            rustok_core::UserRole::Admin | rustok_core::UserRole::SuperAdmin
        ) {
            return Err(<FieldError as GraphQLError>::permission_denied("Forbidden"));
        }

        let app_ctx = ctx.data::<loco_rs::prelude::AppContext>()?;
        let tenant = ctx.data::<TenantContext>()?;
        let registry = ctx.data::<ModuleRegistry>()?;

        let Some(module_impl) = registry.get(&module_slug) else {
            return Err(FieldError::new("Unknown module"));
        };

        let enabled_modules = TenantModulesEntity::find_enabled(&app_ctx.db, tenant.id)
            .await
            .map_err(|err| <FieldError as GraphQLError>::internal_error(&err.to_string()))?;
        let enabled_set: HashSet<String> = enabled_modules.into_iter().collect();

        if enabled {
            let missing: Vec<String> = module_impl
                .dependencies()
                .iter()
                .filter(|dependency| !enabled_set.contains(**dependency))
                .map(|dependency| (*dependency).to_string())
                .collect();

            if !missing.is_empty() {
                return Err(FieldError::new(format!(
                    "Missing module dependencies: {}",
                    missing.join(", ")
                )));
            }
        } else {
            let dependents: Vec<String> = registry
                .list()
                .into_iter()
                .filter(|module| enabled_set.contains(module.slug()))
                .filter(|module| module.dependencies().iter().any(|dep| *dep == module_slug))
                .map(|module| module.slug().to_string())
                .collect();

            if !dependents.is_empty() {
                return Err(FieldError::new(format!(
                    "Module is required by: {}",
                    dependents.join(", ")
                )));
            }
        }
        let module = TenantModulesEntity::toggle(&app_ctx.db, tenant.id, &module_slug, enabled)
            .await
            .map_err(|err| <FieldError as GraphQLError>::internal_error(&err.to_string()))?;

        let module_ctx = ModuleContext {
            db: &app_ctx.db,
            tenant_id: tenant.id,
            config: &module.settings,
        };

        let hook_result = if enabled {
            module_impl.on_enable(module_ctx).await
        } else {
            module_impl.on_disable(module_ctx).await
        };

        if let Err(err) = hook_result {
            tracing::error!(
                "Module hook failed for {} (enabled={}): {}",
                module_slug,
                enabled,
                err
            );
        }

        Ok(TenantModule {
            module_slug: module.module_slug,
            enabled: module.enabled,
            settings: module.settings.to_string(),
        })
    }
}
