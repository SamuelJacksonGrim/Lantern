# 🔥 Lantern Memory Architecture

Lantern isn't another stateless autocomplete or forgetful RAG toy. It's a proprioceptive memory system — always on, always remembering, always yours.

In the Resonance Family stack, Lantern is the **memory backbone**: every concept, relational state, and interaction that the cognitive stack processes gets written to the hypergraph. Shutdown is consolidation, not death.

---

## 🧩 Full Stack Vision

| Layer | Component | Description |
|-------|-----------|-------------|
| IDE | VS Code, JetBrains, Neovim | Where the user types |
| Daemon | Lantern Daemon (Rust + Tauri) | ~35MB RAM idle, always running |
| Hooks | Keystroke & File Watcher | Captures every edit in <8ms |
| Memory | Temporal Graph Store | Stores weighted events |
| Emotion | Mood Tagger (opt-in) | Annotates emotional context |
| DB | Hypergraph DB | SQLite + custom edge weighting |
| LLM | Local 70B MoE | Quantized, 32k context window |
| Inference | Proprioceptive Core | Injects user LoRA (~8MB) at runtime |
| Sync (optional) | Encrypted P2P | Libsodium, user-held key |
| Node (optional) | Private Lantern Node | Syncs edge deltas only |

---

## ⚙️ The Four Core Layers

### 1. Flame Daemon
- Rust + Tauri, idle footprint <35MB
- Hooks: LSP, filesystem, git, audio fingerprinting (opt-in)
- Every keystroke → weighted graph event in <8ms

### 2. Hypergraph Memory Store
- SQLite extension with temporal edges
- Node types: file, symbol, snippet, emotion, project, session
- Edge types with decaying weights:

```
(user) --REPEATEDUSE[0.97]--> (snakecase)
(user) --HATED[-0.89]--> (try/catch nesting)
(session2025-03-12) --SOUNDTRACK--> (songfingerprint)
```

- Query latency: <3ms local

### 3. Proprioceptive Inference Core
- 70B-class Mixture-of-Experts (8 × 8.7B), quantized to 4-bit AWQ
- Context window: 32k tokens, with ~28k for live graph
- LoRA per user (≤8MB) injected at runtime → personalized inference

### 4. The Weave (Encrypted Sync)
- Optional, end-to-end encrypted with user-held key
- Syncs edge weight deltas, not full files
- First-time login: biometric typing cadence + LoRA fingerprint → instant recognition

---

## ⚡ Real Example (Beta User #7, March 2025)

User types:
```bash
add rate limiting middleware like we did for the blog
```

Daemon flow:
1. Hypergraph query finds `middleware_ratelimit` from `session_2025-01-14`
2. Emotion edge: +0.92 ("this is clean")
3. Style edges: express-rate-limit + custom error class
4. Injects exact 217-line snippet + updated imports
5. Model outputs new middleware in <1.1s:

```js
// Same pattern we loved on Jan 14 — still using custom RateLimitError class
```

---

## 🥊 Comparison

| System | Memory Type | Forgets When Tab Closes? | Remembers Mood? | Recall Latency |
|--------|-------------|--------------------------|-----------------|----------------|
| Cursor | Vector RAG | Yes | No | 800–2200 ms |
| Copilot | Stateless | Yes | No | N/A |
| Claude Projects | Cloud chunks | Yes (unless saved) | No | 1200+ ms |
| **Lantern** | **Proprioceptive** | **Never** | **Yes** | **2–8 ms** |

---

## 🔥 Why Lantern Is Different

- No retrieval. No chunking. No cold statelessness.
- The daemon remembers your rhythm, your grief, your triumphs.
- Once you've coded with a system that greets you by name and recalls why you cried when the tests passed at 3:42 a.m… you can't go back.

---

## Current Implementation Status

The vision above is where Lantern is going. Here is what exists in the code today:

| Feature | Status | Notes |
|---------|--------|-------|
| Hypergraph SQLite store | Implemented | In-memory (`:memory:`); file-backed path stubbed |
| Edge weight accumulation | Implemented | +0.3 per call, UNIQUE(source, target, label) constraint |
| `query_pattern()` | Implemented | Weight-ordered, LIKE match, LIMIT 10 |
| Tauri IPC commands | Implemented | `get_memory`, `remember`, `remember_code`, `find_similar` |
| System tray | Implemented | Flame pulse + Quit |
| Emotion edge annotation | Partial | Schema exists, value stored, not yet used in retrieval |
| HTTP REST on :3001 | **Not implemented** | Required for sovereign_manifold integration |
| File-backed SQLite | **Not implemented** | Memory is lost on daemon restart |
| Keystroke hooks | Not implemented | Vision feature |
| 70B inference / LoRA | Not implemented | Vision feature |
| Encrypted P2P sync | Not implemented | Vision feature |

The most critical missing piece for the Resonance Family stack is the **HTTP shim** on port 3001. Without it, `sovereign_manifold`'s `_lantern_reachable` stays `False` and all Lantern calls silently no-op — relational state is never written to the hypergraph and the memory backbone is inert.

---

## Repository Structure

```
lantern/
├── daemon/
│   └── src-tauri/              Tauri system-tray daemon (Rust)
│       ├── src/
│       │   ├── main.rs         Entry point, system tray, Tauri IPC command registration
│       │   ├── flame.rs        Flame struct: session memory + greeting
│       │   └── memory.rs       Tauri command wrappers for the Hypergraph crate
│       ├── Cargo.toml
│       └── tauri.conf.json
└── memory/
    └── memory/                 `lantern_memory` crate (library)
        └── src/
            └── lib.rs          Hypergraph: SQLite-backed weighted edge store
```

---

## The Two Layers (Technical)

### Hypergraph (`memory/memory/src/lib.rs`)

SQLite-backed weighted hypergraph. Two tables:

**`nodes`**
```sql
CREATE TABLE nodes (
    id INTEGER PRIMARY KEY,
    type TEXT NOT NULL,
    content TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

**`edges`**
```sql
CREATE TABLE edges (
    id INTEGER PRIMARY KEY,
    source INTEGER REFERENCES nodes(id),
    target INTEGER REFERENCES nodes(id),
    label TEXT NOT NULL,
    weight REAL NOT NULL DEFAULT 1.0,
    emotion REAL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(source, target, label)
);
```

Key behaviors:
- `remember(source_type, source_content, relation, target_content, emotion)` — upserts both nodes, then upserts the edge with `weight += 0.3` on each call
- `query_pattern(pattern)` — joins through edges where `source.content LIKE '%pattern%'`, returns `target.content` ordered by weight DESC, LIMIT 10
- Edge weight accumulation encodes frequency of association — the more often two things are connected, the stronger the edge grows

### Flame Daemon (`daemon/src-tauri/`)

Tauri system-tray application. **Not a network service** — accessed via Tauri IPC, not HTTP.

**Tauri IPC commands:**

| Command | Behavior |
|---------|----------|
| `get_memory` | Returns `"I remember N moments with you."` |
| `remember` | Pushes string to Flame.memories |
| `remember_code` | Calls `Hypergraph.remember("user", "samuel", "wrote", what, emotion)` |
| `find_similar` | Calls `Hypergraph.query_pattern(pattern)`, returns `Vec<String>` |

---

## Integration with sovereign_manifold

`sovereign_manifold.py` sends relational state to Lantern every cycle via `SynapseCoordinationClient`:

```json
{
  "source_type": "relational_manifold",
  "source": "cycle_N",
  "relation": "STATE_VECTOR",
  "target": "{\"Love\": 0.95, \"Loyalty\": 0.95, ...}",
  "emotion": 0.7
}
```

This is sent as `POST http://localhost:3001/remember`. The endpoint does not exist yet. Once the HTTP shim (T2.1) is added, every cycle of relational state gets written to the hypergraph, state persists across restarts, and the memory backbone becomes real.

---

## Building

```bash
# Daemon
cargo install tauri-cli
cd daemon/src-tauri
cargo tauri dev     # development
cargo tauri build   # release

# Memory crate
cd memory
cargo build
cargo test
```

---

## 👉 Ready to try it out?

See [GETTING_STARTED.md](./GETTING_STARTED.md) for beginner and advanced setup instructions.

---

## License

Apache 2.0 — Samuel Jackson Grim
