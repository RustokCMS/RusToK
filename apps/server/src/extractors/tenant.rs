use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use crate::context::TenantContext;
use crate::context::TenantContextExt;

pub struct CurrentTenant(pub TenantContext);

#[async_trait]
impl<S> FromRequestParts<S> for CurrentTenant
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let tenant = parts
            .tenant_context()
            .cloned()
            .ok_or(StatusCode::NOT_FOUND)?;

        Ok(Self(tenant))
    }
}
