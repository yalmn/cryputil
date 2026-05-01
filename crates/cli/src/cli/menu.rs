use crate::cli::prompts::{ask_int, ask_int_min, ask_yes_no, read_line};
use crate::render::text;
use cryputil_core::algorithms::{
    diffie_hellman,
    ecc::{self, Point},
    elgamal, elgamal_signature, fiat_shamir, rsa, rsa_signature, shamir_three_pass,
};
use cryputil_core::analysis::{
    bsgs, columnar_transposition, fermat_factorization, pollard_rho_dlog, pollard_rho_factor,
};
use cryputil_core::core::error::CalcResult;
use cryputil_core::core::trace::Trace;
use cryputil_core::modulo::{cyclic_groups, inverse_additive, inverse_multiplicative, operations};
use cryputil_core::playbooks::{
    dh_geg_g_p_alpha_beta_ges_k, dh_ges_schluessel, elgamal_geg_kpub_ges_kpriv,
    elgamal_geg_kpub_ges_priv_verf, elgamal_geg_sig_kpub_ges_legitsig, elgamal_mult_homomorph,
    rsa_geg_kpub_ges_kpriv_plain, rsa_geg_kpub_kpriv_encm_ges_kpriv_to_kpub,
    rsa_geg_pub_enc_ges_priv_plain,
};

// Input: Trace-Ergebnis
// Calc:  bei Erfolg fragen, ob Schritte angezeigt werden, sonst Fehler ausgeben
// Output: Option<Trace>
pub fn handle(result: CalcResult<Trace>) -> Option<Trace> {
    match result {
        Ok(trace) => {
            text::render_summary(&trace);
            if ask_yes_no("Alle Schritte anzeigen?") {
                text::render(&trace);
            }
            Some(trace)
        }
        Err(e) => {
            text::render_error(&e);
            None
        }
    }
}

// Input: -
// Calc:  Hauptmenü darstellen und Auswahl zurückgeben
// Output: Auswahl als String
pub fn show_main() -> String {
    println!();
    println!("Crypto Lab");
    println!();
    println!("1) Modulo-Arithmetik");
    println!("2) Kryptografische Verfahren");
    println!("3) Identifikationsprotokolle");
    println!("4) Kryptoanalyse");
    println!("5) Playbooks");
    println!("6) Letzte Schritte anzeigen");
    println!("0) Beenden");
    read_line("Auswahl: ")
}

// Input: -
// Calc:  Submenü Modulo
// Output: Trace oder None
pub fn modulo_menu() -> Option<Trace> {
    println!();
    println!("Modulo-Arithmetik");
    println!("1) Addition");
    println!("2) Subtraktion");
    println!("3) Multiplikation");
    println!("4) Exponentiation");
    println!("5) Additives Inverses");
    println!("6) Multiplikatives Inverses");
    println!("7) Zyklische Untergruppe");
    println!("8) Primitive Wurzeln");
    println!("0) Zurück");
    let choice = read_line("Auswahl: ");
    match choice.as_str() {
        "1" => {
            let a = ask_int("a");
            let b = ask_int("b");
            let n = ask_int_min("n", 1);
            handle(operations::add(a, b, n))
        }
        "2" => {
            let a = ask_int("a");
            let b = ask_int("b");
            let n = ask_int_min("n", 1);
            handle(operations::sub(a, b, n))
        }
        "3" => {
            let a = ask_int("a");
            let b = ask_int("b");
            let n = ask_int_min("n", 1);
            handle(operations::mul(a, b, n))
        }
        "4" => {
            let a = ask_int("a");
            let e = ask_int_min("e", 0);
            let n = ask_int_min("n", 1);
            handle(operations::pow(a, e, n))
        }
        "5" => {
            let a = ask_int("a");
            let n = ask_int_min("n", 1);
            handle(inverse_additive::additive_inverse(a, n))
        }
        "6" => {
            let a = ask_int("a");
            let n = ask_int_min("n", 1);
            handle(inverse_multiplicative::multiplicative_inverse(a, n))
        }
        "7" => {
            let g = ask_int("g");
            let p = ask_int_min("p (prim)", 2);
            handle(cyclic_groups::subgroup(g, p))
        }
        "8" => {
            let p = ask_int_min("p (prim)", 2);
            handle(cyclic_groups::primitive_roots(p))
        }
        _ => None,
    }
}

// Input: -
// Calc:  Submenü kryptografische Verfahren
// Output: Trace oder None
pub fn crypto_menu() -> Option<Trace> {
    println!();
    println!("Kryptografische Verfahren");
    println!("1) RSA-Schlüsselerzeugung");
    println!("2) RSA-Verschlüsselung");
    println!("3) RSA-Entschlüsselung");
    println!("4) Diffie-Hellman");
    println!("5) ElGamal-Verschlüsselung");
    println!("6) ElGamal-Entschlüsselung");
    println!("7) Shamir Three-Pass");
    println!("8) ECC: Punktaddition");
    println!("9) ECC: Skalarmultiplikation");
    println!("10) RSA-Signatur: Erzeugung");
    println!("11) RSA-Signatur: Verifikation");
    println!("12) ElGamal-Signatur: Erzeugung");
    println!("13) ElGamal-Signatur: Verifikation");
    println!("0) Zurück");
    let choice = read_line("Auswahl: ");
    match choice.as_str() {
        "1" => {
            let p = ask_int_min("p (prim)", 2);
            let q = ask_int_min("q (prim)", 2);
            let e = ask_int_min("e", 2);
            handle(rsa::keygen(p, q, e))
        }
        "2" => {
            let n = ask_int_min("n", 2);
            let e = ask_int_min("e", 1);
            let m = ask_int_min("m", 0);
            handle(rsa::encrypt(n, e, m))
        }
        "3" => {
            let n = ask_int_min("n", 2);
            let d = ask_int_min("d", 1);
            let c = ask_int_min("c", 0);
            handle(rsa::decrypt(n, d, c))
        }
        "4" => {
            let p = ask_int_min("p (prim)", 2);
            let g = ask_int_min("g", 2);
            let a = ask_int_min("a (privat A)", 1);
            let b = ask_int_min("b (privat B)", 1);
            handle(diffie_hellman::key_exchange(p, g, a, b))
        }
        "5" => {
            let p = ask_int_min("p (prim)", 2);
            let g = ask_int_min("g", 2);
            let x = ask_int_min("x (privat)", 1);
            let m = ask_int_min("m", 0);
            let k = ask_int_min("k (ephemeral)", 1);
            handle(elgamal::encrypt(p, g, x, m, k))
        }
        "6" => {
            let p = ask_int_min("p (prim)", 2);
            let x = ask_int_min("x (privat)", 1);
            let c1 = ask_int_min("c1", 0);
            let c2 = ask_int_min("c2", 0);
            handle(elgamal::decrypt(p, x, c1, c2))
        }
        "7" => {
            let p = ask_int_min("p (prim)", 2);
            let m = ask_int_min("m", 0);
            let a = ask_int_min("a (Alice)", 1);
            let b = ask_int_min("b (Bob)", 1);
            handle(shamir_three_pass::run(p, m, a, b))
        }
        "8" => {
            let a = ask_int("a");
            let b = ask_int("b");
            let p = ask_int_min("p (prim)", 2);
            let px = ask_int("P.x");
            let py = ask_int("P.y");
            let qx = ask_int("Q.x");
            let qy = ask_int("Q.y");
            handle(ecc::add_trace(
                a,
                b,
                p,
                Point::Affine(px, py),
                Point::Affine(qx, qy),
            ))
        }
        "9" => {
            let a = ask_int("a");
            let b = ask_int("b");
            let p = ask_int_min("p (prim)", 2);
            let k = ask_int_min("k", 0);
            let px = ask_int("P.x");
            let py = ask_int("P.y");
            handle(ecc::scalar_trace(a, b, p, k, Point::Affine(px, py)))
        }
        "10" => {
            let n = ask_int_min("n", 2);
            let d = ask_int_min("d (privat)", 1);
            let m = ask_int_min("m (Nachricht)", 0);
            handle(rsa_signature::sign(n, d, m))
        }
        "11" => {
            let n = ask_int_min("n", 2);
            let e = ask_int_min("e (öffentlich)", 1);
            let m = ask_int_min("m (Nachricht)", 0);
            let s = ask_int_min("s (Signatur)", 0);
            handle(rsa_signature::verify(n, e, m, s))
        }
        "12" => {
            let p = ask_int_min("p (prim)", 2);
            let g = ask_int_min("g", 2);
            let d = ask_int_min("d (privat)", 1);
            let m = ask_int_min("m (Nachricht)", 0);
            let k = ask_int_min("k (ephemeral)", 1);
            handle(elgamal_signature::sign(p, g, d, m, k))
        }
        "13" => {
            let p = ask_int_min("p (prim)", 2);
            let g = ask_int_min("g", 2);
            let e = ask_int_min("e (öffentlich)", 1);
            let m = ask_int_min("m (Nachricht)", 0);
            let r = ask_int_min("r", 0);
            let s = ask_int_min("s", 0);
            handle(elgamal_signature::verify(p, g, e, m, r, s))
        }
        _ => None,
    }
}

// Input: -
// Calc:  Submenü Identifikationsprotokolle
// Output: Trace oder None
pub fn ident_menu() -> Option<Trace> {
    println!();
    println!("Identifikationsprotokolle");
    println!("1) Fiat-Shamir (eine Runde)");
    println!("0) Zurück");
    let choice = read_line("Auswahl: ");
    match choice.as_str() {
        "1" => {
            let n = ask_int_min("n", 2);
            let s = ask_int_min("s (Geheimnis)", 1);
            let k = ask_int_min("k (Commitment-Zufall)", 1);
            let e = ask_int_min("e (0 oder 1)", 0);
            handle(fiat_shamir::round(n, s, k, e))
        }
        _ => None,
    }
}

// Input: -
// Calc:  Submenü Kryptoanalyse
// Output: Trace oder None
pub fn analysis_menu() -> Option<Trace> {
    println!();
    println!("Kryptoanalyse");
    println!("1) Baby-Step-Giant-Step");
    println!("2) Pollard-Rho (Faktorisierung)");
    println!("3) Pollard-Rho (diskreter Logarithmus)");
    println!("4) Fermat-Faktorisierung");
    println!("5) Spaltentransposition (Varianten)");
    println!("0) Zurück");
    let choice = read_line("Auswahl: ");
    match choice.as_str() {
        "1" => {
            let g = ask_int_min("g", 2);
            let h = ask_int_min("h", 1);
            let p = ask_int_min("p (prim)", 2);
            handle(bsgs::solve(g, h, p))
        }
        "2" => {
            let n = ask_int_min("n", 2);
            handle(pollard_rho_factor::factor(n))
        }
        "3" => {
            let g = ask_int_min("g", 2);
            let h = ask_int_min("h", 1);
            let p = ask_int_min("p (prim)", 2);
            handle(pollard_rho_dlog::solve(g, h, p))
        }
        "4" => {
            let n = ask_int_min("n", 3);
            handle(fermat_factorization::factor(n))
        }
        "5" => {
            let text = read_line("  Geheimtext: ");
            handle(columnar_transposition::decrypt(&text))
        }
        _ => None,
    }
}

// Input: -
// Calc:  Submenü Playbooks
// Output: Trace oder None
pub fn playbooks_menu() -> Option<Trace> {
    println!();
    println!("Playbooks");
    println!("1) ElGamal: Multiplikativer Homomorphismus");
    println!("2) RSA: aus (n, e) und y → d und Klartext");
    println!("3) Diffie-Hellman: aus g, p, α, β → K");
    println!("4) ElGamal: aus Kpub = (p, g, e) → privater Schlüssel d");
    println!("5) RSA: passt d zu Kpub = (n, e)?");
    println!("6) ElGamal-Signatur: Verifikation");
    println!("7) RSA: aus Kpub = (n, e) und y → d und Klartext (Wireshark/pcap)");
    println!("8) ElGamal: aus Kpub = (p, g, e) und (a, b) → d und Klartext (Wireshark/pcap)");
    println!("9) Diffie-Hellman: gemeinsamer Schlüssel aus Mitschnitt (Wireshark/pcap)");
    println!("0) Zurück");
    let choice = read_line("Auswahl: ");
    match choice.as_str() {
        "2" => {
            println!();
            println!("RSA: gegeben Kpub = (n, e) und Geheimtext y, gesucht d und x.");
            let n = ask_int_min("n", 2);
            let e = ask_int_min("e", 1);
            let y = ask_int_min("y (Geheimtext)", 0);
            let phi = if ask_yes_no("phi(n) bekannt (für Verifikation)?") {
                Some(ask_int_min("phi(n)", 1))
            } else {
                None
            };
            handle(rsa_geg_pub_enc_ges_priv_plain::run(n, e, y, phi))
        }
        "3" => {
            println!();
            println!("Diffie-Hellman: gegeben g, p, α, β; gesucht gemeinsamer Schlüssel K.");
            let g = ask_int_min("g", 2);
            let p = ask_int_min("p (prim)", 2);
            let alpha = ask_int_min("α (Alice öffentlich)", 0);
            let beta = ask_int_min("β (Bob öffentlich)", 0);
            handle(dh_geg_g_p_alpha_beta_ges_k::run(g, p, alpha, beta))
        }
        "4" => {
            println!();
            println!("ElGamal: gegeben Kpub = (p, g, e); gesucht privater Schlüssel d.");
            let p = ask_int_min("p (prim)", 2);
            let g = ask_int_min("g", 2);
            let e = ask_int_min("e", 1);
            handle(elgamal_geg_kpub_ges_priv_verf::run(p, g, e))
        }
        "5" => {
            println!();
            println!("RSA: prüfen, ob privater Schlüssel d zu Kpub = (n, e) passt.");
            let n = ask_int_min("n", 2);
            let e = ask_int_min("e", 1);
            let d = ask_int_min("d (zu prüfen)", 1);
            let y = if ask_yes_no("Beispiel-Geheimtext y für Round-Trip-Test angeben?") {
                Some(ask_int_min("y", 0))
            } else {
                None
            };
            handle(rsa_geg_kpub_kpriv_encm_ges_kpriv_to_kpub::run(n, e, d, y))
        }
        "6" => {
            println!();
            println!("ElGamal-Signatur: Verifikation von (m, r, s) gegen Kpub = (p, g, e).");
            let p = ask_int_min("p (prim)", 2);
            let g = ask_int_min("g", 2);
            let e = ask_int_min("e", 1);
            let m = ask_int_min("m", 0);
            let r = ask_int_min("r", 0);
            let s = ask_int_min("s", 0);
            handle(elgamal_geg_sig_kpub_ges_legitsig::run(p, g, e, m, r, s))
        }
        "7" => {
            println!();
            println!("RSA: aus Kpub = (n, e) und Geheimtext y den privaten Schlüssel d");
            println!("und den Klartext x bestimmen (z. B. nach Wireshark-Auswertung).");
            println!("Hinweis: Werte aus pcap-Daten ggf. von Hex in Dezimal umrechnen.");
            let n = ask_int_min("n", 2);
            let e = ask_int_min("e", 1);
            let y = ask_int_min("y (Geheimtext)", 0);
            handle(rsa_geg_kpub_ges_kpriv_plain::run(n, e, y))
        }
        "8" => {
            println!();
            println!("ElGamal: aus Kpub = (p, g, e) und Geheimtext (a, b) den privaten");
            println!("Schlüssel d und den Klartext m bestimmen (z. B. nach Wireshark-Auswertung).");
            println!("Hinweis: Werte aus pcap-Daten ggf. von Hex in Dezimal umrechnen.");
            let p = ask_int_min("p (prim)", 2);
            let g = ask_int_min("g", 2);
            let e = ask_int_min("e", 1);
            let a = ask_int_min("a (= c1)", 0);
            let b = ask_int_min("b (= c2)", 0);
            handle(elgamal_geg_kpub_ges_kpriv::run(p, g, e, a, b))
        }
        "9" => {
            println!();
            println!("Diffie-Hellman: aus dem Mitschnitt p, g, α, β extrahieren und K bestimmen.");
            println!("Hinweis: Werte aus pcap-Daten ggf. von Hex in Dezimal umrechnen.");
            let p = ask_int_min("p (Modul, prim)", 2);
            let g = ask_int_min("g (Basis)", 2);
            let alpha = ask_int_min("α (Alice öffentlich)", 0);
            let beta = ask_int_min("β (Bob öffentlich)", 0);
            let c = if ask_yes_no("Geheimtext c im Mitschnitt vorhanden?") {
                Some(ask_int_min("c (Geheimtext)", 0))
            } else {
                None
            };
            handle(dh_ges_schluessel::run(p, g, alpha, beta, c))
        }
        "1" => {
            println!();
            println!("ElGamal: Multiplikativer Homomorphismus");
            println!("Öffentlicher Schlüssel K_pub = (p, g, e), privater Schlüssel d.");
            println!("Ursprüngliches Chiffrat (a1, b1) zu m1, kombiniertes Chiffrat (a, b).");
            let p = ask_int_min("p (prim)", 2);
            let g = ask_int_min("g", 2);
            let e = ask_int_min("e (öffentlich)", 1);
            let d = ask_int_min("d (privat)", 1);
            let a1 = ask_int_min("a1", 0);
            let b1 = ask_int_min("b1", 0);
            let a = ask_int_min("a (kombiniert)", 0);
            let b = ask_int_min("b (kombiniert)", 0);
            handle(elgamal_mult_homomorph::run(p, g, e, d, a1, b1, a, b))
        }
        _ => None,
    }
}
