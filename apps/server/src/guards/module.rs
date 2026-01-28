use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::marker::PhantomData;

use crate::context::TenantContextExt;
use crate::models::_entities::tenant_modules::{self, Entity as TenantModules};
use loco_rs::app::AppContext;

pub trait ModuleSlug {
    const SLUG: &'static str;
}

pub struct RequireModule<M: ModuleSlug>(PhantomData<M>);

#[async_trait]
impl<S, M: ModuleSlug> FromRequestParts<S> for RequireModule<M>
where
    S: Send + Sync,
    AppContext: FromRef<S>,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let tenant_id = parts
            .tenant_context()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Tenant context missing"))?
            .id;
        let ctx = AppContext::from_ref(state);

        let is_enabled = TenantModules::find()
            .filter(tenant_modules::Column::TenantId.eq(tenant_id))
            .filter(tenant_modules::Column::ModuleSlug.eq(M::SLUG))
            .filter(tenant_modules::Column::Enabled.eq(true))
            .one(&ctx.db)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
            .is_some();

        if is_enabled {
            Ok(Self(PhantomData))
        } else {
            Err((StatusCode::NOT_FOUND, "Module is disabled or not found"))
        }
    }
}
