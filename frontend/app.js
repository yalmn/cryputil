const paneRoot = document.getElementById("pane-root");
const palette = document.getElementById("palette");
const paletteInput = document.getElementById("palette-input");
const paletteList = document.getElementById("palette-list");
const paletteCwd = document.getElementById("palette-cwd");
const paletteInfo = document.getElementById("palette-info");
const prefixHint = document.getElementById("prefix-hint");

const BANNER = [
  "                             _   _ _ ",
  "  ___ _ __ _   _ _ __  _   _| |_(_) |",
  " / __| '__| | | | '_ \\| | | | __| | |",
  "| (__| |  | |_| | |_) | |_| | |_| | |",
  " \\___|_|   \\__, | .__/ \\__,_|\\__|_|_|",
  "           |___/|_|                  ",
  "",
  "Author: https://github.com/yalmn",
  "v0.2.0, 2026",
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
      { label: "Substitution: Verschlüsselung", cmd: "subst.encrypt", steps: [S("text", "Klartext"), S("key", "Schlüssel (26 Buchstaben, Bild von A..Z)")] },
      { label: "Substitution: Entschlüsselung", cmd: "subst.decrypt", steps: [S("text", "Chiffrat"), S("key", "Schlüssel (26 Buchstaben, Bild von A..Z)")] },
      { label: "Paillier-Schlüsselerzeugung", cmd: "paillier.keygen", steps: [I("p", "p (prim)", 2), I("q", "q (prim)", 2)] },
      { label: "Paillier-Verschlüsselung", cmd: "paillier.encrypt", steps: [I("n", "n", 2), I("g", "g", 1), I("m", "m (Klartext)", 0), I("r", "r (Zufall, gcd(r,n)=1)", 1)] },
      { label: "Paillier-Entschlüsselung", cmd: "paillier.decrypt", steps: [I("n", "n", 2), I("lambda", "λ", 1), I("mu", "μ", 1), I("c", "c (Chiffrat)", 0)] },
      { label: "Caesar: Verschlüsselung", cmd: "caesar.encrypt", steps: [S("text", "Klartext"), I("shift", "Verschiebung k")] },
      { label: "Caesar: Entschlüsselung", cmd: "caesar.decrypt", steps: [S("text", "Chiffrat"), I("shift", "Verschiebung k")] },
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
      { label: "Baby-Step-Giant-Step", cmd: "bsgs", steps: [I("g", "g", 2), I("e", "e", 1), I("p", "p (prim)", 2)] },
      { label: "Pollard-Rho (Faktorisierung)", cmd: "pollard_rho.factor", steps: [I("n", "n", 2)] },
      { label: "Pollard-Rho (diskreter Logarithmus)", cmd: "pollard_rho.dlog", steps: [I("g", "g", 2), I("h", "h", 1), I("p", "p (prim)", 2)] },
      { label: "Fermat-Faktorisierung", cmd: "fermat.factor", steps: [I("n", "n", 3)] },
      { label: "Spaltentransposition (Varianten)", cmd: "transposition.decrypt", steps: [S("text", "Geheimtext")] },
      { label: "Häufigkeitsanalyse", cmd: "freq.analyze", steps: [S("text", "Geheimtext"), S("lang", "Sprache (de/en)")] },
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
      {
        label: "Paillier: Additiver Homomorphismus",
        cmd: "pb.paillier_add_homomorph",
        steps: [
          T("Paillier: Additiver Homomorphismus",
            "E(m1) · E(m2) mod n² entschlüsselt zu (m1 + m2) mod n.",
            "Zufallswerte r1, r2 müssen gcd(r_i, n) = 1 erfüllen."),
          I("p", "p (prim)", 2), I("q", "q (prim)", 2),
          I("m1", "m1", 0), I("m2", "m2", 0),
          I("r1", "r1", 1), I("r2", "r2", 1),
        ],
      },
    ],
  },
};

const FOLDER_DESCRIPTIONS = {
  modulo: "Grundoperationen der Modulo-Arithmetik: Addition, Multiplikation, schnelle Exponentiation, Inverse und zyklische Gruppen. Bausteine für fast alle anderen Verfahren.",
  crypto: "Vollständige Kryptoverfahren: RSA, Diffie-Hellman, ElGamal, ECC, Paillier, Shamir Three-Pass, Substitution und Caesar – jeweils mit Schritt-für-Schritt-Trace.",
  ident: "Identifikations- und Zero-Knowledge-Protokolle. Hier: Fiat-Shamir (eine Runde) mit Commitment, Challenge und Response.",
  analyse: "Kryptoanalyse-Werkzeuge: Baby-Step-Giant-Step, Pollard-Rho (Faktorisierung & DLog), Fermat, Spaltentransposition und Häufigkeitsanalyse.",
  playbooks: "Aufgaben-orientierte Abläufe (z. B. „aus Mitschnitt → privater Schlüssel“). Kombinieren mehrere Schritte zu einem geführten Lösungsweg.",
};

const CMD_DESCRIPTIONS = {
  "mod.add":  "Berechnet (a + b) mod n und zeigt die Normalisierung mit rem_euclid.",
  "mod.sub":  "Berechnet (a − b) mod n. Negative Zwischenergebnisse werden korrekt normalisiert.",
  "mod.mul":  "Berechnet (a · b) mod n; nützlich als Baustein für RSA, ElGamal etc.",
  "mod.pow": "Schnelle modulare Exponentiation a^e mod n nach dem Square-and-Multiply-Verfahren.",
  "mod.inv_add": "Additives Inverses von a modulo n: das x mit (a + x) ≡ 0 (mod n).",
  "mod.inv_mul": "Multiplikatives Inverses via erweitertem Euklidischen Algorithmus. Existiert genau dann, wenn gcd(a, n) = 1.",
  "mod.subgroup": "Erzeugt die von g erzeugte zyklische Untergruppe in (Z/pZ)* und ermittelt deren Ordnung.",
  "mod.primitive_roots": "Listet alle primitiven Wurzeln modulo p (Erzeuger der vollen multiplikativen Gruppe).",
  "rsa.keygen":  "RSA-Schlüsselerzeugung aus zwei Primzahlen p, q und öffentlichem Exponenten e. Liefert n, φ(n) und privaten Schlüssel d.",
  "rsa.encrypt": "RSA-Verschlüsselung: c = m^e mod n.",
  "rsa.decrypt": "RSA-Entschlüsselung: m = c^d mod n.",
  "dh.exchange": "Diffie-Hellman-Schlüsselaustausch mit Generator g, Modul p und privaten Werten a, b. Zeigt beide Sichten und den gemeinsamen Schlüssel.",
  "elgamal.encrypt": "ElGamal-Verschlüsselung im (Z/pZ)*: (c1, c2) = (g^k, m · e^k) mod p.",
  "elgamal.decrypt": "ElGamal-Entschlüsselung: m = c2 · (c1^x)^(−1) mod p.",
  "shamir.three_pass": "Shamir Three-Pass-Protokoll: Nachricht m wird ohne vorher ausgetauschten Schlüssel zwischen Alice und Bob übertragen.",
  "ecc.add":    "Punktaddition P + Q auf der elliptischen Kurve y² = x³ + ax + b über F_p. Behandelt auch P = Q (Verdopplung) und O.",
  "ecc.scalar": "Skalarmultiplikation k · P auf einer elliptischen Kurve mittels Double-and-Add.",
  "rsa.sign":   "RSA-Signaturerzeugung: s = m^d mod n (Schulbuch-Variante ohne Padding).",
  "rsa.verify": "RSA-Signaturverifikation: prüft m ≡ s^e (mod n).",
  "elgamal.sign":   "ElGamal-Signaturerzeugung mit ephemerem k: (r, s) aus r = g^k mod p und s = k^(−1) · (m − d·r) mod (p−1).",
  "elgamal.verify": "ElGamal-Signaturverifikation: prüft g^m ≡ e^r · r^s (mod p).",
  "subst.encrypt": "Monoalphabetische Substitution: Klartext-Buchstaben werden über eine Permutation auf den Schlüsselalphabet abgebildet.",
  "subst.decrypt": "Inverse Substitution mit dem gegebenen Schlüssel.",
  "paillier.keygen":  "Paillier-Schlüsselerzeugung aus p, q. Bestimmt n = p·q, λ = lcm(p−1, q−1), wählt g und berechnet μ.",
  "paillier.encrypt": "Paillier-Verschlüsselung: c = g^m · r^n mod n². Additiv homomorph.",
  "paillier.decrypt": "Paillier-Entschlüsselung mit λ, μ: m = L(c^λ mod n²) · μ mod n.",
  "caesar.encrypt": "Caesar-Chiffre: jeder Buchstabe wird um k Stellen nach rechts verschoben. Erhält Groß-/Kleinschreibung und Satzzeichen.",
  "caesar.decrypt": "Caesar-Entschlüsselung: verschiebt jeden Buchstaben um k Stellen nach links (= Verschlüsselung mit −k).",
  "fiat_shamir.round": "Eine Runde des Fiat-Shamir-Identifikationsprotokolls: Commitment x = k² mod n, Challenge e ∈ {0,1}, Response y.",
  "bsgs": "Baby-Step-Giant-Step: löst diskreten Logarithmus h = g^x mod p in O(√p) mit Lookup-Tabelle.",
  "pollard_rho.factor": "Pollard-Rho-Faktorisierung: findet einen nicht-trivialen Faktor von n via Zyklus-Suche auf einer Pseudo-Zufallsfolge.",
  "pollard_rho.dlog":   "Pollard-Rho für diskrete Logarithmen mit drei Partitionen und Floyd-Zyklus-Erkennung.",
  "fermat.factor": "Fermat-Faktorisierung: sucht n = a² − b² = (a−b)(a+b). Effizient, wenn die Faktoren nahe beieinander liegen.",
  "transposition.decrypt": "Versucht mehrere Schlüssellängen für eine Spaltentransposition und zeigt die plausibelsten Entschlüsselungen.",
  "freq.analyze": "Buchstaben-Häufigkeitsanalyse: vergleicht beobachtete Häufigkeiten mit der erwarteten Verteilung der gewählten Sprache.",
  "pb.elgamal_mult_homomorph": "Demonstriert die multiplikative Homomorphismus-Eigenschaft von ElGamal anhand zweier Chiffrate.",
  "pb.rsa_priv_from_pub_y":    "Aufgabe: aus (n, e) und einem Geheimtext y den privaten Schlüssel d und den Klartext x bestimmen.",
  "pb.dh_k_from_g_p_alpha_beta": "Aufgabe: aus g, p sowie den öffentlichen Werten α, β den gemeinsamen DH-Schlüssel K rekonstruieren.",
  "pb.elgamal_d_from_kpub":    "Aufgabe: aus dem öffentlichen ElGamal-Schlüssel (p, g, e) den privaten Schlüssel d via DLog ermitteln.",
  "pb.rsa_check_d":            "Verifiziert, ob ein gegebener privater Schlüssel d zu (n, e) passt — optional mit Round-Trip-Test.",
  "pb.elgamal_sig_verify":     "Geführte Verifikation einer ElGamal-Signatur (m, r, s) gegen Kpub = (p, g, e).",
  "pb.rsa_pcap_decrypt":       "Klassische Klausuraufgabe: aus (n, e) und y (z. B. aus pcap) den Klartext bestimmen.",
  "pb.elgamal_pcap_decrypt":   "Aus ElGamal-Kpub und mitgeschnittenem (a, b) den Klartext m rekonstruieren.",
  "pb.dh_pcap_shared":         "Aus Mitschnitt-Werten (p, g, α, β) den gemeinsamen DH-Schlüssel ableiten; optional Geheimtext entschlüsseln.",
  "pb.paillier_add_homomorph": "Zeigt die additive Homomorphismus-Eigenschaft von Paillier: E(m1) · E(m2) entschlüsselt zu m1 + m2 mod n.",
};

let wasm = null;
let wasmError = null;
let searchIndex = [];

function escapeHtml(s) {
  return String(s)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

function highlightMath(escaped) {
  const tokens = [
    "\\bmod\\b", "\\bgcd\\b", "\\blcm\\b",
    "≡", "⁻¹", "²", "³", "λ", "μ", "≠", "→", "·"
  ];
  const re = new RegExp("(" + tokens.join("|") + ")", "g");
  return escaped.replace(re, '<span class="op">$1</span>');
}

// ────────── Pane (ein Modulkontext) ──────────
class Pane {
  constructor() {
    const tpl = document.getElementById("pane-template");
    this.el = tpl.content.firstElementChild.cloneNode(true);
    this.kind = "leaf";
    this.parent = null;
    this.breadcrumbEl = this.el.querySelector(".breadcrumb-path");
    this.termEl = this.el.querySelector(".terminal");
    this.formEl = this.el.querySelector(".prompt-form");
    this.inputEl = this.el.querySelector(".prompt-input");

    this.state = { mode: "menu", menu: "main", pending: null };
    this.history = [];
    this.historyIndex = -1;
    this.busy = false;
    this.lastTrace = null;

    this.formEl.addEventListener("submit", (e) => this.onSubmit(e));
    this.el.addEventListener("mousedown", () => setActive(this));
    this.inputEl.addEventListener("focus", () => setActive(this));
  }

  append(text, cls = "line") {
    const el = document.createElement("div");
    el.className = cls;
    el.textContent = text;
    this.termEl.appendChild(el);
    this.termEl.scrollTop = this.termEl.scrollHeight;
    return el;
  }

  appendHtml(html, cls = "line") {
    const el = document.createElement("div");
    el.className = cls;
    el.innerHTML = html;
    this.termEl.appendChild(el);
    this.termEl.scrollTop = this.termEl.scrollHeight;
    return el;
  }

  clearTerminal() { this.termEl.innerHTML = ""; }
  focus() { this.inputEl.focus({ preventScroll: true }); }

  setBreadcrumb(name) {
    const trail = ["cryputil"];
    if (name !== "main") trail.push(MENUS[name]?.title ?? name);
    this.breadcrumbEl.textContent = trail.join(" ▸ ");
  }

  divider(label) {
    const line = "──── " + label + " " + "─".repeat(Math.max(0, 58 - label.length));
    this.appendHtml(
      `<span class="section-divider">${escapeHtml(line.slice(0, 5))}</span>` +
      `<span class="section-title">${escapeHtml(label)}</span> ` +
      `<span class="section-divider">${escapeHtml(line.slice(5 + label.length + 1))}</span>`,
      "section-header"
    );
  }

  showMenu(name) {
    this.state.mode = "menu";
    this.state.menu = name;
    this.setBreadcrumb(name);
    const menu = MENUS[name];
    this.append("");
    this.append(menu.title, "section-title");
    menu.items.forEach((it, i) => this.append(`${i + 1}) ${it.label}`));
    this.append(`0) ${menu.backLabel ?? "Zurück"}`);
  }

  startCollecting(item) {
    this.state.mode = "collect";
    this.state.pending = { item, values: {}, idx: 0 };
    this.advanceStep();
  }

  advanceStep() {
    const p = this.state.pending;
    const steps = p.item.steps || [];
    while (p.idx < steps.length) {
      const step = steps[p.idx];
      if (step.type === "info") {
        this.append("");
        for (const ln of step.lines) this.append(ln, "info");
        p.idx++;
        continue;
      }
      if (step.type === "int" || step.type === "str") {
        this.promptStep(step);
        return;
      }
      if (step.type === "opt_int") {
        this.state.mode = "opt_yesno";
        this.append(`  ${step.question} [j/n]: `, "info");
        return;
      }
      p.idx++;
    }
    this.runCommand();
  }

  promptStep(step) {
    this.state.mode = "collect";
    this.append(`  ${step.label}: `, "info");
  }

  handleStepInput(value) {
    const p = this.state.pending;
    const step = p.item.steps[p.idx];
    if (step.type === "str") {
      p.values[step.key] = value;
      p.idx++;
      this.advanceStep();
      return;
    }
    if (step.type === "int") {
      if (!/^-?\d+$/.test(value.trim())) {
        this.append("    ungültige Ganzzahl", "err");
        this.promptStep(step);
        return;
      }
      const v = BigInt(value.trim());
      if (step.min !== undefined && v < BigInt(step.min)) {
        this.append(`    Wert muss >= ${step.min} sein.`, "err");
        this.promptStep(step);
        return;
      }
      p.values[step.key] = value.trim();
      p.idx++;
      this.advanceStep();
      return;
    }
  }

  handleOptYesNo(value) {
    const p = this.state.pending;
    const step = p.item.steps[p.idx];
    const v = value.trim().toLowerCase();
    if (["j", "ja", "y", "yes"].includes(v)) {
      this.state.mode = "collect";
      p.idx++;
      p.item.steps.splice(p.idx, 0, I(step.key, step.label, step.min));
      this.advanceStep();
      return;
    }
    if (["n", "nein", "no"].includes(v)) {
      this.state.mode = "collect";
      p.idx++;
      this.advanceStep();
      return;
    }
    this.append("    Bitte j oder n eingeben.", "err");
  }

  async runCommand() {
    const p = this.state.pending;
    const cmd = p.item.cmd;
    const params = p.values;
    this.state.pending = null;

    if (!wasm) {
      this.append("[error] WASM-Modul nicht geladen", "err");
      if (wasmError) this.append(`        ${wasmError}`, "err");
      this.showMenu(this.state.menu);
      return;
    }

    this.busy = true;
    try {
      const res = wasm.run(cmd, params);
      if (res.ok) {
        this.lastTrace = res.trace;
        this.renderSummary(res.trace);
        this.askShowSteps();
        this.busy = false;
        return;
      } else {
        this.append("", "");
        this.append(`Fehler: ${res.error}`, "err");
        this.append("");
      }
    } catch (e) {
      this.append(`Fehler: ${e.message || e}`, "err");
    } finally {
      this.busy = false;
    }
    this.showMenu(this.state.menu);
  }

  askShowSteps() {
    this.state.mode = "ask_show_steps";
    this.append("  Alle Schritte anzeigen? [j/n]: ", "info");
  }

  handleShowSteps(value) {
    const v = value.trim().toLowerCase();
    if (["j", "ja", "y", "yes"].includes(v)) {
      this.renderFull(this.lastTrace);
      this.showMenu(this.state.menu);
      return;
    }
    if (["n", "nein", "no"].includes(v)) {
      this.showMenu(this.state.menu);
      return;
    }
    this.append("    Bitte j oder n eingeben.", "err");
  }

  renderTable(table, indent) {
    const widths = table.headers.map((h) => [...h].length);
    for (const row of table.rows) {
      row.forEach((cell, i) => {
        const len = [...cell].length;
        if (len > widths[i]) widths[i] = len;
      });
    }
    const pad = (s, w) => s + " ".repeat(Math.max(0, w - [...s].length));
    const inner = widths.map((w) => "─".repeat(w + 2));
    const top    = indent + "┌" + inner.join("┬") + "┐";
    const mid    = indent + "├" + inner.join("┼") + "┤";
    const bottom = indent + "└" + inner.join("┴") + "┘";
    const cell = (s, w) => " " + pad(s, w) + " ";

    this.appendHtml(`<span class="table-border">${escapeHtml(top)}</span>`);
    this.appendHtml(
      `<span class="table-border">${escapeHtml(indent + "│")}</span>` +
      table.headers
        .map((h, i) => `<span class="table-header">${escapeHtml(cell(h, widths[i]))}</span>` +
                       `<span class="table-border">│</span>`)
        .join("")
    );
    this.appendHtml(`<span class="table-border">${escapeHtml(mid)}</span>`);
    table.rows.forEach((row, ri) => {
      const cls = "table-row " + (ri % 2 === 0 ? "even" : "odd");
      this.appendHtml(
        `<span class="table-border">${escapeHtml(indent + "│")}</span>` +
        row.map((c, i) =>
          `${escapeHtml(cell(c, widths[i] ?? 0))}<span class="table-border">│</span>`
        ).join(""),
        cls
      );
    });
    this.appendHtml(`<span class="table-border">${escapeHtml(bottom)}</span>`);
  }

  renderInputRow(k, v) {
    this.appendHtml(
      `<span class="arrow">▸</span><span class="key">${escapeHtml(k)}</span> = ${escapeHtml(v)}`,
      "input-row"
    );
  }

  renderResultRow(k, v) {
    this.appendHtml(
      `<span class="star">★</span>${escapeHtml(k)} = <strong>${escapeHtml(v)}</strong>`,
      "result-row"
    );
  }

  renderStepHeader(num, title) {
    this.appendHtml(
      `<span class="badge">[${escapeHtml(String(num))}]</span>` +
      `<span class="step-title">${escapeHtml(title)}</span>`,
      "step-header"
    );
  }

  renderStepLine(text) {
    const safe = highlightMath(escapeHtml(text));
    this.appendHtml(`    ${safe}`, "step-line");
  }

  renderSummary(trace) {
    this.append("");
    this.appendHtml(`Algorithmus: ${escapeHtml(trace.algorithm)}`, "algo-title");
    if (trace.result && trace.result.length) {
      this.append("");
      this.divider("Ergebnis");
      for (const [k, v] of trace.result) this.renderResultRow(k, v);
    }
    this.append("");
  }

  renderFull(trace) {
    this.append("");
    this.appendHtml(`Algorithmus: ${escapeHtml(trace.algorithm)}`, "algo-title");
    if (trace.inputs && trace.inputs.length) {
      this.append("");
      this.divider("Eingaben");
      for (const [k, v] of trace.inputs) this.renderInputRow(k, v);
    }
    if (trace.steps && trace.steps.length) {
      this.append("");
      this.divider("Schritte");
      for (const step of trace.steps) {
        this.append("");
        this.renderStepHeader(step.number, step.title);
        for (const ln of step.lines || []) this.renderStepLine(ln);
        if (step.table) {
          this.append("");
          this.renderTable(step.table, "    ");
        }
      }
    }
    if (trace.result && trace.result.length) {
      this.append("");
      this.divider("Ergebnis");
      for (const [k, v] of trace.result) this.renderResultRow(k, v);
    }
    this.append("");
  }

  showHelp() {
    this.append("Bedienung:", "info");
    this.append("  Auswahl per Nummer, 0 für Zurück bzw. Beenden.", "info");
    this.append("  Parameter werden einzeln abgefragt; ungültige Werte werden erneut erfragt.", "info");
    this.append("  Nach jedem Ergebnis wird gefragt, ob alle Schritte angezeigt werden sollen.", "info");
    this.append("  Shortcuts: [/] Suche  [Esc] Zurück  [Ctrl+L] Clear", "info");
    this.append("  Split: [Ctrl+Shift+B] = Prefix, dann:  [%] vertikal  [\"] horizontal  [Pfeile] Pane wechseln  [x] schließen  [o] nächste  [z] Zoom", "info");
    this.append("  Sonderbefehle: help, clear, menu, version", "info");
  }

  abortPending() {
    this.state.pending = null;
    this.state.mode = "menu";
  }

  handleMenuSelection(value) {
    const choice = value.trim();
    const menu = MENUS[this.state.menu];
    if (choice === "0") {
      if (this.state.menu === "main") this.append("Bye.", "info");
      else this.showMenu("main");
      return;
    }
    const idx = parseInt(choice, 10);
    if (!Number.isInteger(idx) || idx < 1 || idx > menu.items.length) {
      this.append("    ungültige Auswahl", "err");
      this.showMenu(this.state.menu);
      return;
    }
    const item = menu.items[idx - 1];
    if (item.goto) {
      this.showMenu(item.goto);
    } else if (item.action === "showLast") {
      if (this.lastTrace) this.renderFull(this.lastTrace);
      else this.append("Keine vorherige Berechnung.", "warn");
      this.showMenu(this.state.menu);
    } else if (item.cmd) {
      const itemCopy = { ...item, steps: [...(item.steps || [])] };
      if (!itemCopy.steps.length) {
        this.state.pending = { item: itemCopy, values: {}, idx: 0 };
        this.runCommand();
      } else {
        this.startCollecting(itemCopy);
      }
    }
  }

  handleSpecial(value) {
    const v = value.trim().toLowerCase();
    if (v === "help") { this.showHelp(); return true; }
    if (v === "clear") { this.clearTerminal(); return true; }
    if (v === "menu") { this.state.pending = null; this.showMenu("main"); return true; }
    if (v === "version") { this.append(wasm ? wasm.version() : "wasm not loaded", "info"); return true; }
    return false;
  }

  dispatchInput(value) {
    if (this.state.mode === "menu" && this.handleSpecial(value)) return;
    if (this.state.mode === "ask_show_steps") return this.handleShowSteps(value);
    if (this.state.mode === "opt_yesno") return this.handleOptYesNo(value);
    if (this.state.mode === "collect") return this.handleStepInput(value);
    return this.handleMenuSelection(value);
  }

  onSubmit(e) {
    e.preventDefault();
    if (this.busy) return;
    const value = this.inputEl.value;
    this.inputEl.value = "";
    this.append(`cryputil> ${value}`, "echo");
    if (value.trim()) {
      this.history.push(value);
      this.historyIndex = this.history.length;
    }
    this.dispatchInput(value);
  }

  historyUp() {
    if (this.historyIndex > 0) this.historyIndex--;
    this.inputEl.value = this.history[this.historyIndex] ?? "";
  }

  historyDown() {
    if (this.historyIndex < this.history.length - 1) {
      this.historyIndex++;
      this.inputEl.value = this.history[this.historyIndex];
    } else {
      this.historyIndex = this.history.length;
      this.inputEl.value = "";
    }
  }

  doEscape() {
    if (this.state.mode === "menu") {
      if (this.state.menu !== "main") this.showMenu("main");
    } else {
      this.abortPending();
      this.append("  (abgebrochen)", "warn");
      this.showMenu(this.state.menu);
    }
  }
}

// ────────── Pane-Tree ──────────
let tree = null;       // root node: Pane (leaf) oder Split
let activeLeaf = null;
let zoomed = null;     // wenn gesetzt: nur diese Pane wird gerendert

function makeSplit(direction, a, b) {
  const node = { kind: "split", direction, children: [a, b], parent: null, el: null };
  a.parent = node;
  b.parent = node;
  return node;
}

function buildNode(node) {
  if (node.kind === "leaf") return node.el;
  const wrap = document.createElement("div");
  wrap.className = "pane-split " + node.direction;
  node.el = wrap;
  for (const child of node.children) wrap.appendChild(buildNode(child));
  return wrap;
}

function renderTree() {
  paneRoot.innerHTML = "";
  if (zoomed) {
    paneRoot.appendChild(zoomed.el);
  } else {
    paneRoot.appendChild(buildNode(tree));
  }
  updateActiveMarker();
}

function updateActiveMarker() {
  paneRoot.querySelectorAll(".pane-leaf").forEach((el) => el.classList.remove("active"));
  if (activeLeaf && (allLeaves(tree).length > 1 || zoomed)) {
    activeLeaf.el.classList.add("active");
  }
}

function setActive(pane) {
  if (activeLeaf === pane) return;
  activeLeaf = pane;
  updateActiveMarker();
}

function focusActivePane() {
  if (paletteState.open) return;
  if (activeLeaf) activeLeaf.focus();
}

function allLeaves(node, out = []) {
  if (!node) return out;
  if (node.kind === "leaf") out.push(node);
  else for (const c of node.children) allLeaves(c, out);
  return out;
}

function firstLeaf(node) {
  while (node.kind === "split") node = node.children[0];
  return node;
}

function splitActive(direction) {
  if (zoomed) zoomed = null;
  const old = activeLeaf;
  const fresh = new Pane();
  const split = makeSplit(direction, old, fresh);
  if (old.parent) {
    const idx = old.parent.children.indexOf(old);
    old.parent.children[idx] = split;
    split.parent = old.parent;
  } else {
    tree = split;
  }
  activeLeaf = fresh;
  renderTree();
  fresh.append(BANNER, "ascii");
  if (!wasm && wasmError) {
    fresh.append("[warn] WASM-Bundle nicht geladen.", "warn");
  }
  fresh.showMenu("main");
  focusActivePane();
}

function closeActive() {
  if (zoomed) zoomed = null;
  const old = activeLeaf;
  if (!old.parent) {
    old.append("  (letzte Pane — kann nicht geschlossen werden)", "warn");
    return;
  }
  const parent = old.parent;
  const sibling = parent.children[0] === old ? parent.children[1] : parent.children[0];
  if (parent.parent) {
    const idx = parent.parent.children.indexOf(parent);
    parent.parent.children[idx] = sibling;
    sibling.parent = parent.parent;
  } else {
    tree = sibling;
    sibling.parent = null;
  }
  activeLeaf = firstLeaf(sibling);
  renderTree();
  focusActivePane();
}

function focusDirection(dir) {
  if (zoomed) return;
  const leaves = allLeaves(tree).filter((l) => l !== activeLeaf);
  if (!leaves.length) return;
  const ar = activeLeaf.el.getBoundingClientRect();
  const acx = ar.left + ar.width / 2;
  const acy = ar.top + ar.height / 2;
  let best = null;
  let bestDist = Infinity;
  for (const l of leaves) {
    const r = l.el.getBoundingClientRect();
    const cx = r.left + r.width / 2;
    const cy = r.top + r.height / 2;
    const dx = cx - acx;
    const dy = cy - acy;
    let primary, secondary;
    if (dir === "Right" || dir === "Left") {
      if ((dir === "Right" && dx <= 0) || (dir === "Left" && dx >= 0)) continue;
      primary = Math.abs(dx);
      secondary = Math.abs(dy);
    } else {
      if ((dir === "Down" && dy <= 0) || (dir === "Up" && dy >= 0)) continue;
      primary = Math.abs(dy);
      secondary = Math.abs(dx);
    }
    const dist = primary + secondary * 2;
    if (dist < bestDist) { bestDist = dist; best = l; }
  }
  if (best) { setActive(best); focusActivePane(); }
}

function cycleNext() {
  if (zoomed) return;
  const leaves = allLeaves(tree);
  if (leaves.length < 2) return;
  const i = leaves.indexOf(activeLeaf);
  setActive(leaves[(i + 1) % leaves.length]);
  focusActivePane();
}

function toggleZoom() {
  if (allLeaves(tree).length < 2) return;
  zoomed = zoomed ? null : activeLeaf;
  renderTree();
  focusActivePane();
}

// ────────── Prefix (tmux Ctrl+B) ──────────
let prefixActive = false;
let prefixTimer = null;

function startPrefix() {
  prefixActive = true;
  prefixHint.textContent = "PREFIX (Ctrl+Shift+B)…";
  prefixHint.classList.add("visible");
  clearTimeout(prefixTimer);
  prefixTimer = setTimeout(endPrefix, 2000);
}

function endPrefix() {
  prefixActive = false;
  prefixHint.classList.remove("visible");
  clearTimeout(prefixTimer);
}

function handlePrefixCommand(e) {
  // Modifier-Only-Events ignorieren (z.B. der Ctrl-Release zwischen Prefix und Taste)
  if (e.key === "Control" || e.key === "Meta" || e.key === "Shift" || e.key === "Alt") return;
  endPrefix();
  const key = e.key;
  if (key === "%") splitActive("row");
  else if (key === '"') splitActive("column");
  else if (key === "ArrowLeft")  focusDirection("Left");
  else if (key === "ArrowRight") focusDirection("Right");
  else if (key === "ArrowUp")    focusDirection("Up");
  else if (key === "ArrowDown")  focusDirection("Down");
  else if (key === "x") closeActive();
  else if (key === "o") cycleNext();
  else if (key === "z") toggleZoom();
  else if (key === "Escape") { /* cancel */ }
  else return;
  e.preventDefault();
}

// ────────── Befehls-Palette ──────────
function buildSearchIndex() {
  const idx = [];
  for (const [menuKey, menu] of Object.entries(MENUS)) {
    if (menuKey === "main") continue;
    for (const item of menu.items) {
      if (!item.cmd) continue;
      idx.push({
        kind: "cmd",
        label: item.label,
        cmd: item.cmd,
        menuKey,
        menuTitle: menu.title,
        item,
      });
    }
  }
  return idx;
}

const paletteState = {
  open: false,
  results: [],
  selected: 0,
  cwd: "main",
  mode: "browse",
};

function openPalette() {
  if (paletteState.open) return;
  paletteState.open = true;
  palette.classList.remove("hidden");
  palette.setAttribute("aria-hidden", "false");
  paletteInput.value = "";
  paletteState.cwd = "main";
  refreshPalette("");
  setTimeout(() => paletteInput.focus(), 0);
}

function closePalette() {
  if (!paletteState.open) return;
  paletteState.open = false;
  palette.classList.add("hidden");
  palette.setAttribute("aria-hidden", "true");
  focusActivePane();
}

function fuzzyMatch(haystack, needle) {
  haystack = haystack.toLowerCase();
  needle = needle.toLowerCase().trim();
  if (!needle) return 0;
  let hi = 0, score = 0, lastMatch = -1;
  for (const ch of needle) {
    const found = haystack.indexOf(ch, hi);
    if (found < 0) return -1;
    score += found - hi;
    if (lastMatch >= 0 && found === lastMatch + 1) score -= 2;
    lastMatch = found;
    hi = found + 1;
  }
  return score;
}

function browseEntries(menuKey) {
  const menu = MENUS[menuKey];
  if (!menu) return [];
  const out = [];
  if (menuKey !== "main") out.push({ kind: "up", label: ".. (Ebene zurück)" });
  for (const item of menu.items) {
    if (item.goto) {
      out.push({ kind: "folder", label: item.label, target: item.goto });
    } else if (item.action) {
      out.push({ kind: "action", label: item.label, action: item.action });
    } else if (item.cmd) {
      out.push({
        kind: "cmd",
        label: item.label,
        cmd: item.cmd,
        menuKey,
        menuTitle: menu.title,
        item,
      });
    }
  }
  return out;
}

function refreshPalette(query) {
  const q = query.trim();
  if (!q) {
    paletteState.mode = "browse";
    paletteState.results = browseEntries(paletteState.cwd);
  } else {
    paletteState.mode = "search";
    paletteState.results = searchIndex
      .map((it) => {
        const sLabel = fuzzyMatch(it.label, q);
        const sCmd = fuzzyMatch(it.cmd, q);
        const sMenu = fuzzyMatch(it.menuTitle, q);
        const candidates = [sLabel, sCmd, sMenu].filter((s) => s >= 0);
        if (!candidates.length) return null;
        return { it, score: Math.min(...candidates) };
      })
      .filter(Boolean)
      .sort((a, b) => a.score - b.score)
      .slice(0, 30)
      .map((r) => r.it);
  }
  paletteState.selected = 0;
  updatePaletteCwd();
  renderPalette();
}

function updatePaletteCwd() {
  const trail = ["cryputil"];
  if (paletteState.cwd !== "main") trail.push(MENUS[paletteState.cwd]?.title ?? paletteState.cwd);
  paletteCwd.innerHTML =
    (paletteState.mode === "search" ? "Suche in allen Befehlen" : "") +
    (paletteState.mode === "browse"
      ? trail
          .map((p, i) =>
            i === trail.length - 1
              ? `<span class="crumb-here">${escapeHtml(p)}</span>`
              : escapeHtml(p)
          )
          .join(" ▸ ")
      : "");
}

function kindGlyph(kind) {
  if (kind === "folder") return "📁";
  if (kind === "up") return "↩";
  if (kind === "action") return "✦";
  return "›";
}

function describeEntry(entry) {
  if (!entry) return null;
  if (entry.kind === "up") {
    return { title: "Eine Ebene zurück", body: "Springt im Browse-Modus zurück zur übergeordneten Kategorie." };
  }
  if (entry.kind === "folder") {
    return {
      title: entry.label,
      body: FOLDER_DESCRIPTIONS[entry.target] ?? "Untermenü mit weiteren Befehlen.",
    };
  }
  if (entry.kind === "action") {
    if (entry.action === "showLast") {
      return { title: entry.label, body: "Zeigt alle Schritte und Tabellen der letzten ausgeführten Berechnung erneut an." };
    }
    return { title: entry.label, body: "" };
  }
  if (entry.kind === "cmd") {
    return {
      title: entry.label,
      cmd: entry.cmd,
      menuTitle: entry.menuTitle,
      body: CMD_DESCRIPTIONS[entry.cmd] ?? "Kein Beschreibungstext hinterlegt.",
    };
  }
  return null;
}

function updatePaletteInfo() {
  if (!paletteInfo) return;
  const entry = paletteState.results[paletteState.selected];
  const info = describeEntry(entry);
  if (!info) {
    paletteInfo.innerHTML = `<div class="palette-info-empty">Eintrag wählen, um eine Erklärung zu sehen…</div>`;
    return;
  }
  let html = `<h4>${escapeHtml(info.title)}</h4>`;
  if (info.cmd) html += `<span class="palette-info-cmd">${escapeHtml(info.cmd)}</span>`;
  if (info.body) html += `<p>${escapeHtml(info.body)}</p>`;
  if (info.menuTitle) html += `<div class="palette-info-meta">Kategorie: ${escapeHtml(info.menuTitle)}</div>`;
  paletteInfo.innerHTML = html;
}

function renderPalette() {
  paletteList.innerHTML = "";
  if (!paletteState.results.length) {
    const li = document.createElement("li");
    li.className = "empty";
    li.textContent = "Keine Treffer";
    paletteList.appendChild(li);
    updatePaletteInfo();
    return;
  }
  paletteState.results.forEach((it, i) => {
    const li = document.createElement("li");
    const classes = [];
    if (i === paletteState.selected) classes.push("active");
    if (it.kind === "folder") classes.push("folder");
    if (it.kind === "up") classes.push("up");
    li.className = classes.join(" ");

    const kind = document.createElement("span");
    kind.className = "palette-kind";
    kind.textContent = kindGlyph(it.kind);
    li.appendChild(kind);

    const label = document.createElement("span");
    label.textContent = it.label;
    li.appendChild(label);

    if (it.kind === "cmd" && paletteState.mode === "search") {
      const path = document.createElement("span");
      path.className = "palette-path";
      path.textContent = "  ▸ " + it.menuTitle;
      li.appendChild(path);
    }
    if (it.kind === "cmd") {
      const cmd = document.createElement("span");
      cmd.className = "palette-cmd";
      cmd.textContent = it.cmd;
      li.appendChild(cmd);
    }

    li.addEventListener("click", () => {
      paletteState.selected = i;
      runPaletteSelection();
    });
    li.addEventListener("mouseenter", () => {
      paletteState.selected = i;
      paletteList.querySelectorAll("li.active").forEach((el) => el.classList.remove("active"));
      li.classList.add("active");
      updatePaletteInfo();
    });
    paletteList.appendChild(li);
  });
  updatePaletteInfo();
}

function movePaletteSelection(delta) {
  if (!paletteState.results.length) return;
  paletteState.selected =
    (paletteState.selected + delta + paletteState.results.length) %
    paletteState.results.length;
  renderPalette();
  const active = paletteList.querySelector("li.active");
  if (active) active.scrollIntoView({ block: "nearest" });
}

function paletteGoUp() {
  if (paletteState.mode === "search") {
    paletteInput.value = "";
    refreshPalette("");
    return;
  }
  if (paletteState.cwd !== "main") {
    paletteState.cwd = "main";
    refreshPalette("");
  }
}

function runPaletteSelection() {
  const pick = paletteState.results[paletteState.selected];
  if (!pick) return;

  if (pick.kind === "up") { paletteGoUp(); return; }
  if (pick.kind === "folder") {
    paletteState.cwd = pick.target;
    paletteInput.value = "";
    refreshPalette("");
    return;
  }

  const p = activeLeaf;
  if (pick.kind === "action") {
    closePalette();
    if (pick.action === "showLast") {
      if (p.lastTrace) p.renderFull(p.lastTrace);
      else p.append("Keine vorherige Berechnung.", "warn");
      p.showMenu(p.state.menu);
    }
    return;
  }

  closePalette();
  p.abortPending();
  p.state.menu = pick.menuKey;
  p.setBreadcrumb(pick.menuKey);
  p.append("");
  p.append(`> ${pick.menuTitle} / ${pick.label}`, "echo");
  const itemCopy = { ...pick.item, steps: [...(pick.item.steps || [])] };
  if (!itemCopy.steps.length) {
    p.state.pending = { item: itemCopy, values: {}, idx: 0 };
    p.runCommand();
  } else {
    p.startCollecting(itemCopy);
  }
}

// ────────── Globale Shortcuts ──────────
function isPrintableKey(e) {
  if (e.ctrlKey || e.metaKey || e.altKey) return false;
  return e.key.length === 1;
}

function onGlobalKeydown(e) {
  // Palette hat eigenen Handler auf paletteInput
  if (paletteState.open) return;

  if (prefixActive) {
    handlePrefixCommand(e);
    return;
  }

  // tmux-Prefix (Ctrl+Shift+B; Ctrl+B kollidiert mit emacs-Cursor-Bindings in Chrome auf macOS)
  if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key.toLowerCase() === "b") {
    startPrefix();
    e.preventDefault();
    return;
  }

  const p = activeLeaf;
  if (!p) return;

  const inInput = e.target === p.inputEl;

  if (e.key === "ArrowUp" && inInput) {
    p.historyUp();
    e.preventDefault();
  } else if (e.key === "ArrowDown" && inInput) {
    p.historyDown();
    e.preventDefault();
  } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "l") {
    p.clearTerminal();
    e.preventDefault();
  } else if (e.key === "Escape") {
    p.doEscape();
    e.preventDefault();
  } else if (e.key === "/" && (!inInput || !p.inputEl.value) && p.state.mode === "menu") {
    openPalette();
    e.preventDefault();
  } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
    openPalette();
    e.preventDefault();
  } else if (!inInput && isPrintableKey(e)) {
    // User tippt, ohne dass Input fokussiert ist — Fokus zurückholen.
    p.focus();
  }
}

function onPaletteKeydown(e) {
  if (e.key === "Escape") {
    closePalette();
    e.preventDefault();
  } else if (e.key === "ArrowDown") {
    movePaletteSelection(1);
    e.preventDefault();
  } else if (e.key === "ArrowUp") {
    movePaletteSelection(-1);
    e.preventDefault();
  } else if (e.key === "Enter") {
    runPaletteSelection();
    e.preventDefault();
  } else if (e.key === "Backspace" && !paletteInput.value) {
    paletteGoUp();
    e.preventDefault();
  }
}

async function loadWasm() {
  try {
    const mod = await import("./pkg/cryputil_wasm.js");
    await mod.default();
    wasm = mod;
    for (const l of allLeaves(tree)) l.append("[ready]", "info");
  } catch (e) {
    wasmError = e.message || String(e);
    for (const l of allLeaves(tree)) {
      l.append("[warn] WASM-Bundle nicht gefunden – nur help/clear verfügbar.", "warn");
      l.append("       Build: wasm-pack build --target web --out-dir ../../frontend/pkg crates/wasm", "warn");
    }
  }
  for (const l of allLeaves(tree)) l.showMenu("main");
}

document.addEventListener("DOMContentLoaded", () => {
  searchIndex = buildSearchIndex();

  const first = new Pane();
  tree = first;
  activeLeaf = first;
  renderTree();
  first.append(BANNER, "ascii");
  loadWasm();

  document.addEventListener("keydown", onGlobalKeydown);
  paletteInput.addEventListener("input", (e) => refreshPalette(e.target.value));
  paletteInput.addEventListener("keydown", onPaletteKeydown);
  palette.addEventListener("click", (e) => { if (e.target === palette) closePalette(); });

  document.addEventListener("click", (e) => {
    if (paletteState.open) return;
    if (e.target.closest(".palette-box")) return;
    focusActivePane();
  });

  window.addEventListener("focus", focusActivePane);
  window.addEventListener("resize", focusActivePane);

  const bindToolbar = (id, fn) => {
    const el = document.getElementById(id);
    if (!el) return;
    el.addEventListener("click", (e) => {
      e.preventDefault();
      fn();
      focusActivePane();
    });
  };
  bindToolbar("btn-split-v", () => splitActive("row"));
  bindToolbar("btn-split-h", () => splitActive("column"));
  bindToolbar("btn-cycle", cycleNext);
  bindToolbar("btn-zoom", toggleZoom);
  bindToolbar("btn-close", closeActive);

  first.focus();
});
