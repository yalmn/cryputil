# Umbau-Plan: CLI → Web (Retro-Terminal) + API

Fortschritts-Tracker. Jede Phase ist eigenständig abschließbar. Bei Unterbrechung
nächste offene Phase wieder aufnehmen.

Konventionen:
- Commits kurz, kleinbuchstaben, kein AI-Hinweis, keine Emojis, ein Thema je Commit.
- Keine Änderungen direkt auf `main` außer Phase 0.
- Tests/Build vor jedem Merge.

---

## Phase 0 — main aufräumen + stable taggen

Branch: `main`

- [ ] `.gitignore` anlegen (target, .env, dist, node_modules, .DS_Store, *.wasm staging)
- [ ] `git rm -r --cached target/` und committen
- [ ] Bestehende Modul-Änderungen in logischen Commits sichern:
  - [ ] `add rsa signature module`
  - [ ] `add elgamal signature module`
  - [ ] `rename fiat-shamir commitment to k`
  - [ ] `add columnar transposition module`
  - [ ] `wire signature and transposition into menu`
- [ ] `cargo test` grün
- [ ] Tag `v0.1.0-stable`
- [ ] Push `main` + Tag

## Phase 1 — development Branch

Branch: `main` → `development`

- [ ] `git checkout -b development`
- [ ] Push `development` mit `-u`

## Phase 2 — feature/cargo-workspace-refactor

Branch: `development` → `feature/cargo-workspace-refactor`

Ziel: Logik als Library-Crate, CLI als Binary-Crate. Keine Verhaltensänderung.

- [ ] Workspace-`Cargo.toml` im Root
- [ ] `crates/core/` (lib): enthält `core/`, `algorithms/`, `analysis/`, `modulo/`, `playbooks/`
- [ ] `crates/cli/` (bin `cryputil`): enthält `main.rs`, `cli/`, `render/`
- [ ] `crates/core/src/lib.rs` reexportiert alle Module
- [ ] `crates/cli` depends on `cryputil-core`
- [ ] `cargo test --workspace` grün
- [ ] `cargo run -p cryputil-cli` läuft wie zuvor
- [ ] Merge in `development` (no-ff oder squash, einheitlich)

## Phase 3 — feature/wasm-bindings

Branch: `development` → `feature/wasm-bindings`

Ziel: Logik im Browser per WASM ausführbar.

- [ ] `crates/wasm/` mit `wasm-bindgen`, `serde`, `serde-wasm-bindgen`
- [ ] `core` bekommt optional-feature `serde` für `Trace`/`Step`/`Table`
- [ ] Dispatcher `run(command: &str, params: JsValue) -> Result<JsValue, JsValue>`
  - Whitelist erlaubter Kommandos (rsa.keygen, rsa.encrypt, ..., modulo.add, ...)
  - Param-Parsing per serde
  - Trace → JSON
- [ ] `wasm-pack build --target web --out-dir ../../frontend/pkg`
- [ ] Smoke-Test über minimales HTML
- [ ] Merge in `development`

## Phase 4 — feature/web-ui

Branch: `development` → `feature/web-ui`

Ziel: Retro-Terminal-Frontend, lädt WASM.

- [ ] `frontend/index.html`, `frontend/styles.css`, `frontend/app.js`
- [ ] CRT/Scanline-Effekt, Amber+Green Themes
- [ ] ASCII-Banner, Prompt `cryputil> `, blinkender Cursor
- [ ] History (↑/↓), `help`, `clear`, `processing…`, `[error]` Format
- [ ] Command-Parser → Whitelist-Dispatch in WASM
- [ ] Mobile-tauglich (viewport, einfaches Touch-Eingabefeld)
- [ ] Merge in `development`

## Phase 5 — feature/api-wrapper

Branch: `development` → `feature/api-wrapper`

Ziel: HTTP-API als Alternative zum WASM-Pfad.

- [ ] `crates/server/` mit `axum`, `tokio`, `tower-http` (CORS, timeout)
- [ ] `POST /api/run` mit JSON `{command, params}` → `{ok, trace?, error?}`
- [ ] Whitelist (gleiche wie WASM-Dispatcher, gemeinsame Funktion in `core`)
- [ ] Request-Timeout (z. B. 5s), Body-Size-Limit
- [ ] `GET /api/health`
- [ ] `Dockerfile` für Container-Deploy (Render/Railway)
- [ ] Integration-Test mit `reqwest` oder `tower::ServiceExt`
- [ ] Merge in `development`

## Phase 6 — feature/deployment-setup

Branch: `development` → `feature/deployment-setup`

Ziel: GitHub-basiertes Deployment vorbereiten.

- [ ] `.github/workflows/ci.yml` — `cargo fmt --check` + `cargo test --workspace` auf push/PR
- [ ] `.github/workflows/pages.yml` — wasm-pack Build + Frontend → GitHub Pages
- [ ] `.github/workflows/server.yml` — Docker-Build + Push (optional, manual trigger)
- [ ] `README.md` ergänzen: Local Dev (CLI/Web/Server), Deploy-Pfade
- [ ] `.env.example` mit Platzhaltern (PORT, RUST_LOG, ALLOWED_ORIGIN)
- [ ] `frontend/README.md` Stub
- [ ] Merge in `development`

## Phase 7 — Release-Vorbereitung (optional)

- [ ] `development` → `main` PR
- [ ] Review, Tag `v0.2.0`
- [ ] GitHub Release-Notes (manuell, kein AI-Hinweis)

---

## Status-Logbuch

- Phase 0 erledigt: `.gitignore` + target untrack, 5 Modul-Commits, Tag `v0.1.0-stable`, gepusht.
- Phase 1 erledigt: `development` angelegt und auf Remote gepusht.
- Phase 2 erledigt: Workspace mit `crates/core` + `crates/cli`, alle 29 Tests grün, Binary `cryputil` läuft, in `development` gemerged und gepusht.
- Nächste Phase: 3 (wasm bindings).
