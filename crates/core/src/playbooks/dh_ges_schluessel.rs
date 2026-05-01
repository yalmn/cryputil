use std::collections::HashMap;

use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{is_prime, isqrt, mod_inv, mod_pow, rem_euclid};
use crate::core::trace::Trace;

// Input: p (Modul, prim), g (Basis), alpha (Alice öffentlich), beta (Bob öffentlich), c (optional: Geheimtext mit K verschlüsselt)
// Calc:  Diskreten Logarithmus a aus alpha bestimmen (BSGS), gemeinsamen Schlüssel K = beta^a mod p berechnen, Geheimtext-Hinweise ausgeben
// Output: Trace mit privatem Exponent a, b und gemeinsamem Schlüssel K (sowie Entschlüsselungs-Kandidaten)
pub fn run(p: i128, g: i128, alpha: i128, beta: i128, c: Option<i128>) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    let mut t = Trace::new("Diffie-Hellman: gemeinsamen Schlüssel aus Mitschnitt bestimmen");
    t.input("Modul p", p);
    t.input("Basis g", g);
    t.input("α (Alice öffentlich)", alpha);
    t.input("β (Bob öffentlich)", beta);
    if let Some(c_v) = c {
        t.input("Geheimtext c", c_v);
    }

    let s0 = t.step("Hinweis zur Notation");
    t.line(
        s0,
        "Aus dem Mitschnitt sind zu entnehmen: Modul p, Basis g sowie die ausgetauschten",
    );
    t.line(
        s0,
        "öffentlichen Werte α = g^a mod p (Alice) und β = g^b mod p (Bob).",
    );
    t.line(
        s0,
        "Der gemeinsame Schlüssel ist K = α^b mod p = β^a mod p.",
    );
    t.line(
        s0,
        "Da nur die öffentlichen Werte gesendet werden, muss zum Brechen ein diskreter",
    );
    t.line(s0, "Logarithmus berechnet werden.");

    let s1 = t.step("a) Diskreter Logarithmus a aus α via Baby-Step-Giant-Step");
    let m = {
        let r = isqrt(p - 1);
        if r * r == p - 1 {
            r
        } else {
            r + 1
        }
    };
    t.line(s1, format!("m = ⌈√(p − 1)⌉ = {}", m));

    let mut baby: HashMap<i128, i128> = HashMap::new();
    let mut acc: i128 = 1;
    for j in 0..m {
        baby.entry(acc).or_insert(j);
        acc = rem_euclid(acc * g, p);
    }

    let factor = mod_inv(mod_pow(g, m, p)?, p)?;
    let mut gamma = rem_euclid(alpha, p);
    let mut a_priv: Option<i128> = None;
    for i in 0..m {
        if let Some(&j) = baby.get(&gamma) {
            let a = rem_euclid(i * m + j, p - 1);
            t.line(
                s1,
                format!("Treffer bei i = {}, j = {} → a = i·m + j = {}", i, j, a),
            );
            t.line(s1, format!("Probe: g^a mod p = {}", mod_pow(g, a, p)?));
            a_priv = Some(a);
            break;
        }
        gamma = rem_euclid(gamma * factor, p);
    }
    let a = a_priv.ok_or_else(|| CalcError::KeineLoesung("a nicht gefunden".into()))?;

    let s2 = t.step("a) Diskreter Logarithmus b aus β (zur Kontrolle)");
    let mut gamma = rem_euclid(beta, p);
    let mut b_priv: Option<i128> = None;
    for i in 0..m {
        if let Some(&j) = baby.get(&gamma) {
            let b = rem_euclid(i * m + j, p - 1);
            t.line(
                s2,
                format!("Treffer bei i = {}, j = {} → b = i·m + j = {}", i, j, b),
            );
            t.line(s2, format!("Probe: g^b mod p = {}", mod_pow(g, b, p)?));
            b_priv = Some(b);
            break;
        }
        gamma = rem_euclid(gamma * factor, p);
    }
    let b = b_priv.ok_or_else(|| CalcError::KeineLoesung("b nicht gefunden".into()))?;

    let s3 = t.step("a) Gemeinsamer Schlüssel K");
    let k_a = mod_pow(beta, a, p)?;
    let k_b = mod_pow(alpha, b, p)?;
    t.line(
        s3,
        format!("K_A = β^a mod p = {}^{} mod {} = {}", beta, a, p, k_a),
    );
    t.line(
        s3,
        format!("K_B = α^b mod p = {}^{} mod {} = {}", alpha, b, p, k_b),
    );
    if k_a != k_b {
        return Err(CalcError::KeineLoesung(
            "K_A ≠ K_B – Eingaben prüfen".into(),
        ));
    }
    t.line(s3, "K_A = K_B ✓");

    if let Some(c_v) = c {
        let s4 = t.step("Entschlüsselungs-Kandidaten für den Geheimtext c");
        t.line(
            s4,
            "Da das verwendete symmetrische Verfahren nicht spezifiziert ist, werden",
        );
        t.line(s4, "die gängigsten einfachen Konstruktionen ausgewertet.");
        if c_v >= 0 && c_v < p {
            if let Ok(k_inv) = mod_inv(k_a, p) {
                let m_mul = rem_euclid(c_v * k_inv, p);
                t.line(
                    s4,
                    format!(
                        "Variante c = m · K mod p:   m = c · K^(-1) mod p = {}",
                        m_mul
                    ),
                );
            }
            let m_add = rem_euclid(c_v - k_a, p);
            t.line(
                s4,
                format!("Variante c = m + K mod p:   m = (c − K) mod p = {}", m_add),
            );
        } else {
            t.line(
                s4,
                "c liegt außerhalb von [0, p) – modulare Entschlüsselungsvarianten übersprungen.",
            );
        }
        let m_xor = (c_v as u128) ^ (k_a as u128);
        t.line(
            s4,
            format!("Variante c = m XOR K:        m = c XOR K = {}", m_xor),
        );
        t.line(
            s4,
            "Welcher Wert lesbar ist, hängt vom konkreten Kanal-Cipher im Mitschnitt ab.",
        );
    }

    t.result("a (Alice privat)", a);
    t.result("b (Bob privat)", b);
    t.result("K (gemeinsamer Schlüssel)", k_a);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aufgabe() {
        let t = run(28051, 1837, 9966, 1112, Some(6269)).unwrap();
        let a: i128 = t
            .result
            .iter()
            .find(|(name, _)| name.starts_with("a "))
            .unwrap()
            .1
            .parse()
            .unwrap();
        let k: i128 = t
            .result
            .iter()
            .find(|(name, _)| name.starts_with("K "))
            .unwrap()
            .1
            .parse()
            .unwrap();
        assert_eq!(mod_pow(1837, a, 28051).unwrap(), 9966);
        assert_eq!(mod_pow(1112, a, 28051).unwrap(), k);
    }
}
