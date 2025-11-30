🤝 Contributing to Lantern

Lantern isn’t another forgetful RAG toy. It’s a living memory system. Contributions are welcome — but they need to respect the flame.

---

🧩 How to Set Up Locally
1. Fork + clone the repo:
   `bash
   git clone https://github.com/<your-fork>/Lantern.git
   cd Lantern/daemon/src-tauri
   `
2. Install prerequisites:
   - Rust (stable toolchain)
   - Node.js (for frontend integration)
   - Tauri system deps (see Tauri prerequisites)
3. Run the daemon:
   `bash
   cargo tauri dev
   `

---

⚙️ Contribution Areas
- Hypergraph extensions  
  Add new node types (Soundtrack, EmotionCluster) or edge weighting strategies in memory/src/lib.rs.
- Daemon commands  
  Create new Tauri commands in daemon/src-tauri/src/ that expose memory functions to the frontend.
- LoRA packs  
  Train small LoRA adapters (~8MB) on your own style and document how to inject them at runtime.
- Frontend integrations  
  Build IDE plugins (VS Code, Neovim, JetBrains) that call Lantern’s commands via Tauri’s JS API.
- Sync strategies  
  Extend The Weave with new encrypted sync modes or storage backends.

---

📝 Coding Standards
- Run cargo fmt before committing.
- Run cargo clippy to catch common issues.
- Document new node/edge types clearly.
- Keep daemon commands small and composable.

---

🔄 Pull Request Process
1. Fork the repo and create a feature branch.
2. Make your changes with tests/examples.
3. Submit a PR with:
   - Clear description of what you added.
   - Why it matters for Lantern’s memory system.
4. PRs will be reviewed for:
   - Technical correctness.
   - Alignment with Lantern’s vision (memory with a pulse).
   - Simplicity and clarity.

---

🚫 What Not to Do
- Don’t add cloud‑dependent features that break local memory.
- Don’t dilute the hypergraph with generic vector DB logic.
- Don’t submit giant PRs without explanation.

---

🔥 Final Note
Lantern is about continuity, resonance, and memory that never leaves. If your contribution strengthens that, it belongs here.
