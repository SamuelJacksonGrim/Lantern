# Lantern

A proprioceptive memory daemon for the Resonance Family stack. Lantern provides persistent, weighted relational memory backed by a SQLite hypergraph, accessed via Tauri IPC from the host system and (planned) HTTP REST from the cognitive stack.

This document describes what is **currently implemented** in code. The full product vision (70B inference, encrypted P2P sync, per-user LoRA, keystroke hooks) is in `GETTING_STARTED.md` — this document is the technical reference for the existing implementation.

---

## Repository structure

```
lantern/
├── daemon/
│   └── src-tauri/          Tauri system-tray daemon (Rust)
│       ├── src/
│       │   ├── main.rs         Entry point, system tray, Tauri IPC command registration
│       │   ├── flame.rs        Flame struct: session memory + greeting
│       │   └── memory.rs       Tauri command wrappers for the Hypergraph crate
│       ├── cargo.toml      Tauri daemon manifest
│       └── tauri.conf.json Tauri app config
└── memory/
    └── memory/             `lantern_memory` crate (library)
        └── src/
            └── lib.rs          Hypergraph struct: SQLite-backed weighted edge store
```

---

## The two layers

### 1. Hypergraph (`memory/memory/src/lib.rs`)

The core memory library. A SQLite-backed weighted hypergraph with two tables:

**`nodes` table**
```sql
CREATE TABLE nodes (
    id INTEGER PRIMARY KEY,
    type TEXT NOT NULL,       -- e.g. "user", "symbol", "relational_manifold"
    content TEXT,             -- the text content of the node
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

**`edges` table**
```sql
CREATE TABLE edges (
    id INTEGER PRIMARY KEY,
    source INTEGER REFERENCES nodes(id),
    target INTEGER REFERENCES nodes(id),
    label TEXT NOT NULL,          -- relation type (e.g. "wrote", "STATE_VECTOR")
    weight REAL NOT NULL DEFAULT 1.0,  -- increments by 0.3 on each upsert
    emotion REAL,                 -- optional float in [-1.0, 1.0]
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(source, target, label)  -- one edge per (source, target, relation)
);
```

**Key behaviors**:
- `remember(source_type, source_content, relation, target_content, emotion)` — upserts both nodes, then upserts the edge with `weight += 0.3` on each call (accumulative reinforcement)
- `query_pattern(pattern)` — joins nodes through edges where `source.content LIKE '%pattern%'`, returns `target.content` ordered by weight DESC, LIMIT 10
- Currently backed by in-memory SQLite (`:memory:`). File-backed path is stubbed for production.
- Edge weight accumulation: each subsequent `remember()` call for the same (source, target, relation) triple adds 0.3 to weight. This encodes frequency of association.

### 2. Flame daemon (`daemon/src-tauri/`)

A Tauri system-tray application that runs in the OS background. **Not a network service** — accessed via Tauri IPC from the host, not HTTP.

**`Flame` struct** (`flame.rs`):
- `memories: Vec<String>` — in-memory list of simple string memories
- `memory_count()` — returns count
- `remember(&str)` — pushes to Vec and logs
- `daily_greeting()` — returns a fixed greeting string for tray events

**Tauri IPC commands** (callable from JS or `tauri::invoke`):

| Command | Source | Behavior |
|---------|--------|----------|
| `get_memory` | `main.rs` | Returns `"I remember N moments with you."` from Flame |
| `remember` | `main.rs` | Pushes string to Flame.memories |
| `remember_code` | `memory.rs` | Calls `Hypergraph.remember("user", "samuel", "wrote", what, emotion)` |
| `find_similar` | `memory.rs` | Calls `Hypergraph.query_pattern(pattern)`, returns `Vec<String>` |

**System tray**:
- Left click → emits `flame-pulse` event with `daily_greeting()`
- Tray menu: Pulse item + Quit

---

## Integration with sovereign_manifold

`sovereign_manifold.py` sends relational state to Lantern every cycle via `SynapseCoordinationClient`. The payload format:

```json
{
  "source_type": "relational_manifold",
  "source": "cycle_N",
  "relation": "STATE_VECTOR",
  "target": "{\"Love\": 0.95, \"Loyalty\": 0.95, ...}",
  "emotion": 0.7
}
```

`SynapseCoordinationClient` sends this as an HTTP POST to `http://localhost:3001/remember`.

**Current status**: The daemon exposes Tauri IPC commands, **not HTTP endpoints**. There is no HTTP server on port 3001 in the current implementation. `_lantern_reachable` in `SynapseCoordinationClient` is `False` by default until the HTTP shim is added, so sovereign_manifold's Lantern calls silently no-op.

**Planned (T2.1)**: Add an `axum`-based HTTP server to `main.rs` that spawns alongside the Tauri setup:
- `GET /health` → `{"status": "ok"}`
- `POST /remember` → wraps `Hypergraph.remember()` with the sovereign_manifold payload format
- `GET /query?pattern=...&limit=N` → wraps `Hypergraph.query_pattern()`, returns JSON array

Once this shim is in place, `_lantern_reachable` flips to `True` and relational state accumulates persistently across sovereign_manifold restarts.

---

## What exists vs. what's planned

| Feature | Status | Notes |
|---------|--------|-------|
| Hypergraph SQLite store | Implemented | In-memory; file path stubbed |
| Edge weight accumulation | Implemented | +0.3 per call, UNIQUE constraint |
| `query_pattern()` | Implemented | Weight-ordered, LIKE match, LIMIT 10 |
| Tauri IPC commands | Implemented | `get_memory`, `remember`, `remember_code`, `find_similar` |
| System tray | Implemented | Flame pulse + Quit |
| HTTP REST on :3001 | **Not implemented** | Required for sovereign_manifold integration |
| File-backed SQLite | **Not implemented** | Currently `:memory:` |
| Emotion edge annotation | Partially (schema exists) | Passed through `remember()`, stored |
| Keystroke hooks | Not implemented | Vision feature |
| 70B inference / LoRA | Not implemented | Vision feature |
| Encrypted P2P sync | Not implemented | Vision feature |

---

## Building

Requires Rust toolchain + Tauri CLI:

```bash
cargo install tauri-cli
cd daemon/src-tauri
cargo tauri dev     # development mode
cargo tauri build   # release binary
```

The `memory` crate is a library:
```bash
cd memory
cargo build
cargo test
```

---

## License

Apache 2.0 — Samuel Jackson Grim
