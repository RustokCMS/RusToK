use async_graphql::{Context, EmptySubscription, MergedObject, Object, Schema, SimpleObject};
use loco_rs::prelude::AppContext;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::context::TenantContext;
use crate::extractors::auth::CurrentUser;
use crate::models::{tenant_modules, users};

#[derive(SimpleObject, Clone)]
pub struct Tenant {
    pub id: uuid::Uuid,
    pub name: String,
    pub slug: String,
    pub domain: Option<String>,
    pub settings: String,
    pub is_active: bool,
}

impl From<&TenantContext> for Tenant {
    fn from(tenant: &TenantContext) -> Self {
        Self {
            id: tenant.id,
            name: tenant.name.clone(),
            slug: tenant.slug.clone(),
            domain: tenant.domain.clone(),
            settings: tenant.settings.to_string(),
            is_active: tenant.is_active,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
    pub status: String,
}

impl From<&users::Model> for User {
    fn from(user: &users::Model) -> Self {
        Self {
            id: user.id,
            email: user.email.clone(),
            name: user.name.clone(),
            role: user.role.to_string(),
            status: user.status.to_string(),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct TenantModule {
    pub module_slug: String,
    pub enabled: bool,
    pub settings: String,
}

impl From<tenant_modules::Model> for TenantModule {
    fn from(module: tenant_modules::Model) -> Self {
        Self {
            module_slug: module.module_slug,
            enabled: module.enabled,
            settings: module.settings.to_string(),
        }
    }
}

#[derive(Default)]
pub struct CoreQuery;

#[Object]
impl CoreQuery {
    async fn health(&self) -> &str {
        "GraphQL is working!"
    }

    async fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    async fn tenant(&self, ctx: &Context<'_>) -> async_graphql::Result<Tenant> {
        let tenant = ctx.data::<TenantContext>()?;
        Ok(Tenant::from(tenant))
    }

    async fn enabled_modules(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<String>> {
        let app_ctx = ctx.data::<AppContext>()?;
        let tenant = ctx.data::<TenantContext>()?;
        let modules =
            tenant_modules::Entity::find_enabled(&app_ctx.db, tenant.id)
                .await
                .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(modules)
    }

    async fn tenant_modules(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<TenantModule>> {
        let app_ctx = ctx.data::<AppContext>()?;
        let tenant = ctx.data::<TenantContext>()?;
        let modules = tenant_modules::Entity::find()
            .filter(tenant_modules::Column::TenantId.eq(tenant.id))
            .all(&app_ctx.db)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(modules.into_iter().map(TenantModule::from).collect())
    }

    async fn me(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<User>> {
        let current_user = ctx.data::<Option<CurrentUser>>()?;
        Ok(current_user.as_ref().map(|current_user| User::from(&current_user.user)))
    }
}

#[derive(Default)]
pub struct CoreMutation;

#[Object]
impl CoreMutation {
    async fn toggle_module(
        &self,
        ctx: &Context<'_>,
        module_slug: String,
        enabled: bool,
    ) -> async_graphql::Result<TenantModule> {
        let current_user = ctx.data::<Option<CurrentUser>>()?;
        let current_user = current_user
            .as_ref()
            .ok_or_else(|| async_graphql::Error::new("Unauthorized"))?;

        if !current_user.user.is_admin() {
            return Err(async_graphql::Error::new("Forbidden"));
        }

        let app_ctx = ctx.data::<AppContext>()?;
        let tenant = ctx.data::<TenantContext>()?;
        let module = tenant_modules::Entity::toggle(
            &app_ctx.db,
            tenant.id,
            &module_slug,
            enabled,
        )
        .await
        .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        Ok(TenantModule::from(module))
    }
}

#[derive(MergedObject, Default)]
pub struct Query(CoreQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(CoreMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema() -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish()
}
