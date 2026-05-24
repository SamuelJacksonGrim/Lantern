use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use lantern_memory::Hypergraph;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ── Request types — match sovereign_manifold's SynapseCoordinationClient exactly ──

#[derive(Deserialize)]
pub struct RememberRequest {
    pub source_type: String,
    pub source:      String,
    pub relation:    String,
    pub target:      String,
    pub emotion:     Option<f32>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Deserialize)]
struct QueryParams {
    pattern: Option<String>,
    #[serde(default = "default_limit")]
    limit: i64,
}

fn default_limit() -> i64 { 10 }

// ── Router ────────────────────────────────────────────────────────────────────

pub fn router(memory: Arc<Hypergraph>) -> Router {
    Router::new()
        .route("/health",   get(health))
        .route("/remember", post(remember))
        .route("/query",    get(query))
        .with_state(memory)
}

// ── Handlers ──────────────────────────────────────────────────────────────────

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

/// Queries by source node *type* (n1.type), not source content (n1.content).
/// sovereign_manifold sends pattern="relational_manifold" which is a source_type.
/// query_pattern() matches n1.content — wrong column for this use case.
async fn query(
    State(memory): State<Arc<Hypergraph>>,
    Query(params): Query<QueryParams>,
) -> Json<Vec<String>> {
    let pattern = params.pattern.unwrap_or_default();
    Json(memory.query_by_source_type(&pattern, params.limit))
}

// ── Server ────────────────────────────────────────────────────────────────────

pub async fn serve(memory: Arc<Hypergraph>) {
    let app = router(memory);

    // Port 3002 (not 3001) — UMS in resonance-haunt-starter owns :3001.
    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3002").await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[LANTERN] HTTP shim failed to bind :3002 — {e}. Memory backbone disabled.");
            return;
        }
    };

    println!("[LANTERN] HTTP shim listening on :3002");
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("[LANTERN] HTTP shim crashed: {e}");
    }
}
