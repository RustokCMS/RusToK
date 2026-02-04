use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use loco_rs::prelude::*;

use crate::middleware::tenant::tenant_cache_stats;

pub async fn metrics() -> Result<Response> {
    match rustok_telemetry::metrics_handle() {
        Some(handle) => {
            let mut payload = handle.render();
            if !payload.ends_with('\n') {
                payload.push('\n');
            }
            payload.push_str(&render_tenant_cache_metrics());

            Ok((
                [(header::CONTENT_TYPE, "text/plain; version=0.0.4")],
                payload,
            )
                .into_response())
        }
        None => Ok((StatusCode::SERVICE_UNAVAILABLE, "metrics disabled").into_response()),
    }
}

pub fn routes() -> Routes {
    Routes::new().prefix("metrics").add("/", get(metrics))
}

fn render_tenant_cache_metrics() -> String {
    let stats = tenant_cache_stats();
    format!(
        "rustok_tenant_cache_hits {hits}\n\
rustok_tenant_cache_misses {misses}\n\
rustok_tenant_cache_evictions {evictions}\n\
rustok_tenant_cache_entries {entries}\n\
rustok_tenant_cache_negative_hits {negative_hits}\n\
rustok_tenant_cache_negative_misses {negative_misses}\n\
rustok_tenant_cache_negative_evictions {negative_evictions}\n\
rustok_tenant_cache_negative_entries {negative_entries}\n\
rustok_tenant_cache_negative_inserts {negative_inserts}\n",
        hits = stats.hits,
        misses = stats.misses,
        evictions = stats.evictions,
        entries = stats.entries,
        negative_hits = stats.negative_hits,
        negative_misses = stats.negative_misses,
        negative_evictions = stats.negative_evictions,
        negative_entries = stats.negative_entries,
        negative_inserts = stats.negative_inserts,
    )
}
