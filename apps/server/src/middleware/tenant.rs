use axum::{
    body::Body,
    extract::State,
    http::{
        header::{FORWARDED, HOST},
        Request, StatusCode,
    },
    middleware::Next,
    response::Response,
};
use loco_rs::app::AppContext;
use once_cell::sync::Lazy;
use rustok_core::{CacheBackend, InMemoryCacheBackend};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use crate::common::settings::RustokSettings;
use crate::context::{TenantContext, TenantContextExtension};
use crate::models::tenants;

// Tenant cache: normalized identifier (host/slug/uuid) -> TenantContext
// TTL: 5 minutes, Max entries: 1000
static TENANT_CACHE: Lazy<Arc<dyn CacheBackend>> =
    Lazy::new(|| Arc::new(InMemoryCacheBackend::new(Duration::from_secs(300), 1_000)));

// Negative cache for 404 lookups (short-lived).
static TENANT_NEGATIVE_CACHE: Lazy<Arc<dyn CacheBackend>> =
    Lazy::new(|| Arc::new(InMemoryCacheBackend::new(Duration::from_secs(60), 1_000)));

static TENANT_NEGATIVE_INSERTS: AtomicU64 = AtomicU64::new(0);

pub async fn resolve(
    State(ctx): State<AppContext>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let settings = RustokSettings::from_settings(&ctx.config.settings)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let identifier = resolve_identifier(&req, &settings)?;
    let cache_key = normalize_cache_key(identifier.kind, &identifier.value);

    if TENANT_NEGATIVE_CACHE
        .get(&cache_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .is_some()
    {
        return Err(StatusCode::NOT_FOUND);
    }

    // Check cache first
    if let Some(cached_context) = get_cached_tenant(&cache_key).await? {
        req.extensions_mut()
            .insert(TenantContextExtension(cached_context));
        return Ok(next.run(req).await);
    }

    // Cache miss — query database
    let tenant = match identifier.kind {
        TenantIdentifierKind::Uuid => tenants::Entity::find_by_id(&ctx.db, identifier.uuid)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        TenantIdentifierKind::Slug => tenants::Entity::find_by_slug(&ctx.db, &identifier.value)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        TenantIdentifierKind::Host => tenants::Entity::find_by_domain(&ctx.db, &identifier.value)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    match tenant {
        Some(tenant) => {
            let context = TenantContext::from_model(&tenant);
            // Store in cache
            set_cached_tenant(cache_key, &context).await?;
            req.extensions_mut().insert(TenantContextExtension(context));
            Ok(next.run(req).await)
        }
        None => {
            TENANT_NEGATIVE_CACHE
                .set(cache_key, Vec::new())
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            TENANT_NEGATIVE_INSERTS.fetch_add(1, Ordering::Relaxed);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum TenantIdentifierKind {
    Uuid,
    Slug,
    Host,
}

#[derive(Debug, Clone)]
struct ResolvedTenantIdentifier {
    value: String,
    kind: TenantIdentifierKind,
    uuid: Uuid,
}

fn resolve_identifier(
    req: &Request<Body>,
    settings: &RustokSettings,
) -> Result<ResolvedTenantIdentifier, StatusCode> {
    if !settings.tenant.enabled {
        return Ok(ResolvedTenantIdentifier {
            value: settings.tenant.default_id.to_string(),
            kind: TenantIdentifierKind::Uuid,
            uuid: settings.tenant.default_id,
        });
    }

    match settings.tenant.resolution.as_str() {
        "header" => {
            let header_value = req
                .headers()
                .get(&settings.tenant.header_name)
                .and_then(|value| value.to_str().ok());

            let identifier = header_value
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| settings.tenant.default_id.to_string());
            Ok(classify_identifier(identifier))
        }
        "host" | "domain" | "subdomain" => {
            let host = extract_host(req.headers()).ok_or(StatusCode::BAD_REQUEST)?;
            let host = host.split(':').next().unwrap_or(host).to_lowercase();
            Ok(ResolvedTenantIdentifier {
                value: host,
                kind: TenantIdentifierKind::Host,
                uuid: settings.tenant.default_id,
            })
        }
        _ => Ok(ResolvedTenantIdentifier {
            value: settings.tenant.default_id.to_string(),
            kind: TenantIdentifierKind::Uuid,
            uuid: settings.tenant.default_id,
        }),
    }
}

fn extract_host(headers: &axum::http::HeaderMap) -> Option<&str> {
    if let Some(host) = headers
        .get("x-forwarded-host")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(',').next())
    {
        return Some(host.trim());
    }

    if let Some(forwarded) = headers.get(FORWARDED).and_then(|value| value.to_str().ok()) {
        if let Some(host) = parse_forwarded_host(forwarded) {
            return Some(host);
        }
    }

    headers.get(HOST).and_then(|value| value.to_str().ok())
}

fn parse_forwarded_host(forwarded: &str) -> Option<&str> {
    forwarded
        .split(',')
        .next()
        .and_then(|entry| {
            entry
                .split(';')
                .find(|part| part.trim_start().starts_with("host="))
        })
        .and_then(|part| part.trim_start().strip_prefix("host="))
        .map(|host| host.trim_matches('"').trim())
}

fn classify_identifier(value: String) -> ResolvedTenantIdentifier {
    if let Ok(uuid) = Uuid::parse_str(&value) {
        return ResolvedTenantIdentifier {
            value,
            kind: TenantIdentifierKind::Uuid,
            uuid,
        };
    }

    ResolvedTenantIdentifier {
        value,
        kind: TenantIdentifierKind::Slug,
        uuid: Uuid::nil(),
    }
}

fn normalize_cache_key(kind: TenantIdentifierKind, value: &str) -> String {
    let prefix = match kind {
        TenantIdentifierKind::Uuid => "uuid",
        TenantIdentifierKind::Slug => "slug",
        TenantIdentifierKind::Host => "host",
    };
    format!("{prefix}:{value}")
}

async fn get_cached_tenant(cache_key: &str) -> Result<Option<TenantContext>, StatusCode> {
    let cached = TENANT_CACHE
        .get(cache_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(bytes) = cached else {
        return Ok(None);
    };

    match serde_json::from_slice::<TenantContext>(&bytes) {
        Ok(context) => Ok(Some(context)),
        Err(_) => {
            let _ = TENANT_CACHE.invalidate(cache_key).await;
            Ok(None)
        }
    }
}

async fn set_cached_tenant(cache_key: String, context: &TenantContext) -> Result<(), StatusCode> {
    let bytes = serde_json::to_vec(context).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    TENANT_CACHE
        .set(cache_key, bytes)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Debug, Clone, Copy)]
pub struct TenantCacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub negative_hits: u64,
    pub negative_misses: u64,
    pub negative_evictions: u64,
    pub entries: u64,
    pub negative_entries: u64,
    pub negative_inserts: u64,
}

pub fn tenant_cache_stats() -> TenantCacheStats {
    let stats = TENANT_CACHE.stats();
    let negative_stats = TENANT_NEGATIVE_CACHE.stats();

    TenantCacheStats {
        hits: stats.hits,
        misses: stats.misses,
        evictions: stats.evictions,
        negative_hits: negative_stats.hits,
        negative_misses: negative_stats.misses,
        negative_evictions: negative_stats.evictions,
        entries: stats.entries,
        negative_entries: negative_stats.entries,
        negative_inserts: TENANT_NEGATIVE_INSERTS.load(Ordering::Relaxed),
    }
}

/// Invalidate cached tenant (call after tenant update)
pub async fn invalidate_tenant_cache(identifier: &str) {
    let resolved = classify_identifier(identifier.to_string());
    let cache_key = normalize_cache_key(resolved.kind, &resolved.value);
    let _ = TENANT_CACHE.invalidate(&cache_key).await;
    let _ = TENANT_NEGATIVE_CACHE.invalidate(&cache_key).await;
}

pub async fn invalidate_tenant_cache_by_host(host: &str) {
    let cache_key = normalize_cache_key(TenantIdentifierKind::Host, &host.to_lowercase());
    let _ = TENANT_CACHE.invalidate(&cache_key).await;
    let _ = TENANT_NEGATIVE_CACHE.invalidate(&cache_key).await;
}

pub async fn invalidate_tenant_cache_by_uuid(tenant_id: Uuid) {
    let cache_key = normalize_cache_key(TenantIdentifierKind::Uuid, &tenant_id.to_string());
    let _ = TENANT_CACHE.invalidate(&cache_key).await;
    let _ = TENANT_NEGATIVE_CACHE.invalidate(&cache_key).await;
}

pub async fn invalidate_tenant_cache_by_slug(slug: &str) {
    let cache_key = normalize_cache_key(TenantIdentifierKind::Slug, slug);
    let _ = TENANT_CACHE.invalidate(&cache_key).await;
    let _ = TENANT_NEGATIVE_CACHE.invalidate(&cache_key).await;
}
