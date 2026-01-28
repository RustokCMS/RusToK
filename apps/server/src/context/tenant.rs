use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::_entities::tenants;

/// Tenant context available after TenantMiddleware runs.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TenantContext {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub domain: Option<String>,
    pub settings: serde_json::Value,
    pub is_active: bool,
}

impl TenantContext {
    pub fn from_model(tenant: &tenants::Model) -> Self {
        Self {
            id: tenant.id,
            name: tenant.name.clone(),
            slug: tenant.slug.clone(),
            domain: tenant.domain.clone(),
            settings: tenant.settings.clone(),
            is_active: tenant.is_active,
        }
    }

    pub fn get_setting<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.settings
            .get(key)
            .and_then(|value| serde_json::from_value(value.clone()).ok())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TenantError {
    #[error("Tenant not found")]
    NotFound,
    #[error("Tenant is disabled")]
    Disabled,
    #[error("Missing tenant identifier")]
    MissingIdentifier,
    #[error("Invalid tenant identifier")]
    InvalidIdentifier,
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
}

impl TenantError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Disabled => StatusCode::FORBIDDEN,
            Self::MissingIdentifier | Self::InvalidIdentifier => StatusCode::BAD_REQUEST,
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Clone)]
pub struct TenantContextExt(pub TenantContext);

#[async_trait]
impl<S> FromRequestParts<S> for TenantContext
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<TenantContextExt>()
            .map(|ext| ext.0.clone())
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "TenantContext not found. Did you forget TenantMiddleware?".to_string(),
            ))
    }
}

pub struct OptionalTenant(pub Option<TenantContext>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalTenant
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self(
            parts
                .extensions
                .get::<TenantContextExt>()
                .map(|ext| ext.0.clone()),
        ))
    }
}
