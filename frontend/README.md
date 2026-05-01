# cryputil – Web-Frontend

Retro-Terminal-Oberfläche, die das WASM-Bundle aus `crates/wasm` lädt und alle
Kommandos lokal im Browser ausführt. Keine Server-Komponente nötig.

## Voraussetzungen

- Rust mit `wasm32-unknown-unknown` Target (`rustup target add wasm32-unknown-unknown`)
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
- beliebiger statischer HTTP-Server (z. B. `python3 -m http.server`)

## Build

Aus dem Repo-Root:

```sh
wasm-pack build --target web --out-dir ../../frontend/pkg crates/wasm
```

Das erzeugt `frontend/pkg/` (gitignored). `cryputil_wasm.js` wird vom Frontend
geladen.

## Start lokal

```sh
cd frontend
python3 -m http.server 8000
```

Dann `http://localhost:8000/` im Browser öffnen.

## Bedienung

- Eingabeformat: `<kommando> key=value key=value`
- Beispiele:
  - `help`
  - `clear`
  - `version`
  - `rsa.encrypt n=143 e=7 m=9`
  - `transposition.decrypt text=WIDIPIKEA`
- Pfeil hoch / runter blättert durch History.
- Strg/Cmd+L löscht das Terminal.
