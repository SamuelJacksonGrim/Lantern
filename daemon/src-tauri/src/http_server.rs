use axum::{
    extract::Query,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::memory::MEMORY;

// ── Request types — match sovereign_manifold's SynapseCoordinationClient exactly ──

#[derive(Deserialize)]
pub struct RememberPayload {
    pub source_type: String,
    pub source:      String,
    pub relation:    String,
    pub target:      String,
    pub emotion:     Option<f32>,
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub pattern: String,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_limit() -> i64 { 10 }

// ── Handlers ──────────────────────────────────────────────────────────────────

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

async fn remember(Json(p): Json<RememberPayload>) -> Json<Value> {
    MEMORY.remember(&p.source_type, &p.source, &p.relation, &p.target, p.emotion);
    Json(json!({"ok": true}))
}

/// Queries by source node *type* (n1.type), not source content (n1.content).
/// sovereign_manifold sends pattern="relational_manifold" which is a source_type.
async fn query(Query(params): Query<QueryParams>) -> Json<Vec<String>> {
    Json(MEMORY.query_by_source_type(&params.pattern, params.limit))
}

// ── Server ────────────────────────────────────────────────────────────────────

pub async fn start() {
    let app = Router::new()
        .route("/health",  get(health))
        .route("/remember", post(remember))
        .route("/query",   get(query));

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
