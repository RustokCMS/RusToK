use axum::{
    body::Body,
    extract::State,
    http::{header::HOST, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use loco_rs::app::AppContext;
use uuid::Uuid;

use crate::context::TenantContext;
use crate::models::tenants;

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
    let identifier = host.split(':').next().unwrap_or(host);

    let tenant = if let Ok(uuid) = Uuid::parse_str(identifier) {
        tenants::Entity::find_by_id(&ctx.db, uuid)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else if let Some(tenant) = tenants::Entity::find_by_slug(&ctx.db, identifier)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        Some(tenant)
    } else {
        tenants::Entity::find_by_domain(&ctx.db, identifier)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    match tenant {
        Some(tenant) => {
            req.extensions_mut().insert(TenantContext::new(tenant));
            Ok(next.run(req).await)
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
