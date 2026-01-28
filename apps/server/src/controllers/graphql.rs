use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use axum::{extract::State, routing::get, Extension, Json};
use loco_rs::prelude::*;

use crate::context::TenantContext;

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn health(&self) -> &str {
        "GraphQL is working!"
    }

    async fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
}

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

fn build_schema() -> AppSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription).finish()
}

async fn graphql_handler(
    State(ctx): State<AppContext>,
    Extension(schema): Extension<AppSchema>,
    tenant_ctx: TenantContext,
    Json(req): Json<async_graphql::Request>,
) -> Json<async_graphql::Response> {
    let request = req.data(ctx).data(tenant_ctx);
    Json(schema.execute(request).await)
}

async fn graphql_playground() -> impl axum::response::IntoResponse {
    axum::response::Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/api/graphql"),
    ))
}

pub fn routes() -> Routes {
    let schema = build_schema();

    Routes::new()
        .prefix("api/graphql")
        .add("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema))
}
