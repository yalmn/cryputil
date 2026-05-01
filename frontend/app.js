const term = document.getElementById("terminal");
const form = document.getElementById("prompt-form");
const input = document.getElementById("prompt-input");

const BANNER = [
  "                             _   _ _ ",
  "  ___ _ __ _   _ _ __  _   _| |_(_) |",
  " / __| '__| | | | '_ \\| | | | __| | |",
  "| (__| |  | |_| | |_) | |_| | |_| | |",
  " \\___|_|   \\__, | .__/ \\__,_|\\__|_|_|",
  "           |___/|_|                  ",
  "",
  "        terminal interface  v0.2.0",
  "        type help for hints",
].join("\n");

const MENUS = {
  main: {
    title: "Hauptmenü",
    items: [
      { label: "Modulo-Arithmetik", goto: "modulo" },
      { label: "Krypto-Verfahren", goto: "krypto" },
      { label: "Identifikation", goto: "ident" },
      { label: "Analyse", goto: "analyse" },
      { label: "Playbooks", goto: "playbooks" },
    ],
  },
  modulo: {
    title: "Modulo-Arithmetik",
    items: [
      { label: "Addition", cmd: "mod.add", params: ["a", "b", "n"] },
      { label: "Subtraktion", cmd: "mod.sub", params: ["a", "b", "n"] },
      { label: "Multiplikation", cmd: "mod.mul", params: ["a", "b", "n"] },
      { label: "Schnelle Modulare Exponentiation", cmd: "mod.pow", params: ["a", "e", "n"] },
      { label: "Additives Inverses", cmd: "mod.inv_add", params: ["a", "n"] },
      { label: "Multiplikatives Inverses", cmd: "mod.inv_mul", params: ["a", "n"] },
      { label: "Zyklische Untergruppe", cmd: "mod.subgroup", params: ["g", "p"] },
      { label: "Primitive Wurzeln", cmd: "mod.primitive_roots", params: ["p"] },
    ],
  },
  krypto: {
    title: "Krypto-Verfahren",
    items: [
      { label: "RSA: Schlüsselerzeugung", cmd: "rsa.keygen", params: ["p", "q", "e"] },
      { label: "RSA: Verschlüsselung", cmd: "rsa.encrypt", params: ["n", "e", "m"] },
      { label: "RSA: Entschlüsselung", cmd: "rsa.decrypt", params: ["n", "d", "c"] },
      { label: "RSA: Signatur erzeugen", cmd: "rsa.sign", params: ["n", "d", "m"] },
      { label: "RSA: Signatur prüfen", cmd: "rsa.verify", params: ["n", "e", "m", "s"] },
      { label: "ElGamal: Verschlüsselung", cmd: "elgamal.encrypt", params: ["p", "g", "x", "m", "k"] },
      { label: "ElGamal: Entschlüsselung", cmd: "elgamal.decrypt", params: ["p", "x", "c1", "c2"] },
      { label: "ElGamal: Signatur erzeugen", cmd: "elgamal.sign", params: ["p", "g", "d", "m", "k"] },
      { label: "ElGamal: Signatur prüfen", cmd: "elgamal.verify", params: ["p", "g", "e", "m", "r", "s"] },
      { label: "Diffie-Hellman", cmd: "dh.exchange", params: ["p", "g", "a", "b"] },
      { label: "Shamir Three-Pass", cmd: "shamir.three_pass", params: ["p", "m", "a", "b"] },
      { label: "ECC: Punktaddition", cmd: "ecc.add", params: ["a", "b", "p", "px", "py", "qx", "qy"] },
      { label: "ECC: Skalarmultiplikation", cmd: "ecc.scalar", params: ["a", "b", "p", "k", "px", "py"] },
    ],
  },
  ident: {
    title: "Identifikation",
    items: [
      { label: "Fiat-Shamir-Runde", cmd: "fiat_shamir.round", params: ["n", "s", "k", "e"] },
    ],
  },
  analyse: {
    title: "Analyse",
    items: [
      { label: "Baby-Step-Giant-Step", cmd: "bsgs", params: ["g", "h", "p"] },
      { label: "Pollard-Rho Faktorisierung", cmd: "pollard_rho.factor", params: ["n"] },
      { label: "Pollard-Rho Logarithmus", cmd: "pollard_rho.dlog", params: ["g", "h", "p"] },
      { label: "Fermat-Faktorisierung", cmd: "fermat.factor", params: ["n"] },
      { label: "Spaltentransposition", cmd: "transposition.decrypt", params: ["text"] },
    ],
  },
  playbooks: {
    title: "Playbooks",
    items: [
      { label: "ElGamal Homomorphismus", cmd: "pb.elgamal_mult_homomorph", params: ["p", "g", "e", "d", "a1", "b1", "a", "b"] },
      { label: "RSA: d aus (n,e,y)", cmd: "pb.rsa_priv_from_pub_y", params: ["n", "e", "y", "phi?"] },
      { label: "DH: K aus α,β", cmd: "pb.dh_k_from_g_p_alpha_beta", params: ["g", "p", "alpha", "beta"] },
      { label: "ElGamal: d aus Kpub", cmd: "pb.elgamal_d_from_kpub", params: ["p", "g", "e"] },
      { label: "RSA: passt d zu Kpub?", cmd: "pb.rsa_check_d", params: ["n", "e", "d", "y?"] },
      { label: "ElGamal-Signatur prüfen", cmd: "pb.elgamal_sig_verify", params: ["p", "g", "e", "m", "r", "s"] },
      { label: "RSA aus pcap", cmd: "pb.rsa_pcap_decrypt", params: ["n", "e", "y"] },
      { label: "ElGamal aus pcap", cmd: "pb.elgamal_pcap_decrypt", params: ["p", "g", "e", "a", "b"] },
      { label: "DH aus pcap", cmd: "pb.dh_pcap_shared", params: ["p", "g", "alpha", "beta", "c?"] },
    ],
  },
};

const history = [];
let historyIndex = -1;
let busy = false;
let wasm = null;
let wasmError = null;

const state = {
  mode: "menu",
  menu: "main",
  parent: null,
  pending: null,
};

function append(text, cls = "line") {
  const el = document.createElement("div");
  el.className = cls;
  el.textContent = text;
  term.appendChild(el);
  term.scrollTop = term.scrollHeight;
}

function clearTerminal() {
  term.innerHTML = "";
}

function showMenu(name) {
  state.mode = "menu";
  state.menu = name;
  const menu = MENUS[name];
  append("");
  append(`-- ${menu.title} --`, "info");
  menu.items.forEach((it, i) => append(`  ${i + 1}) ${it.label}`));
  append(name === "main" ? "  0) Beenden" : "  0) Zurück");
}

function startCollecting(item, parentMenu) {
  state.mode = "collect";
  state.pending = { item, parent: parentMenu, values: {}, idx: 0 };
  promptNextParam();
}

function promptNextParam() {
  const p = state.pending;
  const params = p.item.params;
  if (p.idx >= params.length) {
    runCommand();
    return;
  }
  const raw = params[p.idx];
  const optional = raw.endsWith("?");
  const key = optional ? raw.slice(0, -1) : raw;
  const hint = optional ? " (optional, leer lassen für überspringen)" : "";
  append(`  ${key}${hint}: `, "info");
}

function handleParamInput(value) {
  const p = state.pending;
  const raw = p.item.params[p.idx];
  const optional = raw.endsWith("?");
  const key = optional ? raw.slice(0, -1) : raw;
  if (value === "" && !optional) {
    append("[error] Pflichtfeld – bitte Wert eingeben.", "err");
    promptNextParam();
    return;
  }
  if (value !== "") p.values[key] = value;
  p.idx++;
  promptNextParam();
}

async function runCommand() {
  const p = state.pending;
  const cmd = p.item.cmd;
  const params = p.values;
  const parent = p.parent;
  state.pending = null;

  if (!wasm) {
    append("[error] WASM-Modul nicht geladen", "err");
    if (wasmError) append(`        ${wasmError}`, "err");
    showMenu(parent);
    return;
  }

  busy = true;
  append("processing…", "info");
  try {
    const res = wasm.run(cmd, params);
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
  showMenu(parent);
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
  append("Bedienung:", "info");
  append("  Auswahl per Nummer (1–9), 0 für Zurück bzw. Beenden.", "info");
  append("  Nach Auswahl eines Verfahrens werden die Parameter einzeln abgefragt.", "info");
  append("  Optionale Parameter (mit ?) können leer gelassen werden.", "info");
  append("  Sonderbefehle jederzeit: help, clear, menu, version", "info");
}

function handleMenuSelection(value) {
  const choice = value.trim();
  const menu = MENUS[state.menu];
  if (choice === "0") {
    if (state.menu === "main") {
      append("Bye.", "info");
    } else {
      showMenu("main");
    }
    return;
  }
  const idx = parseInt(choice, 10);
  if (!Number.isInteger(idx) || idx < 1 || idx > menu.items.length) {
    append("[error] ungültige Auswahl", "err");
    showMenu(state.menu);
    return;
  }
  const item = menu.items[idx - 1];
  if (item.goto) {
    showMenu(item.goto);
  } else if (item.cmd) {
    append(`> ${item.label}`, "echo");
    if (!item.params || item.params.length === 0) {
      state.pending = { item, parent: state.menu, values: {}, idx: 0 };
      runCommand();
    } else {
      startCollecting(item, state.menu);
    }
  }
}

function handleSpecial(value) {
  const v = value.trim().toLowerCase();
  if (v === "help") { showHelp(); return true; }
  if (v === "clear") { clearTerminal(); return true; }
  if (v === "menu") {
    state.pending = null;
    showMenu("main");
    return true;
  }
  if (v === "version") {
    append(wasm ? wasm.version() : "wasm not loaded", "info");
    return true;
  }
  return false;
}

function dispatchInput(value) {
  if (handleSpecial(value)) return;
  if (state.mode === "collect") {
    handleParamInput(value);
  } else {
    handleMenuSelection(value);
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
  showMenu("main");
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
  dispatchInput(value);
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
  append(BANNER, "ascii");
  loadWasm();
  form.addEventListener("submit", onSubmit);
  input.addEventListener("keydown", onKeydown);
  document.addEventListener("click", () => input.focus());
});
