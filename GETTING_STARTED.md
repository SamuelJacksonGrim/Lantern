# 🚀 Getting Started with Lantern

Lantern is a local memory daemon. It runs quietly in the background while you code, remembering your patterns, style, and emotional fingerprint.

---

   🌱 Beginner Setup

1. Install prerequisites (Rust, Node.js, Tauri system deps).
2. Clone the repo:
   `bash
   git clone https://github.com/SamuelJacksonGrim/Lantern.git
   cd Lantern/daemon/src-tauri
   `
3. Run the daemon:
   `bash
   cargo tauri dev
   `
4. Open your IDE — Lantern runs silently, capturing keystrokes and file events.
5. Use the tray icon → “Pulse” → daily greeting + memory count.

---

⚡ Advanced Setup

1. Project structure: daemon orchestrates, memory crate stores hypergraph.
2. Extend the hypergraph: add new node/edge types in memory/src/lib.rs.
3. Inject custom LoRA: train a small LoRA (~8MB) on your style, load at runtime.
4. Encrypted sync: configure The Weave with your own key.
5. Frontend integration: call Tauri commands from JS:
   `js
   import { invoke } from '@tauri-apps/api/tauri';
   await invoke("remember_code", { what: "rate limiter", emotion: 0.92 });
   const results = await invoke("find_similar", { pattern: "middleware" });
   `

---

🧭 Summary

- Beginners: install Rust, run cargo tauri dev, let Lantern remember while you code.
- Advanced users: extend the hypergraph, inject LoRA, wire encrypted sync, or build custom frontends.
`

---
