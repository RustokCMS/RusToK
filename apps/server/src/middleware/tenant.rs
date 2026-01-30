use axum::{
    body::Body,
    extract::State,
    http::{header::HOST, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use loco_rs::app::AppContext;
use moka::future::Cache;
use once_cell::sync::Lazy;
use std::time::Duration;
use uuid::Uuid;

use crate::context::{TenantContext, TenantContextExtension};
use crate::models::tenants;

// Tenant cache: identifier (host/slug/uuid) -> TenantContext
// TTL: 5 minutes, Max entries: 1000
static TENANT_CACHE: Lazy<Cache<String, TenantContext>> = Lazy::new(|| {
    Cache::builder()
        .time_to_live(Duration::from_secs(300))
        .max_capacity(1_000)
        .build()
});

pub async fn resolve(
    State(ctx): State<AppContext>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let host = req
        .headers()
        .get(HOST)
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;
    let identifier = host.split(':').next().unwrap_or(host).to_string();

    // Check cache first
    if let Some(cached_context) = TENANT_CACHE.get(&identifier).await {
        req.extensions_mut()
            .insert(TenantContextExtension(cached_context));
        return Ok(next.run(req).await);
    }

    // Cache miss — query database
    let tenant = if let Ok(uuid) = Uuid::parse_str(&identifier) {
        tenants::Entity::find_by_id(&ctx.db, uuid)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else if let Some(tenant) = tenants::Entity::find_by_slug(&ctx.db, &identifier)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        Some(tenant)
    } else {
        tenants::Entity::find_by_domain(&ctx.db, &identifier)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    match tenant {
        Some(tenant) => {
            let context = TenantContext::from_model(&tenant);
            // Store in cache
            TENANT_CACHE.insert(identifier, context.clone()).await;
            req.extensions_mut().insert(TenantContextExtension(context));
            Ok(next.run(req).await)
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Invalidate cached tenant (call after tenant update)
pub async fn invalidate_tenant_cache(identifier: &str) {
    TENANT_CACHE.invalidate(identifier).await;
}
