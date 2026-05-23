# CLAUDE.md — lantern

## What Lantern actually is (architecture, not marketing)

Lantern is a memory persistence daemon built in Rust + Tauri. It provides a hypergraph memory store with temporal edge weighting, accessible via Tauri IPC commands from the desktop frontend. It is NOT a REST service by default — the existing HTTP surface is planned but not yet implemented.

## Tauri IPC commands (what exists today)

The daemon exposes these commands:
- `remember` — store a relational event (`source_type`, `source`, `relation`, `target`, `emotion`)
- `remember_code` — store a code snippet with metadata
- `find_similar` — similarity search in the hypergraph
- `get_memory` — retrieve a specific memory node
- `query_pattern` — ordered-by-weight pattern query (in `memory/memory/src/lib.rs`)

These are Tauri IPC commands callable from the Tauri frontend. They are NOT HTTP endpoints.

## HTTP shim — planned, not built

`sovereign_manifold.py`'s `SynapseCoordinationClient` targets `http://localhost:3001`. This works with `lantern_mock.py` (the Python FastAPI mock in sovereign_manifold). The real Lantern daemon does not expose port 3001 yet.

The Phase 2 plan is to add an Axum HTTP server inside `daemon/src-tauri/src/http_server.rs` that wraps the existing Tauri commands:
- `GET /health` → `{"status": "ok"}`
- `POST /remember` → delegates to `remember` Tauri command
- `GET /query?pattern=&limit=N` → delegates to `query_pattern()`

Spawn the HTTP server from `main.rs` via `tokio::spawn` alongside the existing Tauri setup on port 3001.

## lantern_mock.py is the production integration shim

`sovereign_manifold/lantern_mock.py` is the production integration point for the current stack, not a test fixture. It implements the three HTTP routes above using FastAPI with an in-memory list + threading.Lock.

Memory is in-memory only — it resets on restart. Lantern nodes are not persisted across container restarts. The Witness file (`witness_state.json` in sovereign_manifold) is the durable state substrate; Lantern is a warm-start fallback.

## Memory edge types

The real hypergraph uses typed edges with decaying weights (edge type: `REPEATEDUSE`, `HATED`, `SOUNDTRACK`, etc. per the README). The mock uses a flat list. When implementing the HTTP shim in Rust, preserve the `source_type` field — sovereign_manifold uses `source_type=relational_manifold` to scope its hydration queries.

## Do not start Tauri expecting an HTTP port

`cargo tauri dev` starts the desktop application, not a headless daemon with a port. To run Lantern headlessly for integration testing, use either the Python mock (`lantern_mock.py`) or the planned HTTP shim (once built).

## README.md — marketing prose, not architecture spec

The existing README.md is marketing copy. It describes a vision, not the current implementation state. The `70B MoE` model, biometric login, encrypted P2P sync, and `<1.1s` generation times are product goals, not shipped features. When editing the README, preserve the vision but do not imply these features are live.
