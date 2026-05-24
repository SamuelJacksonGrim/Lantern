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
}

// ── Handlers ──────────────────────────────────────────────────────────────────

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

async fn remember(Json(p): Json<RememberPayload>) -> Json<Value> {
    MEMORY.remember(&p.source_type, &p.source, &p.relation, &p.target, p.emotion);
    Json(json!({"ok": true}))
}

async fn query(Query(params): Query<QueryParams>) -> Json<Vec<String>> {
    Json(MEMORY.query_pattern(&params.pattern))
}

// ── Server ────────────────────────────────────────────────────────────────────

pub async fn start() {
    let app = Router::new()
        .route("/health",  get(health))
        .route("/remember", post(remember))
        .route("/query",   get(query));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002")
        .await
        .expect("[LANTERN] Failed to bind HTTP shim on :3002");

    println!("[LANTERN] HTTP shim listening on :3002");
    axum::serve(listener, app)
        .await
        .expect("[LANTERN] HTTP shim crashed");
}
