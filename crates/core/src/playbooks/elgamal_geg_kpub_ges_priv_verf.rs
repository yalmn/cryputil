use std::collections::HashMap;

use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{is_prime, isqrt, mod_inv, mod_pow, rem_euclid};
use crate::core::trace::Trace;

// Input: p (prim), g, e (öffentliche Komponenten von Kpub = (p, g, e))
// Calc:  Diskreter Logarithmus d mit g^d ≡ e mod p (BSGS), anschließend Verifikation
// Output: Trace mit privatem Schlüssel d und Verifikationsergebnis
pub fn run(p: i128, g: i128, e: i128) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    let mut t = Trace::new("ElGamal: aus Kpub = (p, g, e) → privater Schlüssel d");
    t.input("p", p);
    t.input("g", g);
    t.input("e", e);

    let s1 = t.step("Aufgabenstellung");
    t.line(
        s1,
        "Beim ElGamal-Verfahren gilt e ≡ g^d mod p mit d als privatem Schlüssel.",
    );
    t.line(
        s1,
        "Zur Bestimmung von d aus Kpub = (p, g, e) muss der diskrete Logarithmus",
    );
    t.line(s1, "d = log_g(e) mod p berechnet werden.");

    let s2 = t.step("a) Diskreter Logarithmus d via Baby-Step-Giant-Step");
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
    t.line(
        s2,
        format!("Baby-Steps berechnet: {} Einträge.", baby.len()),
    );

    let factor = mod_inv(mod_pow(g, m, p)?, p)?;
    let mut gamma = rem_euclid(e, p);
    let mut d_priv: Option<i128> = None;
    for i in 0..m {
        if let Some(&j) = baby.get(&gamma) {
            let d = rem_euclid(i * m + j, p - 1);
            t.line(
                s2,
                format!("Treffer bei i = {}, j = {} → d = i·m + j = {}", i, j, d),
            );
            d_priv = Some(d);
            break;
        }
        gamma = rem_euclid(gamma * factor, p);
    }
    let d = d_priv.ok_or_else(|| CalcError::KeineLoesung("d nicht gefunden".into()))?;

    let s3 = t.step("b) Verifikation des privaten Schlüssels");
    let probe = mod_pow(g, d, p)?;
    t.line(s3, format!("g^d mod p = {}^{} mod {} = {}", g, d, p, probe));
    t.line(s3, format!("erwartet: e = {}", e));
    if probe != rem_euclid(e, p) {
        return Err(CalcError::KeineLoesung(
            "g^d mod p stimmt nicht mit e überein".into(),
        ));
    }
    t.line(s3, "g^d ≡ e mod p ✓ – Schlüssel korrekt.");

    t.result("d (privat)", d);
    t.result("Verifikation g^d mod p", probe);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aufgabe() {
        let t = run(25763, 2618, 2025).unwrap();
        let d: i128 = t
            .result
            .iter()
            .find(|(name, _)| name.starts_with("d "))
            .unwrap()
            .1
            .parse()
            .unwrap();
        assert_eq!(mod_pow(2618, d, 25763).unwrap(), 2025);
    }
}
