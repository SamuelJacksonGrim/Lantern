use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use lantern_memory::Hypergraph;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct RememberRequest {
    pub source_type: String,
    pub source: String,
    pub relation: String,
    pub target: String,
    pub emotion: Option<f32>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Deserialize)]
struct QueryParams {
    pattern: Option<String>,
}

pub fn router(memory: Arc<Hypergraph>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/remember", post(remember))
        .route("/query", get(query))
        .with_state(memory)
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn remember(
    State(memory): State<Arc<Hypergraph>>,
    Json(req): Json<RememberRequest>,
) -> StatusCode {
    memory.remember(
        &req.source_type,
        &req.source,
        &req.relation,
        &req.target,
        req.emotion,
    );
    StatusCode::OK
}

async fn query(
    State(memory): State<Arc<Hypergraph>>,
    Query(params): Query<QueryParams>,
) -> Json<Vec<String>> {
    let pattern = params.pattern.unwrap_or_default();
    Json(memory.query_pattern(&pattern))
}

pub async fn serve(memory: Arc<Hypergraph>) {
    let app = router(memory);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("Lantern HTTP server failed to bind :3001");
    axum::serve(listener, app)
        .await
        .expect("Lantern HTTP server exited unexpectedly");
}
