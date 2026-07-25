# CLAUDE.md — lantern

> **Cross-repo plan.** Lantern is one layer of the Resonance memory stack — the Phase-2 hypergraph substrate that will back Simple Memory behind an unchanged MCP API. The portable work-order (roadmap + per-repo backlog) lives at https://github.com/SamuelJacksonGrim/resonance-memory-stack — read it before doing memory work here.

## What Lantern actually is (architecture, not marketing)

Lantern is a memory persistence daemon built in Rust + Tauri. It provides a SQLite-backed hypergraph memory store with temporal edge weighting, accessible via **two interfaces** that share a single `Arc<Hypergraph>` instance:

1. **Tauri IPC commands** — called from the desktop frontend
2. **Axum HTTP shim on `:3002`** — called from `sovereign_manifold` and other Python services in the Resonance Family stack

Both interfaces write to and read from the same in-process Hypergraph, so a write from either path is immediately visible to the other.

## Why port 3002 (not 3001)

The UMS in `resonance-haunt-starter` owns `:3001`. The Lantern HTTP shim moved to `:3002` during T2.1 to avoid the collision. `sovereign_manifold`'s `SynapseCoordinationClient` and `lantern_mock.py` both target `:3002`. Do not change this without coordinating across all three repos and the global CLAUDE.md.

## HTTP shim — built (T2.1)

`daemon/src-tauri/src/http_server.rs` exposes three routes, all backed by `Arc<Hypergraph>` injected via axum `State`:

| Route | Body / Query | Behavior |
|-------|--------------|----------|
| `GET /health` | — | Returns `{"status": "ok"}` |
| `POST /remember` | `{source_type, source, relation, target, emotion}` | Calls `Hypergraph.remember(...)`, returns 200 |
| `GET /query?pattern=X&limit=N` | `pattern` (source_type), `limit` (default 10) | Calls `Hypergraph.query_by_source_type(pattern, limit)`, returns `Vec<String>` |

The shim runs in a dedicated `std::thread` with its own multi-threaded Tokio runtime — isolated from Tauri's event loop. If the runtime fails to build or the bind fails, the thread logs to stderr and exits without taking down the Tauri app.

## `query_pattern` vs `query_by_source_type` — the n1.content vs n1.type distinction

`Hypergraph` exposes two query methods that look similar but match different columns:

| Method | SQL filter | Use case |
|--------|------------|----------|
| `query_pattern(pattern)` | `n1.content LIKE '%pattern%'` | Free-text search on source node content. Wired to the `find_similar` Tauri command. |
| `query_by_source_type(source_type, limit)` | `n1.type = ?` (parameterized) | Exact match on source node type. Wired to the HTTP `/query` endpoint because `sovereign_manifold` sends `pattern="relational_manifold"` which is a source_type, not source content. |

Using the wrong one returns `[]` silently. If you add a new query path, pick the right one based on whether the caller is passing a type or a content fragment.

## Tauri IPC commands

The daemon exposes these commands. All `memory::*` commands take `State<Arc<Hypergraph>>` as their first parameter — the shared Hypergraph is registered via `tauri::manage(memory)` in `main.rs`.

| Command | Signature | Behavior |
|---------|-----------|----------|
| `get_memory` | `() -> String` | Returns flame greeting + memory count |
| `remember` | `(what: String)` | Pushes string into in-process `Flame.memories` (NOT the hypergraph) |
| `remember_code` | `(memory: State<Arc<Hypergraph>>, what: String, emotion: Option<f32>)` | Calls `Hypergraph.remember("user", "samuel", "wrote", what, emotion)` |
| `find_similar` | `(memory: State<Arc<Hypergraph>>, pattern: String) -> Vec<String>` | Calls `Hypergraph.query_pattern(pattern)` |

Note: the top-level `remember` command writes to `Flame.memories` (a `Vec<String>` for session greetings), not the hypergraph. Use `remember_code` for hypergraph writes.

## `lantern_mock.py` is the cloud-stack integration shim

`stack/lantern_mock.py` is a FastAPI server on `:3002` that implements the same three HTTP routes as the real daemon, using an in-memory list + `threading.Lock`. It mirrors the real daemon's interface so the integration tests and `sovereign_manifold` can run on machines or in cloud sessions where the Tauri daemon isn't installed.

Both the mock and the real daemon hold memory in-process — they reset on restart. `witness_state.json` in `sovereign_manifold` is the durable state substrate; Lantern is a warm-start fallback. If `witness_state.json` exists with `cycle > 0`, Lantern hydration is skipped.

## Memory edge types

The hypergraph uses typed edges with weight accumulation. `remember(source_type, source_content, relation, target_content, emotion)` upserts both nodes and the edge, with `weight += 0.3` on each call (via `INSERT OR REPLACE` + `COALESCE` arithmetic in `lib.rs::remember`). `query_by_source_type` returns target content ordered by edge weight DESC.

Edge weight encodes frequency of association — more frequent connections grow stronger. `query_pattern` and `query_by_source_type` both honor this ordering.

When implementing new query paths, preserve the `source_type` field. `sovereign_manifold` uses `source_type=relational_manifold` to scope its hydration queries; flattening this would mix relational state with code memories.

## Storage is `:memory:` — file-backed is stubbed but not wired

`Hypergraph::ignite()` calls `sqlite::open(":memory:")`. The intent is file-backed SQLite in production, but the path is not yet plumbed. Daemon restarts lose all memory unless `sovereign_manifold` re-writes from its own state.

## Known bugs (pre-existing, flagged not fixed)

| Bug | Location | Status |
|-----|----------|--------|
| **Bug 3** — `remember()` acquires `read()` lock for SQL writes | `memory/memory/src/lib.rs::remember` | Works because SQLite serializes writes internally, but the `parking_lot::RwLock` semantics are wrong. Concurrent `remember()` calls are not truly serialized from Rust's perspective. Fix: use `write()` lock. |
| **Bug 4** — `tauri.conf.json` missing `build` section | `daemon/src-tauri/tauri.conf.json` | `tauri build` and `tauri dev` fail. `cargo build` on the daemon crate alone succeeds. `Cargo.lock` is now committed for reproducible builds. |

Both are commented in-source. Don't silently re-fix without the comment, future readers need the context.

## SQL pitfall: `query_pattern` uses string interpolation

`query_pattern()` builds its SQL via `format!()` with `pattern.replace("'", "''")` for escaping — not parameterized. It's only called via Tauri commands today (trusted local IPC), but if it ever gets exposed to untrusted input, switch to parameterized binding like `query_by_source_type` does.

## sqlite v0.34 API note

`sqlite::Connection` has no `last_insert_rowid()` method. `ensure_node()` uses a `SELECT last_insert_rowid()` query instead. If you upgrade the sqlite crate, check whether this can be simplified back to a method call.

## Building

`cargo check` on `memory/` works in any container. The full daemon needs system libraries Tauri v1 depends on (`gdk-3.0`, `webkit2gtk-4.0`); Ubuntu 24.04 ships `webkit2gtk-4.1` only, so `cargo check` on the daemon crate fails in default cloud containers. Build the daemon on a desktop Linux machine (older Ubuntu, Arch, etc.), macOS, or Windows where Tauri v1's GTK/WebKit deps resolve.

## README.md — marketing prose, not architecture spec

The README's headline material (Proprioceptive Inference Core, 70B MoE, biometric login, encrypted P2P sync, `<1.1s` generation times, sub-8ms keystroke hooks) is product vision, not shipped code. The "Current Implementation Status" table at the bottom is the source of truth for what actually exists. When editing the README, preserve the vision but do not imply unbuilt features are live.
