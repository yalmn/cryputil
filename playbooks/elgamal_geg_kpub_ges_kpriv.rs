use std::collections::HashMap;

use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{is_prime, isqrt, mod_inv, mod_pow, rem_euclid};
use crate::core::trace::Trace;

// Input: p (prim), g, e (Kpub = (p, g, e)), a und b (Geheimtext-Paar (a, b) = (c1, c2))
// Calc:  Diskreter Logarithmus d aus e (BSGS), dann ElGamal-Entschlüsselung m = b · (a^d)^(-1) mod p
// Output: Trace mit privatem Schlüssel d und Klartext m
pub fn run(p: i128, g: i128, e: i128, a: i128, b: i128) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    if a < 0 || a >= p || b < 0 || b >= p {
        return Err(CalcError::Bereich("a und b müssen in [0, p) liegen".into()));
    }
    let mut t = Trace::new("ElGamal: aus Kpub = (p, g, e) und (a, b) → d und Klartext");
    t.input("p", p);
    t.input("g", g);
    t.input("e", e);
    t.input("a (= c1)", a);
    t.input("b (= c2)", b);

    let s0 = t.step("Hinweis zur Notation");
    t.line(
        s0,
        "In Mitschnitten (z. B. .pcapng) liegen Werte häufig als Hex- oder Big-Endian-Bytes vor.",
    );
    t.line(
        s0,
        "Vor der Eingabe in dieses Playbook sind p, g, e, a und b in Dezimalform umzurechnen.",
    );
    t.line(
        s0,
        "Notation: Kpub = (p, g, e) mit e = g^d mod p, Geheimtext (a, b) = (g^k mod p, m·e^k mod p).",
    );

    let s1 = t.step("a) Diskreter Logarithmus d via Baby-Step-Giant-Step");
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
    t.line(
        s1,
        format!("Baby-Steps berechnet: {} Einträge.", baby.len()),
    );

    let factor = mod_inv(mod_pow(g, m, p)?, p)?;
    let mut gamma = rem_euclid(e, p);
    let mut d_priv: Option<i128> = None;
    for i in 0..m {
        if let Some(&j) = baby.get(&gamma) {
            let d = rem_euclid(i * m + j, p - 1);
            t.line(
                s1,
                format!("Treffer bei i = {}, j = {} → d = i·m + j = {}", i, j, d),
            );
            t.line(s1, format!("Probe: g^d mod p = {}", mod_pow(g, d, p)?));
            d_priv = Some(d);
            break;
        }
        gamma = rem_euclid(gamma * factor, p);
    }
    let d = d_priv.ok_or_else(|| CalcError::KeineLoesung("d nicht gefunden".into()))?;

    let s2 = t.step("a) Maskierungselement s = a^d mod p");
    let s_mask = mod_pow(a, d, p)?;
    let s_inv = mod_inv(s_mask, p)?;
    t.line(
        s2,
        format!("s = a^d mod p = {}^{} mod {} = {}", a, d, p, s_mask),
    );
    t.line(s2, format!("s^(-1) mod p = {}", s_inv));

    let s3 = t.step("a) Entschlüsselung m = b · s^(-1) mod p");
    let m_plain = rem_euclid(b * s_inv, p);
    t.line(
        s3,
        format!(
            "m = b · s^(-1) mod p = {} · {} mod {} = {}",
            b, s_inv, p, m_plain
        ),
    );

    let s4 = t.step("Zusammenfassung");
    t.line(s4, format!("Kpub = (p, g, e) = ({}, {}, {})", p, g, e));
    t.line(s4, format!("Kpriv = d = {}", d));
    t.line(s4, format!("Klartext m = {}", m_plain));

    t.result("d (privat)", d);
    t.result("m (Klartext)", m_plain);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let p = 25763i128;
        let g = 2618i128;
        let d_known = 1234i128;
        let e_pub = mod_pow(g, d_known, p).unwrap();
        let m = 9999i128;
        let k = 77i128;
        let a = mod_pow(g, k, p).unwrap();
        let b = rem_euclid(m * mod_pow(e_pub, k, p).unwrap(), p);
        let t = run(p, g, e_pub, a, b).unwrap();
        let m_rec = t
            .result
            .iter()
            .find(|(name, _)| name.starts_with("m "))
            .unwrap()
            .1
            .clone();
        assert_eq!(m_rec, m.to_string());
        let d_rec: i128 = t
            .result
            .iter()
            .find(|(name, _)| name.starts_with("d "))
            .unwrap()
            .1
            .parse()
            .unwrap();
        assert_eq!(mod_pow(g, d_rec, p).unwrap(), e_pub);
    }
}
