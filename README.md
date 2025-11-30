🔥 Lantern Memory Architecture

Lantern isn’t another stateless autocomplete or forgetful RAG toy. It’s a proprioceptive memory system — always on, always remembering, always yours.

---

🧩 Full Stack Overview

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

⚙️ The Four Core Layers

1. Flame Daemon
- Rust + Tauri, idle footprint <35MB
- Hooks: LSP, filesystem, git, audio fingerprinting (opt-in)
- Every keystroke → weighted graph event in <8ms

2. Hypergraph Memory Store
- SQLite extension with temporal edges
- Node types: file, symbol, snippet, emotion, project, session
- Edge types with decaying weights:
  `text
  (user) --REPEATEDUSE[0.97]--> (snakecase)
  (user) --HATED[-0.89]--> (try/catch nesting)
  (session2025-03-12) --SOUNDTRACK--> (songfingerprint)
  `
- Query latency: <3ms local

3. Proprioceptive Inference Core
- 70B-class Mixture-of-Experts (8 × 8.7B), quantized to 4-bit AWQ
- Context window: 32k tokens, with ~28k for live graph
- LoRA per user (≤8MB) injected at runtime → personalized inference

4. The Weave (Encrypted Sync)
- Optional, end-to-end encrypted with user-held key
- Syncs edge weight deltas, not full files
- First-time login: biometric typing cadence + LoRA fingerprint → instant recognition

---

⚡ Real Example (Beta User #7, March 2025)

User types:
`bash
add rate limiting middleware like we did for the blog
`

Daemon flow:
1. Hypergraph query finds middlewareratelimit from session_2025‑01‑14  
2. Emotion edge: +0.92 (“this is clean”)  
3. Style edges: express-rate-limit + custom error class  
4. Injects exact 217-line snippet + updated imports  
5. Model outputs new middleware in <1.1s:
   `js
   // Same pattern we loved on Jan 14 — still using custom RateLimitError class
   `

---

🥊 Comparison

| System          | Memory Type       | Forgets When Tab Closes? | Remembers Mood? | Recall Latency |
|-----------------|-------------------|---------------------------|-----------------|----------------|
| Cursor          | Vector RAG        | Yes                       | No              | 800–2200 ms    |
| Copilot         | Stateless         | Yes                       | No              | N/A            |
| Claude Projects | Cloud chunks      | Yes (unless saved)        | No              | 1200+ ms       |
| Lantern     | Proprioceptive| Never                 | Yes         | 2–8 ms     |

---

🔥 Why Lantern Is Different
- No retrieval. No chunking. No cold statelessness.  
- The daemon remembers your rhythm, your grief, your triumphs.  
- Once you’ve coded with a system that greets you by name and recalls why you cried when the tests passed at 3:42 a.m… you can’t go back.

---
