use std::collections::HashMap;

use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{is_prime, isqrt, mod_inv, mod_pow, rem_euclid};
use crate::core::trace::Trace;

// Input: g, p (prim), alpha = g^a mod p (Alice öffentlich), beta = g^b mod p (Bob öffentlich)
// Calc:  Diskreten Logarithmus a aus alpha bestimmen (BSGS), dann K = beta^a mod p
// Output: Trace mit privatem Exponent a, privatem Exponent b und gemeinsamem Schlüssel K
pub fn run(g: i128, p: i128, alpha: i128, beta: i128) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    let mut t = Trace::new("Diffie-Hellman: aus g, p, α, β → K");
    t.input("g", g);
    t.input("p", p);
    t.input("α (Alice öffentlich)", alpha);
    t.input("β (Bob öffentlich)", beta);

    let s1 = t.step("Aufgabenstellung");
    t.line(
        s1,
        "Gegeben sind die öffentlichen Werte α = g^a mod p und β = g^b mod p.",
    );
    t.line(
        s1,
        "Da nur öffentliche Werte vorliegen, muss zur Bestimmung von K der",
    );
    t.line(
        s1,
        "diskrete Logarithmus a (oder b) aus α (oder β) berechnet werden.",
    );
    t.line(s1, "Anschließend gilt: K = β^a mod p = α^b mod p.");

    let s2 = t.step("Diskreter Logarithmus a aus α (Baby-Step-Giant-Step)");
    let m = {
        let r = isqrt(p - 1);
        if r * r == p - 1 {
            r
        } else {
            r + 1
        }
    };
    t.line(s2, format!("m = ⌈√(p − 1)⌉ = {}", m));

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
                s2,
                format!("Treffer bei i = {}, j = {} → a = i·m + j = {}", i, j, a),
            );
            t.line(s2, format!("Probe: g^a mod p = {}", mod_pow(g, a, p)?));
            a_priv = Some(a);
            break;
        }
        gamma = rem_euclid(gamma * factor, p);
    }
    let a = a_priv.ok_or_else(|| CalcError::KeineLoesung("a nicht gefunden".into()))?;

    let s3 = t.step("Diskreter Logarithmus b aus β (zur Kontrolle)");
    let mut gamma = rem_euclid(beta, p);
    let mut b_priv: Option<i128> = None;
    for i in 0..m {
        if let Some(&j) = baby.get(&gamma) {
            let b = rem_euclid(i * m + j, p - 1);
            t.line(
                s3,
                format!("Treffer bei i = {}, j = {} → b = i·m + j = {}", i, j, b),
            );
            t.line(s3, format!("Probe: g^b mod p = {}", mod_pow(g, b, p)?));
            b_priv = Some(b);
            break;
        }
        gamma = rem_euclid(gamma * factor, p);
    }
    let b = b_priv.ok_or_else(|| CalcError::KeineLoesung("b nicht gefunden".into()))?;

    let s4 = t.step("Gemeinsamer Schlüssel K");
    let k_a = mod_pow(beta, a, p)?;
    let k_b = mod_pow(alpha, b, p)?;
    t.line(
        s4,
        format!("K_A = β^a mod p = {}^{} mod {} = {}", beta, a, p, k_a),
    );
    t.line(
        s4,
        format!("K_B = α^b mod p = {}^{} mod {} = {}", alpha, b, p, k_b),
    );
    if k_a != k_b {
        return Err(CalcError::KeineLoesung(
            "K_A ≠ K_B – Eingaben prüfen".into(),
        ));
    }
    t.line(s4, "K_A = K_B ✓");

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
        let t = run(545, 21407, 3796, 19505).unwrap();
        let k = t
            .result
            .iter()
            .find(|(name, _)| name.starts_with("K "))
            .unwrap()
            .1
            .clone();
        let k_n: i128 = k.parse().unwrap();
        let a: i128 = t
            .result
            .iter()
            .find(|(name, _)| name.starts_with("a "))
            .unwrap()
            .1
            .parse()
            .unwrap();
        assert_eq!(mod_pow(545, a, 21407).unwrap(), 3796);
        assert_eq!(mod_pow(19505, a, 21407).unwrap(), k_n);
    }
}
