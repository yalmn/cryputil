# cryputil

Lehr- und Übungswerkzeug für klassische und moderne Krypto-Verfahren mit
Schritt-für-Schritt-Trace-Ausgabe. Verfügbar als CLI, im Browser (WASM) und
als HTTP-API.

## Workspace-Layout

```
crates/
  core/    # gesamte Logik, kein I/O
  cli/     # Terminal-Frontend (Binary `cryputil`)
  wasm/    # wasm-bindgen-Wrapper für den Browser
  server/  # axum-HTTP-API
frontend/  # statisches Retro-Terminal-UI
```

Die Logik liegt vollständig in `cryputil-core`. CLI, WASM und Server sind
dünne Wrapper um denselben Dispatcher.

## Lokale Entwicklung

### Voraussetzungen

- Rust stable (`rustup install stable`)
- Für WASM: `rustup target add wasm32-unknown-unknown` und
  [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)

### CLI

```sh
cargo run -p cryputil-cli
```

Zeigt das deutsche interaktive Menü.

### Tests

```sh
cargo test --workspace --all-features
```

### Web-Frontend (lokal)

```sh
wasm-pack build --target web --out-dir ../../frontend/pkg crates/wasm
cd frontend
python3 -m http.server 8000
```

Dann `http://localhost:8000/` öffnen. Eingabeformat:
`<kommando> key=value key=value`. Hilfe: `help`.

### HTTP-API

```sh
cp .env.example .env
cargo run -p cryputil-server
```

Routen:

- `GET  /api/health`
- `GET  /api/commands`
- `POST /api/run` mit Body `{"command": "rsa.encrypt", "params": {"n": "143", "e": 7, "m": 9}}`

## Deployment

### Frontend → GitHub Pages

Der Workflow `.github/workflows/pages.yml` baut das WASM-Bundle, kopiert es
nach `frontend/pkg/` und veröffentlicht das `frontend/`-Verzeichnis als
GitHub-Pages-Site. Trigger: Push auf `main` oder manuell.

Voraussetzung: in den Repo-Settings unter „Pages" als Source „GitHub Actions"
einstellen.

### HTTP-Server → Container-Hosting

Der Workflow `.github/workflows/server.yml` baut ein Docker-Image und pusht
es bei einem `v*`-Tag oder manuell nach
`ghcr.io/<owner>/cryputil-server`. Das Image kann von Render, Railway, Fly,
Cloud Run o. ä. genutzt werden. Erwartete Env-Variablen siehe
`.env.example`.

### CI

`.github/workflows/ci.yml` prüft `cargo fmt` und `cargo test --workspace
--all-features` auf jedem Push und Pull Request gegen `main` und
`development`.

## Branch-Strategie

- `main` enthält stabile Releases (getaggt, z. B. `v0.1.0-stable`).
- `development` ist der laufende Integrationszweig.
- Features entstehen auf `feature/<name>`-Branches und werden nach
  `development` gemerged. Erst nach Stabilisierung wandert `development`
  nach `main`.

## Sicherheitshinweis

`POST /api/run` führt nur Kommandos aus der internen Whitelist aus
(`cryputil_core::dispatch::commands()`). Es gibt **keine** Shell-Ausführung,
keine Dateizugriffe, keine externen Calls. Der Server hat ein 5-Sekunden-
Request-Timeout und ein 32 KiB-Body-Limit.
