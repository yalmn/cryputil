const term = document.getElementById("terminal");
const form = document.getElementById("prompt-form");
const input = document.getElementById("prompt-input");

const BANNER = String.raw`
   ___  ____ _ _ _____  _____  _ _
  / __||  _ \| | |  _  \|_   _|| | |
 | (__ |    /| | | |_) |  | |  | | |
  \___||_|\_\\_/|_/  __/   |_|  |_|_|
                  |_|
        terminal interface  v0.2.0
        type help to see commands
`;

const history = [];
let historyIndex = -1;
let busy = false;
let wasm = null;
let wasmError = null;

function append(text, cls = "line") {
  const el = document.createElement("div");
  el.className = cls;
  el.textContent = text;
  term.appendChild(el);
  term.scrollTop = term.scrollHeight;
}

function appendBanner() {
  append(BANNER, "ascii");
}

function clearTerminal() {
  term.innerHTML = "";
}

function parseInput(line) {
  const trimmed = line.trim();
  if (!trimmed) return null;
  const parts = trimmed.split(/\s+/);
  const command = parts.shift();
  const params = {};
  for (const p of parts) {
    const eq = p.indexOf("=");
    if (eq < 0) {
      throw new Error(`Parameter ohne '=' : ${p}`);
    }
    const key = p.slice(0, eq);
    const value = p.slice(eq + 1);
    params[key] = value;
  }
  return { command, params };
}

function renderTrace(trace) {
  append(`-- ${trace.algorithm} --`, "info");
  if (trace.inputs && trace.inputs.length) {
    append("Eingaben:", "info");
    for (const [k, v] of trace.inputs) append(`  ${k} = ${v}`);
  }
  for (const step of trace.steps || []) {
    append(`${step.number}) ${step.title}`, "info");
    for (const ln of step.lines || []) append(`    ${ln}`);
    if (step.table) {
      const headers = step.table.headers.join("  ");
      append(`    ${headers}`);
      for (const row of step.table.rows) append(`    ${row.join("  ")}`);
    }
  }
  if (trace.result && trace.result.length) {
    append("Ergebnis:", "info");
    for (const [k, v] of trace.result) append(`  ${k} = ${v}`);
  }
}

function showHelp() {
  if (!wasm) {
    append("WASM-Modul nicht geladen. Build: wasm-pack build --target web --out-dir ../../frontend/pkg crates/wasm", "warn");
    return;
  }
  const cmds = wasm.commands();
  append("Verfügbare Kommandos:", "info");
  for (const c of cmds) {
    const args = c.params.map((p) => `${p}=…`).join(" ");
    append(`  ${c.name}  ${args}    ${c.description}`);
  }
  append("");
  append("Eingabeformat: <kommando> key1=value key2=value", "info");
  append("Beispiel:      rsa.encrypt n=143 e=7 m=9", "info");
  append("Sonderbefehle: help, clear", "info");
}

async function run(line) {
  let parsed;
  try {
    parsed = parseInput(line);
  } catch (e) {
    append(`[error] ${e.message}`, "err");
    return;
  }
  if (!parsed) return;
  const { command, params } = parsed;

  if (command === "help") return showHelp();
  if (command === "clear") return clearTerminal();
  if (command === "version") {
    append(wasm ? wasm.version() : "wasm not loaded", "info");
    return;
  }

  if (!wasm) {
    append("[error] WASM-Modul nicht geladen", "err");
    if (wasmError) append(`        ${wasmError}`, "err");
    return;
  }

  busy = true;
  append("processing…", "info");
  try {
    const res = wasm.run(command, params);
    if (res.ok) {
      renderTrace(res.trace);
    } else {
      append(`[error] ${res.error}`, "err");
    }
  } catch (e) {
    append(`[error] ${e.message || e}`, "err");
  } finally {
    busy = false;
  }
}

async function loadWasm() {
  try {
    const mod = await import("./pkg/cryputil_wasm.js");
    await mod.default();
    wasm = mod;
    append("[ready]", "info");
  } catch (e) {
    wasmError = e.message || String(e);
    append("[warn] WASM-Bundle nicht gefunden – nur help/clear verfügbar.", "warn");
    append("       Build: wasm-pack build --target web --out-dir ../../frontend/pkg crates/wasm", "warn");
  }
}

function onSubmit(e) {
  e.preventDefault();
  if (busy) return;
  const value = input.value;
  input.value = "";
  append(`cryputil> ${value}`, "echo");
  if (value.trim()) {
    history.push(value);
    historyIndex = history.length;
  }
  run(value);
}

function onKeydown(e) {
  if (e.key === "ArrowUp") {
    if (historyIndex > 0) historyIndex--;
    input.value = history[historyIndex] ?? "";
    e.preventDefault();
  } else if (e.key === "ArrowDown") {
    if (historyIndex < history.length - 1) {
      historyIndex++;
      input.value = history[historyIndex];
    } else {
      historyIndex = history.length;
      input.value = "";
    }
    e.preventDefault();
  } else if (e.key === "l" && (e.ctrlKey || e.metaKey)) {
    clearTerminal();
    e.preventDefault();
  }
}

document.addEventListener("DOMContentLoaded", () => {
  appendBanner();
  loadWasm();
  form.addEventListener("submit", onSubmit);
  input.addEventListener("keydown", onKeydown);
  document.addEventListener("click", () => input.focus());
});
