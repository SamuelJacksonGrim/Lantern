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

---

## License

Lantern is licensed under the **GNU Affero General Public License v3.0**
(AGPL-3.0) — see [`LICENSE`](LICENSE). The project is also offered under separate
**commercial license terms** for those who cannot or do not wish to comply with the
AGPL (for example, embedding it in a closed-source or hosted product). Commercial
licensing enquiries: **samgrim97@gmail.com**.

This dual-licensing is only possible while the project's copyright holder can license
the whole work under both sets of terms. The agreement below is what makes that
possible while letting you keep the copyright in your own work.

## Contributor License Agreement (CLA)

By submitting a contribution to this project (a pull request, patch, or any change),
you agree to the following. This applies to every contribution you submit, now and in
the future, unless you clearly state otherwise in writing at the time.

**1. You keep your copyright.** You retain all right, title, and interest in your
contribution. This agreement does **not** assign or transfer your copyright.

**2. Certificate of origin.** You certify that:
   - the contribution is your original work, or you have the right to submit it under
     the terms here; and
   - to your knowledge, it does not violate any third party's intellectual-property
     rights; and
   - you are legally entitled to grant the licenses below.

**3. Licence to the project and its users.** You license your contribution to the
project and to everyone who receives the project under the **AGPL-3.0**, the same terms
as the rest of the work.

**4. Licence to the maintainer (the dual-licensing grant).** You additionally grant
**Samuel Jackson Grim** (the "Maintainer") a perpetual, worldwide, non-exclusive,
royalty-free, irrevocable, and **sublicensable** license to reproduce, modify, prepare
derivative works of, publicly display and perform, sublicense, and distribute your
contribution and derivative works of it **under any license terms the Maintainer
chooses, including proprietary or commercial terms.** This is what allows the project to
continue to be offered under both AGPL-3.0 and commercial licenses.

**5. Patent grant.** You grant the project's users and the Maintainer a perpetual,
worldwide, non-exclusive, royalty-free, irrevocable license under any patent claims you
can license that are necessarily infringed by your contribution, to make, use, sell,
offer to sell, import, and otherwise transfer the work. If any party brings a patent
claim alleging the contribution infringes, the patent licenses you granted under this
agreement for that contribution terminate.

**6. No obligation.** You understand the Maintainer is not obligated to use or merge
your contribution.

**7. As-is.** Unless required by law or agreed in writing, you provide your contribution
on an "AS IS" basis, without warranties of any kind.

### How you signify agreement

Add a `Signed-off-by` line to each commit (this is the Developer Certificate of Origin
sign-off, and for this project also indicates agreement to the CLA above):

```
Signed-off-by: Your Name <your.email@example.com>
```

You can add it automatically with `git commit -s`. Opening a pull request that contains
signed-off commits is taken as your agreement to this CLA for those contributions.

---

*This is a plain-language contributor agreement modeled on widely used open-source CLAs
(the Apache Individual CLA, the Developer Certificate of Origin, and Harmony-style
grant-back agreements). It is not legal advice; for a contribution of substantial size
or from a company, the Maintainer may ask for a separately signed agreement.*
