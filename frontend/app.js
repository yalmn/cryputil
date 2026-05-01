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
].join("\n");

const I = (key, label, min) => ({ type: "int", key, label, min });
const S = (key, label) => ({ type: "str", key, label });
const O = (question, key, label, min) => ({ type: "opt_int", question, key, label, min });
const T = (...lines) => ({ type: "info", lines });

const MENUS = {
  main: {
    title: "Crypto Lab",
    items: [
      { label: "Modulo-Arithmetik", goto: "modulo" },
      { label: "Kryptografische Verfahren", goto: "crypto" },
      { label: "Identifikationsprotokolle", goto: "ident" },
      { label: "Kryptoanalyse", goto: "analyse" },
      { label: "Playbooks", goto: "playbooks" },
      { label: "Letzte Schritte anzeigen", action: "showLast" },
    ],
    backLabel: "Beenden",
  },
  modulo: {
    title: "Modulo-Arithmetik",
    items: [
      { label: "Addition", cmd: "mod.add", steps: [I("a", "a"), I("b", "b"), I("n", "n", 1)] },
      { label: "Subtraktion", cmd: "mod.sub", steps: [I("a", "a"), I("b", "b"), I("n", "n", 1)] },
      { label: "Multiplikation", cmd: "mod.mul", steps: [I("a", "a"), I("b", "b"), I("n", "n", 1)] },
      { label: "Schnelle Modulare Exponentiation", cmd: "mod.pow", steps: [I("a", "a"), I("e", "e", 0), I("n", "n", 1)] },
      { label: "Additives Inverses", cmd: "mod.inv_add", steps: [I("a", "a"), I("n", "n", 1)] },
      { label: "Multiplikatives Inverses", cmd: "mod.inv_mul", steps: [I("a", "a"), I("n", "n", 1)] },
      { label: "Zyklische Untergruppe", cmd: "mod.subgroup", steps: [I("g", "g"), I("p", "p (prim)", 2)] },
      { label: "Primitive Wurzeln", cmd: "mod.primitive_roots", steps: [I("p", "p (prim)", 2)] },
    ],
  },
  crypto: {
    title: "Kryptografische Verfahren",
    items: [
      { label: "RSA-Schlüsselerzeugung", cmd: "rsa.keygen", steps: [I("p", "p (prim)", 2), I("q", "q (prim)", 2), I("e", "e", 2)] },
      { label: "RSA-Verschlüsselung", cmd: "rsa.encrypt", steps: [I("n", "n", 2), I("e", "e", 1), I("m", "m", 0)] },
      { label: "RSA-Entschlüsselung", cmd: "rsa.decrypt", steps: [I("n", "n", 2), I("d", "d", 1), I("c", "c", 0)] },
      { label: "Diffie-Hellman", cmd: "dh.exchange", steps: [I("p", "p (prim)", 2), I("g", "g", 2), I("a", "a (privat A)", 1), I("b", "b (privat B)", 1)] },
      { label: "ElGamal-Verschlüsselung", cmd: "elgamal.encrypt", steps: [I("p", "p (prim)", 2), I("g", "g", 2), I("x", "x (privat)", 1), I("m", "m", 0), I("k", "k (ephemeral)", 1)] },
      { label: "ElGamal-Entschlüsselung", cmd: "elgamal.decrypt", steps: [I("p", "p (prim)", 2), I("x", "x (privat)", 1), I("c1", "c1", 0), I("c2", "c2", 0)] },
      { label: "Shamir Three-Pass", cmd: "shamir.three_pass", steps: [I("p", "p (prim)", 2), I("m", "m", 0), I("a", "a (Alice)", 1), I("b", "b (Bob)", 1)] },
      { label: "ECC: Punktaddition", cmd: "ecc.add", steps: [I("a", "a"), I("b", "b"), I("p", "p (prim)", 2), I("px", "P.x"), I("py", "P.y"), I("qx", "Q.x"), I("qy", "Q.y")] },
      { label: "ECC: Skalarmultiplikation", cmd: "ecc.scalar", steps: [I("a", "a"), I("b", "b"), I("p", "p (prim)", 2), I("k", "k", 0), I("px", "P.x"), I("py", "P.y")] },
      { label: "RSA-Signatur: Erzeugung", cmd: "rsa.sign", steps: [I("n", "n", 2), I("d", "d (privat)", 1), I("m", "m (Nachricht)", 0)] },
      { label: "RSA-Signatur: Verifikation", cmd: "rsa.verify", steps: [I("n", "n", 2), I("e", "e (öffentlich)", 1), I("m", "m (Nachricht)", 0), I("s", "s (Signatur)", 0)] },
      { label: "ElGamal-Signatur: Erzeugung", cmd: "elgamal.sign", steps: [I("p", "p (prim)", 2), I("g", "g", 2), I("d", "d (privat)", 1), I("m", "m (Nachricht)", 0), I("k", "k (ephemeral)", 1)] },
      { label: "ElGamal-Signatur: Verifikation", cmd: "elgamal.verify", steps: [I("p", "p (prim)", 2), I("g", "g", 2), I("e", "e (öffentlich)", 1), I("m", "m (Nachricht)", 0), I("r", "r", 0), I("s", "s", 0)] },
    ],
  },
  ident: {
    title: "Identifikationsprotokolle",
    items: [
      { label: "Fiat-Shamir (eine Runde)", cmd: "fiat_shamir.round", steps: [I("n", "n", 2), I("s", "s (Geheimnis)", 1), I("k", "k (Commitment-Zufall)", 1), I("e", "e (0 oder 1)", 0)] },
    ],
  },
  analyse: {
    title: "Kryptoanalyse",
    items: [
      { label: "Baby-Step-Giant-Step", cmd: "bsgs", steps: [I("g", "g", 2), I("h", "h", 1), I("p", "p (prim)", 2)] },
      { label: "Pollard-Rho (Faktorisierung)", cmd: "pollard_rho.factor", steps: [I("n", "n", 2)] },
      { label: "Pollard-Rho (diskreter Logarithmus)", cmd: "pollard_rho.dlog", steps: [I("g", "g", 2), I("h", "h", 1), I("p", "p (prim)", 2)] },
      { label: "Fermat-Faktorisierung", cmd: "fermat.factor", steps: [I("n", "n", 3)] },
      { label: "Spaltentransposition (Varianten)", cmd: "transposition.decrypt", steps: [S("text", "Geheimtext")] },
    ],
  },
  playbooks: {
    title: "Playbooks",
    items: [
      {
        label: "ElGamal: Multiplikativer Homomorphismus",
        cmd: "pb.elgamal_mult_homomorph",
        steps: [
          T("ElGamal: Multiplikativer Homomorphismus",
            "Öffentlicher Schlüssel K_pub = (p, g, e), privater Schlüssel d.",
            "Ursprüngliches Chiffrat (a1, b1) zu m1, kombiniertes Chiffrat (a, b)."),
          I("p", "p (prim)", 2), I("g", "g", 2), I("e", "e (öffentlich)", 1), I("d", "d (privat)", 1),
          I("a1", "a1", 0), I("b1", "b1", 0), I("a", "a (kombiniert)", 0), I("b", "b (kombiniert)", 0),
        ],
      },
      {
        label: "RSA: aus (n, e) und y → d und Klartext",
        cmd: "pb.rsa_priv_from_pub_y",
        steps: [
          T("RSA: gegeben Kpub = (n, e) und Geheimtext y, gesucht d und x."),
          I("n", "n", 2), I("e", "e", 1), I("y", "y (Geheimtext)", 0),
          O("phi(n) bekannt (für Verifikation)?", "phi", "phi(n)", 1),
        ],
      },
      {
        label: "Diffie-Hellman: aus g, p, α, β → K",
        cmd: "pb.dh_k_from_g_p_alpha_beta",
        steps: [
          T("Diffie-Hellman: gegeben g, p, α, β; gesucht gemeinsamer Schlüssel K."),
          I("g", "g", 2), I("p", "p (prim)", 2), I("alpha", "α (Alice öffentlich)", 0), I("beta", "β (Bob öffentlich)", 0),
        ],
      },
      {
        label: "ElGamal: aus Kpub = (p, g, e) → privater Schlüssel d",
        cmd: "pb.elgamal_d_from_kpub",
        steps: [
          T("ElGamal: gegeben Kpub = (p, g, e); gesucht privater Schlüssel d."),
          I("p", "p (prim)", 2), I("g", "g", 2), I("e", "e", 1),
        ],
      },
      {
        label: "RSA: passt d zu Kpub = (n, e)?",
        cmd: "pb.rsa_check_d",
        steps: [
          T("RSA: prüfen, ob privater Schlüssel d zu Kpub = (n, e) passt."),
          I("n", "n", 2), I("e", "e", 1), I("d", "d (zu prüfen)", 1),
          O("Beispiel-Geheimtext y für Round-Trip-Test angeben?", "y", "y", 0),
        ],
      },
      {
        label: "ElGamal-Signatur: Verifikation",
        cmd: "pb.elgamal_sig_verify",
        steps: [
          T("ElGamal-Signatur: Verifikation von (m, r, s) gegen Kpub = (p, g, e)."),
          I("p", "p (prim)", 2), I("g", "g", 2), I("e", "e", 1), I("m", "m", 0), I("r", "r", 0), I("s", "s", 0),
        ],
      },
      {
        label: "RSA: aus Kpub = (n, e) und y → d und Klartext (Wireshark/pcap)",
        cmd: "pb.rsa_pcap_decrypt",
        steps: [
          T("RSA: aus Kpub = (n, e) und Geheimtext y den privaten Schlüssel d",
            "und den Klartext x bestimmen (z. B. nach Wireshark-Auswertung).",
            "Hinweis: Werte aus pcap-Daten ggf. von Hex in Dezimal umrechnen."),
          I("n", "n", 2), I("e", "e", 1), I("y", "y (Geheimtext)", 0),
        ],
      },
      {
        label: "ElGamal: aus Kpub = (p, g, e) und (a, b) → d und Klartext (Wireshark/pcap)",
        cmd: "pb.elgamal_pcap_decrypt",
        steps: [
          T("ElGamal: aus Kpub = (p, g, e) und Geheimtext (a, b) den privaten",
            "Schlüssel d und den Klartext m bestimmen (z. B. nach Wireshark-Auswertung).",
            "Hinweis: Werte aus pcap-Daten ggf. von Hex in Dezimal umrechnen."),
          I("p", "p (prim)", 2), I("g", "g", 2), I("e", "e", 1), I("a", "a (= c1)", 0), I("b", "b (= c2)", 0),
        ],
      },
      {
        label: "Diffie-Hellman: gemeinsamer Schlüssel aus Mitschnitt (Wireshark/pcap)",
        cmd: "pb.dh_pcap_shared",
        steps: [
          T("Diffie-Hellman: aus dem Mitschnitt p, g, α, β extrahieren und K bestimmen.",
            "Hinweis: Werte aus pcap-Daten ggf. von Hex in Dezimal umrechnen."),
          I("p", "p (Modul, prim)", 2), I("g", "g (Basis)", 2),
          I("alpha", "α (Alice öffentlich)", 0), I("beta", "β (Bob öffentlich)", 0),
          O("Geheimtext c im Mitschnitt vorhanden?", "c", "c (Geheimtext)", 0),
        ],
      },
    ],
  },
};

const history = [];
let historyIndex = -1;
let busy = false;
let wasm = null;
let wasmError = null;
let lastTrace = null;

const state = {
  mode: "menu", // "menu" | "collect" | "ask_show_steps" | "opt_yesno"
  menu: "main",
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
  append(menu.title, "info");
  menu.items.forEach((it, i) => append(`${i + 1}) ${it.label}`));
  append(`0) ${menu.backLabel ?? "Zurück"}`);
}

function startCollecting(item) {
  state.mode = "collect";
  state.pending = { item, values: {}, idx: 0 };
  advanceStep();
}

function advanceStep() {
  const p = state.pending;
  const steps = p.item.steps || [];
  while (p.idx < steps.length) {
    const step = steps[p.idx];
    if (step.type === "info") {
      append("");
      for (const ln of step.lines) append(ln, "info");
      p.idx++;
      continue;
    }
    if (step.type === "int" || step.type === "str") {
      promptStep(step);
      return;
    }
    if (step.type === "opt_int") {
      state.mode = "opt_yesno";
      append(`  ${step.question} [j/n]: `, "info");
      return;
    }
    p.idx++;
  }
  runCommand();
}

function promptStep(step) {
  state.mode = "collect";
  append(`  ${step.label}: `, "info");
}

function handleStepInput(value) {
  const p = state.pending;
  const step = p.item.steps[p.idx];
  if (step.type === "str") {
    p.values[step.key] = value;
    p.idx++;
    advanceStep();
    return;
  }
  if (step.type === "int") {
    if (!/^-?\d+$/.test(value.trim())) {
      append("    ungültige Ganzzahl", "err");
      promptStep(step);
      return;
    }
    const v = BigInt(value.trim());
    if (step.min !== undefined && v < BigInt(step.min)) {
      append(`    Wert muss >= ${step.min} sein.`, "err");
      promptStep(step);
      return;
    }
    p.values[step.key] = value.trim();
    p.idx++;
    advanceStep();
    return;
  }
}

function handleOptYesNo(value) {
  const p = state.pending;
  const step = p.item.steps[p.idx];
  const v = value.trim().toLowerCase();
  if (["j", "ja", "y", "yes"].includes(v)) {
    state.mode = "collect";
    p.idx++;
    p.item.steps.splice(p.idx, 0, I(step.key, step.label, step.min));
    advanceStep();
    return;
  }
  if (["n", "nein", "no"].includes(v)) {
    state.mode = "collect";
    p.idx++;
    advanceStep();
    return;
  }
  append("    Bitte j oder n eingeben.", "err");
}

async function runCommand() {
  const p = state.pending;
  const cmd = p.item.cmd;
  const params = p.values;
  state.pending = null;

  if (!wasm) {
    append("[error] WASM-Modul nicht geladen", "err");
    if (wasmError) append(`        ${wasmError}`, "err");
    showMenu(state.menu);
    return;
  }

  busy = true;
  try {
    const res = wasm.run(cmd, params);
    if (res.ok) {
      lastTrace = res.trace;
      renderSummary(res.trace);
      askShowSteps();
      busy = false;
      return;
    } else {
      append("", "");
      append(`Fehler: ${res.error}`, "err");
      append("");
    }
  } catch (e) {
    append(`Fehler: ${e.message || e}`, "err");
  } finally {
    busy = false;
  }
  showMenu(state.menu);
}

function askShowSteps() {
  state.mode = "ask_show_steps";
  append("  Alle Schritte anzeigen? [j/n]: ", "info");
}

function handleShowSteps(value) {
  const v = value.trim().toLowerCase();
  if (["j", "ja", "y", "yes"].includes(v)) {
    renderFull(lastTrace);
    showMenu(state.menu);
    return;
  }
  if (["n", "nein", "no"].includes(v)) {
    showMenu(state.menu);
    return;
  }
  append("    Bitte j oder n eingeben.", "err");
}

function columnWidths(table) {
  const widths = table.headers.map((h) => [...h].length);
  for (const row of table.rows) {
    row.forEach((cell, i) => {
      const len = [...cell].length;
      if (len > widths[i]) widths[i] = len;
    });
  }
  return widths;
}

function pad(s, w) {
  const len = [...s].length;
  return s + " ".repeat(Math.max(0, w - len));
}

function renderTable(table, indent) {
  const widths = columnWidths(table);
  append(indent + table.headers.map((h, i) => pad(h, widths[i])).join(" | "));
  append(indent + widths.map((w) => "-".repeat(w)).join("-+-"));
  for (const row of table.rows) {
    append(indent + row.map((c, i) => pad(c, widths[i] ?? 0)).join(" | "));
  }
}

function renderSummary(trace) {
  append("");
  append(`Algorithmus: ${trace.algorithm}`, "info");
  if (trace.result && trace.result.length) {
    append("");
    append("Ergebnis:", "info");
    for (const [k, v] of trace.result) append(`  ${k} = ${v}`);
  }
  append("");
}

function renderFull(trace) {
  append("");
  append(`Algorithmus: ${trace.algorithm}`, "info");
  if (trace.inputs && trace.inputs.length) {
    append("");
    append("Eingaben:", "info");
    for (const [k, v] of trace.inputs) append(`  ${k} = ${v}`);
  }
  if (trace.steps && trace.steps.length) {
    append("");
    append("Schritte:", "info");
    for (const step of trace.steps) {
      append("");
      append(`[${step.number}] ${step.title}`, "info");
      for (const ln of step.lines || []) append(`    ${ln}`);
      if (step.table) {
        append("");
        renderTable(step.table, "    ");
      }
    }
  }
  if (trace.result && trace.result.length) {
    append("");
    append("Ergebnis:", "info");
    for (const [k, v] of trace.result) append(`  ${k} = ${v}`);
  }
  append("");
}

function showHelp() {
  append("Bedienung:", "info");
  append("  Auswahl per Nummer, 0 für Zurück bzw. Beenden.", "info");
  append("  Parameter werden einzeln abgefragt; ungültige Werte werden erneut erfragt.", "info");
  append("  Nach jedem Ergebnis wird gefragt, ob alle Schritte angezeigt werden sollen.", "info");
  append("  Sonderbefehle: help, clear, menu, version", "info");
}

function handleMenuSelection(value) {
  const choice = value.trim();
  const menu = MENUS[state.menu];
  if (choice === "0") {
    if (state.menu === "main") append("Bye.", "info");
    else showMenu("main");
    return;
  }
  const idx = parseInt(choice, 10);
  if (!Number.isInteger(idx) || idx < 1 || idx > menu.items.length) {
    append("    ungültige Auswahl", "err");
    showMenu(state.menu);
    return;
  }
  const item = menu.items[idx - 1];
  if (item.goto) {
    showMenu(item.goto);
  } else if (item.action === "showLast") {
    if (lastTrace) renderFull(lastTrace);
    else append("Keine vorherige Berechnung.", "warn");
    showMenu(state.menu);
  } else if (item.cmd) {
    const itemCopy = { ...item, steps: [...(item.steps || [])] };
    if (!itemCopy.steps.length) {
      state.pending = { item: itemCopy, values: {}, idx: 0 };
      runCommand();
    } else {
      startCollecting(itemCopy);
    }
  }
}

function handleSpecial(value) {
  const v = value.trim().toLowerCase();
  if (v === "help") { showHelp(); return true; }
  if (v === "clear") { clearTerminal(); return true; }
  if (v === "menu") { state.pending = null; showMenu("main"); return true; }
  if (v === "version") { append(wasm ? wasm.version() : "wasm not loaded", "info"); return true; }
  return false;
}

function dispatchInput(value) {
  if (state.mode === "menu" && handleSpecial(value)) return;
  if (state.mode === "ask_show_steps") return handleShowSteps(value);
  if (state.mode === "opt_yesno") return handleOptYesNo(value);
  if (state.mode === "collect") return handleStepInput(value);
  return handleMenuSelection(value);
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
