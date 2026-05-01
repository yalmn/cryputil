use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{gcd, is_prime, mod_inv, mod_pow, rem_euclid};
use crate::core::trace::Trace;

// Input: p (prim), g, d (privat), m (Nachricht), k (ephemeral, gcd(k, p−1) = 1)
// Calc:  ElGamal-Signatur (r, s) mit r = g^k mod p, s = (m − d·r) · k^(-1) mod (p−1)
// Output: Trace mit Signatur (r, s)
pub fn sign(p: i128, g: i128, d: i128, m: i128, k: i128) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    if k <= 0 || k >= p - 1 {
        return Err(CalcError::Bereich("k muss in (0, p−1) liegen".into()));
    }
    if gcd(k, p - 1) != 1 {
        return Err(CalcError::UngueltigeEingabe(format!(
            "gcd(k, p−1) = {} ≠ 1",
            gcd(k, p - 1)
        )));
    }
    let mut t = Trace::new("ElGamal-Signatur: Erzeugung");
    t.input("p", p);
    t.input("g", g);
    t.input("d (privat)", d);
    t.input("m (Nachricht)", m);
    t.input("k (ephemeral)", k);

    let s1 = t.step("Öffentlicher Schlüssel e = g^d mod p");
    let e = mod_pow(g, d, p)?;
    t.line(s1, format!("e = g^d mod p = {}^{} mod {} = {}", g, d, p, e));

    let s2 = t.step("r = g^k mod p");
    let r = mod_pow(g, k, p)?;
    t.line(s2, format!("r = g^k mod p = {}^{} mod {} = {}", g, k, p, r));

    let s3 = t.step("s = (m − d·r) · k^(-1) mod (p − 1)");
    let k_inv = mod_inv(k, p - 1)?;
    let inner = rem_euclid(m - rem_euclid(d * r, p - 1), p - 1);
    let s_val = rem_euclid(inner * k_inv, p - 1);
    t.line(s3, format!("k^(-1) mod (p − 1) = {}", k_inv));
    t.line(
        s3,
        format!(
            "(m − d·r) mod (p − 1) = {} mod {} = {}",
            m - d * r,
            p - 1,
            inner
        ),
    );
    t.line(
        s3,
        format!("s = {} · {} mod {} = {}", inner, k_inv, p - 1, s_val),
    );

    t.result("e (öffentlich)", e);
    t.result("r", r);
    t.result("s", s_val);
    t.result("Signatur (r, s)", format!("({}, {})", r, s_val));
    Ok(t)
}

// Input: p (prim), g, e (öffentlich), m (Nachricht), r, s (Signatur)
// Calc:  Verifikation per Vergleich e^r · r^s mod p mit g^m mod p
// Output: Trace mit Bewertung gültig/ungültig
pub fn verify(p: i128, g: i128, e: i128, m: i128, r: i128, s: i128) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    let mut t = Trace::new("ElGamal-Signatur: Verifikation");
    t.input("p", p);
    t.input("g", g);
    t.input("e (öffentlich)", e);
    t.input("m (Nachricht)", m);
    t.input("r", r);
    t.input("s", s);

    let s1 = t.step("Bereichsprüfung von r und s");
    let r_ok = r > 0 && r < p;
    let s_ok = s > 0 && s < p - 1;
    t.line(
        s1,
        format!(
            "0 < r < p: 0 < {} < {} → {}",
            r,
            p,
            if r_ok { "ok" } else { "verletzt" }
        ),
    );
    t.line(
        s1,
        format!(
            "0 < s < p − 1: 0 < {} < {} → {}",
            s,
            p - 1,
            if s_ok { "ok" } else { "verletzt" }
        ),
    );
    if !r_ok || !s_ok {
        t.result("Signatur gültig", false);
        return Ok(t);
    }

    let s2 = t.step("v1 = e^r · r^s mod p");
    let er = mod_pow(e, r, p)?;
    let rs = mod_pow(r, s, p)?;
    let v1 = rem_euclid(er * rs, p);
    t.line(s2, format!("e^r mod p = {}", er));
    t.line(s2, format!("r^s mod p = {}", rs));
    t.line(s2, format!("v1 = {} · {} mod {} = {}", er, rs, p, v1));

    let s3 = t.step("v2 = g^m mod p");
    let v2 = mod_pow(g, m, p)?;
    t.line(
        s3,
        format!("v2 = g^m mod p = {}^{} mod {} = {}", g, m, p, v2),
    );

    let s4 = t.step("Vergleich v1 = v2");
    let valid = v1 == v2;
    t.line(s4, format!("v1 = {},  v2 = {}", v1, v2));
    if valid {
        t.line(s4, "v1 = v2 ✓ – Signatur ist gültig.");
    } else {
        t.line(s4, "v1 ≠ v2 – Signatur ist ungültig.");
    }

    t.result("v1", v1);
    t.result("v2", v2);
    t.result("Signatur gültig", valid);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_verify_roundtrip() {
        let p = 467i128;
        let g = 2i128;
        let d = 127i128;
        let m = 100i128;
        let k = 213i128;
        let sig = sign(p, g, d, m, k).unwrap();
        let r: i128 = sig
            .result
            .iter()
            .find(|(k, _)| k == &"r")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let s: i128 = sig
            .result
            .iter()
            .find(|(k, _)| k == &"s")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let e: i128 = sig
            .result
            .iter()
            .find(|(k, _)| k.starts_with("e "))
            .unwrap()
            .1
            .parse()
            .unwrap();
        let v = verify(p, g, e, m, r, s).unwrap();
        let valid = v
            .result
            .iter()
            .find(|(k, _)| k.starts_with("Signatur gültig"))
            .unwrap()
            .1
            .clone();
        assert_eq!(valid, "true");
    }
}
