use axum::routing::get;
use loco_rs::prelude::*;

pub async fn health() -> Result<Response> {
    format::json(serde_json::json!({
        "status": "ok",
        "app": "rustok",
    }))
}

pub fn routes() -> Routes {
    Routes::new().prefix("health").add("/", get(health))
}
